// Text chat between the players of a co-op session (plugin/chat). Game-free core: everything in this folder except the files named
// *_win / *_module / *_engine compiles and is tested without the game (plugin\chat\run_tests.cmd).
//
// Shapes shared by the core files:
//   * a chat LINE is who said it (a slot, or kSystemSlot for a system line), the name to show, and the text. Every text that came from
//     another PC is run through sanitizeText again here, whatever the sender did.
//   * the host is the relay: a guest sends Say to the host, the host sends Line to every other joined guest (the sender shows its own
//     line at once). System lines (joins, leaves, party saves) are made by the host and sent as Line with kSystemSlot.
#pragma once
#include <cstdint>
#include <functional>
#include <string>
#include <vector>

namespace nvcoop::chat {

using Bytes = std::vector<uint8_t>;
using Slot = uint8_t;

constexpr uint8_t kChannelChat = 0xC8;       // first byte of every datagram of this module (docs/protocol.md section 2)
constexpr uint8_t kWireVersion = 1;
constexpr Slot kSystemSlot = 0xFF;           // a line the session made, not a player
constexpr Slot kHostSlot = 0;

constexpr size_t kMaxTextBytes = 160;        // one chat line, after sanitising
constexpr size_t kMaxNameBytes = 24;         // a display name, after sanitising
constexpr size_t kMaxDatagramBytes = 3 + 1 + 4 + 1 + 1 + kMaxNameBytes + 1 + kMaxTextBytes;   // the largest datagram the decoder accepts

// What the player can change in nvcoop.ini [chat] (chat_config.h reads it).
struct Config {
    bool enabled = true;
    int openKey = 'Y';                       // a Windows virtual key (the ini takes a name or a number)
    std::string name;                        // "" = the session names the player ("Player 2")
    std::string display = "auto";            // auto | hud | overlay
    uint32_t lineSeconds = 8;                // how long a new line stays on screen
    uint32_t visibleLines = 6;               // lines on screen together while not typing
    uint32_t historyLines = 40;              // lines kept (and shown, the newest ones, while typing)
    uint32_t rateLines = 5;                  // a sender may say this many lines ...
    uint32_t rateSeconds = 10;               // ... in this many seconds
};

enum class LineKind : uint8_t { Player = 0, System = 1 };

struct Line {
    uint32_t seq = 0;                        // this machine's running number (order of arrival), never on the wire
    Slot slot = kSystemSlot;                 // who said it (the origin slot), kSystemSlot for a system line
    uint32_t msgId = 0;                      // the sender's own running number (for de-duplication)
    LineKind kind = LineKind::System;
    std::string name;                        // sanitised; empty for a system line
    std::string text;                        // sanitised
    uint64_t atMs = 0;                       // when it reached this machine
};

}  // namespace nvcoop::chat
