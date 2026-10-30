// A pretend Steam for the tests (docs/steam.md): several helpers on one PC. Lobbies are small text files in a shared folder, messages are
// localhost UDP datagrams, "friends" is a list given on the command line. It keeps the Steam rules the real helper depends on:
// a lobby is only seen by the owner's friends, a lobby has a member limit, and the first message from a user nobody accepted yet is held
// until acceptSession() (a stranger who is never accepted never gets through).
#include "backend.h"

#include <algorithm>
#include <chrono>
#include <cstdio>
#include <cstring>
#include <deque>
#include <fstream>
#include <map>
#include <sstream>

#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#endif
#ifndef NOMINMAX
#define NOMINMAX
#endif
#include <winsock2.h>
#include <windows.h>

namespace fnvlink {
namespace {

namespace chr = std::chrono;

struct LobbyFile {
    std::map<std::string, std::string> kv;
    bool exists = false;
};

class MockBackend final : public Backend {
public:
    explicit MockBackend(MockConfig c) : c_(std::move(c)) {}

    ~MockBackend() override {
        for (uint64_t l : joined_) removeMember(l, c_.id);
        if (sock_ != INVALID_SOCKET) closesocket(sock_);
    }

    bool init(std::string& why) override {
        WSADATA w;
        if (WSAStartup(MAKEWORD(2, 2), &w) != 0) { why = "Winsock did not start."; return false; }
        sock_ = socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP);
        sockaddr_in a = addr(c_.id);
        if (bind(sock_, reinterpret_cast<sockaddr*>(&a), sizeof a) != 0) { why = "The pretend Steam port is taken."; return false; }
        u_long nb = 1;
        ioctlsocket(sock_, FIONBIO, &nb);
        if (c_.name.empty()) c_.name = "Player" + std::to_string(c_.id % 1000);
        write(path("N" + std::to_string(c_.id)), c_.name + "\n");
        std::string list;
        for (SteamId f : c_.friends) list += std::to_string(f) + "\n";
        write(path("F" + std::to_string(c_.id)), list);
        return true;
    }

    SteamId me() override { return c_.id; }
    std::string myName() override { return c_.name; }
    std::string nameOf(SteamId id) override {
        std::ifstream f(path("N" + std::to_string(id)));
        std::string n;
        std::getline(f, n);
        return n;
    }
    // Steam friendship is mutual: both sides have to list each other (a stranger who lists the host as a friend is still a stranger)
    bool isFriend(SteamId id) override {
        if (!c_.friends.count(id)) return false;
        std::ifstream f(path("F" + std::to_string(id)));
        if (!f) return true;
        std::string line;
        while (std::getline(f, line)) if (std::strtoull(line.c_str(), nullptr, 10) == c_.id) return true;
        return false;
    }

    void pump(std::vector<BackendEvent>& out) override {
        for (auto& e : queued_) out.push_back(std::move(e));
        queued_.clear();
        const auto now = chr::steady_clock::now();
        if (now - lastScan_ > chr::milliseconds(20)) {
            lastScan_ = now;
            scanLobbies(out);
            scanInvites(out);
        }
        char buf[70000];
        for (;;) {
            sockaddr_in from{};
            int fl = sizeof from;
            const int n = recvfrom(sock_, buf, sizeof buf, 0, reinterpret_cast<sockaddr*>(&from), &fl);
            if (n < 9) break;
            Datagram d;
            std::memcpy(&d.from, buf, 8);
            const int ch = static_cast<uint8_t>(buf[8]);
            d.data.assign(buf + 9, buf + n);
            if (accepted_.count(d.from)) {
                inbox_[ch].push_back(std::move(d));
            } else {
                if (!asked_.count(d.from)) {
                    asked_.insert(d.from);
                    BackendEvent e;
                    e.type = BackendEvent::SessionRequest;
                    e.user = d.from;
                    out.push_back(e);
                }
                held_[d.from].push_back({ch, std::move(d)});
            }
        }
    }

    void createLobby(int maxMembers) override {
        const uint64_t lobby = 109775240000000000ull + (c_.id % 100000) * 1000 + (++counter_);
        LobbyFile lf;
        lf.kv["owner"] = std::to_string(c_.id);
        lf.kv["max"] = std::to_string(maxMembers);
        store(lobby, lf);
        addMember(lobby, c_.id);
        joined_.insert(lobby);
        members_[lobby] = {c_.id};
        BackendEvent e;
        e.type = BackendEvent::LobbyCreated;
        e.lobby = lobby;
        queued_.push_back(e);
    }

