// Wire round trips of every combat struct, and fuzz-safety of the decoders (truncated or flipped bytes never crash, never read garbage as valid).
#include <limits>
#include <random>
#include "fake_world.h"

using namespace combat_test;

namespace {
template <class T>
std::vector<uint8_t> enc(T v) {
    Archive a = Archive::writer();
    io(a, v);
    REQUIRE(a.ok());
    return a.finish();
}
template <class T>
bool dec(const std::vector<uint8_t>& b, T& out) {
    Archive a = Archive::reader(b.data(), b.size());
    if (!a.ok()) return false;
    io(a, out);
    return a.ok() && a.atEnd();
}

HitEvent fullHit() {
    HitEvent h;
    h.hitId = (3ull << 48) | 77; h.attacker = Entity::actor(0xFF000012u); h.victim = Entity::player(2); h.damage = 42; h.limb = Limb::RightLeg;
    h.impactPoint = {1.5f, -2.25f, 300.f}; h.weapon = rifle(); h.ammo = FormRef("dlc01.esm", 0x77); h.projectile = 99; h.explosionId = 5;
    h.firedAtTick = 123456; h.attackerEpoch = 9; h.claimedRange = 1234.5f; h.critical = true; h.vats = true;
    return h;
}
}  // namespace

TEST(wire, hit_round_trip) {
    HitEvent h = fullHit(), back;
    std::vector<uint8_t> b = enc(h);
    REQUIRE(dec(b, back));
    CHECK(back.hitId == h.hitId && back.attacker == h.attacker && back.victim == h.victim && back.damage == h.damage && back.limb == h.limb);
    CHECK(back.impactPoint == h.impactPoint && back.weapon == h.weapon && back.ammo == h.ammo && back.projectile == h.projectile);
    CHECK(back.explosionId == h.explosionId && back.firedAtTick == h.firedAtTick && back.attackerEpoch == h.attackerEpoch);
    CHECK(back.claimedRange == h.claimedRange && back.critical && back.vats);
    // an empty weapon / none attacker survives too
    HitEvent bare;
    HitEvent bareBack;
    REQUIRE(dec(enc(bare), bareBack));
    CHECK(bareBack.attacker.none() && bareBack.weapon.plugin.empty());
    // a hit is a handful of bytes, not a datagram
    CHECK(b.size() < 96);
}

TEST(wire, death_round_trip) {
    DeathDecl d;
    d.deathId = 5; d.who = Entity::actor(100); d.source = DeathSource::StoryHost; d.declarer = 0; d.epoch = 3; d.killer = Entity::player(1);
    d.weapon = rifle(); d.limbMask = 0x15; d.cell = 8; d.where = {1, 2, 3}; d.tick = 99;
    DeathDecl back;
    REQUIRE(dec(enc(d), back));
    CHECK(back.deathId == 5 && back.who == d.who && back.source == DeathSource::StoryHost && back.epoch == 3 && back.killer == d.killer);
    CHECK(back.weapon == d.weapon && back.limbMask == 0x15 && back.cell == 8 && back.where == d.where && back.tick == 99);
}

TEST(wire, projectile_and_explosion_round_trip) {
    ProjectileSpawn p;
    p.id = (1ull << 48) | 3; p.shooter = Entity::player(1); p.shooterEpoch = 0; p.weapon = rifle(); p.ammo = FormRef("falloutnv.esm", 5); p.projectileBase = FormRef("falloutnv.esm", 6);
    p.cell = 4; p.origin = {1, 2, 3}; p.velocity = {100, 0, -5}; p.firedAtTick = 1000; p.lifetimeMs = 4000; p.explosive = true;
    ProjectileSpawn pb;
    REQUIRE(dec(enc(p), pb));
    CHECK(pb.id == p.id && pb.shooter == p.shooter && pb.origin == p.origin && pb.velocity == p.velocity && pb.lifetimeMs == 4000 && pb.explosive && pb.projectileBase == p.projectileBase);
    ExplosionEvent e;
    e.id = 8; e.source = Entity::actor(7); e.sourceEpoch = 2; e.projectile = p.id; e.weapon = rifle(); e.explosionBase = FormRef("falloutnv.esm", 9);
    e.cell = 4; e.pos = {5, 6, 7}; e.radius = 250; e.damage = 120; e.tick = 77; e.firedAtTick = 70;
    ExplosionEvent eb;
    REQUIRE(dec(enc(e), eb));
    CHECK(eb.id == 8 && eb.source == e.source && eb.sourceEpoch == 2 && eb.projectile == p.id && eb.pos == e.pos && eb.radius == 250 && eb.damage == 120 && eb.tick == 77 && eb.firedAtTick == 70);
}

