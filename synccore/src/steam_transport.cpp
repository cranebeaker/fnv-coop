#include "fnvcoop/steam_transport.h"
#include <algorithm>

namespace fnvcoop {

namespace {
// First byte of every packet. 0..2 carry one of co-op's messages (whole, a piece, the last piece); the letters are the link's own.
constexpr uint8_t kWhole = 0, kPiece = 1, kLastPiece = 2;
constexpr uint8_t kConnect = 'C', kAccepted = 'A', kStillHere = 'K', kEcho = 'E', kBye = 'B';
constexpr uint32_t kConnectEveryMs = 1000, kKeepAliveEveryMs = 1000;

bool due(uint32_t now, uint32_t at) { return static_cast<int32_t>(now - at) >= 0; }
}  // namespace

bool SteamTransport::listen(size_t maxPeers) {
    stop();
    running_ = true;
    server_ = true;
    maxPeers_ = maxPeers;
    nextPeer_ = 1;
    lastPump_ = 0;
    looked_ = false;
    lastError_.clear();
    steam_.setHosting(true);
    return true;
}

bool SteamTransport::connect(SteamId host) {
    stop();
    if (!host) return false;
    running_ = true;
    server_ = false;
    host_ = host;
    lastPump_ = 0;
    looked_ = false;
    lastError_.clear();
    connecting_ = true;
    connectSince_ = nextConnect_ = steam_.nowMs();
    return true;
}

bool SteamTransport::reconnect() {
    if (server_ || !host_) return false;
    peers_.clear();
    inbox_.clear();
    events_.clear();
    running_ = true;
    looked_ = false;
    lastPump_ = 0;
    lastError_.clear();
    connecting_ = true;
    connectSince_ = nextConnect_ = steam_.nowMs();
    return true;
}

void SteamTransport::stop() {
    if (!running_) return;
    for (const auto& [peer, p] : peers_) sendControl(p.id, kBye, 0, true);
    if (server_) steam_.setHosting(false);
    peers_.clear();
    inbox_.clear();
    events_.clear();
    running_ = connecting_ = false;
    host_ = 0;
    lastPump_ = 0;
    looked_ = false;
}

PeerId SteamTransport::peerOf(SteamId id) const {
    for (const auto& [peer, p] : peers_)
        if (p.id == id) return peer;
    return kNoPeer;
}

void SteamTransport::sendControl(SteamId to, uint8_t kind, uint32_t value, bool reliable) {
    const uint8_t packet[5] = {kind, static_cast<uint8_t>(value), static_cast<uint8_t>(value >> 8), static_cast<uint8_t>(value >> 16),
                               static_cast<uint8_t>(value >> 24)};
    steam_.send(to, packet, sizeof(packet), reliable, kChannel);
    ++stats_.packetsOut;
}

void SteamTransport::sendEx(PeerId to, Bytes data, Channel ch) {
    auto it = peers_.find(to);
    if (it == peers_.end() || data.empty() || data.size() > kMaxMessage) return;
    const SteamId id = it->second.id;
    stats_.bytesOut += data.size();
    const bool reliable = ch == Channel::Reliable || data.size() + 1 > kUnreliableMax;
    if (data.size() + 1 <= kWholeMax) {
        data.insert(data.begin(), kWhole);
        steam_.send(id, data.data(), static_cast<uint32_t>(data.size()), reliable, kChannel);
        ++stats_.packetsOut;
        return;
    }
    // Pieces go reliable on the one channel: they arrive in order, and a "bye" sent after them cannot overtake them.
    const size_t piece = kWholeMax - 1;
    Bytes buf;
    for (size_t at = 0; at < data.size(); at += piece) {
        const size_t n = std::min(piece, data.size() - at);
        const bool last = at + n >= data.size();
        buf.assign(1, last ? kLastPiece : kPiece);
        buf.insert(buf.end(), data.begin() + static_cast<std::ptrdiff_t>(at), data.begin() + static_cast<std::ptrdiff_t>(at + n));
        steam_.send(id, buf.data(), static_cast<uint32_t>(buf.size()), true, kChannel);
        ++stats_.packetsOut;
        ++stats_.pieces;
    }
}

size_t SteamTransport::backlogBytes(PeerId to) const {
    auto it = peers_.find(to);
    return it == peers_.end() ? 0 : steam_.queuedBytes(it->second.id);
}

uint32_t SteamTransport::rttMs(PeerId peer) const {
    auto it = peers_.find(peer);
    return it == peers_.end() ? 0 : it->second.rtt;
}

SteamId SteamTransport::steamIdOf(PeerId peer) const {
    auto it = peers_.find(peer);
    return it == peers_.end() ? 0 : it->second.id;
}

void SteamTransport::dropPeer(PeerId peer, bool event) {
    if (!peers_.erase(peer)) return;
    if (event) events_.push_back({Event::Disconnected, peer});
}

void SteamTransport::control(SteamId from, const Bytes& packet, uint32_t now) {
    if (packet.size() != 5) return;
    const uint8_t kind = packet[0];
    const uint32_t value = packet[1] | (packet[2] << 8) | (packet[3] << 16) | (static_cast<uint32_t>(packet[4]) << 24);
    PeerId peer = peerOf(from);
    if (peer != kNoPeer) peers_[peer].lastHeard = now;
    switch (kind) {
        case kConnect:
            if (!server_ || value != kConnectMagic) return;
            if (!steamTwins().acceptAnyone && !steam_.isFriend(from)) { ++stats_.refused; return; }   // a stranger gets no answer at all
            if (peer == kNoPeer) {
                if (peers_.size() >= maxPeers_) return sendControl(from, kBye, 0, true);
                peer = nextPeer_++;
                Peer p;
                p.id = from;
                p.lastHeard = now;
                peers_[peer] = std::move(p);
                events_.push_back({Event::Connected, peer});
            }
            sendControl(from, kAccepted, 0, true);   // again if they ask again
            return;
        case kAccepted:
            if (server_ || from != host_ || !connecting_) return;
            connecting_ = false;
            lastError_.clear();
            {
                Peer p;
                p.id = from;
                p.lastHeard = now;
                peers_[0] = std::move(p);
            }
            events_.push_back({Event::Connected, 0});
            return;
        case kStillHere:
            if (peer != kNoPeer) sendControl(from, kEcho, value, false);
            return;
        case kEcho:
            if (peer != kNoPeer) peers_[peer].rtt = now - value;
            return;
        case kBye:
            if (steamTwins().byeIgnored) return;
            if (peer != kNoPeer) dropPeer(peer, true);
            else if (!server_ && from == host_ && connecting_) {
                connecting_ = false;
                lastError_ = "The host's party is full.";
                events_.push_back({Event::Disconnected, 0});
            }
            return;
        default: return;
    }
}

void SteamTransport::pump(uint32_t now) {
    // Our game was not running (a loading screen): we could not hear anyone, so that is not their silence.
    if (lastPump_ && static_cast<int32_t>(now - lastPump_) > static_cast<int32_t>(kPausedMs) && !steamTwins().noPauseForgiveness) {
        for (auto& [peer, p] : peers_) p.lastHeard = now;
        connectSince_ += now - lastPump_;
    }
    lastPump_ = now;

    SteamId who = 0;
    while (steam_.takeSessionRequest(who)) {
        // Only friends (as the host), or the host (as a guest), get through to us at all.
        const bool let = server_ ? steam_.isFriend(who) : who == host_;
        if ((let || steamTwins().acceptAnyone) && !steamTwins().noSessionAccept) steam_.accept(who);
    }
    uint8_t reason = 0;
    while (steam_.takeConnectFail(who, reason)) {
        const PeerId peer = peerOf(who);
        if (peer != kNoPeer) {
            lastError_ = "Lost " + name(who) + " (Steam reason " + std::to_string(reason) + ").";
            dropPeer(peer, true);
        } else if (!server_ && who == host_ && connecting_) {
            // The host's game may just not be up yet: keep saying "connect" until connectMs_ runs out.
            lastError_ = reason == 1 ? "The host's game is not running (yet)." : reason == 2 ? "Steam says one of you does not own the game." :
                reason == 3 ? "The host is not online on Steam." : "Steam could not reach the host.";
        }
    }

    Bytes packet;
    size_t bytes = 0;
    int packets = 0;
    while ((steamTwins().unboundedRound || (packets < kMaxPacketsPerRound && bytes < kMaxBytesPerRound)) && steam_.read(packet, who, kChannel)) {
        ++packets;
        bytes += packet.size();
        ++stats_.packetsIn;
        stats_.bytesIn += packet.size();
        if (packet.empty()) continue;
        const uint8_t kind = packet[0];
        if (kind > kLastPiece) { control(who, packet, now); continue; }
        const PeerId peer = peerOf(who);
        if (peer == kNoPeer) continue;   // only from who we are connected to
        Peer& p = peers_[peer];
        p.lastHeard = now;
        if (kind == kWhole) {
            if (packet.size() < 2 || packet.size() > kMaxMessage + 1) continue;
            packet.erase(packet.begin());
            inbox_.emplace_back(peer, std::move(packet));
            packet = Bytes();
            continue;
        }
        p.inPieces = true;
        p.partial.insert(p.partial.end(), packet.begin() + 1, packet.end());
        if (p.partial.size() > kMaxMessage) { p.partial.clear(); p.inPieces = false; ++stats_.dropped; continue; }
        if (kind == kLastPiece) {
            if (!p.partial.empty()) inbox_.emplace_back(peer, std::move(p.partial));
            p.partial = Bytes();
            p.inPieces = false;
        }
    }

    if (!server_ && connecting_) {
        if (static_cast<int32_t>(now - connectSince_) > static_cast<int32_t>(connectMs_)) {
            if (lastError_.empty()) lastError_ = "The host did not answer.";
            connecting_ = false;
            events_.push_back({Event::Disconnected, 0});
        } else if (due(now, nextConnect_)) {
            sendControl(host_, kConnect, kConnectMagic, true);
            nextConnect_ = now + kConnectEveryMs;
        }
    }

    const bool keepAlive = due(now, nextKeepAlive_) && !steamTwins().noKeepAlive;
    if (keepAlive) nextKeepAlive_ = now + kKeepAliveEveryMs;
    std::vector<PeerId> silent;
    for (auto& [peer, p] : peers_) {
        if (!steamTwins().noTimeout && static_cast<int32_t>(now - p.lastHeard) > static_cast<int32_t>(silentMs_)) silent.push_back(peer);
        else if (keepAlive) sendControl(p.id, kStillHere, now, false);
    }
    for (PeerId peer : silent) {
        lastError_ = "Nothing from " + name(peers_[peer].id) + " for " + std::to_string(silentMs_ / 1000) + " s.";
        dropPeer(peer, true);
    }
}

bool SteamTransport::poll(PeerId& from, Bytes& data) {
    if (!running_) return false;
    if (inbox_.empty()) {
        // One look at Steam per round of poll() calls (a frame): what it holds (up to the caps), then "no more" until the next round.
        if (looked_) { looked_ = false; return false; }
        pump(steam_.nowMs());
        looked_ = true;
    }
    if (inbox_.empty()) { looked_ = false; return false; }
    if (steamTwins().lifoInbox) {
        from = inbox_.back().first;
        data = std::move(inbox_.back().second);
        inbox_.pop_back();
    } else {
        from = inbox_.front().first;
        data = std::move(inbox_.front().second);
        inbox_.pop_front();
    }
    return true;
}

bool SteamTransport::nextEvent(Event& e) {
    if (events_.empty()) return false;
    e = events_.front();
    events_.pop_front();
    return true;
}

}  // namespace fnvcoop
