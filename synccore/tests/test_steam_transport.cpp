// SteamTransport (docs/steam.md) against a pretend Steam: connect and accept, keepalive, timeout, a loading-screen gap, ordering on one
// channel, a flood bounded per frame, only friends get in, big messages in pieces, reconnect. Group "steam". The mutation twins (group
// "steamtwin") switch one SteamTwins flag on and run the check that guards it; each twin test passes only when that check FAILS.
#include <string>
#include <vector>
#include "fnvcoop/steam_fake.h"
#include "fnvcoop/steam_transport.h"
#include "minitest.h"

using namespace fnvcoop;

namespace {
constexpr SteamId kHost = 76561190000000001ull, kA = 76561190000000002ull, kB = 76561190000000003ull, kStranger = 76561190000000009ull;

struct Got { PeerId from; Bytes data; };
using Ev = ILinkTransport::Event;

struct Rig {
    FakeSteamNet net;
    FakeSteamNet::Node& hostNode = net.player(kHost);
    FakeSteamNet::Node& aNode = net.player(kA);
    FakeSteamNet::Node& bNode = net.player(kB);
    FakeSteamNet::Node& strangerNode = net.player(kStranger);
    SteamTransport host{hostNode}, a{aNode}, b{bNode}, stranger{strangerNode};
    std::vector<Got> hostGot, aGot, bGot, strangerGot;
    std::vector<Ev> hostEv, aEv, bEv, strangerEv;

    Rig() {
        net.befriend(kHost, kA);
        net.befriend(kHost, kB);
        host.setTimeouts(5000, 4000);
        a.setTimeouts(5000, 4000);
        b.setTimeouts(5000, 4000);
        stranger.setTimeouts(5000, 4000);
    }
    static void frame(SteamTransport& t, std::vector<Got>& got, std::vector<Ev>& ev) {
        PeerId from;
        Bytes d;
        while (t.poll(from, d)) got.push_back({from, std::move(d)});
        Ev e;
        while (t.nextEvent(e)) ev.push_back(e);
    }
    // One game frame for everybody who runs, then the clock moves.
    void step(uint32_t ms = 100) {
        if (host.running()) frame(host, hostGot, hostEv);
        if (a.running()) frame(a, aGot, aEv);
        if (b.running()) frame(b, bGot, bEv);
        if (stranger.running()) frame(stranger, strangerGot, strangerEv);
        net.nowMs += ms;
    }
    void run(uint32_t totalMs, uint32_t ms = 100) { for (uint32_t t = 0; t < totalMs; t += ms) step(ms); }
    void up() {  // host listening, A connected
        host.listen(2);
        a.connect(kHost);
        run(1500);
    }
    static int count(const std::vector<Ev>& ev, Ev::Type t, PeerId p) {
        int n = 0;
        for (auto& e : ev) n += e.type == t && e.peer == p;
        return n;
    }
};

Bytes msg(uint8_t first, size_t n = 8, uint8_t fill = 0) {
    Bytes b(n, fill);
    b[0] = first;
    return b;
}
}  // namespace

TEST(steam, connect_and_accept_then_data_flows_both_ways) {
    Rig r;
    r.up();
    CHECK(r.hostNode.hosting);
    CHECK(r.host.connected() && r.a.connected());
    CHECK(Rig::count(r.aEv, Ev::Connected, 0) == 1);      // a client sees its host as peer 0
    CHECK(Rig::count(r.hostEv, Ev::Connected, 1) == 1);   // a host numbers its guests from 1
    CHECK(r.host.steamIdOf(1) == kA);
    CHECK(r.a.steamIdOf(0) == kHost);
    r.host.send(1, msg(0xFC, 20, 7));
    r.a.send(0, msg(0xD1, 12, 9));
    r.run(300);
    REQUIRE(r.aGot.size() == 1);
    REQUIRE(r.hostGot.size() == 1);
    CHECK(r.aGot[0].from == 0);
    CHECK((r.aGot[0].data.size() == 20 && r.aGot[0].data[0] == 0xFC && r.aGot[0].data[5] == 7));
    CHECK(r.hostGot[0].from == 1);
    CHECK((r.hostGot[0].data.size() == 12 && r.hostGot[0].data[0] == 0xD1));
    r.host.disconnect();
    CHECK(!r.hostNode.hosting);   // friends stop seeing the host as joinable
}

