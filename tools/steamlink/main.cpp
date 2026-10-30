// fnvsteam.exe: the co-op launcher's helper for playing over Steam (docs/steam.md). It runs next to the launcher for the whole session,
// as Valve's sample app "Spacewar" (AppID 480, steam_appid.txt beside the exe), and does two jobs.
//
//   1. The party lobby over Steam. The host publishes a friends-only Steam lobby that carries a marker key, so only this mod's lobbies are
//      found; friends list the hosting friends, join, ready up and begin. The launcher talks to the helper in JSON lines and the helper
//      carries the lobby's small datagrams (ISteamNetworkingMessages, channel 0).
//   2. A UDP tunnel for the game (channel 1). The game itself never talks to Steam: it plays over its normal ENet/UDP.
//        guest:  the game connects to a local UDP port of the helper; everything it sends goes to the host's Steam ID, and back.
//        host:   for every friend in the lobby the helper opens one local UDP socket and forwards to the host game's port, so the game's
//                ENet sees each friend as a separate peer.
//
//   fnvsteam.exe [--dir <folder with steam_api.dll>]               the real thing (default: the folder of the exe)
//   fnvsteam.exe --mock-id <id> --mock-dir <folder> [--mock-base <port>] [--mock-friends id,id,...] [--mock-name <name>]
//                                                                  no Steam: players on one PC (tests)
//
// launcher -> helper, one JSON object a line (Steam IDs and lobby IDs are strings):
//   {"cmd":"friends","tag":"6"}                       -> {"ev":"friends","list":[{"id","name","mode","same","lobby"}]}
//   {"cmd":"host","tag":"6","mode":"lobby","port":27960,"max":4}
//                                                     open a friends-only lobby; the game will host ENet on 127.0.0.1:port
//                                                     -> {"ev":"hosting"} or {"ev":"hosting","error":"..."}
//   {"cmd":"mode","mode":"game"}                      the host is in the world (friends may join in progress)
//   {"cmd":"join","id":"7656...","lobby":"..."}       enter that friend's lobby and open the guest tunnel
//                                                     -> {"ev":"tunnel","port":N} (the game connects to 127.0.0.1:N) or {"ev":"link","error":"..."}
//   {"cmd":"online"}                                  -> {"ev":"online","list":[{"id","name"}]}  friends online now
//   {"cmd":"invite","id":"7656..."}                   host: Steam's lobby invite to that friend -> {"ev":"invited","id":"...","error"?:"..."}
//   {"cmd":"send","peer":N,"b64":"..."}               one lobby datagram (a guest sees its host as peer 0, a host numbers guests from 1)
//   {"cmd":"stop"}                                    leave the lobby, close every tunnel
//   {"cmd":"quit"}
// helper -> launcher:
//   {"ev":"ready","me":"7656...","name":"..."}  |  {"ev":"error","why":"..."}
//   {"ev":"peer","peer":N,"state":"up"|"down","id":"..."}      {"ev":"recv","peer":N,"b64":"..."}
//   {"ev":"join_request","id":"7656...","connect":"...","lobby":"..."}   the player clicked Join Game on a hosting friend, or a friend's
//                                                     invite arrived while the helper runs (connect "invite"; nothing clicked yet)
//   {"ev":"link","error":"..."}                                 why the last join did not work, or that the host left
#include <algorithm>
#include <atomic>
#include <chrono>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <deque>
#include <fcntl.h>
#include <io.h>
#include <iostream>
#include <map>
#include <memory>
#include <mutex>
#include <string>
#include <thread>
#include <vector>

#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#endif
#ifndef NOMINMAX
#define NOMINMAX
#endif
#include <winsock2.h>
#include <mstcpip.h>
#ifndef SIO_UDP_CONNRESET
#define SIO_UDP_CONNRESET _WSAIOW(IOC_VENDOR, 12)   // from mswsock.h
#endif
#include <windows.h>

#include "backend.h"
#include "common/minijson.h"

using nvcoop::json::Value;
using namespace fnvlink;

