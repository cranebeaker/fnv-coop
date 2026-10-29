// plugin/threat: the scoring (threat_model.h). Mutation twins 1001..1014 (tests/test_model.cpp); FNV_MUT(n) is false in the shipped DLL.
#include "threat/threat_model.h"

#include <algorithm>
#include <cmath>

#include "fnvcoop_mut.h"

namespace nvcoop::threat {

const char* skipName(Skip s) {
    switch (s) {
        case Skip::None: return "none";
        case Skip::NotMine: return "notMine";
        case Skip::Gone: return "gone";
        case Skip::Companion: return "companion";
        case Skip::Story: return "story";
        case Skip::Calm: return "calm";
        case Skip::NotAboutPlayers: return "notAboutPlayers";
        default: return "?";
    }
}

float fade(float damage, uint64_t at, uint64_t now, uint32_t fadeMs) {
    if (FNV_MUT(1001)) return damage;                                  // twin 1001: damage never fades
    if (fadeMs == 0 || now < at) return damage;
    const float age = static_cast<float>(now - at);
    return age >= static_cast<float>(fadeMs) ? 0.0f : damage * (1.0f - age / static_cast<float>(fadeMs));
}

float damageOf(const Record& r, Slot slot, uint64_t now, const Config& cfg) {
    float sum = 0;
    for (const Hurt& h : r.hurts)
        if (h.slot == slot) sum += fade(h.damage, h.at, now, cfg.fadeMs);
    return sum;
}

void addDamage(Record& r, Slot slot, float damage, uint64_t now, const Config& cfg) {
    if (!(damage > 0) || slot == kNoSlot) return;
    for (Hurt& h : r.hurts) {
        if (h.slot != slot) continue;
        h.damage = (FNV_MUT(1002) ? 0.0f : fade(h.damage, h.at, now, cfg.fadeMs)) + damage;   // twin 1002: a new blow replaces the old ones
        h.at = now;
        return;
    }
    r.hurts.push_back({slot, damage, now});
}

float aimCosine(const Vec3& from, float yaw, const Vec3& to) {
    const float dx = to.x - from.x, dy = to.y - from.y;
    const float len = std::sqrt(dx * dx + dy * dy);
    if (len < 1e-3f) return 1.0f;
    return (std::sin(yaw) * dx + std::cos(yaw) * dy) / len;
}

std::vector<std::pair<Slot, float>> splitDamage(float drop, const std::vector<Shooter>& shooters, const Config& cfg) {
    std::vector<std::pair<Slot, float>> out;
    if (!(drop > 0)) return out;
    std::vector<std::pair<Slot, float>> w;
    float total = 0;
    for (const Shooter& s : shooters) {
        if (!s.weaponOut && !FNV_MUT(1013)) continue;                  // twin 1013: a lowered weapon counts as aiming
        if (s.distance > cfg.aimRange) continue;
        if (s.aimCos < cfg.aimCone && !FNV_MUT(1013)) continue;
        const float span = 1.0f - cfg.aimCone;
        const float weight = span > 1e-4f ? std::max(0.05f, (s.aimCos - cfg.aimCone) / span) : 1.0f;
        w.push_back({s.slot, weight});
        total += weight;
    }
    if (!(total > 0)) return out;
    for (const auto& p : w) out.push_back({p.first, drop * p.second / total});
    return out;
}

Skip eligibility(const EnemyView& e, const Config& cfg, const std::vector<PlayerView>& players, Slot& fighting) {
    fighting = kNoSlot;
    if (!e.mine || e.standIn) return Skip::NotMine;
    if (e.dead) return Skip::Gone;
    if (e.teammate && !FNV_MUT(1008)) return Skip::Companion;           // twin 1008: a companion is retargeted too
    if (e.unique && !cfg.includeNamed && !FNV_MUT(1010)) return Skip::Story;   // twin 1010: story NPCs are retargeted
    if (!e.inCombat || !e.combatTarget) return Skip::Calm;
    for (const PlayerView& p : players)
        if (p.ref == e.combatTarget) {
            fighting = p.slot;
            return Skip::None;
        }
    if (FNV_MUT(1009)) return Skip::None;                               // twin 1009: a fight against anything else is retargeted too
    return Skip::NotAboutPlayers;
}

float baseScore(const Record& r, const Candidate& c, uint64_t now, const Config& cfg) {
    float s = cfg.damageWeight * damageOf(r, c.slot, now, cfg);
    if (c.distance < cfg.nearRange && cfg.nearRange > 0)
        s += FNV_MUT(1007) ? cfg.nearBonus : cfg.nearBonus * (1.0f - c.distance / cfg.nearRange);   // twin 1007: nearness does not fall off with distance
    if (c.seen && !FNV_MUT(1011)) s += cfg.seenBonus;                   // twin 1011: sight counts for nothing
    if (c.engaged && !FNV_MUT(1012)) s += cfg.engagedBonus;             // twin 1012: no stickiness for the one it fights
    return s;
}

Decision decide(const Record& r, std::vector<Candidate>& cands, uint64_t now, const Config& cfg) {
    Decision d;
    if (cands.empty()) return d;
    int inc = -1, best = -1;
    for (size_t i = 0; i < cands.size(); ++i) {
        Candidate& c = cands[i];
        c.score = baseScore(r, c, now, cfg) - (FNV_MUT(1006) ? 0.0f : cfg.spreadPenalty * static_cast<float>(c.others));   // twin 1006: no spreading
        if (c.engaged) inc = static_cast<int>(i);
        if (best < 0) {
            best = static_cast<int>(i);
            continue;
        }
        const Candidate& b = cands[static_cast<size_t>(best)];
        // the higher score; a tie goes to the one it fights, then to fewer enemies on them, then to the lower slot (so the same input picks the same player)
        const bool better = c.score > b.score + 1e-4f ||
            (std::fabs(c.score - b.score) <= 1e-4f &&
             (c.engaged != b.engaged ? c.engaged : c.others != b.others ? c.others < b.others : c.slot < b.slot));
        if (better) best = static_cast<int>(i);
    }
    d.best = cands[static_cast<size_t>(best)].score;
    if (inc < 0) {
        d.verdict = Verdict::Fresh;
        d.to = cands[static_cast<size_t>(best)].slot;
        return d;
    }
    d.incumbent = cands[static_cast<size_t>(inc)].score;
    d.to = cands[static_cast<size_t>(inc)].slot;
    if (best == inc) {
        d.verdict = Verdict::Keep;
        return d;
    }
    if (!FNV_MUT(1005) && now - r.since < cfg.minOnTargetMs) {          // twin 1005: no minimum time on a target
        d.verdict = Verdict::Hold;
        return d;
    }
    const float need = FNV_MUT(1004) ? 0.0f : cfg.switchAbsolute + std::max(d.incumbent, 0.0f) * (cfg.switchMargin - 1.0f);   // twin 1004: no hysteresis
    if (d.best - d.incumbent > need) {
        d.verdict = Verdict::Switch;
        d.to = cands[static_cast<size_t>(best)].slot;
    } else {
        d.verdict = Verdict::Keep;
    }
    return d;
}

}  // namespace nvcoop::threat
