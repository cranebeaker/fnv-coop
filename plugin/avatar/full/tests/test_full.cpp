// Player avatars, whole chain (FULL-SYNC-PLAN 3.1, roadmap step 3): three machines on a lossy in-memory link, each with the REAL
// AvatarRuntime on a FakeGameBackend, the REAL AvatarNet (live hub, Roster / Look / snapshot streams) and the REAL live codec.
//   * 20 Hz near / 5 Hz far / 1 Hz elsewhere, the uplink follows the highest tier, the last position is never lost behind the rate
//   * appearance + equipment + weapon mods travel, apply in place, and respawn the puppet only for another template
//   * lifecycle: join, leave, rejoin, cell change, interior, fast travel across worldspaces, host load, leave and come back
//   * the ghost-ref map (avatar_api.h) stays honest through all of it
//   * a hit on a puppet reaches the real player's machine through the combat API (HitValidator / applyHit), exactly once
//   * the Look v2 wire format
// Built once with FNVCOOP_RUNTIME_MUT; run once for real and once per mutation twin (FNVCOOP_MUT=<n>, tools/mut/run_twins.py):
//   101..104 avatar_wire.cpp   105..110 avatar_net.cpp
// Everything here is a model of the game and the network. It proves the logic, not that the engine accepts the calls.
#include <algorithm>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <functional>
#include <map>
#include <memory>
#include <set>
#include <string>
#include <tuple>
#include <vector>

#include "avatar_api.h"
#include "common/minijson.h"
#include "avatar_net.h"
#include "avatar_runtime_side.h"
#include "avatar_wire.h"
#include "combat.h"
#include "live_hub.h"
#include "puppet_hits.h"
#include "stream_rate.h"
#include "testing/fake_backend.h"

using namespace nvcoop::livewire;
namespace live = fnvcoop::live;
namespace av = nvcoop::avatar;
namespace cc = fnvcoop::combat;