namespace {

namespace chr = std::chrono;
using Clock = chr::steady_clock;

constexpr int kChannelLobby = 0;     // lobby datagrams (launcher <-> launcher)
constexpr int kChannelTunnel = 1;    // the game's UDP
constexpr auto kTunnelIdle = chr::seconds(120);
constexpr auto kFriendsTimeout = chr::seconds(10);
constexpr size_t kMaxTunnels = 8;

// ---- base64 -----------------------------------------------------------------------------------------------------------------------
const char* const kAlphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

std::string encode64(const uint8_t* p, size_t n) {
    std::string o;
    o.reserve((n + 2) / 3 * 4);
    for (size_t i = 0; i < n; i += 3) {
        const size_t left = n - i;
        const uint32_t v = (static_cast<uint32_t>(p[i]) << 16) | (left > 1 ? static_cast<uint32_t>(p[i + 1]) << 8 : 0) | (left > 2 ? p[i + 2] : 0u);
        o.push_back(kAlphabet[v >> 18 & 63]);
        o.push_back(kAlphabet[v >> 12 & 63]);
        o.push_back(left > 1 ? kAlphabet[v >> 6 & 63] : '=');
        o.push_back(left > 2 ? kAlphabet[v & 63] : '=');
    }
    return o;
}

bool decode64(const std::string& s, std::vector<uint8_t>& out) {
    int val[256];
    std::fill(val, val + 256, -1);
    for (int i = 0; i < 64; ++i) val[static_cast<uint8_t>(kAlphabet[i])] = i;
    out.clear();
    uint32_t acc = 0;
    int bits = 0;
    for (char c : s) {
        if (c == '=') break;
        const int v = val[static_cast<uint8_t>(c)];
        if (v < 0) return false;
        acc = (acc << 6) | static_cast<uint32_t>(v);
        bits += 6;
        if (bits >= 8) { bits -= 8; out.push_back(static_cast<uint8_t>(acc >> bits & 0xFF)); }
    }
    return true;
}

// ---- talking to the launcher ------------------------------------------------------------------------------------------------------
std::mutex g_outMutex;

void emit(const Value& v) {
    const std::string line = v.dump() + "\n";
    std::lock_guard<std::mutex> lock(g_outMutex);
    std::fwrite(line.data(), 1, line.size(), stdout);
    std::fflush(stdout);
}

Value event(const char* name) {
    Value v = Value::object();
    v.set("ev", name);
    return v;
}

std::string str(SteamId id) { return std::to_string(id); }
uint64_t number(const std::string& s) { return std::strtoull(s.c_str(), nullptr, 10); }

// Mutation twins, switched at run time like the plugin's (FNVSTEAM_MUT=<name>; tools/launcher/run_tests.py runs the helper with each one and
// expects its checks to fail): "merge" = every friend shares one tunnel socket, "noaccept" = Steam sessions are never accepted.
bool twin(const char* name) {
    const char* m = std::getenv("FNVSTEAM_MUT");
    return m && std::strcmp(m, name) == 0;
}

// ---- the helper -------------------------------------------------------------------------------------------------------------------
class UdpEndpoint {
public:
    UdpEndpoint() = default;
    UdpEndpoint(const UdpEndpoint&) = delete;
    UdpEndpoint& operator=(const UdpEndpoint&) = delete;
    ~UdpEndpoint() { close(); }

    bool open() {
        close();
        s_ = socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP);
        if (s_ == INVALID_SOCKET) return false;
        sockaddr_in a{};
        a.sin_family = AF_INET;
        a.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
        if (bind(s_, reinterpret_cast<sockaddr*>(&a), sizeof a) != 0) { close(); return false; }
        u_long nonBlocking = 1;
        ioctlsocket(s_, FIONBIO, &nonBlocking);
        BOOL off = FALSE;
        DWORD got = 0;
        WSAIoctl(s_, SIO_UDP_CONNRESET, &off, sizeof off, nullptr, 0, &got, nullptr, nullptr);   // a gone peer must not look like a read error
        int len = sizeof a;
        getsockname(s_, reinterpret_cast<sockaddr*>(&a), &len);
        port_ = ntohs(a.sin_port);
        return true;
    }
    void close() {
        if (s_ != INVALID_SOCKET) closesocket(s_);
        s_ = INVALID_SOCKET;
        port_ = 0;
    }
    bool isOpen() const { return s_ != INVALID_SOCKET; }
    SOCKET handle() const { return s_; }
    uint16_t port() const { return port_; }
    int receive(char* buf, int cap, sockaddr_in& from) {
        int len = sizeof from;
        return recvfrom(s_, buf, cap, 0, reinterpret_cast<sockaddr*>(&from), &len);
    }
    void sendTo(const void* data, int size, const sockaddr_in& to) {
        sendto(s_, static_cast<const char*>(data), size, 0, reinterpret_cast<const sockaddr*>(&to), sizeof to);
    }

private:
    SOCKET s_ = INVALID_SOCKET;
    uint16_t port_ = 0;
};

