// plugin/threat tests, part 3: combat style tactics (tactics.cpp). Twins 1040..1048 must each make one of these fail.
#include <cmath>
#include <map>

#include "minitest.h"
#include "threat/tactics.h"

using namespace nvcoop::threat;

namespace {

struct FakeStyles : IStyleStore {
    std::map<uint32_t, StyleValues> forms;
    int writes = 0;
    bool read(uint32_t style, StyleValues& out) override {
        auto it = forms.find(style);
        if (it == forms.end()) return false;
        out = it->second;
        return true;
    }
    bool write(uint32_t style, const StyleValues& v) override {
        auto it = forms.find(style);
        if (it == forms.end()) return false;
        it->second = v;
        ++writes;
        return true;
    }
};

StyleValues raider() { return StyleValues{1024, 50, 50, 0}; }        // a raider style: radius 1024, cover 50 %, side step 50 %, flee 0
StyleValues animal() { return StyleValues{0, 0, 25, 0}; }

constexpr uint32_t kRaider = 0x1000, kDog = 0x2000;

}  // namespace

TEST(threat_tactics, a_style_that_uses_cover_gets_more_of_it_and_every_style_may_flee) {
    TacticsConfig c;
    const StyleValues v = tacticsFor(raider(), c);
    CHECK(v.takeCoverChance == 60);
    CHECK(std::fabs(v.coverSearchRadius - 1433.6f) < 0.01f);
    CHECK(v.lrChance == 50);
    CHECK((v.csFlags & kCsFlagFleeBasedOnPersonalSurvival) != 0);
    StyleValues keen{800, 90, 80, 0x21};
    const StyleValues k = tacticsFor(keen, c);
    CHECK(k.takeCoverChance == 90);                                    // twin 1041: capped at 60
    CHECK(k.lrChance == 80);
    CHECK(k.csFlags == 0x25);                                      // the flags that are not ours stay
}

TEST(threat_tactics, a_style_that_never_takes_cover_keeps_its_cover_and_side_step) {
    TacticsConfig c;
    const StyleValues v = tacticsFor(animal(), c);
    CHECK(v.takeCoverChance == 0 && v.coverSearchRadius == 0 && v.lrChance == 25);   // twin 1040
    CHECK((v.csFlags & kCsFlagFleeBasedOnPersonalSurvival) != 0);
    c.fleeWhenHurt = false;
    CHECK(tacticsFor(animal(), c).csFlags == 0);
}

TEST(threat_tactics, styles_are_patched_when_tactics_go_on_and_put_back_when_they_go_off) {
    FakeStyles s;
    s.forms[kRaider] = raider();
    s.forms[kDog] = animal();
    StyleBook b(s);
    b.see(kRaider);
    b.see(kDog);
    CHECK(b.known() == 2 && b.patched() == 0);
    CHECK(s.forms[kRaider] == raider());                         // off: untouched
    b.setOn(true);
    CHECK(b.patched() == 2);
    CHECK(s.forms[kRaider].takeCoverChance == 60);
    CHECK(s.forms[kDog].takeCoverChance == 0);
    b.setOn(false);
    CHECK(b.patched() == 0);
    CHECK(s.forms[kRaider] == raider());
    CHECK(s.forms[kDog] == animal());
}

TEST(threat_tactics, a_style_seen_while_on_is_patched_at_once_and_an_unreadable_one_is_ignored) {
    FakeStyles s;
    s.forms[kRaider] = raider();
    StyleBook b(s);
    b.setOn(true);
    b.see(kRaider);
    CHECK(b.patched() == 1 && s.forms[kRaider].takeCoverChance == 60);
    b.see(0x9999);                                               // not a combat style (the read refuses)
    b.see(0);
    CHECK(b.known() == 1);
    const int writes = s.writes;
    b.see(kRaider);
    CHECK(s.writes == writes);                                   // nothing to do twice
}

TEST(threat_tactics, new_numbers_are_worked_out_from_the_originals_not_on_top_of_the_patch) {
    FakeStyles s;
    s.forms[kRaider] = raider();
    StyleBook b(s);
    b.setOn(true);
    b.see(kRaider);
    TacticsConfig c;
    c.coverRadiusMult = 2.0f;
    b.configure(c);
    CHECK(s.forms[kRaider].coverSearchRadius == 2048);                 // twin 1043: 1024 * 2 = 2048 (not 1433.6 * 2)
    c.coverRadiusMult = 1.0f;
    b.configure(c);
    CHECK(s.forms[kRaider].coverSearchRadius == 1024);
    b.setOn(false);
    CHECK(s.forms[kRaider] == raider());
}

TEST(threat_tactics, a_style_changed_by_something_else_is_not_overwritten_when_putting_back) {
    FakeStyles s;
    s.forms[kRaider] = raider();
    StyleBook b(s);
    b.setOn(true);
    b.see(kRaider);
    s.forms[kRaider].takeCoverChance = 20;                             // a mod / script changed it after we patched it
    b.setOn(false);
    CHECK(s.forms[kRaider].takeCoverChance == 20);                     // twin 1046
    CHECK(b.foreign() == 1);
}

TEST(threat_tactics, a_style_changed_before_we_patched_it_is_what_we_put_back) {
    FakeStyles s;
    s.forms[kRaider] = raider();
    StyleBook b(s);
    b.see(kRaider);                                              // seen with its own numbers, tactics still off
    s.forms[kRaider].takeCoverChance = 30;                             // changed before tactics came on
    b.setOn(true);
    CHECK(s.forms[kRaider].takeCoverChance == 60);
    b.setOn(false);
    CHECK(s.forms[kRaider].takeCoverChance == 30);                     // twin 1045: the stale 50
}

TEST(threat_tactics, putting_everything_back_also_switches_tactics_off) {
    FakeStyles s;
    s.forms[kRaider] = raider();
    StyleBook b(s);
    b.setOn(true);
    b.see(kRaider);
    b.restoreAll();
    CHECK(s.forms[kRaider] == raider());
    CHECK(!b.on());                                              // twin 1048
    b.see(kRaider);
    CHECK(s.forms[kRaider] == raider());                         // and a style seen afterwards stays as it was
    StyleValues o;
    CHECK(b.original(kRaider, o) && o == raider());
    CHECK(!b.original(0x1, o));
}
