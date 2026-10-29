// fnvsteam.exe: the launcher's hands on Steam (docs/steam.md). The launcher is Python and the party lobby (ready-up, start, Continue)
// runs before anyone's game does, so something small has to talk to Steam on its behalf: this. It loads the game's own steam_api.dll
// (the player's Steam client does the rest, under the game's AppID), lists the Steam friends who are hosting, hosts / joins with the same
// fnvcoop::SteamTransport the game itself uses, and passes datagrams to and from the launcher as JSON lines on stdin / stdout. The
// launcher's lobby code sees a socket that happens to go through Steam.
//
//   fnvsteam.exe --game-dir <folder with steam_api.dll>            the real thing
//   fnvsteam.exe --mock-id <id> --mock-base <port> [--mock-hosts id:name,...] [--mock-strangers id,...]
//                                                                   no Steam: players on one PC talk over localhost UDP (tests)
//
// launcher -> helper, one JSON object a line:
//   {"cmd":"friends"}                          -> {"ev":"friends","list":[{"id","name","mode","same"}]}
//   {"cmd":"host","tag":"6","mode":""}         start hosting (friends see "joinable"); -> {"ev":"hosting"}
//   {"cmd":"join","id":"7656..."}              connect to a host
//   {"cmd":"send","peer":N,"b64":"..."}        one datagram to a peer (a guest sees its host as peer 0, a host numbers guests from 1)
//   {"cmd":"stop"}                             leave: tells the peers, clears the presence
//   {"cmd":"quit"}
// helper -> launcher:
//   {"ev":"ready","me":"7656...","name":"..."} | {"ev":"error","why":"..."}
//   {"ev":"peer","peer":N,"state":"up"|"down","id":"..."}   {"ev":"recv","peer":N,"b64":"..."}
//   {"ev":"join_request","id":"7656...","connect":"..."}     the player clicked Join Game on a hosting friend
//   {"ev":"link","error":"..."}                              why the last connect did not work
#include <atomic>
#include <chrono>
#include <condition_variable>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <deque>
#include <iostream>
#include <map>
#include <memory>
#include <mutex>
#include <set>
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
#include <windows.h>

#include "common/minijson.h"
#include "fnvcoop/steam_transport.h"
#include "steam_binding.h"

using nvcoop::json::Value;
using namespace fnvcoop;

namespace {

// ---- base64 ----------------------------------------------------------------------------------------------------------------
const char kB64[] = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

std::string b64enc(const Bytes& d) {
    std::string o;
    o.reserve((d.size() + 2) / 3 * 4);
    for (size_t i = 0; i < d.size(); i += 3) {
        uint32_t v = d[i] << 16;
        if (i + 1 < d.size()) v |= d[i + 1] << 8;
        if (i + 2 < d.size()) v |= d[i + 2];
        o += kB64[(v >> 18) & 63];
        o += kB64[(v >> 12) & 63];
        o += i + 1 < d.size() ? kB64[(v >> 6) & 63] : '=';
        o += i + 2 < d.size() ? kB64[v & 63] : '=';
    }
    return o;
}

bool b64dec(const std::string& s, Bytes& out) {
    out.clear();
    uint32_t acc = 0;
    int bits = 0;
    for (char c : s) {
        if (c == '=') break;
        const char* p = std::strchr(kB64, c);
        if (!p || !c) return false;
        acc = (acc << 6) | static_cast<uint32_t>(p - kB64);
        bits += 6;
        if (bits >= 8) { bits -= 8; out.push_back(static_cast<uint8_t>(acc >> bits)); }
    }
    return true;
}

// ---- output: one JSON object a line, whole lines only ---------------------------------------------------------------------------
std::mutex g_out;
void emit(const Value& v) {
    const std::string line = v.dump() + "\n";
    std::lock_guard<std::mutex> lock(g_out);
    std::fwrite(line.data(), 1, line.size(), stdout);
    std::fflush(stdout);
}
Value ev(const char* name) {
    Value v = Value::object();
    v.set("ev", name);
    return v;
}
void fail(const std::string& why) {
    Value v = ev("error");
    v.set("why", why);
    emit(v);
}

// ---- no Steam: players on one PC over localhost UDP (what the launcher tests run against) -----------------------------------------
class MockSteam : public ISteamApi {
public:
    MockSteam(SteamId me, int base, std::set<SteamId> strangers) : me_(me), base_(base), strangers_(std::move(strangers)) {
        WSADATA w;
        WSAStartup(MAKEWORD(2, 2), &w);
        sock_ = socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP);
        sockaddr_in a{};
        a.sin_family = AF_INET;
        a.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
        a.sin_port = htons(static_cast<u_short>(portOf(me)));
        ok_ = bind(sock_, reinterpret_cast<sockaddr*>(&a), sizeof a) == 0;
        u_long nb = 1;
        ioctlsocket(sock_, FIONBIO, &nb);
    }
    bool ok() const { return ok_; }
    SteamId me() override { return me_; }
    uint32_t nowMs() override { return GetTickCount(); }
    bool send(SteamId to, const uint8_t* data, uint32_t size, bool, int channel) override {
        Bytes b(9 + size);
        std::memcpy(b.data(), &me_, 8);
        b[8] = static_cast<uint8_t>(channel);
        if (size) std::memcpy(b.data() + 9, data, size);
        sockaddr_in a{};
        a.sin_family = AF_INET;
        a.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
        a.sin_port = htons(static_cast<u_short>(portOf(to)));
        return sendto(sock_, reinterpret_cast<const char*>(b.data()), static_cast<int>(b.size()), 0, reinterpret_cast<sockaddr*>(&a), sizeof a) >= 0;
    }
    bool read(Bytes& out, SteamId& from, int channel) override {
        Bytes b(70000);
        for (;;) {
            const int n = recv(sock_, reinterpret_cast<char*>(b.data()), static_cast<int>(b.size()), 0);
            if (n < 9) return false;
            if (b[8] != channel) continue;
            std::memcpy(&from, b.data(), 8);
            out.assign(b.begin() + 9, b.begin() + n);
            return true;
        }
    }
    void accept(SteamId) override {}
    bool isFriend(SteamId who) override { return strangers_.count(who) == 0; }
    bool takeSessionRequest(SteamId&) override { return false; }
    bool takeConnectFail(SteamId&, uint8_t&) override { return false; }
    void setHosting(bool on) override { hosting_ = on; }

private:
    int portOf(SteamId id) const { return base_ + static_cast<int>(id % 1000); }
    SteamId me_;
    int base_;
    std::set<SteamId> strangers_;
    SOCKET sock_ = INVALID_SOCKET;
    bool ok_ = false;
    bool hosting_ = false;
};

