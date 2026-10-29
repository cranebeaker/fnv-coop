// What the chat looks like at one moment, as plain text lines: the surfaces (HUD message queue, overlay window, or a test) only draw this.
//   not typing : the newest `visibleLines` lines that arrived less than `ttlMs` ago; the last second fades out
//   typing     : the newest `typingHistory` lines whatever their age (the short history), then the line being typed with its caret
#pragma once
#include <string>
#include <vector>

#include "chat/chat_log.h"
#include "chat/chat_types.h"
#include "chat/line_editor.h"

namespace nvcoop::chat {

struct ViewLine {
    std::string text;
    uint8_t alpha = 255;
    bool system = false;
    bool operator==(const ViewLine& o) const { return text == o.text && alpha == o.alpha && system == o.system; }
};

struct View {
    std::vector<ViewLine> lines;     // oldest first, already wrapped to the column width
    bool composing = false;
    std::string compose;             // "> hello|" when composing
    bool empty() const { return lines.empty() && !composing; }
    bool operator==(const View& o) const { return composing == o.composing && compose == o.compose && lines == o.lines; }
};

struct ViewOptions {
    uint32_t ttlMs = 8000;
    uint32_t visibleLines = 6;
    uint32_t typingHistory = 10;     // lines shown while typing
    uint32_t fadeMs = 1500;          // the last stretch of a line's time is a fade
    size_t columns = 72;             // wrap width
};

std::string formatLine(const Line& l);          // "Name: text", or "* text" for a system line
View buildView(const ChatLog& log, const LineEditor& editor, uint64_t nowMs, const ViewOptions& opt);
ViewOptions viewOptions(const Config& c);

}  // namespace nvcoop::chat
