// ChatCore: the rules of the chat for one machine, with no game and no transport in it (both come in through Ports).
//   host  : takes Say from a guest (checks the text, the rate, repeats), shows the line and sends it to every OTHER joined guest; makes the system lines
//           (joined, left, and what other modules post with system()); answers a Hello with the player's name made unique.
//   guest : sends Say to the host and shows its own line at once; shows what the host sends (never its own line again, never a repeat).
// All text from the network is sanitised here again. All time comes in as nowMs.
#pragma once
#include <functional>
#include <map>
#include <set>
#include <string>
#include <string_view>

#include "chat/chat_limiter.h"
#include "chat/chat_log.h"
#include "chat/chat_types.h"
#include "chat/chat_wire.h"

namespace nvcoop::chat {

struct Ports {
    std::function<void(Bytes)> sendToHost;                  // guest: a datagram for the host
    std::function<void(Slot, Bytes)> sendToSlot;            // host: a datagram for one joined guest
    std::function<std::vector<Slot>()> joinedSlots;         // host: the guests that are joined right now (not slot 0)
};

enum class SayResult : uint8_t { Sent, Empty, RateLimited, NoSession };

class ChatCore {
public:
    explicit ChatCore(Config cfg = {}, uint32_t idSeed = 1);
    void setPorts(Ports p) { ports_ = std::move(p); }
    void setOnLine(std::function<void(const Line&)> f) { onLine_ = std::move(f); }   // a line went into the history (the surfaces); survives setPorts
    void setConfig(const Config& c);

    // A session started (or the role / slot changed): the repeat memory and the names start over, the history stays.
    void begin(bool isHost, Slot self);
    void end();
    bool active() const { return active_; }
    bool isHost() const { return isHost_; }
    Slot self() const { return self_; }

    // Guest: the link to the host is up (joined, not in the baseline transfer). Also after every reconnect: the host learns the name.
    void linkUp();
    // Host: a guest joined / left (compared each frame by the session adapter).
    void playerJoined(Slot slot);
    void playerLeft(Slot slot, uint64_t nowMs);

    // The local player says something (raw typed text).
    SayResult say(std::string_view raw, uint64_t nowMs);
    // A system line. Host with broadcast: every machine shows it; otherwise only this one.
    void system(std::string_view text, bool broadcast, uint64_t nowMs);
    // A line that only this machine shows (feedback to the player: "slow down", "not in a session"). Not stored for repeats, not sent.
    void notice(std::string_view text, uint64_t nowMs);
    // One datagram of the chat channel from `from` (host: a joined guest's slot; guest: the host, slot 0).
    void onDatagram(Slot from, const uint8_t* data, size_t size, uint64_t nowMs);

    ChatLog& log() { return log_; }
    const ChatLog& log() const { return log_; }
    const Config& config() const { return cfg_; }
    std::string selfName() const;
    std::string nameOf(Slot slot) const;

    struct Stats {
        uint64_t said = 0, relayed = 0, received = 0, rateLimited = 0, duplicates = 0, undecodable = 0, unknownSender = 0, ownEchoes = 0, joinsAnnounced = 0, leavesAnnounced = 0;
    };
    const Stats& stats() const { return stats_; }

private:
    std::string defaultName(Slot s) const { return "Player " + std::to_string(unsigned(s) + 1); }
    std::string uniqueName(Slot slot, std::string base) const;
    void push(Line l);
    void broadcast(const Bytes& data, Slot except);
    bool isJoined(Slot s) const;

    Config cfg_;
    Ports ports_;
    std::function<void(const Line&)> onLine_;
    ChatLog log_;
    RateLimiter limiter_;
    bool active_ = false, isHost_ = false;
    Slot self_ = kSystemSlot;
    uint32_t nextId_;
    std::map<Slot, std::string> names_;       // host: the name of every joined guest (and slot 0)
    std::set<Slot> announced_;                // host: guests whose "joined" line went out
    Stats stats_;
};

}  // namespace nvcoop::chat
