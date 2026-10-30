// Game-free tests of the live sync parity checker. Twins 1800..1827 (FNVCOOP_MUT=<n>): every one of them must make a test here fail.
//   wire 1800 1801 1802 1803 | anim 1808 | config 1814 | compare 1804 1805 1806 1807 1817 1819 1820 1821 | reporter 1809 1810 1811
//   sampler 1812 1813 1816 1822 1823 | file 1815 | service 1818 1824 1825 1826 1827
// (1828..1829 are free.)
#include <algorithm>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <filesystem>
#include <fstream>
#include <map>
#include <memory>
#include <sstream>

#include "common/minijson.h"
#include "fnvcoop/transport_util.h"
#include "fnvcoop/wire.h"
#include "minitest.h"
#include "parity/parity_anim.h"
#include "parity/parity_compare.h"
#include "parity/parity_config.h"
#include "parity/parity_json.h"
#include "parity/parity_sampler.h"
#include "parity/parity_service.h"
#include "parity/parity_wire.h"

using namespace nvcoop::parity;
namespace js = nvcoop::json;

namespace {

// ---- a sample from the JSON shape of a line of nvcoop_parity.jsonl (the same shape tools/parity/vectors.json uses) ------------------------------------------

uint32_t hexOf(const js::Value& v) { return static_cast<uint32_t>(std::strtoul(v.asString("0").c_str(), nullptr, 16)); }

Sample fromJson(const js::Value& o) {
    Sample s;
    s.slot = static_cast<uint8_t>(o.get("slot").asInt(255));
    s.seq = static_cast<uint32_t>(o.get("seq").asInt(0));
    s.ms = static_cast<uint64_t>(o.get("ms").asInt(0));
    s.cell = hexOf(o.get("cell"));
    s.ws = hexOf(o.get("ws"));
    s.rad = static_cast<uint32_t>(o.get("rad").asInt(0));
    s.cov = static_cast<uint32_t>(o.get("cov").asInt(0));
    s.menu = o.get("menu").asInt(0) != 0;
    for (const js::Value& e : o.get("e").items()) {
        Ent x;
        const std::string t = e.get("t").asString("n");
        x.kind = t == "p" ? EntKind::Puppet : t == "s" ? EntKind::Self : EntKind::Npc;
        x.slot = x.kind == EntKind::Npc ? kNoSlot : static_cast<uint8_t>(e.get("sl").asInt(0));
        x.ref = hexOf(e.get("r"));
        x.base = hexOf(e.get("b"));
        x.x = static_cast<int32_t>(e.get("x").asInt());
        x.y = static_cast<int32_t>(e.get("y").asInt());
        x.z = static_cast<int32_t>(e.get("z").asInt());
        x.speed16 = static_cast<uint8_t>(e.get("sp").asInt() / 16);
        x.seated = e.get("se").asInt() != 0;
        x.dead = e.get("d").asInt() != 0;
        x.disabled = e.get("dis").asInt() != 0;
        x.weaponOut = e.get("w").asInt() != 0;
        const int64_t hp = e.get("hp").asInt(-1);
        x.hp = hp < 0 ? kNoHp : static_cast<uint8_t>(hp);
        const int64_t g = e.get("g").asInt(-1);
        x.group = g < 0 ? kNoGroup : static_cast<uint8_t>(g);
        x.acts = static_cast<uint32_t>(e.get("ac").asInt(0));
        if (const js::Value* lk = e.find("look")) {
            x.look.known = true;
            x.look.race = hexOf(lk->get("race"));
            x.look.female = lk->get("f").asInt() != 0;
            x.look.hair = hexOf(lk->get("hair"));
            x.look.eyes = hexOf(lk->get("eyes"));
            x.look.faceHash = hexOf(lk->get("fh"));
        }
        s.ents.push_back(x);
    }
    return s;
}

js::Value loadVectors() {
    std::ifstream f(PARITY_VECTORS_PATH, std::ios::binary);
    std::stringstream ss;
    ss << f.rdbuf();
    js::Value v;
    std::string err;
    if (!js::parse(ss.str(), v, err)) {
        std::printf("    vectors.json: %s\n", err.c_str());
        return js::Value();
    }
    return v;
}

// ---- small builders -----------------------------------------------------------------------------------------------------------------------------------------

Ent npc(uint32_t ref, int x = 0, int y = 0, int z = 0) {
    Ent e;
    e.kind = EntKind::Npc;
    e.ref = ref;
    e.x = x; e.y = y; e.z = z;
    e.hp = 5;
    e.group = 0;
    return e;
}

Ent playerEnt(EntKind k, uint8_t slot, int x = 0, int y = 0) {
    Ent e;
    e.kind = k;
    e.slot = slot;
    e.ref = k == EntKind::Self ? 0x14u : 0x200000u + slot;
    e.x = x; e.y = y;
    e.hp = 10;
    e.group = 0;
    e.look = Look{true, 0x19, false, 0x1000, 0x2000, 0xAABBCCDDu};
    return e;
}

Sample sampleOf(uint8_t slot, std::vector<Ent> ents, uint32_t cov = 6000) {
    Sample s;
    s.slot = slot;
    s.cell = 0x1234;
    s.rad = 6000;
    s.cov = cov;
    s.ents = std::move(ents);
    return s;
}

// host = slot 0, guest = slot 1; each has its own player and the other's puppet
Sample hostSample(std::vector<Ent> npcs = {}, uint32_t cov = 6000) {
    npcs.push_back(playerEnt(EntKind::Puppet, 1, 100));
    npcs.push_back(playerEnt(EntKind::Self, 0));
    return sampleOf(0, std::move(npcs), cov);
}
Sample guestSample(std::vector<Ent> npcs = {}, uint32_t cov = 6000) {
    npcs.push_back(playerEnt(EntKind::Self, 1, 100));
    npcs.push_back(playerEnt(EntKind::Puppet, 0));
    return sampleOf(1, std::move(npcs), cov);
}

std::vector<std::string> ids(const CompareResult& r) {
    std::vector<std::string> v;
    for (const Mismatch& m : r.confirmed) v.push_back(m.kind + "|" + m.key);
    std::sort(v.begin(), v.end());
    return v;
}
bool has(const CompareResult& r, const std::string& id) {
    for (const std::string& s : ids(r)) if (s == id) return true;
    return false;
}

}  // namespace

