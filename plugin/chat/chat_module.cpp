// nvcoop module for the text chat (x86, compiled into nvcoop.dll). NOTHING here has run in a game; everything it calls is tested game-free in plugin/chat.
//   * reads nvcoop.ini [chat] (chat_config.h); [chat] enabled=0 leaves the module doing nothing at all
//   * a WH_GETMESSAGE hook on the game thread feeds the window messages to the InputTranslator (the open key, the typed line); the game keeps reading the
//     keyboard through DirectInput, so the keys are also disabled with DisableKey while the line is open (chat_keyblock.h)
//   * once per frame: the session adapter (Runtime::active() -> channel 0xC8), the controller (the line), the key blocker, the surface (HUD or overlay)
//   * bridge methods: chat.status, chat.say, chat.log, chat.open, chat.type, chat.key, chat.system (for the automated in-game checks; docs/chat.md)
//   * other modules put lines in with nvcoop::chat::postSystem / postLocal (chat_api.h)
#include <Windows.h>

#include <chrono>
#include <cstring>
#include <cctype>
#include <memory>
#include <string>

#include "PluginAPI.h"
#include "chat/chat_api.h"
#include "chat/chat_config.h"
#include "chat/chat_controller.h"
#include "chat/chat_input.h"
#include "chat/chat_keyblock.h"
#include "chat/chat_overlay_win.h"
#include "chat/chat_session.h"
#include "chat/chat_text.h"
#include "chat/chat_surface.h"
#include "common/minijson.h"
#include "common/nvcoop_api.h"
#include "coop_runtime.h"
#include "engine_hooks.h"
#include "reload/reload_state.h"

