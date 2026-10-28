// plugin/areadigest tests (game-free), part 2: host + client AreaServices over real MuxTransports and the in-memory network, with fake actor
// facades, session shadows and a real flight recorder. Twins 76..79 (runtime), 80 and 81 (flight link) must each make one of these fail.
#include <algorithm>
#include <cstdio>
#include <filesystem>
#include <string>
#include "areadigest/ad_wire.h"
#include "flight/flight.h"
#include "plugin_rig.h"

using namespace nvat;
namespace fl = fnvcoop::flight;

namespace {

constexpr CellId kCell = 0x100;

std::map<std::string, std::string> metaOf(const fl::Trace& t) {
    std::map<std::string, std::string> m;
    for (const auto& r : t.records) {
        if (r.type != fl::kMeta) continue;
        size_t i = 0;
        auto str = [&]() -> std::string {
            if (i + 2 > r.payload.size()) return {};
            const size_t n = r.payload[i] | (r.payload[i + 1] << 8);
            i += 2;
            if (i + n > r.payload.size()) return {};
            std::string s(reinterpret_cast<const char*>(r.payload.data() + i), n);
            i += n;
            return s;
        };
        while (i < r.payload.size()) {
            std::string k = str();
            std::string v = str();
            m[k] = v;
        }
    }
    return m;
}

std::vector<std::string> healMarkers(const fl::Trace& t) {
    std::vector<std::string> out;
    for (const auto& r : t.records) {
        if (r.type != fl::kMarker || r.payload.size() < 4) continue;
        const uint16_t kind = static_cast<uint16_t>(r.payload[0] | (r.payload[1] << 8));
        if (kind != fl::kMarkHeal) continue;
        const size_t n = r.payload[2] | (r.payload[3] << 8);
        if (4 + n <= r.payload.size()) out.emplace_back(reinterpret_cast<const char*>(r.payload.data() + 4), n);
    }
    return out;
}

bool dumpAndRead(fl::FlightRecorder& rec, fl::Trace& t) {
    const std::string path = (std::filesystem::temp_directory_path() / "areadigest_plugin_flight.nvfr").string();
    if (!rec.dump(path, fl::kDumpTest)) return false;
    std::string err;
    const bool ok = fl::readTrace(path, t, &err);
    std::remove(path.c_str());
    return ok;
}

fl::Config recCfg(const char* label) {
    fl::Config c;
    c.ringBytes = 1 << 20;
    c.label = label;
    return c;
}

}  // namespace

// ---- the whole chain: facade -> source -> AreaSync -> mux -> network -> mux -> AreaSync -> source -> facade ------------------------------
TEST(service, a_wrong_health_is_found_and_set_through_the_facade) {
    Pair p;
    p.actor(5, kCell, V(0, 0, 0), 100);
    p.run(4000);
    CHECK(p.client->sync()->healsTotalAll() == 0 && p.host->sync()->healsTotalAll() == 0);
    p.cf.a[5].r.hp = 30;
    p.run(9000);
    CHECK(p.cf.a[5].r.hp == 100 && p.cf.calls.size() == 1 && p.cf.calls[0] == "hp 5 100");
    CHECK(p.client->sync()->healsTotal(ad::Cat::Actors) == 1 && p.host->sync()->healsTotalAll() == 0);
    CHECK(p.client->sync()->healsPerMinute(ad::Cat::Actors, p.now) == 1);
    CHECK(p.hf.calls.empty());
    p.run(20000);
    CHECK(p.client->sync()->healsTotalAll() == 1);                  // and it stays that way
}

TEST(service, a_position_error_is_smoothed_by_the_per_frame_ticks) {
    Pair p;
    p.actor(5, kCell, V(0, 0, 0), 100);
    p.run(4000);
    p.cf.a[5].r.pos = V(4 * kU, 0, 0);
    p.run(7000);
    int posWrites = 0;
    for (const auto& c : p.cf.calls) posWrites += c == "pos 5";
    CHECK(posWrites >= 5);                                           // several small steps, not one jump
    p.run(2000);
    CHECK(p.cf.a[5].r.pos.x == 0 && p.client->actorSource()->smoothing() == 0);
    CHECK(p.client->sync()->healsTotal(ad::Cat::Actors) == 1);
}

