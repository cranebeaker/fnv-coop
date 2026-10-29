// The chat history of this machine: the last N lines, what is still on screen, and the memory that keeps one line from being shown twice.
#pragma once
#include <deque>
#include <set>
#include <vector>

#include "chat/chat_types.h"

namespace nvcoop::chat {

class ChatLog {
public:
    explicit ChatLog(size_t capacity = 40) : capacity_(capacity ? capacity : 1) {}
    void setCapacity(size_t n);
    // Adds a line (assigns seq and keeps atMs as given). A line with a msgId whose (slot, msgId) was seen before is refused: returns false. Lines
    // without a msgId (this machine's own notices) are never refused.
    bool add(Line l);
    bool seen(Slot slot, uint32_t msgId) const;
    void forgetSender(Slot slot);               // that slot's ids are forgotten (the player left; a new one may reuse the slot)
    void clearSeen() { seen_.clear(); seenOrder_.clear(); }
    void clear() { lines_.clear(); clearSeen(); }
    const std::deque<Line>& lines() const { return lines_; }
    // The newest `maxLines` lines that arrived less than ttlMs ago, oldest first.
    std::vector<const Line*> visible(uint64_t nowMs, uint32_t ttlMs, size_t maxLines) const;
    // The newest `n` lines whatever their age, oldest first.
    std::vector<const Line*> recent(size_t n) const;
    uint32_t lastSeq() const { return seq_; }

private:
    static uint64_t key(Slot s, uint32_t id) { return (uint64_t(s) << 32) | id; }
    size_t capacity_;
    std::deque<Line> lines_;
    std::set<uint64_t> seen_;
    std::deque<uint64_t> seenOrder_;
    uint32_t seq_ = 0;
};

}  // namespace nvcoop::chat
