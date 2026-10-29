#include "chat/chat_overlay_win.h"

#include <algorithm>

#include "fnvcoop_mut.h"

// Twins: 1150 = the window takes the mouse (no HTTRANSPARENT); 1151 = a faded line is drawn at full brightness
namespace nvcoop::chat {

namespace {

const wchar_t* kClass = L"NVCoopChatOverlay";
constexpr COLORREF kBack = RGB(0, 0, 0), kPlayer = RGB(190, 255, 200), kSystem = RGB(255, 214, 120), kTyping = RGB(255, 255, 255), kBorder = RGB(60, 120, 70);

COLORREF fade(COLORREF c, uint8_t alpha) {
    if (FNV_MUT(1151)) return c;
    const int a = alpha;
    return RGB(GetRValue(c) * a / 255, GetGValue(c) * a / 255, GetBValue(c) * a / 255);   // towards the black of the panel
}

LRESULT CALLBACK OverlayProc(HWND h, UINT m, WPARAM w, LPARAM l) {
    OverlayWindow* self = reinterpret_cast<OverlayWindow*>(GetWindowLongPtrA(h, GWLP_USERDATA));
    switch (m) {
        case WM_NCHITTEST: return FNV_MUT(1150) ? HTCLIENT : HTTRANSPARENT;
        case WM_MOUSEACTIVATE: return MA_NOACTIVATE;
        case WM_ERASEBKGND: return 1;
        case WM_PAINT: {
            PAINTSTRUCT ps;
            HDC dc = BeginPaint(h, &ps);
            if (self) self->paint(dc);
            EndPaint(h, &ps);
            return 0;
        }
        default: return DefWindowProcW(h, m, w, l);
    }
}

bool registerClass() {
    static bool done = false, ok = false;
    if (done) return ok;
    done = true;
    WNDCLASSW wc{};
    wc.lpfnWndProc = OverlayProc;
    wc.hInstance = GetModuleHandleW(nullptr);
    wc.lpszClassName = kClass;
    wc.hCursor = LoadCursorA(nullptr, IDC_ARROW);
    ok = RegisterClassW(&wc) != 0 || GetLastError() == ERROR_CLASS_ALREADY_EXISTS;
    return ok;
}

}  // namespace

OverlayFont makeOverlayFont(HDC dc, int clientH) {
    OverlayFont f;
    const int px = std::clamp(clientH / 42, 14, 26);
    f.font = CreateFontW(-px, 0, 0, 0, FW_NORMAL, FALSE, FALSE, FALSE, ANSI_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS, ANTIALIASED_QUALITY,
                         FIXED_PITCH | FF_MODERN, L"Consolas");
    HGDIOBJ old = SelectObject(dc, f.font);
    TEXTMETRICW tm{};
    GetTextMetricsW(dc, &tm);
    SelectObject(dc, old);
    f.lineH = tm.tmHeight + 2;
    f.charW = tm.tmAveCharWidth > 0 ? tm.tmAveCharWidth : px / 2;
    return f;
}

void releaseOverlayFont(OverlayFont& f) {
    if (f.font) DeleteObject(f.font);
    f.font = nullptr;
}

void renderPanel(HDC dc, const View& v, const OverlayLayout& lay, const OverlayFont& font) {
    RECT r{0, 0, lay.w, lay.h};
    HBRUSH back = CreateSolidBrush(kBack);
    FillRect(dc, &r, back);
    DeleteObject(back);
    HBRUSH edge = CreateSolidBrush(kBorder);
    FrameRect(dc, &r, edge);
    DeleteObject(edge);
    HGDIOBJ old = SelectObject(dc, font.font);
    SetBkMode(dc, TRANSPARENT);
    const int pad = lay.rowY.empty() ? 6 : lay.rowY[0];
    for (size_t i = 0; i < v.lines.size() && i < lay.rowY.size(); ++i) {
        const ViewLine& l = v.lines[i];
        SetTextColor(dc, fade(l.system ? kSystem : kPlayer, l.alpha));
        TextOutA(dc, pad, lay.rowY[i], l.text.data(), static_cast<int>(l.text.size()));
    }
    if (v.composing && lay.rowY.size() > v.lines.size()) {
        SetTextColor(dc, kTyping);
        TextOutA(dc, pad, lay.rowY[v.lines.size()], v.compose.data(), static_cast<int>(v.compose.size()));
    }
    SelectObject(dc, old);
}

bool OverlayWindow::create(HWND owner) {
    if (hwnd_) return true;
    if (!owner || !IsWindow(owner) || !registerClass()) return false;
    hwnd_ = CreateWindowExW(WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_NOACTIVATE | WS_EX_TOOLWINDOW, kClass, L"", WS_POPUP, 0, 0, 10, 10, owner, nullptr,
                            GetModuleHandleW(nullptr), nullptr);
    if (!hwnd_) return false;
    owner_ = owner;
    SetWindowLongPtrA(hwnd_, GWLP_USERDATA, reinterpret_cast<LONG_PTR>(this));
    SetLayeredWindowAttributes(hwnd_, 0, 225, LWA_ALPHA);
    return true;
}

void OverlayWindow::destroy() {
    if (hwnd_) {
        SetWindowLongPtrA(hwnd_, GWLP_USERDATA, 0);
        DestroyWindow(hwnd_);
    }
    hwnd_ = nullptr;
    owner_ = nullptr;
    releaseOverlayFont(font_);
    visible_ = false;
    last_ = View{};
}

void OverlayWindow::hide() {
    if (hwnd_ && visible_) ShowWindow(hwnd_, SW_HIDE);
    visible_ = false;
    last_ = View{};
}

void OverlayWindow::present(const View& v, uint64_t) {
    if (!hwnd_) return;
    if (v.empty() || !IsWindow(owner_) || IsIconic(owner_) || !IsWindowVisible(owner_)) {
        hide();
        return;
    }
    RECT rc{};
    GetClientRect(owner_, &rc);
    POINT origin{0, 0};
    ClientToScreen(owner_, &origin);
    const int cw = rc.right - rc.left, ch = rc.bottom - rc.top;
    if (cw <= 0 || ch <= 0) { hide(); return; }
    const bool moved = origin.x != originX_ || origin.y != originY_ || cw != clientW_ || ch != clientH_;
    if (!moved && visible_ && v == last_) return;
    if (!font_.font || ch != clientH_) {
        releaseOverlayFont(font_);
        HDC dc = GetDC(hwnd_);
        font_ = makeOverlayFont(dc, ch);
        ReleaseDC(hwnd_, dc);
    }
    clientW_ = cw; clientH_ = ch;
    originX_ = origin.x; originY_ = origin.y;
    lay_ = layoutOverlay(v, cw, ch, font_.lineH, font_.charW);
    last_ = v;
    if (!lay_.visible()) { hide(); return; }
    SetWindowPos(hwnd_, HWND_TOP, originX_ + lay_.x, originY_ + lay_.y, lay_.w, lay_.h, SWP_NOACTIVATE | SWP_SHOWWINDOW);
    visible_ = true;
    ++shown_;
    InvalidateRect(hwnd_, nullptr, FALSE);
    UpdateWindow(hwnd_);
}

void OverlayWindow::paint(HDC dc) { renderPanel(dc, last_, lay_, font_); }

}  // namespace nvcoop::chat