TEST(service, a_client_that_owns_the_actor_pushes_to_the_host_not_the_other_way) {
    Pair p;
    p.actor(5, kCell, V(0, 0, 0), 100, /*owner*/ 1);
    p.run(4000);
    p.hf.a[5].r.hp = 20;                                            // the host's copy is wrong, the client is the owner
    p.run(9000);
    CHECK(p.hf.a[5].r.hp == 100 && p.cf.calls.empty());
    CHECK(p.host->sync()->healsTotal(ad::Cat::Actors) == 1 && p.client->sync()->healsTotalAll() == 0);
}

TEST(service, a_death_that_never_arrived_asks_the_actor_module_for_the_owners_state) {
    Pair p;
    p.actor(5, kCell, V(0, 0, 0), 100);
    p.run(4000);
    p.hf.a[5].r.alive = false;
    p.hf.a[5].r.hp = 0;
    p.run(9000);
    CHECK(p.cf.resyncs.size() >= 1 && p.cf.resyncs[0] == 5);
    CHECK(p.cf.a[5].r.alive);                                        // never killed from here
    CHECK(p.client->sync()->healsTotal(ad::Cat::Actors) >= 1);       // a delegated heal still counts: the desync was real
}

TEST(service, the_story_digest_asks_the_existing_repair_and_its_repairs_are_the_story_heals) {
    Pair p;
    p.hw.stage(0x100, 10);
    p.cw.stage(0x100, 20);                                           // same applied seq, different value
    p.run(7000);
    REQUIRE(!p.hw.repairAsked.empty());
    CHECK(p.hw.repairAsked[0] == 1);
    CHECK(p.host->sync()->counters().storyMismatch >= 1);
    CHECK(p.host->sync()->healsTotal(ad::Cat::Story) == 0);          // asking is not healing
    p.hw.repairs = 1;
    p.cw.st = p.hw.st;
    p.cw.seq = p.hw.seq;
    p.run(3000);
    CHECK(p.host->sync()->healsTotal(ad::Cat::Story) == 1);
}

TEST(service, a_container_whose_contents_differ_goes_to_the_story_layer) {
    AreaSettings s;
    s.story = false;                                                 // isolate the container path: the story digest would also see this drift
    s.cfg.compareStory = false;
    Pair p(s);
    p.hw.addItem(0x500, 0x10, 3);
    p.cw.addItem(0x500, 0x10, 3);
    p.run(6000);
    CHECK(p.hw.repairAsked.empty());
    p.cw.addItem(0x500, 0x11, 9);
    p.run(9000);
    CHECK(!p.hw.repairAsked.empty() && p.hw.repairAsked[0] == 1);
    CHECK(p.host->sync()->counters().cat[2].delegated >= 1 && p.host->sync()->counters().healsSent == 0);
}

// ---- runtime twins ---------------------------------------------------------------------------------------------------------------
TEST(runtime, a_client_digests_only_while_the_host_link_is_up) {
    Pair p;
    p.actor(5, kCell, V(0, 0, 0), 100);
    p.hostLinkUp = false;
    p.run(8000);
    CHECK(p.client->sync() != nullptr && p.client->sync()->counters().digestsSent == 0);    // twin 78: the link state is ignored
    p.hostLinkUp = true;
    p.run(6000);
    CHECK(p.client->sync()->counters().digestsSent >= 2);
}

