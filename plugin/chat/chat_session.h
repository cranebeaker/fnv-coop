// ChatSession: plugs a ChatCore into the co-op session's mux (first byte kChannelChat). Game-free, the same shape as plugin/areadigest AreaRuntime: the glue (or
// a test) calls frame() every frame with the facts it knows; this class registers the channel when a session starts, maps peers to slots, tells the core
// about joins, leaves and the guest's link, and lets go of everything when the session ends. The core (and its history) outlives sessions.
#pragma once
#include <functional>
#include <memory>
#include <set>
#include <vector>

#include "chat/chat_core.h"
#include "fnvcoop/transport_util.h"

namespace nvcoop::chat {

struct ClientLink { Slot slot = kSystemSlot; fnvcoop::PeerId peer = 0; bool joined = false; };

struct FrameInfo {
    bool active = false;                          // the co-op session is running
    bool isHost = false;
    Slot self = kSystemSlot;                      // guest: the slot the host gave (kSystemSlot until joined)
    bool hostLinkUp = false;                      // guest: joined and not in the baseline transfer
    std::vector<ClientLink> clients;              // host: the clients the session knows
    std::function<std::vector<ClientLink>()> clientsNow;   // host, optional: asked when a datagram comes from a peer not in `clients` (it may have joined this frame)
    fnvcoop::MuxTransport* mux = nullptr;         // the session's mux; a new pointer = a new session
    uint64_t nowMs = 0;
};

class ChatSession {
public:
    explicit ChatSession(Config cfg = {}, uint32_t idSeed = 1) : core_(std::move(cfg), idSeed), alive_(std::make_shared<uint64_t>(0)) {}
    ~ChatSession() { *alive_ = 0; }
    ChatSession(const ChatSession&) = delete;
    ChatSession& operator=(const ChatSession&) = delete;

    void frame(const FrameInfo& f);
    ChatCore& core() { return core_; }
    const ChatCore& core() const { return core_; }
    bool up() const { return up_; }
    uint64_t sessions() const { return sessions_; }
    uint64_t droppedUnknownPeer() const { return droppedUnknownPeer_; }

private:
    void teardown();
    Slot slotOfPeer(fnvcoop::PeerId p) const;
    void send(Slot slot, Bytes data);

    ChatCore core_;
    std::shared_ptr<uint64_t> alive_;             // the generation the registered handler belongs to; 0 = none
    uint64_t gen_ = 0, sessions_ = 0, droppedUnknownPeer_ = 0;
    fnvcoop::MuxTransport* mux_ = nullptr;
    bool up_ = false, isHost_ = false, linkUp_ = false;
    Slot self_ = kSystemSlot;
    std::vector<ClientLink> clients_;
    std::function<std::vector<ClientLink>()> clientsNow_;
    std::set<Slot> known_;                        // host: guests announced to the core as joined
    uint64_t nowMs_ = 0;
};

}  // namespace nvcoop::chat
