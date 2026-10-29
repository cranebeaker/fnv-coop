// nvcoop.dll - xNVSE plugin entry points + the real (game-backed) bridge Host.
// Built with the xNVSE headers (header-only use; nothing from NVSE's .cpp files is linked).
#include "nvse/PluginAPI.h"
#include "nvse/nvse_version.h"
#include "nvse/GameData.h"

#include <cstdio>
#include <cstring>
#include <cstdlib>
#include <string>

#include "bridge/bridge.h"
#include "common/nvcoop_api.h"
#include "common/nvse_opcode_base.h"
#include "common/registry.h"
#include "bridge/screenshot.h"
#ifdef NVCOOP_WITH_FOCUS_GUARD
#include "splitscreen/focus_guard.h"
#endif
#ifdef NVCOOP_LOADER
#include "loader/loader_host.h"   // split build: this DLL is the permanent loader, the modules live in nvcoop_mod.dll
#endif

#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#endif
#include <windows.h>
#include <shlobj.h>
#include <vector>
#include <atomic>
#include <thread>

#ifdef NVCOOP_WITH_SNAPSHOT
#include "fnvcoop_snapshot/snapshot.h"
#endif
#ifdef NVCOOP_WITH_LIVE
#include "coop/live/live_glue.h"   // dll-wiring: nvcoop.ini [live] switches for the enemy / NPC / avatar sync (all OFF by default)
#endif

namespace {

using nvcoop::Log;

constexpr UInt32 kConsoleManager_GetSingleton = 0x0071B160;  // cdecl ConsoleManager*(bool create)
constexpr UInt32 kConsoleOutputFileOffset = 0x810;            // char[260], see NVSE GameAPI.h ConsoleManager

PluginHandle g_pluginHandle = kPluginHandle_Invalid;
NVSEInterface* g_nvse = nullptr;
NVSEMessagingInterface* g_msg = nullptr;
NVSEConsoleInterface* g_console = nullptr;

nvcoop::bridge::Bridge g_bridge;
#ifdef NVCOOP_LOADER
nvcoop::loader::CoreHost* g_coreHost = nullptr;
#endif
std::string g_gameDir;

std::string IniPath() { return g_gameDir + "Data\\NVSE\\Plugins\\nvcoop.ini"; }

std::string ReadIni(const char* section, const char* key, const char* def) {
    char buf[512];
    GetPrivateProfileStringA(section, key, def, buf, sizeof buf, IniPath().c_str());
    return buf;
}

std::string EnvOr(const char* name, const std::string& fallback) {
    char buf[512];
    DWORD n = GetEnvironmentVariableA(name, buf, sizeof buf);
    return (n > 0 && n < sizeof buf) ? std::string(buf) : fallback;
}

// The folder the game saves into, decided the way the game does: the Fallout.ini it reads (My Games first, else the one next to
// the exe) says bUseMyGamesDirectory (default 1: <Documents>\My Games\FalloutNV\) and SLocalSavePath (default Saves\).
std::string GameSaveDir() {
    char docs[MAX_PATH] = {0};
    std::string myGames;
    if (SUCCEEDED(SHGetFolderPathA(nullptr, CSIDL_PERSONAL, nullptr, 0, docs))) myGames = std::string(docs) + "\\My Games\\FalloutNV\\";
    std::string ini = myGames + "Fallout.ini";
    if (myGames.empty() || GetFileAttributesA(ini.c_str()) == INVALID_FILE_ATTRIBUTES) ini = g_gameDir + "Fallout.ini";
    const bool useMyGames = GetPrivateProfileIntA("General", "bUseMyGamesDirectory", 1, ini.c_str()) != 0 && !myGames.empty();
    char local[MAX_PATH];
    GetPrivateProfileStringA("General", "SLocalSavePath", "Saves\\", local, sizeof local, ini.c_str());
    std::string dir = (useMyGames ? myGames : g_gameDir) + local;
    if (!dir.empty() && dir.back() != '\\' && dir.back() != '/') dir += '\\';
    return dir;
}

// [coop] autostart=start (the launcher lobby): the party starts from the start save we ship (nvcoop_start.fos, taken in a clean
// game right after the opening movie, black screen at Doc Mitchell's, quest timer stopped). Copy it into the game's save folder;
// the bridge then loads it with the console's LoadGame as soon as the main menu is up. The coop module holds everyone on the black
// screen until the whole party is in, then plays the opening movie for all at once (coop_module.cpp, IntroFrame).
constexpr const char* kStartSave = "nvcoop_start";
bool PrepareStartSave(std::string& err) {
    const std::string src = g_gameDir + "Data\\NVSE\\Plugins\\" + kStartSave + ".fos";
    if (GetFileAttributesA(src.c_str()) == INVALID_FILE_ATTRIBUTES) { err = "missing " + src; return false; }
    const std::string dir = GameSaveDir();
    CreateDirectoryA(dir.c_str(), nullptr);
    const std::string dst = dir + kStartSave + ".fos";
    if (!CopyFileA(src.c_str(), dst.c_str(), FALSE)) { err = "copy to " + dst + " failed: " + std::to_string(GetLastError()); return false; }
    Log("autostart: start save copied to %s", dst.c_str());
    return true;
}

// [coop] autostart=load:<save name> (the launcher's CONTINUE, docs/party-save.md): a save name the console's LoadGame can take, and the file must be in the
// game's save folder. A name with a path, a quote or a control character is refused: it goes into a console line.
bool LoadableSaveName(const std::string& name, std::string& err) {
    if (name.empty() || name.size() > 150) { err = "empty or too long"; return false; }
    for (unsigned char c : name)
        if (c < 32 || c == 127 || std::strchr("\\/:*?\"<>|", c)) { err = "not a plain save name"; return false; }
    const std::string path = GameSaveDir() + name + ".fos";
    if (GetFileAttributesA(path.c_str()) == INVALID_FILE_ATTRIBUTES) { err = "no such save: " + path; return false; }
    return true;
}

// --- multiInstance: per-process name for the game's single-instance mutex (own process only) ---
// FalloutNV.exe calls CreateMutexA("FalloutNVGameMutex") and refuses to start if it already exists. With
// [bridge] multiInstance=1 we patch this exe's kernel32 IAT entry so that name gets a "-<pid>" suffix.
using CreateMutexA_t = HANDLE(WINAPI*)(LPSECURITY_ATTRIBUTES, BOOL, LPCSTR);
CreateMutexA_t g_realCreateMutexA = nullptr;
std::atomic<int> g_mutexRenamed{0};
std::atomic<DWORD> g_mutexLastError{0};
std::atomic<uintptr_t> g_mutexHandle{0};

HANDLE WINAPI CreateMutexA_Hook(LPSECURITY_ATTRIBUTES sa, BOOL owner, LPCSTR name) {
    if (name && std::strcmp(name, "FalloutNVGameMutex") == 0) {
        char buf[64];
        std::snprintf(buf, sizeof buf, "FalloutNVGameMutex-%lu", GetCurrentProcessId());
        g_mutexRenamed++;
        HANDLE h = g_realCreateMutexA(sa, owner, buf);
        g_mutexLastError = GetLastError();  // 183 = ERROR_ALREADY_EXISTS (would still block the start)
        g_mutexHandle = reinterpret_cast<uintptr_t>(h);
        Log("multiInstance: CreateMutexA(FalloutNVGameMutex) renamed to %s -> %p err=%lu (183=ALREADY_EXISTS)", buf, h, g_mutexLastError.load());
        SetLastError(g_mutexLastError);
        return h;
    }
    return g_realCreateMutexA(sa, owner, name);
}

// Returns true if the IAT slot was found and patched.
bool HookExeCreateMutexA() {
    auto base = reinterpret_cast<BYTE*>(GetModuleHandleA(nullptr));
    auto dos = reinterpret_cast<IMAGE_DOS_HEADER*>(base);
    auto nt = reinterpret_cast<IMAGE_NT_HEADERS*>(base + dos->e_lfanew);
    const auto& dir = nt->OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_IMPORT];
    if (!dir.VirtualAddress) return false;
    FARPROC target = GetProcAddress(GetModuleHandleA("kernel32.dll"), "CreateMutexA");
    for (auto imp = reinterpret_cast<IMAGE_IMPORT_DESCRIPTOR*>(base + dir.VirtualAddress); imp->Name; ++imp) {
        if (_stricmp(reinterpret_cast<char*>(base + imp->Name), "kernel32.dll") != 0) continue;
        auto thunk = reinterpret_cast<IMAGE_THUNK_DATA*>(base + imp->FirstThunk);
        for (; thunk->u1.Function; ++thunk) {
            if (reinterpret_cast<FARPROC>(thunk->u1.Function) != target) continue;
            DWORD old;
            if (!VirtualProtect(&thunk->u1.Function, sizeof(void*), PAGE_READWRITE, &old)) return false;
            g_realCreateMutexA = reinterpret_cast<CreateMutexA_t>(thunk->u1.Function);
            thunk->u1.Function = reinterpret_cast<ULONG_PTR>(&CreateMutexA_Hook);
            VirtualProtect(&thunk->u1.Function, sizeof(void*), old, &old);
            return true;
        }
    }
    return false;
}


