// Co-op over Steam (docs/steam.md): an ILinkTransport on Steam's peer-to-peer packets. No IP addresses and no open ports: Steam
// finds a way between two players, or relays. Only the host's Steam friends get in. Game-free: the Steam calls sit behind
// ISteamApi (plugin/coop/steam_binding.* is the real one, steam_fake.h the test double).
//
// Steam's P2P has no connections, only packets between Steam IDs, so this adds them: a friend says "connect" until the host answers
// "accepted"; both say "still here" every second (the echo gives the round trip); a silent peer is dropped after silentMs; "bye" ends
// a link at once. Every packet is on ONE Steam channel and starts with a kind byte, so a "bye" can never overtake the messages sent
// before it. Messages above kWholeMax are cut into reliable pieces and put back together (Steam's limit is 1 MB a packet; a Welcome
// snapshot is bigger). At most kMaxPacketsPerRound packets (and kMaxBytesPerRound bytes) are read per round of poll() calls, so a
// flood cannot hold a frame up; the rest waits inside Steam for the next frame.
//
// The same PeerId mapping as EnetTransport: a client sees its host as peer 0, a host numbers its guests 1, 2, ... Call from one
// thread (the game's): Steam is asked from poll().
#pragma once
#include <cstdint>
#include <deque>
#include <map>
#include <string>
#include <vector>
#include "fnvcoop/transport.h"

namespace fnvcoop {

using SteamId = uint64_t;
constexpr PeerId kNoPeer = 0xFFFFFFFFu;   // (peer 0 is real on a client: its host)

// The parts of Steam the transport needs. One implementation talks to the game's own Steam client, one is a pretend Steam.
class ISteamApi {
public:
    virtual ~ISteamApi() = default;
    virtual SteamId me() = 0;
    virtual uint32_t nowMs() = 0;                                     // wraps; compared with signed differences
    virtual bool send(SteamId to, const uint8_t* data, uint32_t size, bool reliable, int channel) = 0;
    virtual bool read(Bytes& out, SteamId& from, int channel) = 0;    // the next packet on a channel, if any
    virtual void accept(SteamId who) = 0;                             // let their packets through
    virtual bool isFriend(SteamId who) = 0;
    virtual bool takeSessionRequest(SteamId& who) = 0;                // someone we do not talk to yet wants to
    // Steam could not reach them. reason is Steam's: 1 their game is not running, 2 we or they do not own the game, 3 they are not logged in, 4 timeout.
    virtual bool takeConnectFail(SteamId& who, uint8_t& reason) = 0;
    virtual size_t queuedBytes(SteamId /*to*/) { return 0; }          // bytes Steam has not sent yet
    virtual void setHostTag(const std::string& /*tag*/, const std::string& /*mode*/) {}   // what the "joinable" presence says (a friend lists only matching hosts)
    virtual void setHosting(bool /*on*/) {}                           // friends see "joinable" (rich presence) while hosting
};

// TEST-ONLY fault switches (the mutation twins of test_steam_transport.cpp); all false in production. Each one re-introduces a bug the
// matching test must catch. Kept here, not in mutations.h, so this file stands alone.
struct SteamTwins {
    bool acceptAnyone = false;       // the host lets in anyone who says "connect", friend or not
    bool noKeepAlive = false;        // nobody says "still here": a quiet link looks dead after silentMs
    bool noTimeout = false;          // a peer that went silent is never dropped
    bool unboundedRound = false;     // poll() reads everything Steam holds, however much (a flood holds the frame up)
    bool noPauseForgiveness = false; // a gap in our polls (a loading screen) counts as the peers' silence
    bool lifoInbox = false;          // messages come out newest first (order on one channel lost)
    bool noSessionAccept = false;    // a stranger's first packet is never accepted at Steam's level (Steam holds it for ever)
    bool byeIgnored = false;         // "bye" is not acted on (a left player stays until the timeout)
};
inline SteamTwins& steamTwins() {
    static SteamTwins t;
    return t;
}
struct SteamTwinScope {
    SteamTwins saved;
    explicit SteamTwinScope(const SteamTwins& t) : saved(steamTwins()) { steamTwins() = t; }
    ~SteamTwinScope() { steamTwins() = saved; }
};

class SteamTransport : public ILinkTransport {
public:
    static constexpr uint32_t kUnreliableMax = 1200;       // a bigger message goes reliable (Steam's unreliable limit)
    static constexpr uint32_t kWholeMax = 512 * 1024;      // a bigger message is sent in pieces of this size
    static constexpr int kMaxPacketsPerRound = 256;
    static constexpr size_t kMaxBytesPerRound = 8u << 20;
    static constexpr size_t kMaxMessage = 8u << 20;        // the same cap as the wire codec (kMaxMessageBytes)
    static constexpr int kChannel = 0;
    static constexpr uint32_t kConnectMagic = 0x4F43564E;  // "NVCO": a connect from co-op, not something else on the channel
    static constexpr uint32_t kPausedMs = 2000;            // a longer gap between two polls: our game was not running (a loading screen)

