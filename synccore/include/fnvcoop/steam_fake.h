// A pretend Steam for tests (docs/steam.md): any number of players on one virtual clock, packets between them, Steam's own rules kept:
// the first packet from a stranger raises a session request and is HELD until the receiver accepts it (a refused one is lost), only
// friends are friends, a reliable packet is never lost or reordered, an unreliable one can be dropped on request. Game-free.
#pragma once
#include <deque>
#include <map>
#include <memory>
#include <set>
#include <string>
#include "fnvcoop/steam_transport.h"

namespace fnvcoop {

class FakeSteamNet {
public:
    class Node : public ISteamApi {
    public:
        Node(FakeSteamNet& net, SteamId id) : net_(net), id_(id) {}
        SteamId me() override { return id_; }
        uint32_t nowMs() override { return net_.nowMs; }
        bool send(SteamId to, const uint8_t* data, uint32_t size, bool reliable, int channel) override {
            ++sent;
            if (net_.cutOff.count(id_) || net_.cutOff.count(to)) return true;   // the link is down: Steam takes it and loses it
            if (!reliable && net_.dropUnreliable) return true;
            Node* n = net_.find(to);
            if (!n) { fails.push_back({to, 1}); return false; }
            n->arrive(id_, Bytes(data, data + size), channel);
            return true;
        }
        bool read(Bytes& out, SteamId& from, int channel) override {
            auto it = inbox.find(channel);
            if (it == inbox.end() || it->second.empty()) return false;
            from = it->second.front().first;
            out = std::move(it->second.front().second);
            it->second.pop_front();
            ++reads;
            return true;
        }
        void accept(SteamId who) override {
            accepted.insert(who);
            auto h = held.find(who);
            if (h == held.end()) return;
            for (auto& [ch, data] : h->second) inbox[ch].emplace_back(who, std::move(data));
            held.erase(h);
        }
        bool isFriend(SteamId who) override { return friends.count(who) != 0; }
        bool takeSessionRequest(SteamId& who) override {
            if (requests.empty()) return false;
            who = requests.front();
            requests.pop_front();
            return true;
        }
        bool takeConnectFail(SteamId& who, uint8_t& reason) override {
            if (fails.empty()) return false;
            who = fails.front().first;
            reason = fails.front().second;
            fails.pop_front();
            return true;
        }
        size_t queuedBytes(SteamId) override { return queued; }
        void setHosting(bool on) override { hosting = on; ++hostingCalls; }
        void setHostTag(const std::string& t, const std::string& m) override { tag = t; mode = m; }

        // test knobs and counters
        std::set<SteamId> friends;
        std::set<SteamId> accepted;
        std::deque<SteamId> requests;
        std::deque<std::pair<SteamId, uint8_t>> fails;
        size_t queued = 0;
        bool hosting = false;
        std::string tag, mode;
        int hostingCalls = 0;
        uint64_t sent = 0, reads = 0;
        size_t waiting() const { size_t n = 0; for (auto& [c, q] : inbox) n += q.size(); return n; }
        size_t heldFrom(SteamId who) const { auto h = held.find(who); return h == held.end() ? 0 : h->second.size(); }

    private:
        void arrive(SteamId from, Bytes data, int channel) {
            if (accepted.count(from)) { inbox[channel].emplace_back(from, std::move(data)); return; }
            if (!held.count(from)) requests.push_back(from);   // one request per stranger, however many packets follow
            held[from].emplace_back(channel, std::move(data));
        }
        FakeSteamNet& net_;
        SteamId id_;
        std::map<int, std::deque<std::pair<SteamId, Bytes>>> inbox;
        std::map<SteamId, std::deque<std::pair<int, Bytes>>> held;
    };

    Node& player(SteamId id) {
        auto& p = nodes_[id];
        if (!p) p = std::make_unique<Node>(*this, id);
        return *p;
    }
    void befriend(SteamId a, SteamId b) { player(a).friends.insert(b); player(b).friends.insert(a); }
    Node* find(SteamId id) { auto it = nodes_.find(id); return it == nodes_.end() ? nullptr : it->second.get(); }

    uint32_t nowMs = 1000;
    std::set<SteamId> cutOff;        // players whose packets (both ways) are lost
    bool dropUnreliable = false;

private:
    std::map<SteamId, std::unique_ptr<Node>> nodes_;
};

}  // namespace fnvcoop