// --- multiInstance diagnostics: LOG-ONLY IAT hooks (own process) showing which check ends WinMain ---
std::atomic<int> g_diagCount{0};
bool DiagBudget() { return g_diagCount.fetch_add(1) < 60; }

template <class F>
bool PatchImport(const char* dll, const char* fn, void* hook, F& real) {
    auto base = reinterpret_cast<BYTE*>(GetModuleHandleA(nullptr));
    auto dos = reinterpret_cast<IMAGE_DOS_HEADER*>(base);
    auto nt = reinterpret_cast<IMAGE_NT_HEADERS*>(base + dos->e_lfanew);
    const auto& dir = nt->OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_IMPORT];
    if (!dir.VirtualAddress) return false;
    HMODULE m = GetModuleHandleA(dll);
    FARPROC target = m ? GetProcAddress(m, fn) : nullptr;
    if (!target) return false;
    for (auto imp = reinterpret_cast<IMAGE_IMPORT_DESCRIPTOR*>(base + dir.VirtualAddress); imp->Name; ++imp) {
        if (_stricmp(reinterpret_cast<char*>(base + imp->Name), dll) != 0) continue;
        auto thunk = reinterpret_cast<IMAGE_THUNK_DATA*>(base + imp->FirstThunk);
        for (; thunk->u1.Function; ++thunk) {
            if (reinterpret_cast<FARPROC>(thunk->u1.Function) != target) continue;
            DWORD old;
            if (!VirtualProtect(&thunk->u1.Function, sizeof(void*), PAGE_READWRITE, &old)) return false;
            real = reinterpret_cast<F>(thunk->u1.Function);
            thunk->u1.Function = reinterpret_cast<ULONG_PTR>(hook);
            VirtualProtect(&thunk->u1.Function, sizeof(void*), old, &old);
            return true;
        }
    }
    return false;
}

using CreateMutexW_t = HANDLE(WINAPI*)(LPSECURITY_ATTRIBUTES, BOOL, LPCWSTR);
using OpenMutexA_t = HANDLE(WINAPI*)(DWORD, BOOL, LPCSTR);
using OpenMutexW_t = HANDLE(WINAPI*)(DWORD, BOOL, LPCWSTR);
using FindWindowA_t = HWND(WINAPI*)(LPCSTR, LPCSTR);
using FindWindowW_t = HWND(WINAPI*)(LPCWSTR, LPCWSTR);
CreateMutexW_t g_realCreateMutexW = nullptr;
OpenMutexA_t g_realOpenMutexA = nullptr;
OpenMutexW_t g_realOpenMutexW = nullptr;
FindWindowA_t g_realFindWindowA = nullptr;
FindWindowW_t g_realFindWindowW = nullptr;