struct FriendTunnel {
    UdpEndpoint sock;
    Clock::time_point lastActive;
};

class Helper {
public:
    explicit Helper(std::unique_ptr<Backend> b) : b_(std::move(b)) {}

    bool start() {
        std::string why;
        if (!b_->init(why)) {
            Value v = event("error");
            v.set("why", why);
            emit(v);
            return false;
        }
        Value v = event("ready");
        v.set("me", str(b_->me())).set("name", b_->myName());
        emit(v);
        return true;
    }

    void run() {
        std::thread reader([this] {
            std::string line;
            while (std::getline(std::cin, line)) {
                std::lock_guard<std::mutex> lock(inMutex_);
                lines_.push_back(line);
            }
            stdinClosed_ = true;
        });
        reader.detach();
        while (!quit_) {
            drainCommands();
            if (stdinClosed_ && lines_.empty()) break;
            events_.clear();
            b_->pump(events_);
            for (const BackendEvent& e : events_) onBackendEvent(e);
            onLobbyDatagrams();
            onTunnelDatagrams();
            pollLocalSockets();
            housekeeping();
        }
        leaveEverything();
    }

private:
    // ---- commands ----
    void drainCommands() {
        std::deque<std::string> take;
        {
            std::lock_guard<std::mutex> lock(inMutex_);
            take.swap(lines_);
        }
        for (const std::string& line : take) {
            Value c;
            std::string err;
            if (!nvcoop::json::parse(line, c, err) || !c.isObject()) continue;
            onCommand(c);
        }
    }

    void onCommand(const Value& c) {
        const std::string cmd = c.get("cmd").asString("");
        if (cmd == "friends") {
            friendsTag_ = c.get("tag").asString("");
            friendsAsked_ = Clock::now();
            wantFriends_ = true;
            b_->findLobbies();
        } else if (cmd == "host") {
            openLobby(c);
        } else if (cmd == "mode") {
            hostMode_ = c.get("mode").asString("");
            if (hostLobby_) b_->setLobbyData(hostLobby_, "mode", hostMode_);
        } else if (cmd == "join") {
            joinFriend(number(c.get("id").asString("")), number(c.get("lobby").asString("")));
        } else if (cmd == "online") {
            Value list = Value::array();
            for (SteamId id : b_->onlineFriends()) {
                std::string name = b_->nameOf(id);
                if (name.empty()) name = str(id);
                Value r = Value::object();
                r.set("id", str(id)).set("name", name);
                list.push(std::move(r));
            }
            Value v = event("online");
            v.set("list", std::move(list));
            emit(v);
        } else if (cmd == "invite") {
            // only the host invites, only into its own lobby, only a friend (Steam refuses anybody else anyway)
            const SteamId to = number(c.get("id").asString(""));
            Value v = event("invited");
            v.set("id", str(to));
            if (!hostLobby_) v.set("error", "Open the lobby first: HOST A GAME with the connection set to Steam.");
            else if (!to || !b_->isFriend(to)) v.set("error", "Only Steam friends can be invited.");
            else if (!b_->inviteToLobby(hostLobby_, to)) v.set("error", "Steam did not send the invite.");
            emit(v);
        } else if (cmd == "send") {
            sendLobbyDatagram(static_cast<int>(c.get("peer").asInt(-1)), c.get("b64").asString(""));
        } else if (cmd == "stop") {
            leaveEverything();
        } else if (cmd == "quit") {
            quit_ = true;
        }
    }

