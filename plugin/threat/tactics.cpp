// plugin/threat: combat style tactics (tactics.h). Mutation twins 1040..1049 (tests/test_tactics.cpp); FNV_MUT(n) is false in the shipped DLL.
#include "threat/tactics.h"

#include <algorithm>

#include "fnvcoop_mut.h"

namespace nvcoop::threat {

StyleValues tacticsFor(const StyleValues& original, const TacticsConfig& cfg) {
    StyleValues v = original;
    const bool usesCover = original.takeCoverChance > 0 || FNV_MUT(1040);   // twin 1040: styles that never take cover are made to
    if (usesCover) {
        v.takeCoverChance = std::max(original.takeCoverChance, cfg.takeCoverAtLeast);
        v.coverSearchRadius = original.coverSearchRadius * cfg.coverRadiusMult;
        v.lrChance = static_cast<uint8_t>(std::max<int>(original.lrChance, cfg.sideStepAtLeast));
    }
    if (cfg.fleeWhenHurt) v.csFlags = static_cast<uint16_t>(original.csFlags | kCsFlagFleeBasedOnPersonalSurvival);
    if (FNV_MUT(1041)) v.takeCoverChance = std::min(original.takeCoverChance, cfg.takeCoverAtLeast);   // twin 1041: cover is capped instead of raised
    return v;
}

void StyleBook::configure(const TacticsConfig& cfg) {
    cfg_ = cfg;
    if (!on_) return;
    for (auto& kv : styles_) patch(kv.first, kv.second);          // new numbers: always computed from the originals, never on top of our own patch
}

void StyleBook::patch(uint32_t style, Entry& e) {
    StyleValues now;
    if (!store_.read(style, now)) return;
    if (!e.patched && now != e.original && !FNV_MUT(1045)) {         // twin 1045: a value changed before we patched is written over with the stale original later
        e.original = now;                                          // (something else changed it since we looked: that is the original now)
    }
    e.applied = tacticsFor(FNV_MUT(1043) ? now : e.original, cfg_);   // twin 1043: the numbers are computed from what the form holds, so they compound
    if (e.applied == now) {
        e.patched = true;
        return;
    }
    if (store_.write(style, e.applied)) e.patched = true;
}

void StyleBook::restore(uint32_t style, Entry& e) {
    if (!e.patched) return;
    StyleValues now;
    if (!store_.read(style, now)) {
        e.patched = false;
        return;
    }
    if (now == e.applied || FNV_MUT(1046)) {                       // twin 1046: put back even what something else changed since
        store_.write(style, e.original);
    } else {
        ++foreign_;
    }
    e.patched = false;
}

void StyleBook::setOn(bool on) {
    if (on == on_) return;
    on_ = on;
    for (auto& kv : styles_) {
        if (on_) patch(kv.first, kv.second);
        else restore(kv.first, kv.second);
    }
}

void StyleBook::see(uint32_t style) {
    if (!style) return;
    auto it = styles_.find(style);
    if (it == styles_.end()) {
        Entry e;
        if (!store_.read(style, e.original)) return;
        e.applied = e.original;
        it = styles_.emplace(style, e).first;
    }
    if (on_ && !it->second.patched) patch(style, it->second);
}

void StyleBook::restoreAll() {
    for (auto& kv : styles_) restore(kv.first, kv.second);
    if (!FNV_MUT(1048)) on_ = false;                                // twin 1048: still "on" after everything was put back
}

size_t StyleBook::patched() const {
    size_t n = 0;
    for (const auto& kv : styles_) n += kv.second.patched ? 1 : 0;
    return n;
}

bool StyleBook::original(uint32_t style, StyleValues& out) const {
    auto it = styles_.find(style);
    if (it == styles_.end()) return false;
    out = it->second.original;
    return true;
}

}  // namespace nvcoop::threat
