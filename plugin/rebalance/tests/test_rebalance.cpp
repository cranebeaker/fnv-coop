// plugin/rebalance (docs/rebalance.md): the campaign setting, the profile by party size, the smooth ramp, the GMST applier ("applied once"), the bigger fights
// (host only, once per enemy), and the director that ties them together on a host and on guests. Game-free; the engine is a fake.
// Each behaviour has a twin (FNVCOOP_MUT=1200..1252 and 1470..1482): the test must FAIL with its twin on. The party-save half (the setting in the sidecar and in the campaign
// order) is tested in plugin/coop/tests/test_party_save.cpp.
#include <cmath>
#include <cstdio>
#include <map>
#include <memory>
#include <set>
#include <string>
#include <vector>

#include "rebalance/balance.h"
#include "rebalance/campaign_settings.h"
#include "rebalance/director.h"
#include "rebalance/effect_applier.h"
#include "rebalance/fights.h"
#include "rebalance/gmst_applier.h"
#include "rebalance/ramp.h"
#include "rebalance/scarcity.h"

using namespace nvcoop::rebalance;

static int g_checks = 0, g_failed = 0;
#define CHECK(c)                                                                  \
    do {                                                                          \
        ++g_checks;                                                               \
        if (!(c)) { ++g_failed; std::printf("  FAIL %s:%d  %s\n", __FILE__, __LINE__, #c); } \
    } while (0)
static bool nearf(float a, float b, float eps = 0.001f) { return std::fabs(a - b) <= eps; }

// ---- the campaign setting ----------------------------------------------------------------------------------------------------------------------
static void testSettings() {
    std::printf("rebalance: the campaign setting\n");
    CampaignSettings none;
    CHECK(!none.chosen && !none.balanced());
    CHECK(sidecarLines(none).empty());                                   // a campaign without the setting writes nothing
    CampaignSettings b = chooseSettings("balanced", true);
    CHECK(b.chosen && b.balanced() && b.friendlyFire);
    CampaignSettings n = chooseSettings("normal", false);
    CHECK(n.chosen && !n.balanced() && !n.friendlyFire);
    CHECK(chooseSettings("Co-op", false).balanced());
    CHECK(chooseSettings("nonsense", true).chosen && !chooseSettings("nonsense", true).balanced());   // unreadable = a decision for Normal

    // the sidecar lines round trip through the key reader, both modes, both friendly fire values
    for (int m = 0; m < 2; ++m)
        for (int f = 0; f < 2; ++f) {
            CampaignSettings s = chooseSettings(m ? "balanced" : "normal", f != 0), r;
            std::string text = sidecarLines(s);
            size_t pos = 0;
            while (pos < text.size()) {
                size_t e = text.find('\n', pos);
                std::string line = text.substr(pos, e - pos);
                pos = e + 1;
                size_t eq = line.find('=');
                CHECK(applySidecarKey(r, line.substr(0, eq), line.substr(eq + 1)));
            }
            CHECK(r == s);
        }
    CampaignSettings r;
    CHECK(!applySidecarKey(r, "campaign", "x"));                         // not ours
    CHECK(applySidecarKey(r, "rebalance", "garbage") && !r.chosen);      // a bad value changes nothing
    CHECK(applySidecarKey(r, "friendlyFire", "7") && !r.friendlyFire);

    // the campaign order fields
    nvcoop::json::Value o = nvcoop::json::Value::object();
    addJson(o, b);
    CampaignSettings got = fromJson(o);
    CHECK(got == b);
    nvcoop::json::Value o2 = nvcoop::json::Value::object();
    addJson(o2, none);
    CHECK(!fromJson(o2).chosen);                                         // a host that does not send it: the guest plays Normal
    CHECK(describe(b) == "Co-op balanced, friendly fire on" && describe(n) == "Normal, friendly fire off");
    CHECK(hudLine(b, 3) == "This campaign plays Co-op balanced, friendly fire on (3 players).");
    CHECK(hudLine(n, 3).find("players") == std::string::npos);           // the size matters only in Co-op balanced
    CHECK(hudLine(b, 3).find("\xE2\x80\x94") == std::string::npos);      // no em dash in a line a player reads
}

// ---- the profile by party size -----------------------------------------------------------------------------------------------------------------
static void testProfile() {
    std::printf("rebalance: the profile by party size\n");
    CampaignSettings bal = chooseSettings("balanced", false), nor = chooseSettings("normal", false), unset;
    CHECK(profileFor(nor, 3).vanilla());                                 // Normal is the game as it is, whatever the party
    CHECK(profileFor(unset, 4).vanilla());
    CHECK(!profileFor(bal, 2).vanilla());
    Profile p1 = profileFor(bal, 1), p2 = profileFor(bal, 2), p3 = profileFor(bal, 3), p4 = profileFor(bal, 4);
    // one player: the game's own difficulty in every factor (it is still "balanced": blend 1)
    CHECK(nearf(p1.damageToPlayers, 1) && nearf(p1.damageByPlayers, 1) && nearf(p1.killXp, 1) && nearf(p1.stimpakHeal, 1) && nearf(p1.headMult, 1) && nearf(p1.blend, 1));
    CHECK(nearf(p1.suppliesKept, 1) && nearf(p1.bodyAmmoKept, 1) && p1.mediumArmorDr == 0 && p1.heavyArmorDr == 0);
    // the model's building blocks (docs/rebalance.md section 2): 0.7 focus, 0.5 toughness, 0.2 extras, 0.1 damage per extra player
    CHECK(nearf(focus(1), 1.0f) && nearf(focus(2), 1.7f) && nearf(focus(3), 2.4f) && nearf(focus(4), 3.1f));
    CHECK(nearf(toughness(1), 1.0f) && nearf(toughness(2), 1.5f) && nearf(toughness(3), 2.0f) && nearf(toughness(4), 2.5f));
    CHECK(nearf(extrasPerEnemy(1), 0) && nearf(extrasPerEnemy(2), 0.2f) && nearf(extrasPerEnemy(3), 0.4f) && nearf(extrasPerEnemy(4), 0.6f));
    CHECK(maxExtras(1) == 0 && maxExtras(2) == 2 && maxExtras(3) == 4 && maxExtras(4) == 6);
    CHECK(nearf(damageToPlayersFactor(2), 1.1f) && nearf(damageToPlayersFactor(4), 1.3f));
    CHECK(nearf(p2.damageToPlayers, 1.1f) && nearf(p3.damageToPlayers, 1.2f) && nearf(p4.damageToPlayers, 1.3f));
    CHECK(nearf(p2.damageByPlayers, 1.0f / 1.5f) && nearf(p3.damageByPlayers, 0.5f) && nearf(p4.damageByPlayers, 0.4f));
    CHECK(p3.damageByPlayers < p2.damageByPlayers && p4.damageByPlayers < p3.damageByPlayers);   // every shot takes less off a tougher enemy
    CHECK(nearf(p2.killXp, 1.0f / 1.2f) && nearf(p3.killXp, 1.0f / 1.4f) && nearf(p4.killXp, 1.0f / 1.6f));
    CHECK(p4.killXp < p3.killXp && p3.killXp < p2.killXp);               // more kills to make, less XP for each
    // the outcome the numbers were chosen for: a fight lasts 6 % to 29 % longer than a lone player's, and each player takes about two thirds of the damage
    CHECK(nearf(fightLength(2), 1.2f * 1.5f / 1.7f) && nearf(fightLength(3), 1.4f * 2.0f / 2.4f) && nearf(fightLength(4), 1.6f * 2.5f / 3.1f));
    CHECK(nearf(fightLength(1), 1.0f) && nearf(loadPerPlayer(1), 1.0f));
    CHECK(nearf(loadPerPlayer(2), 1.1f * 1.44f * 1.5f / 3.4f) && nearf(loadPerPlayer(3), 1.2f * 1.96f * 2.0f / 7.2f) && nearf(loadPerPlayer(4), 1.3f * 2.56f * 2.5f / 12.4f));
    for (int n = 2; n <= 4; ++n) CHECK(loadPerPlayer(n) > 0.62f && loadPerPlayer(n) < 0.72f && fightLength(n) > 1.0f && fightLength(n) < 1.3f);
    // the head factor gives back the square root of the toughness; limbs are the game's own
    CHECK(nearf(p2.headMult, std::sqrt(1.5f)) && nearf(p4.headMult, std::sqrt(2.5f)) && nearf(p4.limbMult, 1.0f));
    // armour is a party role: nothing alone, a step each in a party
    CHECK(p2.mediumArmorDr == 7 && p2.heavyArmorDr == 15 && p4.heavyArmorDr == 15);
    CHECK(nearf(toughness(50), toughness(kMaxScaledPlayers)));            // a huge party is scaled as the largest
    CHECK(nearf(toughness(0), 1.0f) && maxExtras(0) == 0);
}

// ---- the smooth change -------------------------------------------------------------------------------------------------------------------------
static void testRamp() {
    std::printf("rebalance: the ramp\n");
    CampaignSettings bal = chooseSettings("balanced", false);
    ProfileRamp r(1000);
    r.setTarget(profileFor(bal, 2), 0);
    CHECK(r.step(0).vanilla());                                          // starts from the game's numbers
    const float mid = r.step(500).damageByPlayers;
    CHECK(mid > 0.7f && mid < 1.0f && r.step(500).blend > 0 && r.step(500).blend < 1);   // part of the way from the game's 1 to the balanced 0.667
    CHECK(r.step(1000).damageByPlayers == profileFor(bal, 2).damageByPlayers && r.settled());
    // a player joins: the next target is reached smoothly, in order, with no jump
    r.setTarget(profileFor(bal, 3), 5000);
    float last = r.step(5000).damageByPlayers;
    CHECK(nearf(last, profileFor(bal, 2).damageByPlayers));               // no jump on the first frame
    bool monotone = true;
    for (uint64_t t = 5100; t <= 6000; t += 100) {
        const float now = r.step(t).damageByPlayers;
        monotone = monotone && now <= last + 1e-6f;
        last = now;
    }
    CHECK(monotone && nearf(last, profileFor(bal, 3).damageByPlayers) && r.settled());
    // a change in the middle of a move starts from where the numbers are, not from the old start
    r.setTarget(profileFor(bal, 4), 10000);
    const float at = r.step(10500).damageByPlayers;
    r.setTarget(profileFor(bal, 2), 10500);
    CHECK(nearf(r.step(10500).damageByPlayers, at));
    // the same target again changes nothing
    ProfileRamp q(1000);
    q.setTarget(profileFor(bal, 2), 0);
    q.step(400);
    const float v = q.current().damageByPlayers;
    q.setTarget(profileFor(bal, 2), 450);
    CHECK(q.step(450).damageByPlayers == v || nearf(q.step(450).damageByPlayers, v, 0.05f));
    q.step(1000);
    CHECK(q.settled());
    // back to Normal is smooth too
    q.setTarget(Profile(), 2000);
    CHECK(q.step(2500).damageByPlayers > profileFor(bal, 2).damageByPlayers && q.step(2500).damageByPlayers < 1.0f);   // on its way back up to the game's 1
    CHECK(q.step(3000).vanilla());
}

// ---- the GMST applier ---------------------------------------------------------------------------------------------------------------------------
struct FakeGmst : IGmst {
    std::map<std::string, float> v;
    int reads = 0, writes = 0;
    bool ready = true;
    FakeGmst() {
        for (const GmstEntry& e : gmstTable()) {
            float base = e.kind == GmstKind::KillXp ? 100 : 1.0f;
            v[e.name] = base;
        }
    }
    bool read(const std::string& n, float& out) override { ++reads; if (!ready || !v.count(n)) return false; out = v[n]; return true; }
    bool write(const std::string& n, float x) override { ++writes; if (!ready || !v.count(n)) return false; v[n] = x; return true; }
};

struct FakeEffects : IEffectPort {
    std::map<uint32_t, float> v;
    int reads = 0, writes = 0;
    bool ready = true;
    FakeEffects() { v[0x00015169u] = 5.0f; v[0x000CCEF2u] = 20.0f; }   // the game's own Stimpak and Super Stimpak healing per second
    bool read(uint32_t id, float& out) override { ++reads; if (!ready || !v.count(id)) return false; out = v[id]; return true; }
    bool write(uint32_t id, float x) override { ++writes; if (!ready || !v.count(id)) return false; v[id] = x; return true; }
};

static void testApplier() {
    std::printf("rebalance: the GMST applier\n");
    CampaignSettings bal = chooseSettings("balanced", false);
    {   // Normal never touches the game
        FakeGmst g;
        GmstApplier a(g);
        CHECK(a.apply(Profile()) && g.reads == 0 && g.writes == 0);
    }
    FakeGmst g;
    GmstApplier a(g);
    const Profile p2 = profileFor(bal, 2);
    CHECK(a.apply(p2));
    CHECK(nearf(g.v["fDiffMultHPToPCN"], 1.1f) && nearf(g.v["fDiffMultHPByPCN"], 1.0f / 1.5f));
    CHECK(g.v.count("fAVDCarryWeightsBase") == 0 && g.v.count("iLevelUpSkillPointsBase") == 0);   // carry weight and skill points are not in the table
    CHECK(nearf(g.v["iXPLevelKillNPCAverage"], 83));                      // 100 / 1.2 = 83.3, an integer setting
    const int w = g.writes;
    CHECK(w > 0);
    CHECK(a.apply(p2) && g.writes == w);                                 // applied again: nothing is written, nothing is multiplied twice
    for (int i = 0; i < 5; ++i) a.apply(p2);
    CHECK(nearf(g.v["fDiffMultHPToPCN"], 1.1f) && nearf(g.v["iXPLevelKillNPCAverage"], 83));
    // a bigger party: the numbers are worked out from the game's own value, not from the current one
    a.apply(profileFor(bal, 4));
    CHECK(nearf(g.v["fDiffMultHPByPCN"], 0.4f) && nearf(g.v["fDiffMultHPToPCN"], 1.3f) && nearf(g.v["iXPLevelKillNPCAverage"], 63));   // 100 / 1.6 = 62.5
    a.apply(profileFor(bal, 2));
    CHECK(nearf(g.v["fDiffMultHPByPCN"], 1.0f / 1.5f) && nearf(g.v["iXPLevelKillNPCAverage"], 83));   // back exactly where it was
    // Normal again puts the game's own numbers back
    CHECK(a.apply(Profile()));
    CHECK(nearf(g.v["fDiffMultHPToPCN"], 1.0f) && nearf(g.v["fDiffMultHPByPCN"], 1.0f) && nearf(g.v["iXPLevelKillNPCAverage"], 100));
    // the game was not ready: nothing is lost, the next call reads the originals
    FakeGmst late;
    late.ready = false;
    GmstApplier b(late);
    CHECK(!b.apply(p2) && late.writes == 0 && b.readsFailed() > 0);
    late.ready = true;
    CHECK(b.apply(p2) && nearf(late.v["fDiffMultHPToPCN"], 1.1f));
    // a new instance (a hot reload) that gets the old state back does not take a changed value for the original
    GmstApplier c(late);
    c.restoreState(b.state());
    CHECK(c.apply(Profile()) && nearf(late.v["fDiffMultHPToPCN"], 1.0f));
}

// ---- the resource shares in the profile -----------------------------------------------------------------------------------------------------------
static void testResourceProfile() {
    std::printf("rebalance: supplies, ammo and Stimpak strength by party size\n");
    CampaignSettings bal = chooseSettings("balanced", false);
    const Profile p2 = profileFor(bal, 2), p3 = profileFor(bal, 3), p4 = profileFor(bal, 4);
    CHECK(profileFor(chooseSettings("normal", false), 4).stimpakHeal == 1.0f && profileFor(CampaignSettings(), 4).suppliesKept == 1.0f);   // Normal: the game's own
    CHECK(nearf(rewardShare(1), 1.0f) && nearf(rewardShare(2), 1.0f / 1.2f) && nearf(rewardShare(3), 1.0f / 1.4f) && nearf(rewardShare(4), 1.0f / 1.6f));
    CHECK(nearf(p2.suppliesKept, 1.0f / 1.2f) && nearf(p4.suppliesKept, 1.0f / 1.6f));   // what a body holds shrinks by the share the fight grew by
    CHECK(nearf(p3.bodyAmmoKept, 1.0f / 1.4f) && nearf(p4.bodyAmmoKept, 1.0f / 1.6f));
    CHECK(nearf(p2.stimpakHeal, loadPerPlayer(2)) && nearf(p3.stimpakHeal, loadPerPlayer(3)) && nearf(p4.stimpakHeal, loadPerPlayer(4)));   // a Stimpak heals what a player now loses per fight
    CHECK(nearf(p2.stimpakHeal, 0.6988f, 0.001f) && nearf(p3.stimpakHeal, 0.6533f, 0.001f) && nearf(p4.stimpakHeal, 0.6710f, 0.001f));
    CHECK(nearf(profileFor(bal, 1).stimpakHeal, 1.0f));                  // alone: the game's Stimpak
    // the shares ride along in the smooth change
    const Profile m = mix(Profile(), p2, 0.5f);
    CHECK(nearf(m.suppliesKept, 0.5f * (1.0f + 1.0f / 1.2f)) && nearf(m.stimpakHeal, 0.5f * (1.0f + p2.stimpakHeal)));
}

// ---- Stimpak strength: the effect applier ---------------------------------------------------------------------------------------------------------
static void testEffects() {
    std::printf("rebalance: the Stimpak effect applier\n");
    CampaignSettings bal = chooseSettings("balanced", false);
    const uint32_t kStim = 0x00015169u, kSuper = 0x000CCEF2u;
    {   // Normal never touches the forms
        FakeEffects f;
        EffectApplier a(f);
        CHECK(a.apply(Profile()) && f.reads == 0 && f.writes == 0 && !a.changedAnything());
    }
    FakeEffects f;
    EffectApplier a(f);
    const Profile p2 = profileFor(bal, 2);
    CHECK(a.apply(p2));
    CHECK(nearf(f.v[kStim], 5.0f * p2.stimpakHeal) && nearf(f.v[kSuper], 20.0f * p2.stimpakHeal));   // the game's per-second amounts times the Stimpak factor
    CHECK(nearf(a.original(kStim), 5.0f) && a.changedAnything());
    const int w = f.writes;
    CHECK(w == 2 && a.writes() == 2);
    for (int i = 0; i < 6; ++i) CHECK(a.apply(p2));
    CHECK(f.writes == w && nearf(f.v[kStim], 5.0f * p2.stimpakHeal));    // applied again and again: nothing written, nothing shrunk twice
    // a bigger party and back: worked out from the game's own magnitude every time
    a.apply(profileFor(bal, 4));
    CHECK(nearf(f.v[kStim], 5.0f * profileFor(bal, 4).stimpakHeal));
    a.apply(profileFor(bal, 2));
    CHECK(nearf(f.v[kStim], 5.0f * p2.stimpakHeal) && nearf(f.v[kSuper], 20.0f * p2.stimpakHeal));   // exactly where it was
    // something else put the magnitude back (the form was reloaded): the next pass corrects it, from the original and not from what it found
    f.v[kStim] = 5.0f;
    const int w2 = f.writes;
    a.apply(p2);
    CHECK(f.writes == w2 + 1 && nearf(f.v[kStim], 5.0f * p2.stimpakHeal));
    // a new instance (a hot reload) that gets the state back writes nothing and does not take the changed magnitude for the original
    EffectApplier b(f);
    b.restoreState(a.state());
    const int w3 = f.writes;
    CHECK(b.apply(p2) && f.writes == w3);
    CHECK(b.apply(profileFor(bal, 3)) && nearf(f.v[kStim], 5.0f * profileFor(bal, 3).stimpakHeal));
    // Normal puts the game's own magnitudes back, and then the applier is "never changed" again
    CHECK(b.apply(Profile()));
    CHECK(nearf(f.v[kStim], 5.0f) && nearf(f.v[kSuper], 20.0f) && !b.changedAnything());
    const int r = f.reads, w4 = f.writes;
    CHECK(b.apply(Profile()) && f.reads == r && f.writes == w4);
    // the forms were not there yet (the game is loading): nothing is lost, the next call reads the originals
    FakeEffects late;
    late.ready = false;
    EffectApplier c(late);
    CHECK(!c.apply(p2) && late.writes == 0 && c.readsFailed() > 0);
    late.ready = true;
    CHECK(c.apply(p2) && nearf(late.v[kStim], 5.0f * p2.stimpakHeal));
}

// ---- supplies and ammo scarcity: the rule ---------------------------------------------------------------------------------------------------------
static LootStack stack(const char* id, int32_t n, ItemKind k) { LootStack s; s.id = id; s.count = n; s.kind = k; return s; }
static int32_t countOf(const std::vector<LootStack>& v, const char* id) { for (const LootStack& s : v) if (s.id == id) return s.count; return -1; }

static void testScarcity() {
    std::printf("rebalance: supplies and ammo scarcity\n");
    const CampaignSettings bal = chooseSettings("balanced", false);
    const Profile p2 = profileFor(bal, 2), p4 = profileFor(bal, 4);
    // the share: exact for whole stacks, right on average for small ones, and the same answer every time
    CHECK(keptCount(7, "a", 10, 0.5f) == 5 && keptCount(7, "a", 10, 1.0f) == 10 && keptCount(7, "a", 10, 0.0f) == 0 && keptCount(7, "a", 0, 0.5f) == 0);
    CHECK(keptCount(7, "a", 1, 0.5f) == keptCount(7, "a", 1, 0.5f));
    int survivors = 0;
    for (uint32_t net = 1; net <= 1000; ++net) survivors += keptCount(net, "stimpak", 1, 0.5f);
    CHECK(survivors > 440 && survivors < 560);                           // a single Stimpak survives about half the time
    int fewer = 0, more = 0;
    for (uint32_t net = 1; net <= 1000; ++net) { fewer += keptCount(net, "stimpak", 3, 0.5f); more += keptCount(net, "stimpak", 3, 0.75f); }
    CHECK(fewer > 1440 && fewer < 1560 && more > 2190 && more < 2310);   // 1.5 and 2.25 on average
    int differ = 0;
    for (uint32_t net = 1; net <= 64; ++net) differ += keptCount(net, "stimpak", 1, 0.5f) != keptCount(net + 1, "stimpak", 1, 0.5f);
    CHECK(differ > 10);                                                  // not the same answer for every container

    auto body = [] { return std::vector<LootStack>{stack("stimpak", 40, ItemKind::Supply), stack("9mm", 40, ItemKind::Ammo), stack("pistol", 1, ItemKind::Other), stack("caps", 40, ItemKind::Other)}; };
    LootSource corpse; corpse.net = 11; corpse.corpse = true;
    {   // a generic body: supplies and ammo thinned to the share, everything else whole
        std::vector<LootStack> s = body();
        const int32_t gone = trimLoot(corpse, s, p2);
        const int32_t keptStim = countOf(s, "stimpak"), keptAmmo = countOf(s, "9mm");
        CHECK(keptStim >= 33 && keptStim <= 34 && keptAmmo >= 33 && keptAmmo <= 34);   // 40 / 1.2 = 33.3
        CHECK(countOf(s, "pistol") == 1 && countOf(s, "caps") == 40 && gone == 80 - keptStim - keptAmmo);
        // a bigger party: less of both
        std::vector<LootStack> t = body();
        trimLoot(corpse, t, p4);
        CHECK(countOf(t, "stimpak") < 33 && countOf(t, "9mm") < 33 && countOf(t, "caps") == 40);
        CHECK(countOf(t, "stimpak") == keptCount(11, "stimpak", 40, p4.suppliesKept));   // 40 / 1.6 = 25
    }
    {   // a box does not grow with the party: nothing in it is thinned
        LootSource box; box.net = 12;
        std::vector<LootStack> s = body();
        CHECK(trimLoot(box, s, p4) == 0 && countOf(s, "stimpak") == 40 && countOf(s, "9mm") == 40);
    }
    {   // the body of a unique character is left alone
        LootSource uniq = corpse; uniq.uniqueActor = true;
        std::vector<LootStack> s = body();
        CHECK(trimLoot(uniq, s, p4) == 0 && countOf(s, "stimpak") == 40 && countOf(s, "9mm") == 40);
    }
    {   // the game's own numbers: nothing happens
        std::vector<LootStack> s = body();
        CHECK(trimLoot(corpse, s, Profile()) == 0 && countOf(s, "stimpak") == 40 && countOf(s, "9mm") == 40);
    }
    {   // the answer for one container is the same on every machine
        std::vector<LootStack> a = body(), b = body();
        trimLoot(corpse, a, p4);
        trimLoot(corpse, b, p4);
        CHECK(countOf(a, "stimpak") == countOf(b, "stimpak") && countOf(a, "9mm") == countOf(b, "9mm"));
    }
}

// ---- bigger fights ------------------------------------------------------------------------------------------------------------------------------
struct FakeFights {
    std::vector<Fighter> fighters;
    std::map<uint32_t, FightDirector::ExtraInfo> extras;
    uint32_t nextRef = 0x1000;
    int spawns = 0, removes = 0;
    FightDirector::Env env() {
        FightDirector::Env e;
        e.spawn = [this](const Fighter&, uint32_t) { ++spawns; uint32_t r = ++nextRef; extras[r] = {true, false, true}; return r; };
        e.inspect = [this](uint32_t r) { auto it = extras.find(r); return it == extras.end() ? FightDirector::ExtraInfo() : it->second; };
        e.remove = [this](uint32_t r) { ++removes; extras.erase(r); };
        return e;
    }
    void fight(uint32_t net, bool generic = true) {
        Fighter f;
        f.netId = net; f.ref = 0x5000 + net; f.baseId = 0x77; f.generic = generic; f.fightsParty = true; f.targetRef = 0x14;
        fighters.push_back(f);
    }
};

static void testFights() {
    std::printf("rebalance: bigger fights\n");
    {   // one player or Normal: no extras at all
        FakeFights w;
        FightDirector d(w.env());
        for (uint32_t i = 1; i <= 12; ++i) w.fight(i);
        d.tick(w.fighters, 1, true);
        CHECK(w.spawns == 0);
        d.tick(w.fighters, 4, false);
        CHECK(w.spawns == 0);
    }
    {   // four players: 0.6 of an extra for every enemy, so 8 enemies owe 4.8 and 4 are made, one a call
        FakeFights w;
        FightDirector d(w.env());
        for (uint32_t i = 1; i <= 8; ++i) w.fight(i);
        d.tick(w.fighters, 4, true);
        CHECK(w.spawns == 1);                                            // at most one a tick
        for (int i = 0; i < 6; ++i) d.tick(w.fighters, 4, true);
        CHECK(w.spawns == 4 && d.spawned() == 4 && d.counted() == 8);
        // the same enemies again bring nothing; an enemy that changes owner is the same enemy
        for (int i = 0; i < 4; ++i) d.tick(w.fighters, 4, true);
        CHECK(w.spawns == 4 && d.counted() == 8);
        // the extras themselves are fighters now: never copied
        for (const auto& [ref, info] : w.extras) {
            Fighter f; f.netId = 900 + ref; f.ref = ref; f.generic = true; f.fightsParty = true; f.targetRef = 0x14;
            w.fighters.push_back(f);
        }
        for (int i = 0; i < 4; ++i) d.tick(w.fighters, 4, true);
        CHECK(w.spawns == 4);
    }
    {   // a named character brings none; a dead one or one not fighting the party neither
        FakeFights w;
        FightDirector d(w.env());
        for (uint32_t i = 1; i <= 8; ++i) { w.fight(i, false); }
        Fighter dead; dead.netId = 50; dead.ref = 0x6000; dead.generic = true; dead.fightsParty = true; dead.dead = true; w.fighters.push_back(dead);
        Fighter calm; calm.netId = 51; calm.ref = 0x6001; calm.generic = true; calm.fightsParty = false; w.fighters.push_back(calm);
        for (int i = 0; i < 6; ++i) d.tick(w.fighters, 4, true);
        CHECK(w.spawns == 0);
    }
    {   // the cap on extras alive, and no rush of them once some die
        FakeFights w;
        FightDirector d(w.env());
        uint32_t net = 1;
        for (int round = 0; round < 40; ++round) {
            for (int k = 0; k < 4; ++k) w.fight(net++);
            d.tick(w.fighters, 8, true);   // 8 players: 1.4 extras per enemy and room for 14, far more owed than the cap lets live
        }
        CHECK(w.spawns == maxExtras(8) && maxExtras(8) == 14);
        for (auto& [ref, info] : w.extras) info.dead = true;             // all die
        w.fight(net++);
        d.tick(w.fighters, 8, true);
        CHECK(w.spawns == maxExtras(8) + 1);             // one more, not a rush of what was owed
        d.tick(w.fighters, 8, true);
        CHECK(w.spawns <= maxExtras(8) + 2);
    }
    {   // extras left behind (out of everyone's sight) are cleared away; a body near the group stays
        FakeFights w;
        FightDirector d(w.env());
        for (uint32_t i = 1; i <= 4; ++i) w.fight(i);
        for (int i = 0; i < 4; ++i) d.tick(w.fighters, 4, true);
        CHECK(w.spawns == 2 && d.extrasKnown() == 2);
        auto it = w.extras.begin();
        it->second.nearAnyPlayer = false;
        std::map<uint32_t, FightDirector::ExtraInfo>::iterator second = std::next(it);
        second->second.dead = true;                                      // a body, still near: stays
        d.tick(w.fighters, 4, true);
        CHECK(w.removes == 1 && d.extrasKnown() == 1 && d.removed() == 1);
    }
}

// ---- the director: a host and guests ------------------------------------------------------------------------------------------------------------
struct Machine {
    bool host = false;
    bool ready = true;
    CampaignSettings settings;
    int players = 0;
    uint64_t now = 0;
    FakeGmst gmst;
    FakeEffects effects;
    FakeFights fights;
    std::vector<std::string> hud;
    int ff = -2, ffCalls = 0;
    std::unique_ptr<Director> d;
    explicit Machine(bool isHost, uint32_t rampMs = 1000) : host(isHost) {
        DirectorConfig cfg;
        cfg.rampMs = rampMs;
        cfg.sizeNoticeMinMs = 2000;
        DirectorEnv e;
        e.worldReady = [this] { return ready; };
        e.isHost = [this] { return host; };
        e.settings = [this] { return settings; };
        e.players = [this] { return players; };
        e.nowMs = [this] { return now; };
        e.setFriendlyFire = [this](int v) { ff = v; ++ffCalls; };
        e.hud = [this](const std::string& s) { hud.push_back(s); };
        e.fighters = [this] { return fights.fighters; };
        e.fight = fights.env();
        d.reset(new Director(cfg, e, gmst, effects));
    }
    void run(uint64_t ms, uint64_t step = 50) {
        for (uint64_t t = 0; t < ms; t += step) { now += step; d->tick(); }
    }
};

static void testDirector() {
    std::printf("rebalance: the director\n");
    const CampaignSettings bal = chooseSettings("balanced", true), nor = chooseSettings("normal", false);
    {   // a Normal campaign: the game is untouched, friendly fire is the campaign's, one line says so
        Machine m(true);
        m.settings = nor; m.players = 3;
        m.run(2000);
        CHECK(m.gmst.writes == 0 && m.gmst.reads == 0);
        CHECK(m.ff == 0);
        CHECK(m.hud.size() == 1 && m.hud[0] == "This campaign plays Normal, friendly fire off.");
        CHECK(!m.d->balancedOn());
    }
    {   // a campaign without the setting: Normal, and friendly fire stays with the combat module's ini value
        Machine m(false);
        m.settings = CampaignSettings(); m.players = 2;
        m.run(1000);
        CHECK(m.ff == -1 && m.hud.empty() && m.gmst.writes == 0);
    }
    {   // balanced, first settings after a load: applied at once; the line names the mode, friendly fire and the size
        Machine m(true);
        m.settings = bal; m.players = 2;
        m.run(100);
        CHECK(nearf(m.gmst.v["fDiffMultHPToPCN"], 1.1f));                 // no ramp on a fresh load
        CHECK(m.ff == 1);
        CHECK(m.hud.size() == 1 && m.hud[0] == "This campaign plays Co-op balanced, friendly fire on (2 players).");
        CHECK(m.d->balancedOn());
        CHECK(m.d->statusJson().find("\"balanced\":true") != std::string::npos);
    }
    {   // a player joins: smooth, one notice, and never above the target
        Machine m(true, 2000);
        m.settings = bal; m.players = 2;
        m.run(500);
        m.players = 3;
        m.run(50);
        const float first = m.gmst.v["fDiffMultHPByPCN"];
        CHECK(first > 0.5f + 0.05f);                                     // not at the new value yet
        m.run(500);
        const float mid = m.gmst.v["fDiffMultHPByPCN"];
        CHECK(mid < first && mid > 0.5f);
        m.run(3000);
        CHECK(nearf(m.gmst.v["fDiffMultHPByPCN"], 0.5f));
        CHECK(m.hud.size() == 2 && m.hud[1] == "Co-op balanced is now scaled for 3 players.");
        m.players = 4; m.run(100); m.players = 3; m.run(100);             // flapping inside the notice interval: no spam
        CHECK(m.hud.size() <= 3);
        m.players = 2;
        m.run(4000);
        CHECK(nearf(m.gmst.v["fDiffMultHPByPCN"], 1.0f / 1.5f));          // someone left: back, smoothly
    }
    {   // the game is at the main menu: nothing is touched; the next game is applied at once
        Machine m(true);
        m.settings = bal; m.players = 2;
        m.ready = false;
        m.run(500);
        CHECK(m.gmst.writes == 0 && m.hud.empty() && m.ffCalls == 0);
        m.ready = true;
        m.run(100);
        CHECK(nearf(m.gmst.v["fDiffMultHPToPCN"], 1.1f));
    }
    {   // the mode of the next campaign: a load of a Normal campaign puts the game's numbers back
        Machine m(true);
        m.settings = bal; m.players = 2;
        m.run(200);
        m.ready = false; m.run(100);
        m.settings = nor; m.ready = true;
        m.run(2500);
        CHECK(nearf(m.gmst.v["fDiffMultHPToPCN"], 1.0f) && nearf(m.gmst.v["fDiffMultHPByPCN"], 1.0f));
        CHECK(m.hud.back() == "This campaign plays Normal, friendly fire off.");
    }
    {   // a host and two guests on one campaign: the same numbers on every machine, extras only from the host
        Machine host(true), g1(false), g2(false);
        Machine* all[3] = {&host, &g1, &g2};
        for (Machine* m : all) { m->settings = bal; m->players = 3; }
        for (uint32_t i = 1; i <= 8; ++i) { host.fights.fight(i); g1.fights.fight(i); g2.fights.fight(i); }   // every machine sees the same fights
        for (int i = 0; i < 12; ++i) for (Machine* m : all) m->run(1000, 1000);
        for (const GmstEntry& e : gmstTable()) {
            CHECK(nearf(host.gmst.v[e.name], g1.gmst.v[e.name]) && nearf(host.gmst.v[e.name], g2.gmst.v[e.name]));
        }
        CHECK(host.fights.spawns == 3);                                  // 8 enemies x 0.4 for three players = 3.2
        CHECK(g1.fights.spawns == 0 && g2.fights.spawns == 0);
        CHECK(host.ff == 1 && g1.ff == 1 && g2.ff == 1);
        // the host hands the world to... an enemy changing owner is still one enemy: counted once
        const int before = host.fights.spawns;
        for (Fighter& f : host.fights.fighters) f.ref += 0x100;          // another machine's copy of it now
        host.run(3000, 1000);
        CHECK(host.fights.spawns == before);
    }
    {   // not balanced: no extras even with a big party
        Machine m(true);
        m.settings = nor; m.players = 4;
        for (uint32_t i = 1; i <= 8; ++i) m.fights.fight(i);
        m.run(6000, 1000);
        CHECK(m.fights.spawns == 0);
    }
}

// ---- the director: Stimpak strength and the loot hook --------------------------------------------------------------------------------------------
static void testDirectorResources() {
    std::printf("rebalance: the director, Stimpak strength and loot scarcity\n");
    const CampaignSettings bal = chooseSettings("balanced", false), nor = chooseSettings("normal", false);
    const uint32_t kStim = 0x00015169u;
    LootSource corpse; corpse.net = 21; corpse.corpse = true;
    auto stacks = [] { return std::vector<LootStack>{stack("stimpak", 12, ItemKind::Supply), stack("9mm", 12, ItemKind::Ammo)}; };
    {   // Normal: the forms are never read or written, the loot hook does nothing
        Machine m(true);
        m.settings = nor; m.players = 3;
        m.run(2000);
        CHECK(m.effects.reads == 0 && m.effects.writes == 0);
        std::vector<LootStack> s = stacks();
        CHECK(m.d->trimLoot(corpse, s) == 0 && countOf(s, "stimpak") == 12 && m.d->trimmedSources() == 0);
        Machine u(true);
        u.settings = CampaignSettings(); u.players = 3;
        u.run(500);
        std::vector<LootStack> t = stacks();
        CHECK(u.d->trimLoot(corpse, t) == 0 && countOf(t, "9mm") == 12);   // a campaign that never chose: Normal
    }
    {   // balanced, two players: Stimpak at 0.70 at once on a fresh load, and the loot hook thins the bodies
        Machine m(true);
        m.settings = bal; m.players = 2;
        m.run(100);
        CHECK(nearf(m.effects.v[kStim], 5.0f * profileFor(bal, 2).stimpakHeal));
        std::vector<LootStack> s = stacks();
        const int32_t ks = keptCount(21, "stimpak", 12, 1.0f / 1.2f), ka = keptCount(21, "9mm", 12, 1.0f / 1.2f);
        CHECK(ks >= 10 && ks <= 11 && ka >= 10 && ka <= 11);              // 12 / 1.2 = 10
        const int32_t gone = 24 - ks - ka;
        CHECK(m.d->trimLoot(corpse, s) == gone && countOf(s, "stimpak") == ks && countOf(s, "9mm") == ka);
        CHECK(m.d->trimmedSources() == 1 && m.d->trimmedItems() == static_cast<uint64_t>(gone));
        const std::string j = m.d->statusJson();
        CHECK(j.find("\"stimpakHeal\":0.699") != std::string::npos && j.find("\"suppliesKept\":0.833") != std::string::npos &&
              j.find("\"itemsRemoved\":" + std::to_string(gone)) != std::string::npos);
    }
    {   // the loot hook uses the numbers the campaign is HEADING to, not a point of the smooth change
        Machine m(true, 4000);
        m.settings = bal; m.players = 2;
        m.run(200);
        m.players = 4;
        m.run(100);                                                       // the ramp has barely started: the live numbers are still the two player ones
        CHECK(m.d->profile().suppliesKept > 0.78f);
        std::vector<LootStack> s = stacks();
        m.d->trimLoot(corpse, s);
        CHECK(countOf(s, "stimpak") == keptCount(21, "stimpak", 12, 1.0f / 1.6f) && countOf(s, "stimpak") <= 8);   // 12 / 1.6 = 7.5; two players would keep 10
    }
    {   // a host and two guests: the same Stimpak magnitude everywhere, and the same thinned list for the same container whoever registers it
        Machine host(true), g1(false), g2(false);
        Machine* all[3] = {&host, &g1, &g2};
        for (Machine* m : all) { m->settings = bal; m->players = 3; m->run(3000, 1000); }
        CHECK(nearf(host.effects.v[kStim], g1.effects.v[kStim]) && nearf(host.effects.v[kStim], g2.effects.v[kStim]) && nearf(host.effects.v[kStim], 5.0f * profileFor(bal, 3).stimpakHeal));
        std::vector<LootStack> a = stacks(), b = stacks(), c = stacks();
        host.d->trimLoot(corpse, a); g1.d->trimLoot(corpse, b); g2.d->trimLoot(corpse, c);
        CHECK(countOf(a, "stimpak") == countOf(b, "stimpak") && countOf(b, "stimpak") == countOf(c, "stimpak") && countOf(a, "9mm") == countOf(c, "9mm"));
    }
    {   // back to Normal (a different campaign is loaded): the game's Stimpak comes back
        Machine m(true);
        m.settings = bal; m.players = 2;
        m.run(200);
        m.ready = false; m.run(100);
        m.settings = nor; m.ready = true;
        m.run(2500);
        CHECK(nearf(m.effects.v[kStim], 5.0f) && !m.d->effects().changedAnything());
    }
}

// Co-op balanced head, limb and armour factor (balance.cpp bodyHitFactor, twins 1495 and 1496).
static void testBodyHits() {
    std::printf("rebalance: head, limb and armour factor\n");
    CampaignSettings bal = chooseSettings("balanced", false), nor = chooseSettings("normal", false);
    const Profile b = profileFor(bal, 2), n = profileFor(nor, 2);
    auto near = [](float a, float x) { return a > x - 0.001f && a < x + 0.001f; };
    CHECK(near(bodyHitFactor(n, 1, ArmorClass::Heavy), 1.0f));          // Normal: the game's own
    CHECK(near(bodyHitFactor(b, 0, ArmorClass::None), 1.0f));           // a body shot on nothing
    const float head = std::sqrt(1.5f);                                  // the toughness of two players gives back its square root on the head
    CHECK(near(bodyHitFactor(b, 1, ArmorClass::None), head));           // the head
    for (uint8_t limb = 2; limb <= 5; ++limb) CHECK(near(bodyHitFactor(b, limb, ArmorClass::Light), 1.0f));   // limbs and light armour: the game's own
    CHECK(near(bodyHitFactor(b, 0, ArmorClass::Medium), 0.93f));        // +7 DR
    CHECK(near(bodyHitFactor(b, 0, ArmorClass::Heavy), 0.85f));         // +15 DR
    CHECK(near(bodyHitFactor(b, 1, ArmorClass::Heavy), head * 0.85f));  // both
    const Profile solo = profileFor(bal, 1);                             // alone there is no role to play: no armour step
    CHECK(near(bodyHitFactor(solo, 0, ArmorClass::Heavy), 1.0f) && near(bodyHitFactor(solo, 1, ArmorClass::None), 1.0f));
    const Profile half = mix(n, b, 0.5f);                                // the smooth change goes through every factor
    CHECK(near(bodyHitFactor(half, 1, ArmorClass::None), 0.5f * (1.0f + head)) && near(bodyHitFactor(half, 0, ArmorClass::Heavy), 0.925f));
}

int main() {
    std::setvbuf(stdout, nullptr, _IONBF, 0);   // a crashing twin still shows where
    testSettings();
    testProfile();
    testBodyHits();
    testResourceProfile();
    testRamp();
    testApplier();
    testEffects();
    testScarcity();
    testFights();
    testDirector();
    testDirectorResources();
    std::printf("%d checks, %d failed\n", g_checks, g_failed);
    return g_failed ? 1 : 0;
}
