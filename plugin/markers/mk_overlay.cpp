// See mk_overlay.h. UNPROVEN IN GAME.
#include <Windows.h>
#ifdef min
#undef min
#endif
#ifdef max
#undef max
#endif

#include <algorithm>
#include <cmath>
#include <cstdint>
#include <cstdio>
#include <cstring>
#include <string>
#include <vector>

#include "common/nvcoop_api.h"
#include "mk_overlay.h"

namespace nvcoop::markers::overlay {

namespace {

// ---------------------------------------------------------------------------------------------------------------------------------- COM plumbing
// IDirect3DDevice9 vtable slots (d3d9.h order).
constexpr int kRelease = 2;
constexpr int kPresent = 17, kGetBackBuffer = 18, kCreateTexture = 23, kSetRenderTarget = 37, kGetRenderTarget = 38, kBeginScene = 41, kEndScene = 42,
              kSetViewport = 47, kSetRenderState = 57, kCreateStateBlock = 59, kSetTexture = 65, kSetTextureStageState = 67, kSetSamplerState = 69,
              kDrawPrimitiveUP = 83, kSetFVF = 89, kSetVertexShader = 92, kSetPixelShader = 107;
// IDirect3DSurface9::GetDesc = 12; IDirect3DTexture9::LockRect = 19, UnlockRect = 20; IDirect3DStateBlock9::Capture = 4, Apply = 5.

template <class F> F V(void* obj, int idx) { return reinterpret_cast<F>((*reinterpret_cast<void***>(obj))[idx]); }
using Rel_t = ULONG(__stdcall*)(void*);

struct D3DSurfaceDesc { UINT Format, Type, Usage, Pool, MultiSampleType, MultiSampleQuality, Width, Height; };
struct D3DLockedRect { INT Pitch; void* pBits; };
struct D3DViewport { DWORD X, Y, Width, Height; float MinZ, MaxZ; };
struct D3DPresentParams {
    UINT BackBufferWidth, BackBufferHeight, BackBufferFormat, BackBufferCount, MultiSampleType;
    DWORD MultiSampleQuality;
    UINT SwapEffect;
    HWND hDeviceWindow;
    BOOL Windowed, EnableAutoDepthStencil;
    UINT AutoDepthStencilFormat;
    DWORD Flags;
    UINT FullScreen_RefreshRateInHz, PresentationInterval;
};

// d3d9types.h values used below
constexpr DWORD kFvf = 0x004 /*XYZRHW*/ | 0x040 /*DIFFUSE*/ | 0x100 /*TEX1*/;
constexpr DWORD kFmtA8R8G8B8 = 21, kPoolManaged = 1, kSbtAll = 1, kTriList = 4;
enum Rs : DWORD { kFillMode = 8, kShadeMode = 9, kZWrite = 14, kAlphaTest = 15, kSrcBlend = 19, kDestBlend = 20, kCull = 22, kAlphaBlend = 27, kFog = 28,
                  kSpecular = 29, kStencil = 52, kLighting = 137, kColorWrite = 168, kBlendOp = 171, kScissor = 174, kSrgbWrite = 194, kSepAlpha = 206, kZEnable = 7,
                  kColorVertex = 141 };

struct Vtx {
    float x, y, z, rhw;
    DWORD color;
    float u, v;
};

// ---------------------------------------------------------------------------------------------------------------------------------- state
Provider g_provider = nullptr;
void* g_ctx = nullptr;
using Present_t = HRESULT(__stdcall*)(void*, const RECT*, const RECT*, HWND, const void*);
Present_t g_origPresent = nullptr;
void** g_vtable = nullptr;
bool g_hookTried = false;
volatile bool g_enabled = true;
bool g_inHook = false;
int g_failStreak = 0;
bool g_dead = false;
uint64_t g_frames = 0, g_drawn = 0, g_faults = 0, g_fails = 0;
int g_lastTexts = 0, g_lastRects = 0;
unsigned g_lastW = 0, g_lastH = 0;

// glyph atlas
constexpr int kAtlasW = 512, kAtlasH = 256, kCell = 32, kCols = 16, kFirst = 32, kGlyphs = 96;
void* g_atlasDev = nullptr;
void* g_atlas = nullptr;
float g_adv[kGlyphs] = {};
float g_lineH = 30.0f;

void ReleaseAtlas() {
    if (g_atlas) {
        __try { V<Rel_t>(g_atlas, kRelease)(g_atlas); } __except (EXCEPTION_EXECUTE_HANDLER) {}
    }
    g_atlas = nullptr;
    g_atlasDev = nullptr;
}

// Rasterise the printable ASCII range with GDI (a system font, white on black = the glyph's alpha), into a managed texture.
bool BuildAtlas(void* dev) {
    std::vector<uint32_t> px(static_cast<size_t>(kAtlasW) * kAtlasH, 0);
    BITMAPINFO bi{};
    bi.bmiHeader.biSize = sizeof bi.bmiHeader;
    bi.bmiHeader.biWidth = kAtlasW;
    bi.bmiHeader.biHeight = -kAtlasH;
    bi.bmiHeader.biPlanes = 1;
    bi.bmiHeader.biBitCount = 32;
    bi.bmiHeader.biCompression = BI_RGB;
    void* bits = nullptr;
    HDC dc = CreateCompatibleDC(nullptr);
    HBITMAP bmp = dc ? CreateDIBSection(dc, &bi, DIB_RGB_COLORS, &bits, nullptr, 0) : nullptr;
    if (!dc || !bmp || !bits) {
        if (bmp) DeleteObject(bmp);
        if (dc) DeleteDC(dc);
        return false;
    }
    std::memset(bits, 0, static_cast<size_t>(kAtlasW) * kAtlasH * 4);
    HGDIOBJ oldBmp = SelectObject(dc, bmp);
    HFONT font = CreateFontA(-25, 0, 0, 0, FW_SEMIBOLD, FALSE, FALSE, FALSE, ANSI_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS, ANTIALIASED_QUALITY,
                             DEFAULT_PITCH | FF_SWISS, "Arial");
    HGDIOBJ oldFont = font ? SelectObject(dc, font) : nullptr;
    SetBkMode(dc, TRANSPARENT);
    SetTextColor(dc, RGB(255, 255, 255));
    TEXTMETRICA tm{};
    GetTextMetricsA(dc, &tm);
    g_lineH = tm.tmHeight > 8 ? static_cast<float>(tm.tmHeight) : 30.0f;
    for (int i = 0; i < kGlyphs; ++i) {
        const char ch = static_cast<char>(kFirst + i);
        const int cx = (i % kCols) * kCell, cy = (i / kCols) * kCell;
        SIZE sz{};
        GetTextExtentPoint32A(dc, &ch, 1, &sz);
        g_adv[i] = static_cast<float>(sz.cx > 0 ? sz.cx : 8);
        TextOutA(dc, cx + 2, cy, &ch, 1);
    }
    GdiFlush();
    const uint32_t* src = static_cast<const uint32_t*>(bits);
    for (size_t i = 0; i < px.size(); ++i) {
        const uint32_t p = src[i];
        const uint32_t a = std::max({(p >> 16) & 0xFFu, (p >> 8) & 0xFFu, p & 0xFFu});
        px[i] = (a << 24) | 0x00FFFFFFu;
    }
    for (int y = kAtlasH - 4; y < kAtlasH; ++y)   // a solid block for rectangles, bottom right
        for (int x = kAtlasW - 4; x < kAtlasW; ++x) px[static_cast<size_t>(y) * kAtlasW + x] = 0xFFFFFFFFu;
    if (oldFont) SelectObject(dc, oldFont);
    if (font) DeleteObject(font);
    SelectObject(dc, oldBmp);
    DeleteObject(bmp);
    DeleteDC(dc);

    using CreateTex = HRESULT(__stdcall*)(void*, UINT, UINT, UINT, DWORD, DWORD, DWORD, void**, void*);
    void* tex = nullptr;
    if (V<CreateTex>(dev, kCreateTexture)(dev, kAtlasW, kAtlasH, 1, 0, kFmtA8R8G8B8, kPoolManaged, &tex, nullptr) != 0 || !tex) return false;
    D3DLockedRect lr{};
    using Lock = HRESULT(__stdcall*)(void*, UINT, D3DLockedRect*, const RECT*, DWORD);
    if (V<Lock>(tex, 19)(tex, 0, &lr, nullptr, 0) != 0 || !lr.pBits) {
        V<Rel_t>(tex, kRelease)(tex);
        return false;
    }
    for (int y = 0; y < kAtlasH; ++y)
        std::memcpy(static_cast<unsigned char*>(lr.pBits) + static_cast<size_t>(y) * lr.Pitch, &px[static_cast<size_t>(y) * kAtlasW], static_cast<size_t>(kAtlasW) * 4);
    V<HRESULT(__stdcall*)(void*, UINT)>(tex, 20)(tex, 0);
    g_atlas = tex;
    g_atlasDev = dev;
    return true;
}

float GlyphAdv(char c) {
    const int i = (c >= kFirst && c < kFirst + kGlyphs) ? c - kFirst : '?' - kFirst;
    return g_adv[i];
}
float TextWidth(const std::string& s, float h) {
    float w = 0;
    for (char c : s) w += GlyphAdv(c);
    return w * (h / g_lineH);
}

// ---------------------------------------------------------------------------------------------------------------------------------- batching
DWORD Argb(uint32_t rgb, float a) {
    const DWORD al = static_cast<DWORD>(std::max(0.0f, std::min(1.0f, a)) * 255.0f + 0.5f);
    return (al << 24) | (rgb & 0xFFFFFFu);
}
void AddQuad(std::vector<Vtx>& v, float x0, float y0, float x1, float y1, float u0, float v0, float u1, float v1, DWORD c) {
    x0 -= 0.5f; y0 -= 0.5f; x1 -= 0.5f; y1 -= 0.5f;   // pixel centres
    const Vtx a{x0, y0, 0, 1, c, u0, v0}, b{x1, y0, 0, 1, c, u1, v0}, d{x0, y1, 0, 1, c, u0, v1}, e{x1, y1, 0, 1, c, u1, v1};
    v.push_back(a); v.push_back(b); v.push_back(d);
    v.push_back(b); v.push_back(e); v.push_back(d);
}
void AddSolid(std::vector<Vtx>& v, float x, float y, float w, float h, DWORD c) {
    const float u = (kAtlasW - 2) / static_cast<float>(kAtlasW), vv = (kAtlasH - 2) / static_cast<float>(kAtlasH);
    AddQuad(v, x, y, x + w, y + h, u, vv, u, vv, c);
}
void AddText(std::vector<Vtx>& v, const std::string& s, float x, float y, float h, DWORD color) {
    const float k = h / g_lineH;
    float pen = x;
    for (char ch : s) {
        const int i = (ch >= kFirst && ch < kFirst + kGlyphs) ? ch - kFirst : '?' - kFirst;
        const float adv = g_adv[i];
        const float cx = static_cast<float>((i % kCols) * kCell + 2), cy = static_cast<float>((i / kCols) * kCell);
        AddQuad(v, pen - k, y, pen + (adv + 1.0f) * k, y + kCell * k, (cx - 1.0f) / kAtlasW, cy / kAtlasH, (cx + adv + 1.0f) / kAtlasW, (cy + kCell) / kAtlasH, color);
        pen += adv * k;
    }
}

void Batch(const nvmk::PaintList& list, std::vector<Vtx>& v) {
    for (const nvmk::PRect& r : list.rects) AddSolid(v, std::floor(r.x + 0.5f), std::floor(r.y + 0.5f), std::max(1.0f, std::floor(r.w + 0.5f)), std::max(1.0f, std::floor(r.h + 0.5f)), Argb(r.rgb, r.a));
    for (const nvmk::PText& t : list.texts) {
        const float w = TextWidth(t.s, t.h);
        float x = t.align == nvmk::Align::Center ? t.x - w * 0.5f : t.x;
        x = std::floor(x + 0.5f);
        const float y = std::floor(t.y + 0.5f);
        if (t.pill) AddSolid(v, x - t.h * 0.3f, y - t.h * 0.04f, w + t.h * 0.6f, t.h * 1.08f, Argb(0x000000, 0.45f * t.a));
        AddText(v, t.s, x + std::max(1.0f, t.h * 0.05f), y + std::max(1.0f, t.h * 0.05f), t.h, Argb(0x000000, 0.8f * t.a));   // shadow
        AddText(v, t.s, x, y, t.h, Argb(t.rgb, t.a));
    }
}

// ---------------------------------------------------------------------------------------------------------------------------------- the frame
// Returns 0 = nothing to draw, 1 = drawn, -1 = a D3D call failed.
int DrawFrame(void* dev) {
    using GetBB = HRESULT(__stdcall*)(void*, UINT, UINT, UINT, void**);
    using GetRT = HRESULT(__stdcall*)(void*, DWORD, void**);
    using SetRT = HRESULT(__stdcall*)(void*, DWORD, void*);
    using GetDesc = HRESULT(__stdcall*)(void*, D3DSurfaceDesc*);
    void* bb = nullptr;
    if (V<GetBB>(dev, kGetBackBuffer)(dev, 0, 0, 0, &bb) != 0 || !bb) return -1;
    D3DSurfaceDesc d{};
    V<GetDesc>(bb, 12)(bb, &d);
    int result = 0;
    nvmk::PaintList list;
    if (d.Width >= 64 && d.Height >= 64 && g_provider && g_provider(g_ctx, static_cast<float>(d.Width), static_cast<float>(d.Height), list) && !list.empty()) {
        if (g_atlasDev != dev) ReleaseAtlas();
        if (!g_atlas && !BuildAtlas(dev)) {
            V<Rel_t>(bb, kRelease)(bb);
            return -1;
        }
        std::vector<Vtx> verts;
        verts.reserve(1024);
        Batch(list, verts);
        g_lastTexts = static_cast<int>(list.texts.size());
        g_lastRects = static_cast<int>(list.rects.size());
        g_lastW = d.Width; g_lastH = d.Height;

        void* sb = nullptr;
        void* oldRt = nullptr;
        using CreateSB = HRESULT(__stdcall*)(void*, DWORD, void**);
        if (V<CreateSB>(dev, kCreateStateBlock)(dev, kSbtAll, &sb) != 0 || !sb) {
            V<Rel_t>(bb, kRelease)(bb);
            return -1;
        }
        V<HRESULT(__stdcall*)(void*)>(sb, 4)(sb);   // Capture
        V<GetRT>(dev, kGetRenderTarget)(dev, 0, &oldRt);
        V<SetRT>(dev, kSetRenderTarget)(dev, 0, bb);
        D3DViewport vp{0, 0, d.Width, d.Height, 0.0f, 1.0f};
        V<HRESULT(__stdcall*)(void*, const D3DViewport*)>(dev, kSetViewport)(dev, &vp);
        auto rs = [&](DWORD s, DWORD val) { V<HRESULT(__stdcall*)(void*, DWORD, DWORD)>(dev, kSetRenderState)(dev, s, val); };
        auto tss = [&](DWORD stage, DWORD type, DWORD val) { V<HRESULT(__stdcall*)(void*, DWORD, DWORD, DWORD)>(dev, kSetTextureStageState)(dev, stage, type, val); };
        auto ss = [&](DWORD sampler, DWORD type, DWORD val) { V<HRESULT(__stdcall*)(void*, DWORD, DWORD, DWORD)>(dev, kSetSamplerState)(dev, sampler, type, val); };
        V<HRESULT(__stdcall*)(void*, void*)>(dev, kSetVertexShader)(dev, nullptr);
        V<HRESULT(__stdcall*)(void*, void*)>(dev, kSetPixelShader)(dev, nullptr);
        V<HRESULT(__stdcall*)(void*, DWORD)>(dev, kSetFVF)(dev, kFvf);
        V<HRESULT(__stdcall*)(void*, DWORD, void*)>(dev, kSetTexture)(dev, 0, g_atlas);
        rs(kZEnable, 0); rs(kZWrite, 0); rs(kCull, 1 /*none*/); rs(kFillMode, 3); rs(kShadeMode, 2);
        rs(kAlphaBlend, 1); rs(kSrcBlend, 5 /*srcalpha*/); rs(kDestBlend, 6 /*invsrcalpha*/); rs(kBlendOp, 1); rs(kSepAlpha, 0);
        rs(kAlphaTest, 0); rs(kStencil, 0); rs(kScissor, 0); rs(kFog, 0); rs(kLighting, 0); rs(kSpecular, 0); rs(kColorVertex, 1);
        rs(kColorWrite, 0xF); rs(kSrgbWrite, 0);
        tss(0, 1, 4); tss(0, 2, 2); tss(0, 3, 0);   // colour: modulate(texture, diffuse)
        tss(0, 4, 4); tss(0, 5, 2); tss(0, 6, 0);   // alpha:  modulate(texture, diffuse)
        tss(0, 11, 0); tss(0, 24, 0);               // texture coordinate set 0, no transform
        tss(1, 1, 1); tss(1, 4, 1);                 // stage 1 off
        ss(0, 5, 2); ss(0, 6, 2); ss(0, 7, 0); ss(0, 1, 3); ss(0, 2, 3); ss(0, 11, 0);   // linear, no mips, clamp, no sRGB read
        using Scene = HRESULT(__stdcall*)(void*);
        const bool began = V<Scene>(dev, kBeginScene)(dev) == 0;
        using DrawUP = HRESULT(__stdcall*)(void*, DWORD, UINT, const void*, UINT);
        if (!verts.empty()) {
            const HRESULT hr = V<DrawUP>(dev, kDrawPrimitiveUP)(dev, kTriList, static_cast<UINT>(verts.size() / 3), verts.data(), sizeof(Vtx));
            result = hr == 0 ? 1 : -1;
        }
        if (began) V<Scene>(dev, kEndScene)(dev);
        if (oldRt) {
            V<SetRT>(dev, kSetRenderTarget)(dev, 0, oldRt);
            V<Rel_t>(oldRt, kRelease)(oldRt);
        }
        V<HRESULT(__stdcall*)(void*)>(sb, 5)(sb);   // Apply
        V<Rel_t>(sb, kRelease)(sb);
    }
    V<Rel_t>(bb, kRelease)(bb);
    return result;
}

int SafeDraw(void* dev) {
    __try {
        return DrawFrame(dev);
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        ++g_faults;
        return -2;
    }
}

HRESULT __stdcall PresentHook(void* dev, const RECT* src, const RECT* dst, HWND wnd, const void* dirty) {
    ++g_frames;
    if (g_enabled && !g_dead && !g_inHook && g_provider) {
        g_inHook = true;
        const int r = SafeDraw(dev);
        g_inHook = false;
        if (r == 1) { ++g_drawn; g_failStreak = 0; }
        else if (r == 0) g_failStreak = 0;
        else {
            ++g_fails;
            if (++g_failStreak >= 10 || g_faults >= 3) {
                g_dead = true;
                nvcoop::Log("markers: overlay: %d failed frames in a row (%llu faults): the overlay is off for this run", g_failStreak, static_cast<unsigned long long>(g_faults));
            }
        }
    }
    return g_origPresent(dev, src, dst, wnd, dirty);
}

// The vtable address shared by every device of the same D3D9 class: a throw-away HAL device on a hidden window; its vtable pointer is read and the
// device released at once. Nothing is called on the game's own objects.
void** ProbeVtable() {
    using Create9 = void*(__stdcall*)(UINT);
    HMODULE m = GetModuleHandleA("d3d9.dll");
    if (!m) return nullptr;
    auto create = reinterpret_cast<Create9>(GetProcAddress(m, "Direct3DCreate9"));
    if (!create) return nullptr;
    HWND w = CreateWindowExA(0, "STATIC", "nvcoop-probe", WS_POPUP, 0, 0, 64, 64, nullptr, nullptr, nullptr, nullptr);
    void* d3d = create(32);
    void** vt = nullptr;
    if (d3d && w) {
        D3DPresentParams pp{};
        pp.BackBufferWidth = 64; pp.BackBufferHeight = 64; pp.BackBufferFormat = 0; pp.BackBufferCount = 1;
        pp.SwapEffect = 1; pp.hDeviceWindow = w; pp.Windowed = TRUE;
        void* dev = nullptr;
        using CreateDev = HRESULT(__stdcall*)(void*, UINT, UINT, HWND, DWORD, D3DPresentParams*, void**);
        if (V<CreateDev>(d3d, 16)(d3d, 0, 1 /*HAL*/, w, 0x20 /*software vertex processing*/, &pp, &dev) == 0 && dev) {
            vt = *reinterpret_cast<void***>(dev);
            V<Rel_t>(dev, kRelease)(dev);
        }
    }
    if (d3d) V<Rel_t>(d3d, kRelease)(d3d);
    if (w) DestroyWindow(w);
    return vt;
}

bool InModule(void* addr, char* nameOut, size_t n) {
    HMODULE mod = nullptr;
    if (!GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, reinterpret_cast<LPCSTR>(addr), &mod) || !mod) return false;
    char path[MAX_PATH] = {};
    GetModuleFileNameA(mod, path, MAX_PATH);
    const char* base = std::strrchr(path, '\\');
    std::snprintf(nameOut, n, "%s", base ? base + 1 : path);
    return true;
}

}  // namespace