    void joinLobby(uint64_t lobby) override {
        BackendEvent e;
        e.type = BackendEvent::LobbyEntered;
        e.lobby = lobby;
        LobbyFile lf = load(lobby);
        const SteamId owner = lf.exists ? std::strtoull(lf.kv["owner"].c_str(), nullptr, 10) : 0;
        if (!lf.exists) { e.ok = false; e.why = "The host's lobby is closed, or it is not your friend's."; }
        else if (!isFriend(owner)) { e.ok = false; e.why = "The host's lobby is closed, or it is not your friend's."; }
        else if (static_cast<int>(listMembers(lobby).size()) >= std::atoi(lf.kv["max"].c_str())) { e.ok = false; e.why = "The party is full."; }
        else {
            addMember(lobby, c_.id);
            joined_.insert(lobby);
            members_[lobby] = listMembers(lobby);
        }
        queued_.push_back(e);
    }

    void leaveLobby(uint64_t lobby) override {
        removeMember(lobby, c_.id);
        joined_.erase(lobby);
        members_.erase(lobby);
        if (load(lobby).exists && std::strtoull(load(lobby).kv["owner"].c_str(), nullptr, 10) == c_.id) std::remove(path("L" + std::to_string(lobby)).c_str());
    }

    void findLobbies() override {
        BackendEvent e;
        e.type = BackendEvent::LobbyList;
        WIN32_FIND_DATAA fd;
        HANDLE h = FindFirstFileA((c_.dir + "\\L*").c_str(), &fd);
        if (h != INVALID_HANDLE_VALUE) {
            do {
                if (std::strchr(fd.cFileName, '.')) continue;   // a half-written copy
                const uint64_t lobby = std::strtoull(fd.cFileName + 1, nullptr, 10);
                LobbyFile lf = load(lobby);
                if (!lf.exists || lf.kv["fnvcoop"] != "1") continue;
                if (!isFriend(std::strtoull(lf.kv["owner"].c_str(), nullptr, 10))) continue;   // Steam shows friends-only lobbies to friends only
                e.lobbies.push_back(lobby);
            } while (FindNextFileA(h, &fd));
            FindClose(h);
        }
        queued_.push_back(e);
    }

    void setLobbyData(uint64_t lobby, const std::string& k, const std::string& v) override {
        LobbyFile lf = load(lobby);
        if (!lf.exists) return;
        lf.kv[k] = v;
        store(lobby, lf);
    }
    std::string lobbyData(uint64_t lobby, const std::string& k) override {
        LobbyFile lf = load(lobby);
        auto it = lf.kv.find(k);
        return it == lf.kv.end() ? std::string() : it->second;
    }
    SteamId lobbyOwner(uint64_t lobby) override { return std::strtoull(load(lobby).kv["owner"].c_str(), nullptr, 10); }
    std::vector<SteamId> lobbyMembers(uint64_t lobby) override { return listMembers(lobby); }
    void setJoinText(const std::string&) override {}
    std::vector<SteamId> onlineFriends() override {   // every mutual friend who has started a pretend Steam (its name file is there)
        std::vector<SteamId> r;
        for (SteamId f : c_.friends) {
            if (isFriend(f) && GetFileAttributesA(path("N" + std::to_string(f)).c_str()) != INVALID_FILE_ATTRIBUTES) r.push_back(f);
        }
        return r;
    }
    bool inviteToLobby(uint64_t lobby, SteamId friendId) override {   // like Steam: only a friend, only into a lobby that exists
        if (!isFriend(friendId) || !load(lobby).exists) return false;
        write(path("I" + std::to_string(friendId) + "_" + std::to_string(c_.id) + "_" + std::to_string(++counter_)),
              std::to_string(c_.id) + " " + std::to_string(lobby) + "\n");
        return true;
    }