TEST(runtime, a_new_session_gets_a_new_sync_on_the_new_mux) {
    Pair p;
    p.actor(5, kCell, V(0, 0, 0), 100);
    p.run(4000);
    CHECK(p.client->runtime().rebuilds() == 1);
    fnvcoop::MuxTransport mux2(*p.net.endpoint(1));                  // the session was restarted: a new mux
    p.clientMuxOverride = &mux2;
    p.run(100);
    CHECK(p.client->runtime().rebuilds() == 2);                      // twin 77: the old sync, still bound to the dead mux, is kept
    p.cf.a[5].r.hp = 30;
    p.run(12000);
    CHECK(p.cf.a[5].r.hp == 100);                                    // and the new one hears the host
    p.clientMuxOverride = nullptr;                                   // mux2 dies with this scope: do not leave a dangling pointer behind
    p.clientActive = false;
    p.run(100);
    CHECK(p.client->sync() == nullptr);                              // the session ended: everything is dropped
}

TEST(runtime, a_datagram_from_a_peer_the_host_does_not_know_is_dropped) {
    Pair p;
    p.actor(5, kCell, V(0, 0, 0), 100);
    p.run(2000);
    ad::MDigest d;
    ad::Envelope e;
    e.src = 0;
    e.body = d;
    std::vector<uint8_t> bytes = ad::encodeEnvelope(e);
    REQUIRE(!bytes.empty());
    p.net.endpoint(5)->send(0, bytes);                               // an endpoint that is no joined client
    p.run(200);
    CHECK(p.host->runtime().droppedUnknownPeer() == 1);              // twin 76: taken as the host's own datagram
    CHECK(p.host->sync()->counters().unknownPeer == 0);
}

TEST(runtime, joins_and_leaves_reach_the_sync) {
    Pair p;
    p.actor(5, kCell, V(0, 0, 0), 100);
    p.run(5000);
    CHECK(p.host->sync()->counters().digestsReceived >= 2 && p.host->sync()->counters().unknownPeer == 0);   // twin 79: nobody ever joined
    p.clientListed = false;                                          // the client left the host's list
    p.run(5000);
    CHECK(p.host->runtime().droppedUnknownPeer() >= 1);              // its digests are not compared any more
    const uint64_t before = p.host->sync()->counters().digestsReceived;
    p.clientListed = true;
    p.run(5000);
    CHECK(p.host->sync()->counters().digestsReceived > before);
}

// ---- flight recorder -----------------------------------------------------------------------------------------------------------------
TEST(flightlink, heals_and_counters_are_in_the_trace) {
    fl::FlightRecorder hostRec(recCfg("host")), clientRec(recCfg("client"));
    Pair p(AreaSettings(), &hostRec, &clientRec);
    p.actor(5, kCell, V(0, 0, 0), 100);
    p.run(4000);
    p.cf.a[5].r.hp = 30;
    p.run(9000);
    REQUIRE(p.client->sync()->healsTotal(ad::Cat::Actors) == 1);
    p.client->exportFlight(p.now);
    fl::Trace t;
    REQUIRE(dumpAndRead(clientRec, t));
    auto meta = metaOf(t);
    CHECK(meta.count("heals.actors.total") == 1 && meta["heals.actors.total"] == "1");             // twin 81: never exported
    CHECK(meta.count("heals.total") == 1 && meta["heals.total"] == "1" && meta["heals.perMin"] == "1");
    int perKey = 0;
    for (const auto& m : healMarkers(t)) {
        if (m.rfind("heal actors ", 0) == 0) {
            ++perKey;
            CHECK(m.find("hp 30") != std::string::npos && m.find("hp 100") != std::string::npos);
        }
    }
    CHECK(perKey == 1);                                              // twin 80: the heal left no marker
    CHECK(p.client->flightLink().healsRecorded() == 1);
}

TEST(flightlink, a_repeated_mismatch_is_a_bug_line_with_both_values) {
    fl::FlightRecorder hostRec(recCfg("host"));
    Pair p(AreaSettings(), &hostRec, nullptr);
    p.actor(5, kCell, V(0, 0, 0), 100);
    p.run(4000);
    p.cf.ignoreHealth = true;                                        // the engine keeps its own health
    p.cf.a[5].r.hp = 30;
    p.run(40000);
    REQUIRE(!p.host->sync()->bugs().empty());
    const auto& b = p.host->sync()->bugs()[0];
    CHECK(b.authority.find("hp 100") != std::string::npos && b.other.find("hp 30") != std::string::npos);
    CHECK(p.host->flightLink().bugsRecorded() >= 1);
    fl::Trace t;
    REQUIRE(dumpAndRead(hostRec, t));
    bool found = false;
    for (const auto& m : healMarkers(t)) found = found || (m.rfind("BUG repeated actors", 0) == 0 && m.find("hp 100") != std::string::npos && m.find("hp 30") != std::string::npos);
    CHECK(found);
}

