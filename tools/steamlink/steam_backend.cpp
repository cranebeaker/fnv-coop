// The real Steam behind fnvsteam.exe (docs/steam.md): Valve's modern steam_api.dll, loaded by name, only its flat C API (plain __cdecl functions
// that take the interface pointer first), so no Steamworks SDK header or import library is needed. The helper runs as AppID 480 (Valve's
// "Spacewar" sample app, steam_appid.txt next to the exe), which every Steam account may run.
//
// Callbacks use the DLL's manual dispatch: pump() asks Steam for the pending callbacks and turns the few we care about into BackendEvents.
// Struct layouts are those of the 32-bit Windows SDK (callback structs are packed to 8 bytes).
#include "backend.h"

#include <cstdio>
#include <cstring>
#include <map>

#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#endif
#ifndef NOMINMAX
#define NOMINMAX
#endif
#include <windows.h>

namespace fnvlink {
namespace {

using Call = uint64_t;
using Pipe = int32_t;

enum : int {
    kLobbyFriendsOnly = 1,
    kCompareEqual = 0,
    kDistanceWorldwide = 3,
    kFriendImmediate = 4,
    kSendUnreliableNoNagle = 1 | 32,      // NoNagle, AutoRestartBrokenSession
    kBufferSendSize = 9,
    kBufferRecvSize = 47,
    kMaxRecvMessageSize = 49,
    kIdentityTypeSteamId = 16,
    cbGameLobbyJoinRequested = 333,
    cbGameRichPresenceJoinRequested = 337,
    cbLobbyInvite = 503,
    cbLobbyEnter = 504,
    cbLobbyChatUpdate = 506,
    cbLobbyMatchList = 510,
    cbLobbyCreated = 513,
    cbCallCompleted = 703,
    cbSessionRequest = 1251,
    cbSessionFailed = 1252,
    kMemberLeft = 0x2, kMemberDisconnected = 0x4, kMemberKicked = 0x8, kMemberBanned = 0x10,
};

#pragma pack(push, 8)
struct CallbackMsg { int32_t user; int32_t callback; uint8_t* param; int32_t size; };
struct CallCompleted { Call call; int32_t callback; uint32_t size; };
struct LobbyCreatedCb { int32_t result; uint64_t lobby; };
struct LobbyEnterCb { uint64_t lobby; uint32_t perms; bool locked; uint32_t response; };
struct LobbyMatchListCb { uint32_t count; };
struct LobbyChatUpdateCb { uint64_t lobby; uint64_t changed; uint64_t by; uint32_t state; };
struct LobbyJoinRequestedCb { uint64_t lobby; uint64_t friendId; };
struct LobbyInviteCb { uint64_t inviter; uint64_t lobby; uint64_t gameId; };
struct RichJoinRequestedCb { uint64_t friendId; char connect[256]; };
#pragma pack(pop)

struct Identity { uint8_t raw[160]; };   // 136 bytes in the SDK; only ever touched through Valve's own accessors

const int kIdentityOffsets[2] = {16, 12};   // where a received message keeps its sender, for the two struct layouts a 32-bit build can have

class SteamBackend final : public Backend {
public:
    explicit SteamBackend(std::wstring dll) : dllPath_(std::move(dll)) {}

    bool init(std::string& why) override {
        lib_ = LoadLibraryW(dllPath_.c_str());
        if (!lib_) { why = "steam_api.dll is missing next to fnvsteam.exe. Reinstall the co-op release."; return false; }
        if (!bind(why)) return false;
        if (!isSteamRunning_()) { why = "Steam is not running. Start Steam and log in, then try again."; return false; }
        char err[1024] = "";
        const int r = initFlat_(err);
        if (r != 0) {
            why = r == 2 ? "Steam is not running. Start Steam and log in, then try again."
                         : std::string("Could not reach the Steam client (code ") + std::to_string(r) + "). " + err;
            return false;
        }
        manualInit_();
        pipe_ = getPipe_();
        user_ = iface("SteamAPI_SteamUser_v023", why);
        mm_ = iface("SteamAPI_SteamMatchmaking_v009", why);
        friends_ = iface("SteamAPI_SteamFriends_v018", why);
        msgs_ = iface("SteamAPI_SteamNetworkingMessages_SteamAPI_v002", why);
        utils_ = iface("SteamAPI_SteamNetworkingUtils_SteamAPI_v004", why);
        if (!user_ || !mm_ || !friends_ || !msgs_ || !utils_) return false;
        if (!loggedOn_(user_)) { why = "Steam is offline or not logged in. Log in to Steam and try again."; return false; }
        me_ = getId_(user_);
        // a message is at most a UDP packet here, but give Steam room so a burst never stalls
        setInt_(utils_, kBufferSendSize, 8 << 20);
        setInt_(utils_, kBufferRecvSize, 8 << 20);
        setInt_(utils_, kMaxRecvMessageSize, 1 << 20);
        return true;
    }

