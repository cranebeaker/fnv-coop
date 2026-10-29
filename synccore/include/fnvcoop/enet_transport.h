// ENet-backed ITransport (UDP). Channel 0 = reliable ordered, channel 1 = unreliable. Link with fnvcoop_enet.
// PeerId mapping: a client sees its server as peer 0; a server numbers clients by ENet peer slot + 1.
#pragma once
#include <deque>
#include <string>
#include <vector>
#include "fnvcoop/transport.h"

struct _ENetHost;
struct _ENetPeer;

namespace fnvcoop {

class EnetTransport : public ILinkTransport {
public:
    using Event = ILinkTransport::Event;

    EnetTransport() = default;
    ~EnetTransport() override;
    EnetTransport(const EnetTransport&) = delete;
    EnetTransport& operator=(const EnetTransport&) = delete;

    // Server: bind `port` (0 = any; see port()), up to maxPeers clients. bindAny=false binds 127.0.0.1 only.
    bool listen(uint16_t port, size_t maxPeers = 8, bool loopbackOnly = false);
    // Client: begin connecting to host:port (asynchronous; watch for the Connected event).
    bool connect(const std::string& host, uint16_t port);
    // Client: drop the dead link and start a fresh connection to the same host:port (after a Disconnected event).
    bool reconnect() override;
    void disconnect() override;  // graceful for all peers
    // A peer is dropped only after this long without an acknowledgement. ENet's default (5-30 s) is shorter than a long loading
    // screen, and the socket is serviced from the game's main loop, so a load would otherwise end the session.
    static constexpr unsigned kPeerTimeoutMinMs = 60000, kPeerTimeoutMaxMs = 120000, kPeerTimeoutLimit = 64;

    void send(PeerId to, Bytes data) override { sendEx(to, std::move(data), Channel::Reliable); }
    void sendEx(PeerId to, Bytes data, Channel ch) override;
    bool poll(PeerId& from, Bytes& data) override;  // services the socket
    bool nextEvent(Event& e) override;
    size_t backlogBytes(PeerId to) const override;

    uint16_t port() const { return port_; }
    bool isServer() const { return server_; }
    bool connected() const override { return connectedPeers_ > 0; }
    struct Stats { uint64_t datagramsIn = 0, datagramsOut = 0, bytesIn = 0, bytesOut = 0; };
    const Stats& stats() const { return stats_; }

private:
    void service();
    _ENetPeer* peerFor(PeerId id) const;
    PeerId idOf(const _ENetPeer* p) const;

    _ENetHost* host_ = nullptr;
    bool server_ = false;
    std::string connectHost_;
    uint16_t connectPort_ = 0;
    uint16_t port_ = 0;
    int connectedPeers_ = 0;
    std::deque<std::pair<PeerId, Bytes>> inbox_;
    std::deque<Event> events_;
    Stats stats_;
};

}  // namespace fnvcoop
