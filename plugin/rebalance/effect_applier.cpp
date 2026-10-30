#include "rebalance/effect_applier.h"

#include <cmath>

namespace nvcoop::rebalance {

const std::vector<EffectEntry>& effectTable() {
    // FalloutNV.esm form ids (load order slot 0, so the runtime ids are the file ids) of the two ingestibles this module rewrites, read from the ESM's ALCH records:
    // EDID Stimpak = 0x15169, EDID NVSuperStimpak = 0xCCEF2. The port still reads each form's own name before it touches it.
    static const std::vector<EffectEntry> t = {
        {0x00015169u, "Stimpak"},
        {0x000CCEF2u, "Super Stimpak"},
    };
    return t;
}

float effectValue(float original, const Profile& p) { return original * p.stimpakHeal; }

bool EffectApplier::apply(const Profile& p) {
    if (p.vanilla() && written_.empty()) return true;   // the game's own numbers, never touched: nothing to read or write
    bool allOk = true;
    for (const EffectEntry& e : effectTable()) {
        auto it = originals_.find(e.formId);
        if (it == originals_.end()) {
            float v = 0;
            if (!e_.read(e.formId, v)) { ++readsFailed_; allOk = false; continue; }
            it = originals_.emplace(e.formId, v).first;
        }
        float want = effectValue(it->second, p);
        if (FNV_MUT(1480)) want = effectValue(lastWritten(e.formId), p);   // MUTANT 1480: computed from the current magnitude, so it compounds
        const float last = lastWritten(e.formId);
        float live = last;
        const bool haveLive = e_.read(e.formId, live);
        if (!haveLive) live = last;
        // nothing to do when the form already holds the value (written by us, or by an earlier instance whose state we carry)
        if (std::fabs(want - live) < 0.0005f && !FNV_MUT(1481)) {   // MUTANT 1481: written every pass
            if (std::fabs(want - it->second) >= 0.0005f) written_[e.formId] = want;
            else written_.erase(e.formId);
            continue;
        }
        if (e_.write(e.formId, want)) {
            if (std::fabs(want - it->second) < 0.0005f && !FNV_MUT(1482)) written_.erase(e.formId);   // MUTANT 1482: the game's own magnitude is not put back as "unchanged"
            else written_[e.formId] = want;
            ++writes_;
        } else {
            allOk = false;
        }
    }
    return allOk;
}

}  // namespace nvcoop::rebalance
