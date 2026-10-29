// The overlay surface: one click-through window owned by the game window, drawn with GDI, showing the chat View as a dark panel at the bottom left of the
// game's client area. Windows only (no game code). It sits above the game window in windowed and borderless modes; in exclusive full screen a window cannot
// show over the game, which is why display=auto picks the HUD surface there (chat_module.cpp). Nothing in it takes the focus or the mouse.
#pragma once
#include <windows.h>

#include <memory>

#include "chat/chat_surface.h"

namespace nvcoop::chat {

struct OverlayFont {
    HFONT font = nullptr;
    int lineH = 16, charW = 8;
};
OverlayFont makeOverlayFont(HDC dc, int clientH);
void releaseOverlayFont(OverlayFont& f);
// Draws the panel into `dc` with its top left at (0, 0): background, border, the lines (faded by their alpha), the typing row.
void renderPanel(HDC dc, const View& v, const OverlayLayout& lay, const OverlayFont& font);

class OverlayWindow : public IChatSurface {
public:
    OverlayWindow() = default;
    ~OverlayWindow() override { destroy(); }
    OverlayWindow(const OverlayWindow&) = delete;
    OverlayWindow& operator=(const OverlayWindow&) = delete;

    bool create(HWND owner);                  // false: no window could be made (the module falls back to the HUD)
    void destroy();
    bool created() const { return hwnd_ != nullptr; }
    HWND hwnd() const { return hwnd_; }

    const char* name() const override { return "overlay"; }
    void onLine(const Line&, uint64_t) override {}
    void present(const View& v, uint64_t nowMs) override;
    void hide() override;

    void paint(HDC dc);                       // WM_PAINT
    uint64_t shown() const { return shown_; }

private:
    HWND hwnd_ = nullptr;
    HWND owner_ = nullptr;
    View last_;
    OverlayLayout lay_;
    OverlayFont font_;
    int clientW_ = 0, clientH_ = 0, originX_ = 0, originY_ = 0;
    bool visible_ = false;
    uint64_t shown_ = 0;
};

}  // namespace nvcoop::chat