std::string Narrow(LPCWSTR w) {
    if (!w) return "(null)";
    char b[260];
    int n = WideCharToMultiByte(CP_UTF8, 0, w, -1, b, sizeof b - 1, nullptr, nullptr);
    b[n > 0 ? n - 1 : 0] = 0;
    return b;
}
const char* Z(LPCSTR s) { return s ? s : "(null)"; }

HANDLE WINAPI Diag_CreateMutexW(LPSECURITY_ATTRIBUTES sa, BOOL o, LPCWSTR n) {
    HANDLE h = g_realCreateMutexW(sa, o, n);
    DWORD e = GetLastError();
    if (DiagBudget()) Log("diag CreateMutexW(%s) -> %p err=%lu", Narrow(n).c_str(), h, e);
    SetLastError(e);
    return h;
}
HANDLE WINAPI Diag_OpenMutexA(DWORD a, BOOL i, LPCSTR n) {
    HANDLE h = g_realOpenMutexA(a, i, n);
    DWORD e = GetLastError();
    if (DiagBudget()) Log("diag OpenMutexA(%s) -> %p err=%lu", Z(n), h, e);
    SetLastError(e);
    return h;
}
HANDLE WINAPI Diag_OpenMutexW(DWORD a, BOOL i, LPCWSTR n) {
    HANDLE h = g_realOpenMutexW(a, i, n);
    DWORD e = GetLastError();
    if (DiagBudget()) Log("diag OpenMutexW(%s) -> %p err=%lu", Narrow(n).c_str(), h, e);
    SetLastError(e);
    return h;
}
// NEUTRALIZED (multiInstance, own process only). WinMain (0x86A850) calls FindWindowA(*(char**)0x11A2FE8 /*the game's window class*/, NULL)
// right after the mutex check; if a window of that class exists (the other instance) it calls SetForegroundWindow and returns 0
// silently ("returned from winmain (0)"). We answer NULL for exactly that call (class == the game's own class name, no title).
HWND WINAPI Diag_FindWindowA(LPCSTR c, LPCSTR t) {
    HWND h = g_realFindWindowA(c, t);
    const char* gameClass = *reinterpret_cast<const char* const*>(0x11A2FE8);
    bool isGameClass = c && !t && gameClass && lstrcmpA(c, gameClass) == 0;
    if (DiagBudget()) Log("diag FindWindowA(class=%s title=%s) -> %p%s", Z(c), Z(t), h, (isGameClass && h) ? "  [NEUTRALIZED -> NULL: another instance's window]" : "");
    if (isGameClass) return nullptr;
    return h;
}
HWND WINAPI Diag_FindWindowW(LPCWSTR c, LPCWSTR t) {
    HWND h = g_realFindWindowW(c, t);
    if (DiagBudget()) Log("diag FindWindowW(class=%s title=%s) -> %p", Narrow(c).c_str(), Narrow(t).c_str(), h);
    return h;
}

void InstallMutexDiagHooks() {
    bool a = PatchImport("kernel32.dll", "CreateMutexW", reinterpret_cast<void*>(&Diag_CreateMutexW), g_realCreateMutexW);
    bool b = PatchImport("kernel32.dll", "OpenMutexA", reinterpret_cast<void*>(&Diag_OpenMutexA), g_realOpenMutexA);
    bool c = PatchImport("kernel32.dll", "OpenMutexW", reinterpret_cast<void*>(&Diag_OpenMutexW), g_realOpenMutexW);
    bool d = PatchImport("user32.dll", "FindWindowA", reinterpret_cast<void*>(&Diag_FindWindowA), g_realFindWindowA);
    bool e = PatchImport("user32.dll", "FindWindowW", reinterpret_cast<void*>(&Diag_FindWindowW), g_realFindWindowW);
    Log("multiInstance diag hooks (log-only): CreateMutexW=%d OpenMutexA=%d OpenMutexW=%d FindWindowA=%d FindWindowW=%d", a, b, c, d, e);
}

// Isolated in its own function: __try cannot live in a function that has C++ objects needing unwinding.
// thisObj = the console's CALLING REFERENCE (nullptr = none). The game's console script compiler accepts "player.X" but
// not "<hexFormID>.X" (Script command "00103E61.GetOpenState" not found) nor the EditorID of a non-persistent ref, so
// bridge exec {cmd, ref} passes the reference here instead (as plugin/hooks/apply_lines.h does for Track A).
int RunLineSEH(const char* line, TESObjectREFR* thisObj) {
    __try {
        return g_console->RunScriptLine2(line, thisObj, false) ? 1 : 0;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return -1;
    }
}
int RunLineLogged(const char* line, TESObjectREFR* thisObj) {
    const int rc = RunLineSEH(line, thisObj);
    // A fault swallowed here skips the engine's cleanup (the temporary Script on xNVSE's stack is never destroyed), so it can set up a
    // later crash far away from this line (issue #33). Name it in the log.
    if (rc < 0) Log("console line faulted and was abandoned: %.200s", line);
    return rc;
}

// ---- exec {cmd, ref}: the engine's load order and form map (the same data plugin/hooks' DefaultResolver reads) ----
constexpr UInt32 kDataHandlerPtr = 0x011C3F2C;  // DataHandler* (NVSE DataHandler::Get for runtime 1.4.0.525)
constexpr UInt32 kLookupFormByID = 0x004839C0;  // cdecl TESForm*(UInt32 refID); prologue pinned in plugin/hooks/engine_addrs.h

// Load-order index = position in DataHandler::modList.modInfoList (NVSE GetModIndex, plugin/hooks FormResolver::fromRef).
int PluginIndexSEH(const char* name) {
    __try {
        DataHandler* dh = *reinterpret_cast<DataHandler**>(kDataHandlerPtr);
        if (!dh) return -1;
        int i = 0;
        for (auto it = dh->modList.modInfoList.Begin(); !it.End(); ++it, ++i)
            if (it.Get() && _stricmp(it.Get()->name, name) == 0) return i;
        return -1;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return -1;
    }
}

TESForm* LookupFormSEH(UInt32 id) {
    __try {
        return id ? reinterpret_cast<TESForm*(__cdecl*)(UInt32)>(kLookupFormByID)(id) : nullptr;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return nullptr;
    }
}