    void openLobby(const Value& c) {
        leaveEverything();
        hostTag_ = c.get("tag").asString("");
        hostMode_ = c.get("mode").asString("lobby");
        gamePort_ = static_cast<uint16_t>(c.get("port").asInt(0));
        const int maxPlayers = static_cast<int>(std::max<int64_t>(2, std::min<int64_t>(8, c.get("max").asInt(4))));
        creating_ = true;
        b_->createLobby(maxPlayers);
    }

    void joinFriend(SteamId host, uint64_t lobby) {
        leaveEverything();
        if (!host) { linkError("That is not a Steam ID."); return; }
        if (lobby == 0) {
            auto it = lastRows_.find(host);
            if (it != lastRows_.end()) lobby = it->second;
        }
        wantedHost_ = host;
        if (lobby == 0) {   // not seen yet: look for it first
            finding_ = true;
            b_->findLobbies();
        } else {
            joining_ = lobby;
            b_->joinLobby(lobby);
        }
    }

    void sendLobbyDatagram(int peer, const std::string& b64) {
        std::vector<uint8_t> data;
        if (!decode64(b64, data)) return;
        SteamId to = 0;
        if (guestUp_ && peer == 0) {
            to = guestHost_;
        } else if (hostLobby_) {
            auto it = idOfPeer_.find(peer);
            if (it != idOfPeer_.end()) to = it->second;
        }
        if (to) b_->send(to, kChannelLobby, data.data(), static_cast<uint32_t>(data.size()));
    }

    // ---- what Steam tells us ----
    void onBackendEvent(const BackendEvent& e) {
        switch (e.type) {
            case BackendEvent::LobbyCreated: onLobbyCreated(e); break;
            case BackendEvent::LobbyEntered: onLobbyEntered(e); break;
            case BackendEvent::LobbyList: onLobbyList(e); break;
            case BackendEvent::MemberLeft: onMemberLeft(e); break;
            case BackendEvent::SessionRequest:
                // friends only (the lobby is friends only too); what a friend may do is decided when its data arrives (mayTalk)
                if (!twin("noaccept") && (b_->isFriend(e.user) || (guestUp_ && e.user == guestHost_))) b_->acceptSession(e.user);
                break;
            case BackendEvent::SessionFailed: break;
            case BackendEvent::JoinRequest: {
                Value v = event("join_request");
                v.set("id", str(e.user)).set("connect", e.connect).set("lobby", str(e.lobby));
                emit(v);
                break;
            }
        }
    }

    void onLobbyCreated(const BackendEvent& e) {
        if (!creating_) { if (e.ok) b_->leaveLobby(e.lobby); return; }
        creating_ = false;
        Value v = event("hosting");
        if (!e.ok) {
            v.set("error", e.why);
            emit(v);
            return;
        }
        hostLobby_ = e.lobby;
        b_->setLobbyData(e.lobby, "fnvcoop", "1");
        b_->setLobbyData(e.lobby, "tag", hostTag_);
        b_->setLobbyData(e.lobby, "mode", hostMode_);
        b_->setLobbyData(e.lobby, "owner", str(b_->me()));
        b_->setLobbyData(e.lobby, "name", b_->myName());
        b_->setJoinText("+connect_lobby " + str(e.lobby));
        v.set("lobby", str(e.lobby));
        emit(v);
    }

    void onLobbyEntered(const BackendEvent& e) {
        if (e.lobby == hostLobby_ || !joining_ || e.lobby != joining_) return;
        joining_ = 0;
        if (!e.ok) { linkError(e.why); wantedHost_ = 0; return; }
        SteamId owner = b_->lobbyOwner(e.lobby);
        if (!owner) owner = number(b_->lobbyData(e.lobby, "owner"));
        if (owner != wantedHost_) {   // never tunnel to anybody but the friend the player picked
            b_->leaveLobby(e.lobby);
            linkError("That lobby does not belong to the friend you picked.");
            wantedHost_ = 0;
            return;
        }
        if (!guestSock_.open()) {
            b_->leaveLobby(e.lobby);
            linkError("The helper could not open its local port for the game.");
            wantedHost_ = 0;
            return;
        }
        guestHost_ = owner;
        guestLobby_ = e.lobby;
        guestUp_ = true;
        gameKnown_ = false;
        Value t = event("tunnel");
        t.set("port", static_cast<int>(guestSock_.port()));
        emit(t);
        emitPeer(0, guestHost_, true);
    }

