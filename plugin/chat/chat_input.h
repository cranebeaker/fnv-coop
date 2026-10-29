// Window messages -> chat input events, with the decision "does the game see this message". Game-free and Windows-header-free: the message numbers are
// constants below, so the tests feed synthetic messages. The Windows glue (chat_hook_win.cpp) calls onMessage from a WH_GETMESSAGE hook on the game thread.
//
//   * not typing: the open key (key down, not an auto-repeat, no Ctrl / Alt, and canOpen() says no menu is up) opens the line. The key down is swallowed so
//     the game's window code never sees it; no character follows it (TranslateMessage on a swallowed key down makes none).
//   * typing: every key message is swallowed. Enter, Escape, Backspace, Delete, the arrows, Home, End become Key events from their key down. A key down that
//     makes a character is swallowed too, and the glue calls TranslateMessage on the original so the character arrives as a WM_CHAR (printable ones become Char
//     events). Ctrl+V becomes a Paste event. Alt combinations (Alt+Tab, Alt+F4) are left to Windows.
//   The game keeps reading the keyboard through DirectInput, which window messages do not reach: chat_keyblock.h disables those keys while typing.
#pragma once
#include <cstdint>
#include <functional>
#include <mutex>
#include <string>
#include <vector>

#include "chat/line_editor.h"

namespace nvcoop::chat {

constexpr uint32_t kWmKeyDown = 0x0100, kWmKeyUp = 0x0101, kWmChar = 0x0102, kWmSysKeyDown = 0x0104, kWmSysKeyUp = 0x0105, kWmSysChar = 0x0106;
constexpr int kVkBack = 0x08, kVkReturn = 0x0D, kVkShift = 0x10, kVkControl = 0x11, kVkMenu = 0x12, kVkEscape = 0x1B, kVkEnd = 0x23, kVkHome = 0x24, kVkLeft = 0x25,
              kVkUp = 0x26, kVkRight = 0x27, kVkDown = 0x28, kVkDelete = 0x2E;

enum class InputType : uint8_t { Open, Char, Key, Paste, Cancel };
struct InputEvent {
    InputType type = InputType::Char;
    unsigned char ch = 0;        // Char
    Key key = Key::Left;         // Key
};

class InputTranslator {
public:
    enum class Verdict : uint8_t { Pass, Swallow, SwallowAndTranslate };
    struct Deps {
        std::function<bool()> canOpen;           // no menu on screen and a session runs (null = always)
        std::function<bool(int vk)> keyDown;     // is this virtual key held right now (GetKeyState)
    };
    explicit InputTranslator(Deps d = {}) : deps_(std::move(d)) {}

    void setOpenKey(int vk);
    void setEnabled(bool on);
    // The controller's state, once per frame: false closes the typing state here too (a menu opened, the line was sent through the bridge).
    void setComposing(bool on);
    bool composing() const;

    Verdict onMessage(uint32_t message, uint64_t wParam, int64_t lParam);
    void drain(std::vector<InputEvent>& out);    // takes the queued events
    uint64_t swallowed() const;

private:
    bool held(int vk) const { return deps_.keyDown && deps_.keyDown(vk); }
    Deps deps_;
    mutable std::mutex mu_;
    int openKey_ = 'Y';
    bool enabled_ = true, composing_ = false;
    std::vector<InputEvent> events_;
    uint64_t swallowed_ = 0;
};

}  // namespace nvcoop::chat