int FormTypeSEH(UInt32 id) {
    __try {
        TESForm* f = LookupFormSEH(id);
        return f ? static_cast<int>(f->typeID) : -1;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return -1;
    }
}

// Command handlers only print their result (e.g. "player.getpos x") when g_consoleOpen is set AND the
// per-thread "executing from console" byte (TLS slot, +0x268) is set - NVSE's IsConsoleMode(). We set both
// for the duration of one synchronous console line and restore them afterwards (no code patching).
constexpr UInt32 kConsoleOpenFlag = 0x11DEA2E;
constexpr UInt32 kTlsIndexAddr = 0x126FD98;
constexpr UInt32 kTlsConsoleModeOffset = 0x268;
constexpr UInt32 kEchoConsoleFlag = 0x011F158C;  // bEchoConsole (NVSE GetConsoleEcho)
bool g_d3dShot = false;
bool g_consoleEcho = true;  // set bEchoConsole + console-mode flags around exec so plain command results are printed

struct ConsoleModeScope {
    BYTE* openFlag = nullptr;
    BYTE* tlsFlag = nullptr;
    BYTE savedOpen = 0, savedTls = 0, savedEcho = 0;
    bool active = false;
    ConsoleModeScope() {
        if (!g_consoleEcho) return;
        openFlag = reinterpret_cast<BYTE*>(kConsoleOpenFlag);
        void** tls = reinterpret_cast<void**>(__readfsdword(0x2C));
        void* slot = tls ? tls[*reinterpret_cast<DWORD*>(kTlsIndexAddr)] : nullptr;
        if (!slot) { openFlag = nullptr; return; }
        tlsFlag = reinterpret_cast<BYTE*>(slot) + kTlsConsoleModeOffset;
        savedOpen = *openFlag; savedTls = *tlsFlag;
        savedEcho = *reinterpret_cast<BYTE*>(kEchoConsoleFlag);
        *openFlag = 1; *tlsFlag = 1; *reinterpret_cast<BYTE*>(kEchoConsoleFlag) = 1;
        active = true;
        static int n = 0;
        if (n++ < 3) Log("consoleMode scope: open was %u tls was %u echo was %u", savedOpen, savedTls, savedEcho);
    }
    ~ConsoleModeScope() {
        if (active) { *openFlag = savedOpen; *tlsFlag = savedTls; *reinterpret_cast<BYTE*>(kEchoConsoleFlag) = savedEcho; }
    }
};

char* ConsoleOutputFileBuf() {
    using Fn = void* (*)(bool);
    void* mgr = reinterpret_cast<Fn>(kConsoleManager_GetSingleton)(true);
    return mgr ? reinterpret_cast<char*>(mgr) + kConsoleOutputFileOffset : nullptr;
}


// ---- keep the game window out of the owner's way: minimize without activating, never steal focus ----
std::atomic<bool> g_stopMinimizer{false};
std::thread g_minimizer;
std::atomic<bool> g_minimize{false};
std::atomic<bool> g_focusGuard{false};  // plugin/splitscreen/focus_guard.h: deactivation must not open the pause menu
std::atomic<int> g_windowSide{0};       // [bridge] windowSide=left|right (two copies on one PC): -1 left, 1 right, 0 leave the window where the game puts it

// Two copies on one PC start on top of each other. Put this copy's window on its side of its monitor, keeping the size and never activating it.
// The game moves its window itself at start-up (and re-creates it after the splash), so a new window is held there for its first seconds only;
// after that the player may move it.
constexpr DWORD kWindowHoldMs = 5000;
void PlaceWindow(HWND h) {
    static HWND s_win = nullptr;
    static DWORD s_firstSeen = 0;
    if (h != s_win) { s_win = h; s_firstSeen = GetTickCount(); }
    if (GetTickCount() - s_firstSeen > kWindowHoldMs) return;
    RECT wr;
    MONITORINFO mi = {sizeof mi};
    if (!GetWindowRect(h, &wr) || !GetMonitorInfoA(MonitorFromWindow(h, MONITOR_DEFAULTTONEAREST), &mi)) return;
    const LONG w = wr.right - wr.left, ht = wr.bottom - wr.top;
    if (w <= 0 || ht <= 0) return;
    const RECT& a = mi.rcWork;
    const LONG x = g_windowSide.load() < 0 ? a.left : (std::max)(a.left, a.right - w);
    const LONG y = a.top + (std::max)(0L, (a.bottom - a.top - ht) / 2);
    if (wr.left == x && wr.top == y) return;
    SetWindowPos(h, nullptr, x, y, 0, 0, SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE);
}

BOOL CALLBACK MinimizeEnumProc(HWND h, LPARAM) {
    DWORD pid = 0;
    GetWindowThreadProcessId(h, &pid);
    if (pid != GetCurrentProcessId() || !IsWindowVisible(h) || GetWindow(h, GW_OWNER) != nullptr) return TRUE;
    if (g_windowSide.load() != 0 && !IsIconic(h)) {
        char wcls[128] = {0};
        const char* gcls = *reinterpret_cast<const char* const*>(0x11A2FE8);  // see Diag_FindWindowA
        if (gcls && GetClassNameA(h, wcls, sizeof(wcls)) && lstrcmpA(wcls, gcls) == 0) PlaceWindow(h);
    }
#ifdef NVCOOP_WITH_FOCUS_GUARD
    // Before minimizing: minimizing deactivates the window, and in gameplay the engine answers that with the pause menu.
    char cls[128] = {0};
    const char* gameClass = *reinterpret_cast<const char* const*>(0x11A2FE8);  // see Diag_FindWindowA
    if (g_focusGuard.load() && nvsplit::Installed() != h && GetClassNameA(h, cls, sizeof(cls)) && gameClass &&
        lstrcmpA(cls, gameClass) == 0) {
        Log("focus guard %s on window %p", nvsplit::Install(h) ? "installed" : "NOT installed", h);
    }
#endif
    if (g_minimize.load() && !IsIconic(h)) ShowWindowAsync(h, SW_SHOWMINNOACTIVE);
    return TRUE;
}