    SteamId me() override { return me_; }
    std::string myName() override { const char* n = personaName_(friends_); return n ? n : ""; }
    std::string nameOf(SteamId id) override { const char* n = friendName_(friends_, id); return n ? n : ""; }
    bool isFriend(SteamId id) override { return hasFriend_(friends_, id, kFriendImmediate); }

    void pump(std::vector<BackendEvent>& out) override {
        runFrame_(pipe_);
        CallbackMsg m;
        while (nextCb_(pipe_, &m)) {
            if (m.callback == cbCallCompleted && m.size >= static_cast<int32_t>(sizeof(CallCompleted))) {
                CallCompleted done;
                std::memcpy(&done, m.param, sizeof done);
                std::vector<uint8_t> buf(done.size + 16, 0);
                bool failed = false;
                if (callResult_(pipe_, done.call, buf.data(), static_cast<int>(done.size), done.callback, &failed) && !failed) {
                    handle(done.callback, buf.data(), static_cast<int>(done.size), out);
                } else {
                    failCall(done.call, out);
                }
            } else {
                handle(m.callback, m.param, m.size, out);
            }
            freeCb_(pipe_);
        }
    }

    void createLobby(int maxMembers) override { createCall_ = mkLobby_(mm_, kLobbyFriendsOnly, maxMembers); }
    void joinLobby(uint64_t lobby) override { joinCall_ = join_(mm_, lobby); joinWanted_ = lobby; }
    void leaveLobby(uint64_t lobby) override { leave_(mm_, lobby); }
    void findLobbies() override {
        addStr_(mm_, "fnvcoop", "1", kCompareEqual);
        addDist_(mm_, kDistanceWorldwide);
        addCount_(mm_, 50);
        listCall_ = list_(mm_);
    }
    void setLobbyData(uint64_t lobby, const std::string& k, const std::string& v) override { setData_(mm_, lobby, k.c_str(), v.c_str()); }
    std::string lobbyData(uint64_t lobby, const std::string& k) override { const char* v = getData_(mm_, lobby, k.c_str()); return v ? v : ""; }
    SteamId lobbyOwner(uint64_t lobby) override { return owner_(mm_, lobby); }
    std::vector<SteamId> lobbyMembers(uint64_t lobby) override {
        std::vector<SteamId> r;
        const int n = memberCount_(mm_, lobby);
        for (int i = 0; i < n; ++i) r.push_back(memberAt_(mm_, lobby, i));
        return r;
    }
    void setJoinText(const std::string& text) override {
        if (text.empty()) { clearRich_(friends_); return; }
        setRich_(friends_, "connect", text.c_str());
        setRich_(friends_, "status", "FNV Co-Op");
    }
    std::vector<SteamId> onlineFriends() override {
        std::vector<SteamId> r;
        const int n = friendCount_(friends_, kFriendImmediate);
        for (int i = 0; i < n && i < 2000; ++i) {
            const SteamId id = friendAt_(friends_, i, kFriendImmediate);
            if (id && personaState_(friends_, id) != 0) r.push_back(id);   // 0 = offline
        }
        return r;
    }
    bool inviteToLobby(uint64_t lobby, SteamId friendId) override { return lobby && friendId && invite_(mm_, lobby, friendId); }

    bool send(SteamId to, int channel, const uint8_t* data, uint32_t size) override {
        Identity id = identity(to);
        return sendMsg_(msgs_, &id, data, size, kSendUnreliableNoNagle, channel) == 1;
    }
    void receive(int channel, std::vector<Datagram>& out) override {
        void* batch[32];
        for (;;) {
            const int n = recvMsgs_(msgs_, channel, batch, 32);
            for (int i = 0; i < n; ++i) {
                const uint8_t* m = static_cast<const uint8_t*>(batch[i]);
                Datagram d;
                const void* data = nullptr;
                int size = 0;
                std::memcpy(&data, m, sizeof data);
                std::memcpy(&size, m + 4, sizeof size);
                d.from = senderOf(m);
                if (data && size > 0) d.data.assign(static_cast<const uint8_t*>(data), static_cast<const uint8_t*>(data) + size);
                releaseMsg_(batch[i]);
                if (d.from) out.push_back(std::move(d));
            }
            if (n < 32) break;
        }
    }
    void acceptSession(SteamId user) override { Identity id = identity(user); accept_(msgs_, &id); }
    void closeSession(SteamId user) override { Identity id = identity(user); closeSess_(msgs_, &id); }

private:
    template <typename T> bool get(T& fn, const char* name, std::string& why) {
        fn = reinterpret_cast<T>(reinterpret_cast<void*>(GetProcAddress(lib_, name)));
        if (!fn) { why = std::string("steam_api.dll is too old for this release (it lacks ") + name + "). Reinstall the co-op release."; return false; }
        return true;
    }

