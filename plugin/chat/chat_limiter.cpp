#include "chat/chat_limiter.h"

#include "fnvcoop_mut.h"

// Twins: 1010 = nothing is ever refused; 1011 = one shared bucket for every sender (a spammer starves the others);
//        1012 = the window never slides (a sender once refused stays refused); 1013 = a refused line counts too (the lock-out never ends while they keep trying)
namespace nvcoop::chat {

bool RateLimiter::allow(Slot who, uint64_t nowMs) {
    if (FNV_MUT(1010)) return true;
    auto& q = times_[FNV_MUT(1011) ? Slot(0) : who];
    if (!q.empty() && nowMs < q.back()) q.clear();   // the clock went backwards (a reload, a new session): start over
    if (!FNV_MUT(1012))
        while (!q.empty() && nowMs - q.front() >= windowMs_) q.pop_front();
    if (q.size() >= lines_) {
        ++denied_;
        if (FNV_MUT(1013)) q.push_back(nowMs);
        return false;
    }
    q.push_back(nowMs);
    return true;
}

}  // namespace nvcoop::chat
