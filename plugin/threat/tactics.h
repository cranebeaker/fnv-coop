// plugin/threat: smarter enemies for a group (cover, side steps, backing off when hurt). The engine's combat styles (TESCombatStyle, one per kind of
// enemy) hold how often an enemy takes cover, how far it looks for cover, how often it steps aside and whether it flees when badly hurt; "co-op
// balanced" turns those up for every enemy this machine runs. Game-free: the form data is reached through IStyleStore, the values are plain numbers.
#pragma once
#include <cstdint>
#include <map>

namespace nvcoop::threat {

// The four fields of a combat style that tactics changes (xNVSE GameForms.h TESCombatStyle: +0x18 coverSearchRadius, +0x1C takeCoverChance %,
// +0x59 LRChance %, +0xA8 csFlags with 0x04 = flee based on personal survival).
struct StyleValues {
    float coverRadius = 0;
    float takeCover = 0;
    uint8_t sideStep = 0;
    uint16_t flags = 0;
    friend bool operator==(const StyleValues& a, const StyleValues& b) {
        return a.coverRadius == b.coverRadius && a.takeCover == b.takeCover && a.sideStep == b.sideStep && a.flags == b.flags;
    }
    friend bool operator!=(const StyleValues& a, const StyleValues& b) { return !(a == b); }
};
constexpr uint16_t kStyleFleeWhenHurt = 0x04;

struct TacticsConfig {
    float takeCoverAtLeast = 75;           // % chance to take cover, for enemies that use cover at all
    float coverRadiusMult = 1.5f;          // they look this much further for cover
    uint8_t sideStepAtLeast = 75;          // % chance to step aside
    bool fleeWhenHurt = true;              // every style backs off when badly hurt
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
