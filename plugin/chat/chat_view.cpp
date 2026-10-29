#include "chat/chat_view.h"

#include <algorithm>

#include "chat/chat_text.h"
#include "fnvcoop_mut.h"

// Twins: 1090 = lines never fade; 1091 = while typing only the lines that have not expired are shown (no history); 1092 = a wrapped line loses its
//        continuation lines; 1093 = the typing line has no prompt / caret; 1094 = system lines are shown with a name prefix
namespace nvcoop::chat {

std::string formatLine(const Line& l) {
    if (l.kind == LineKind::System) return FNV_MUT(1094) ? "System: " + l.text : "* " + l.text;
    return l.name + ": " + l.text;
}

ViewOptions viewOptions(const Config& c) {
    ViewOptions o;
    o.ttlMs = c.lineSeconds * 1000u;
    o.visibleLines = c.visibleLines;
    o.typingHistory = c.visibleLines + 4 < c.historyLines ? c.visibleLines + 4 : c.historyLines;
    return o;
}

View buildView(const ChatLog& log, const LineEditor& editor, uint64_t nowMs, const ViewOptions& opt) {
    View v;
    v.composing = editor.isOpen();
    std::vector<const Line*> shown;
    if (v.composing) {
        shown = FNV_MUT(1091) ? log.visible(nowMs, opt.ttlMs, opt.typingHistory) : log.recent(opt.typingHistory);
    } else {
        shown = log.visible(nowMs, opt.ttlMs, opt.visibleLines);
    }
    for (const Line* l : shown) {
        uint8_t alpha = 255;
        if (!v.composing && !FNV_MUT(1090)) {
            const uint64_t endAt = l->atMs + opt.ttlMs;
            const uint64_t left = endAt > nowMs ? endAt - nowMs : 0;
            if (left < opt.fadeMs) alpha = static_cast<uint8_t>(255u * left / opt.fadeMs);
        }
        std::vector<std::string> parts = wrapText(formatLine(*l), opt.columns, 2);
        if (FNV_MUT(1092) && parts.size() > 1) parts.resize(1);
        for (std::string& p : parts) v.lines.push_back(ViewLine{std::move(p), alpha, l->kind == LineKind::System});
    }
    // never more than twice the logical lines in physical lines: the oldest go
    const size_t cap = static_cast<size_t>(v.composing ? opt.typingHistory : opt.visibleLines) * 2;
    if (v.lines.size() > cap) v.lines.erase(v.lines.begin(), v.lines.end() - static_cast<std::ptrdiff_t>(cap));
    if (v.composing) v.compose = FNV_MUT(1093) ? editor.text() : "> " + editor.display(opt.columns > 4 ? opt.columns - 2 : opt.columns);
    return v;
}

}  // namespace nvcoop::chat