TEST(steam, a_second_friend_is_peer_two_and_peers_are_told_apart) {
    Rig r;
    r.up();
    r.b.connect(kHost);
    r.run(1500);
    CHECK(Rig::count(r.hostEv, Ev::Connected, 2) == 1);
    r.b.send(0, msg(0x11));
    r.a.send(0, msg(0x22));
    r.run(300);
    REQUIRE(r.hostGot.size() == 2);
    for (auto& g : r.hostGot) CHECK(((g.from == 2 && g.data[0] == 0x11) || (g.from == 1 && g.data[0] == 0x22)));
    r.host.send(2, msg(0x33));
    r.run(300);
    CHECK(r.bGot.size() == 1);
    CHECK(r.aGot.empty());
}

TEST(steam, a_full_party_turns_the_next_friend_away) {
    Rig r;
    r.host.listen(1);
    r.a.connect(kHost);
    r.run(1500);
    r.b.connect(kHost);
    r.run(1500);
    CHECK(r.host.connected());
    CHECK(Rig::count(r.hostEv, Ev::Connected, 1) == 1);
    CHECK(Rig::count(r.hostEv, Ev::Connected, 2) == 0);
    CHECK(!r.b.connected());
    CHECK(Rig::count(r.bEv, Ev::Disconnected, 0) >= 1);
    CHECK(r.b.lastError().find("full") != std::string::npos);
}

TEST(steam, messages_on_one_channel_keep_their_order) {
    Rig r;
    r.up();
    for (int i = 0; i < 300; ++i) {
        Bytes m(6, 0);
        m[0] = 0xFC;
        m[1] = static_cast<uint8_t>(i);
        m[2] = static_cast<uint8_t>(i >> 8);
        r.host.send(1, std::move(m));
    }
    r.run(500);
    REQUIRE(r.aGot.size() == 300);
    bool inOrder = true;
    for (int i = 0; i < 300; ++i) inOrder = inOrder && (r.aGot[i].data[1] | (r.aGot[i].data[2] << 8)) == i;
    CHECK(inOrder);
}

TEST(steam, a_goodbye_cannot_overtake_what_was_sent_before_it) {
    Rig r;
    r.up();
    for (int i = 0; i < 5; ++i) r.host.send(1, msg(0xFC, 8, static_cast<uint8_t>(i)));
    r.host.disconnect();   // sends "bye" after the five
    r.run(300);
    REQUIRE(r.aGot.size() == 5);
    for (int i = 0; i < 5; ++i) CHECK(r.aGot[i].data[3] == i);
    CHECK(Rig::count(r.aEv, Ev::Disconnected, 0) == 1);
    CHECK(!r.a.connected());
}

TEST(steam, bye_ends_the_link_at_once_not_after_the_timeout) {
    Rig r;
    r.up();
    r.a.disconnect();
    r.run(300);   // far below the 5 s silence timeout
    CHECK(Rig::count(r.hostEv, Ev::Disconnected, 1) == 1);
    CHECK(!r.host.connected());
}

TEST(steam, keepalive_holds_a_quiet_link_and_measures_the_round_trip) {
    Rig r;
    r.up();
    r.run(30000);   // 30 s of silence from the game, six times the timeout
    CHECK(r.host.connected() && r.a.connected());
    CHECK(Rig::count(r.hostEv, Ev::Disconnected, 1) == 0);
    CHECK(r.a.rttMs(0) <= 400);
    CHECK(r.host.stats().packetsOut > 20);
}