    explicit SteamTransport(ISteamApi& steam) : steam_(steam) {}
    ~SteamTransport() override { stop(); }
    SteamTransport(const SteamTransport&) = delete;
    SteamTransport& operator=(const SteamTransport&) = delete;

    bool listen(size_t maxPeers = 8);          // Steam friends who connect become peers 1, 2, ...
    bool connect(SteamId host);                // the host is peer 0; asynchronous, watch for the Connected event
    void setTimeouts(uint32_t silentMs, uint32_t connectMs) { silentMs_ = silentMs; connectMs_ = connectMs; }
    void stop();                               // tells every peer "bye"

    void send(PeerId to, Bytes data) override { sendEx(to, std::move(data), Channel::Reliable); }
    void sendEx(PeerId to, Bytes data, Channel ch) override;
    bool poll(PeerId& from, Bytes& data) override;
    size_t backlogBytes(PeerId to) const override;

    bool nextEvent(Event& e) override;         // Connected / Disconnected, filled by poll()
    bool reconnect() override;                 // client: say "connect" to the same host again
    void disconnect() override { stop(); }
    bool connected() const override { return !peers_.empty(); }

    bool isServer() const { return server_; }
    bool running() const { return running_; }
    SteamId hostId() const { return host_; }
    uint32_t rttMs(PeerId peer) const;
    SteamId steamIdOf(PeerId peer) const;
    const std::string& lastError() const { return lastError_; }   // why the last connect did not work, for the player
    struct Stats { uint64_t packetsIn = 0, packetsOut = 0, bytesIn = 0, bytesOut = 0, refused = 0, dropped = 0, pieces = 0; };
    const Stats& stats() const { return stats_; }

    static std::string name(SteamId id) { return "steam:" + std::to_string(id); }

private:
    struct Peer {
        SteamId id = 0;
        uint32_t lastHeard = 0;
        uint32_t rtt = 0;
        Bytes partial;         // a message arriving in pieces
        bool inPieces = false;
    };
    void pump(uint32_t now);
    void control(SteamId from, const Bytes& packet, uint32_t now);
    void sendControl(SteamId to, uint8_t kind, uint32_t value, bool reliable);
    void dropPeer(PeerId peer, bool event);
    PeerId peerOf(SteamId id) const;

    ISteamApi& steam_;
    bool running_ = false;
    bool server_ = false;
    size_t maxPeers_ = 0;
    SteamId host_ = 0;                  // client: who we join (kept for reconnect())
    bool connecting_ = false;           // client: waiting for the host's "accepted"
    uint32_t connectSince_ = 0, nextConnect_ = 0, nextKeepAlive_ = 0, lastPump_ = 0;
    PeerId nextPeer_ = 1;
    std::map<PeerId, Peer> peers_;
    std::deque<std::pair<PeerId, Bytes>> inbox_;
    std::deque<Event> events_;
    uint32_t silentMs_ = 60000, connectMs_ = 20000;
    bool looked_ = false;               // this round of poll() calls has looked at Steam already
    std::string lastError_;
    Stats stats_;
};

}  // namespace fnvcoop
