// Test rigs for plugin/chat: a transport-free hub of ChatCores (one host, guests by slot), and a real-mux rig (ChatSession over InMemoryNetwork).
#pragma once
#include <deque>
#include <map>
#include <memory>
#include <string>
#include <vector>

#include "chat/chat_core.h"
#include "chat/chat_session.h"
#include "chat/chat_view.h"
#include "fnvcoop/transport.h"
#include "fnvcoop/transport_util.h"
#include "minitest.h"

namespace nvchat {

using namespace nvcoop::chat;

inline std::vector<std::string> texts(const ChatLog& log) {
    std::vector<std::string> out;
    for (const Line& l : log.lines()) out.push_back(formatLine(l));
    return out;
}
inline bool has(const ChatLog& log, const std::string& formatted) {
    for (const Line& l : log.lines())
        if (formatLine(l) == formatted) return true;
    return false;
}
inline size_t count(const ChatLog& log, const std::string& formatted) {
    size_t n = 0;
    for (const Line& l : log.lines()) n += formatLine(l) == formatted ? 1 : 0;
    return n;
}

// One host core and any number of guest cores wired through a queue (no transport, no session adapter). pump() delivers in order.
struct Hub {
    struct Msg { Slot from, to; Bytes data; };
    Config cfg;
    ChatCore host;
    std::map<Slot, std::unique_ptr<ChatCore>> guests;
    std::deque<Msg> q;
    std::vector<Slot> joined;
    uint64_t now = 1000;

    explicit Hub(Config c = {}) : cfg(c), host(c, 7) {
        Ports p;
        p.sendToSlot = [this](Slot s, Bytes d) { q.push_back(Msg{0, s, std::move(d)}); };
        p.joinedSlots = [this] { return joined; };
        host.setPorts(std::move(p));
        host.begin(true, 0);
    }
    ChatCore& join(Slot s, const std::string& name = "", uint32_t idSeed = 0) {
        Config c = cfg;
        c.name = name;
        auto g = std::make_unique<ChatCore>(c, idSeed ? idSeed : 1000u * s + 1);
        Ports p;
        p.sendToHost = [this, s](Bytes d) { q.push_back(Msg{s, 0, std::move(d)}); };
        g->setPorts(std::move(p));
        g->begin(false, s);
        joined.push_back(s);
        host.playerJoined(s);
        ChatCore& ref = *g;
        guests[s] = std::move(g);
        ref.linkUp();
        pump();
        return ref;
    }
    void leave(Slot s) {
        for (size_t i = 0; i < joined.size(); ++i)
            if (joined[i] == s) { joined.erase(joined.begin() + static_cast<std::ptrdiff_t>(i)); break; }
        guests.erase(s);
        host.playerLeft(s, now);
        pump();
    }
    ChatCore& g(Slot s) { return *guests.at(s); }
    void pump() {
        for (int guard = 0; !q.empty() && guard < 10000; ++guard) {
            Msg m = std::move(q.front());
            q.pop_front();
            if (m.to == 0) host.onDatagram(m.from, m.data.data(), m.data.size(), now);
            else if (guests.count(m.to)) guests[m.to]->onDatagram(0, m.data.data(), m.data.size(), now);
        }
    }
    // A hand-made datagram as if guest `from` sent it.
    void inject(Slot from, const Bytes& d) { host.onDatagram(from, d.data(), d.size(), now); pump(); }
    void injectToGuest(Slot to, const Bytes& d) { g(to).onDatagram(0, d.data(), d.size(), now); }
};

// Host and guests on real MuxTransports over the in-memory network (the host is peer 0, guest slot s is peer s). pump() advances the clock and polls every mux.
struct MuxRig {
    fnvcoop::InMemoryNetwork net{5};
    fnvcoop::MuxTransport hostMux{*net.endpoint(0)};
    std::map<Slot, std::unique_ptr<fnvcoop::MuxTransport>> guestMux;
    ChatSession host;
    std::map<Slot, std::unique_ptr<ChatSession>> guest;
    std::vector<Slot> slots;
    uint64_t now = 1000;
    fnvcoop::MuxTransport* hostMuxOverride = nullptr;

    explicit MuxRig(Config c = {}) : host(c, 7) {}
    void addGuest(Slot s, const std::string& name = "") {
        Config c = host.core().config();
        c.name = name;
        guestMux[s] = std::make_unique<fnvcoop::MuxTransport>(*net.endpoint(s));
        guest[s] = std::make_unique<ChatSession>(c, 1000u * s + 1);
        slots.push_back(s);
    }
    FrameInfo hostInfo(bool joinedFlag = true) {
        FrameInfo f;
        f.active = true; f.isHost = true; f.self = 0; f.mux = hostMuxOverride ? hostMuxOverride : &hostMux; f.nowMs = now;
        for (Slot s : slots) f.clients.push_back(ClientLink{s, s, joinedFlag});
        return f;
    }
    FrameInfo guestInfo(Slot s, bool linkUp = true) {
        FrameInfo f;
        f.active = true; f.isHost = false; f.self = s; f.hostLinkUp = linkUp; f.mux = guestMux[s].get(); f.nowMs = now;
        return f;
    }
    void frames() {
        host.frame(hostInfo());
        for (Slot s : slots) guest[s]->frame(guestInfo(s));
    }
    void pump() {
        for (int i = 0; i < 4; ++i) {
            net.advance(now);
            fnvcoop::PeerId from; fnvcoop::Bytes d;
            while ((hostMuxOverride ? *hostMuxOverride : hostMux).poll(from, d)) {}
            for (Slot s : slots) while (guestMux[s]->poll(from, d)) {}
            frames();
        }
    }
};

}  // namespace nvchat