    void onLobbyList(const BackendEvent& e) {
        std::vector<Value> rows;
        lastRows_.clear();
        for (uint64_t lobby : e.lobbies) {
            if (b_->lobbyData(lobby, "fnvcoop") != "1") continue;
            SteamId owner = number(b_->lobbyData(lobby, "owner"));
            if (!owner) owner = b_->lobbyOwner(lobby);
            if (!owner || owner == b_->me() || !b_->isFriend(owner)) continue;
            lastRows_[owner] = lobby;
            std::string name = b_->nameOf(owner);
            if (name.empty()) name = b_->lobbyData(lobby, "name");
            if (name.empty()) name = str(owner);
            const std::string tag = b_->lobbyData(lobby, "tag");
            Value r = Value::object();
            r.set("id", str(owner)).set("name", name).set("mode", b_->lobbyData(lobby, "mode")).set("lobby", str(lobby))
                .set("same", friendsTag_.empty() || friendsTag_ == tag);
            rows.push_back(std::move(r));
        }
        if (wantFriends_) {
            wantFriends_ = false;
            Value list = Value::array();
            for (auto& r : rows) list.push(std::move(r));
            Value v = event("friends");
            v.set("list", std::move(list));
            emit(v);
        }
        if (finding_) {
            finding_ = false;
            auto it = lastRows_.find(wantedHost_);
            if (it == lastRows_.end()) {
                linkError("Your friend's lobby was not found. Is the host still hosting, and are you Steam friends?");
                wantedHost_ = 0;
            } else {
                joining_ = it->second;
                b_->joinLobby(it->second);
            }
        }
    }

    void onMemberLeft(const BackendEvent& e) {
        if (e.lobby == hostLobby_ && e.user != b_->me()) {
            dropFriend(e.user);
        } else if (e.lobby == guestLobby_ && e.user == guestHost_ && guestUp_) {
            guestUp_ = false;
            guestSock_.close();
            b_->leaveLobby(guestLobby_);
            guestLobby_ = 0;
            emitPeer(0, guestHost_, false);
            linkError("The host left the party.");
        }
    }

    SteamId tunnelKey(SteamId from) const { return twin("merge") ? 1 : from; }

    // ---- who may talk to us ----
    bool isMember(SteamId id) {
        if (!hostLobby_) return false;
        for (SteamId m : b_->lobbyMembers(hostLobby_)) if (m == id) return true;
        return false;
    }
    bool mayTalk(SteamId id) {
        if (guestUp_ && id == guestHost_) return true;
        return hostLobby_ != 0 && b_->isFriend(id) && isMember(id);
    }

    // ---- the lobby's datagrams ----
    void emitPeer(int peer, SteamId id, bool up) {
        Value v = event("peer");
        v.set("peer", peer).set("state", up ? "up" : "down").set("id", str(id));
        emit(v);
    }

    void onLobbyDatagrams() {
        std::vector<Datagram> got;
        b_->receive(kChannelLobby, got);
        for (Datagram& d : got) {
            int peer = -1;
            if (guestUp_ && d.from == guestHost_) {
                peer = 0;
            } else if (hostLobby_) {
                auto it = peerOfId_.find(d.from);
                if (it != peerOfId_.end()) {
                    peer = it->second;
                } else if (mayTalk(d.from)) {
                    peer = 1;
                    while (idOfPeer_.count(peer)) ++peer;
                    peerOfId_[d.from] = peer;
                    idOfPeer_[peer] = d.from;
                    emitPeer(peer, d.from, true);
                }
            }
            if (peer < 0) continue;
            Value v = event("recv");
            v.set("peer", peer).set("b64", encode64(d.data.data(), d.data.size()));
            emit(v);
        }
    }