TEST(wire, hostility_round_trip) {
    HostilityEffect e;
    e.id = 0xABCDEF; e.kind = HostilityKind::SetAlly; e.factionA = FormRef("falloutnv.esm", 1); e.factionB = FormRef("falloutnv.esm", 2); e.actor = Entity::actor(3); e.cause = 1;
    HostilityEffect b;
    REQUIRE(dec(enc(e), b));
    CHECK(b == e);
}

TEST(wire, decoders_survive_truncation_and_flipped_bytes) {
    const std::vector<uint8_t> hit = enc(fullHit());
    DeathDecl d;
    d.deathId = 5; d.who = Entity::actor(100); d.weapon = rifle(); d.where = {1, 2, 3};
    const std::vector<uint8_t> death = enc(d);
    ProjectileSpawn p;
    p.id = 1; p.shooter = Entity::player(1); p.weapon = rifle();
    const std::vector<uint8_t> proj = enc(p);
    ExplosionEvent x;
    x.id = 1; x.source = Entity::player(1); x.radius = 3; x.damage = 4; x.weapon = rifle();
    const std::vector<uint8_t> expl = enc(x);
    // every prefix is either rejected or (if it ever parses) leaves the archive ok: never a crash
    for (size_t n = 0; n < hit.size(); ++n) { HitEvent o; std::vector<uint8_t> t(hit.begin(), hit.begin() + static_cast<long>(n)); CHECK(!dec(t, o)); }
    for (size_t n = 0; n < death.size(); ++n) { DeathDecl o; std::vector<uint8_t> t(death.begin(), death.begin() + static_cast<long>(n)); CHECK(!dec(t, o)); }
    for (size_t n = 0; n < proj.size(); ++n) { ProjectileSpawn o; std::vector<uint8_t> t(proj.begin(), proj.begin() + static_cast<long>(n)); CHECK(!dec(t, o)); }
    for (size_t n = 0; n < expl.size(); ++n) { ExplosionEvent o; std::vector<uint8_t> t(expl.begin(), expl.begin() + static_cast<long>(n)); CHECK(!dec(t, o)); }
    // random single-byte damage: decoding must not crash, and a damaged limb or entity kind must be refused, not stored
    std::mt19937 rng(7);
    int refused = 0;
    for (int i = 0; i < 2000; ++i) {
        std::vector<uint8_t> t = hit;
        t[rng() % t.size()] ^= static_cast<uint8_t>(1u << (rng() % 8));
        HitEvent o;
        if (!dec(t, o)) ++refused;
        else CHECK(static_cast<uint8_t>(o.limb) < kLimbCount && static_cast<uint8_t>(o.victim.kind) < 3 && static_cast<uint8_t>(o.attacker.kind) < 3);
    }
    CHECK(refused > 0);
    for (int i = 0; i < 500; ++i) {   // pure noise
        std::vector<uint8_t> t(rng() % 64);
        for (auto& c : t) c = static_cast<uint8_t>(rng());
        HitEvent h; DeathDecl dd; ProjectileSpawn pp; ExplosionEvent ee; HostilityEffect he;
        (void)dec(t, h); (void)dec(t, dd); (void)dec(t, pp); (void)dec(t, ee); (void)dec(t, he);
    }
}

TEST(wire, non_finite_floats_are_refused) {
    HitEvent h = fullHit();
    h.impactPoint.x = std::numeric_limits<float>::infinity();
    Archive a = Archive::writer();
    io(a, h);
    CHECK(!a.ok());
}