#ifdef NVCOOP_WITH_FOCUS_GUARD
bool FocusStatusMethod(const std::string&, std::string& out) {
    char buf[160];
    std::snprintf(buf, sizeof(buf), "{\"enabled\":%s,\"installed\":%s,\"swallowed\":%lu}", g_focusGuard.load() ? "true" : "false",
                  nvsplit::Installed() ? "true" : "false", nvsplit::Swallowed());
    out = buf;
    return true;
}
#endif

void MinimizerLoop() {
    while (!g_stopMinimizer.load()) {
        EnumWindows(MinimizeEnumProc, 0);
        Sleep(100);
    }
}

#ifdef NVCOOP_WITH_SNAPSHOT
std::string StateProviderImpl(const std::string& sectionsCsv) {
    fnvcoop::snapshot::SnapshotOptions o;
    o.playerId = "host";
    o.instance = "host";
    o.indent = -1;
    {
        std::string wr = EnvOr("FNVCOOP_WATCHREFS", ReadIni("bridge", "watchRefs", ""));
        if (!wr.empty()) o.watchRefsFile = wr;
    }
    if (!sectionsCsv.empty()) {
        o.shared = sectionsCsv.find("shared") != std::string::npos;
        o.perPlayer = sectionsCsv.find("perPlayer") != std::string::npos;
    }
    return fnvcoop::snapshot::SnapshotJson(o);
}
#endif


// ---- direct engine save (addresses from L5's static RE of 1.4.0.525, see internal lane notes) ----
constexpr UInt32 kSaveLoadMgrPtr = 0x11DE134;     // BGSSaveLoadGame* (SaveGameManager)
constexpr UInt32 kSaveGameFn = 0x8503B0;          // bool __thiscall (mgr, const char* name, int slot, bool flag)
constexpr UInt32 kCanSaveNowFn = 0x850FE0;        // bool __thiscall (mgr, bool ignoreMenus)
constexpr UInt32 kPlayerPtr = 0x11DEA3C;
constexpr UInt32 kBusyObj = 0x11F2250;

struct SaveAttempt { int ret = -1; int canSave0 = -1, canSave1 = -1; unsigned playerFlags = 0, busy = 0, dead = 0; bool exception = false; };

SaveAttempt DirectSaveSEH(const char* name) {
    SaveAttempt a;
    __try {
        void* mgr = *reinterpret_cast<void**>(kSaveLoadMgrPtr);
        if (!mgr) { a.ret = -2; return a; }
        auto canSave = reinterpret_cast<bool(__thiscall*)(void*, bool)>(kCanSaveNowFn);
        a.canSave0 = canSave(mgr, false);
        a.canSave1 = canSave(mgr, true);
        BYTE* player = *reinterpret_cast<BYTE**>(kPlayerPtr);
        if (player) {
            a.playerFlags = *reinterpret_cast<UInt32*>(player + 0x798);
            a.dead = *reinterpret_cast<UInt32*>(player + 0x108);
        }
        a.busy = *reinterpret_cast<UInt32*>(kBusyObj + 8);
        a.ret = reinterpret_cast<bool(__thiscall*)(void*, const char*, int, bool)>(kSaveGameFn)(mgr, name, -1, false) ? 1 : 0;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        a.exception = true;
    }
    return a;
}

class GameHost : public nvcoop::bridge::Host {
public:
    nvcoop::bridge::ExecResult Exec(const std::string& line) override { return ExecLine(line, nullptr); }

    // exec {cmd, ref}: the bridge core parses the ref and checks the form type through these two, then calls ExecOn.
    int PluginIndex(const std::string& plugin) override { return PluginIndexSEH(plugin.c_str()); }
    int FormType(uint32_t runtimeId) override { return FormTypeSEH(runtimeId); }

    nvcoop::bridge::ExecResult ExecOn(const std::string& line, uint32_t runtimeId) override {
        TESForm* form = LookupFormSEH(runtimeId);  // the bridge checked its type on this main thread a moment ago
        if (!form) {
            nvcoop::bridge::ExecResult r;
            r.output = "reference is no longer loaded";
            return r;
        }
        return ExecLine(line, reinterpret_cast<TESObjectREFR*>(form));
    }

    nvcoop::bridge::ExecResult ExecLine(const std::string& line, TESObjectREFR* thisObj) {
        nvcoop::bridge::ExecResult r;
        if (!g_console) return r;
        // Capture console output: the engine mirrors console prints into ConsoleManager::COFileName
        // when it is set. (UNVERIFIED in-game; if the file never appears, output stays "".)
        char* cof = ConsoleOutputFileBuf();
        char saved[260] = {0};
        std::string cap;
        if (cof) {
            std::memcpy(saved, cof, sizeof saved);
            cap = g_gameDir + "nvcoop_console_" + std::to_string(GetCurrentProcessId()) + ".txt";
            DeleteFileA(cap.c_str());
            std::strncpy(cof, cap.c_str(), 259);
            cof[259] = 0;
        }
        int rc;
        {
            ConsoleModeScope echo;
            rc = RunLineLogged(line.c_str(), thisObj);
        }
        if (cof) {
            std::memcpy(cof, saved, sizeof saved);
            if (FILE* f = std::fopen(cap.c_str(), "rb")) {
                char buf[4096];
                size_t n;
                while ((n = std::fread(buf, 1, sizeof buf, f)) > 0) r.output.append(buf, n);
                std::fclose(f);
                DeleteFileA(cap.c_str());
            }
            while (!r.output.empty() && (r.output.back() == '\n' || r.output.back() == '\r')) r.output.pop_back();
        }
        r.ok = rc > 0;
        if (rc < 0) r.output = "exception while running console line";
        return r;
    }

    bool LoadSave(const std::string& name, std::string& err) override {
        // Console "LoadGame <name>" (save name without extension). UNVERIFIED from main menu.
        auto r = Exec("LoadGame \"" + name + "\"");
        if (!r.ok) err = "console LoadGame failed: " + r.output;
        return r.ok;
    }

