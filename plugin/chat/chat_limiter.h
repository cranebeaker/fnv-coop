// Per sender rate limit for chat lines: a sender may say `lines` lines in any window of `windowMs`. The host applies it to every guest (and to its own
// player), a guest applies it to its own player first so the player is told at once. A refused line does not count, so a sender that stops is free again
// after one window.
#pragma once
#include <cstdint>
#include <deque>
#include <map>

#include "chat/chat_types.h"

namespace nvcoop::chat {

class RateLimiter {
public:
    RateLimiter(uint32_t lines = 5, uint32_t windowMs = 10000) : lines_(lines), windowMs_(windowMs) {}
    bool allow(Slot who, uint64_t nowMs);     // counts the line when allowed
    void forget(Slot who) { times_.erase(who); }
    void reset() { times_.clear(); denied_ = 0; }
    uint64_t denied() const { return denied_; }

private:
    uint32_t lines_, windowMs_;
    std::map<Slot, std::deque<uint64_t>> times_;
    uint64_t denied_ = 0;
};

}  // namespace nvcoop::chat
