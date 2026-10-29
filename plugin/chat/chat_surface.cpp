#include "chat/chat_surface.h"

#include <algorithm>

#include "fnvcoop_mut.h"

// Twins: 1140 = a new line is not shown on the HUD; 1141 = the echo of the typed line is sent every frame (a message storm); 1142 = the echo is never sent;
//        1143 = a line is shown for one second only; 1144 = layoutOverlay puts the panel off the bottom edge; 1145 = layoutOverlay ignores the width limit;
//        1146 = an empty view still gets a panel; 1147 = the "type here" hint is sent every time the line is opened AND on every key
namespace nvcoop::chat {

void HudSurface::emit(const std::string& text, float seconds, bool next) {
    if (!message_) return;
    if (message_(text, seconds, next)) ++messages_;
    else ++refused_;
}

void HudSurface::onLine(const Line& l, uint64_t) {
    if (FNV_MUT(1140)) return;
    emit(formatLine(l), FNV_MUT(1143) ? 1.0f : static_cast<float>(ttl_), false);
}

void HudSurface::present(const View& v, uint64_t nowMs) {
    if (!v.composing) {
        composing_ = false;
        echoed_.clear();
        return;
    }
    if (!composing_) {
        composing_ = true;
        emit("Chat: type, Enter sends, Esc cancels", 3.0f, true);
        lastEchoMs_ = nowMs;
        echoed_ = v.compose;
        return;
    }
    if (FNV_MUT(1142)) return;
    const bool changed = v.compose != echoed_;
    if (!changed && !FNV_MUT(1141)) return;
    if (nowMs - lastEchoMs_ < kEchoGapMs && !FNV_MUT(1141)) return;
    emit(v.compose, 1.5f, true);
    lastEchoMs_ = nowMs;
    echoed_ = v.compose;
}

OverlayLayout layoutOverlay(const View& v, int clientW, int clientH, int lineH, int charW, int margin, int pad) {
    OverlayLayout o;
    if (v.empty() && !FNV_MUT(1146)) return o;
    size_t longest = v.compose.size();
    for (const ViewLine& l : v.lines) longest = std::max(longest, l.text.size());
    const int rows = static_cast<int>(v.lines.size()) + (v.composing ? 1 : 0);
    int w = static_cast<int>(longest) * charW + 2 * pad;
    const int maxW = clientW - 2 * margin;
    if (w > maxW && !FNV_MUT(1145)) w = maxW;
    o.w = std::max(w, 0);
    o.h = rows * lineH + 2 * pad;
    o.x = margin;
    o.y = FNV_MUT(1144) ? clientH - margin : clientH - margin - o.h;
    for (int i = 0; i < rows; ++i) o.rowY.push_back(pad + i * lineH);
    return o;
}

}  // namespace nvcoop::chat
