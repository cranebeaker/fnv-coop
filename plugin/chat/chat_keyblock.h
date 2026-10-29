// Keeps the game from acting on the keys the player types into the chat line. The game reads the keyboard through DirectInput (window messages do not
// reach it), so while the line is open every key the game could use is disabled with xNVSE's own commands:
//     DisableKey <scancode> 2      EnableKey <scancode> 2
// (mask 2 = the script flag only: keys the player disabled in xNVSE's own ini stay as they are). Scancodes 1..255 are the keyboard, 256..263 the mouse
// buttons (no shooting while typing). The lines are spread over a few frames (perFrame at a time) because each one is a console script; the first batch
// has Escape, WASD, Tab, Enter, Space and the modifiers. Closing the line runs the same walk the other way and only touches keys this class disabled.
#pragma once
#include <cstdint>
#include <functional>
#include <string>
#include <vector>

namespace nvcoop::chat {

class KeyBlocker {
public:
    using Exec = std::function<bool(const std::string& consoleLine)>;
    static constexpr int kFirstKey = 1, kLastKey = 263;
    explicit KeyBlocker(Exec exec = {}, int perFrame = 70) : exec_(std::move(exec)), perFrame_(perFrame > 0 ? perFrame : 1), disabled_(kLastKey + 1, false) {}

    void want(bool blocked) { if (blocked != want_) { want_ = blocked; failed_ = false; failures_ = 0; } }
    void frame();                              // runs at most perFrame console lines
    bool settled() const;                      // what is wanted is what is applied
    int disabledCount() const;
    bool failed() const { return failed_; }    // the console refused several lines in a row: disabling is given up until want() changes; enabling waits a while and walks again
    uint64_t linesRun() const { return lines_; }

private:
    Exec exec_;
    int perFrame_;
    bool want_ = false, failed_ = false;
    int failures_ = 0, cooldown_ = 0;
    std::vector<bool> disabled_;
    uint64_t lines_ = 0;
};

}  // namespace nvcoop::chat