// A failed lab room used to say only "heals.total +1": which key was healed, from what to what, was in a flight file nobody had at hand. The status names the newest
// heals (also without a recorder), so the room's failure reason can quote the key.
TEST(flightlink, the_status_names_the_healed_key_with_both_values) {
    Pair p;                                                          // no flight recorder at all
    p.actor(5, kCell, V(0, 0, 0), 100);
    p.run(4000);
    p.cf.a[5].r.hp = 30;
    p.run(9000);
    REQUIRE(p.client->sync()->healsTotal(ad::Cat::Actors) == 1);
    REQUIRE(p.client->flightLink().lastHeals().size() == 1);          // twin 82: the status never names the healed key
    const std::string& line = p.client->flightLink().lastHeals().front();
    CHECK_MSG(line.find("actors key=5 ") == 0, line);
    CHECK_MSG(line.find("hp 30") != std::string::npos && line.find("hp 100") != std::string::npos, line);
    const std::string j = p.client->statusJson(p.now);
    CHECK(j.find("\"lastHeals\":[\"actors key=5 ") != std::string::npos);
}

TEST(flightlink, no_recorder_is_fine) {
    Pair p;
    p.actor(5, kCell, V(0, 0, 0), 100);
    p.run(4000);
    p.cf.a[5].r.hp = 30;
    p.run(9000);
    p.client->exportFlight(p.now);                                   // nothing to write to: no crash, no effect
    CHECK(p.cf.a[5].r.hp == 100);
}

// ---- status -----------------------------------------------------------------------------------------------------------------------
TEST(service, status_json_describes_the_settings_the_sources_and_the_counters) {
    Pair p;
    p.actor(5, kCell, V(0, 0, 0), 100);
    p.run(4000);
    p.cf.a[5].r.hp = 30;
    p.run(9000);
    const std::string j = p.client->statusJson(p.now);
    int depth = 0;
    bool balanced = true, inString = false;
    for (size_t i = 0; i < j.size(); ++i) {
        const char ch = j[i];
        if (ch == '\\') { ++i; continue; }
        if (ch == '"') inString = !inString;
        if (inString) continue;
        if (ch == '{' || ch == '[') ++depth;
        if (ch == '}' || ch == ']') { --depth; if (depth < 0) balanced = false; }
    }
    CHECK(balanced && depth == 0);
    for (const char* k : {"\"running\":true", "\"settings\":\"enabled=1", "\"actorSource\":{", "\"hpSet\":1", "\"flight\":{", "\"heals\":{\"total\":1,\"perMin\":1",
                          "\"role\":\"client\"", "\"byCat\":{\"story\":"})
        CHECK_MSG(j.find(k) != std::string::npos, k);
    // a service that has no session yet says so
    AreaSettings s;
    s.enabled = true;
    FakeFacade f;
    FakeWorld w;
    AreaService idle(s, w.inputs(&f));
    const std::string ij = idle.statusJson(1000);
    CHECK(ij.find("\"running\":false") != std::string::npos && ij.find("\"enabled\":true") != std::string::npos);
}

TEST(service, categories_that_are_switched_off_are_not_read) {
    AreaSettings s;
    s.actors = false;
    Pair p(s);
    p.actor(5, kCell, V(0, 0, 0), 100);
    p.run(4000);
    p.cf.a[5].r.hp = 30;
    p.run(12000);
    CHECK(p.cf.a[5].r.hp == 30 && p.client->actorSource() == nullptr);   // no actor category: nothing compared, nothing healed
    CHECK(p.client->sync()->healsTotalAll() == 0);
}
