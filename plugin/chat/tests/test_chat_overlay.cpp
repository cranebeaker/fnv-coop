// The overlay window (Windows only, no game): the panel is drawn into a bitmap and looked at pixel by pixel; the window is made over a dummy owner window.
// Twins 1150, 1151. Headless: no window is ever shown on the desktop; a machine with no window station skips the window test (it says so).
#include <algorithm>
#include <cstdio>
#include <string>

#include "chat/chat_overlay_win.h"
#include "minitest.h"

using namespace nvcoop::chat;

namespace {

struct Bitmap {
    HDC dc = nullptr;
    HBITMAP bmp = nullptr, old = nullptr;
    void* bits = nullptr;
    int w = 0, h = 0;
    Bitmap(int width, int height) : w(width), h(height) {
        HDC screen = GetDC(nullptr);
        dc = CreateCompatibleDC(screen);
        ReleaseDC(nullptr, screen);
        BITMAPINFO bi{};
        bi.bmiHeader.biSize = sizeof bi.bmiHeader;
        bi.bmiHeader.biWidth = w;
        bi.bmiHeader.biHeight = -h;
        bi.bmiHeader.biPlanes = 1;
        bi.bmiHeader.biBitCount = 32;
        bi.bmiHeader.biCompression = BI_RGB;
        bmp = CreateDIBSection(dc, &bi, DIB_RGB_COLORS, &bits, nullptr, 0);
        old = static_cast<HBITMAP>(SelectObject(dc, bmp));
    }
    ~Bitmap() { SelectObject(dc, old); DeleteObject(bmp); DeleteDC(dc); }
    uint32_t px(int x, int y) const { return static_cast<uint32_t*>(bits)[y * w + x] & 0xFFFFFF; }
    // the brightest channel found in rows [y0, y1) (a row of text drawn bright has a high one, a faded row a low one, an empty row 0)
    int brightest(int y0, int y1, int x0 = 3, int x1 = -1) const {
        int best = 0;
        for (int y = y0; y < y1 && y < h; ++y)
            for (int x = x0; x < (x1 < 0 ? w - 3 : x1) && x < w; ++x) {
                const uint32_t p = px(x, y);
                best = std::max({best, int(p & 0xFF), int((p >> 8) & 0xFF), int((p >> 16) & 0xFF)});
            }
        return best;
    }
};

View twoLines() {
    View v;
    v.lines.push_back(ViewLine{"Ann: hello there", 255, false});
    v.lines.push_back(ViewLine{"Bob: fading out", 40, false});
    v.lines.push_back(ViewLine{"* Cy joined", 255, true});
    return v;
}

}  // namespace

TEST(chat_overlay, the_panel_is_drawn_with_text_and_a_fade) {
    HDC screen = GetDC(nullptr);
    OverlayFont font = makeOverlayFont(screen, 720);
    ReleaseDC(nullptr, screen);
    REQUIRE(font.font != nullptr);
    CHECK(font.lineH >= 12 && font.charW >= 4);
    View v = twoLines();
    OverlayLayout lay = layoutOverlay(v, 1280, 720, font.lineH, font.charW);
    REQUIRE(lay.visible());
    Bitmap b(lay.w, lay.h);
    renderPanel(b.dc, v, lay, font);
    GdiFlush();
    CHECK(b.px(lay.w / 2, lay.h - 2) == 0);   // empty background below the text is black (the border is on the outer pixel)
    const int row0 = b.brightest(lay.rowY[0], lay.rowY[0] + font.lineH);
    const int row1 = b.brightest(lay.rowY[1], lay.rowY[1] + font.lineH);
    const int row2 = b.brightest(lay.rowY[2], lay.rowY[2] + font.lineH);
    CHECK_MSG(row0 > 150, std::to_string(row0));
    CHECK_MSG(row1 > 0 && row1 < row0 / 2, std::to_string(row1) + " vs " + std::to_string(row0));   // faded: dimmer, but there
    CHECK_MSG(row2 > 150, std::to_string(row2));
    // a system line is amber, a player line green-white: the blue channel of the system line is lower than its red one
    uint32_t sysBest = 0;
    for (int x = 3; x < lay.w - 3; ++x)
        for (int y = lay.rowY[2]; y < lay.rowY[2] + font.lineH; ++y) sysBest = std::max(sysBest, b.px(x, y) & 0xFF);
    CHECK(sysBest < 200);
    releaseOverlayFont(font);
}

TEST(chat_overlay, the_typing_row_is_drawn_last_and_bright) {
    HDC screen = GetDC(nullptr);
    OverlayFont font = makeOverlayFont(screen, 720);
    ReleaseDC(nullptr, screen);
    View v;
    v.lines.push_back(ViewLine{"Ann: hi", 255, false});
    v.composing = true;
    v.compose = "> typing here|";
    OverlayLayout lay = layoutOverlay(v, 1280, 720, font.lineH, font.charW);
    REQUIRE(lay.rowY.size() == 2);
    Bitmap b(lay.w, lay.h);
    renderPanel(b.dc, v, lay, font);
    GdiFlush();
    CHECK(b.brightest(lay.rowY[1], lay.rowY[1] + font.lineH) > 200);
    releaseOverlayFont(font);
}

TEST(chat_overlay, the_window_never_takes_the_mouse_or_the_focus) {
    // Headless: the owner and the overlay are made but never shown (no window appears on the desktop, nothing takes the focus).
    HWND owner = CreateWindowExW(0, L"STATIC", L"chat overlay test owner", WS_OVERLAPPEDWINDOW, 100, 100, 800, 600, nullptr, nullptr, GetModuleHandleW(nullptr), nullptr);
    if (!owner) { std::printf("    (no window station: window test skipped)\n"); return; }
    {
        OverlayWindow o;
        REQUIRE(o.create(owner));
        CHECK(o.created());
        CHECK(!IsWindowVisible(o.hwnd()));
        o.present(View{}, 0);                 // nothing to show: stays hidden
        CHECK(!IsWindowVisible(o.hwnd()));
        CHECK(SendMessageW(o.hwnd(), WM_NCHITTEST, 0, MAKELPARAM(50, 50)) == HTTRANSPARENT);
        CHECK(SendMessageW(o.hwnd(), WM_MOUSEACTIVATE, 0, 0) == MA_NOACTIVATE);
        const LONG ex = GetWindowLongW(o.hwnd(), GWL_EXSTYLE);
        CHECK((ex & WS_EX_NOACTIVATE) && (ex & WS_EX_TRANSPARENT) && (ex & WS_EX_LAYERED));
        CHECK((GetWindowLongW(o.hwnd(), GWL_STYLE) & WS_POPUP) != 0);
        CHECK(GetWindow(o.hwnd(), GW_OWNER) == owner);
        // with the owner not shown, a view with content still shows nothing (a hidden or minimized game gets no chat on the desktop)
        o.present(twoLines(), 1);
        CHECK(!IsWindowVisible(o.hwnd()));
        o.destroy();
        CHECK(!o.created());
    }
    DestroyWindow(owner);
}

TEST(chat_overlay, a_missing_owner_is_refused) {
    OverlayWindow o;
    CHECK(!o.create(nullptr));
    CHECK(!o.created());
    o.present(twoLines(), 1);   // no crash
}