static int g_fail = 0, g_checks = 0;
static const char* g_test = "";
#define CHECK(c)                                                                      \
    do {                                                                              \
        ++g_checks;                                                                   \
        if (!(c)) {                                                                   \
            ++g_fail;                                                                 \
            std::printf("  FAIL [%s] %s:%d  %s\n", g_test, __FILE__, __LINE__, #c);  \
        }                                                                             \
    } while (0)
static bool Verbose() {
    static const bool v = std::getenv("AVATAR_FULL_VERBOSE") != nullptr;
    return v;
}
#define NOTE(...)                           do {                                        if (Verbose()) {                            std::printf("    ");                    std::printf(__VA_ARGS__);               std::printf("\n");                  }                                   } while (0)
#define CHECK_NEAR(a, b, eps) CHECK(std::fabs(static_cast<double>(a) - static_cast<double>(b)) <= (eps))

namespace {

using nvcoop::avatar::engine::AvatarRuntime;
using nvcoop::avatar::engine::FakeGameBackend;
using nvcoop::avatar::engine::RuntimeConfig;

constexpr uint32_t kTplM = 0x1002, kTplF = 0x1003;   // male / female template NPC of race 1
constexpr uint32_t kArmor = 0x2000, kHat = 0x2001, kGun = 0x3000, kGun2 = 0x3001;
constexpr av::CellId kExtA = FakeGameBackend::kExtA, kExtB = FakeGameBackend::kExtB, kInt = FakeGameBackend::kInterior,
                     kOtherWorld = FakeGameBackend::kOtherWorldExt;

// ---- the side the AvatarNet talks to, with a counter in front of it --------------------------------------------------------------
struct CountingSide : IAvatarSide {
    IAvatarSide& in;
    std::map<av::Slot, std::vector<uint32_t>> arrivals;                // slot -> arrival times (ms) of accepted remote snapshots
    std::map<av::Slot, live::ActorSnapshot> last;                      // slot -> newest snapshot handed to the engine side
    unsigned addRemoteCalls = 0, appearanceCalls = 0, removeCalls = 0;
    explicit CountingSide(IAvatarSide& i) : in(i) {}
    void setSnapshotSink(SnapshotSink s) override { in.setSnapshotSink(std::move(s)); }
    void setEquipSink(EquipSink s) override { in.setEquipSink(std::move(s)); }
    void setLocalIdentity(live::NetId n, live::OwnerEpoch e) override { in.setLocalIdentity(n, e); }
    void setLocalSlot(av::Slot sl) override { in.setLocalSlot(sl); }
    bool addRemote(av::Slot s, const av::Appearance& a) override { ++addRemoteCalls; return in.addRemote(s, a); }
    void removeRemote(av::Slot s) override { ++removeCalls; in.removeRemote(s); }
    bool setRemoteAppearance(av::Slot s, const av::Appearance& a) override { ++appearanceCalls; return in.setRemoteAppearance(s, a); }
    bool readLocalAppearance(av::Appearance& out) override { return in.readLocalAppearance(out); }
    void onRemoteSnapshot(av::Slot s, const live::ActorSnapshot& snap, uint32_t arrivalMs) override {
        arrivals[s].push_back(arrivalMs);
        last[s] = snap;
        in.onRemoteSnapshot(s, snap, arrivalMs);
    }
    void onRemoteEquipment(av::Slot s, const std::vector<av::EquipEntry>& w) override { in.onRemoteEquipment(s, w); }
    void tick(uint32_t nowMs) override { in.tick(nowMs); }
    void onEvent(Event e) override { in.onEvent(e); }
    bool purgePending() const override { return in.purgePending(); }
    av::RefId refOf(av::Slot s) const override { return in.refOf(s); }
    bool slotOf(av::RefId r, av::Slot& s) const override { return in.slotOf(r, s); }
    std::string statusJson() override { return in.statusJson(); }
    size_t countBetween(av::Slot s, uint32_t from, uint32_t to) const {
        auto it = arrivals.find(s);
        if (it == arrivals.end()) return 0;
        size_t n = 0;
        for (uint32_t t : it->second)
            if (t >= from && t < to) ++n;
        return n;
    }
};

// ---- network and machines ----------------------------------------------------------------------------------------------------------
struct Net3 {
    fnvcoop::InMemoryNetwork net{7};
    std::map<fnvcoop::PeerId, std::unique_ptr<fnvcoop::MuxTransport>> mux;
    uint64_t now = 0;
    fnvcoop::MuxTransport* m(fnvcoop::PeerId id) {
        auto& p = mux[id];
        if (!p) p.reset(new fnvcoop::MuxTransport(*net.endpoint(id)));
        return p.get();
    }
    void pump() {
        net.advance(now);
        for (auto& kv : mux) { fnvcoop::PeerId f; fnvcoop::Bytes d; while (kv.second->poll(f, d)) {} }
    }
};

SessionView hostView(fnvcoop::MuxTransport* mux, std::vector<ClientLink> clients, uint64_t now) {
    SessionView v;
    v.active = true; v.isHost = true; v.self = kHostPlayer; v.mux = mux; v.clients = std::move(clients); v.nowMs = now;
    return v;
}
SessionView clientView(fnvcoop::MuxTransport* mux, PlayerId self, bool up, uint64_t now) {
    SessionView v;
    v.active = true; v.isHost = false; v.self = self; v.hostLinkUp = up; v.mux = mux; v.nowMs = now;
    return v;
}
ClientLink cl(PlayerId slot, fnvcoop::PeerId peer, bool joined) { ClientLink c; c.slot = slot; c.peer = peer; c.joined = joined; return c; }

const std::vector<av::CellId> kAllCells = {kExtA, kExtB, kInt, FakeGameBackend::kOtherInterior, kOtherWorld};
bool IsExterior(av::CellId c) { return c == kExtA || c == kExtB || c == kOtherWorld; }

struct Node {
    fnvcoop::PeerId peer = 0;
    PlayerId slot = 0;
    bool host = false;
    bool present = true;      // connected to the session
    bool linkUp = true;
    fnvcoop::MuxTransport* mux = nullptr;
    FakeGameBackend be;
    std::unique_ptr<AvatarRuntime> rt;
    std::unique_ptr<RuntimeSide> inner;
    std::unique_ptr<CountingSide> side;
    std::unique_ptr<AvatarNet> an;
    LiveHub hub;
    uint32_t epoch = 1000;
    std::vector<std::tuple<PlayerId, bool, av::CellId, live::Vec3>> poses;
    float vx = 0;             // the local player walks along +x at this speed (units/s)

    Node(fnvcoop::PeerId p, PlayerId s, bool h, fnvcoop::MuxTransport* m) : peer(p), slot(s), host(h), mux(m) {
        be.knownBases = {kTplM, kTplF};
        be.vanishAfterFrames = 2;
        be.weaponItems = {kGun, kGun2};
        be.slotOf = {{kArmor, 1}, {kHat, 2}};
        RuntimeConfig rc;
        rc.puppet.templates = {{1, false, kTplM}, {1, true, kTplF}};
        rc.puppet.defaultTemplate = kTplM;
        // localSlot stays at the ini default 0, exactly as the shipped module has it: the session start (AvatarNet::sessionBegin) must tell the runtime its slot
        rt.reset(new AvatarRuntime(be, rc));
        inner.reset(new RuntimeSide(*rt));
        side.reset(new CountingSide(*inner));
        an.reset(new AvatarNet(*side, AvatarNetOptions{}, [this] { return epoch++; }));
        an->setPoseSink([this](PlayerId sl, bool pr, live::CellId c, const live::Vec3& pos) { poses.emplace_back(sl, pr, c, pos); });
        hub.add(an.get(), true);
        be.Player().pos = {1000.0f * (slot + 1), 0, 0};
        be.Player().cell = kExtA;
        av::Appearance me;
        me.race = 1;
        me.name = "Courier" + std::to_string(slot);
        be.baseAppearance[7] = me;   // the fake player's base form (id 7): what readLocalAppearance finds
    }
    // Every actor that exists in this machine's world and is not the player.
    int actors() {
        int n = 0;
        for (av::CellId c : kAllCells) {
            std::vector<nvcoop::avatar::engine::ActorEntry> v;
            be.ListActors(c, v);
            for (auto& e : v) {
                auto* f = be.Get(e.ref);
                if (e.ref != nvcoop::avatar::engine::kPlayerRef && f && f->exists && !f->deleted) ++n;
            }
        }
        return n;
    }
    FakeGameBackend::FRef* puppet(PlayerId s) {
        av::RefId r = an->puppetRef(s);
        auto* f = r ? be.Get(r) : nullptr;
        return f && f->exists ? f : nullptr;
    }
    // Which cells are loaded around this machine's player (the fake needs to be told what the engine would load).
    void refreshLoaded() {
        const av::CellId pc = be.Player().cell;
        for (av::CellId c : kAllCells) {
            bool loaded = c == pc || ((pc == kExtA || pc == kExtB) && (c == kExtA || c == kExtB));
            const bool interior = !IsExterior(c);
            be.SetCell(c, loaded, interior, interior ? 0 : (c == kOtherWorld ? FakeGameBackend::kWorld + 1 : FakeGameBackend::kWorld));
        }
    }
};

struct Rig {
    Net3 n;
    std::vector<std::unique_ptr<Node>> nodes;   // [0] host (peer 0), then clients (peer = slot)
    bool realCells = false;                     // true: each machine loads only the cells around its own player
    explicit Rig(bool cellsLikeTheEngine = false) : realCells(cellsLikeTheEngine) {
        fnvcoop::LinkParams lp;
        lp.minLatencyMs = 20;
        lp.maxLatencyMs = 50;   // a decent connection: 20 to 50 ms one way, with reordering
        n.net.setLink(lp);
        nodes.emplace_back(new Node(0, 0, true, n.m(0)));
    }
    Node& host() { return *nodes[0]; }
    Node& addClient(PlayerId slot) {
        nodes.emplace_back(new Node(slot, slot, false, n.m(slot)));
        return *nodes.back();
    }
    void step(uint64_t dtMs = 16) {
        n.now += dtMs;
        n.pump();
        std::vector<ClientLink> links;
        for (auto& p : nodes)
            if (!p->host) links.push_back(cl(p->slot, p->peer, p->present));
        for (auto& p : nodes) {
            if (p->vx != 0) p->be.Player().pos.x += p->vx * float(dtMs) / 1000.0f;
            if (realCells) p->refreshLoaded();
            SessionView v;
            if (p->host) {
                v = hostView(p->mux, links, n.now);
            } else {
                v = clientView(p->mux, p->slot, p->linkUp, n.now);
                v.active = p->present;
            }
            p->hub.frame(v);
            p->be.Frame();
        }
    }
    void run(uint64_t ms) { for (uint64_t t = 0; t < ms; t += 16) step(); }
};

float PuppetX(Node& viewer, PlayerId slot, bool* ok = nullptr) {
    auto* f = viewer.puppet(slot);
    if (ok) *ok = f != nullptr;
    return f ? f->pos.x : -1e9f;
}
float PlayerX(Node& n) { return n.be.Player().pos.x; }

// ---- invariants checked at the end of lifecycle tests ------------------------------------------------------------------------------
void CheckHealthy(Rig& r, const char* when) {
    for (auto& p : r.nodes) {
        const auto es = p->rt->engine().stats();
        if (!p->be.badLines.empty()) { std::printf("  [%s] node %d bad console lines, first: %s\n", when, (int)p->slot, p->be.badLines[0].c_str()); }
        CHECK(p->be.badLines.empty());
        CHECK(es.foreignRefused == 0);
        CHECK(es.linesException == 0);
        CHECK(es.spawnAmbiguous == 0 && es.spawnRejected == 0);
        // the ghost-ref map is a bijection over what exists
        std::set<av::RefId> seen;
        for (PlayerId s : p->an->roster()) {
            av::RefId ref = p->an->puppetRef(s);
            if (!ref) continue;
            PlayerId back = 99;
            CHECK(p->an->slotOfPuppetRef(ref, back) && back == s);
            CHECK(seen.insert(ref).second);
        }
        // nothing in the world is a puppet the map does not know: no orphans, no duplicates
        CHECK(p->actors() == static_cast<int>(seen.size()));
    }
}

// ============================================================================================================================
// stream rates
// ============================================================================================================================
void rates_near_far_and_out() {
    g_test = "rates_near_far_and_out";
    Rig r;
    Node& h = r.host();
    Node& a = r.addClient(1);
    Node& b = r.addClient(2);
    h.vx = a.vx = b.vx = 60;                   // everybody walks, so no stream is idle-skipped
    a.be.Player().pos.x = 2000;                // 1000 units (14 m) from the host: near
    b.be.Player().pos.x = 30000;               // 28000 units away from both: far
    r.run(2500);
    const uint32_t t0 = static_cast<uint32_t>(r.n.now);
    r.run(6000);
    const uint32_t t1 = static_cast<uint32_t>(r.n.now);
    auto cnt = [&](Node& viewer, av::Slot s) { return viewer.side->countBetween(s, t0, t1); };
    NOTE("rates over 6 s: host<-A %zu host<-B %zu A<-host %zu A<-B %zu B<-host %zu B<-A %zu", cnt(h, 1), cnt(h, 2), cnt(a, 0), cnt(a, 2), cnt(b, 0), cnt(b, 1));
    // near: every snapshot (20 Hz -> about 120 in 6 s); far: 5 Hz -> about 30
    CHECK(cnt(h, 1) >= 100 && cnt(h, 1) <= 135);   // A -> host: A's observers are the host (near) and B (far): the highest tier wins
    CHECK(cnt(a, 0) >= 100 && cnt(a, 0) <= 135);   // host -> A: near
    CHECK(cnt(h, 2) >= 20 && cnt(h, 2) <= 38);     // B -> host: everybody is far from B
    CHECK(cnt(a, 2) >= 20 && cnt(a, 2) <= 38);     // B -> A through the host's relay: far
    CHECK(cnt(b, 0) >= 20 && cnt(b, 0) <= 38);     // host -> B: far
    CHECK(cnt(b, 1) >= 20 && cnt(b, 1) <= 38);     // A -> B: far
    CHECK(h.an->tierOf(0, 1) == av::StreamTier::Near && h.an->tierOf(0, 2) == av::StreamTier::Far);
    // the far puppets still follow (a 5 Hz stream behind a deeper buffer)
    bool ok = false;
    CHECK_NEAR(PuppetX(a, 2, &ok), PlayerX(b), 400.0f);
    CHECK(ok);
    CHECK_NEAR(PuppetX(h, 1, &ok), PlayerX(a), 150.0f);
    CHECK(h.an->stats().rateKept > 0);
}

void uplink_follows_the_highest_tier() {
    g_test = "uplink_follows_the_highest_tier";
    Rig r;
    Node& h = r.host();
    Node& a = r.addClient(1);
    Node& b = r.addClient(2);
    h.vx = a.vx = b.vx = 60;
    a.be.Player().pos.x = 30000;               // A and B stand together, far from the host
    b.be.Player().pos.x = 31000;
    r.run(2500);
    const uint32_t t0 = static_cast<uint32_t>(r.n.now);
    r.run(6000);
    const uint32_t t1 = static_cast<uint32_t>(r.n.now);
    NOTE("uplink: B<-A %zu host<-A %zu A<-B %zu A<-host %zu B<-host %zu", b.side->countBetween(1, t0, t1), h.side->countBetween(1, t0, t1),
         a.side->countBetween(2, t0, t1), a.side->countBetween(0, t0, t1), b.side->countBetween(0, t0, t1));
    // B sees A at the near rate, and for that the host must receive A at the near rate as well (it relays what it gets)
    CHECK(b.side->countBetween(1, t0, t1) >= 100);
    CHECK(h.side->countBetween(1, t0, t1) >= 100);
    CHECK(a.side->countBetween(2, t0, t1) >= 100);
    // the host is far from both: it is served at the far rate
    CHECK(a.side->countBetween(0, t0, t1) <= 38 && b.side->countBetween(0, t0, t1) <= 38);
    CHECK(a.an->tierOf(2, 1) == av::StreamTier::Near);
}

void elsewhere_trickles_and_comes_back() {
    g_test = "elsewhere_trickles_and_comes_back";
    Rig r(true);
    Node& h = r.host();
    Node& a = r.addClient(1);
    Node& b = r.addClient(2);
    h.vx = a.vx = b.vx = 60;
    b.be.Player().cell = kInt;                 // B is inside a building, the others are outside
    b.be.Player().pos = {100, 100, 0};
    b.vx = 0;
    r.run(2500);
    uint32_t t0 = static_cast<uint32_t>(r.n.now);
    r.run(6000);
    uint32_t t1 = static_cast<uint32_t>(r.n.now);
    // not silence (the ownership and spawn services want every player's place), but a trickle
    const size_t atA = a.side->countBetween(2, t0, t1);
    NOTE("elsewhere: A<-B %zu in 6 s", atA);
    CHECK(atA >= 3 && atA <= 14);
    CHECK(a.an->tierOf(1, 2) == av::StreamTier::Out);
    bool placed = false;
    for (auto& p : a.poses)
        if (std::get<0>(p) == 2 && std::get<1>(p) && std::get<2>(p) == kInt) placed = true;
    CHECK(placed);                             // A's ownership service knows B is in the building
    CHECK(a.an->puppetRef(2) == 0 && h.an->puppetRef(2) == 0);   // and there is no puppet of B in the open air (its cell is not loaded here)
    // B walks out and stands next to A: the rate goes up, the puppet appears
    b.be.Player().cell = kExtA;
    b.be.Player().pos = {2500, 0, 0};
    b.vx = 60;
    r.run(2500);
    t0 = static_cast<uint32_t>(r.n.now);
    r.run(6000);
    t1 = static_cast<uint32_t>(r.n.now);
    CHECK(a.side->countBetween(2, t0, t1) >= 100);
    bool ok = false;
    CHECK_NEAR(PuppetX(a, 2, &ok), PlayerX(b), 160.0f);
    CHECK(ok);
    CheckHealthy(r, "elsewhere");
}

void last_position_is_never_lost_behind_the_rate() {
    g_test = "last_position_is_never_lost_behind_the_rate";
    // B is far from everybody (5 Hz). When B stops, the final position must reach A soon, whatever the phase of the rate gate was.
    for (int variant = 0; variant < 8; ++variant) {
        Rig r;
        Node& a = r.addClient(1);
        Node& b = r.addClient(2);
        (void)a;
        b.be.Player().pos.x = 30000;
        b.vx = 120;
        r.run(3000 + variant * 23);
        b.vx = 0;                                                  // B stops
        r.run(900);
        const float finalX = PlayerX(b);
        auto it = a.side->last.find(2);
        CHECK(it != a.side->last.end());
        if (it != a.side->last.end()) CHECK_NEAR(it->second.pos.x, finalX, 2.0f);   // A has received B's final place (not just the interpolated puppet)
        auto ht = r.host().side->last.find(2);
        CHECK(ht != r.host().side->last.end());
        if (ht != r.host().side->last.end()) CHECK_NEAR(ht->second.pos.x, finalX, 2.0f);
    }
}

// ============================================================================================================================
// appearance, equipment, weapon mods
// ============================================================================================================================
av::Appearance BigLook() {
    av::Appearance a;
    a.race = 1;
    a.female = false;
    a.hair = 0x1234;
    a.eyes = 0x55;
    a.faceHash = 0xABCDEF01;
    a.hairColor = 0x00A0B0C0;
    a.height = 1100;
    a.weight = 950;
    for (int i = 0; i < 130; ++i) a.faceCoeffs.push_back(static_cast<int8_t>((i * 7) % 61 - 30));
    a.name = "Alpha";
    return a;
}

void look_travels_and_applies_in_place() {
    g_test = "look_travels_and_applies_in_place";
    Rig r;
    Node& h = r.host();
    Node& a = r.addClient(1);
    Node& b = r.addClient(2);
    a.vx = 30;
    a.be.baseAppearance[7] = BigLook();
    a.be.Player().worn = {kArmor, kGun};
    a.be.Player().inv[kArmor] = 1;
    a.be.Player().inv[kGun] = 1;
    a.be.Player().mods[kGun] = 0x05;
    r.run(6000);
    // everybody learned what A looks like, field by field
    for (Node* viewer : {&h, &b}) {
        const AvLook* l = viewer->an->lookOf(1);
        CHECK(l != nullptr);
        if (!l) continue;
        const av::Appearance want = BigLook();
        CHECK(l->appearance.race == 1 && l->appearance.hair == 0x1234 && l->appearance.eyes == 0x55 && l->appearance.faceHash == 0xABCDEF01);
        CHECK(l->appearance.hairColor == 0x00A0B0C0 && l->appearance.height == 1100 && l->appearance.weight == 950);
        CHECK(l->appearance.faceCoeffs == want.faceCoeffs && l->appearance.name == "Alpha");
        CHECK(l->equipment.size() == 2);
        // ... and the puppet wears it: armor, the weapon with its two modifications, the body height
        auto* f = viewer->puppet(1);
        CHECK(f != nullptr);
        if (!f) continue;
        CHECK(f->worn.count(kArmor) == 1 && f->worn.count(kGun) == 1);
        CHECK(f->mods[kGun] == 0x05);
        CHECK_NEAR(f->scale, 1.1, 1e-3);
        CHECK(viewer->rt->engine().stats().appearanceApproximate >= 1);   // the face is the template's: reported, not faked
    }
    const av::RefId ref = h.an->puppetRef(1);
    const uint64_t updates = h.an->stats().appearanceUpdates;
    // a modification added to the held weapon: travels, applies, the puppet is NOT rebuilt
    a.be.Player().mods[kGun] = 0x07;
    r.run(4000);
    CHECK(h.puppet(1) && h.puppet(1)->mods[kGun] == 0x07 && b.puppet(1) && b.puppet(1)->mods[kGun] == 0x07);
    CHECK(h.an->puppetRef(1) == ref);
    CHECK(h.an->stats().appearanceUpdates == updates);
    // a new hat: an equipment change, still the same puppet
    a.be.Player().worn.insert(kHat);
    a.be.Player().inv[kHat] = 1;
    r.run(4000);
    CHECK(h.puppet(1) && h.puppet(1)->worn.count(kHat) == 1);
    CHECK(h.an->puppetRef(1) == ref);
    // the body changes (weight / height): applied to the same puppet
    a.be.baseAppearance[7].height = 1200;
    r.run(5000);
    CHECK(h.an->puppetRef(1) == ref);
    CHECK_NEAR(h.puppet(1)->scale, 1.2, 1e-3);
    CHECK(h.an->stats().appearanceUpdates > updates);
    // another sex needs the other template NPC: a new puppet, still wearing the same things, same height
    a.be.baseAppearance[7].female = true;
    r.run(6000);
    auto* f = h.puppet(1);
    CHECK(f && h.an->puppetRef(1) != ref && f->base == kTplF);
    CHECK(f && f->worn.count(kArmor) == 1 && f->worn.count(kGun) == 1 && f->worn.count(kHat) == 1);
    CHECK(f && f->mods[kGun] == 0x07);
    CHECK(f && std::fabs(f->scale - 1.2) < 1e-3);
    CHECK(h.an->stats().puppetRespawnsForLook >= 1);
    CheckHealthy(r, "look");
}

void a_late_joiner_learns_every_look() {
    g_test = "a_late_joiner_learns_every_look";
    Rig r;
    Node& h = r.host();
    Node& a = r.addClient(1);
    Node& b = r.addClient(2);
    b.present = false;
    h.be.baseAppearance[7].height = 1030;
    a.be.baseAppearance[7] = BigLook();
    a.be.Player().worn = {kGun};
    a.be.Player().inv[kGun] = 1;
    a.be.Player().mods[kGun] = 0x01;
    h.vx = a.vx = 20;
    r.run(4000);
    b.present = true;                            // B arrives after everybody's look was sent once
    b.vx = 20;
    r.run(6000);
    CHECK(b.an->lookOf(0) != nullptr && b.an->lookOf(1) != nullptr);
    CHECK(b.puppet(1) && b.puppet(1)->mods[kGun] == 0x01 && std::fabs(b.puppet(1)->scale - 1.1) < 1e-3);
    CHECK(b.puppet(0) && std::fabs(b.puppet(0)->scale - 1.03) < 1e-3);
    CheckHealthy(r, "late joiner");
}

// ============================================================================================================================
// lifecycle: join, leave, cell change, fast travel, load
// ============================================================================================================================
// Live game 2026-10-22 (live_wiring_all): the client's avatar runtime was created with the ini default localSlot 0 and never told its real slot, so
// it refused the HOST's puppet as "the local slot" ("client has a puppet ref for the host" failed with ref 0 while 169 snapshots had arrived).
void a_client_gets_the_hosts_puppet_whatever_the_ini_local_slot_says() {
    g_test = "a_client_gets_the_hosts_puppet_whatever_the_ini_local_slot_says";
    Rig r;
    Node& h = r.host();
    Node& a = r.addClient(1);
    Node& b = r.addClient(2);
    CHECK(a.rt->manager().LocalSlot() == 0 && b.rt->manager().LocalSlot() == 0);   // the ini default: wrong for a client until the session says so
    r.run(3500);
    CHECK(a.rt->manager().LocalSlot() == 1 && b.rt->manager().LocalSlot() == 2 && h.rt->manager().LocalSlot() == 0);
    CHECK(a.puppet(0) != nullptr && b.puppet(0) != nullptr);            // the host's puppet, on every client
    CHECK(h.puppet(1) != nullptr && h.puppet(2) != nullptr && a.puppet(2) != nullptr && b.puppet(1) != nullptr);
    CHECK(a.puppet(1) == nullptr && b.puppet(2) == nullptr);            // nobody gets a puppet of himself
    CHECK(h.actors() == 2 && a.actors() == 2 && b.actors() == 2);
}

void lifecycle_end_to_end() {
    g_test = "lifecycle_end_to_end";
    Rig r(true);
    Node& h = r.host();
    Node& a = r.addClient(1);
    Node& b = r.addClient(2);
    b.present = false;
    h.vx = a.vx = b.vx = 40;
    av::ManagerDirectory dir(h.rt->manager(), 0);
    av::setPuppetDirectory(&dir);
    struct Unreg { ~Unreg() { av::setPuppetDirectory(nullptr); } } unreg;

    // 1. A joins; B is not there yet
    r.run(3500);
    CHECK(h.actors() == 1 && a.actors() == 1 && b.actors() == 0);
    CHECK(av::puppetRefOf(1) == h.an->puppetRef(1) && av::puppetRefOf(1) != 0);
    CHECK(av::actingRefOf(1) == av::puppetRefOf(1) && av::actingRefOf(0) == av::kLocalPlayerRefId);
    av::Slot sl = 99;
    CHECK(av::slotOfPuppet(av::puppetRefOf(1), sl) && sl == 1);

    // 2. B joins: everybody gets one puppet per other player, through the host
    b.present = true;
    r.run(4000);
    CHECK(h.actors() == 2 && a.actors() == 2 && b.actors() == 2);
    CheckHealthy(r, "after B joined");
    const av::RefId aOnHost = av::puppetRefOf(1);

    // 3. A walks into a building: parked everywhere (its cell is not loaded there), still on the roster, the ownership service knows where it is
    a.be.Player().cell = kInt;
    a.be.Player().pos = {100, 100, 0};
    a.vx = 0;
    r.run(4500);
    CHECK(h.an->roster().count(1) == 1 && h.an->puppetRef(1) == 0 && b.an->puppetRef(1) == 0);
    CHECK(av::puppetRefOf(1) == 0 && !av::isPuppet(aOnHost));      // the API never answers a ref that is gone
    CHECK(a.actors() == 0);                                        // inside, the others' cell is not loaded: A has no puppets either
    bool placed = false;
    for (auto& p : h.poses)
        if (std::get<0>(p) == 1 && std::get<1>(p) && std::get<2>(p) == kInt) placed = true;
    CHECK(placed);
    CheckHealthy(r, "A inside");

    // 4. A comes out again, next to B: one puppet again
    a.be.Player().cell = kExtA;
    a.be.Player().pos = {2600, 0, 0};
    a.vx = 40;
    r.run(4500);
    CHECK(h.actors() == 2 && av::puppetRefOf(1) != 0);
    bool ok = false;
    CHECK_NEAR(PuppetX(h, 1, &ok), PlayerX(a), 200.0f);
    CHECK(ok);
    CheckHealthy(r, "A outside again");

    // 5. B fast travels into the next exterior cell of the same worldspace: SetPos only, the puppet of B on the host stays the same object
    const av::RefId bOnHost = av::puppetRefOf(2);
    const uint64_t spawnedBefore = h.rt->engine().stats().spawnCalls;
    b.be.Player().cell = kExtB;
    b.be.Player().pos = {4600, 0, 0};
    r.run(4000);
    CHECK(av::puppetRefOf(2) == bOnHost);
    CHECK(h.rt->engine().stats().spawnCalls == spawnedBefore);
    CHECK_NEAR(PuppetX(h, 2, &ok), PlayerX(b), 250.0f);

    // 6. A fast travels to another worldspace, far away: parked on the other machines, back later: exactly one puppet again
    a.be.Player().cell = kOtherWorld;
    a.be.Player().pos = {90000, 0, 0};
    r.run(4500);
    CHECK(h.an->puppetRef(1) == 0 && b.an->puppetRef(1) == 0 && h.actors() == 1);
    CheckHealthy(r, "A in another worldspace");
    a.be.Player().cell = kExtA;
    a.be.Player().pos = {2600, 0, 0};
    r.run(5000);
    CHECK(h.actors() == 2 && b.actors() == 2 && a.actors() == 2);
    CheckHealthy(r, "A back from the other worldspace");

    // 7. the host loads a save (the world is torn down; the save carried puppets of the old session): one puppet per remote player afterwards
    h.hub.onLoadBegin();
    h.be.Load();
    h.hub.onLoadEnd();
    r.run(5000);
    CHECK(h.actors() == 2);
    CHECK(av::puppetRefOf(1) != 0 && av::puppetRefOf(2) != 0 && av::puppetRefOf(1) != av::puppetRefOf(2));
    CheckHealthy(r, "after the host loaded");

    // 8. B quits, A stays; B comes back somewhere else
    b.present = false;
    r.run(2500);
    CHECK(h.actors() == 1 && a.actors() == 1 && b.actors() == 0);
    CHECK(av::puppetRefOf(2) == 0 && av::actingRefOf(2) == 0);
    b.be.Player().cell = kExtA;
    b.be.Player().pos = {5200, 0, 0};
    b.present = true;
    r.run(5000);
    CHECK(h.actors() == 2 && a.actors() == 2 && b.actors() == 2);
    CHECK_NEAR(PuppetX(h, 2, &ok), PlayerX(b), 250.0f);
    CheckHealthy(r, "B rejoined");
}

void client_load_and_link_loss() {
    g_test = "client_load_and_link_loss";
    Rig r(true);
    Node& h = r.host();
    Node& a = r.addClient(1);
    h.vx = a.vx = 30;
    r.run(3500);
    CHECK(a.actors() == 1 && h.actors() == 1);
    // A loads a save: its puppets are dropped without touching the engine and come back; the host keeps its puppet of A throughout
    const av::RefId onHost = h.an->puppetRef(1);
    a.hub.onLoadBegin();
    a.be.Load();
    a.hub.onLoadEnd();
    r.run(4500);
    CHECK(a.actors() == 1);
    CHECK(h.an->puppetRef(1) == onHost);
    // the link drops and returns: A's puppets go with the roster and come back with it; the host's stays until A leaves
    a.linkUp = false;
    r.run(1500);
    CHECK(a.actors() == 0);
    a.linkUp = true;
    r.run(4500);
    CHECK(a.actors() == 1 && h.actors() == 1);
    CheckHealthy(r, "client load and link loss");
}

// ============================================================================================================================
// hits on a puppet reach the real player's machine (combat API, read-only)
// ============================================================================================================================
// The victim's machine as combat sees it: the player is its own owner, an NPC attacker is owned by the sender.
struct VictimWorld : cc::ICombatWorld {
    cc::CellId cell = 7;
    cc::Vec3 playerPos{1000, 0, 0}, npcPos{1400, 0, 0};
    uint8_t npcOwner = 0;
    fnvcoop::OwnerEpoch npcEpoch = 3;
    bool facts(const cc::Entity& e, cc::ActorFacts& out) const override {
        out = cc::ActorFacts{};
        if (e.isPlayer() && e.id == 1) {
            out.known = true; out.who = e; out.alive = true; out.cell = cell; out.pos = playerPos; out.owner = 1; out.health = 100; out.maxHealth = 100;
            return true;
        }
        if (e.isActor() && e.id == 0x100) {
            out.known = true; out.who = e; out.alive = true; out.cell = cell; out.pos = npcPos; out.owner = npcOwner; out.epoch = npcEpoch; out.health = 50; out.maxHealth = 50;
            return true;
        }
        return false;
    }
};

// The port of the machine that owns the enemy: it builds the HitEvent for the victim player and "sends" it to that player's machine.
struct VictimMachine {
    cc::HitValidator validator;
    VictimWorld world;
    cc::VictimVitals vitals;
    uint64_t received = 0, applied = 0, rejected = 0;
    VictimMachine() { vitals.health = vitals.maxHealth = 100; }
    void receive(const cc::HitEvent& h, fnvcoop::PlayerId sender) {
        ++received;
        cc::HitVerdict v = validator.judge(h, sender, 1, 5000, world);
        if (!v.apply) { ++rejected; return; }
        cc::DamageOutcome o = cc::applyHit(vitals, h, validator.rules());
        if (o.applied) ++applied;
    }
};

struct EnemyOwnerPort : av::IPuppetHitPort {
    VictimMachine& victim;
    uint64_t counter = 0;
    av::HitAction answer = av::HitAction::Clamp;
    explicit EnemyOwnerPort(VictimMachine& v) : victim(v) {}
    cc::HitEvent build(av::Slot slot, const av::PuppetHit& h) {
        cc::HitEvent e;
        e.hitId = (uint64_t(0) << 48) | ++counter;
        e.attacker = cc::Entity::actor(0x100);
        e.victim = cc::Entity::player(slot);
        e.damage = static_cast<int32_t>(h.healthDmg);
        e.limb = static_cast<cc::Limb>(h.hitLocation >= 0 && h.hitLocation < cc::kLimbCount ? h.hitLocation : 0);
        e.impactPoint = cc::Vec3{h.impact.x, h.impact.y, h.impact.z};
        e.attackerEpoch = 3;
        e.claimedRange = 2000;
        e.firedAtTick = 5000;
        return e;
    }
    bool Forward(av::Slot slot, const av::PuppetHit& h, av::HitAction& act) override {
        act = answer;
        cc::HitEvent e = build(slot, h);
        victim.receive(e, 0);
        last = e;
        return true;
    }
    cc::HitEvent last;
};

void hits_on_a_puppet_reach_the_real_player() {
    g_test = "hits_on_a_puppet_reach_the_real_player";
    Rig r;
    Node& h = r.host();
    Node& a = r.addClient(1);
    h.vx = a.vx = 30;
    r.run(3500);
    av::ManagerDirectory dir(h.rt->manager(), 0);
    av::setPuppetDirectory(&dir);
    struct Unreg { ~Unreg() { av::setPuppetDirectory(nullptr); } } unreg;
    const av::RefId puppet = av::puppetRefOf(1);
    CHECK(puppet != 0);

    VictimMachine victim;                       // A's machine
    EnemyOwnerPort port(victim);                // the host owns the enemy that shoots A's puppet
    av::PuppetHitRouter router;
    router.SetPort(&port);
    av::PuppetHit hit;
    hit.victimRef = puppet;
    hit.attackerRef = 0x5150;
    hit.healthDmg = 18;
    hit.hitLocation = 2;

    // the enemy hits the puppet: A's machine takes exactly the damage, the puppet on the host stays alive
    av::HitOutcome o = router.OnHit(hit);
    CHECK(o.route == av::HitRoute::Forwarded && o.action == av::HitAction::Clamp && o.victimSlot == 1);
    CHECK(victim.applied == 1 && victim.vitals.health == 82);
    h.be.Hit(puppet, 18);                       // the local engine may still hurt the stand-in (clamped by the gate): it never dies of it
    r.run(3000);
    CHECK(h.puppet(1) && !h.puppet(1)->dead && h.puppet(1)->health == 100);   // topped up again

    // the same hit delivered twice (a retried datagram) counts once
    port.victim.receive(port.last, 0);
    CHECK(victim.applied == 1 && victim.vitals.health == 82);
    // a hit from a machine that does not own the enemy is a forgery
    cc::HitEvent forged = port.build(1, hit);
    victim.receive(forged, 2);
    CHECK(victim.applied == 1 && victim.rejected >= 2);
    // an absurd damage value is refused
    av::PuppetHit big = hit;
    big.healthDmg = 5000;
    router.OnHit(big);
    CHECK(victim.vitals.health == 82);
    // the enemy was nowhere near the player: out of range is refused (the puppet's position on the host does not matter, A's does)
    victim.world.npcPos = cc::Vec3{90000, 0, 0};
    router.OnHit(hit);
    CHECK(victim.vitals.health == 82);
    victim.world.npcPos = cc::Vec3{1400, 0, 0};
    // another hit, 2 more: it adds up and a lethal one kills the REAL player, not the stand-in
    router.OnHit(hit);
    CHECK(victim.vitals.health == 64);
    av::PuppetHit lethal = hit;
    lethal.healthDmg = 400;
    router.OnHit(lethal);
    CHECK(victim.vitals.health <= 0);
    // the combat layer is off: nothing is forwarded, the stand-in is clamped and A is untouched
    const int before = victim.vitals.health;
    router.SetPort(nullptr);
    o = router.OnHit(hit);
    CHECK(o.route == av::HitRoute::NoPort && o.action == av::HitAction::Clamp && victim.vitals.health == before);
    // a hit on something that is not a puppet is none of the router's business
    hit.victimRef = 0x77777;
    o = router.OnHit(hit);
    CHECK(o.route == av::HitRoute::NotAPuppet && o.action == av::HitAction::Proceed);
}

// ============================================================================================================================
// the rate planner on its own
// ============================================================================================================================
void planner_tiers_hysteresis_and_pending() {
    g_test = "planner_tiers_hysteresis_and_pending";
    av::StreamRateConfig cfg;
    av::StreamRatePlanner p(cfg);
    live::Vec3 o{0, 0, 0};
    p.setPlace(0, 5, o);
    p.setPlace(1, 5, live::Vec3{1000, 0, 0});
    p.setPlace(2, 5, live::Vec3{3000, 0, 0});          // 3000 > 2800: far
    p.setPlace(3, 6, live::Vec3{10, 0, 0});            // another cell: out
    CHECK(p.tierOf(0, 1) == av::StreamTier::Near);
    CHECK(p.tierOf(0, 2) == av::StreamTier::Far);
    CHECK(p.tierOf(0, 3) == av::StreamTier::Out);
    CHECK(p.tierOf(0, 9) == av::StreamTier::Near);     // unknown place: never starved
    // hysteresis: a near player drops to far only beyond 2800 * 1.25 = 3500
    p.setPlace(2, 5, live::Vec3{2500, 0, 0});
    CHECK(p.tierOf(0, 2) == av::StreamTier::Near);
    p.setPlace(2, 5, live::Vec3{3300, 0, 0});
    CHECK(p.tierOf(0, 2) == av::StreamTier::Near);     // inside the hysteresis band: stays near
    p.setPlace(2, 5, live::Vec3{3600, 0, 0});
    CHECK(p.tierOf(0, 2) == av::StreamTier::Far);
    p.setPlace(2, 5, live::Vec3{3000, 0, 0});
    CHECK(p.tierOf(0, 2) == av::StreamTier::Far);      // and back to near only inside 2800
    // adjacency makes a different cell far instead of out
    p.setAdjacency([](live::CellId a, live::CellId b) { return a == 5 && b == 6; });
    CHECK(p.tierOf(0, 3) == av::StreamTier::Far);
    // far: one per 200 ms, the newest kept in between and released on time
    live::ActorSnapshot s, out;
    s.netId = 1;
    int sent = 0;
    for (uint32_t t = 0; t < 2000; t += 50) {
        s.seq = t / 50 + 1;
        s.pos.x = float(t);
        if (p.offer(0, 2, s, t, out)) ++sent;
    }
    CHECK(sent >= 9 && sent <= 11);
    // latest wins: a stopped stream's final snapshot is released when its turn comes (the last send above was at t = 1800)
    s.seq = 1000;
    s.pos.x = 1942;                                     // (a step of a walker: a jump of more than 350 units from the last one sent goes at once, see below)
    bool direct = p.offer(0, 2, s, 1850, out);
    std::vector<av::StreamRatePlanner::Release> rel;
    p.poll(1900, rel);
    CHECK(!direct && rel.empty() && p.pending() == 1);
    s.seq = 1001;                                       // a newer one replaces the kept one
    s.pos.x = 1943;
    CHECK(!p.offer(0, 2, s, 1920, out));
    p.poll(2000, rel);
    CHECK(rel.size() == 1 && rel[0].snap.seq == 1001 && rel[0].snap.pos.x == 1943 && rel[0].observer == 0 && rel[0].subject == 2);
    p.poll(2400, rel);
    CHECK(rel.size() == 1 && p.pending() == 0);        // nothing more to release
    // the uplink takes the highest tier of its observers
    live::ActorSnapshot u;
    u.seq = 1;
    int got = 0;
    for (uint32_t t = 10000; t < 12000; t += 50) {
        u.seq++;
        if (p.offerUplink(0, std::vector<uint8_t>{1, 2}, u, t, out)) ++got;   // player 1 is near player 0
    }
    CHECK(got >= 38);
    // disabled: everything goes through
    cfg.enabled = false;
    av::StreamRatePlanner q(cfg);
    q.setPlace(0, 1, o);
    q.setPlace(1, 1, live::Vec3{90000, 0, 0});
    int all = 0;
    for (uint32_t t = 0; t < 1000; t += 50) all += q.offer(0, 1, s, t, out) ? 1 : 0;
    CHECK(all == 20);
}

// ============================================================================================================================
// Look v2 wire format
// ============================================================================================================================
AvLook FullLook(uint8_t slot) {
    AvLook l;
    l.slot = slot;
    l.appearance = BigLook();
    l.equipment = {{kArmor, false, 0}, {kGun, true, 0x05}, {0xFF000123u, false, 0}};
    return l;
}

void wire_look_v2_roundtrip_and_rejections() {
    g_test = "wire_look_v2_roundtrip_and_rejections";
    AvLook l = FullLook(3);
    AvBytes b = encodeLook(l);
    CHECK(b[1] == kAvKindLook2 && b[2] == kAvatarWireVersion);
    AvLook o;
    CHECK(decodeLook(b.data(), b.size(), o) && o == l);
    CHECK(o.equipment[1].mods == 0x05 && o.appearance.faceCoeffs.size() == 130 && o.appearance.height == 1100);
    for (size_t n = 0; n < b.size(); ++n) CHECK(!decodeLook(b.data(), n, o));           // every truncation is rejected
    AvBytes extra = b;
    extra.push_back(7);
    CHECK(!decodeLook(extra.data(), extra.size(), o));
    // a minimal look carries none of the optional blocks
    AvLook m;
    m.slot = 1;
    m.appearance.race = 1;
    AvBytes mb = encodeLook(m);
    CHECK(mb.size() < b.size() / 4);
    CHECK(decodeLook(mb.data(), mb.size(), o) && o == m);
    // reserved flag bits: rejected (a future field must not be misread by this build)
    AvBytes bad = b;
    bad[4] |= 0x20;
    CHECK(!decodeLook(bad.data(), bad.size(), o));
    // head parts (eyebrows, facial hair): flag 0x10, a count and the form ids; a look without them is the same bytes as before
    AvLook hp = l;
    hp.appearance.headParts = {0x70, 0x71, 0x72};
    AvBytes hb = encodeLook(hp);
    CHECK(hb.size() == b.size() + 1 + 12 && (hb[4] & 0x10) && !(b[4] & 0x10));
    CHECK(decodeLook(hb.data(), hb.size(), o) && o == hp && o.appearance.headParts == (std::vector<uint32_t>{0x70, 0x71, 0x72}));
    CHECK(!(o == l));                                                                   // a changed head part is a changed look
    AvLook hp17 = l;
    hp17.appearance.headParts.assign(av::kMaxHeadParts + 1, 5);
    AvBytes hb17 = encodeLook(hp17);
    CHECK(decodeLook(hb17.data(), hb17.size(), o) && o.appearance.headParts.size() == av::kMaxHeadParts);   // the sender cuts at the cap
    AvBytes zero = hb;
    zero[4 + 1 + 16 + 4 + 4 + 1 + 130] = 0;                                              // flags, race hair eyes face, hair colour, height weight, face count + 130 coefficients, then the part count
    CHECK(!decodeLook(zero.data(), zero.size(), o));                                     // a part block of none is not a block
    // a face block over the cap is rejected
    AvLook many = l;
    many.appearance.faceCoeffs.assign(av::kMaxFaceCoeffs, 3);
    AvBytes fb = encodeLook(many);
    CHECK(decodeLook(fb.data(), fb.size(), o) && o.appearance.faceCoeffs.size() == av::kMaxFaceCoeffs);
    // find the face count byte (after slot flags race hair eyes face hairColor height weight) and push it over the cap
    const size_t faceCountAt = 2 + 1 + 1 + 1 + 16 + 4 + 4;
    CHECK(fb[faceCountAt] == av::kMaxFaceCoeffs);
    AvBytes over = fb;                                  // one more coefficient, structurally valid: only the cap can refuse it
    over[faceCountAt] = static_cast<uint8_t>(av::kMaxFaceCoeffs + 1);
    over.insert(over.begin() + faceCountAt + 1 + av::kMaxFaceCoeffs, 0);
    CHECK(!decodeLook(over.data(), over.size(), o));
    // junk modification flags: zero, bits above the three mods, or mods on armor are rejected
    AvLook junk = l;
    junk.equipment[1].mods = 0x05;
    AvBytes jb = encodeLook(junk);
    size_t modAt = 0;
    for (size_t i = jb.size(); i-- > 0;)
        if (jb[i] == 0x05) { modAt = i; break; }
    CHECK(modAt > 0);
    AvBytes j2 = jb;
    j2[modAt] = 0x08;
    CHECK(!decodeLook(j2.data(), j2.size(), o));
    j2[modAt] = 0x00;
    CHECK(!decodeLook(j2.data(), j2.size(), o));
    // the wrong version byte
    AvBytes ver = b;
    ver[2] = 9;
    CHECK(!decodeLook(ver.data(), ver.size(), o));
    // the old format is still read
    AvBytes v1;
    v1.push_back(kChannelAvatars);
    v1.push_back(kAvKindLook);
    v1.push_back(2);                                   // slot
    const uint8_t race[4] = {1, 0, 0, 0}, item[4] = {0x00, 0x30, 0x00, 0x00};
    v1.insert(v1.end(), race, race + 4);               // race
    v1.push_back(1);                                   // female
    for (int i = 0; i < 12; ++i) v1.push_back(0);      // hair eyes face
    v1.push_back(0);                                   // name length
    v1.push_back(1);                                   // one item
    v1.insert(v1.end(), item, item + 4);
    v1.push_back(1);                                   // weapon
    CHECK(decodeLook(v1.data(), v1.size(), o) && o.slot == 2 && o.appearance.female && o.equipment.size() == 1 && o.equipment[0].weapon && o.equipment[0].mods == 0);
    // a changed modification is a different look (it must travel)
    AvLook l2 = l;
    l2.equipment[1].mods = 0x07;
    CHECK(!(l == l2));
    // random garbage never crashes
    uint32_t x = 99;
    AvLook sink;
    for (int i = 0; i < 4000; ++i) {
        AvBytes g(2 + (x % 120));
        for (auto& c : g) { x = x * 1664525u + 1013904223u; c = static_cast<uint8_t>(x >> 24); }
        g[0] = kChannelAvatars;
        g[1] = (i & 1) ? kAvKindLook2 : kAvKindLook;
        if (g.size() > 2 && (i & 1)) g[2] = kAvatarWireVersion;
        decodeLook(g.data(), g.size(), sink);
    }
    CHECK(true);
}

// The keys tools/avatar/proof_avatars.py reads from statusJson (the answer of coop.live "avatars").
void status_json_contract() {
    g_test = "status_json_contract";
    Rig r;
    Node& h = r.host();
    Node& a = r.addClient(1);
    h.vx = a.vx = 40;
    a.be.baseAppearance[7] = BigLook();
    a.be.Player().worn = {kGun};
    a.be.Player().inv[kGun] = 1;
    a.be.Player().mods[kGun] = 0x03;
    a.be.Player().cell = kExtA;
    r.run(5000);
    nvcoop::json::Value v;
    std::string err;
    CHECK(nvcoop::json::parse(h.an->statusJson(), v, err));
    const nvcoop::json::Value* roster = v.find("roster");
    CHECK(roster && roster->items().size() == 1);
    if (!roster || roster->items().empty()) return;
    const nvcoop::json::Value& e = roster->items()[0];
    CHECK(e.get("slot").asInt(-1) == 1 && e.get("ref").asInt(0) != 0 && e.get("look").asBool(false));
    CHECK(e.get("tier").asString("") == "near" && e.get("rx").asInt(0) > 50);
    const nvcoop::json::Value* li = e.find("lookInfo");
    CHECK(li && li->get("race").asInt(0) == 1 && li->get("height").asInt(0) == 1100 && li->get("weight").asInt(0) == 950 && li->get("hairColor").asInt(0) == 0x00A0B0C0);
    CHECK(li && li->get("faceCoeffs").asInt(0) == 130 && li->find("equip") && li->find("equip")->items().size() == 1);
    if (li && li->find("equip") && !li->find("equip")->items().empty()) {
        const nvcoop::json::Value& q = li->find("equip")->items()[0];
        CHECK(q.get("form").asInt(0) == static_cast<int>(kGun) && q.get("weapon").asBool(false) && q.get("mods").asInt(0) == 3);
    }
    // the source machine reports the look it sent
    CHECK(nvcoop::json::parse(a.an->statusJson(), v, err));
    const nvcoop::json::Value* mine = v.find("selfLook");
    CHECK(mine && mine->get("height").asInt(0) == 1100 && mine->find("equip") && mine->find("equip")->items().size() == 1);
    CHECK(v.get("rateKept").asInt(-1) >= 0 && v.get("rateReleased").asInt(-1) >= 0 && v.get("appearanceUpdates").asInt(-1) >= 0);
}

struct TestCase { const char* name; void (*fn)(); };
#define T(n) {#n, n}
const TestCase kTests[] = {
    T(wire_look_v2_roundtrip_and_rejections), T(planner_tiers_hysteresis_and_pending),
    T(rates_near_far_and_out), T(uplink_follows_the_highest_tier), T(elsewhere_trickles_and_comes_back), T(last_position_is_never_lost_behind_the_rate),
    T(look_travels_and_applies_in_place), T(a_late_joiner_learns_every_look),
    T(a_client_gets_the_hosts_puppet_whatever_the_ini_local_slot_says), T(lifecycle_end_to_end), T(client_load_and_link_loss), T(hits_on_a_puppet_reach_the_real_player), T(status_json_contract),
};

}  // namespace

int main(int argc, char** argv) {
    std::setvbuf(stdout, nullptr, _IONBF, 0);
    std::vector<std::string> filters;
    bool list = false;
    for (int i = 1; i < argc; ++i) {
        if (!std::strcmp(argv[i], "--list")) list = true;
        else filters.push_back(argv[i]);
    }
    if (list) {
        for (const TestCase& t : kTests) std::printf("%s\n", t.name);
        return 0;
    }
    int ran = 0;
    for (const TestCase& t : kTests) {
        bool pick = filters.empty();
        for (const std::string& f : filters)
            if (std::strstr(t.name, f.c_str())) pick = true;
        if (!pick) continue;
        int before = g_fail;
        g_test = t.name;
        t.fn();
        ++ran;
        std::printf("[%s] %s\n", g_fail == before ? " ok " : "FAIL", t.name);
    }
    std::printf("avatar full: %d tests, %d checks, %d failed\n", ran, g_checks, g_fail);
    return g_fail ? 1 : 0;
}