namespace {

using namespace nvcoop::chat;

// ---- engine pins (FalloutNV.exe 1.4.0.525, checked once against these bytes; a mismatch turns the call off) --------------------------------------------
// MenuModeAny: cdecl bool(), true while any menu that pauses or overlays the world is up (Pip-Boy, console, dialogue, container, loading ...).
//   Same pin as plugin/avatar/engine/engine_pins.h MenuModeAny.
constexpr unsigned kMenuModeAny = 0x702360u;
constexpr unsigned char kMenuModeAnyBytes[16] = {0x55, 0x8b, 0xec, 0xe8, 0xa8, 0x4e, 0xdb, 0xff, 0x85, 0xc0, 0x74, 0x21, 0xe8, 0x9f, 0x4e, 0xdb};
// QueueUIMessage: cdecl bool(const char* msg, UInt32 emotion, const char* ddsPath, const char* soundName, float time, bool next), the HUD message line.
//   Same pin as plugin/intents/engine/intents_addrs.h QueueUIMessage.
constexpr unsigned kQueueUIMessage = 0x7052F0u;
constexpr unsigned char kQueueUIMessageBytes[16] = {0x55, 0x8b, 0xec, 0x6a, 0xff, 0x68, 0x38, 0xc3, 0xee, 0x00, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00};

bool pinMatches(unsigned addr, const unsigned char* want) {
    unsigned char got[16] = {};
    __try {
        std::memcpy(got, reinterpret_cast<const void*>(addr), 16);
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
    return std::memcmp(got, want, 16) == 0;
}

bool menuModeSEH(bool* out) {
    __try {
        *out = reinterpret_cast<bool(__cdecl*)()>(kMenuModeAny)();
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

bool queueMessageSEH(const char* text, float seconds, bool next) {
    __try {
        reinterpret_cast<bool(__cdecl*)(const char*, uint32_t, const char*, const char*, float, bool)>(kQueueUIMessage)(text, 0, nullptr, nullptr, seconds, next);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

bool g_menuPin = false, g_msgPin = false, g_pinsChecked = false;
void checkPins() {
    if (g_pinsChecked) return;
    g_pinsChecked = true;
    g_menuPin = pinMatches(kMenuModeAny, kMenuModeAnyBytes);
    g_msgPin = pinMatches(kQueueUIMessage, kQueueUIMessageBytes);
    if (!g_menuPin) nvcoop::Log("chat: MenuModeAny pin does not match this FalloutNV.exe: the chat key stays off (it could not tell a menu from the game)");
    if (!g_msgPin) nvcoop::Log("chat: QueueUIMessage pin does not match this FalloutNV.exe: the HUD surface shows nothing");
}

// Any menu on screen. Unknown (pin mismatch or a fault) counts as a menu: the chat then never opens, which beats opening over the console.
bool menuOpen() {
    checkPins();
    if (!g_menuPin) return true;
    bool v = true;
    return menuModeSEH(&v) ? v : true;
}

bool hudMessage(const std::string& text, float seconds, bool next) {
    checkPins();
    return g_msgPin && queueMessageSEH(text.c_str(), seconds, next);
}

// ---- state -----------------------------------------------------------------------------------------------------------------------------------------------

uint64_t nowMs() {
    return static_cast<uint64_t>(std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::steady_clock::now().time_since_epoch()).count());
}

struct State {
    Config cfg;
    std::unique_ptr<ChatSession> session;
    std::unique_ptr<ChatController> ctl;
    std::unique_ptr<InputTranslator> input;
    std::unique_ptr<KeyBlocker> keys;
    std::unique_ptr<HudSurface> hud;
    std::unique_ptr<OverlayWindow> overlay;
    IChatSurface* surface = nullptr;
    HHOOK hook = nullptr;
    HMODULE self = nullptr;
    bool configured = false, running = false, surfaceChosen = false;
};
State g;

std::string iniPath() {
    HMODULE self = nullptr;
    GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, reinterpret_cast<LPCSTR>(&iniPath), &self);
    char buf[MAX_PATH] = {};
    GetModuleFileNameA(self, buf, MAX_PATH);
    std::string p = buf;
    const size_t s = p.find_last_of("\\/");
    return (s == std::string::npos ? std::string() : p.substr(0, s + 1)) + "nvcoop.ini";
}

IniGetter makeGetter() {
    const std::string ini = iniPath();
    return [ini](const char* section, const char* key, const char* def) -> std::string {
        std::string env = "NVCOOP_";
        for (const char* p = section; *p; ++p) env += static_cast<char>(std::toupper(static_cast<unsigned char>(*p)));
        env += '_';
        for (const char* p = key; *p; ++p) env += static_cast<char>(std::toupper(static_cast<unsigned char>(*p)));
        char ebuf[256] = {};
        if (GetEnvironmentVariableA(env.c_str(), ebuf, sizeof ebuf) > 0) return ebuf;
        char buf[256] = {};
        GetPrivateProfileStringA(section, key, def, buf, sizeof buf, ini.c_str());
        return buf;
    };
}

BOOL CALLBACK pickWindow(HWND h, LPARAM lp) {
    HWND* best = reinterpret_cast<HWND*>(lp);   // [0] visible and unowned
    if (!best[0] && IsWindowVisible(h) && !GetWindow(h, GW_OWNER)) best[0] = h;
    return TRUE;
}

HWND gameWindow() {
    HWND best[1] = {nullptr};
    EnumThreadWindows(GetCurrentThreadId(), pickWindow, reinterpret_cast<LPARAM>(best));
    return best[0];
}

LRESULT CALLBACK getMsgProc(int code, WPARAM wp, LPARAM lp) {
    if (code == HC_ACTION && wp == PM_REMOVE && g.input) {
        MSG* m = reinterpret_cast<MSG*>(lp);
        const MSG original = *m;
        const InputTranslator::Verdict v = g.input->onMessage(m->message, m->wParam, m->lParam);
        if (v != InputTranslator::Verdict::Pass) {
            m->message = WM_NULL;   // the game's window code gets a no-op
            if (v == InputTranslator::Verdict::SwallowAndTranslate) TranslateMessage(&original);   // the character arrives as a WM_CHAR we then swallow too
        }
    }
    return CallNextHookEx(nullptr, code, wp, lp);
}

std::string clipboardText() {
    std::string out;
    if (!OpenClipboard(nullptr)) return out;
    if (HANDLE h = GetClipboardData(CF_TEXT)) {
        if (const char* p = static_cast<const char*>(GlobalLock(h))) {
            out.assign(p, strnlen(p, 1024));
            GlobalUnlock(h);
        }
    }
    CloseClipboard();
    return out;
}

void chooseSurface() {
    HWND game = gameWindow();
    if (!game) return;   // not up yet: try again next frame
    g.surfaceChosen = true;
    bool wantOverlay = g.cfg.display == "overlay";
    if (g.cfg.display == "auto") wantOverlay = (GetWindowLongW(game, GWL_STYLE) & WS_CAPTION) == WS_CAPTION;   // windowed: a window can sit over the game
    if (wantOverlay) {
        g.overlay.reset(new OverlayWindow());
        if (g.overlay->create(game)) g.surface = g.overlay.get();
        else g.overlay.reset();
    }
    if (!g.surface) g.surface = g.hud.get();
    nvcoop::Log("chat: display=%s (setting '%s')", g.surface->name(), g.cfg.display.c_str());
}

void initOnce() {
    if (g.configured) return;
    g.configured = true;
    ChatSettings s = readSettings(makeGetter());
    g.cfg = s.cfg;
    nvcoop::Log("chat: %s", describe(g.cfg).c_str());
    if (!s.warnings.empty()) nvcoop::Log("chat: %s", s.warnings.c_str());
}

void start() {
    if (g.running || !g.cfg.enabled) return;
    g.running = true;
    checkPins();
    g.session.reset(new ChatSession(g.cfg, static_cast<uint32_t>(nowMs() & 0x7FFFFFFF) | 1u));
    g.hud.reset(new HudSurface(hudMessage, g.cfg.lineSeconds));
    ControllerDeps cd;
    cd.menuOpen = menuOpen;
    cd.clipboard = clipboardText;
    g.ctl.reset(new ChatController(g.session->core(), std::move(cd)));
    InputTranslator::Deps id;
    id.canOpen = [] { return g.session && g.session->core().active() && !menuOpen(); };
    id.keyDown = [](int vk) { return (GetKeyState(vk) & 0x8000) != 0; };
    g.input.reset(new InputTranslator(std::move(id)));
    g.input->setOpenKey(g.cfg.openKey);
    g.keys.reset(new KeyBlocker([](const std::string& line) { return nvhooks::execConsoleLine(line); }));
    g.session->core().setOnLine([](const Line& l) { if (g.surface) g.surface->onLine(l, nowMs()); });
    nvcoop::chat::setPoster([](const std::string& text, bool everyone) {
        if (g.session) g.session->core().system(text, everyone, nowMs());
    });
}

void stop() {
    if (!g.running) return;
    nvcoop::chat::setPoster(nullptr);
    if (g.input) g.input->setEnabled(false);
    if (g.hook) { UnhookWindowsHookEx(g.hook); g.hook = nullptr; }
    if (g.keys) {   // the keys must come back whatever happens
        g.keys->want(false);
        for (int i = 0; i < 12 && !g.keys->settled(); ++i) g.keys->frame();
    }
    if (g.surface) g.surface->hide();
    if (g.overlay) g.overlay->destroy();
    g.surface = nullptr;
    g.ctl.reset(); g.input.reset(); g.keys.reset(); g.overlay.reset(); g.hud.reset(); g.session.reset();
    g.surfaceChosen = false;
    g.running = false;
}

void frameInfoFrom(nvcoop::coop::Runtime* rt, FrameInfo& f) {
    f.nowMs = nowMs();
    if (!rt || !rt->running()) return;
    f.active = true;
    f.mux = rt->mux();
    if (auto* host = rt->host()) {
        f.isHost = true;
        f.self = kHostSlot;
        auto links = [rt]() {
            std::vector<ClientLink> out;
            if (auto* h = rt->host())
                for (const auto& cv : h->clients()) out.push_back(ClientLink{cv.slot, cv.peer, cv.joined});
            return out;
        };
        f.clients = links();
        f.clientsNow = links;
        (void)host;
    } else if (auto* cl = rt->client()) {
        f.isHost = false;
        f.self = cl->joined() ? cl->slot() : kSystemSlot;
        f.hostLinkUp = cl->joined() && !cl->baselineActive();
    } else {
        f.active = false;
    }
}

void onFrame() {
    if (!g.configured || !g.running) return;
    if (!g.surfaceChosen) chooseSurface();
    if (!g.hook && g.input) {
        // the thread that pumps the game's messages is the one that runs the frame
        g.hook = SetWindowsHookExW(WH_GETMESSAGE, getMsgProc, g.self, GetCurrentThreadId());
        if (!g.hook) nvcoop::Log("chat: the key hook could not be installed (error %lu): the chat line cannot be opened by key", GetLastError());
    }
    FrameInfo f;
    frameInfoFrom(nvcoop::coop::Runtime::active(), f);
    g.session->frame(f);
    const uint64_t now = f.nowMs;
    std::vector<InputEvent> events;
    g.input->drain(events);
    g.ctl->frame(now, events);
    g.input->setComposing(g.ctl->composing());
    g.keys->want(g.ctl->composing());
    g.keys->frame();
    if (g.surface) g.surface->present(g.ctl->view(now), now);
}

void onLoad(void*) {
    initOnce();
    GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT, reinterpret_cast<LPCSTR>(&iniPath), &g.self);
    if (g.cfg.enabled) start();

    nvcoop::RegisterMethodClosure("chat.status", [](const std::string&, std::string& out) {
        nvcoop::json::Value v = nvcoop::json::Value::object();
        v.set("enabled", g.cfg.enabled);
        v.set("running", g.running);
        if (g.running && g.session) {
            const auto& c = g.session->core();
            v.set("session", c.active());
            v.set("host", c.isHost());
            v.set("self", static_cast<int>(c.self()));
            v.set("name", c.selfName());
            v.set("composing", g.ctl->composing());
            v.set("lines", static_cast<uint64_t>(c.log().lines().size()));
            v.set("display", g.surface ? g.surface->name() : "none");
            v.set("key", keyName(g.cfg.openKey));
            v.set("hook", g.hook != nullptr);
            v.set("menuPin", g_menuPin);
            v.set("messagePin", g_msgPin);
            v.set("keysDisabled", g.keys->disabledCount());
            v.set("keysFailed", g.keys->failed());
            nvcoop::json::Value s = nvcoop::json::Value::object();
            const auto& st = c.stats();
            s.set("said", st.said); s.set("relayed", st.relayed); s.set("received", st.received); s.set("rateLimited", st.rateLimited);
            s.set("duplicates", st.duplicates); s.set("undecodable", st.undecodable); s.set("unknownSender", st.unknownSender);
            s.set("joins", st.joinsAnnounced); s.set("leaves", st.leavesAnnounced);
            v.set("stats", s);
            v.set("swallowed", g.input->swallowed());
            v.set("opened", g.ctl->opened());
            v.set("sent", g.ctl->sent());
            v.set("sessions", g.session->sessions());
        }
        out = v.dump();
        return true;
    }, true, 0);
    nvcoop::RegisterMethodClosure("chat.say", [](const std::string& params, std::string& out) {
        if (!g.running) { out = "chat is off"; return false; }
        std::string err;
        nvcoop::json::Value p;
        const std::string text = nvcoop::json::parse(params, p, err) ? p.get("text").asString("") : std::string();
        const SayResult r = g.session->core().say(text, nowMs());
        const char* names[] = {"sent", "empty", "rateLimited", "noSession"};
        out = std::string("{\"result\":\"") + names[static_cast<int>(r)] + "\"}";
        return true;
    }, true, 0);
    nvcoop::RegisterMethodClosure("chat.log", [](const std::string& params, std::string& out) {
        if (!g.running) { out = "chat is off"; return false; }
        nvcoop::json::Value p;
        std::string err;
        nvcoop::json::parse(params, p, err);
        const size_t n = static_cast<size_t>(p.get("n").asInt(20));
        nvcoop::json::Value arr = nvcoop::json::Value::array();
        const uint64_t now = nowMs();
        for (const Line* l : g.session->core().log().recent(n)) {
            nvcoop::json::Value o = nvcoop::json::Value::object();
            o.set("slot", static_cast<int>(l->slot));
            o.set("system", l->kind == LineKind::System);
            o.set("name", l->name);
            o.set("text", l->text);
            o.set("ageMs", now > l->atMs ? now - l->atMs : 0);
            arr.push(o);
        }
        nvcoop::json::Value res = nvcoop::json::Value::object();
        res.set("lines", arr);
        out = res.dump();
        return true;
    }, true, 0);
    nvcoop::RegisterMethodClosure("chat.open", [](const std::string&, std::string& out) {
        if (!g.running) { out = "chat is off"; return false; }
        g.ctl->open(nowMs());
        g.input->setComposing(g.ctl->composing());
        out = std::string("{\"composing\":") + (g.ctl->composing() ? "true" : "false") + "}";
        return true;
    }, true, 0);
    nvcoop::RegisterMethodClosure("chat.type", [](const std::string& params, std::string& out) {
        if (!g.running) { out = "chat is off"; return false; }
        nvcoop::json::Value p;
        std::string err;
        nvcoop::json::parse(params, p, err);
        std::vector<InputEvent> ev;
        for (char c : p.get("text").asString(""))
            ev.push_back(InputEvent{InputType::Char, static_cast<unsigned char>(c)});
        g.ctl->frame(nowMs(), ev);
        out = "{\"text\":\"" + g.ctl->editor().text() + "\"}";
        return true;
    }, true, 0);
    nvcoop::RegisterMethodClosure("chat.key", [](const std::string& params, std::string& out) {
        if (!g.running) { out = "chat is off"; return false; }
        nvcoop::json::Value p;
        std::string err;
        nvcoop::json::parse(params, p, err);
        const std::string k = p.get("key").asString("");
        static const std::pair<const char*, Key> names[] = {{"left", Key::Left}, {"right", Key::Right}, {"home", Key::Home}, {"end", Key::End}, {"delete", Key::Delete},
                                                            {"backspace", Key::Backspace}, {"up", Key::Up}, {"down", Key::Down}, {"enter", Key::Enter}, {"escape", Key::Escape}};
        for (const auto& n : names) {
            if (k != n.first) continue;
            std::vector<InputEvent> ev = {InputEvent{InputType::Key, 0, n.second}};
            g.ctl->frame(nowMs(), ev);
            g.input->setComposing(g.ctl->composing());
            out = std::string("{\"composing\":") + (g.ctl->composing() ? "true" : "false") + "}";
            return true;
        }
        out = "unknown key '" + k + "' (left right home end delete backspace up down enter escape)";
        return false;
    }, true, 0);
    nvcoop::RegisterMethodClosure("chat.system", [](const std::string& params, std::string& out) {
        if (!g.running) { out = "chat is off"; return false; }
        nvcoop::json::Value p;
        std::string err;
        nvcoop::json::parse(params, p, err);
        nvcoop::chat::postSystem(p.get("text").asString(""));
        out = "{}";
        return true;
    }, true, 0);
}

void onMessage(uint32_t type, void*, uint32_t) {
    if (type == NVSEMessagingInterface::kMessage_ExitGame || type == NVSEMessagingInterface::kMessage_ExitGame_Console) stop();
}

}  // namespace

NVCOOP_MODULE(text_chat, "text_chat", onLoad, onMessage, onFrame);

// Hot reload: the hook, the overlay window and the disabled keys are given back before the core is replaced; the new core starts the chat again at its load.
NVCOOP_RELOAD_STATE(text_chat, "text_chat", 1, 47,
    /*canUnload*/ nullptr,
    /*save*/ [](nvreload::Writer& w) { w.boolean(g.running); return true; },
    /*restore*/ [](uint32_t, nvreload::Reader& r) { (void)r.boolean(); return true; },
    /*migrate*/ nullptr,
    /*shutdown*/ [] { stop(); },
    /*fresh*/ nullptr);
