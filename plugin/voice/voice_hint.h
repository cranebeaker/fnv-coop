// When the HUD hint "you are on the air" is shown. Game-free. The hint appears when a burst starts and is renewed every
// refreshMs while the player keeps talking (a HUD message lasts a few seconds). A player who taps the key does not fill the
// message queue: a new burst shows it again only after minGapMs. A separate one-off hint says the microphone is not available.
#pragma once
#include <cstdint>

#include "fnvcoop_mut.h"

namespace nvcoop::voice {

class HintGate {
public:
    HintGate(uint32_t minGapMs = 1500, uint32_t refreshMs = 3000) : gap_(minGapMs), refresh_(refreshMs) {}

    // Call every frame. True when the "talking" hint should be shown now.
    bool talking(bool talkingNow, uint64_t nowMs) {
        const bool rising = talkingNow && !was_;
        was_ = talkingNow;
        if (!talkingNow) return false;
        const uint64_t since = nowMs - last_;
        bool show = false;
        if (rising) show = !shown_ || since >= gap_ || FNV_MUT(381);   // twin 381: every burst shows the hint again, however fast they come
        else show = since >= refresh_ && !FNV_MUT(382);                // twin 382: a long sentence shows the hint only once
        if (!show) return false;
        shown_ = true;
        last_ = nowMs;
        return true;
    }
    // True once: the microphone is not available.
    bool micFailed(bool failed) {
        if (!failed || micShown_) return false;
        micShown_ = true;
        return true;
    }
    void reset() { was_ = false; shown_ = false; micShown_ = false; }

private:
    uint32_t gap_, refresh_;
    bool was_ = false, shown_ = false, micShown_ = false;
    uint64_t last_ = 0;
};

}  // namespace nvcoop::voice
