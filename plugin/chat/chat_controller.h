// ChatController: the player's side of the chat. Takes the input events, runs the line editor, hands a finished line to the ChatCore, and answers what the
// surfaces ask for (the View) and what the key blocker needs to know (is a line open). Game-free; the game comes in through Deps.
#pragma once
#include <functional>
#include <string>
#include <vector>

#include "chat/chat_core.h"
#include "chat/chat_input.h"
#include "chat/chat_view.h"
#include "chat/line_editor.h"

namespace nvcoop::chat {

struct ControllerDeps {
    std::function<bool()> menuOpen;                // any game menu on screen (Pip-Boy, console, dialogue, loading ...)
    std::function<std::string()> clipboard;        // the text on the clipboard (optional)
};

class ChatController {
public:
    ChatController(ChatCore& core, ControllerDeps deps) : core_(core), deps_(std::move(deps)) {}

    // Once per frame: the events the input translator collected, in order. A menu that opens closes the line.
    void frame(uint64_t nowMs, const std::vector<InputEvent>& events);
    // Open the line without a key (the bridge method chat.open).
    void open(uint64_t nowMs);
    bool composing() const { return editor_.isOpen(); }
    const LineEditor& editor() const { return editor_; }
    View view(uint64_t nowMs) const { return buildView(core_.log(), editor_, nowMs, viewOptions(core_.config())); }

    uint64_t opened() const { return opened_; }
    uint64_t sent() const { return sent_; }
    uint64_t cancelled() const { return cancelled_; }

private:
    void submit(uint64_t nowMs);
    ChatCore& core_;
    ControllerDeps deps_;
    LineEditor editor_;
    uint64_t opened_ = 0, sent_ = 0, cancelled_ = 0;
};

}  // namespace nvcoop::chat