// ---- the commands from the launcher ------------------------------------------------------------------------------------------------
std::mutex g_inMutex;
std::deque<std::string> g_in;
std::atomic<bool> g_inClosed{false};

void readStdin() {
    std::string line;
    while (std::getline(std::cin, line)) {
        std::lock_guard<std::mutex> lock(g_inMutex);
        if (g_in.size() < 4096) g_in.push_back(line);
    }
    g_inClosed = true;
}

struct Friend { SteamId id; std::string name, mode; };

}  // namespace

int main(int argc, char** argv) {
    std::string gameDir, mockHosts, mockStrangers;
    SteamId mockId = 0;
    int mockBase = 0;
    for (int i = 1; i < argc; ++i) {
        const std::string a = argv[i];
        auto next = [&]() -> std::string { return i + 1 < argc ? argv[++i] : ""; };
        if (a == "--game-dir") gameDir = next();
        else if (a == "--mock-id") mockId = std::strtoull(next().c_str(), nullptr, 10);
        else if (a == "--mock-base") mockBase = std::atoi(next().c_str());
        else if (a == "--mock-hosts") mockHosts = next();
        else if (a == "--mock-strangers") mockStrangers = next();
    }

    std::unique_ptr<MockSteam> mock;
    nvcoop::coop::SteamBinding* real = nullptr;
    ISteamApi* api = nullptr;
    void (*runCallbacks)() = nullptr;
    void (*shutdown)() = nullptr;
    std::string name = "Courier";
    std::vector<Friend> mockFriends;

    if (mockId) {
        std::set<SteamId> strangers;
        size_t at = 0;
        while (at < mockStrangers.size()) {
            size_t c = mockStrangers.find(',', at);
            if (c == std::string::npos) c = mockStrangers.size();
            strangers.insert(std::strtoull(mockStrangers.substr(at, c - at).c_str(), nullptr, 10));
            at = c + 1;
        }
        at = 0;
        while (at < mockHosts.size()) {
            size_t c = mockHosts.find(',', at);
            if (c == std::string::npos) c = mockHosts.size();
            const std::string item = mockHosts.substr(at, c - at);
            const size_t colon = item.find(':');
            if (colon != std::string::npos) mockFriends.push_back({std::strtoull(item.substr(0, colon).c_str(), nullptr, 10), item.substr(colon + 1), ""});
            at = c + 1;
        }
        mock = std::make_unique<MockSteam>(mockId, mockBase, std::move(strangers));
        if (!mock->ok()) { fail("The test Steam could not open its port."); return 2; }
        api = mock.get();
    } else {
        if (gameDir.empty()) { fail("No game folder given."); return 2; }
        const std::string dllPath = gameDir + "\\steam_api.dll";
        SetEnvironmentVariableA("SteamAppId", "22380");
        SetEnvironmentVariableA("SteamGameId", "22380");
        HMODULE dll = LoadLibraryExA(dllPath.c_str(), nullptr, LOAD_WITH_ALTERED_SEARCH_PATH);
        if (!dll) { fail("Cannot load steam_api.dll from the game folder. Is Fallout: New Vegas installed from Steam?"); return 2; }
        auto init = reinterpret_cast<bool(__cdecl*)()>(GetProcAddress(dll, "SteamAPI_Init"));
        runCallbacks = reinterpret_cast<void (*)()>(GetProcAddress(dll, "SteamAPI_RunCallbacks"));
        shutdown = reinterpret_cast<void (*)()>(GetProcAddress(dll, "SteamAPI_Shutdown"));
        if (!init || !runCallbacks) { fail("The game's steam_api.dll is not the one Steam ships."); return 2; }
        if (!init()) { fail("Steam is not running or you are not logged in. Start Steam, then try again."); return 2; }
        real = &nvcoop::coop::SteamBinding::instance();
        std::string why;
        if (!real->attach(dll, &why)) { fail(why); return 2; }
        name = real->personaName();
        api = real;
    }

    SteamTransport link(*api);
    Value ready = ev("ready");
    ready.set("me", std::to_string(api->me())).set("name", name);
    emit(ready);

    std::thread(readStdin).detach();
    std::string lastError;
    for (bool quit = false; !quit;) {
        if (runCallbacks) runCallbacks();

        std::deque<std::string> lines;
        {
            std::lock_guard<std::mutex> lock(g_inMutex);
            lines.swap(g_in);
        }
        for (const std::string& line : lines) {
            Value cmd;
            std::string err;
            if (!nvcoop::json::parse(line, cmd, err) || !cmd.isObject()) continue;
            const std::string c = cmd.get("cmd").asString("");
            if (c == "quit") quit = true;
            else if (c == "friends") {
                Value list = Value::array();
                std::vector<Friend> found;
                if (real) {
                    for (auto& f : real->hostingFriends(cmd.get("tag").asString(""))) {
                        Value row = Value::object();
                        row.set("id", std::to_string(f.id)).set("name", f.name).set("mode", f.mode).set("same", f.sameRelease);
                        list.push(row);
                    }
                } else {
                    for (auto& f : mockFriends) {
                        Value row = Value::object();
                        row.set("id", std::to_string(f.id)).set("name", f.name).set("mode", f.mode).set("same", true);
                        list.push(row);
                    }
                }
                Value e = ev("friends");
                e.set("list", list);
                emit(e);
            } else if (c == "host") {
                link.stop();
                api->setHostTag(cmd.get("tag").asString(""), cmd.get("mode").asString(""));
                link.listen(8);
                emit(ev("hosting"));
            } else if (c == "join") {
                link.stop();
                const SteamId id = std::strtoull(cmd.get("id").asString("0").c_str(), nullptr, 10);
                if (!link.connect(id)) fail("That is not a Steam ID.");
                lastError.clear();
            } else if (c == "send") {
                Bytes data;
                if (b64dec(cmd.get("b64").asString(""), data) && !data.empty())
                    link.sendEx(static_cast<PeerId>(cmd.get("peer").asInt(0)), std::move(data), ITransport::Channel::Unreliable);
            } else if (c == "stop") {
                link.stop();
            }
        }
        if (g_inClosed) {
            std::lock_guard<std::mutex> lock(g_inMutex);
            if (g_in.empty()) quit = true;   // the launcher is gone
        }

        PeerId from;
        Bytes d;
        while (link.poll(from, d)) {
            Value e = ev("recv");
            e.set("peer", static_cast<int64_t>(from)).set("b64", b64enc(d));
            emit(e);
        }
        ILinkTransport::Event le;
        while (link.nextEvent(le)) {
            Value e = ev("peer");
            e.set("peer", static_cast<int64_t>(le.peer)).set("state", le.type == ILinkTransport::Event::Connected ? "up" : "down");
            e.set("id", std::to_string(link.steamIdOf(le.peer)));
            emit(e);
        }
        if (link.lastError() != lastError) {
            lastError = link.lastError();
            if (!lastError.empty()) {
                Value e = ev("link");
                e.set("error", lastError);
                emit(e);
            }
        }
        if (real) {
            SteamId who;
            std::string connect;
            while (real->takeJoinRequest(who, connect)) {
                Value e = ev("join_request");
                e.set("id", std::to_string(who)).set("connect", connect);
                emit(e);
            }
        }
        std::this_thread::sleep_for(std::chrono::milliseconds(20));
    }
    link.stop();
    if (shutdown) shutdown();
    return 0;
}