    // ---- the tunnel ----
    void onTunnelDatagrams() {
        std::vector<Datagram> got;
        b_->receive(kChannelTunnel, got);
        for (Datagram& d : got) {
            if (d.data.empty()) continue;
            if (guestUp_ && d.from == guestHost_) {
                if (gameKnown_) guestSock_.sendTo(d.data.data(), static_cast<int>(d.data.size()), gameAddr_);
                continue;
            }
            if (!hostLobby_ || !gamePort_) continue;
            auto it = tunnels_.find(tunnelKey(d.from));
            if (it == tunnels_.end()) {
                if (tunnels_.size() >= kMaxTunnels || !mayTalk(d.from)) continue;
                auto t = std::make_unique<FriendTunnel>();
                if (!t->sock.open()) continue;
                it = tunnels_.emplace(tunnelKey(d.from), std::move(t)).first;
            }
            it->second->lastActive = Clock::now();
            sockaddr_in game{};
            game.sin_family = AF_INET;
            game.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
            game.sin_port = htons(gamePort_);
            it->second->sock.sendTo(d.data.data(), static_cast<int>(d.data.size()), game);
        }
    }

    void pollLocalSockets() {
        std::vector<WSAPOLLFD> fds;
        std::vector<SteamId> owners;   // 0 = the guest socket
        if (guestUp_) { fds.push_back({guestSock_.handle(), POLLRDNORM, 0}); owners.push_back(0); }
        for (auto& t : tunnels_) { fds.push_back({t.second->sock.handle(), POLLRDNORM, 0}); owners.push_back(t.first); }
        if (fds.empty()) { Sleep(2); return; }
        if (WSAPoll(fds.data(), static_cast<ULONG>(fds.size()), 2) <= 0) return;
        char buf[4096];
        for (size_t i = 0; i < fds.size(); ++i) {
            if (!(fds[i].revents & POLLRDNORM)) continue;
            UdpEndpoint& ep = owners[i] == 0 ? guestSock_ : tunnels_[owners[i]]->sock;
            for (int n = 0; n < 64; ++n) {
                sockaddr_in from{};
                const int got = ep.receive(buf, sizeof buf, from);
                if (got < 0) break;
                if (owners[i] == 0) {
                    gameAddr_ = from;
                    gameKnown_ = true;
                    b_->send(guestHost_, kChannelTunnel, reinterpret_cast<const uint8_t*>(buf), static_cast<uint32_t>(got));
                } else {
                    tunnels_[owners[i]]->lastActive = Clock::now();
                    b_->send(owners[i], kChannelTunnel, reinterpret_cast<const uint8_t*>(buf), static_cast<uint32_t>(got));
                }
            }
        }
    }

    void housekeeping() {
        const auto now = Clock::now();
        if (now - lastSweep_ < chr::seconds(1)) return;
        lastSweep_ = now;
        for (auto it = tunnels_.begin(); it != tunnels_.end();) {
            if (now - it->second->lastActive > kTunnelIdle) it = tunnels_.erase(it);
            else ++it;
        }
        if (wantFriends_ && now - friendsAsked_ > kFriendsTimeout) {
            wantFriends_ = false;
            Value v = event("friends");
            v.set("list", Value::array());
            emit(v);
        }
    }

    // ---- leaving ----
    void dropFriend(SteamId id) {
        tunnels_.erase(id);
        auto it = peerOfId_.find(id);
        if (it != peerOfId_.end()) {
            idOfPeer_.erase(it->second);
            emitPeer(it->second, id, false);
            peerOfId_.erase(it);
        }
        b_->closeSession(id);
    }

    void leaveEverything() {
        if (hostLobby_) {
            std::vector<SteamId> ids;
            for (auto& p : peerOfId_) ids.push_back(p.first);
            for (auto& t : tunnels_) ids.push_back(t.first);
            b_->leaveLobby(hostLobby_);
            for (SteamId id : ids) b_->closeSession(id);
        }
        if (guestLobby_) b_->leaveLobby(guestLobby_);
        if (guestUp_) b_->closeSession(guestHost_);
        if (joining_) b_->leaveLobby(joining_);
        b_->setJoinText("");
        hostLobby_ = guestLobby_ = joining_ = 0;
        wantedHost_ = guestHost_ = 0;
        creating_ = finding_ = guestUp_ = gameKnown_ = false;
        guestSock_.close();
        tunnels_.clear();
        peerOfId_.clear();
        idOfPeer_.clear();
    }

    void linkError(const std::string& why) {
        Value v = event("link");
        v.set("error", why);
        emit(v);
    }

    std::unique_ptr<Backend> b_;
    std::vector<BackendEvent> events_;
    std::mutex inMutex_;
    std::deque<std::string> lines_;
    std::atomic<bool> stdinClosed_{false};
    bool quit_ = false;
    Clock::time_point lastSweep_{};