TEST(wire, the_combat_channel_bundle_round_trips_every_kind_and_refuses_the_rest) {   // twins 364, 365
    DeathDecl d;
    d.deathId = 9; d.who = Entity::actor(100); d.where = {1, 2, 3};
    ProjectileSpawn p;
    p.id = 5; p.shooter = Entity::player(1);
    ExplosionEvent x;
    x.id = 6; x.source = Entity::player(1); x.radius = 100; x.damage = 10;
    HostilityEffect h;
    h.id = 7; h.factionA = FormRef("falloutnv.esm", 1); h.factionB = FormRef("falloutnv.esm", 2);
    const std::vector<CombatMsg> all = {fullHit(), d, p, x, h};
    for (size_t i = 0; i < all.size(); ++i) {
        const std::vector<uint8_t> b = encodeMsg(all[i]);
        REQUIRE(b.size() > 3);
        CHECK(b[0] == kCombatWireByte && b[1] == kCombatWireVersion);
        CombatMsg back;
        CHECK_MSG(decodeMsg(b.data(), b.size(), back), msgName(all[i]));
        CHECK(back.index() == all[i].index());
        CHECK(encodeMsg(back) == b);   // re-encoding the decoded message is byte-identical
    }
    const std::vector<uint8_t> hit = encodeMsg(CombatMsg{fullHit()});
    CombatMsg out;
    std::vector<uint8_t> wrongChannel = hit;
    wrongChannel[0] = 0xAC;   // an actor-sync datagram
    CHECK(!decodeMsg(wrongChannel.data(), wrongChannel.size(), out));
    std::vector<uint8_t> wrongVersion = hit;
    wrongVersion[1] = static_cast<uint8_t>(kCombatWireVersion + 1);
    CHECK(!decodeMsg(wrongVersion.data(), wrongVersion.size(), out));
    std::vector<uint8_t> trailing = hit;
    trailing.push_back(0);
    CHECK(!decodeMsg(trailing.data(), trailing.size(), out));
    for (size_t n = 0; n < hit.size(); ++n) CHECK(!decodeMsg(hit.data(), n, out));
    std::mt19937 rng(11);
    for (int i = 0; i < 1500; ++i) {
        std::vector<uint8_t> t = hit;
        t[rng() % t.size()] ^= static_cast<uint8_t>(1u << (rng() % 8));
        (void)decodeMsg(t.data(), t.size(), out);   // must not crash; either answer is fine
    }
    CHECK(encodeMsg(CombatMsg{[] { HitEvent h2; h2.impactPoint.x = std::numeric_limits<float>::quiet_NaN(); return h2; }()}).empty());
}

TEST(wire, the_node_resets_everything_for_a_new_world) {
    CombatNode n;
    FakeWorld w = standardWorld();
    CHECK(n.hits.judge(goodHit(1), 1, 0, kNow, w).apply);
    DeathDecl d;
    d.deathId = 1; d.who = Entity::actor(100); d.source = DeathSource::Owner; d.declarer = 0; d.epoch = 3;
    CHECK(n.deaths.judge(d, 0, true, w).accept);
    CHECK(n.deaths.isDead(Entity::actor(100)));
    n.reset();
    CHECK(!n.deaths.isDead(Entity::actor(100)));
    CHECK(n.hits.judge(goodHit(1), 1, 0, kNow, w).apply);   // the same hit id is new again in the new world
    CombatRules r = defaultRules();
    r.maxDamage = 5;
    n.setRules(r);
    CHECK(n.hits.rules().maxDamage == 5);
}

TEST(wire, an_entity_is_its_net_id_on_the_wire) {   // S7: players are playerNet(slot), the reserved part of 0xFE.. is refused
    for (PlayerId s = 0; s < kMaxSlots; ++s) {
        CHECK(Entity::fromNet(Entity::player(s).netId()) == Entity::player(s));
        CHECK(Entity::player(s).slot() == s && Entity::player(s).netId() == fnvcoop::live::playerNet(s));
    }
    CHECK(Entity::fromNet(0).none() && Entity::fromNet(0x0012ABCD) == Entity::actor(0x0012ABCD) && Entity::fromNet(0xFF000010u) == Entity::actor(0xFF000010u));
    CHECK(Entity::actor(77).slot() == kNoPlayer);
    for (const uint32_t bad : {0xFE000100u, 0xFE001234u, 0xFE000009u}) {
        Archive w = Archive::writer();
        uint32_t n = bad;
        w.varu32(n);
        std::vector<uint8_t> b = w.finish();
        Archive r = Archive::reader(b.data(), b.size());
        Entity e;
        io(r, e);
        CHECK_MSG(!r.ok(), std::to_string(bad));
    }
}