// ---- animation classes, health, tolerance: the shared vectors ----------------------------------------------------------------------------------------------

TEST(parity_anim, group_classes_match_the_shared_vectors) {
    const js::Value v = loadVectors();
    CHECK(!v.get("group_class").items().empty());
    for (const js::Value& row : v.get("group_class").items()) {
        const int g = static_cast<int>(row.items()[0].asInt());
        CHECK_MSG(std::string(className(groupClass(g))) == row.items()[1].asString(), "group class");
    }
}

TEST(parity_anim, every_group_id_has_the_class_the_python_comparer_gives) {
    const js::Value v = loadVectors();
    CHECK(v.get("group_class_all").items().size() == 256);
    for (int g = 0; g < 256 && g < static_cast<int>(v.get("group_class_all").items().size()); ++g)
        CHECK_MSG(std::string(className(groupClass(g))) == v.get("group_class_all").items()[static_cast<size_t>(g)].asString(), "group class of an id");
}

TEST(parity_anim, the_primary_group_matches_the_shared_vectors) {
    const js::Value v = loadVectors();
    CHECK(!v.get("primary_group").items().empty());
    for (const js::Value& row : v.get("primary_group").items()) {
        std::vector<int> g;
        for (const js::Value& x : row.items()[0].items()) g.push_back(static_cast<int>(x.asInt()));
        CHECK_MSG(primaryGroup(g) == static_cast<uint8_t>(row.items()[1].asInt()), "primary group");
    }
}

TEST(parity_anim, run_outranks_walk_in_one_animation_set) {
    CHECK(primaryGroup({3, 7}) == 7);
    CHECK(primaryGroup({7, 3}) == 7);
}

TEST(parity_anim, health_buckets_match_the_shared_vectors) {
    const js::Value v = loadVectors();
    CHECK(!v.get("hp_bucket").items().empty());
    for (const js::Value& row : v.get("hp_bucket").items())
        CHECK_MSG(hpBucket(true, static_cast<float>(row.items()[0].asNumber())) == static_cast<uint8_t>(row.items()[1].asInt()), "hp bucket");
    CHECK(hpBucket(false, 100) == kNoHp);
}

TEST(parity_compare, the_position_tolerance_matches_the_shared_vectors) {
    const js::Value v = loadVectors();
    CHECK(!v.get("pos_tol").items().empty());
    Tolerances t;
    for (const js::Value& row : v.get("pos_tol").items())
        CHECK_MSG(std::fabs(posTolerance(t, static_cast<uint8_t>(row.get("a").asInt()), static_cast<uint8_t>(row.get("b").asInt())) - row.get("tol").asNumber()) < 0.01, "tolerance");
}

TEST(parity_compare, speed_widens_the_position_tolerance) {
    Tolerances t;
    CHECK(posTolerance(t, 0, 0) == 210.0);
    CHECK(posTolerance(t, 19, 19) > 500.0);
    CHECK(posTolerance(t, 19, 0) < posTolerance(t, 19, 19));
}

TEST(parity_compare, every_shared_case_gives_the_shared_answer) {
    const js::Value v = loadVectors();
    CHECK(v.get("cases").items().size() >= 20);
    for (const js::Value& c : v.get("cases").items()) {
        Comparer cmp;
        int stepNo = 0;
        for (const js::Value& st : c.get("steps").items()) {
            const Sample h = fromJson(st.get("host")), g = fromJson(st.get("client"));
            const CompareResult r = cmp.compare(h, g, static_cast<uint64_t>(st.get("t").asInt()));
            std::vector<std::string> want;
            for (const js::Value& e : st.get("expect").items()) want.push_back(e.asString());
            std::sort(want.begin(), want.end());
            const bool ok = ids(r) == want && r.skipped == st.get("skipped").asBool(false);
            if (!ok) {
                std::printf("    case %s step %d: expected %zu differences, got %zu%s\n", c.get("name").asString().c_str(), stepNo, want.size(), r.confirmed.size(), r.skipped ? " (skipped)" : "");
                for (const Mismatch& m : r.confirmed) std::printf("      got %s %s host=%s client=%s\n", m.kind.c_str(), m.key.c_str(), m.host.c_str(), m.client.c_str());
            }
            CHECK(ok);
            ++stepNo;
        }
    }
}

// ---- compare: the rules one by one (the shared cases above cover the same ground in both languages; these pin each twin) --------------------------------------

TEST(parity_compare, identical_views_have_no_differences_and_count_their_references) {
    Comparer c;
    const CompareResult r = c.compare(hostSample({npc(0x100), npc(0x101)}), guestSample({npc(0x100), npc(0x101)}), 0);
    CHECK(r.confirmed.empty());
    CHECK(r.refs == 4);   // two actors and two players
    CHECK(r.players == 2);
    CHECK(r.refsBad == 0);
}

