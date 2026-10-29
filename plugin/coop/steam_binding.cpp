#include "steam_binding.h"

#include <cstring>
#include <deque>
#include <mutex>

#if defined(_WIN32) && !defined(_WIN64)
#define NVCOOP_STEAM_LIVE 1
#include <windows.h>
#endif

namespace nvcoop::coop {

#ifndef NVCOOP_STEAM_LIVE

// Not the game's architecture: there is no Steam to reach. The build stays whole; joining over Steam just reports why it is off.
SteamBinding& SteamBinding::instance() { static SteamBinding b; return b; }
bool SteamBinding::attach(void*, std::string* why) { if (why) *why = "Steam joining needs the 32-bit game."; return false; }
bool SteamBinding::attachToGame(std::string* why) { return attach(nullptr, why); }
std::string SteamBinding::personaName() const { return {}; }
fnvcoop::SteamId SteamBinding::me() { return 0; }
uint32_t SteamBinding::nowMs() { return 0; }
bool SteamBinding::send(fnvcoop::SteamId, const uint8_t*, uint32_t, bool, int) { return false; }
bool SteamBinding::read(fnvcoop::Bytes&, fnvcoop::SteamId&, int) { return false; }
void SteamBinding::accept(fnvcoop::SteamId) {}
bool SteamBinding::isFriend(fnvcoop::SteamId) { return false; }
bool SteamBinding::takeSessionRequest(fnvcoop::SteamId&) { return false; }
bool SteamBinding::takeConnectFail(fnvcoop::SteamId&, uint8_t&) { return false; }
size_t SteamBinding::queuedBytes(fnvcoop::SteamId) { return 0; }
void SteamBinding::setHostingPresence(bool, const std::string&, const std::string&) {}
std::vector<SteamFriend> SteamBinding::hostingFriends(const std::string&) { return {}; }
bool SteamBinding::takeJoinRequest(fnvcoop::SteamId&, std::string&) { return false; }
int SteamBinding::friendCount() { return 0; }

#else

namespace {
// ---- interface layouts: Steamworks SDK vtable slots, each (HSteamUser, HSteamPipe, version) asked from the Steam client ----
// ISteamClient (SteamClient009 era): GetISteamUser 5, GetISteamFriends 8, GetISteamNetworking 17.
constexpr int kClientUser = 5, kClientFriends = 8, kClientNetworking = 17;
// ISteamUser "SteamUser013": GetSteamID 2.
constexpr const char* kUserVersion = "SteamUser013";
constexpr int kUserSteamId = 2;
// ISteamNetworking "SteamNetworking006": SendP2PPacket 0 (to, data, size, send type: 0 unreliable, 2 reliable; channel), IsP2PPacketAvailable 1,
// ReadP2PPacket 2, AcceptP2PSessionWithUser 3, GetP2PSessionState 6 (P2PSessionState_t: 4 flag bytes, then int32 bytes queued), AllowP2PPacketRelay 7.
constexpr const char* kNetworkingVersion = "SteamNetworking006";
constexpr int kSend = 0, kAvailable = 1, kRead = 2, kAccept = 3, kSessionState = 6, kAllowRelay = 7;
// ISteamFriends "SteamFriends018": GetPersonaName 0, GetFriendCount 2 (flags: 4 regular friends), GetFriendByIndex 3, GetFriendRelationship 4
// (3: friend), GetFriendPersonaName 6, GetFriendGamePlayed 7 (FriendGameInfo_t, 24 bytes: game id first, app id in its low 24 bits),
// SetRichPresence 41, ClearRichPresence 42, GetFriendRichPresence 43.
constexpr const char* kFriendsVersion = "SteamFriends018";
constexpr int kPersonaName = 0, kFriendCount = 2, kFriendByIndex = 3, kRelationship = 4, kFriendName = 6, kFriendGame = 7,
              kSetPresence = 41, kClearPresence = 42, kFriendPresence = 43;
// Callbacks: P2PSessionRequest_t 1202 (the CSteamID), P2PSessionConnectFail_t 1203 (CSteamID, then a uint8 reason; 16 bytes),
// GameRichPresenceJoinRequested_t 337 (CSteamID friend, char connect[256]).
constexpr int kSessionRequest = 1202, kConnectFail = 1203, kJoinRequested = 337;

void* g_client = nullptr;
void* g_user = nullptr;
void* g_friends = nullptr;
void* g_networking = nullptr;
bool g_attached = false;
bool g_gameOk = false;
std::string g_gameWhy;

template <typename Fn>
Fn Slot(void* iface, int index) {
    return reinterpret_cast<Fn>((*reinterpret_cast<void***>(iface))[index]);
}

// Steam's CCallbackBase, declared as Valve's headers do (MSVC lays the vtable out the same way for both).
class CallbackBase {
public:
    virtual void Run(void* param) = 0;
    virtual void Run(void* param, bool ioFailure, uint64_t call) = 0;
    virtual int GetCallbackSizeBytes() = 0;

protected:
    uint8_t m_flags = 0;
    int m_id = 0;
};

std::mutex g_queueMutex;
std::deque<fnvcoop::SteamId> g_requests;
std::deque<std::pair<fnvcoop::SteamId, uint8_t>> g_fails;   // who, Steam's reason
std::deque<std::pair<fnvcoop::SteamId, std::string>> g_joins;   // who, their connect string

class SessionRequest : public CallbackBase {
public:
    void Run(void* param) override {
        fnvcoop::SteamId who;
        std::memcpy(&who, param, sizeof(who));
        std::lock_guard<std::mutex> lock(g_queueMutex);
        if (g_requests.size() < 64) g_requests.push_back(who);
    }
    void Run(void* param, bool, uint64_t) override { Run(param); }
    int GetCallbackSizeBytes() override { return 8; }
} g_sessionRequest;

class ConnectFail : public CallbackBase {
public:
    void Run(void* param) override {
        fnvcoop::SteamId who;
        std::memcpy(&who, param, sizeof(who));
        const uint8_t reason = static_cast<const uint8_t*>(param)[8];
        std::lock_guard<std::mutex> lock(g_queueMutex);
        if (g_fails.size() < 64) g_fails.push_back({who, reason});
    }
    void Run(void* param, bool, uint64_t) override { Run(param); }
    int GetCallbackSizeBytes() override { return 16; }
} g_connectFail;

class JoinRequested : public CallbackBase {
public:
    void Run(void* param) override {
        fnvcoop::SteamId who;
        std::memcpy(&who, param, sizeof(who));
        const char* connect = static_cast<const char*>(param) + 8;
        std::string text(connect, strnlen(connect, 255));
        std::lock_guard<std::mutex> lock(g_queueMutex);
        if (g_joins.size() < 16) g_joins.push_back({who, std::move(text)});
    }
    void Run(void* param, bool, uint64_t) override { Run(param); }
    int GetCallbackSizeBytes() override { return 264; }
} g_joinRequested;

std::string FriendName(fnvcoop::SteamId id) {
    const char* n = Slot<const char*(__thiscall*)(void*, fnvcoop::SteamId)>(g_friends, kFriendName)(g_friends, id);
    return n ? n : "";
}
std::string FriendPresence(fnvcoop::SteamId id, const char* key) {
    const char* v = Slot<const char*(__thiscall*)(void*, fnvcoop::SteamId, const char*)>(g_friends, kFriendPresence)(g_friends, id, key);
    return v ? v : "";
}
}  // namespace

SteamBinding& SteamBinding::instance() {
    static SteamBinding b;
    return b;
}

bool SteamBinding::attach(void* module, std::string* why) {
    auto fail = [&](const char* m) { if (why) *why = m; return false; };
    if (ready_) return true;
    HMODULE dll = static_cast<HMODULE>(module);
    auto steamClient = dll ? reinterpret_cast<void*(__cdecl*)()>(GetProcAddress(dll, "SteamClient")) : nullptr;
    auto steamUser = dll ? reinterpret_cast<int(__cdecl*)()>(GetProcAddress(dll, "SteamAPI_GetHSteamUser")) : nullptr;
    auto steamPipe = dll ? reinterpret_cast<int(__cdecl*)()>(GetProcAddress(dll, "SteamAPI_GetHSteamPipe")) : nullptr;
    auto registerCallback = dll ? reinterpret_cast<void(__cdecl*)(void*, int)>(GetProcAddress(dll, "SteamAPI_RegisterCallback")) : nullptr;
    if (!steamClient || !steamUser || !steamPipe || !registerCallback)
        return fail("The game's steam_api.dll is missing or is not Steam's, so joining over Steam is off.");
    g_client = steamClient();
    const int user = steamUser(), pipe = steamPipe();
    if (!g_client || !user || !pipe) return fail("Steam is not running or you are not logged in, so joining over Steam is off.");
    using GetInterface = void*(__thiscall*)(void*, int, int, const char*);
    g_user = Slot<GetInterface>(g_client, kClientUser)(g_client, user, pipe, kUserVersion);
    g_friends = Slot<GetInterface>(g_client, kClientFriends)(g_client, user, pipe, kFriendsVersion);
    g_networking = Slot<GetInterface>(g_client, kClientNetworking)(g_client, user, pipe, kNetworkingVersion);
    if (!g_user || !g_friends || !g_networking) return fail("Your Steam client is too old or too new for co-op over Steam.");
    Slot<bool(__thiscall*)(void*, bool)>(g_networking, kAllowRelay)(g_networking, true);
    if (!g_attached) {
        registerCallback(&g_sessionRequest, kSessionRequest);
        registerCallback(&g_connectFail, kConnectFail);
        registerCallback(&g_joinRequested, kJoinRequested);
        g_attached = true;
    }
    ready_ = true;
    return true;
}

bool SteamBinding::attachToGame(std::string* why) {
    // Only a success is remembered: if the game has not started its own Steam yet, a later try (the co-op session starts lazily, on a game frame) may work.
    if (!g_gameOk) g_gameOk = attach(GetModuleHandleA("steam_api.dll"), &g_gameWhy);
    if (!g_gameOk && why) *why = g_gameWhy;
    return g_gameOk;
}

std::string SteamBinding::personaName() const {
    if (!ready_) return {};
    const char* n = Slot<const char*(__thiscall*)(void*)>(g_friends, kPersonaName)(g_friends);
    return n ? n : "";
}

fnvcoop::SteamId SteamBinding::me() {
    fnvcoop::SteamId id = 0;
    if (ready_) Slot<void*(__thiscall*)(void*, fnvcoop::SteamId*)>(g_user, kUserSteamId)(g_user, &id);
    return id;
}

uint32_t SteamBinding::nowMs() { return GetTickCount(); }

bool SteamBinding::send(fnvcoop::SteamId to, const uint8_t* data, uint32_t size, bool reliable, int channel) {
    if (!ready_) return false;
    return Slot<bool(__thiscall*)(void*, fnvcoop::SteamId, const void*, uint32_t, int, int)>(g_networking, kSend)(g_networking, to, data, size,
                                                                                                                   reliable ? 2 : 0, channel);
}

bool SteamBinding::read(fnvcoop::Bytes& out, fnvcoop::SteamId& from, int channel) {
    if (!ready_) return false;
    uint32_t size = 0;
    if (!Slot<bool(__thiscall*)(void*, uint32_t*, int)>(g_networking, kAvailable)(g_networking, &size, channel)) return false;
    out.resize(size ? (size < (1u << 20) ? size : (1u << 20)) : 1);
    uint32_t got = 0;
    if (!Slot<bool(__thiscall*)(void*, void*, uint32_t, uint32_t*, fnvcoop::SteamId*, int)>(g_networking, kRead)(
            g_networking, out.data(), static_cast<uint32_t>(out.size()), &got, &from, channel))
        return false;
    out.resize(got < out.size() ? got : out.size());
    return true;
}

void SteamBinding::accept(fnvcoop::SteamId who) {
    if (ready_) Slot<bool(__thiscall*)(void*, fnvcoop::SteamId)>(g_networking, kAccept)(g_networking, who);
}

bool SteamBinding::isFriend(fnvcoop::SteamId who) {
    return ready_ && Slot<int(__thiscall*)(void*, fnvcoop::SteamId)>(g_friends, kRelationship)(g_friends, who) == 3;
}

bool SteamBinding::takeSessionRequest(fnvcoop::SteamId& who) {
    std::lock_guard<std::mutex> lock(g_queueMutex);
    if (g_requests.empty()) return false;
    who = g_requests.front();
    g_requests.pop_front();
    return true;
}

bool SteamBinding::takeConnectFail(fnvcoop::SteamId& who, uint8_t& reason) {
    std::lock_guard<std::mutex> lock(g_queueMutex);
    if (g_fails.empty()) return false;
    who = g_fails.front().first;
    reason = g_fails.front().second;
    g_fails.pop_front();
    return true;
}

bool SteamBinding::takeJoinRequest(fnvcoop::SteamId& who, std::string& connect) {
    std::lock_guard<std::mutex> lock(g_queueMutex);
    if (g_joins.empty()) return false;
    who = g_joins.front().first;
    connect = std::move(g_joins.front().second);
    g_joins.pop_front();
    return true;
}

size_t SteamBinding::queuedBytes(fnvcoop::SteamId to) {
    if (!ready_) return 0;
    uint8_t state[64] = {};
    if (!Slot<bool(__thiscall*)(void*, fnvcoop::SteamId, void*)>(g_networking, kSessionState)(g_networking, to, state)) return 0;
    int32_t queued = 0;
    std::memcpy(&queued, state + 4, sizeof(queued));
    return queued > 0 ? static_cast<size_t>(queued) : 0;
}

void SteamBinding::setHostingPresence(bool on, const std::string& tag, const std::string& mode) {
    if (!ready_) return;
    auto set = Slot<bool(__thiscall*)(void*, const char*, const char*)>(g_friends, kSetPresence);
    if (!on) {
        Slot<void(__thiscall*)(void*)>(g_friends, kClearPresence)(g_friends);
        return;
    }
    set(g_friends, kPresenceKey, ("host " + tag).c_str());
    set(g_friends, kPresenceMode, mode.substr(0, 64).c_str());
    set(g_friends, kPresenceConnect, ("+fnvcoop_join " + std::to_string(me())).c_str());
}

int SteamBinding::friendCount() {
    return ready_ ? Slot<int(__thiscall*)(void*, int)>(g_friends, kFriendCount)(g_friends, 4) : 0;
}

std::vector<SteamFriend> SteamBinding::hostingFriends(const std::string& wantedTag) {
    std::vector<SteamFriend> hosting;
    if (!ready_) return hosting;
    const int count = friendCount();
    for (int i = 0; i < count && i < 2000; ++i) {
        fnvcoop::SteamId id = 0;
        Slot<void*(__thiscall*)(void*, fnvcoop::SteamId*, int, int)>(g_friends, kFriendByIndex)(g_friends, &id, i, 4);
        uint8_t game[24] = {};
        if (!id || !Slot<bool(__thiscall*)(void*, fnvcoop::SteamId, void*)>(g_friends, kFriendGame)(g_friends, id, game)) continue;
        const std::string presence = FriendPresence(id, kPresenceKey);
        if (presence.compare(0, 5, "host ") != 0) continue;
        SteamFriend f;
        f.id = id;
        f.name = FriendName(id);
        if (f.name.empty()) f.name = fnvcoop::SteamTransport::name(id);
        f.mode = FriendPresence(id, kPresenceMode).substr(0, 64);
        f.sameRelease = wantedTag.empty() || presence == "host " + wantedTag;
        hosting.push_back(std::move(f));
    }
    return hosting;
}

#endif  // NVCOOP_STEAM_LIVE

}  // namespace nvcoop::coop