    bool bind(std::string& why) {
        return get(isSteamRunning_, "SteamAPI_IsSteamRunning", why) && get(initFlat_, "SteamAPI_InitFlat", why) &&
               get(getPipe_, "SteamAPI_GetHSteamPipe", why) && get(manualInit_, "SteamAPI_ManualDispatch_Init", why) &&
               get(runFrame_, "SteamAPI_ManualDispatch_RunFrame", why) && get(nextCb_, "SteamAPI_ManualDispatch_GetNextCallback", why) &&
               get(freeCb_, "SteamAPI_ManualDispatch_FreeLastCallback", why) && get(callResult_, "SteamAPI_ManualDispatch_GetAPICallResult", why) &&
               get(loggedOn_, "SteamAPI_ISteamUser_BLoggedOn", why) && get(getId_, "SteamAPI_ISteamUser_GetSteamID", why) &&
               get(mkLobby_, "SteamAPI_ISteamMatchmaking_CreateLobby", why) && get(join_, "SteamAPI_ISteamMatchmaking_JoinLobby", why) &&
               get(leave_, "SteamAPI_ISteamMatchmaking_LeaveLobby", why) && get(list_, "SteamAPI_ISteamMatchmaking_RequestLobbyList", why) &&
               get(addStr_, "SteamAPI_ISteamMatchmaking_AddRequestLobbyListStringFilter", why) &&
               get(addDist_, "SteamAPI_ISteamMatchmaking_AddRequestLobbyListDistanceFilter", why) &&
               get(addCount_, "SteamAPI_ISteamMatchmaking_AddRequestLobbyListResultCountFilter", why) &&
               get(byIndex_, "SteamAPI_ISteamMatchmaking_GetLobbyByIndex", why) && get(setData_, "SteamAPI_ISteamMatchmaking_SetLobbyData", why) &&
               get(getData_, "SteamAPI_ISteamMatchmaking_GetLobbyData", why) && get(owner_, "SteamAPI_ISteamMatchmaking_GetLobbyOwner", why) &&
               get(memberCount_, "SteamAPI_ISteamMatchmaking_GetNumLobbyMembers", why) &&
               get(memberAt_, "SteamAPI_ISteamMatchmaking_GetLobbyMemberByIndex", why) &&
               get(personaName_, "SteamAPI_ISteamFriends_GetPersonaName", why) && get(friendName_, "SteamAPI_ISteamFriends_GetFriendPersonaName", why) &&
               get(hasFriend_, "SteamAPI_ISteamFriends_HasFriend", why) && get(setRich_, "SteamAPI_ISteamFriends_SetRichPresence", why) &&
               get(clearRich_, "SteamAPI_ISteamFriends_ClearRichPresence", why) &&
               get(friendCount_, "SteamAPI_ISteamFriends_GetFriendCount", why) && get(friendAt_, "SteamAPI_ISteamFriends_GetFriendByIndex", why) &&
               get(personaState_, "SteamAPI_ISteamFriends_GetFriendPersonaState", why) &&
               get(invite_, "SteamAPI_ISteamMatchmaking_InviteUserToLobby", why) &&
               get(setIdentity_, "SteamAPI_SteamNetworkingIdentity_SetSteamID64", why) && get(getIdentity_, "SteamAPI_SteamNetworkingIdentity_GetSteamID64", why) &&
               get(sendMsg_, "SteamAPI_ISteamNetworkingMessages_SendMessageToUser", why) &&
               get(recvMsgs_, "SteamAPI_ISteamNetworkingMessages_ReceiveMessagesOnChannel", why) &&
               get(accept_, "SteamAPI_ISteamNetworkingMessages_AcceptSessionWithUser", why) &&
               get(closeSess_, "SteamAPI_ISteamNetworkingMessages_CloseSessionWithUser", why) &&
               get(releaseMsg_, "SteamAPI_SteamNetworkingMessage_t_Release", why) &&
               get(setInt_, "SteamAPI_ISteamNetworkingUtils_SetGlobalConfigValueInt32", why);
    }

