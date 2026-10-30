// plugin/threat: smarter enemies for a group (cover, side steps, backing off when hurt). The engine's combat styles (TESCombatStyle, one per kind of
// enemy) hold how often an enemy takes cover, how far it looks for cover, how often it steps aside and whether it flees when badly hurt; "co-op
// balanced" turns those up for every enemy this machine runs. Game-free: the form data is reached through IStyleStore, the values are plain numbers.
#pragma once
#include <cstdint>
#include <map>

namespace nvcoop::threat {

// The four fields of a combat style that tactics changes, named as xNVSE names them (third_party/NVSE nvse/nvse/GameForms.h, class TESCombatStyle):
// coverSearchRadius at +0x18, takeCoverChance at +0x1C (percent), LRChance at +0x59 (percent), csFlags at +0xA8.
struct StyleValues {
    float coverSearchRadius = 0;
    float takeCoverChance = 0;
    uint8_t lrChance = 0;
    uint16_t csFlags = 0;
    friend bool operator==(const StyleValues& a, const StyleValues& b) {
        return a.coverSearchRadius == b.coverSearchRadius && a.takeCoverChance == b.takeCoverChance && a.lrChance == b.lrChance && a.csFlags == b.csFlags;
    }
    friend bool operator!=(const StyleValues& a, const StyleValues& b) { return !(a == b); }
};
// TESCombatStyle::kFlag_FleeBasedOnPersonalSurvival (GameForms.h): the style backs off when badly hurt. threat_module.cpp static_asserts the value against the header.
constexpr uint16_t kCsFlagFleeBasedOnPersonalSurvival = 0x04;

// Why these numbers (docs/threat.md, "Tactics"): a party attacks from several directions at once, and an enemy that stands in the open trades shots with all of
// them. Rather than only making enemies spongier (rebalance's toughness), they use the engine's own combat-style levers a little more.
struct TacticsConfig {
    // Floor for the chance to take cover, for enemies that use cover at all. 60: in about three of five firing decisions the enemy looks for cover, and two of
    // five it trades in the open, so it is never a turret and never a coward. Styles that already take cover more often keep their own value.
    float takeCoverAtLeast = 60;
    // Cover search radius times this. Cover that hides an enemy from two attackers at an angle `a` apart lies about 1 / cos(a / 2) times as far out as cover
    // that hides it from one; for two players about 90 degrees apart that is 1 / cos(45 degrees) = 1.41, rounded to 1.4.
    float coverRadiusMult = 1.4f;
    // Floor for the side-step chance (same rule as cover). 50: it steps aside in half of its decisions, so a moving target costs the party more shots without
    // making the enemy a dancer. Cover comes first, strafing second, hence the lower floor.
    uint8_t sideStepAtLeast = 50;
    // Every style may back off when badly hurt: a creature that is losing against several attackers retreats, a dog as much as a raider. It only matters at
    // low health, so it shortens the tail of a fight instead of changing its start.
    bool fleeWhenHurt = true;
};

// The values a style gets. A style that never takes cover (takeCover 0: animals, robots) keeps its cover and side-step values.
StyleValues tacticsFor(const StyleValues& original, const TacticsConfig& cfg);

class IStyleStore {
public:
    virtual ~IStyleStore() = default;
    virtual bool read(uint32_t style, StyleValues& out) = 0;      // false: not a readable combat style (the caller leaves it alone)
    virtual bool write(uint32_t style, const StyleValues& v) = 0;
};

// Remembers what every style was before we touched it, applies tactics to the ones enemies here use, and puts them back when tactics go off.
class StyleBook {
public:
    explicit StyleBook(IStyleStore& store) : store_(store) {}

    void configure(const TacticsConfig& cfg);
    // Tactics on or off; on a change every known style is patched / restored.
    void setOn(bool on);
    bool on() const { return on_; }
    // An enemy here uses this style: patched now when tactics are on.
    void see(uint32_t style);
    // Put every style back (tactics off, the session ended, the DLL unloads); a style somebody else changed since we patched it is left as it is.
    void restoreAll();

    size_t known() const { return styles_.size(); }
    size_t patched() const;
    size_t foreign() const { return foreign_; }                   // styles left alone because something else changed them under us
    bool original(uint32_t style, StyleValues& out) const;

private:
    struct Entry {
        StyleValues original, applied;
        bool patched = false;
    };
    void patch(uint32_t style, Entry& e);
    void restore(uint32_t style, Entry& e);

    IStyleStore& store_;
    TacticsConfig cfg_;
    bool on_ = false;
    size_t foreign_ = 0;
    std::map<uint32_t, Entry> styles_;
};

}  // namespace nvcoop::threat
