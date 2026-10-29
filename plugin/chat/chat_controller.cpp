#include "chat/chat_controller.h"

#include "fnvcoop_mut.h"

// Twins: 1100 = a menu that opens does not close the line; 1101 = a rate-limited line is dropped without telling the player; 1102 = Open is taken while a line
//        is already open (the text typed so far is lost); 1103 = Open is taken outside a session; 1104 = the line is sent even when Escape was pressed;
//        1105 = a Paste is ignored; 1106 = Open is taken while a menu is up; 1107 = the player is not told when there is no session
namespace nvcoop::chat {

void ChatController::open(uint64_t nowMs) {
    if (!core_.config().enabled) return;
    if (editor_.isOpen() && !FNV_MUT(1102)) return;
    if (!FNV_MUT(1106) && deps_.menuOpen && deps_.menuOpen()) return;
    if (!core_.active() && !FNV_MUT(1103)) {
        if (!FNV_MUT(1107)) core_.notice("Chat works while you are in a co-op session.", nowMs);
        return;
    }
    editor_.open();
    ++opened_;
}

void ChatController::submit(uint64_t nowMs) {
    const std::string text = editor_.take();
    if (text.empty()) return;
    switch (core_.say(text, nowMs)) {
        case SayResult::Sent: ++sent_; break;
        case SayResult::RateLimited:
            if (!FNV_MUT(1101)) core_.notice("You are sending messages too fast. Wait a moment.", nowMs);
            break;
        case SayResult::NoSession: core_.notice("Not in a co-op session. Nobody heard that.", nowMs); break;
        case SayResult::Empty: break;
    }
}

void ChatController::frame(uint64_t nowMs, const std::vector<InputEvent>& events) {
    if (editor_.isOpen() && !FNV_MUT(1100) && deps_.menuOpen && deps_.menuOpen()) {
        editor_.close();
        ++cancelled_;
    }
    for (const InputEvent& e : events) {
        switch (e.type) {
            case InputType::Open: open(nowMs); break;
            case InputType::Char: editor_.typeChar(e.ch); break;
            case InputType::Paste:
                if (deps_.clipboard && !FNV_MUT(1105)) editor_.insertText(deps_.clipboard());
                break;
            case InputType::Cancel:
                if (editor_.isOpen()) { editor_.close(); ++cancelled_; }
                break;
            case InputType::Key: {
                const std::string before = editor_.text();
                const EditResult r = editor_.key(e.key);
                if (r == EditResult::Send) submit(nowMs);
                else if (r == EditResult::Cancel) {
                    if (FNV_MUT(1104) && !before.empty()) core_.say(before, nowMs);   // twin: Escape sends what was typed
                    editor_.close();
                    ++cancelled_;
                }
                break;
            }
        }
    }
}

}  // namespace nvcoop::chat