    void* iface(const char* name, std::string& why) {
        using Getter = void*(__cdecl*)();
        Getter g = nullptr;
        if (!get(g, name, why)) return nullptr;
        void* p = g();
        if (!p) why = std::string("Steam did not give out the interface ") + name + ".";
        return p;
    }

    Identity identity(SteamId id) {
        Identity i;
        std::memset(&i, 0, sizeof i);
        setIdentity_(&i, id);
        return i;
    }

    SteamId senderOf(const uint8_t* msg) {
        for (int off : kIdentityOffsets) {
            int32_t type = 0;
            std::memcpy(&type, msg + off, sizeof type);
            if (type == kIdentityTypeSteamId) return getIdentity_(reinterpret_cast<const Identity*>(msg + off));
        }
        return 0;
    }

    void failCall(Call call, std::vector<BackendEvent>& out) {
        BackendEvent e;
        e.ok = false;
        e.why = "Steam did not answer.";
        if (call == createCall_) { e.type = BackendEvent::LobbyCreated; createCall_ = 0; }
        else if (call == joinCall_) { e.type = BackendEvent::LobbyEntered; e.lobby = joinWanted_; joinCall_ = 0; }
        else if (call == listCall_) { e.type = BackendEvent::LobbyList; listCall_ = 0; }
        else return;
        out.push_back(e);
    }

    static const char* enterWhy(uint32_t r) {
        switch (r) {
            case 3: return "Steam would not let you into that lobby.";
            case 4: return "The party is full.";
            case 5: return "Steam reported an error joining the lobby.";
            case 6: case 9: return "Steam would not let you into that lobby (banned).";
            case 10: case 11: return "Steam would not let you into that lobby (blocked).";
            default: return "The host's lobby is closed, or it is not your friend's.";
        }
    }

    void handle(int cb, const uint8_t* p, int size, std::vector<BackendEvent>& out) {
        BackendEvent e;
        switch (cb) {
            case cbLobbyCreated: {
                if (size < static_cast<int>(sizeof(LobbyCreatedCb))) return;
                LobbyCreatedCb c;
                std::memcpy(&c, p, sizeof c);
                e.type = BackendEvent::LobbyCreated;
                e.ok = c.result == 1 && c.lobby != 0;
                e.lobby = c.lobby;
                if (!e.ok) e.why = "Steam could not create the lobby (result " + std::to_string(c.result) + ").";
                createCall_ = 0;
                out.push_back(e);
                break;
            }
            case cbLobbyEnter: {
                if (size < static_cast<int>(sizeof(LobbyEnterCb))) return;
                LobbyEnterCb c;
                std::memcpy(&c, p, sizeof c);
                e.type = BackendEvent::LobbyEntered;
                e.ok = c.response == 1;
                e.lobby = c.lobby;
                if (!e.ok) e.why = enterWhy(c.response);
                joinCall_ = 0;
                out.push_back(e);
                break;
            }
            case cbLobbyMatchList: {
                if (size < static_cast<int>(sizeof(LobbyMatchListCb))) return;
                LobbyMatchListCb c;
                std::memcpy(&c, p, sizeof c);
                e.type = BackendEvent::LobbyList;
                for (uint32_t i = 0; i < c.count && i < 200; ++i) e.lobbies.push_back(byIndex_(mm_, static_cast<int>(i)));
                listCall_ = 0;
                out.push_back(e);
                break;
            }
            case cbLobbyChatUpdate: {
                if (size < static_cast<int>(sizeof(LobbyChatUpdateCb))) return;
                LobbyChatUpdateCb c;
                std::memcpy(&c, p, sizeof c);
                if (!(c.state & (kMemberLeft | kMemberDisconnected | kMemberKicked | kMemberBanned))) return;
                e.type = BackendEvent::MemberLeft;
                e.lobby = c.lobby;
                e.user = c.changed;
                out.push_back(e);
                break;
            }
            case cbGameLobbyJoinRequested: {
                if (size < static_cast<int>(sizeof(LobbyJoinRequestedCb))) return;
                LobbyJoinRequestedCb c;
                std::memcpy(&c, p, sizeof c);
                e.type = BackendEvent::JoinRequest;
                e.lobby = c.lobby;
                e.user = c.friendId;
                out.push_back(e);
                break;
            }
            case cbLobbyInvite: {   // a friend invited us while we run: the launcher asks the player
                if (size < static_cast<int>(sizeof(LobbyInviteCb))) return;
                LobbyInviteCb c;
                std::memcpy(&c, p, sizeof c);
                e.type = BackendEvent::JoinRequest;
                e.user = c.inviter;
                e.lobby = c.lobby;
                e.connect = "invite";
                out.push_back(e);
                break;
            }
            case cbGameRichPresenceJoinRequested: {
                if (size < static_cast<int>(sizeof(RichJoinRequestedCb))) return;
                RichJoinRequestedCb c;
                std::memcpy(&c, p, sizeof c);
                c.connect[255] = 0;
                e.type = BackendEvent::JoinRequest;
                e.user = c.friendId;
                e.connect = c.connect;
                out.push_back(e);
                break;
            }
            case cbSessionRequest:
            case cbSessionFailed: {
                if (size < 136) return;
                e.type = cb == cbSessionRequest ? BackendEvent::SessionRequest : BackendEvent::SessionFailed;
                e.user = getIdentity_(reinterpret_cast<const Identity*>(p));
                out.push_back(e);
                break;
            }
            default: break;
        }
    }