    bool send(SteamId to, int channel, const uint8_t* data, uint32_t size) override {
        std::vector<char> b(9 + size);
        std::memcpy(b.data(), &c_.id, 8);
        b[8] = static_cast<char>(channel);
        if (size) std::memcpy(b.data() + 9, data, size);
        sockaddr_in a = addr(to);
        return sendto(sock_, b.data(), static_cast<int>(b.size()), 0, reinterpret_cast<sockaddr*>(&a), sizeof a) >= 0;
    }
    void receive(int channel, std::vector<Datagram>& out) override {
        auto& q = inbox_[channel];
        while (!q.empty()) { out.push_back(std::move(q.front())); q.pop_front(); }
    }
    void acceptSession(SteamId user) override {
        accepted_.insert(user);
        auto it = held_.find(user);
        if (it == held_.end()) return;
        for (auto& h : it->second) inbox_[h.first].push_back(std::move(h.second));
        held_.erase(it);
    }
    void closeSession(SteamId user) override { accepted_.erase(user); asked_.erase(user); }

private:
    std::string path(const std::string& f) const { return c_.dir + "\\" + f; }
    sockaddr_in addr(SteamId id) const {
        sockaddr_in a{};
        a.sin_family = AF_INET;
        a.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
        a.sin_port = htons(static_cast<u_short>(c_.basePort + static_cast<int>(id % 1000)));
        return a;
    }
    // Another helper may have the file open right now (Windows refuses to replace an open file): try again for a moment
    static void write(const std::string& p, const std::string& text) {
        const std::string tmp = p + "." + std::to_string(GetCurrentProcessId()) + ".tmp";
        { std::ofstream f(tmp, std::ios::binary | std::ios::trunc); f << text; }
        for (int i = 0; i < 100; ++i) {
            if (MoveFileExA(tmp.c_str(), p.c_str(), MOVEFILE_REPLACE_EXISTING)) return;
            Sleep(2);
        }
        std::remove(tmp.c_str());
    }
    void store(uint64_t lobby, const LobbyFile& lf) {
        std::string t;
        for (auto& kv : lf.kv) t += kv.first + "=" + kv.second + "\n";
        write(path("L" + std::to_string(lobby)), t);
    }
    LobbyFile load(uint64_t lobby) const {
        LobbyFile lf;
        std::ifstream f(path("L" + std::to_string(lobby)));
        for (int i = 0; !f && i < 4; ++i) {   // being replaced by its owner at this very moment, or really gone
            Sleep(3);
            f.clear();
            f.open(path("L" + std::to_string(lobby)));
        }
        if (!f) return lf;
        lf.exists = true;
        std::string line;
        while (std::getline(f, line)) {
            const size_t eq = line.find('=');
            if (eq != std::string::npos) lf.kv[line.substr(0, eq)] = line.substr(eq + 1);
        }
        return lf;
    }
    void addMember(uint64_t lobby, SteamId id) { write(path("M" + std::to_string(lobby) + "_" + std::to_string(id)), "x"); }
    void removeMember(uint64_t lobby, SteamId id) { std::remove(path("M" + std::to_string(lobby) + "_" + std::to_string(id)).c_str()); }
    std::vector<SteamId> listMembers(uint64_t lobby) const {
        std::vector<SteamId> r;
        const std::string prefix = "M" + std::to_string(lobby) + "_";
        WIN32_FIND_DATAA fd;
        HANDLE h = FindFirstFileA((c_.dir + "\\" + prefix + "*").c_str(), &fd);
        if (h == INVALID_HANDLE_VALUE) return r;
        do {
            const std::string n = fd.cFileName;
            if (n.size() > prefix.size() && n.find('.') == std::string::npos) r.push_back(std::strtoull(n.c_str() + prefix.size(), nullptr, 10));
        } while (FindNextFileA(h, &fd));
        FindClose(h);
        std::sort(r.begin(), r.end());
        return r;
    }

    // someone left a lobby we are in: tell the helper, like Steam's chat-update callback
    void scanLobbies(std::vector<BackendEvent>& out) {
        for (auto& kv : members_) {
            const uint64_t lobby = kv.first;
            const bool gone = !load(lobby).exists;
            const std::vector<SteamId> now = gone ? std::vector<SteamId>{} : listMembers(lobby);
            for (SteamId was : kv.second) {
                if (was == c_.id) continue;
                if (std::find(now.begin(), now.end(), was) == now.end()) {
                    BackendEvent e;
                    e.type = BackendEvent::MemberLeft;
                    e.lobby = lobby;
                    e.user = was;
                    out.push_back(e);
                }
            }
            kv.second = now;
            if (gone) kv.second.clear();
        }
    }

    // invites addressed to us: one file each, taken once (Steam's LobbyInvite callback)
    void scanInvites(std::vector<BackendEvent>& out) {
        const std::string prefix = "I" + std::to_string(c_.id) + "_";
        WIN32_FIND_DATAA fd;
        HANDLE h = FindFirstFileA((c_.dir + "\\" + prefix + "*").c_str(), &fd);
        if (h == INVALID_HANDLE_VALUE) return;
        std::vector<std::string> names;
        do {
            if (!std::strchr(fd.cFileName, '.')) names.push_back(fd.cFileName);
        } while (FindNextFileA(h, &fd));
        FindClose(h);
        for (const std::string& n : names) {
            std::ifstream f(path(n));
            uint64_t inviter = 0, lobby = 0;
            f >> inviter >> lobby;
            f.close();
            std::remove(path(n).c_str());
            if (!inviter || !lobby || !isFriend(inviter)) continue;
            BackendEvent e;
            e.type = BackendEvent::JoinRequest;
            e.user = inviter;
            e.lobby = lobby;
            e.connect = "invite";
            out.push_back(e);
        }
    }

    MockConfig c_;
    SOCKET sock_ = INVALID_SOCKET;
    uint64_t counter_ = 0;
    std::set<uint64_t> joined_;
    std::map<uint64_t, std::vector<SteamId>> members_;
    std::vector<BackendEvent> queued_;
    chr::steady_clock::time_point lastScan_{};
    std::set<SteamId> accepted_, asked_;
    std::map<SteamId, std::vector<std::pair<int, Datagram>>> held_;
    std::map<int, std::deque<Datagram>> inbox_;
};

}  // namespace

std::unique_ptr<Backend> makeMockBackend(const MockConfig& cfg) { return std::make_unique<MockBackend>(cfg); }

}  // namespace fnvlink
