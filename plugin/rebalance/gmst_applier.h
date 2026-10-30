// Writes a Profile into the game's own settings (GMSTs: the difficulty multipliers and kill XP) and puts the game's value back
// when the campaign is Normal. Game-free; the settings themselves come in through IGmst.
//
// "Applied once": every value is computed from the game's ORIGINAL value (read once, before the first change) and a profile, never from the current value, and a
// value is written only when it differs from the last one written. So running it twice, or on two machines, gives the same numbers: nothing is ever multiplied twice.
#pragma once
#include <cstdint>
#include <map>
#include <string>
#include <vector>

#include "rebalance/balance.h"

namespace nvcoop::rebalance {

class IGmst {
public:
    virtual ~IGmst() = default;
    virtual bool read(const std::string& name, float& out) = 0;    // false: no such setting, or the game is not ready
    virtual bool write(const std::string& name, float value) = 0;
};

enum class GmstKind : uint8_t { DamageToPlayers, DamageByPlayers, KillXp };
struct GmstEntry { const char* name; GmstKind kind; bool integer; };
const std::vector<GmstEntry>& gmstTable();

// What one entry becomes for a profile, from the game's original value.
float gmstValue(const GmstEntry& e, float original, const Profile& p);

class GmstApplier {
public:
    explicit GmstApplier(IGmst& g) : g_(g) {}
    // Brings the game's settings to `p`. Returns false when the originals could not be read yet (call again later). A vanilla profile on a game that was never
    // changed touches nothing, not even a read.
    bool apply(const Profile& p);
    // Original and last-written values, so a hot reload can carry them over (the new instance must not take a changed value for the original).
    struct State { std::map<std::string, float> originals, written; };
    State state() const { return {originals_, written_}; }
    void restoreState(const State& s) { originals_ = s.originals; written_ = s.written; }
    uint64_t writes() const { return writes_; }
    uint64_t readsFailed() const { return readsFailed_; }
    bool changedAnything() const { return !written_.empty(); }
    bool hasOriginal(const std::string& name) const { return originals_.count(name) != 0; }
    float original(const std::string& name) const { auto it = originals_.find(name); return it == originals_.end() ? 0.0f : it->second; }
    float lastWritten(const std::string& name) const { auto it = written_.find(name); return it == written_.end() ? original(name) : it->second; }
private:
    IGmst& g_;
    std::map<std::string, float> originals_;
    std::map<std::string, float> written_;
    uint64_t writes_ = 0, readsFailed_ = 0;
};

}  // namespace nvcoop::rebalance
