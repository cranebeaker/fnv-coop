// Transport decorators + ENet in a single process (two-process loopback lives in loopback.cpp / ctest loopback_*).
#include <atomic>
#include <chrono>
#include <cstdlib>
#include <thread>
#include <tuple>
#include "fnvcoop/enet_transport.h"
#include "fnvcoop/session.h"
#include "fnvcoop/transport_util.h"
#include "minitest.h"

using namespace fnvcoop;

namespace {
struct NullSink : IEffectSink {
    size_t n = 0;
    void apply(const Effect&, const Context&) override { ++n; }
};
}  // namespace

TEST(transport, mux_routes_by_first_byte_and_passes_story_through) {
    InMemoryNetwork net;
    MuxTransport mux(*net.endpoint(1));
    int dialogue = 0;
    mux.registerChannel(kChannelDialogue, [&](PeerId, Bytes&&) { ++dialogue; });
    ITransport* peer = net.endpoint(2);
    peer->send(1, Bytes{kChannelDialogue, 9});
    peer->send(1, Bytes{kChannelStory, 1, 2});
    peer->send(1, Bytes{});
    net.advance(0);
    PeerId from; Bytes d;
    CHECK(mux.poll(from, d));
    CHECK((d.size() == 3 && d[0] == kChannelStory));
    CHECK(!mux.poll(from, d));
    CHECK(dialogue == 1);
    CHECK(mux.unroutable() == 1);
}

TEST(transport, faulty_wrapper_is_seeded_and_applies_faults) {
    auto run = [](uint64_t seed) {
        InMemoryNetwork net;
        net.endpoint(2);  // receiver must exist (up) before anything is sent to it
        LinkParams p; p.loss = 0.3; p.duplicate = 0.2; p.corrupt = 0.1; p.maxLatencyMs = 30;
        FaultyTransport f(*net.endpoint(1), p, seed);
        for (int i = 0; i < 2000; ++i) f.send(2, Bytes(8, uint8_t(i)));
        for (uint64_t t = 0; t <= 100; ++t) { f.pump(t); net.advance(t); }
        size_t got = 0; PeerId from; Bytes d;
        while (net.endpoint(2)->poll(from, d)) ++got;
        return std::make_tuple(got, f.stats().lost, f.stats().duplicated, f.stats().corrupted);
    };
    auto a = run(5), b = run(5), c = run(6);
    CHECK(a == b);
    CHECK(a != c);
    auto [got, lost, dup, cor] = a;
    CHECK((lost > 400 && lost < 800));
    CHECK(dup > 100);
    CHECK(cor > 50);
    CHECK(got > 1000);
}

// The host drops the client's link (what a stall past the peer timeout looks like to the client): the client re-creates its
// transport, reconnects with a new session id and resumes from its shadow, then keeps applying the host's txns exactly once.
TEST(transport, enet_client_reconnects_after_the_link_drops_and_resumes) {
    EnetTransport server, client;
    REQUIRE(server.listen(0, 4, true));
    REQUIRE(client.connect("127.0.0.1", server.port()));
    NullSink sink;
    HostSession host(server);
    ClientSession cs(client, 0, 43, sink);
    auto t0 = std::chrono::steady_clock::now();
    auto ms = [&] { return uint64_t(std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::steady_clock::now() - t0).count()); };
    auto commit = [&](uint32_t i) {
        host.commit(0, Cause{CauseKind::QuestResult, FormRef("falloutnv.esm", 1), 0}, {SetStage{FormRef("falloutnv.esm", 1), i}});
    };
    int phase = 0;  // 0 joining, 1 dropped, 2 reconnected
    bool started = false, sawDrop = false;
    uint32_t committed = 0;
    uint64_t appliedAtDrop = 0;
    while (ms() < 20000) {
        EnetTransport::Event ev;
        while (client.nextEvent(ev)) {
            if (ev.type == EnetTransport::Event::Connected && !started) { started = true; cs.connect(ms(), 1); }
            if (ev.type == EnetTransport::Event::Disconnected) { sawDrop = true; cs.disconnect(); }
        }
        EnetTransport::Event sev;
        while (server.nextEvent(sev))
            if (sev.type == EnetTransport::Event::Disconnected) host.peerDisconnected(sev.peer);
        host.tick(ms());
        cs.tick(ms());
        if (phase == 0 && cs.joined() && committed < 10) commit(committed++);
        if (phase == 0 && committed == 10 && cs.applied() == 10) { server.disconnect(); phase = 1; }
        if (phase == 1 && sawDrop) {
            appliedAtDrop = cs.applied();
            for (uint32_t i = 0; i < 5; ++i) commit(committed++);          // the host moves on while the client is away
            CHECK(client.reconnect());
            cs.connect(ms(), cs.lastSession() + 1);
            phase = 2;
        }
        if (phase == 2 && cs.joined() && committed < 20) commit(committed++);
        if (phase == 2 && committed == 20 && cs.applied() == 20) break;
        std::this_thread::sleep_for(std::chrono::milliseconds(1));
    }
    CHECK(sawDrop);
    CHECK(appliedAtDrop == 10);           // nothing arrived while the link was down
    CHECK(cs.applied() == 20);            // after the reconnect: the missed 5 and the next 5, each once
    CHECK(cs.state() == host.state());
    CHECK(client.connected());
}

