#include "chat/chat_keyblock.h"

#include "fnvcoop_mut.h"

// Twins: 1080 = closing the line never enables the keys again (the player is stuck); 1081 = Escape (scancode 1) is not disabled; 1082 = enabling stops
//        after the first batch; 1083 = enabling also touches keys this class did not disable; 1084 = a console that refuses every line is retried forever;
//        1087 = enabling gives up after repeated failures like disabling does; 1085 = the mouse buttons are not disabled; 1086 = the walk is not spread over frames (everything in one frame)
namespace nvcoop::chat {

bool KeyBlocker::settled() const {
    for (int k = kFirstKey; k <= kLastKey; ++k)
        if (disabled_[static_cast<size_t>(k)] != want_) return false;
    return true;
}

int KeyBlocker::disabledCount() const {
    int n = 0;
    for (int k = kFirstKey; k <= kLastKey; ++k) n += disabled_[static_cast<size_t>(k)] ? 1 : 0;
    return n;
}

void KeyBlocker::frame() {
    if (!exec_) return;
    if (failed_) return;
    if (cooldown_ > 0) { --cooldown_; return; }
    int budget = FNV_MUT(1086) ? 100000 : perFrame_;
    for (int k = kFirstKey; k <= kLastKey && budget > 0; ++k) {
        const bool on = disabled_[static_cast<size_t>(k)];
        if (want_ && FNV_MUT(1081) && k == 1) continue;
        if (want_ && FNV_MUT(1085) && k >= 256) continue;
        if (!want_ && FNV_MUT(1080)) return;
        if (!want_ && FNV_MUT(1082) && k > perFrame_) return;
        if (want_ == on && !(!want_ && FNV_MUT(1083))) continue;
        const std::string line = std::string(want_ ? "DisableKey " : "EnableKey ") + std::to_string(k) + " 2";
        ++lines_;
        --budget;
        if (exec_(line)) {
            failures_ = 0;
            disabled_[static_cast<size_t>(k)] = want_;
        } else if (++failures_ >= 8 && !FNV_MUT(1084)) {
            // Disabling gives up (the chat then types with the game still listening, which beats a retry storm). Enabling must not give up, a player
            // with dead keys is far worse: it waits two seconds' worth of frames and walks again.
            if (want_ || FNV_MUT(1087)) failed_ = true;
            else { cooldown_ = 120; failures_ = 0; }
            return;
        }
    }
}

}  // namespace nvcoop::chat
