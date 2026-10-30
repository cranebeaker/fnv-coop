// Game-free tests of the animprobe core. Built once (FNVCOOP_RUNTIME_MUT); run once for real and once per FNVCOOP_MUT=<n> (each twin MUST fail).
#include <chrono>
#include <cstdio>
#include <cstring>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>

#include "animprobe_core.h"

using namespace animprobe;
using nvcoop::json::Value;

static int g_fail = 0, g_checks = 0;
#define CHECK(c)                                                       \
    do {                                                               \
        ++g_checks;                                                    \
        if (!(c)) {                                                    \
            ++g_fail;                                                  \
            std::printf("  FAIL %s:%d  %s\n", __FILE__, __LINE__, #c); \
        }                                                              \
    } while (0)

static void testParse() {
    Request r;
    std::string err;
    CHECK(ParseRequest("", r, err));
    CHECK(r.nearby && r.includePlayer && r.bones && r.maxActors == 16);

    CHECK(ParseRequest(R"({"refs":["0xFF000E","player","1A2B"],"bones":false,"max":8})", r, err));
    CHECK(r.refs.size() == 2 && r.refs[0] == 0xFF000E && r.refs[1] == 0x1A2B && r.includePlayer);
    CHECK(!r.bones && r.maxActors == 8 && !r.nearby);

    CHECK(ParseRequest(R"({"near":{"radius":500}})", r, err));
    CHECK(r.nearby && r.nearRadius == 500.f);
    CHECK(ParseRequest(R"({"near":300,"bone_names":["Bip01 Head","Weapon"]})", r, err));
    CHECK(r.nearRadius == 300.f && r.boneNames.size() == 2);

    CHECK(!ParseRequest(R"({"refs":["zz"]})", r, err));
    CHECK(!ParseRequest(R"({"max":0,"near":true})", r, err));
    CHECK(!ParseRequest(R"({"max":65,"near":true})", r, err));  // above kMaxActorsPerProbe
    CHECK(!ParseRequest(R"({"near":{"radius":-5}})", r, err));
    CHECK(!ParseRequest(R"({})", r, err));
    CHECK(!ParseRequest("[1,2]", r, err));
}

static void testSelect() {
    std::vector<float> d2 = {400.f * 400.f, 10.f * 10.f, 5000.f * 5000.f, 100.f * 100.f, 50.f * 50.f};
    std::vector<int> s = SelectNearest(d2, 1000.f, 16);
    CHECK(s.size() == 4);  // the 5000-unit one is outside the radius
    CHECK(s.size() == 4 && s[0] == 1 && s[1] == 4 && s[2] == 3 && s[3] == 0);  // nearest first
    std::vector<int> two = SelectNearest(d2, 1000.f, 2);
    CHECK(two.size() == 2 && two[0] == 1 && two[1] == 4);
    CHECK(SelectNearest({}, 1000.f, 4).empty());
}

static void testJson() {
    ActorSample a;
    a.ref = 0x0010A2B3;
    a.base = 0x00000007;
    a.isPlayer = true;
    a.valid3d = true;
    a.pos = {1.5f, -2.f, 3.f};
    a.haveAnim = true;
    a.action = 3;
    SeqSample sq;
    sq.slot = 1;
    sq.group = 12;
    sq.state = 1;
    sq.t = 0.25f;
    a.seqs.push_back(sq);
    a.haveMove = true;
    a.moveFlags = 0x100;
    a.bones.emplace_back("Bip01 Head", Vec3{1, 2, 3});
    Value j = ToJson(a);
    std::string text = j.dump();
    Value back;
    std::string err;
    CHECK(nvcoop::json::parse(text, back, err));
    CHECK(back.get("ref").asString() == "0x0010A2B3");
    CHECK(back.get("player").asBool());
    CHECK(back.get("anim").get("seqs").items().size() == 1);
    CHECK(back.get("anim").get("seqs").items()[0].get("group").asInt() == 12);
    CHECK(back.get("move_flags").asInt() == 0x100);
    const auto& head = back.get("bones").get("Bip01 Head").items();
    CHECK(head.size() == 3);
    CHECK(head.size() == 3 && head[2].asNumber() == 3.0);   // (never index an empty list: a missing "bones" key must FAIL the test, not crash it)
    CHECK(HexId(0xABCDEF) == "0x00ABCDEF");

    // the player's first-person AnimData: a human's swing plays there, so the probe must carry it (twin 7)
    SeqSample fp;
    fp.slot = 4;
    fp.group = 26;
    fp.state = 1;
    a.fpSeqs.push_back(fp);
    Value jf = ToJson(a);
    Value backf;
    CHECK(nvcoop::json::parse(jf.dump(), backf, err));
    CHECK(backf.get("anim").get("seqs_first_person").items().size() == 1);
    CHECK(backf.get("anim").get("seqs_first_person").items().size() == 1 && backf.get("anim").get("seqs_first_person").items()[0].get("group").asInt() == 26);
    CHECK(backf.get("anim").find("seqs_alt") == nullptr && back.get("anim").find("seqs_first_person") == nullptr);

    ActorSample b;  // no bones, not the player: those keys must be absent
    Value jb = ToJson(b);
    CHECK(jb.find("bones") == nullptr);
    CHECK(jb.find("player") == nullptr);
    CHECK(jb.find("anim") == nullptr);
}


// The sample whose JSON is committed in tests/golden_sample.json; tools/animcheck parses that same file in its tests, so
// the two languages cannot drift apart. Regenerate with:  animprobe_tests --write-golden <path>
static ActorSample goldenSample() {
    ActorSample a;
    a.ref = 0x0010A2B3;
    a.base = 0x00000007;
    a.isPlayer = true;
    a.valid3d = true;
    a.pos = {1.5f, -2.f, 3.f};
    a.rot = {0.f, 0.f, 0.5f};
    a.cell = 0x00000DEA;
    a.haveAnim = true;
    a.action = 3;
    a.sequenceRaw = 0x00ABCDEF;
    SeqSample sq;
    sq.slot = 1;
    sq.group = 12;
    sq.state = 1;
    sq.t = 0.25f;
    sq.end = 1.0f;
    sq.weight = 1.0f;
    a.seqs.push_back(sq);
    a.haveMove = true;
    a.moveFlags = 0x100;
    a.weaponForm = 0x00001234;
    a.inCombat = true;
    a.combatTarget = 0x0010A2B4;
    a.bones.emplace_back("Bip01 Head", Vec3{1, 2, 3});
    return a;
}

static void testGolden() {
#ifdef ANIMPROBE_GOLDEN
    std::ifstream f(ANIMPROBE_GOLDEN, std::ios::binary);
    CHECK(f.good());
    std::stringstream ss;
    ss << f.rdbuf();
    std::string want = ss.str();
    while (!want.empty() && (want.back() == 0x0A || want.back() == 0x0D)) want.pop_back();
    CHECK(ToJson(goldenSample()).dump() == want);
#endif
}

static void testCost() {
    CostStats c;
    CHECK(c.Percentile(0.5) == 0);
    for (int i = 1; i <= 100; ++i) c.Add(static_cast<double>(i), 2);
    CHECK(c.count() == 100);
    CHECK(c.Percentile(0.5) >= 49 && c.Percentile(0.5) <= 52);
    CHECK(c.Percentile(0.95) >= 94 && c.Percentile(0.95) <= 97);
    Value j = c.ToJson();
    CHECK(j.get("max_us").asNumber() == 100.0);
    CHECK(j.get("us_per_actor").asNumber() > 25.0 && j.get("us_per_actor").asNumber() < 26.0);  // mean 50.5 / 2
    for (int i = 0; i < 1000; ++i) c.Add(7.0, 1);  // window wraps; max stays the all-time max
    CHECK(c.Percentile(0.5) == 7.0);
    CHECK(c.ToJson().get("max_us").asNumber() == 100.0);
}

static void testBoneTable() {
    CHECK(kDefaultBoneCount >= 14 && kDefaultBoneCount <= 18);
    bool head = false, weapon = false;
    for (int i = 0; i < kDefaultBoneCount; ++i) {
        std::string n = kDefaultBones[i];
        head |= n == "Bip01 Head";
        weapon |= n == "Weapon";
    }
    CHECK(head && weapon);
}

int main(int argc, char** argv) {
    if (argc == 3 && std::strcmp(argv[1], "--write-golden") == 0) {
        std::ofstream o(argv[2], std::ios::binary);
        o << ToJson(goldenSample()).dump() << "\n";
        return o.good() ? 0 : 1;
    }
    if (argc == 2 && std::strcmp(argv[1], "--bench") == 0) {
        // game-free part of a probe: encode 16 actors x 16 bones to JSON (the engine reads are measured in game via anim.stats)
        ActorSample a = goldenSample();
        a.bones.clear();
        for (int i = 0; i < 16; ++i) a.bones.emplace_back(kDefaultBones[i % kDefaultBoneCount], Vec3{1.f * i, 2.f, 3.f});
        const int iters = 2000;
        size_t bytes = 0;
        auto t0 = std::chrono::steady_clock::now();
        for (int it = 0; it < iters; ++it) {
            nvcoop::json::Value arr = nvcoop::json::Value::array();
            for (int k = 0; k < 16; ++k) arr.push(ToJson(a));
            bytes += arr.dump().size();
        }
        double us = std::chrono::duration<double, std::micro>(std::chrono::steady_clock::now() - t0).count() / iters;
        std::printf("bench: encode 16 actors x 16 bones = %.1f us per probe (%.2f us per actor), %zu bytes per probe\n", us, us / 16, bytes / iters);
        return 0;
    }
    testParse();
    testGolden();
    testSelect();
    testJson();
    testCost();
    testBoneTable();
    std::printf("animprobe core: %d checks, %d failed\n", g_checks, g_fail);
    return g_fail == 0 ? 0 : 1;
}