    // friends list
    bool wantFriends_ = false;
    std::string friendsTag_;
    Clock::time_point friendsAsked_{};
    std::map<SteamId, uint64_t> lastRows_;   // hosting friend -> their lobby, from the last search

    // host side
    uint64_t hostLobby_ = 0;
    bool creating_ = false;
    std::string hostTag_, hostMode_;
    uint16_t gamePort_ = 0;
    std::map<SteamId, int> peerOfId_;
    std::map<int, SteamId> idOfPeer_;
    std::map<SteamId, std::unique_ptr<FriendTunnel>> tunnels_;

    // guest side
    SteamId wantedHost_ = 0;
    bool finding_ = false;
    uint64_t joining_ = 0;
    uint64_t guestLobby_ = 0;
    SteamId guestHost_ = 0;
    bool guestUp_ = false;
    UdpEndpoint guestSock_;
    sockaddr_in gameAddr_{};
    bool gameKnown_ = false;
};

// ---- start-up -----------------------------------------------------------------------------------------------------------------------
std::wstring exeFolder() {
    wchar_t path[MAX_PATH * 2] = L"";
    GetModuleFileNameW(nullptr, path, static_cast<DWORD>(sizeof path / sizeof path[0]));
    std::wstring p = path;
    const size_t cut = p.find_last_of(L"\\/");
    return cut == std::wstring::npos ? L"." : p.substr(0, cut);
}

std::wstring widen(const std::string& s) {
    if (s.empty()) return L"";
    const int n = MultiByteToWideChar(CP_UTF8, 0, s.c_str(), -1, nullptr, 0);
    std::wstring w(static_cast<size_t>(n), L'\0');
    MultiByteToWideChar(CP_UTF8, 0, s.c_str(), -1, &w[0], n);
    w.resize(static_cast<size_t>(n) - 1);
    return w;
}

// steam_api.dll reads the AppID from steam_appid.txt in the working directory: 480, Valve's sample app that every account may run.
void writeAppId(const std::wstring& dir) {
    const std::wstring file = dir + L"\\steam_appid.txt";
    FILE* f = _wfopen(file.c_str(), L"rb");
    char cur[16] = "";
    if (f) { const size_t n = std::fread(cur, 1, sizeof cur - 1, f); cur[n] = 0; std::fclose(f); }
    if (std::strncmp(cur, "480", 3) == 0) return;
    f = _wfopen(file.c_str(), L"wb");
    if (f) { std::fputs("480", f); std::fclose(f); }
}

}  // namespace

int main(int argc, char** argv) {
    _setmode(_fileno(stdout), _O_BINARY);
    WSADATA wsa;
    WSAStartup(MAKEWORD(2, 2), &wsa);

    std::wstring dir = exeFolder();
    MockConfig mock;
    bool useMock = false;
    for (int i = 1; i < argc; ++i) {
        const std::string a = argv[i];
        const char* next = i + 1 < argc ? argv[i + 1] : "";
        if (a == "--dir") { dir = widen(next); ++i; }
        else if (a == "--mock-id") { mock.id = number(next); useMock = true; ++i; }
        else if (a == "--mock-base") { mock.basePort = std::atoi(next); ++i; }
        else if (a == "--mock-dir") { mock.dir = next; ++i; }
        else if (a == "--mock-name") { mock.name = next; ++i; }
        else if (a == "--mock-friends") {
            std::string list = next;
            size_t pos = 0;
            while (pos < list.size()) {
                size_t comma = list.find(',', pos);
                if (comma == std::string::npos) comma = list.size();
                if (comma > pos) mock.friends.insert(number(list.substr(pos, comma - pos)));
                pos = comma + 1;
            }
            ++i;
        }
    }

    std::unique_ptr<Backend> backend;
    if (useMock) {
        backend = makeMockBackend(mock);
    } else {
        SetCurrentDirectoryW(dir.c_str());
        writeAppId(dir);
        backend = makeSteamBackend(dir + L"\\steam_api.dll");
    }
    Helper helper(std::move(backend));
    if (!helper.start()) return 2;
    helper.run();
    return 0;
}