TEST(parity_compare, a_difference_is_reported_only_after_it_has_lasted) {
    Comparer c;
    Ent a = npc(0x100), b = npc(0x100);
    b.seated = true;
    CHECK(c.compare(hostSample({a}), guestSample({b}), 0).confirmed.empty());
    CHECK(c.compare(hostSample({a}), guestSample({b}), 2900).confirmed.empty());
    CHECK(has(c.compare(hostSample({a}), guestSample({b}), 3000), "seated|0x00000100"));
}

TEST(parity_compare, seated_is_compared) {
    Comparer c;
    Ent a = npc(0x100), b = npc(0x100);
    a.seated = true;
    c.compare(hostSample({a}), guestSample({b}), 0);
    CHECK(has(c.compare(hostSample({a}), guestSample({b}), 4000), "seated|0x00000100"));
}

TEST(parity_compare, a_moving_actor_gets_a_wider_position_tolerance) {
    Comparer c;
    Ent a = npc(0x100, 0), b = npc(0x100, 500);
    a.speed16 = b.speed16 = 19;
    c.compare(hostSample({a}), guestSample({b}), 0);
    CHECK(!has(c.compare(hostSample({a}), guestSample({b}), 4000), "pos|0x00000100"));
    Comparer still;
    a.speed16 = b.speed16 = 0;
    still.compare(hostSample({a}), guestSample({b}), 0);
    CHECK(has(still.compare(hostSample({a}), guestSample({b}), 4000), "pos|0x00000100"));
}

TEST(parity_compare, a_big_position_gap_is_reported_at_once) {
    Comparer c;
    CHECK(has(c.compare(hostSample({npc(0x100, 0)}), guestSample({npc(0x100, 2500)}), 0), "pos|0x00000100"));
}

TEST(parity_compare, a_difference_that_stops_is_forgotten) {
    Comparer c;
    Ent a = npc(0x100), b = npc(0x100);
    b.seated = true;
    c.compare(hostSample({a}), guestSample({b}), 0);
    c.compare(hostSample({a}), guestSample({a}), 1500);
    CHECK(c.compare(hostSample({a}), guestSample({b}), 3000).confirmed.empty());
    CHECK(has(c.compare(hostSample({a}), guestSample({b}), 6000), "seated|0x00000100"));
}

TEST(parity_compare, the_face_hash_is_part_of_the_look) {
    Comparer c;
    Sample h = hostSample(), g = guestSample();
    for (Ent& e : h.ents) if (e.kind == EntKind::Puppet) e.look.faceHash ^= 0xFF;
    c.compare(h, g, 0);
    CHECK(has(c.compare(h, g, 4000), "look|player1"));
}

TEST(parity_compare, a_puppet_with_no_look_is_a_difference) {
    Comparer c;
    Sample h = hostSample(), g = guestSample();
    for (Ent& e : h.ents) if (e.kind == EntKind::Puppet) e.look = Look{};
    c.compare(h, g, 0);
    CHECK(has(c.compare(h, g, 4000), "look|player1"));
}

TEST(parity_compare, an_actor_the_other_machine_should_have_is_absent_only_inside_its_coverage) {
    Comparer c;
    c.compare(hostSample({npc(0x100, 1000)}), guestSample({}), 0);
    CHECK(has(c.compare(hostSample({npc(0x100, 1000)}), guestSample({}), 4000), "absent|0x00000100"));
    Comparer far;
    far.compare(hostSample({npc(0x100, 1000)}), guestSample({}, 500), 0);
    CHECK(far.compare(hostSample({npc(0x100, 1000)}), guestSample({}, 500), 4000).confirmed.empty());
}

TEST(parity_compare, one_shot_animations_are_not_compared_with_a_locomotion_group) {
    Comparer c;
    Ent a = npc(0x100), b = npc(0x100);
    a.group = 26;   // an attack
    b.group = 0;
    c.compare(hostSample({a}), guestSample({b}), 0);
    CHECK(c.compare(hostSample({a}), guestSample({b}), 4000).confirmed.empty());
    b.group = 3;
    a.group = 0;
    Comparer d;
    d.compare(hostSample({a}), guestSample({b}), 0);
    CHECK(has(d.compare(hostSample({a}), guestSample({b}), 4000), "anim|0x00000100"));
}

TEST(parity_compare, health_buckets_two_apart_differ_and_one_apart_do_not) {
    Comparer c;
    Ent a = npc(0x100), b = npc(0x100);
    a.hp = 10;
    b.hp = 12;
    c.compare(hostSample({a}), guestSample({b}), 0);
    CHECK(has(c.compare(hostSample({a}), guestSample({b}), 4000), "hp|0x00000100"));
    b.hp = 11;
    Comparer d;
    d.compare(hostSample({a}), guestSample({b}), 0);
    CHECK(d.compare(hostSample({a}), guestSample({b}), 4000).confirmed.empty());
}

TEST(parity_compare, created_actors_menus_and_other_places_are_not_compared) {
    Comparer c;
    CHECK(c.compare(hostSample({npc(0xFF000010, 0)}), guestSample({}), 4000).confirmed.empty());
    Sample h = hostSample({npc(0x100, 0)}), g = guestSample({npc(0x100, 5000)});
    h.menu = true;
    CHECK(c.compare(h, g, 0).skipped);
    h.menu = false;
    g.cell = 0x9999;
    CHECK(c.compare(h, g, 0).skipped);
}

// ---- reporter ---------------------------------------------------------------------------------------------------------------------------------------------