    using Cdecl = void*;
    std::wstring dllPath_;
    HMODULE lib_ = nullptr;
    Pipe pipe_ = 0;
    SteamId me_ = 0;
    void *user_ = nullptr, *mm_ = nullptr, *friends_ = nullptr, *msgs_ = nullptr, *utils_ = nullptr;
    Call createCall_ = 0, joinCall_ = 0, listCall_ = 0;
    uint64_t joinWanted_ = 0;

    bool(__cdecl* isSteamRunning_)() = nullptr;
    int(__cdecl* initFlat_)(char*) = nullptr;
    Pipe(__cdecl* getPipe_)() = nullptr;
    void(__cdecl* manualInit_)() = nullptr;
    void(__cdecl* runFrame_)(Pipe) = nullptr;
    bool(__cdecl* nextCb_)(Pipe, CallbackMsg*) = nullptr;
    void(__cdecl* freeCb_)(Pipe) = nullptr;
    bool(__cdecl* callResult_)(Pipe, Call, void*, int, int, bool*) = nullptr;
    bool(__cdecl* loggedOn_)(void*) = nullptr;
    SteamId(__cdecl* getId_)(void*) = nullptr;
    Call(__cdecl* mkLobby_)(void*, int, int) = nullptr;
    Call(__cdecl* join_)(void*, uint64_t) = nullptr;
    void(__cdecl* leave_)(void*, uint64_t) = nullptr;
    Call(__cdecl* list_)(void*) = nullptr;
    void(__cdecl* addStr_)(void*, const char*, const char*, int) = nullptr;
    void(__cdecl* addDist_)(void*, int) = nullptr;
    void(__cdecl* addCount_)(void*, int) = nullptr;
    uint64_t(__cdecl* byIndex_)(void*, int) = nullptr;
    bool(__cdecl* setData_)(void*, uint64_t, const char*, const char*) = nullptr;
    const char*(__cdecl* getData_)(void*, uint64_t, const char*) = nullptr;
    SteamId(__cdecl* owner_)(void*, uint64_t) = nullptr;
    int(__cdecl* memberCount_)(void*, uint64_t) = nullptr;
    SteamId(__cdecl* memberAt_)(void*, uint64_t, int) = nullptr;
    const char*(__cdecl* personaName_)(void*) = nullptr;
    const char*(__cdecl* friendName_)(void*, SteamId) = nullptr;
    bool(__cdecl* hasFriend_)(void*, SteamId, int) = nullptr;
    bool(__cdecl* setRich_)(void*, const char*, const char*) = nullptr;
    void(__cdecl* clearRich_)(void*) = nullptr;
    int(__cdecl* friendCount_)(void*, int) = nullptr;
    SteamId(__cdecl* friendAt_)(void*, int, int) = nullptr;
    int(__cdecl* personaState_)(void*, SteamId) = nullptr;
    bool(__cdecl* invite_)(void*, uint64_t, SteamId) = nullptr;
    void(__cdecl* setIdentity_)(Identity*, uint64_t) = nullptr;
    uint64_t(__cdecl* getIdentity_)(const Identity*) = nullptr;
    int(__cdecl* sendMsg_)(void*, const Identity*, const void*, uint32_t, int, int) = nullptr;
    int(__cdecl* recvMsgs_)(void*, int, void**, int) = nullptr;
    bool(__cdecl* accept_)(void*, const Identity*) = nullptr;
    bool(__cdecl* closeSess_)(void*, const Identity*) = nullptr;
    void(__cdecl* releaseMsg_)(void*) = nullptr;
    bool(__cdecl* setInt_)(void*, int, int32_t) = nullptr;
};

}  // namespace

std::unique_ptr<Backend> makeSteamBackend(const std::wstring& dllPath) { return std::make_unique<SteamBackend>(dllPath); }

}  // namespace fnvlink