TEST(steam, a_silent_peer_is_dropped_after_the_timeout_on_both_sides) {
    Rig r;
    r.up();
    r.net.cutOff.insert(kA);
    r.run(4000);
    CHECK(r.host.connected());   // not yet: the timeout is 5 s
    r.run(3000);
    CHECK(Rig::count(r.hostEv, Ev::Disconnected, 1) == 1);
    CHECK(Rig::count(r.aEv, Ev::Disconnected, 0) == 1);
    CHECK(!r.host.connected() && !r.a.connected());
    CHECK(r.host.lastError().find("Nothing from") != std::string::npos);
}

TEST(steam, a_loading_screen_is_not_the_peers_silence) {
    Rig r;
    r.up();
    // Both games load for 20 s: neither runs its code, so nobody answers or hears anything. Nobody is gone (the timeout is 5 s).
    r.net.nowMs += 20000;
    r.run(1000);
    CHECK(r.a.connected());
    CHECK(r.host.connected());
    CHECK(Rig::count(r.aEv, Ev::Disconnected, 0) == 0);
    CHECK(Rig::count(r.hostEv, Ev::Disconnected, 1) == 0);
    // But a peer that keeps running and says nothing for 20 s is gone.
    r.net.cutOff.insert(kA);
    r.run(7000);
    CHECK(!r.host.connected());
}

TEST(steam, a_flood_is_read_in_bounded_rounds_and_nothing_is_lost) {
    Rig r;
    r.up();
    const int total = SteamTransport::kMaxPacketsPerRound * 5 + 17;
    for (int i = 0; i < total; ++i) {
        Bytes m(5, 0);
        m[0] = 0xFC;
        m[1] = static_cast<uint8_t>(i);
        m[2] = static_cast<uint8_t>(i >> 8);
        r.host.send(1, std::move(m));
    }
    // One frame of the guest: Steam must not be drained past the cap, and the rest must still be there.
    const uint64_t before = r.aNode.reads;
    Rig::frame(r.a, r.aGot, r.aEv);
    const uint64_t one = r.aNode.reads - before;
    CHECK(one <= static_cast<uint64_t>(SteamTransport::kMaxPacketsPerRound));
    CHECK(r.aNode.waiting() > 0);
    r.run(2000);
    CHECK(r.aGot.size() == static_cast<size_t>(total));
    bool inOrder = true;
    for (size_t i = 0; i < r.aGot.size(); ++i) inOrder = inOrder && static_cast<int>(r.aGot[i].data[1] | (r.aGot[i].data[2] << 8)) == static_cast<int>(i);
    CHECK(inOrder);
}

TEST(steam, only_friends_get_in) {
    Rig r;
    r.host.listen(4);
    r.stranger.connect(kHost);
    r.run(6000);
    CHECK(!r.host.connected());
    CHECK(r.hostEv.empty());
    CHECK(r.strangerEv.size() == 1);   // after its own connect timeout: Disconnected(0)
    CHECK(Rig::count(r.strangerEv, Ev::Connected, 0) == 0);
    CHECK(r.hostNode.heldFrom(kStranger) > 0);   // Steam holds the stranger's packets: the host never accepted the session
    CHECK(r.hostNode.accepted.count(kStranger) == 0);
    // A friend still gets in beside the refused stranger.
    r.a.connect(kHost);
    r.run(1500);
    CHECK(r.host.connected());
}

TEST(steam, a_friend_who_never_said_connect_cannot_inject_messages) {
    Rig r;
    r.up();
    // B is a friend and the host accepted its session, but B never connected: its data is not a peer's data.
    r.hostNode.accept(kB);
    Bytes m(10, 0);
    m[0] = 0;   // a "message" packet
    m[1] = 0xFC;
    r.bNode.send(kHost, m.data(), static_cast<uint32_t>(m.size()), true, 0);
    r.run(300);
    CHECK(r.hostGot.empty());
}