    bool SaveExists(const std::string& name) override {
        // The game may save to Documents\My Games\FalloutNV\Saves or (bUseMyGamesDirectory=0) <game dir>\Saves.
        std::vector<std::string> dirs = {g_gameDir + "Saves\\"};
        char docs[MAX_PATH];
        if (SUCCEEDED(SHGetFolderPathA(nullptr, CSIDL_PERSONAL, nullptr, 0, docs)))
            dirs.push_back(std::string(docs) + "\\My Games\\FalloutNV\\Saves\\");
        for (auto& d : dirs)
            if (GetFileAttributesA((d + name + ".fos").c_str()) != INVALID_FILE_ATTRIBUTES) return true;
        return false;
    }

    bool SaveGame(const std::string& name, std::string& err) override {
#ifdef NVCOOP_WITH_LIVE
        nvcoop::livewire::glueBeforeSave();   // clones made by the actor port must not be written into the save
#endif
        SaveAttempt a = DirectSaveSEH(name.c_str());
#ifdef NVCOOP_WITH_LIVE
        nvcoop::livewire::glueAfterSave();
#endif
        Log("saveGame(%s): ret=%d canSave(menus)=%d canSave(ignoreMenus)=%d playerFlags798=%08X dead108=%u busy=%u exc=%d",
            name.c_str(), a.ret, a.canSave0, a.canSave1, a.playerFlags, a.dead, a.busy, a.exception);
        if (a.ret == 1) return true;  // NVSE's SaveGame message completes the bridge job
        char buf[256];
        std::snprintf(buf, sizeof buf, "engine SaveGame refused (ret=%d canSave=%d/%d playerFlags=%08X dead=%u busy=%u%s)", a.ret,
                      a.canSave0, a.canSave1, a.playerFlags, a.dead, a.busy, a.exception ? " EXCEPTION" : "");
        err = buf;
        return false;
    }

    bool Screenshot(std::string& path, std::string& err) override {
        if (!g_d3dShot) {
            err = "screenshot unavailable: no console command exists and the D3D9 device lookup is experimental (ini [bridge] screenshotD3D9=1, may crash)";
            return false;
        }
        static int n = 0;
        if (path.empty()) path = g_gameDir + "Data\\NVSE\\Plugins\\nvcoop_shot_" + std::to_string(GetCurrentProcessId()) + "_" + std::to_string(++n) + ".png";
        nvcoop::ShotInfo info;
        if (!nvcoop::CaptureBackbufferPng(path, info, err)) return false;
        return true;
    }

    void Quit() override {
        Log("nvbridge: quit requested");
        // 'qqq' is the console quit; fall back to hard exit if the console line is rejected.
        if (!Exec("qqq").ok) ExitProcess(0);
    }
};


#ifdef NVCOOP_WITH_SNAPSHOT
// "plugins": loaded mod list with load-order indices (tests compare it with tools/questdata/loadorder.json).
bool PluginsMethod(const std::string&, std::string& out) {
    DataHandler* dh = DataHandler::Get();
    if (!dh) { out = "DataHandler not available"; return false; }
    out = "{\"count\":" + std::to_string(dh->GetActiveModCount()) + ",\"plugins\":[";
    const UInt32 n = dh->GetActiveModCount();
    for (UInt32 i = 0, first = 1; i < 255 && i < n + 8; ++i) {
        const char* name = dh->GetNthModName(i);
        if (!name || !*name) continue;
        if (!first) out += ",";
        first = 0;
        out += "{\"index\":" + std::to_string(i) + ",\"name\":\"" + name + "\"}";
    }
    out += "]}";
    return true;
}
#endif

