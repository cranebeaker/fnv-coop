// plugin/threat: the director (threat_director.h). Mutation twins 1020..1031 (tests/test_director.cpp); FNV_MUT(n) is false in the shipped DLL.
#include "threat/threat_director.h"

#include <algorithm>
#include <cmath>
#include <cstdio>

#include "fnvcoop_mut.h"

namespace nvcoop::threat {

namespace {

float dist3(const Vec3& a, const Vec3& b) {
    const float dx = a.x - b.x, dy = a.y - b.y, dz = a.z - b.z;
    return std::sqrt(dx * dx + dy * dy + dz * dz);
}

struct Work {
    const EnemyView* e = nullptr;
    Record* r = nullptr;
    Slot fighting = kNoSlot;
};

const char* verdictName(Verdict v) {
    switch (v) {
        case Verdict::None: return "none";
        case Verdict::Keep: return "keep";
        case Verdict::Hold: return "hold";
        case Verdict::Switch: return "switch";
        case Verdict::Fresh: return "fresh";
    }
    return "?";
}

}  // namespace

void ThreatDirector::note(const std::string& line) {
    recent_.push_back(line);
    while (recent_.size() > 24) recent_.pop_front();
    if (log_) log_(line);
}

void ThreatDirector::reset() {
    book_.clear();
    nextAt_ = 0;
}

const Record* ThreatDirector::record(RefId enemy) const {
    auto it = book_.find(enemy);
    return it == book_.end() ? nullptr : &it->second;
}

Slot ThreatDirector::targetOf(RefId enemy) const {
    const Record* r = record(enemy);
    return r ? r->target : kNoSlot;
}

void ThreatDirector::noteHit(RefId enemy, Slot slot, float damage, uint64_t now) {
    if (!enemy || slot == kNoSlot) return;
    if (!(damage > 0) && !FNV_MUT(1030)) return;                  // twin 1030: (a blow of nothing is kept as a record)
    auto it = book_.find(enemy);
    if (it == book_.end()) {
        if (book_.size() >= 512) return;                          // (a hit feed on a huge battle must not grow the book without end)
        it = book_.emplace(enemy, Record{}).first;
        it->second.seenAt = now;
        it->second.since = now >= cfg_.minOnTargetMs ? now - cfg_.minOnTargetMs : 0;
    }
    addDamage(it->second, slot, damage, now, cfg_);
    it->second.fedAt = now;
    ++stats_.hitsNoted;
    stats_.damageNoted += static_cast<uint64_t>(damage > 0 ? damage : 0);
}

void ThreatDirector::infer(Record& r, const EnemyView& e, const std::vector<PlayerView>& players, float drop, uint64_t now) {
    std::vector<Shooter> shooters;
    for (const PlayerView& p : players) {
        if (!p.present || p.downed || p.space != e.space) continue;
        Shooter s;
        s.slot = p.slot;
        s.distance = dist3(p.pos, e.pos);
        s.aimCos = aimCosine(p.pos, p.yaw, e.pos);
        s.weaponOut = p.weaponOut;
        shooters.push_back(s);
    }
    bool any = false;
    for (const auto& part : splitDamage(drop, shooters, cfg_)) {
        addDamage(r, part.first, part.second, now, cfg_);
        any = true;
    }
    if (any) stats_.damageInferred += static_cast<uint64_t>(drop);
}

void ThreatDirector::tick(uint64_t now) {
    if (!cfg_.enabled || now < nextAt_) return;
    nextAt_ = now + cfg_.tickMs;
    ++stats_.ticks;

    std::vector<PlayerView> players;
    port_.players(players);
    std::map<Slot, const PlayerView*> bySlot;
    size_t present = 0;
    for (const PlayerView& p : players)
        if (p.present) {
            bySlot[p.slot] = &p;
            ++present;
        }
    if (present < 2) {                                             // one player here: nothing to split
        ++stats_.soloTicks;
        for (auto& kv : book_) kv.second.seenAt = now;             // (the memory is kept: the others may be on their way)
        return;
    }

    std::vector<EnemyView> all;
    port_.enemies(all);
    std::sort(all.begin(), all.end(), [](const EnemyView& a, const EnemyView& b) { return a.ref < b.ref; });

    // ---- who is a fight we may touch; what each remembers ----
    std::vector<Work> work;
    for (const EnemyView& e : all) {
        Slot fighting = kNoSlot;
        const Skip skip = eligibility(e, cfg_, players, fighting);
        ++stats_.skipped[static_cast<size_t>(skip)];
        if (skip != Skip::None) continue;
        ++stats_.enemiesSeen;
        auto it = book_.find(e.ref);
        if (it == book_.end()) {
            it = book_.emplace(e.ref, Record{}).first;
            it->second.target = fighting;
            it->second.since = now >= cfg_.minOnTargetMs ? now - cfg_.minOnTargetMs : 0;   // a new fight may be split at once
        }
        Record& r = it->second;
        r.seenAt = now;
        if (r.target == kNoSlot) {
            r.target = fighting;                                   // (first sight of a record a hit feed made: it may be split at once)
        } else if (fighting != r.target) {
            // somebody changed its target since we looked: the engine going back on what we assigned, or its own AI picking another
            const bool overruled = r.assigned != kNoSlot && r.assigned == r.target && now - r.assignedAt <= cfg_.overrideWindowMs && !FNV_MUT(1025);   // twin 1025: the engine's own AI is never noticed fighting us
            if (overruled) {
                if (r.overrides == 0 || now - r.overrideFrom > 10000) {
                    r.overrides = 0;
                    r.overrideFrom = now;
                }
                ++r.overrides;
                ++stats_.overrides;
                if (r.overrides >= cfg_.overrideLimit) {
                    r.quietUntil = now + cfg_.quietMs;
                    r.overrides = 0;
                    ++stats_.quieted;
                }
            }
            r.target = fighting;
            r.since = now;
        }
        if (cfg_.inferDamage) {
            float h = 0;
            if (port_.health(e.ref, h)) {
                if (r.lastHealth >= 0 && h < r.lastHealth - 0.5f && (now - r.fedAt >= cfg_.feedQuietMs || FNV_MUT(1026)))   // twin 1026: a blow the feed reported is counted again from the health drop
                    infer(r, e, players, r.lastHealth - h, now);
                r.lastHealth = h;
            }
        }
        work.push_back({&e, &r, fighting});
    }

    // ---- candidates (players it could go for) and their sight ----
    struct Plan {
        std::vector<Candidate> cands;
    };
    std::vector<Plan> plans(work.size());
    for (size_t i = 0; i < work.size(); ++i) {
        const EnemyView& e = *work[i].e;
        for (const PlayerView& p : players) {
            if (!p.present) continue;
            if (p.downed && !FNV_MUT(1020)) continue;             // twin 1020: a downed player can be picked
            if (p.space != e.space && !FNV_MUT(1021)) continue;   // twin 1021: a player in another space can be picked
            const float d = dist3(p.pos, e.pos);
            const bool engaged = p.slot == work[i].fighting;
            if (d > cfg_.reach && !engaged && !FNV_MUT(1022)) continue;   // twin 1022: no limit to how far it would walk for a player
            Candidate c;
            c.slot = p.slot;
            c.distance = d;
            c.engaged = engaged;
            plans[i].cands.push_back(c);
        }
    }
    uint32_t budget = cfg_.maxSightChecks;
    const size_t n = work.size();
    for (size_t k = 0; k < n; ++k) {
        const size_t i = (rotate_ + k) % n;
        Record& r = *work[i].r;
        for (Candidate& c : plans[i].cands) {
            SightMemo& m = r.sight[c.slot];
            const bool stale = m.at == 0 || now - m.at >= cfg_.sightMs;
            if (stale && (budget > 0 || FNV_MUT(1031))) {          // twin 1031: no limit to the sight checks per tick
                if (budget > 0) --budget;
                const Sight s = port_.sight(work[i].e->ref, bySlot[c.slot]->ref);
                ++stats_.sightChecks;
                if (s != Sight::Unknown) {
                    m.seen = s == Sight::Seen;
                    m.at = now;
                }
            }
            c.seen = m.seen;
        }
    }

    // ---- decisions, one enemy after the other: each sees where the ones before it are going (the spreading) ----
    std::map<RefId, Slot> planned;
    for (const Work& w : work) planned[w.e->ref] = w.fighting;
    uint32_t switches = 0;
    for (size_t k = 0; k < n; ++k) {
        const size_t i = (rotate_ + k) % n;
        Work& w = work[i];
        Record& r = *w.r;
        std::vector<Candidate>& cands = plans[i].cands;
        for (Candidate& c : cands) {
            int others = 0;
            for (const Work& o : work) {
                if (o.e == w.e && !FNV_MUT(1027)) continue;       // twin 1027: the enemy counts itself as one on its own target
                if (o.e->space != w.e->space || planned[o.e->ref] != c.slot) continue;
                if (dist3(o.e->pos, w.e->pos) > cfg_.groupRadius) continue;
                ++others;
            }
            c.others = others;
        }
        if (now < r.quietUntil) continue;                          // the engine keeps overruling us here: left to it
        const Decision d = decide(r, cands, now, cfg_);
        if (d.verdict == Verdict::Hold) ++stats_.held;
        if (d.verdict != Verdict::Switch && d.verdict != Verdict::Fresh) continue;
        if (switches >= cfg_.maxSwitches && !FNV_MUT(1023)) continue;   // twin 1023: no limit to the retargets per tick
        if (now < r.retryAt) continue;
        auto pit = bySlot.find(d.to);
        if (pit == bySlot.end()) continue;
        const bool ok = port_.retarget(w.e->ref, pit->second->ref);
        if (!ok && !FNV_MUT(1024)) {                              // twin 1024: a retarget the engine refused is taken for done
            ++stats_.failed;
            r.retryAt = now + 3000;
            continue;
        }
        ++switches;
        if (d.verdict == Verdict::Fresh) ++stats_.fresh;
        else ++stats_.switches;
        char line[160];
        std::snprintf(line, sizeof line, "threat: %08X %s: player %u -> %u (%.1f vs %.1f)", w.e->ref, verdictName(d.verdict),
                      static_cast<unsigned>(r.target), static_cast<unsigned>(d.to), static_cast<double>(d.best), static_cast<double>(d.incumbent));
        note(line);
        r.target = d.to;
        r.since = now;
        r.assigned = d.to;
        r.assignedAt = now;
        planned[w.e->ref] = d.to;
    }
    if (n) rotate_ = (rotate_ + 1) % static_cast<uint32_t>(n);

    // ---- forget the fights that ended ----
    for (auto it = book_.begin(); it != book_.end();) {
        if (!FNV_MUT(1029) && now - it->second.seenAt > cfg_.forgetMs) it = book_.erase(it);   // twin 1029: nothing is ever forgotten
        else ++it;
    }
}

std::string ThreatDirector::statusJson() const {
    char buf[900];
    std::snprintf(buf, sizeof buf,
        "{\"enabled\":%s,\"tracked\":%zu,\"ticks\":%llu,\"enemiesSeen\":%llu,\"switches\":%llu,\"fresh\":%llu,\"held\":%llu,\"failed\":%llu,\"overrides\":%llu,\"quieted\":%llu,"
        "\"sightChecks\":%llu,\"hitsNoted\":%llu,\"damageNoted\":%llu,\"damageInferred\":%llu,\"soloTicks\":%llu,"
        "\"skipped\":{\"notMine\":%llu,\"gone\":%llu,\"companion\":%llu,\"story\":%llu,\"calm\":%llu,\"notAboutPlayers\":%llu}}",
        cfg_.enabled ? "true" : "false", book_.size(), (unsigned long long)stats_.ticks, (unsigned long long)stats_.enemiesSeen, (unsigned long long)stats_.switches,
        (unsigned long long)stats_.fresh, (unsigned long long)stats_.held, (unsigned long long)stats_.failed, (unsigned long long)stats_.overrides, (unsigned long long)stats_.quieted,
        (unsigned long long)stats_.sightChecks, (unsigned long long)stats_.hitsNoted, (unsigned long long)stats_.damageNoted, (unsigned long long)stats_.damageInferred,
        (unsigned long long)stats_.soloTicks,
        (unsigned long long)stats_.skipped[static_cast<size_t>(Skip::NotMine)], (unsigned long long)stats_.skipped[static_cast<size_t>(Skip::Gone)],
        (unsigned long long)stats_.skipped[static_cast<size_t>(Skip::Companion)], (unsigned long long)stats_.skipped[static_cast<size_t>(Skip::Story)],
        (unsigned long long)stats_.skipped[static_cast<size_t>(Skip::Calm)], (unsigned long long)stats_.skipped[static_cast<size_t>(Skip::NotAboutPlayers)]);
    return buf;
}

}  // namespace nvcoop::threat