namespace {
CompareResult oneDiff(const std::string& kind, const std::string& key) {
    CompareResult r;
    r.confirmed.push_back(Mismatch{kind, key, "a", "b"});
    r.keysCompared = {key};
    r.keysBad = {key};
    r.refs = r.refsBad = 1;
    return r;
}
}  // namespace

TEST(parity_reporter, one_line_per_reference_and_kind_per_window) {
    Reporter r;
    CHECK(r.report(oneDiff("pos", "0x00000100"), 0).size() == 1);
    CHECK(r.report(oneDiff("pos", "0x00000100"), 4000).empty());
    CHECK(r.report(oneDiff("pos", "0x00000100"), 9999).empty());
    CHECK(r.report(oneDiff("pos", "0x00000100"), 10000).size() == 1);
    CHECK(r.suppressed() == 2);
}

TEST(parity_reporter, another_kind_on_the_same_reference_is_its_own_line) {
    Reporter r;
    CHECK(r.report(oneDiff("pos", "0x00000100"), 0).size() == 1);
    CHECK(r.report(oneDiff("seated", "0x00000100"), 100).size() == 1);
}

TEST(parity_reporter, the_line_has_the_documented_shape) {
    Reporter r;
    const auto lines = r.report(oneDiff("seated", "0x00000100"), 0);
    CHECK(lines.size() == 1);
    CHECK(lines[0] == "DESYNC seated ref 0x00000100 host=a client=b");
}

TEST(parity_reporter, the_summary_counts_references_not_differences) {
    Reporter r;
    CompareResult c;
    c.keysCompared = {"0x1", "0x2", "0x3"};
    c.keysBad = {"0x2"};
    c.confirmed.push_back(Mismatch{"pos", "0x2", "", ""});
    c.confirmed.push_back(Mismatch{"seated", "0x2", "", ""});
    r.report(c, 0);
    CHECK(!r.summaryDue(29999));
    CHECK(r.summaryDue(30000));
    CHECK(r.summary(30000) == "summary 3 refs compared, 1 mismatching");
    CHECK(!r.summaryDue(30001));
}

// ---- config -----------------------------------------------------------------------------------------------------------------------------------------------

namespace {
IniGetter iniOf(std::map<std::string, std::string> kv) {
    return [kv](const char*, const char* key, const char* def) { auto it = kv.find(key); return it == kv.end() ? std::string(def) : it->second; };
}
}  // namespace

TEST(parity_config, off_by_default_with_the_documented_numbers) {
    const Settings s = readSettings(iniOf({}));
    CHECK(!s.enabled);
    CHECK(s.intervalMs == 2000);
    CHECK(s.cap == 40);
    CHECK(s.tol.persistMs == 3000);
    CHECK(s.tol.logEveryMs == 10000);
    CHECK(s.tol.summaryMs == 30000);
    CHECK(s.file == "nvcoop_parity.jsonl");
}

TEST(parity_config, enabled_and_numbers_are_read_and_out_of_range_values_are_clamped) {
    Settings s = readSettings(iniOf({{"enabled", "1"}, {"cap", "9999"}, {"intervalMs", "5"}, {"posTolUnits", "abc"}}));
    CHECK(s.enabled);
    CHECK(s.cap == 60);
    CHECK(s.intervalMs == 500);
    CHECK(s.tol.posTolUnits == 210);
    CHECK(!s.warnings.empty());
    s = readSettings(iniOf({{"cap", "10"}, {"netCap", "30"}}));
    CHECK(s.netCap == 10);   // never more than the file sample
}

// ---- wire -------------------------------------------------------------------------------------------------------------------------------------------------

namespace {
Sample wireSample() {
    Sample s = guestSample({npc(0x104A12, -1500, 220, -30), npc(0x104A13, 70000, -90000, 400)});
    s.seq = 77;
    s.cell = 0x1234;
    s.ws = 0xA1B2;
    s.cov = 4321;
    s.menu = true;
    s.ents[0].seated = true;
    s.ents[0].speed16 = 12;
    s.ents[0].hp = 7;
    s.ents[0].group = 3;
    s.ents[1].dead = true;
    s.ents[1].disabled = true;
    for (Ent& e : s.ents) if (e.kind != EntKind::Npc) e.acts = 300;
    return s;
}
bool sameEnt(const Ent& a, const Ent& b) {
    return a.kind == b.kind && a.slot == b.slot && a.ref == b.ref && a.x == b.x && a.y == b.y && a.z == b.z && a.speed16 == b.speed16 && a.seated == b.seated && a.dead == b.dead &&
           a.disabled == b.disabled && a.hp == b.hp && a.group == b.group && a.acts == b.acts && a.look == b.look;
}
}  // namespace

TEST(parity_wire, a_sample_survives_the_round_trip_including_negative_coordinates) {
    const Sample s = wireSample();
    const fnvcoop::Bytes b = encodeSample(s);
    CHECK(b[0] == kChannelParity);
    Sample o;
    CHECK(decodeSample(b.data(), b.size(), o));
    CHECK(o.slot == s.slot && o.seq == 77 && o.cell == 0x1234 && o.ws == 0xA1B2 && o.cov == 4321 && o.menu);
    CHECK(o.ents.size() == s.ents.size());
    for (size_t i = 0; i < s.ents.size() && i < o.ents.size(); ++i) CHECK(sameEnt(s.ents[i], o.ents[i]));
}

TEST(parity_wire, the_look_travels) {
    const Sample s = wireSample();
    const fnvcoop::Bytes b = encodeSample(s);
    Sample o;
    CHECK(decodeSample(b.data(), b.size(), o));
    const Ent* self = o.bySlot(1);
    CHECK(self && self->look.known && self->look.hair == 0x1000 && self->look.faceHash == 0xAABBCCDDu);
}

