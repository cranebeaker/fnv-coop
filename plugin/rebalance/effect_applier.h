// Stimpak strength (docs/rebalance.md): the game has no setting for how much a Stimpak heals, so the first effect of the Stimpak and Super Stimpak forms (restore
// health, a magnitude per second) is rewritten in memory and put back when the campaign is Normal. Game-free; the forms themselves come in through IEffectPort.
//
// The same "applied once" rule as GmstApplier: every value is computed from the form's ORIGINAL magnitude (read once, before the first change) and a profile, never
// from the current magnitude, so nothing stacks; every machine works out the same absolute value from the same inputs. Unlike a game setting, a form can be read back
// cheaply, so each pass also looks at the live value and puts it right if something else changed it (a hot reload that lost its state still cannot compound it,
// because the original is carried across a reload the way GmstApplier's is).
#pragma once
#include <cstdint>
#include <map>
#include <vector>

#include "rebalance/balance.h"

namespace nvcoop::rebalance {

class IEffectPort {
public:
    virtual ~IEffectPort() = default;
    virtual bool read(uint32_t formId, float& magnitude) = 0;    // false: no such form, not the item expected, or the game is not ready
    virtual bool write(uint32_t formId, float magnitude) = 0;
};

struct EffectEntry { uint32_t formId; const char* name; };
const std::vector<EffectEntry>& effectTable();

// What the magnitude becomes for a profile, from the form's original.
float effectValue(float original, const Profile& p);

class EffectApplier {
public:
    explicit EffectApplier(IEffectPort& e) : e_(e) {}
    // Brings the forms to `p`. Returns false when an original could not be read yet (call again later). A vanilla profile on forms that were never changed touches
    // nothing, not even a read.
    bool apply(const Profile& p);
    struct State { std::map<uint32_t, float> originals, written; };
    State state() const { return {originals_, written_}; }
    void restoreState(const State& s) { originals_ = s.originals; written_ = s.written; }
    uint64_t writes() const { return writes_; }
    uint64_t readsFailed() const { return readsFailed_; }
    bool changedAnything() const { return !written_.empty(); }
    bool hasOriginal(uint32_t id) const { return originals_.count(id) != 0; }
    float original(uint32_t id) const { auto it = originals_.find(id); return it == originals_.end() ? 0.0f : it->second; }
    float lastWritten(uint32_t id) const { auto it = written_.find(id); return it == written_.end() ? original(id) : it->second; }
private:
    IEffectPort& e_;
    std::map<uint32_t, float> originals_;
    std::map<uint32_t, float> written_;
    uint64_t writes_ = 0, readsFailed_ = 0;
};

}  // namespace nvcoop::rebalance