// Crash note: write exception code/address/module+offset to nvcoop.log from inside the dying process, then
// let the normal handlers (WER, NVSE) continue. Tools collect nvcoop.log / nvse.log afterwards.
// Copy n bytes of possibly unmapped memory. Raw pointers only (SEH).
bool CrashPeek(const void* from, void* to, size_t n) {
    __try {
        std::memcpy(to, from, n);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

// The stack words around ESP as hex and text. An EIP made of text (issue #33: 0x6C676E61 "angl") was popped from [esp-4], so these
// lines show the whole string that overwrote it and the frames next to it.
void LogCrashStack(const CONTEXT* c) {
#ifdef _M_IX86
    const uintptr_t esp = c->Esp;
    Log("CRASH regs eax=%08lX ebx=%08lX ecx=%08lX edx=%08lX esi=%08lX edi=%08lX ebp=%08lX esp=%08lX", c->Eax, c->Ebx, c->Ecx, c->Edx,
        c->Esi, c->Edi, c->Ebp, c->Esp);
    for (int row = -2; row < 6; ++row) {
        const uintptr_t at = esp + static_cast<intptr_t>(row) * 16;
        uint32_t w[4] = {};
        if (!CrashPeek(reinterpret_cast<const void*>(at), w, sizeof w)) continue;
        char text[17] = {};
        const auto* b = reinterpret_cast<const unsigned char*>(w);
        for (int i = 0; i < 16; ++i) text[i] = (b[i] >= 0x20 && b[i] < 0x7F) ? static_cast<char>(b[i]) : '.';
        Log("CRASH stack %08lX: %08X %08X %08X %08X  %s", static_cast<unsigned long>(at), w[0], w[1], w[2], w[3], text);
    }
#else
    (void)c;
#endif
}

LONG WINAPI CrashFilter(EXCEPTION_POINTERS* ep) {
    const auto* r = ep->ExceptionRecord;
    char mod[MAX_PATH] = "?";
    HMODULE hm = nullptr;
    DWORD_PTR off = 0;
    if (GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
                           static_cast<LPCSTR>(r->ExceptionAddress), &hm)) {
        GetModuleFileNameA(hm, mod, MAX_PATH);
        off = reinterpret_cast<DWORD_PTR>(r->ExceptionAddress) - reinterpret_cast<DWORD_PTR>(hm);
    }
    Log("CRASH code=%08lX addr=%p module=%s offset=%lX frames=%llu", r->ExceptionCode, r->ExceptionAddress, mod,
        static_cast<unsigned long>(off), static_cast<unsigned long long>(g_bridge.frames()));
    if (ep->ContextRecord) LogCrashStack(ep->ContextRecord);
    nvcoop::PushEvent("game.crash");
    return EXCEPTION_CONTINUE_SEARCH;
}

GameHost g_host;

void OnMessage(NVSEMessagingInterface::Message* msg) {
    static bool seen[64] = {};
    if (msg->type < 64 && !seen[msg->type] && msg->type != NVSEMessagingInterface::kMessage_MainGameLoop) {
        seen[msg->type] = true;
        Log("first NVSE message type=%u", msg->type);
    }
    static bool frameSeen = false;
    if (!frameSeen && msg->type == NVSEMessagingInterface::kMessage_MainGameLoop) { frameSeen = true; Log("first main loop frame"); if (g_mutexRenamed.load() || g_mutexLastError.load()) Log("multiInstance: mutex renamed=%d handle=%p lastError=%lu (183=ALREADY_EXISTS)", g_mutexRenamed.load(), reinterpret_cast<void*>(g_mutexHandle.load()), g_mutexLastError.load()); }
    switch (msg->type) {
    case NVSEMessagingInterface::kMessage_DeferredInit:
        g_bridge.NotifyMainMenuReady();
        break;
    case NVSEMessagingInterface::kMessage_MainGameLoop:
        g_bridge.Pump(g_host);
        nvcoop::RunFrameCallbacks(nvcoop::registry::Modules());   // frameOrder: the co-op session runs after the capture modules (nvcoop_api.h)
#ifdef NVCOOP_LOADER
        if (g_coreHost) g_coreHost->OnFrame();   // also performs a pending dev.reload / file-watcher reload
#endif
        return;  // do not forward the per-frame message to modules
    case NVSEMessagingInterface::kMessage_PostLoadGame:
        if (msg->data) g_bridge.NotifyGameLoaded();
        else nvcoop::PushEvent("game.loadFailed");
        break;
    case NVSEMessagingInterface::kMessage_PreLoadGame:
        nvcoop::PushEvent("game.preLoad");
        break;
    case NVSEMessagingInterface::kMessage_SaveGame:
        g_bridge.NotifySaved(msg->data ? static_cast<const char*>(msg->data) : "");
        break;
    case NVSEMessagingInterface::kMessage_ExitToMainMenu:
        nvcoop::PushEvent("game.exitToMenu");
        break;
    case NVSEMessagingInterface::kMessage_ExitGame:
    case NVSEMessagingInterface::kMessage_ExitGame_Console:
        nvcoop::PushEvent("game.exit");
        g_stopMinimizer = true;
        g_bridge.Stop();
        break;
    default:
        break;
    }
    for (auto& m : nvcoop::registry::Modules())
        if (m.onMessage) m.onMessage(msg->type, msg->data, msg->dataLen);
#ifdef NVCOOP_LOADER
    if (g_coreHost) g_coreHost->OnMessage(msg->type, msg->data, msg->dataLen);
#endif
}

}  // namespace

