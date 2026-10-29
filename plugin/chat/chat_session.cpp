#include "chat/chat_session.h"

#include "fnvcoop_mut.h"

// Twins: 1130 = a new session (new mux) keeps the old channel handler and state; 1131 = a datagram from an unknown peer is taken as the first guest's;
//        1132 = joins and leaves never reach the core; 1133 = the guest's link-up is ignored (no Hello); 1134 = the host does not ask for the clients again when
//        a datagram comes from a peer that joined this very frame; 1135 = datagrams are sent on the unreliable channel; 1136 = a handler of an ended session
//        still delivers
namespace nvcoop::chat {

Slot ChatSession::slotOfPeer(fnvcoop::PeerId p) const {
    if (!isHost_) return kHostSlot;
    for (const ClientLink& c : clients_)
        if (c.peer == p && c.joined) return c.slot;
    if (clientsNow_ && !FNV_MUT(1134))
        for (const ClientLink& c : clientsNow_())
            if (c.peer == p && c.joined) return c.slot;
    return kSystemSlot;
}

void ChatSession::send(Slot slot, Bytes data) {
    if (!mux_) return;
    const auto ch = FNV_MUT(1135) ? fnvcoop::ITransport::Channel::Unreliable : fnvcoop::ITransport::Channel::Reliable;
    if (!isHost_) { mux_->sendEx(0, std::move(data), ch); return; }   // a guest's only peer is the host
    for (const ClientLink& c : clients_)
        if (c.slot == slot && c.joined) { mux_->sendEx(c.peer, std::move(data), ch); return; }
    if (clientsNow_)
        for (const ClientLink& c : clientsNow_())
            if (c.slot == slot && c.joined) { mux_->sendEx(c.peer, std::move(data), ch); return; }
}

void ChatSession::teardown() {
    if (!FNV_MUT(1136)) *alive_ = 0;   // the handler registered on the old mux becomes a no-op
    core_.end();
    mux_ = nullptr;
    up_ = false;
    linkUp_ = false;
    clients_.clear();
    known_.clear();
}

void ChatSession::frame(const FrameInfo& f) {
    nowMs_ = f.nowMs;
    if (!f.active || !f.mux || (!f.isHost && f.self == kSystemSlot)) {
        if (up_) teardown();
        return;
    }
    const bool newMux = FNV_MUT(1130) ? false : f.mux != mux_;
    if (!up_ || newMux || f.isHost != isHost_ || (!f.isHost && f.self != self_)) {
        if (up_) teardown();
        ++sessions_;
        mux_ = f.mux;
        isHost_ = f.isHost;
        self_ = f.isHost ? kHostSlot : f.self;
        up_ = true;
        *alive_ = ++gen_;
        Ports p;
        p.sendToHost = [this](Bytes d) { send(kHostSlot, std::move(d)); };
        p.sendToSlot = [this](Slot s, Bytes d) { send(s, std::move(d)); };
        p.joinedSlots = [this]() {
            std::vector<Slot> out;
            for (const ClientLink& c : clientsNow_ ? clientsNow_() : clients_)
                if (c.joined && c.slot != kSystemSlot) out.push_back(c.slot);
            return out;
        };
        core_.setPorts(std::move(p));
        core_.begin(isHost_, self_);
        std::shared_ptr<uint64_t> alive = alive_;
        const uint64_t gen = gen_;
        f.mux->registerChannel(kChannelChat, [this, alive, gen](fnvcoop::PeerId from, fnvcoop::Bytes&& data) {
            if (*alive != gen && !FNV_MUT(1136)) return;
            Slot slot = slotOfPeer(from);
            if (slot == kSystemSlot) {
                ++droppedUnknownPeer_;
                if (!FNV_MUT(1131)) return;
                if (clients_.empty()) return;
                slot = clients_[0].slot;   // twin: an unknown peer is taken for the first guest (it could speak in that guest's name)
            }
            core_.onDatagram(slot, data.data(), data.size(), nowMs_);
        });
    }
    clients_ = f.clients;
    clientsNow_ = f.clientsNow;
    if (isHost_) {
        std::set<Slot> now;
        for (const ClientLink& c : f.clients)
            if (c.joined && c.slot != kSystemSlot && c.slot != kHostSlot) now.insert(c.slot);
        if (!FNV_MUT(1132)) {
            for (Slot s : now) if (!known_.count(s)) core_.playerJoined(s);
            for (Slot s : known_) if (!now.count(s)) core_.playerLeft(s, f.nowMs);
        }
        known_ = now;
    } else {
        if (f.hostLinkUp && !linkUp_ && !FNV_MUT(1133)) core_.linkUp();
        linkUp_ = f.hostLinkUp;
    }
}

}  // namespace nvcoop::chat