// A sender that outruns the link must not grow ENet's queue without limit (2026-11-17: a 126 KB Welcome every frame ran the host out of
// memory): past the cap the peer is dropped, reported as Disconnected, and the queued packets are freed.
TEST(transport, enet_drops_a_peer_whose_send_queue_passes_the_cap) {
    EnetTransport server, client;
    REQUIRE(server.listen(0, 4, true));
    REQUIRE(client.connect("127.0.0.1", server.port()));
    server.setMaxBacklogBytes(1u << 20);
    auto t0 = std::chrono::steady_clock::now();
    auto ms = [&] { return std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::steady_clock::now() - t0).count(); };
    bool up = false;
    PeerId peer = 0;
    while (ms() < 5000 && !(up && server.connected())) {
        EnetTransport::Event ev;
        while (client.nextEvent(ev)) if (ev.type == EnetTransport::Event::Connected) up = true;
        while (server.nextEvent(ev)) if (ev.type == EnetTransport::Event::Connected) peer = ev.peer;
        PeerId from; Bytes d;
        server.poll(from, d); client.poll(from, d);
        std::this_thread::sleep_for(std::chrono::milliseconds(1));
    }
    REQUIRE(up && server.connected() && peer != 0);
    Bytes big(64 * 1024, 0xAB);
    for (int i = 0; i < 100 && server.connected(); ++i) server.send(peer, big);   // 6.4 MB, the client never reads
    CHECK(server.stats().backlogDrops == 1);
    CHECK(!server.connected());
    CHECK(server.stats().maxBacklog > (1u << 20));
    CHECK(server.stats().bytesOutByMux[0xAB] >= (1u << 20));
    bool reported = false;
    EnetTransport::Event ev;
    while (server.nextEvent(ev)) if (ev.type == EnetTransport::Event::Disconnected && ev.peer == peer) reported = true;
    CHECK(reported);
    CHECK(server.backlogBytes(peer) == 0);   // the queue is gone
}

namespace {
std::atomic<long long> g_testEnetBytes{0};
std::atomic<int> g_testEnetCalls{0};
void* testEnetMalloc(size_t n) { ++g_testEnetCalls; g_testEnetBytes += (long long)n; return std::malloc(n); }
void testEnetFree(void* p) { std::free(p); }
}  // namespace

// The plugin counts ENet's memory through these callbacks (the "network buffers" figure in nvcoop.log).
TEST(transport, enet_runs_on_the_allocator_it_was_given) {
    EnetTransport::setAllocator(&testEnetMalloc, &testEnetFree);
    {
        EnetTransport server, client;
        REQUIRE(server.listen(0, 4, true));
        REQUIRE(client.connect("127.0.0.1", server.port()));
        auto t0 = std::chrono::steady_clock::now();
        bool got = false;
        while (!got && std::chrono::steady_clock::now() - t0 < std::chrono::seconds(5)) {
            EnetTransport::Event ev;
            while (server.nextEvent(ev)) if (ev.type == EnetTransport::Event::Connected) server.send(ev.peer, Bytes(1000, 7));
            while (client.nextEvent(ev)) {}
            PeerId from; Bytes d;
            if (client.poll(from, d) && d.size() == 1000) got = true;
            server.poll(from, d);
            std::this_thread::sleep_for(std::chrono::milliseconds(1));
        }
        CHECK(got);
        CHECK(g_testEnetCalls.load() > 0);
        CHECK(g_testEnetBytes.load() >= 1000);
    }
    EnetTransport::setAllocator(nullptr, nullptr);   // back to ENet's own for the tests that follow
}

TEST(transport, enet_peer_timeouts_tolerate_a_long_load) {
    // A loading screen can block the game's main loop (which services the socket) for tens of seconds.
    CHECK(EnetTransport::kPeerTimeoutMinMs >= 60000);
    CHECK(EnetTransport::kPeerTimeoutMaxMs >= EnetTransport::kPeerTimeoutMinMs);
}

TEST(transport, enet_in_process_session_converges_on_both_channels) {
    for (int unreliable = 0; unreliable < 2; ++unreliable) {
        EnetTransport server, client;
        REQUIRE(server.listen(0, 4, true));
        REQUIRE(client.connect("127.0.0.1", server.port()));
        NullSink sink;
        HostSession host(server);
        ClientSession cs(client, 0, 42, sink);
        auto t0 = std::chrono::steady_clock::now();
        auto ms = [&] { return uint64_t(std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::steady_clock::now() - t0).count()); };
        bool started = false;
        int committed = 0;
        while (ms() < 8000) {
            EnetTransport::Event ev;
            while (client.nextEvent(ev)) if (ev.type == EnetTransport::Event::Connected && !started) { started = true; cs.connect(ms(), 1); }
            host.tick(ms());
            cs.tick(ms());
            if (cs.joined() && committed < 50) {
                host.commit(0, Cause{CauseKind::QuestResult, FormRef("falloutnv.esm", 1), 0},
                            {SetStage{FormRef("falloutnv.esm", 1), uint32_t(committed)}, AddItem{FormRef("falloutnv.esm", 5), 1, Recipient::dflt()}});
                ++committed;
            }
            if (committed == 50 && cs.applied() == 50) break;
            std::this_thread::sleep_for(std::chrono::milliseconds(1));
        }
        CHECK(cs.applied() == 50);
        CHECK(cs.state() == host.state());
        CHECK(sink.n >= 50);
        // explicit channel send reaches the peer too
        client.sendEx(0, Bytes{kChannelDialogue, 7}, unreliable ? ITransport::Channel::Unreliable : ITransport::Channel::Reliable);
        bool got = false;
        for (int i = 0; i < 500 && !got; ++i) {
            PeerId f; Bytes d;
            while (server.poll(f, d)) if (d.size() == 2 && d[0] == kChannelDialogue) got = true;
            std::this_thread::sleep_for(std::chrono::milliseconds(1));
        }
        CHECK(got);
        (void)unreliable;
    }
}