bool install(Provider provider, void* ctx, std::string& err) {
    g_provider = provider;
    g_ctx = ctx;
    if (g_hookTried) {
        if (!g_origPresent) err = "the Present hook did not go in earlier";
        return g_origPresent != nullptr;
    }
    g_hookTried = true;
    void** vt = ProbeVtable();
    if (!vt) { err = "the D3D9 probe device could not be created"; return false; }
    char owner[MAX_PATH] = {};
    if (!InModule(vt[kPresent], owner, sizeof owner)) { err = "the Present slot does not point into a loaded module"; return false; }
    char vtOwner[MAX_PATH] = {};
    if (!InModule(vt, vtOwner, sizeof vtOwner) || _stricmp(vtOwner, "d3d9.dll") != 0) {
        err = std::string("the device vtable is not in d3d9.dll (") + vtOwner + ")";
        return false;
    }
    DWORD old = 0;
    if (!VirtualProtect(&vt[kPresent], sizeof(void*), PAGE_EXECUTE_READWRITE, &old)) { err = "VirtualProtect on the D3D9 vtable failed"; return false; }
    g_origPresent = reinterpret_cast<Present_t>(vt[kPresent]);
    vt[kPresent] = reinterpret_cast<void*>(&PresentHook);
    VirtualProtect(&vt[kPresent], sizeof(void*), old, &old);
    g_vtable = vt;
    nvcoop::Log("markers: overlay: Present hooked (the slot pointed into %s)", owner);
    return true;
}

bool installed() { return g_origPresent != nullptr && !g_dead; }
void setEnabled(bool on) { g_enabled = on; }

std::string statusJson() {
    char b[320];
    std::snprintf(b, sizeof b, "{\"hooked\":%s,\"dead\":%s,\"frames\":%llu,\"drawn\":%llu,\"fails\":%llu,\"faults\":%llu,\"lastTexts\":%d,\"lastRects\":%d,\"window\":\"%ux%u\"}",
                  g_origPresent ? "true" : "false", g_dead ? "true" : "false", (unsigned long long)g_frames, (unsigned long long)g_drawn, (unsigned long long)g_fails,
                  (unsigned long long)g_faults, g_lastTexts, g_lastRects, g_lastW, g_lastH);
    return b;
}

}  // namespace nvcoop::markers::overlay
