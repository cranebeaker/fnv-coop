// What fnvsteam.exe needs from "Steam", in one small interface (docs/steam.md). Two implementations sit behind it:
//   steam_backend.cpp  the real thing: Valve's modern steam_api.dll, loaded by name, flat C API, AppID 480
//   mock_backend.cpp   no Steam: players on one PC share a folder (lobbies) and localhost UDP (messages); what the tests run against
// Everything is called from one thread (the helper's main loop). Results of asynchronous Steam calls come back as events from pump().
#pragma once
#include <cstdint>
#include <memory>
#include <set>
#include <string>
#include <vector>

namespace fnvlink {

using SteamId = uint64_t;

struct BackendEvent {
    enum Type {
        LobbyCreated,     // ok, lobby (why when !ok)
        LobbyEntered,     // ok, lobby (why when !ok): the answer to joinLobby
        LobbyList,        // lobbies: the answer to findLobbies
        MemberLeft,       // lobby, user: someone left (or dropped from) a lobby we are in; for the owner it means the lobby is gone
        JoinRequest,      // user (the friend), lobby (0 when only a connect text is known), connect: the player clicked Join Game or accepted an
                          // invite in Steam; connect "invite" = a friend invited us while the helper runs (nothing clicked yet)
        SessionRequest,   // user: a first message from this user is waiting for acceptSession()
        SessionFailed,    // user: the route to this user broke
    } type = LobbyCreated;
    bool ok = true;
    std::string why;
    uint64_t lobby = 0;
    SteamId user = 0;
    std::string connect;
    std::vector<uint64_t> lobbies;
};

struct Datagram {
    SteamId from = 0;
    std::vector<uint8_t> data;
};

class Backend {
public:
    virtual ~Backend() = default;
    virtual bool init(std::string& why) = 0;             // false: `why` is a sentence for the player
    virtual SteamId me() = 0;
    virtual std::string myName() = 0;
    virtual std::string nameOf(SteamId id) = 0;
    virtual bool isFriend(SteamId id) = 0;
    virtual void pump(std::vector<BackendEvent>& out) = 0;

    virtual void createLobby(int maxMembers) = 0;         // friends only
    virtual void joinLobby(uint64_t lobby) = 0;
    virtual void leaveLobby(uint64_t lobby) = 0;
    virtual void findLobbies() = 0;                       // only lobbies carrying our mod's marker key, wherever they are
    virtual void setLobbyData(uint64_t lobby, const std::string& key, const std::string& value) = 0;
    virtual std::string lobbyData(uint64_t lobby, const std::string& key) = 0;
    virtual SteamId lobbyOwner(uint64_t lobby) = 0;
    virtual std::vector<SteamId> lobbyMembers(uint64_t lobby) = 0;
    virtual void setJoinText(const std::string& text) = 0;   // what a friend's Join Game button carries ("" = no button)
    virtual std::vector<SteamId> onlineFriends() = 0;          // friends who are online now (the ones an invite can reach)
    virtual bool inviteToLobby(uint64_t lobby, SteamId friendId) = 0;   // Steam's own lobby invite: a chat message the friend clicks

    virtual bool send(SteamId to, int channel, const uint8_t* data, uint32_t size) = 0;   // unreliable, no delay
    virtual void receive(int channel, std::vector<Datagram>& out) = 0;
    virtual void acceptSession(SteamId user) = 0;
    virtual void closeSession(SteamId user) = 0;
};

struct MockConfig {
    SteamId id = 0;
    int basePort = 47000;                // player N listens on basePort + (N % 1000)
    std::string dir;                     // shared folder that holds the lobbies
    std::set<SteamId> friends;
    std::string name;
};

std::unique_ptr<Backend> makeSteamBackend(const std::wstring& dllPath);
std::unique_ptr<Backend> makeMockBackend(const MockConfig& cfg);

}  // namespace fnvlink