TEST(parity_wire, a_changed_byte_is_rejected_by_the_checksum) {
    fnvcoop::Bytes b = encodeSample(wireSample());
    b[b.size() / 2] ^= 0x10;
    Sample o;
    CHECK(!decodeSample(b.data(), b.size(), o));
}

TEST(parity_wire, extra_bytes_wrong_channel_and_cut_datagrams_are_rejected) {
    const fnvcoop::Bytes good = encodeSample(wireSample());
    Sample o;
    fnvcoop::Bytes longer(good.begin(), good.end() - 4);
    longer.push_back(0);   // one byte too many inside the checksummed part
    const uint32_t crc = fnvcoop::crc32(longer.data(), longer.size());
    for (int i = 0; i < 4; ++i) longer.push_back(static_cast<uint8_t>(crc >> (8 * i)));
    CHECK(!decodeSample(longer.data(), longer.size(), o));
    fnvcoop::Bytes wrong = good;
    wrong[0] = 0xC8;
    CHECK(!decodeSample(wrong.data(), wrong.size(), o));
    CHECK(!decodeSample(good.data(), good.size() - 5, o));
    CHECK(!decodeSample(good.data(), 2, o));
    CHECK(!decodeSample(nullptr, 0, o));
}

TEST(parity_wire, a_full_sample_is_a_few_hundred_bytes) {
    std::vector<Ent> npcs;
    for (uint32_t i = 0; i < 30; ++i) npcs.push_back(npc(0x104000 + i, 3000 + int(i) * 20, -2000, 100));
    const fnvcoop::Bytes b = encodeSample(guestSample(npcs));
    CHECK(b.size() > 150);
    CHECK(b.size() < 900);
}

// ---- sampler ----------------------------------------------------------------------------------------------------------------------------------------------

namespace {
class FakeSource : public IParitySource {
public:
    bool up = true;
    RawPlayer pl;
    std::vector<RawActor> list;
    std::map<uint32_t, RawState> states;
    std::map<uint32_t, RawLook> looks;
    std::vector<std::pair<uint8_t, uint32_t>> pups;
    std::map<uint32_t, std::vector<int>> playing;   // groups returned by groups()
    uint8_t slot = 0;
    bool ready() override { return up; }
    bool player(RawPlayer& o) override { o = pl; return up; }
    void actors(std::vector<RawActor>& out) override { out = list; }
    bool actor(uint32_t ref, RawActor& o) override {
        for (const RawActor& a : list) if (a.ref == ref) { o = a; return true; }
        return false;
    }
    bool groups(uint32_t ref, bool, std::vector<int>& out) override {
        auto it = playing.find(ref);
        out = it == playing.end() ? std::vector<int>{} : it->second;
        return true;
    }
    bool state(uint32_t ref, bool, RawState& o) override {
        auto it = states.find(ref);
        o = it == states.end() ? RawState{} : it->second;
        if (it == states.end()) { o.ok = true; o.hpOk = true; o.hp = 100; o.groups = {0}; }
        return true;
    }
    bool look(uint32_t ref, RawLook& o) override {
        auto it = looks.find(ref);
        if (it == looks.end()) return false;
        o = it->second;
        return true;
    }
    void puppets(std::vector<std::pair<uint8_t, uint32_t>>& out) override { out = pups; }
    uint8_t localSlot() override { return slot; }
    double gameTime() override { return 12.5; }
};

RawActor ra(uint32_t ref, float x, float y = 0, float z = 0) {
    RawActor a;
    a.ref = ref; a.x = x; a.y = y; a.z = z;
    return a;
}
}  // namespace

TEST(parity_sampler, lists_the_nearest_actors_first_and_caps_them) {
    Settings st;
    st.cap = 40;
    FakeSource src;
    for (uint32_t i = 0; i < 50; ++i) src.list.push_back(ra(0x1000 + i, 100.0f + float(i) * 10.0f));
    Sampler sm(st, src);
    Sample s;
    CHECK(sm.take(1000, 5, s));
    size_t npcs = 0;
    for (const Ent& e : s.ents) if (e.kind == EntKind::Npc) ++npcs;
    CHECK(npcs == 40);
    CHECK(s.ents[0].ref == 0x1000);
    CHECK(s.ents[1].ref == 0x1001);
    CHECK(s.cov == 500);   // the 41st actor (index 40) is at x = 500: complete only that far
    CHECK(s.ents.back().kind == EntKind::Self);
    CHECK(s.seq == 1 && s.ms == 1000 && s.wallMs == 5);
}

TEST(parity_sampler, actors_beyond_the_radius_and_the_players_are_left_out) {
    Settings st;
    st.radius = 1000;
    FakeSource src;
    src.list = {ra(0x14, 0), ra(0x201, 50), ra(0x300, 900), ra(0x301, 1500)};
    src.pups = {{1, 0x201}};
    Sampler sm(st, src);
    Sample s;
    CHECK(sm.take(0, 0, s));
    CHECK(s.byRef(0x300) != nullptr);
    CHECK(s.byRef(0x301) == nullptr);
    CHECK(s.byRef(0x14) == nullptr);
    CHECK(s.byRef(0x201) == nullptr);   // a puppet is a player entity, not an actor entry
    CHECK(s.bySlot(1) && s.bySlot(1)->kind == EntKind::Puppet && s.bySlot(1)->ref == 0x201);
    CHECK(s.bySlot(0) && s.bySlot(0)->kind == EntKind::Self);
}