extern "C" {

__declspec(dllexport) bool NVSEPlugin_Query(const NVSEInterface* nvse, PluginInfo* info) {
    info->infoVersion = PluginInfo::kInfoVersion;
    info->name = "nvcoop";
    info->version = 1;
    if (nvse->isEditor) return false;
    if (nvse->nvseVersion < MAKE_NEW_VEGAS_VERSION(6, 3, 0)) return false;  // headers are newer; the used APIs exist since 6.3
    if (nvse->runtimeVersion < RUNTIME_VERSION_1_4_0_525) return false;
    if (nvse->isNogore) return false;
    return true;
}

__declspec(dllexport) bool NVSEPlugin_Load(NVSEInterface* nvse) {
    g_nvse = nvse;
    nvse->SetOpcodeBase(nvcoop::opcodeBaseToRegister());   // without a base NVSE never calls our save callback: no cosave record (nvse_opcode_base.h)
    g_pluginHandle = nvse->GetPluginHandle();
    const char* rd = nvse->GetRuntimeDirectory();
    g_gameDir = rd ? rd : "";
    if (!g_gameDir.empty() && g_gameDir.back() != '\\' && g_gameDir.back() != '/') g_gameDir += '\\';
    nvcoop::registry::SetLogFile(g_gameDir + "Data\\NVSE\\Plugins\\nvcoop.log");
    Log("nvcoop %d loading (NVSE %08X, runtime %08X)", 1, nvse->nvseVersion, nvse->runtimeVersion);

    g_msg = static_cast<NVSEMessagingInterface*>(nvse->QueryInterface(kInterface_Messaging));
    g_console = static_cast<NVSEConsoleInterface*>(nvse->QueryInterface(kInterface_Console));
    if (!g_msg || !g_console) {
        Log("nvcoop: missing NVSE interfaces (msg=%p console=%p)", g_msg, g_console);
        return false;
    }
    g_msg->RegisterListener(g_pluginHandle, "NVSE", OnMessage);
    // Cosave: NVSE takes serialization callbacks only now, during the first load. Register forwarders once; modules (or, in the
    // split build, the current core through the loader) set the real callbacks with nvcoop::SetCosaveCallbacks.
    if (auto* ser = static_cast<NVSESerializationInterface*>(nvse->QueryInterface(kInterface_Serialization))) {
        static NVSESerializationInterface* s_ser = ser;
        ser->SetSaveCallback(g_pluginHandle, [](void*) { nvcoop::RunCosave(true, s_ser); });
        ser->SetLoadCallback(g_pluginHandle, [](void*) { nvcoop::RunCosave(false, s_ser); });
    }
    SetUnhandledExceptionFilter(CrashFilter);

    if (EnvOr("NVCOOP_MULTI_INSTANCE", ReadIni("bridge", "multiInstance", "0")) == "1") {
        bool hooked = HookExeCreateMutexA();
        InstallMutexDiagHooks();
        // If the mutex already exists in this process, the hook came too late; report it so the runner sees it.
        HANDLE existing = OpenMutexA(SYNCHRONIZE, FALSE, "FalloutNVGameMutex");
        Log("multiInstance: CreateMutexA IAT hook %s; game mutex %s at plugin load", hooked ? "installed" : "NOT FOUND",
            existing ? "ALREADY EXISTS" : "not yet created");
        if (existing) CloseHandle(existing);
    }

    nvcoop::bridge::Config cfg;
    cfg.instance = std::atoi(EnvOr("NVCOOP_INSTANCE", ReadIni("bridge", "instance", "0")).c_str());
    cfg.autoloadSave = EnvOr("NVCOOP_AUTOLOAD", ReadIni("bridge", "autoload", ""));
    cfg.autoloadDelayFrames = std::atoi(EnvOr("NVCOOP_AUTOLOAD_FRAMES", ReadIni("bridge", "autoloadDelayFrames", "600")).c_str());
    // [coop] autostart=start: written by the launcher when the lobby starts the party. One-shot: cleared at once, so the next
    // plain start of the game opens the main menu as usual.
    const std::string autostart = ReadIni("coop", "enable", "0") == "1" ? ReadIni("coop", "autostart", "") : std::string();
    if (autostart == "start") {
        WritePrivateProfileStringA("coop", "autostart", "", IniPath().c_str());
        std::string err;
        if (PrepareStartSave(err)) {
            cfg.autoloadSave = kStartSave;
            cfg.autoloadDelayFrames = 30;
            Log("autostart: the party's start save loads when the main menu is up (launcher lobby)");
        } else {
            Log("autostart: no start save (%s); the main menu stays", err.c_str());
        }
    } else if (autostart.compare(0, 5, "load:") == 0) {
        // autostart=load:<save name> (the launcher's CONTINUE): the host loads its co-op save, a guest loads its own save for that host save. Same one-shot.
        WritePrivateProfileStringA("coop", "autostart", "", IniPath().c_str());
        const std::string name = autostart.substr(5);
        std::string err;
        if (LoadableSaveName(name, err)) {
            cfg.autoloadSave = name;
            cfg.autoloadDelayFrames = 30;
            Log("autostart: the saved game '%s' loads when the main menu is up (launcher lobby, continue)", name.c_str());
        } else {
            Log("autostart: not loading '%s' (%s); the main menu stays", name.c_str(), err.c_str());
        }
    }
#ifdef NVCOOP_WITH_SNAPSHOT
    nvcoop::RegisterMethod("plugins", PluginsMethod, true);
#endif
    g_d3dShot = ReadIni("bridge", "screenshotD3D9", "0") == "1";
    g_consoleEcho = EnvOr("NVCOOP_CONSOLE_ECHO", ReadIni("bridge", "consoleEcho", "1")) != "0";
    if (!g_bridge.Start(cfg)) Log("nvbridge failed to start");
#ifdef NVCOOP_WITH_SNAPSHOT
    nvcoop::SetStateProvider(StateProviderImpl);
#endif
    // Off unless asked for: players must see their game. The test runner (tools/runner/instances.py) writes minimized=1 itself.
    // With the old default of 1 a player's window was minimized every 100 ms and could not be restored (alpha 0.1.0, 2026-10-29).
    g_minimize = EnvOr("NVCOOP_MINIMIZED", ReadIni("bridge", "minimized", "0")) != "0";
#ifdef NVCOOP_WITH_FOCUS_GUARD
    // Default on whenever the plugin minimizes the window; split-screen sets it explicitly for visible copies.
    g_focusGuard = EnvOr("NVCOOP_FOCUS_GUARD", ReadIni("bridge", "focusGuard", g_minimize.load() ? "1" : "0")) != "0";
    nvcoop::RegisterMethod("focus.status", FocusStatusMethod, false);
#endif
    {
        const std::string side = ReadIni("bridge", "windowSide", "");
        g_windowSide = side == "left" ? -1 : side == "right" ? 1 : 0;
    }
    if (g_minimize.load() || g_focusGuard.load() || g_windowSide.load() != 0) {
        g_minimizer = std::thread(MinimizerLoop);
        Log("window thread started (minimize=%d focusGuard=%d windowSide=%d)", g_minimize.load() ? 1 : 0, g_focusGuard.load() ? 1 : 0,
            g_windowSide.load());
    }

#ifdef NVCOOP_WITH_LIVE
    // [live] switches and the [actors] tuning are read BEFORE any module loads, so a lane that installs something at load time already knows
    // whether it is switched on, whatever the module order is.
    nvcoop::livewire::glueConfigure(IniPath());
#endif
    for (auto& m : nvcoop::registry::Modules()) {
        Log("module %s", m.name ? m.name : "?");
        if (m.onLoad) m.onLoad(nvse);
    }
#ifdef NVCOOP_LOADER
    {
        // The loader owns no modules; load the reloadable core (everything else) and optionally watch it.
        HMODULE self = nullptr;
        char me[MAX_PATH] = {0};
        GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
                           reinterpret_cast<LPCSTR>(&NVSEPlugin_Load), &self);
        GetModuleFileNameA(self, me, MAX_PATH);
        std::string dir = me;
        dir = dir.substr(0, dir.find_last_of("\\/") + 1);
        nvcoop::loader::HostEnv env;
        env.gameDir = g_gameDir;
        env.corePath = dir + ReadIni("loader", "core", "nvcoop_mod.dll");
        env.nvse = nvse;
        g_coreHost = new nvcoop::loader::CoreHost(env);   // lives for the whole process
        std::string err;
        if (!g_coreHost->Load(err)) Log("loader: core not loaded (%s); dev.reload can retry", err.c_str());
        if (EnvOr("NVCOOP_WATCH", ReadIni("loader", "watch", "0")) == "1")
            g_coreHost->StartWatcher(std::atoi(ReadIni("loader", "watchIntervalMs", "500").c_str()));
    }
#endif
    return true;
}

}  // extern "C"
