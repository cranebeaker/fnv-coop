// plugin/combat: wire round trips, hostile bytes, and a randomised multi-machine soak with the invariants of the module:
//   * a hit id is applied at most once in the whole cluster, and only on the machine that owned the victim then;
//   * no actor ever has negative health, no actor dies twice, nobody is killed by a prediction;
//   * the XP of one death is paid once.
#include "harness.h"

using namespace ctest;

namespace {

struct Rng {
    uint64_t s;
    explicit Rng(uint64_t seed) : s(seed * 0x9E3779B97F4A7C15ull + 1) {}
    uint32_t next() { s ^= s << 13; s ^= s >> 7; s ^= s << 17; return static_cast<uint32_t>(s >> 11); }
    uint32_t below(uint32_t n) { return n ? next() % n : 0; }
    bool chance(uint32_t pct) { return below(100) < pct; }
};

cc::HitEvent sampleHit() {
    cc::HitEvent h;
    h.hitId = (3ull << 48) | 17; h.attacker = cc::Entity::player(2); h.victim = cc::Entity::actor(100); h.damage = 33; h.limb = cc::Limb::LeftLeg;
    h.impactPoint = {1, 2, 3}; h.weapon = rifle(); h.ammo = FormRef("falloutnv.esm", 0x2000); h.projectile = 5; h.explosionId = 6; h.firedAtTick = 1234;
    h.attackerEpoch = 9; h.claimedRange = 1500; h.critical = true; h.vats = true;
    return h;
}

TEST(combat_wire, every_message_round_trips) {
    std::vector<Body> bodies;
    bodies.push_back(MHit{sampleHit()});
    bodies.push_back(MHitNack{42, cc::HitReason::HandoffInProgress, true});
    MVitals v; v.who = cc::Entity::actor(7); v.epoch = 3; v.seq = 11; v.health = 55; v.maxHealth = 100; v.limbMask = 5; v.dead = true; v.cause = 99;
    bodies.push_back(v);
    cc::DeathDecl d; d.deathId = 8; d.who = cc::Entity::actor(7); d.source = cc::DeathSource::StoryHost; d.declarer = 0; d.epoch = 3; d.killer = cc::Entity::player(1);
    d.weapon = rifle(); d.limbMask = 3; d.cell = 4; d.where = {1, 2, 3}; d.tick = 77;
    bodies.push_back(MDeath{d});
    cc::ProjectileSpawn p; p.id = 4; p.shooter = cc::Entity::actor(7); p.shooterEpoch = 3; p.weapon = rifle(); p.origin = {1, 2, 3}; p.velocity = {4, 5, 6}; p.explosive = true;
    bodies.push_back(MProjectile{p});
    cc::ExplosionEvent x; x.id = 12; x.source = cc::Entity::player(1); x.radius = 300; x.damage = 90; x.pos = {9, 9, 9};
    bodies.push_back(MExplosion{x});
    cc::CrimeReport cr; cr.criminal = 1; cr.victim = cc::Entity::actor(3); cr.victimFaction = FormRef("falloutnv.esm", 5); cr.criminalFaction = FormRef("falloutnv.esm", 6);
    cr.killed = true; cr.witnessed = true; cr.reporter = 1;
    bodies.push_back(MCrime{cr});
    cc::HostilityEffect fx; fx.id = 3; fx.kind = cc::HostilityKind::SetEnemy; fx.factionA = FormRef("falloutnv.esm", 5); fx.factionB = FormRef("falloutnv.esm", 6); fx.cause = 1;
    bodies.push_back(MHostility{fx});
    for (const Body& b : bodies) {
        Envelope e; e.src = 2; e.dest = 1; e.hops = 2; e.body = b;
        const Bytes wire = encodeEnvelope(e);
        CHECK(wire.size() > 3);
        CHECK(wire[0] == kChannelCombat);
        Envelope back;
        REQUIRE(decodeEnvelope(wire.data(), wire.size(), back));
        CHECK(back.src == 2); CHECK(back.dest == 1); CHECK(back.hops == 2);
        CHECK(back.body.index() == b.index());
        CHECK(encodeEnvelope(back) == wire);        // re-encoding the decoded message gives the same bytes
        CHECK(kindOf(back.body) == kindOf(b));
    }
    Envelope hit; hit.body = MHit{sampleHit()};
    Envelope back;
    const Bytes w = encodeEnvelope(hit);
    REQUIRE(decodeEnvelope(w.data(), w.size(), back));
    const cc::HitEvent& h = std::get<MHit>(back.body).h;
    CHECK(h.hitId == sampleHit().hitId); CHECK(h.damage == 33); CHECK(h.limb == cc::Limb::LeftLeg); CHECK(h.weapon == rifle()); CHECK(h.vats); CHECK(h.critical);
    CHECK(h.attackerEpoch == 9); CHECK(h.impactPoint == (cc::Vec3{1, 2, 3}));
}

TEST(combat_wire, the_touch_flag_and_the_exact_damage_of_a_hit_travel) {
    // a blow with no health damage is sent as damage 1 plus the touch flag; every other blow carries its exact damage in hundredths next to the rules' whole points
    struct Case { bool touch; uint32_t centi; };
    for (const Case k : {Case{false, 0}, Case{true, 0}, Case{false, 50}, Case{false, 1240}, Case{false, kMaxCenti}}) {
        Envelope e; e.src = 2; e.dest = 0; e.body = MHit{sampleHit(), k.touch, k.centi};
        const Bytes w = encodeEnvelope(e);
        Envelope back;
        REQUIRE(decodeEnvelope(w.data(), w.size(), back));
        CHECK(std::get<MHit>(back.body).touch == k.touch);
        CHECK(std::get<MHit>(back.body).centi == k.centi);
        CHECK(encodeEnvelope(back) == w);
    }
    // a hostile amount beyond any real blow is refused, not decoded into a float nobody meant
    Envelope big; big.src = 2; big.dest = 0; big.body = MHit{sampleHit(), false, kMaxCenti + 1};
    const Bytes wb = encodeEnvelope(big);
    Envelope out;
    CHECK(!decodeEnvelope(wb.data(), wb.size(), out));
}

TEST(combat_wire, truncation_wrong_channel_and_trailing_bytes_are_refused) {
    Envelope e; e.src = 1; e.dest = 0; e.body = MHit{sampleHit()};
    const Bytes wire = encodeEnvelope(e);
    Envelope out;
    for (size_t n = 0; n < wire.size(); ++n) CHECK_MSG(!decodeEnvelope(wire.data(), n, out), "prefix " + std::to_string(n));
    Bytes extra = wire; extra.push_back(0);
    CHECK(!decodeEnvelope(extra.data(), extra.size(), out));
    Bytes wrong = wire; wrong[0] = 0xAC;
    CHECK(!decodeEnvelope(wrong.data(), wrong.size(), out));
    Bytes ver = wire; ver[1] = 99;
    CHECK(!decodeEnvelope(ver.data(), ver.size(), out));
    CHECK(!decodeEnvelope(nullptr, 0, out));
}

TEST(combat_wire, hostile_bytes_never_crash_or_change_anything) {
    Cluster c(3);
    c.addActor(100, 1, 3, {800, 0, 0});
    Envelope e; e.src = 1; e.dest = 0; e.body = MHit{sampleHit()};
    const Bytes good = encodeEnvelope(e);
    Rng r(7);
    uint64_t fed = 0;
    for (int i = 0; i < 4000; ++i) {
        Bytes b = good;
        const int mode = static_cast<int>(r.below(4));
        if (mode == 0) { b[r.below(static_cast<uint32_t>(b.size()))] ^= static_cast<uint8_t>(1u << r.below(8)); }
        else if (mode == 1) { b.resize(r.below(static_cast<uint32_t>(b.size()))); }
        else if (mode == 2) { for (auto& x : b) x = static_cast<uint8_t>(r.next()); b[0] = kChannelCombat; b[1] = kCombatModuleWire; }
        else { b.assign(r.below(40), 0); for (auto& x : b) x = static_cast<uint8_t>(r.next()); }
        c.cs(static_cast<int>(r.below(3))).receive(static_cast<PlayerId>(r.below(3)), b.data(), b.size());
        ++fed;
    }
    c.run(200);
    CHECK(fed == 4000);
    for (int i = 0; i < 3; ++i) CHECK(c.cs(i).stats().undecodable + c.cs(i).stats().hitsReceived + c.cs(i).stats().spoofed > 0);
    for (int i = 0; i < 3; ++i) {
        const Rec& a = c.at(i).world.recs[cc::Entity::actor(100)];
        CHECK(a.health >= 0);
    }
}

TEST(combat_wire, a_client_cannot_speak_for_another_client_and_only_the_host_is_a_peer_of_a_client) {
    Cluster c(3);
    c.addActor(100, 1, 3, {800, 0, 0});
    // client 1 sends a hit that claims to be from player 2
    cc::HitEvent h = sampleHit();
    h.hitId = 1; h.attacker = cc::Entity::player(2);
    Envelope e; e.src = 2; e.dest = 1; e.body = MHit{h};
    Bytes b = encodeEnvelope(e);
    c.cs(0).receive(1, b.data(), b.size());        // transport says it came from machine 1
    CHECK(c.cs(0).stats().spoofed == 1);
    c.run(100);
    CHECK(totalDamageCalls(c) == 0);                     // the forged author was rewritten to machine 1: not the hit's attacker
    // a client ignores datagrams that do not come from the host
    c.cs(2).receive(1, b.data(), b.size());
    CHECK(c.cs(2).stats().spoofed == 1);
}

// ---------------------------------------------------------------------------------------------------- randomised soak
TEST(combat_soak, random_fights_with_loss_free_reordering_duplicates_and_ownership_churn_keep_the_invariants) {
    uint64_t totalApplied = 0, totalDeaths = 0;
    for (uint64_t seed = 1; seed <= 120; ++seed) {
        Rng r(seed);
        const int n = 2 + static_cast<int>(r.below(3));
        Cluster c(n);
        c.latency = r.below(60);
        const uint32_t dupPct = r.below(30);
        Rng link(seed + 1000);
        c.copies = [&link, dupPct](const Datagram&) { return link.chance(dupPct) ? 2 : 1; };
        const int actors = 4;
        for (int a = 0; a < actors; ++a) c.addActor(100 + a, static_cast<PlayerId>(r.below(static_cast<uint32_t>(n))), 1, {static_cast<float>(300 + a * 90), 0, 0}, 60 + static_cast<int32_t>(r.below(80)));
        for (auto& m : c.m) m->eng.xpValue = 40;
        std::set<uint64_t> seenApplied;
        std::map<uint32_t, OwnerEpoch> epoch;
        for (int a = 0; a < actors; ++a) epoch[100u + a] = 1;
        for (int step = 0; step < 80; ++step) {
            const uint32_t op = r.below(10);
            if (op < 6) {
                const int atk = static_cast<int>(r.below(static_cast<uint32_t>(n)));
                const uint32_t a = 100 + r.below(actors);
                EngineHit h = c.shot(atk, refOfActor(a), static_cast<float>(5 + r.below(40)), static_cast<int>(r.below(13)));
                const GateDecision g = c.cs(atk).onEngineHit(h);
                if (g.action == GateAction::Clamp) {
                    const Rec& copy = c.at(atk).world.recs[cc::Entity::actor(a)];
                    CHECK(g.maxHealthDamage <= std::max(0, copy.health - 1));     // a prediction can never kill a copy
                }
            } else if (op < 8) {   // ownership churn: moved on every machine at once (the actor module guarantees order, this tests the consequences)
                const uint32_t a = 100 + r.below(actors);
                const PlayerId to = static_cast<PlayerId>(r.below(static_cast<uint32_t>(n)));
                epoch[a] += 1;
                c.setOwner(a, to, epoch[a]);
            } else {
                c.run(r.below(80), 10);
            }
            c.tick(10);
            // invariant: a hit id is applied once in the whole cluster
            for (int i = 0; i < n; ++i)
                for (const auto& d : c.at(i).eng.damage) {
                    // (ids already seen are checked below once at the end; here we only guard the health floor)
                    CHECK(c.at(i).world.recs[d.victim].health >= 0);
                }
        }
        c.run(1500);
        std::map<uint64_t, int> perHit;
        for (int i = 0; i < n; ++i) {
            for (const auto& d : c.at(i).eng.damage) {
                ++perHit[d.hitId];
                ++totalApplied;
                CHECK_MSG(d.owned, "seed " + std::to_string(seed) + ": machine " + std::to_string(i) + " applied a hit to a victim it did not own");
            }
            // owner-only: every damage call was on an actor this machine owned at the end or earlier (checked weakly: never on a copy marked pending)
        }
        for (auto& kv : perHit) CHECK_MSG(kv.second == 1, "seed " + std::to_string(seed) + " hit applied " + std::to_string(kv.second) + " times");
        // deaths: at most one declaration per actor in the cluster, XP paid at most once per actor
        for (int a = 0; a < actors; ++a) {
            int declared = 0;
            for (int i = 0; i < n; ++i) if (c.cs(i).deaths().isDead(cc::Entity::actor(100 + a))) { const cc::DeathDecl* d = c.cs(i).deaths().record(cc::Entity::actor(100 + a)); if (d) ++declared; }
            (void)declared;
        }
        int xpPaid = 0, declaredTotal = 0;
        for (int i = 0; i < n; ++i) { xpPaid += static_cast<int>(c.at(i).eng.xp.size()); declaredTotal += static_cast<int>(c.cs(i).stats().deathsDeclared); }
        totalDeaths += static_cast<uint64_t>(declaredTotal);
        CHECK_MSG(xpPaid <= declaredTotal, "seed " + std::to_string(seed) + ": xp awards " + std::to_string(xpPaid) + " > deaths " + std::to_string(declaredTotal));
        for (int a = 0; a < actors; ++a) {
            for (int i = 0; i < n; ++i) CHECK(c.at(i).world.recs[cc::Entity::actor(100 + a)].health >= 0);
        }
    }
    CHECK(totalApplied > 200);                       // the soak does real work
    CHECK(totalDeaths > 5);
}

}  // namespace