TEST(parity_sampler, reads_seated_dead_health_group_and_look) {
    Settings st;
    FakeSource src;
    src.list = {ra(0x300, 100)};
    RawState a;
    a.ok = true; a.seatedOk = true; a.seated = true; a.hpOk = true; a.hp = 60; a.dead = false; a.groups = {0, 7, 3};
    src.states[0x300] = a;
    src.looks[0x14] = RawLook{true, Look{true, 0x19, true, 1, 2, 3}, "Courier"};
    Sampler sm(st, src);
    Sample s;
    CHECK(sm.take(0, 0, s));
    const Ent* e = s.byRef(0x300);
    CHECK(e && e->seated && !e->dead && e->hp == hpBucket(true, 60) && e->group == 7);
    const Ent* self = s.bySlot(0);
    CHECK(self && self->look.known && self->look.female && self->look.hair == 1);
}

TEST(parity_sampler, an_unready_world_gives_no_sample) {
    Settings st;
    FakeSource src;
    src.up = false;
    Sampler sm(st, src);
    Sample s;
    CHECK(!sm.take(0, 0, s));
}

TEST(parity_sampler, speed_is_measured_over_at_least_most_of_a_second) {
    Settings st;
    FakeSource src;
    src.list = {ra(0x300, 100)};
    Sampler sm(st, src);
    Sample s;
    sm.take(0, 0, s);
    CHECK(s.byRef(0x300)->speed16 == 0);   // nothing to measure against yet
    src.list = {ra(0x300, 300)};           // 200 units in 2 s = 100 units/s = 6 steps of 16
    sm.take(2000, 0, s);
    CHECK(s.byRef(0x300)->speed16 == 6);
    src.list = {ra(0x300, 300)};
    sm.take(2005, 0, s);                   // taken a few ms after the last one (the host's sample on arrival): still measured against the older point
    CHECK(s.byRef(0x300)->speed16 == 6);
}

TEST(parity_sampler, attack_starts_are_counted_once_each) {
    Settings st;
    FakeSource src;
    src.pups = {{1, 0x201}};
    src.list = {ra(0x201, 50)};
    Sampler sm(st, src);
    uint64_t t = 1000;
    auto frames = [&](std::vector<int> g, int n) {
        src.playing[0x14] = g;
        src.playing[0x201] = g;
        for (int i = 0; i < n; ++i) { t += 40; sm.frame(t); }
    };
    frames({0}, 3);
    frames({0, 26}, 5);   // a punch, 200 ms long
    frames({0}, 3);
    frames({0, 26}, 4);   // another one
    frames({0}, 2);
    CHECK(sm.attacksOf(0) == 2);   // the local player (slot 0)
    CHECK(sm.attacksOf(1) == 2);   // the puppet of slot 1
    Sample s;
    CHECK(sm.take(t, 0, s));
    CHECK(s.bySlot(1)->acts == 2 && s.bySlot(0)->acts == 2);
}

TEST(parity_sampler, the_frame_watcher_is_throttled) {
    Settings st;
    FakeSource src;
    Sampler sm(st, src);
    src.playing[0x14] = {26};
    sm.frame(1000);
    src.playing[0x14] = {0};
    sm.frame(1005);   // too soon: ignored
    src.playing[0x14] = {26};
    sm.frame(1040);
    CHECK(sm.attacksOf(0) == 1);   // the second 26 is the same punch (the 0 in between was never seen)
}

TEST(parity_sampler, the_net_sample_keeps_the_nearest_actors_and_narrows_the_coverage) {
    Settings st;
    FakeSource src;
    for (uint32_t i = 0; i < 40; ++i) src.list.push_back(ra(0x1000 + i, 100.0f + float(i) * 10.0f));
    Sampler sm(st, src);
    Sample s;
    sm.take(0, 0, s);
    const Sample n = netSample(s, 30);
    size_t npcs = 0;
    for (const Ent& e : n.ents) if (e.kind == EntKind::Npc) ++npcs;
    CHECK(npcs == 30);
    CHECK(n.cov == 400);   // the 31st actor is at x = 400
    CHECK(n.bySlot(0) != nullptr);
    CHECK(netSample(s, 40).cov == s.cov);
}

// ---- json and the file ------------------------------------------------------------------------------------------------------------------------------------

TEST(parity_json, a_line_parses_back_to_the_same_sample_and_carries_the_pairing_link) {
    const js::Value v = loadVectors();
    const js::Value& c = v.get("cases").items()[1];   // pos_needs_persistence
    const Sample s = fromJson(c.get("steps").items()[0].get("host"));
    LineMeta m;
    m.role = "host";
    m.why = "cmp";
    m.cmpSlot = 1;
    m.cmpSeq = 9;
    const std::string line = sampleJson(s, m);
    CHECK(line.find('\n') == std::string::npos);
    js::Value o;
    std::string err;
    CHECK(js::parse(line, o, err));
    CHECK(o.get("k").asString() == "s" && o.get("role").asString() == "host" && o.get("why").asString() == "cmp");
    CHECK(o.get("cmp").get("slot").asInt() == 1 && o.get("cmp").get("seq").asInt() == 9);
    const Sample back = fromJson(o);
    CHECK(back.ents.size() == s.ents.size());
    for (size_t i = 0; i < s.ents.size() && i < back.ents.size(); ++i) {
        Ent a = s.ents[i], b = back.ents[i];
        a.base = b.base = 0;
        CHECK(sameEnt(a, b));
    }
    CHECK(o.get("e").items()[0].get("gn").asString() == nvcoop::actors::anim::groupName(s.ents[0].group));
}