TEST(steam, the_guest_only_lets_its_own_host_through) {
    Rig r;
    r.a.connect(kHost);
    // A friend of the guest (not the host) tries to talk to it: no accept, nothing delivered.
    r.net.befriend(kA, kB);
    Bytes m(10, 1);
    r.bNode.send(kA, m.data(), static_cast<uint32_t>(m.size()), true, 0);
    r.step();
    CHECK(r.aNode.accepted.count(kB) == 0);
    CHECK(r.aGot.empty());
}

TEST(steam, a_connect_to_a_host_that_is_not_there_gives_up_with_a_reason) {
    Rig r;
    r.a.connect(kHost + 100);   // nobody has this id: the fake says "their game is not running"
    r.run(3000);
    CHECK(r.aEv.empty());   // still trying: the host may only be starting up
    r.run(3000);
    CHECK(Rig::count(r.aEv, Ev::Disconnected, 0) == 1);
    CHECK(!r.a.connected());
    CHECK(r.a.lastError().find("not running") != std::string::npos);
}

TEST(steam, a_big_message_goes_in_pieces_and_arrives_whole) {
    Rig r;
    r.up();
    Bytes big(3u << 20);
    for (size_t i = 0; i < big.size(); ++i) big[i] = static_cast<uint8_t>(i * 31 + (i >> 12));
    big[0] = 0xFC;
    r.host.send(1, big);
    r.host.send(1, msg(0xFC, 8, 4));   // and a small one behind it keeps its place
    r.run(500);
    REQUIRE(r.aGot.size() == 2);
    CHECK(r.aGot[0].data == big);
    CHECK(r.aGot[1].data.size() == 8);
    CHECK(r.host.stats().pieces >= 6);
}

TEST(steam, a_message_over_the_cap_is_not_sent_and_empty_ones_neither) {
    Rig r;
    r.up();
    const uint64_t out = r.host.stats().packetsOut;
    r.host.send(1, Bytes(SteamTransport::kMaxMessage + 1, 1));
    r.host.send(1, Bytes());
    r.host.send(99, msg(0xFC));   // nobody
    CHECK(r.host.stats().packetsOut == out);
}

TEST(steam, an_unreliable_message_over_the_steam_limit_goes_reliable) {
    Rig r;
    r.up();
    r.net.dropUnreliable = true;
    r.host.sendEx(1, Bytes(3000, 0xFC), ITransport::Channel::Unreliable);   // too big for an unreliable packet
    r.host.sendEx(1, msg(0xFC, 100), ITransport::Channel::Unreliable);      // fits: Steam may drop it (and does here)
    r.run(300);
    REQUIRE(r.aGot.size() == 1);
    CHECK(r.aGot[0].data.size() == 3000);
}

TEST(steam, the_guest_comes_back_with_reconnect_after_a_drop) {
    Rig r;
    r.up();
    r.net.cutOff.insert(kA);
    r.run(8000);
    CHECK(!r.a.connected());
    r.net.cutOff.clear();
    CHECK(r.a.reconnect());
    r.run(2500);
    CHECK(r.a.connected());
    CHECK(r.host.connected());
    r.a.send(0, msg(0xFC, 9, 3));
    PeerId again = 0;
    for (auto& e : r.hostEv) if (e.type == Ev::Connected) again = e.peer;
    CHECK(again == 2);   // a new link, a new number
    r.host.send(again, msg(0xFC, 9, 5));
    r.run(300);
    CHECK(!r.hostGot.empty());
    CHECK(!r.aGot.empty());
}

TEST(steam, backlog_comes_from_steams_send_queue) {
    Rig r;
    r.up();
    r.hostNode.queued = 123456;
    CHECK(r.host.backlogBytes(1) == 123456);
    CHECK(r.host.backlogBytes(5) == 0);
}

TEST(steam, listen_and_connect_report_what_they_can_do) {
    Rig r;
    CHECK(!r.a.connect(0));
    CHECK(!r.a.reconnect());
    CHECK(r.host.listen());
    CHECK(!r.host.reconnect());   // only a guest reconnects
}

