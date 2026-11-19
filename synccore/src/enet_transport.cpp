#include "fnvcoop/enet_transport.h"
#include <enet/enet.h>
#include <atomic>

namespace fnvcoop {

namespace {
std::atomic<int> g_enetUsers{0};
void* (*g_enetMalloc)(size_t) = nullptr;
void (*g_enetFree)(void*) = nullptr;
int enetInit() {
    if (!g_enetMalloc || !g_enetFree) return enet_initialize();
    ENetCallbacks cb{};
    cb.malloc = g_enetMalloc;
    cb.free = g_enetFree;
    return enet_initialize_with_callbacks(ENET_VERSION, &cb);
}
bool enetAcquire() {
    if (g_enetUsers.fetch_add(1) == 0 && enetInit() != 0) { g_enetUsers.fetch_sub(1); return false; }
    return true;
}
void enetRelease() {
    if (g_enetUsers.fetch_sub(1) == 1) enet_deinitialize();
}
constexpr size_t kChannels = 2;
// ENetPacket::userData points at the owning peer's counter in EnetTransport::queued_.
void onPacketFree(ENetPacket* p) {
    if (auto* n = static_cast<uint64_t*>(p->userData)) *n = *n >= p->dataLength ? *n - p->dataLength : 0;
}
}  // namespace

void EnetTransport::setAllocator(void* (*mallocFn)(size_t), void (*freeFn)(void*)) {
    if (g_enetUsers.load() != 0) return;   // ENet is running with the old pair: blocks must go back to the allocator that made them
    g_enetMalloc = mallocFn;
    g_enetFree = freeFn;
}

void EnetTransport::createQueued(size_t slots) {
    queued_.reset(new uint64_t[slots ? slots : 1]());
    queuedSlots_ = slots;
}

EnetTransport::~EnetTransport() {
    if (host_) {
        enet_host_destroy(host_);
        host_ = nullptr;
        enetRelease();
    }
}

bool EnetTransport::listen(uint16_t port, size_t maxPeers, bool loopbackOnly) {
    if (host_ || !enetAcquire()) return false;
    ENetAddress a;
    a.host = loopbackOnly ? ENET_HOST_TO_NET_32(0x7F000001) : ENET_HOST_ANY;
    a.port = port;
    host_ = enet_host_create(&a, maxPeers, kChannels, 0, 0);
    if (!host_) { enetRelease(); return false; }
    createQueued(host_->peerCount);
    server_ = true;
    port_ = host_->address.port ? host_->address.port : port;
    return true;
}

bool EnetTransport::connect(const std::string& hostName, uint16_t port) {
    if (host_ || !enetAcquire()) return false;
    host_ = enet_host_create(nullptr, 1, kChannels, 0, 0);
    if (!host_) { enetRelease(); return false; }
    createQueued(host_->peerCount);
    ENetAddress a;
    if (enet_address_set_host(&a, hostName.c_str()) != 0) {
        enet_host_destroy(host_); host_ = nullptr; enetRelease(); return false;
    }
    a.port = port;
    ENetPeer* peer = enet_host_connect(host_, &a, kChannels, 0);
    if (!peer) {
        enet_host_destroy(host_); host_ = nullptr; enetRelease(); return false;
    }
    enet_peer_timeout(peer, kPeerTimeoutLimit, kPeerTimeoutMinMs, kPeerTimeoutMaxMs);
    server_ = false;
    connectHost_ = hostName;
    connectPort_ = port;
    return true;
}

bool EnetTransport::reconnect() {
    if (server_ || connectHost_.empty()) return false;
    if (host_) {
        enet_host_destroy(host_);
        host_ = nullptr;
        enetRelease();
    }
    connectedPeers_ = 0;
    inbox_.clear();
    events_.clear();
    const std::string h = connectHost_;
    return connect(h, connectPort_);
}

void EnetTransport::disconnect() {
    if (!host_) return;
    for (size_t i = 0; i < host_->peerCount; ++i)
        if (host_->peers[i].state == ENET_PEER_STATE_CONNECTED) enet_peer_disconnect(&host_->peers[i], 0);
    enet_host_flush(host_);
}

PeerId EnetTransport::idOf(const _ENetPeer* p) const {
    return server_ ? static_cast<PeerId>(p - host_->peers) + 1 : 0;
}

_ENetPeer* EnetTransport::peerFor(PeerId id) const {
    if (!host_) return nullptr;
    if (!server_) return id == 0 && host_->peerCount ? &host_->peers[0] : nullptr;
    if (id == 0 || id > host_->peerCount) return nullptr;
    return &host_->peers[id - 1];
}

void EnetTransport::sendEx(PeerId to, Bytes data, Channel ch) {
    ENetPeer* p = peerFor(to);
    if (!p || p->state != ENET_PEER_STATE_CONNECTED) return;
    stats_.datagramsOut++;
    stats_.bytesOut += data.size();
    if (!data.empty()) stats_.bytesOutByMux[data[0]] += data.size();
    ENetPacket* pk = enet_packet_create(data.data(), data.size(),
                                        ch == Channel::Reliable ? ENET_PACKET_FLAG_RELIABLE : 0);
    if (!pk) return;
    const size_t slot = static_cast<size_t>(p - host_->peers);
    if (slot < queuedSlots_) {
        queued_[slot] += pk->dataLength;
        pk->userData = &queued_[slot];
        pk->freeCallback = &onPacketFree;
    }
    if (enet_peer_send(p, ch == Channel::Reliable ? 0 : 1, pk) != 0) enet_packet_destroy(pk);
    enet_host_flush(host_);
    const uint64_t backlog = slot < queuedSlots_ ? queued_[slot] : 0;
    if (backlog > stats_.maxBacklog) stats_.maxBacklog = backlog;
    if (maxBacklogBytes_ && backlog > maxBacklogBytes_ && p->state == ENET_PEER_STATE_CONNECTED) {
        ++stats_.backlogDrops;
        const PeerId id = idOf(p);
        enet_peer_disconnect_now(p, 0);   // frees the queue now; ENet raises no local event for this, so report it ourselves
        if (connectedPeers_ > 0) --connectedPeers_;
        events_.push_back({Event::Disconnected, id});
    }
}

void EnetTransport::service() {
    if (!host_) return;
    ENetEvent ev;
    while (enet_host_service(host_, &ev, 0) > 0) {
        switch (ev.type) {
            case ENET_EVENT_TYPE_CONNECT:
                enet_peer_timeout(ev.peer, kPeerTimeoutLimit, kPeerTimeoutMinMs, kPeerTimeoutMaxMs);  // server side of the link too
                ++connectedPeers_;
                events_.push_back({Event::Connected, idOf(ev.peer)});
                break;
            case ENET_EVENT_TYPE_DISCONNECT:
                if (connectedPeers_ > 0) --connectedPeers_;
                events_.push_back({Event::Disconnected, idOf(ev.peer)});
                break;
            case ENET_EVENT_TYPE_RECEIVE:
                stats_.datagramsIn++;
                stats_.bytesIn += ev.packet->dataLength;
                inbox_.emplace_back(idOf(ev.peer), Bytes(ev.packet->data, ev.packet->data + ev.packet->dataLength));
                enet_packet_destroy(ev.packet);
                break;
            default: break;
        }
    }
}

bool EnetTransport::poll(PeerId& from, Bytes& data) {
    if (inbox_.empty()) service();
    if (inbox_.empty()) return false;
    from = inbox_.front().first;
    data = std::move(inbox_.front().second);
    inbox_.pop_front();
    return true;
}

size_t EnetTransport::backlogBytes(PeerId to) const {
    ENetPeer* p = peerFor(to);
    if (!p) return 0;
    const size_t slot = static_cast<size_t>(p - host_->peers);
    return slot < queuedSlots_ ? static_cast<size_t>(queued_[slot]) : 0;
}

bool EnetTransport::nextEvent(Event& e) {
    if (events_.empty()) service();
    if (events_.empty()) return false;
    e = events_.front();
    events_.pop_front();
    return true;
}

}  // namespace fnvcoop
