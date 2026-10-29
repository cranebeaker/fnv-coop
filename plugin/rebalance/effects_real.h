// The engine half of the Stimpak strength (docs/rebalance.md): the first effect of an ingestible form (AlchemyItem), read and written in memory (effects_real.cpp).
// x86, compiled into nvcoop.dll only. NOTHING here has run in a game.
#pragma once
#include <map>

#include "rebalance/effect_applier.h"

namespace nvcoop::rebalance {

class GameEffects : public IEffectPort {
public:
    bool read(uint32_t formId, float& magnitude) override;
    bool write(uint32_t formId, float magnitude) override;
    bool broken() const { return broken_; }   // an access faulted: the forms stay as they are for the rest of the session
private:
    uint32_t* find(uint32_t formId);
    std::map<uint32_t, uint32_t*> cache_;     // form id -> the address of its first effect's magnitude (forms live for the whole run)
    std::map<uint32_t, bool> missing_;        // logged once
    bool broken_ = false;
};

}  // namespace nvcoop::rebalance
