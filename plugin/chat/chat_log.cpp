#include "chat/chat_log.h"

#include <algorithm>

#include "fnvcoop_mut.h"

// Twins: 1020 = a line that is too old is still shown; 1021 = the OLDEST lines are shown when there are more than fit; 1022 = no de-duplication;
//        1023 = the history is never trimmed; 1024 = forgetSender forgets nothing (a rejoining player's first lines are taken for repeats);
//        1025 = the memory of seen ids is never trimmed
namespace nvcoop::chat {

constexpr size_t kSeenMemory = 512;

void ChatLog::setCapacity(size_t n) {
    capacity_ = n ? n : 1;
    while (!FNV_MUT(1023) && lines_.size() > capacity_) lines_.pop_front();
}

bool ChatLog::seen(Slot slot, uint32_t msgId) const { return msgId != 0 && seen_.count(key(slot, msgId)) != 0; }

bool ChatLog::add(Line l) {
    if (l.msgId != 0 && !FNV_MUT(1022)) {
        if (!seen_.insert(key(l.slot, l.msgId)).second) return false;
        seenOrder_.push_back(key(l.slot, l.msgId));
        while (!FNV_MUT(1025) && seenOrder_.size() > kSeenMemory) {
            seen_.erase(seenOrder_.front());
            seenOrder_.pop_front();
        }
    }
    l.seq = ++seq_;
    lines_.push_back(std::move(l));
    while (!FNV_MUT(1023) && lines_.size() > capacity_) lines_.pop_front();
    return true;
}

void ChatLog::forgetSender(Slot slot) {
    if (FNV_MUT(1024)) return;
    for (auto it = seen_.begin(); it != seen_.end();) {
        if ((*it >> 32) == slot) it = seen_.erase(it);
        else ++it;
    }
    seenOrder_.erase(std::remove_if(seenOrder_.begin(), seenOrder_.end(), [slot](uint64_t k) { return (k >> 32) == slot; }), seenOrder_.end());
}

std::vector<const Line*> ChatLog::visible(uint64_t nowMs, uint32_t ttlMs, size_t maxLines) const {
    std::vector<const Line*> live;
    for (const Line& l : lines_)
        if (FNV_MUT(1020) || nowMs < l.atMs + ttlMs) live.push_back(&l);
    if (live.size() > maxLines) {
        if (FNV_MUT(1021)) live.resize(maxLines);
        else live.erase(live.begin(), live.end() - static_cast<std::ptrdiff_t>(maxLines));
    }
    return live;
}

std::vector<const Line*> ChatLog::recent(size_t n) const {
    std::vector<const Line*> out;
    const size_t from = lines_.size() > n ? lines_.size() - n : 0;
    for (size_t i = from; i < lines_.size(); ++i) out.push_back(&lines_[i]);
    return out;
}

}  // namespace nvcoop::chat