// ---- mutation twins: each switches one bug on and runs the check that guards it; the twin passes only when that check FAILS -------------
namespace {
template <typename Fn>
bool underTwin(void (*set)(SteamTwins&), Fn&& fn) {
    SteamTwins t;
    set(t);
    SteamTwinScope s(t);
    return fn();
}
bool connectsBothWays() {
    Rig r;
    r.up();
    return r.host.connected() && r.a.connected();
}
bool quietLinkSurvives() {
    Rig r;
    r.up();
    r.run(30000);
    return r.host.connected() && r.a.connected();
}
bool silentPeerIsDropped() {
    Rig r;
    r.up();
    r.net.cutOff.insert(kA);
    r.run(7000);
    return !r.host.connected() && !r.a.connected();
}
bool loadingScreenForgiven() {
    Rig r;
    r.up();
    r.net.nowMs += 20000;
    r.run(1000);
    return r.a.connected() && r.host.connected();
}
bool floodBounded() {
    Rig r;
    r.up();
    for (int i = 0; i < SteamTransport::kMaxPacketsPerRound * 5; ++i) r.host.send(1, msg(0xFC));
    const uint64_t before = r.aNode.reads;
    Rig::frame(r.a, r.aGot, r.aEv);
    return r.aNode.reads - before <= static_cast<uint64_t>(SteamTransport::kMaxPacketsPerRound);
}
bool strangerKeptOut() {
    Rig r;
    r.host.listen(4);
    r.stranger.connect(kHost);
    r.run(3000);
    return !r.host.connected();
}
bool inOrder() {
    Rig r;
    r.up();
    for (int i = 0; i < 20; ++i) r.host.send(1, msg(0xFC, 8, static_cast<uint8_t>(i)));
    r.run(300);
    if (r.aGot.size() != 20) return false;
    for (int i = 0; i < 20; ++i) if (r.aGot[i].data[3] != i) return false;
    return true;
}
bool byeIsPrompt() {
    Rig r;
    r.up();
    r.a.disconnect();
    r.run(300);
    return !r.host.connected();
}
}  // namespace

TEST(steamtwin, the_checks_hold_without_a_twin) {
    CHECK(connectsBothWays());
    CHECK(quietLinkSurvives());
    CHECK(silentPeerIsDropped());
    CHECK(loadingScreenForgiven());
    CHECK(floodBounded());
    CHECK(strangerKeptOut());
    CHECK(inOrder());
    CHECK(byeIsPrompt());
}
TEST(steamtwin, no_keepalive_loses_a_quiet_link) { CHECK(!underTwin([](SteamTwins& t) { t.noKeepAlive = true; }, quietLinkSurvives)); }
TEST(steamtwin, no_timeout_keeps_a_dead_peer) { CHECK(!underTwin([](SteamTwins& t) { t.noTimeout = true; }, silentPeerIsDropped)); }
TEST(steamtwin, no_pause_forgiveness_loses_a_loading_guest) { CHECK(!underTwin([](SteamTwins& t) { t.noPauseForgiveness = true; }, loadingScreenForgiven)); }
TEST(steamtwin, unbounded_round_lets_a_flood_hold_the_frame) { CHECK(!underTwin([](SteamTwins& t) { t.unboundedRound = true; }, floodBounded)); }
TEST(steamtwin, accept_anyone_lets_a_stranger_in) { CHECK(!underTwin([](SteamTwins& t) { t.acceptAnyone = true; }, strangerKeptOut)); }
TEST(steamtwin, lifo_inbox_breaks_the_order) { CHECK(!underTwin([](SteamTwins& t) { t.lifoInbox = true; }, inOrder)); }
TEST(steamtwin, no_session_accept_means_nobody_connects) { CHECK(!underTwin([](SteamTwins& t) { t.noSessionAccept = true; }, connectsBothWays)); }
TEST(steamtwin, ignored_bye_leaves_a_left_player_until_the_timeout) { CHECK(!underTwin([](SteamTwins& t) { t.byeIgnored = true; }, byeIsPrompt)); }
