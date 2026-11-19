// Shared experience. Under NV:MP every player has their own character and the engine pays XP only to the player who earned it: a kill,
// a lock picked, a terminal hacked, a place discovered. Each machine watches its own player's XP (the Player vtable RewardXP, +0x488) and
// sends what the engine paid on mux channel kChannel; the host passes a guest's share on to the other guests and every receiver adds it to
// its own player. XP that already travels another way is not sent again: a RewardXP script command (captured, shared as a reward) and XP
// applied from another player. Game-free: the wire and the batching; coop_module.cpp does the engine part.
//   [0xD6][ver=1][slot u8][xp i32 LE]          a share: XP the sender's engine just paid
//   [0xD6][0x81][slot u8][total i32 LE]        the sender's whole XP, every few seconds: a player left behind (an NV:MP respawn puts the player's
//                                             character back to their last save, XP included; a friend joins later) is topped up to the others
#pragma once
#include <cstddef>
#include <cstdint>
#include <vector>

namespace nvcoop { namespace coop { namespace sharedxp {

constexpr uint8_t kChannel = 0xD6;   // 0xD5 conversation subtitles
constexpr uint8_t kVersion = 1;
constexpr int32_t kMaxShare = 100000;   // one message never carries more than this (a bogus value must not level a player to 50)
constexpr uint8_t kTotalKind = 0x81;
constexpr int32_t kMaxTotal = 2000000;
constexpr int32_t kCatchUpSlack = 25;   // smaller gaps are shares still on the way

struct Share {
    uint8_t slot = 0;   // who earned it
    int32_t xp = 0;
};

inline std::vector<uint8_t> encode(const Share& s) {
    std::vector<uint8_t> b{kChannel, kVersion, s.slot};
    for (int i = 0; i < 4; ++i) b.push_back(static_cast<uint8_t>(static_cast<uint32_t>(s.xp) >> (8 * i)));
    return b;
}

inline bool decode(const uint8_t* p, size_t n, Share* out) {
    if (n != 7 || p[0] != kChannel || p[1] != kVersion) return false;
    Share s;
    s.slot = p[2];
    s.xp = static_cast<int32_t>(uint32_t(p[3]) | uint32_t(p[4]) << 8 | uint32_t(p[5]) << 16 | uint32_t(p[6]) << 24);
    if (s.xp <= 0 || s.xp > kMaxShare) return false;
    *out = s;
    return true;
}

inline std::vector<uint8_t> encodeTotal(uint8_t slot, int32_t total) {
    std::vector<uint8_t> b{kChannel, kTotalKind, slot};
    for (int i = 0; i < 4; ++i) b.push_back(static_cast<uint8_t>(static_cast<uint32_t>(total) >> (8 * i)));
    return b;
}
inline bool decodeTotal(const uint8_t* p, size_t n, uint8_t* slot, int32_t* total) {
    if (n != 7 || p[0] != kChannel || p[1] != kTotalKind) return false;
    const int32_t t = static_cast<int32_t>(uint32_t(p[3]) | uint32_t(p[4]) << 8 | uint32_t(p[5]) << 16 | uint32_t(p[6]) << 24);
    if (t < 0 || t > kMaxTotal) return false;
    *slot = p[2];
    *total = t;
    return true;
}

// Catch-up: the others' totals against this player's. A gap must be seen twice in a row (shares on the way close it by themselves).
class CatchUp {
public:
    // returns the XP to add now (0 = nothing)
    int32_t check(int32_t mine, int32_t bestOther) {
        const int32_t gap = bestOther - mine;
        if (gap <= kCatchUpSlack) { seen_ = 0; return 0; }
        if (++seen_ < 2) return 0;
        seen_ = 0;
        return gap > kMaxShare ? kMaxShare : gap;
    }
private:
    int seen_ = 0;
};

// What the local engine paid, added up between sends (a burst of kills goes out as one message).
class Outbox {
public:
    void earned(int32_t xp) { if (xp > 0) pending_ += xp; }
    // the amount to send now (0 = nothing); at most every `gapMs`
    int32_t take(uint64_t nowMs, uint64_t gapMs = 250) {
        if (pending_ <= 0 || nowMs - last_ < gapMs) return 0;
        last_ = nowMs;
        const int64_t x = pending_ > kMaxShare ? kMaxShare : pending_;
        pending_ -= x;
        return static_cast<int32_t>(x);
    }
    int64_t pending() const { return pending_; }
private:
    int64_t pending_ = 0;
    uint64_t last_ = 0;
};

}}}  // namespace nvcoop::coop::sharedxp
