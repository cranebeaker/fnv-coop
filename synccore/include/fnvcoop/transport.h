// Transport abstraction. Real ENet comes later; the in-memory network (with a fault model) is used by tests/sim.
// Delivery is message-oriented and may be lossy/duplicated/reordered: the sessions above are built for that.
#pragma once
#include <cstdint>
#include <functional>
#include <map>
#include <memory>
#include <queue>
#include <vector>
#include "fnvcoop/rng.h"

namespace fnvcoop {

using PeerId = uint32_t;
using Bytes = std::vector<uint8_t>;

class ITransport {
public:
    virtual ~ITransport() = default;
    enum class Channel : uint8_t { Reliable = 0, Unreliable = 1 };
    // Default send is the reliable ordered channel where the transport has one.
    virtual void send(PeerId to, Bytes data) = 0;
    // Explicit channel (ENet: channel 0 reliable-ordered, channel 1 unreliable). Others just forward to send().
    virtual void sendEx(PeerId to, Bytes data, Channel /*ch*/) { send(to, std::move(data)); }
    // Pops one received message; false when none are pending.
    virtual bool poll(PeerId& from, Bytes& data) = 0;
    // Bytes queued towards `to` that the network has not delivered yet (ENet: waiting + in-transit reliable data).
    // Bulk senders (Welcome snapshots, baseline chunks) hold back while this is large: more data would only queue behind it.
    virtual size_t backlogBytes(PeerId /*to*/) const { return 0; }
};

// A transport that has real connections (ENet): it reports peers coming and going, can drop a link and bring it back, and a
// client can ask for its host again after a drop. The co-op runtime drives it in plugin/coop frame().
// PeerId mapping: a client sees its host as peer 0; a host numbers its guests from 1.
class ILinkTransport : public ITransport {
public:
    struct Event { enum Type { Connected, Disconnected } type; PeerId peer; };
    virtual bool nextEvent(Event& e) = 0;
    virtual bool reconnect() = 0;          // client: drop the dead link and start a fresh connection to the same host
    virtual void disconnect() = 0;         // graceful for all peers
    virtual bool connected() const = 0;    // at least one peer is connected
};

struct LinkParams {
    uint32_t minLatencyMs = 0, maxLatencyMs = 0;  // uniform; jitter above min causes reordering
    double loss = 0;                              // 0..1 per message
    double duplicate = 0;                         // 0..1 chance of an extra copy
    double corrupt = 0;                           // 0..1 chance of a single flipped bit
};

class InMemoryNetwork {
public:
    explicit InMemoryNetwork(uint64_t seed = 1);
    ~InMemoryNetwork();

    ITransport* endpoint(PeerId id);                   // created on first use
    void setLink(const LinkParams& p) { link_ = p; }   // applies to every direction
    const LinkParams& link() const { return link_; }
    // Optional link model replacing LinkParams (e.g. T5's chaos::Impairment). Called per datagram with the virtual time; it may
    // flip bits in `data` and returns the delivery delays in ms (empty = dropped, 2+ = duplicates). Null = use LinkParams.
    using Shaper = std::function<std::vector<uint32_t>(PeerId from, PeerId to, uint64_t nowMs, Bytes& data)>;
    void setShaper(Shaper s) { shaper_ = std::move(s); }
    void setBandwidth(uint32_t bytesPerSec) { bandwidth_ = bytesPerSec; }   // per direction per peer pair; 0 = unlimited
    uint64_t queuedBytes(PeerId from, PeerId to) const;
    uint64_t bytesBetween(PeerId from, PeerId to) const { auto it = sentBytes_.find({from, to}); return it == sentBytes_.end() ? 0 : it->second; }   // accepted by the link, per direction
    void setConnected(PeerId id, bool up);             // down: its queued traffic is dropped, sends vanish
    bool connected(PeerId id) const;
    void advance(uint64_t nowMs);                      // delivers everything whose time has come
    size_t inFlight() const { return pending_.size(); }
    bool idle() const;                                 // nothing in flight and no unread inbox

    struct Stats { uint64_t bytesSent = 0, sent = 0, delivered = 0, lost = 0, duplicated = 0, corrupted = 0, droppedDown = 0; };
    const Stats& stats() const { return stats_; }

private:
    struct Msg {
        uint64_t at; uint64_t order; PeerId from, to; Bytes data;
        bool operator>(const Msg& o) const { return at != o.at ? at > o.at : order > o.order; }
    };
    class Endpoint;
    void sendFrom(PeerId from, PeerId to, Bytes data);
    void enqueue(PeerId from, PeerId to, Bytes data, int64_t forcedLatency = -1);

    Rng rng_;
    Shaper shaper_;
    LinkParams link_;
    uint64_t now_ = 0, order_ = 0;
    uint32_t bandwidth_ = 0;
    std::map<std::pair<PeerId, PeerId>, uint64_t> queued_, busyUntil_, sentBytes_;
    std::map<PeerId, std::unique_ptr<Endpoint>> eps_;
    std::map<PeerId, bool> up_;
    std::priority_queue<Msg, std::vector<Msg>, std::greater<Msg>> pending_;
    Stats stats_;
};

}  // namespace fnvcoop