TEST(parity_json, the_line_the_plugin_writes_is_the_line_in_the_shared_vectors) {
    const js::Value v = loadVectors();
    const js::Value& pl = v.get("plugin_line");
    const Sample s = fromJson(pl.get("sample"));
    LineMeta m;
    m.role = "host";
    m.why = "cmp";
    m.cmpSlot = static_cast<int>(pl.get("meta").get("cmpSlot").asInt());
    m.cmpSeq = pl.get("meta").get("cmpSeq").asInt();
    const std::string line = sampleJson(s, m);
    if (line != pl.get("line").asString()) std::printf("    actual line: %s\n", line.c_str());
    CHECK(!pl.get("line").asString().empty());
    CHECK(line == pl.get("line").asString());
}

TEST(parity_file, the_file_rotates_at_its_size_and_keeps_one_older_file) {
    namespace fs = std::filesystem;
    const fs::path dir = fs::temp_directory_path();
    const std::string path = (dir / "nvcoop_parity_test_rotation.jsonl").string();
    std::remove(path.c_str());
    std::remove((path + ".1").c_str());
    {
        RotatingFile f;
        f.open(path, 300);
        const std::string line(99, 'x');
        for (int i = 0; i < 10; ++i) CHECK(f.append(line));
        CHECK(f.rotations() >= 2);
        CHECK(f.bytes() <= 300);
    }
    CHECK(fs::exists(path + ".1"));
    CHECK(fs::file_size(path) <= 300);
    std::remove(path.c_str());
    std::remove((path + ".1").c_str());
}

TEST(parity_file, a_new_start_moves_the_old_file_aside) {
    namespace fs = std::filesystem;
    const std::string path = (fs::temp_directory_path() / "nvcoop_parity_test_roll.jsonl").string();
    std::remove(path.c_str());
    std::remove((path + ".1").c_str());
    {
        RotatingFile f;
        f.open(path, 100000, true);
        f.append("first run");
    }
    {
        RotatingFile f;
        f.open(path, 100000, true);
        f.append("second run");
    }
    CHECK(fs::exists(path + ".1"));
    std::ifstream a(path), b(path + ".1");
    std::string la, lb;
    std::getline(a, la);
    std::getline(b, lb);
    CHECK(la == "second run" && lb == "first run");
    std::remove(path.c_str());
    std::remove((path + ".1").c_str());
}

// ---- the service, end to end over the in-memory network ---------------------------------------------------------------------------------------------------

namespace {

struct Rig {
    fnvcoop::InMemoryNetwork net{1};
    std::unique_ptr<fnvcoop::MuxTransport> hostMux, guestMux;
    FakeSource hostSrc, guestSrc;
    std::vector<std::string> hostLog, guestLog;
    std::unique_ptr<ParityService> host, guest;
    uint64_t now = 100000;
    Settings st;
    std::string hostFile;

    explicit Rig(Settings s = Settings(), const std::string& file = "") : st(s), hostFile(file) {
        st.enabled = true;
        hostMux.reset(new fnvcoop::MuxTransport(*net.endpoint(0)));
        guestMux.reset(new fnvcoop::MuxTransport(*net.endpoint(1)));
        hostSrc.slot = 0;
        guestSrc.slot = 1;
        hostSrc.pl.cell = guestSrc.pl.cell = 0x1234;
        guestSrc.pl.x = 100;
        hostSrc.pups = {{1, 0x201001}};    // each machine has a copy of the other's player, standing where that player is
        guestSrc.pups = {{0, 0x201000}};
        setLists({}, {});
        host.reset(new ParityService(st, hostSrc, [this](const std::string& l) { hostLog.push_back(l); }, file));
        guest.reset(new ParityService(st, guestSrc, [this](const std::string& l) { guestLog.push_back(l); }, ""));
    }

    void setLists(std::vector<RawActor> h, std::vector<RawActor> g) {
        h.push_back(ra(0x201001, 100));
        g.push_back(ra(0x201000, 0));
        hostSrc.list = std::move(h);
        guestSrc.list = std::move(g);
    }
    FrameInfo hostInfo() {
        FrameInfo f;
        f.active = true; f.isHost = true; f.self = 0; f.mux = hostMux.get(); f.nowMs = now; f.wallMs = now;
        f.clients.push_back(Link{1, 1, true});
        return f;
    }
    FrameInfo guestInfo() {
        FrameInfo f;
        f.active = true; f.isHost = false; f.self = 1; f.hostLinkUp = true; f.mux = guestMux.get(); f.nowMs = now; f.wallMs = now;
        return f;
    }
    void pump() {
        net.advance(now);
        fnvcoop::PeerId from;
        fnvcoop::Bytes data;
        while (hostMux->poll(from, data)) {}
        while (guestMux->poll(from, data)) {}
    }
    void run(uint64_t ms) {
        const uint64_t end = now + ms;
        while (now < end) {
            now += 100;
            pump();
            host->frame(hostInfo());
            guest->frame(guestInfo());
        }
    }
    bool hostSaid(const std::string& needle) const {
        for (const std::string& l : hostLog) if (l.find(needle) != std::string::npos) return true;
        return false;
    }
};

void seatedMismatch(Rig& r) {
    r.setLists({ra(0x300, 200)}, {ra(0x300, 200)});
    RawState a;
    a.ok = true; a.hpOk = true; a.hp = 100; a.seatedOk = true; a.seated = false; a.groups = {0};
    RawState b = a;
    b.seated = true;
    r.hostSrc.states[0x300] = a;
    r.guestSrc.states[0x300] = b;
}

}  // namespace

