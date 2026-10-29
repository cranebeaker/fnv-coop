// Where the chat is drawn. Two surfaces, both fed the same View / new lines by the module:
//   HudSurface     the game's own HUD message line (works in every display mode, also full screen): each new line is one HUD message that stays `ttl` seconds;
//                  the line being typed is echoed as a short message whenever it changed (at most every kEchoGapMs), because a HUD message cannot be edited.
//   OverlaySurface a click-through window over the game's window with the whole View drawn as a panel (chat_overlay_win.cpp; only where a window can sit
//                  over the game: windowed and borderless modes). layoutOverlay is the part of it that needs no window.
// HudSurface here is game-free (the HUD call comes in as a function); the overlay's window is Windows-only.
#pragma once
#include <functional>
#include <string>
#include <vector>

#include "chat/chat_view.h"

namespace nvcoop::chat {

class IChatSurface {
public:
    virtual ~IChatSurface() = default;
    virtual const char* name() const = 0;
    virtual void onLine(const Line& l, uint64_t nowMs) = 0;           // a line reached the history
    virtual void present(const View& v, uint64_t nowMs) = 0;          // once per frame
    virtual void hide() = 0;                                          // the session or the chat ended
};

constexpr uint64_t kEchoGapMs = 700;

class HudSurface : public IChatSurface {
public:
    // message(text, seconds, next): one HUD message; false = the engine refused it (a pin that does not match this exe).
    using MessageFn = std::function<bool(const std::string& text, float seconds, bool next)>;
    HudSurface(MessageFn message, uint32_t ttlSeconds) : message_(std::move(message)), ttl_(ttlSeconds) {}
    const char* name() const override { return "hud"; }
    void onLine(const Line& l, uint64_t nowMs) override;
    void present(const View& v, uint64_t nowMs) override;
    void hide() override { composing_ = false; echoed_.clear(); }
    uint64_t messages() const { return messages_; }
    uint64_t refused() const { return refused_; }

private:
    void emit(const std::string& text, float seconds, bool next);
    MessageFn message_;
    uint32_t ttl_;
    bool composing_ = false;
    std::string echoed_;
    uint64_t lastEchoMs_ = 0, messages_ = 0, refused_ = 0;
};

// The panel of the overlay in client pixels: where it goes and where each text row starts. Bottom left, `margin` from the edges, as wide as the longest
// row (at most the window less the margins), rows `lineH` high. Empty view = a zero-size rect.
struct OverlayLayout {
    int x = 0, y = 0, w = 0, h = 0;
    std::vector<int> rowY;           // top of each row inside the panel (log rows, then the typing row)
    bool visible() const { return w > 0 && h > 0; }
};
OverlayLayout layoutOverlay(const View& v, int clientW, int clientH, int lineH, int charW, int margin = 16, int pad = 6);

}  // namespace nvcoop::chat
