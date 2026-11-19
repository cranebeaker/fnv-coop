// ENet-backed ITransport (UDP). Channel 0 = reliable ordered, channel 1 = unreliable. Link with fnvcoop_enet.
// PeerId mapping: a client sees its server as peer 0; a server numbers clients by ENet peer slot + 1.
#pragma once
#include <deque>
#include <memory>
#include <string>
#include <vector>
#include "fnvcoop/transport.h"

struct _ENetHost;
struct _ENetPeer;

namespace fnvcoop {

class EnetTransport : public ITransport {
public:
    struct Event { enum Type { Connected, Disconnected } type; PeerId peer; };

    EnetTransport() = default;
    ~EnetTransport() override;
    EnetTransport(const EnetTransport&) = delete;
    EnetTransport& operator=(const EnetTransport&) = delete;

    // Server: bind `port` (0 = any; see port()), up to maxPeers clients. bindAny=false binds 127.0.0.1 only.
    bool listen(uint16_t port, size_t maxPeers = 8, bool loopbackOnly = false);
    // Client: begin connecting to host:port (asynchronous; watch for the Connected event).
    bool connect(const std::string& host, uint16_t port);
    // Client: drop the dead link and start a fresh connection to the same host:port (after a Disconnected event).
    bool reconnect();
    void disconnect();  // graceful for all peers
    // A peer is dropped only after this long without an acknowledgement. ENet's default (5-30 s) is shorter than a long loading
    // screen, and the socket is serviced from the game's main loop, so a load would otherwise end the session.
    static constexpr unsigned kPeerTimeoutMinMs = 60000, kPeerTimeoutMaxMs = 120000, kPeerTimeoutLimit = 64;

    void send(PeerId to, Bytes data) override { sendEx(to, std::move(data), Channel::Reliable); }
    void sendEx(PeerId to, Bytes data, Channel ch) override;
    bool poll(PeerId& from, Bytes& data) override;  // services the socket
    bool nextEvent(Event& e);
    size_t backlogBytes(PeerId to) const override;

    uint16_t port() const { return port_; }
    bool isServer() const { return server_; }
    bool connected() const { return connectedPeers_ > 0; }
    struct Stats {
        uint64_t datagramsIn = 0, datagramsOut = 0, bytesIn = 0, bytesOut = 0;
        uint64_t backlogDrops = 0;      // peers dropped because their send queue passed maxBacklogBytes
        uint64_t maxBacklog = 0;        // largest send queue towards one peer seen at a send
        uint64_t bytesOutByMux[256] = {};   // bytes sent, by the datagram's first byte (the MuxTransport channel)
    };
    const Stats& stats() const { return stats_; }
    // ENet keeps every reliable packet until the peer acknowledges it. A sender that outruns the link (2026-11-17: 126 KB per frame)
    // would grow that queue until the game runs out of address space; past this many bytes the peer is dropped instead (the client
    // reconnects and rejoins). 0 = no limit.
    static constexpr size_t kDefaultMaxBacklogBytes = 64u << 20;
    void setMaxBacklogBytes(size_t n) { maxBacklogBytes_ = n; }

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
    size_t maxBacklogBytes_ = kDefaultMaxBacklogBytes;
    // Bytes of our packets ENet still holds, per ENet peer slot: added at send, taken off by the packet's free callback (after the
    // ack for a reliable one, after sending for an unreliable one, or when the peer is reset). ENet has no outgoing byte count of its
    // own (totalWaitingData is RECEIVED data waiting to be read). Heap-allocated so the callbacks of a destroyed host never dangle.
    std::unique_ptr<uint64_t[]> queued_;
    size_t queuedSlots_ = 0;
    void createQueued(size_t slots);
};

}  // namespace fnvcoop