TEST(parity_service, a_difference_between_the_two_machines_reaches_the_host_log) {
    Rig r;
    seatedMismatch(r);
    r.run(20000);
    CHECK(r.host->stats().samples >= 10);
    CHECK(r.host->stats().received >= 4);
    CHECK(r.host->stats().rounds >= 4);
    CHECK(r.hostSaid("parity: DESYNC seated ref 0x00000300 host=standing client=seated slot=1"));
    CHECK(r.guestLog.empty());   // only the host compares
    int lines = 0;
    for (const std::string& l : r.hostLog) if (l.find("DESYNC seated") != std::string::npos) ++lines;
    CHECK(lines <= 2);   // one per ten seconds
}

TEST(parity_service, the_host_logs_a_summary_every_thirty_seconds) {
    Rig r;
    seatedMismatch(r);
    r.run(70000);
    int n = 0;
    for (const std::string& l : r.hostLog) if (l.find("parity: summary ") == 0) ++n;
    CHECK(n >= 1 && n <= 3);
    CHECK(r.host->lastSummary().find("refs compared") != std::string::npos);
    CHECK(r.hostSaid("mismatching slot=1"));
}

TEST(parity_service, identical_worlds_stay_quiet) {
    Rig r;
    r.setLists({ra(0x300, 200), ra(0x301, 400)}, {ra(0x300, 200), ra(0x301, 400)});
    r.run(20000);
    CHECK(r.host->stats().rounds >= 4);
    for (const std::string& l : r.hostLog) CHECK_MSG(l.find("DESYNC") == std::string::npos, l.c_str());
}

TEST(parity_service, a_guest_sends_every_nth_sample) {
    Settings s;
    s.sendEvery = 3;
    Rig r(s);
    r.run(30000);
    CHECK(r.guest->stats().samples >= 14);
    CHECK(r.guest->stats().sent == r.guest->stats().samples / 3);
    CHECK(r.host->stats().received == r.guest->stats().sent);
    Settings never;
    never.sendEvery = 0;
    Rig q(never);
    q.run(10000);
    CHECK(q.guest->stats().sent == 0);
}

TEST(parity_service, a_sample_from_a_peer_that_is_not_a_joined_guest_is_dropped) {
    Rig r;
    r.run(1000);
    Sample s = guestSample();
    s.seq = 1;
    r.net.endpoint(7)->send(0, encodeSample(s));   // an unknown peer talks to the host
    r.run(500);
    CHECK(r.host->stats().droppedUnknownPeer == 1);
    CHECK(r.host->stats().received + 1 >= r.guest->stats().sent);
}

TEST(parity_service, the_host_trusts_the_peer_not_the_slot_inside_the_datagram) {
    Settings s;
    s.sendEvery = 0;   // the real guest stays quiet; this test sends by hand
    Rig r(s);
    r.setLists({ra(0x400, 300)}, {});
    r.run(500);
    Sample forged = guestSample({});
    forged.slot = 5;   // claims to be somebody else
    forged.seq = 3;
    r.net.endpoint(1)->send(0, encodeSample(forged));
    r.run(8000);
    r.net.endpoint(1)->send(0, encodeSample(forged));
    r.run(500);
    CHECK(r.hostSaid("DESYNC absent ref 0x00000400"));
}

TEST(parity_service, a_new_session_registers_the_channel_again) {
    Rig r;
    r.run(6000);
    const uint64_t before = r.host->stats().received;
    r.hostMux.reset(new fnvcoop::MuxTransport(*r.net.endpoint(0)));   // the host's session was rebuilt: a new mux
    r.run(12000);
    CHECK(r.host->stats().sessions == 2);
    CHECK(r.host->stats().received > before);
}

TEST(parity_service, the_sample_file_gets_both_machines_views_and_the_pairing_link) {
    namespace fs = std::filesystem;
    const std::string path = (fs::temp_directory_path() / "nvcoop_parity_test_service.jsonl").string();
    std::remove(path.c_str());
    std::remove((path + ".1").c_str());
    {
        Rig r(Settings(), path);
        r.setLists({ra(0x300, 200)}, {ra(0x300, 200)});
        r.run(10000);
    }
    std::ifstream f(path);
    std::string line;
    int ticks = 0, cmps = 0;
    while (std::getline(f, line)) {
        js::Value o;
        std::string err;
        CHECK(js::parse(line, o, err));
        CHECK(o.get("role").asString() == "host");
        if (o.get("why").asString() == "tick") ++ticks;
        if (o.get("why").asString() == "cmp") {
            ++cmps;
            CHECK(o.get("cmp").get("slot").asInt() == 1);
            CHECK(o.get("cmp").get("seq").asInt() > 0);
        }
    }
    CHECK(ticks >= 4 && cmps >= 2);
    std::remove(path.c_str());
    std::remove((path + ".1").c_str());
}

TEST(parity_service, the_service_does_nothing_when_disabled_or_without_a_session) {
    Settings s;
    s.enabled = false;
    FakeSource src;
    ParityService off(s, src, nullptr, "");
    FrameInfo f;
    f.active = true; f.isHost = true; f.nowMs = 5000;
    off.frame(f);
    CHECK(off.stats().samples == 0);
    s.enabled = true;
    ParityService on(s, src, nullptr, "");
    FrameInfo none;
    none.nowMs = 5000;
    on.frame(none);
    CHECK(on.stats().samples == 0);
    CHECK(on.statusJson().find("\"enabled\":true") != std::string::npos);
}
