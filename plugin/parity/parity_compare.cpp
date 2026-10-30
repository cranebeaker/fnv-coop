#include "parity/parity_compare.h"

#include <algorithm>
#include <cmath>
#include <cstdio>

#include "parity/parity_anim.h"
#include "fnvcoop_mut.h"

// Twins: 1804 = the position tolerance ignores speed; 1805 = a difference is reported the first time it is seen (no persistence); 1806 = the seated flag is not
//        compared; 1807 = the look ignores the face hash; 1809 = the rate limit key leaves out the kind; 1810 = the rate limit is not applied;
//        1811 = the summary counts every confirmed difference instead of every reference with one; 1817 = "absent" ignores how far the other sample covers;
//        1819 = the anim compare also compares Transient groups (a one-shot against a locomotion group is a difference);
//        1820 = the health gap is off by one (a gap of exactly the limit passes); 1821 = a difference that stopped is not forgotten (it stays pending)
namespace nvcoop::parity {

namespace {

std::string hex(uint32_t v) {
    char b[16];
    std::snprintf(b, sizeof b, "0x%08X", static_cast<unsigned>(v));
    return b;
}

std::string at(const Ent& e) {
    char b[64];
    std::snprintf(b, sizeof b, "(%d,%d,%d)", e.x, e.y, e.z);
    return b;
}

std::string groupText(uint8_t g) {
    if (g == kNoGroup) return "none";
    const char* n = nvcoop::actors::anim::groupName(g);
    char b[64];
    std::snprintf(b, sizeof b, "%s[%s]", className(groupClass(g)), n && *n ? n : "?");
    return b;
}

std::string lookText(const Look& l) {
    if (!l.known) return "unknown";
    char b[160];
    std::snprintf(b, sizeof b, "race=%s sex=%c hair=%s eyes=%s face=%08X", hex(l.race).c_str(), l.female ? 'F' : 'M', hex(l.hair).c_str(), hex(l.eyes).c_str(), l.faceHash);
    return b;
}

}  // namespace

double posTolerance(const Tolerances& t, uint8_t a16, uint8_t b16) {
    if (FNV_MUT(1804)) return t.posTolUnits;
    const double speed = (static_cast<double>(a16) + static_cast<double>(b16)) * 16.0 / 2.0;
    return t.posTolUnits + speed * (t.skewSeconds + t.lagSeconds);
}

bool sameArea(const Sample& a, const Sample& b) {
    if (a.ws != 0 || b.ws != 0) return a.ws == b.ws;
    return a.cell == b.cell;
}

double dist3(const Ent& a, const Ent& b) {
    const double dx = double(a.x) - b.x, dy = double(a.y) - b.y, dz = double(a.z) - b.z;
    return std::sqrt(dx * dx + dy * dy + dz * dz);
}

bool Comparer::observe(const std::string& id, bool bad, bool hard, uint64_t now) {
    if (!bad) {
        if (!FNV_MUT(1821)) pending_.erase(id);
        return false;
    }
    auto it = pending_.find(id);
    if (it == pending_.end()) it = pending_.emplace(id, Pending{now, now}).first;
    else it->second.last = now;
    if (FNV_MUT(1805)) return true;
    return hard || now - it->second.first >= tol_.persistMs;
}

void Comparer::check(CompareResult& r, bool& anyBad, const char* kind, const std::string& key, bool bad, bool hard, uint64_t now, const std::string& host,
                     const std::string& client) {
    KindCount& kc = kinds_[kind];
    ++kc.checks;
    if (observe(key + "|" + kind, bad, hard, now)) {
        ++kc.bad;
        anyBad = true;
        r.confirmed.push_back(Mismatch{kind, key, host, client});
    }
}

void Comparer::comparePair(CompareResult& r, const Ent& h, const Ent& c, const std::string& key, bool players, uint64_t now) {
    bool anyBad = false;
    pending_.erase(key + "|absent");   // both machines have it now
    {
        const bool bad = h.disabled != c.disabled;
        check(r, anyBad, "disabled", key, bad, false, now, bad ? (h.disabled ? "disabled" : "enabled") : "", bad ? (c.disabled ? "disabled" : "enabled") : "");
    }
    if (!h.disabled && !c.disabled) {
        {
            const bool bad = h.dead != c.dead;
            check(r, anyBad, "dead", key, bad, false, now, bad ? (h.dead ? "dead" : "alive") : "", bad ? (c.dead ? "dead" : "alive") : "");
        }
        if (!h.dead && !c.dead) {
            const double d = dist3(h, c);
            const double tol = posTolerance(tol_, h.speed16, c.speed16);
            const bool bad = d > tol;
            char b[96];
            std::snprintf(b, sizeof b, " off=%.0f tol=%.0f", d, tol);
            check(r, anyBad, "pos", key, bad, d > tol_.posHardUnits, now, bad ? at(h) : "", bad ? at(c) + b : "");
            if (!FNV_MUT(1806)) {
                const bool sb = h.seated != c.seated;
                check(r, anyBad, "seated", key, sb, false, now, sb ? (h.seated ? "seated" : "standing") : "", sb ? (c.seated ? "seated" : "standing") : "");
            }
            {
                const GroupClass hc = groupClass(h.group), cc = groupClass(c.group);
                const bool comparable = hc != GroupClass::Unknown && cc != GroupClass::Unknown && (FNV_MUT(1819) || (hc != GroupClass::Transient && cc != GroupClass::Transient));
                const bool bad2 = comparable && hc != cc;
                if (comparable) check(r, anyBad, "anim", key, bad2, false, now, bad2 ? groupText(h.group) : "", bad2 ? groupText(c.group) : "");
            }
            if (h.hp != kNoHp && c.hp != kNoHp) {
                const int gap = std::abs(int(h.hp) - int(c.hp));
                const bool bad3 = gap >= tol_.hpBucketGap + (FNV_MUT(1820) ? 1 : 0);
                char hb[24], cb[24];
                std::snprintf(hb, sizeof hb, "bucket %u", h.hp);
                std::snprintf(cb, sizeof cb, "bucket %u", c.hp);
                check(r, anyBad, "hp", key, bad3, false, now, bad3 ? hb : "", bad3 ? cb : "");
            }
        }
    }
    if (players) {
        // the look: one side is the player itself, the other its copy
        const Ent* self = h.kind == EntKind::Self ? &h : c.kind == EntKind::Self ? &c : nullptr;
        const Ent* copy = h.kind == EntKind::Puppet ? &h : c.kind == EntKind::Puppet ? &c : nullptr;
        if (self && copy && self->look.known) {
            const Look& a = self->look;
            const Look& b = copy->look;
            bool differs = !b.known || a.race != b.race || a.female != b.female || a.hair != b.hair || a.eyes != b.eyes;
            if (!FNV_MUT(1807) && b.known && a.faceHash != b.faceHash) differs = true;
            check(r, anyBad, "look", key, differs, false, now, differs ? (&h == self ? lookText(a) : lookText(b)) : "", differs ? (&c == self ? lookText(a) : lookText(b)) : "");
        }
        // attack animations the owner played that its copy never showed (or the other way round)
        if (h.kind != EntKind::Npc && c.kind != EntKind::Npc) {
            ActsState& st = acts_[h.slot];
            if (!st.have || h.acts < st.lastH || c.acts < st.lastC) {
                st.have = true;
                st.base = int64_t(h.acts) - int64_t(c.acts);   // from here on only a change in the gap counts (a copy that started late missed older swings)
            }
            st.lastH = h.acts;
            st.lastC = c.acts;
            const int64_t dev = (int64_t(h.acts) - int64_t(c.acts)) - st.base;
            const bool bad = std::llabs(dev) >= tol_.actsGap;
            char hb[32], cb[64];
            std::snprintf(hb, sizeof hb, "%u attacks", h.acts);
            std::snprintf(cb, sizeof cb, "%u attacks (%+lld apart)", c.acts, static_cast<long long>(-dev));
            check(r, anyBad, "action", key, bad, false, now, bad ? hb : "", bad ? cb : "");
        }
    }
    ++r.refs;
    r.keysCompared.push_back(key);
    if (players) ++r.players;
    if (anyBad) {
        ++r.refsBad;
        r.keysBad.push_back(key);
        if (players) ++r.playersBad;
    }
}

CompareResult Comparer::compare(const Sample& host, const Sample& client, uint64_t nowMs) {
    CompareResult r;
    for (auto it = pending_.begin(); it != pending_.end();) {
        if (nowMs - it->second.last > tol_.staleMs) it = pending_.erase(it);
        else ++it;
    }
    if (host.menu || client.menu) {
        r.skipped = true;
        r.skipReason = "a menu is open";
        return r;
    }
    if (!sameArea(host, client)) {
        r.skipped = true;
        r.skipReason = "the players are in different places";
        return r;
    }
    const Ent* hp = host.bySlot(host.slot);
    const Ent* cp = client.bySlot(client.slot);
    // actors
    for (const Ent& h : host.ents) {
        if (h.kind != EntKind::Npc || (h.ref >> 24) == 0xFF) continue;   // created actors have a different local reference on each machine
        const Ent* c = client.byRef(h.ref);
        const std::string key = hex(h.ref);
        if (c) {
            comparePair(r, h, *c, key, false, nowMs);
        } else if (cp && !h.disabled) {
            const bool covered = FNV_MUT(1817) || dist3(h, *cp) <= double(client.cov);
            if (covered) {
                bool any = false;
                check(r, any, "absent", key, true, false, nowMs, at(h), "missing");
                ++r.refs;
                r.keysCompared.push_back(key);
                if (any) { ++r.refsBad; r.keysBad.push_back(key); }
            }
        }
    }
    for (const Ent& c : client.ents) {
        if (c.kind != EntKind::Npc || (c.ref >> 24) == 0xFF || host.byRef(c.ref) || c.disabled || !hp) continue;
        if (!(FNV_MUT(1817) || dist3(c, *hp) <= double(host.cov))) continue;
        bool any = false;
        const std::string key = hex(c.ref);
        check(r, any, "absent", key, true, false, nowMs, "missing", at(c));
        ++r.refs;
        r.keysCompared.push_back(key);
        if (any) { ++r.refsBad; r.keysBad.push_back(key); }
    }
    // players: the entity for slot k on each machine (its own player, or the copy of it)
    std::set<uint8_t> slots;
    for (const Ent& e : host.ents) if (e.kind != EntKind::Npc) slots.insert(e.slot);
    for (const Ent& e : client.ents) if (e.kind != EntKind::Npc) slots.insert(e.slot);
    for (uint8_t s : slots) {
        const Ent* h = host.bySlot(s);
        const Ent* c = client.bySlot(s);
        const std::string key = "player" + std::to_string(s);
        if (h && c) {
            comparePair(r, *h, *c, key, true, nowMs);
        } else {
            bool any = false;
            check(r, any, "absent", key, true, false, nowMs, h ? at(*h) : "missing", c ? at(*c) : "missing");
            ++r.refs;
            ++r.players;
            r.keysCompared.push_back(key);
            if (any) { ++r.refsBad; ++r.playersBad; r.keysBad.push_back(key); }
        }
    }
    return r;
}

// ---- reporter ---------------------------------------------------------------------------------------------------------------------------------------------

std::vector<std::string> Reporter::report(const CompareResult& r, uint64_t nowMs) {
    if (!started_) {
        started_ = true;
        nextSummary_ = nowMs + tol_.summaryMs;
    }
    if (r.skipped) return {};
    ++windowRounds_;
    for (const std::string& k : r.keysCompared) windowRefs_.insert(k);
    if (FNV_MUT(1811)) {
        for (const Mismatch& m : r.confirmed) windowBad_.insert(m.key + "|" + m.kind);
    } else {
        for (const std::string& k : r.keysBad) windowBad_.insert(k);
    }
    std::vector<std::string> lines;
    for (const Mismatch& m : r.confirmed) {
        const std::string id = FNV_MUT(1809) ? m.key : m.key + "|" + m.kind;
        auto it = lastLogged_.find(id);
        if (!FNV_MUT(1810) && it != lastLogged_.end() && nowMs - it->second < tol_.logEveryMs) {
            ++suppressed_;
            continue;
        }
        lastLogged_[id] = nowMs;
        ++logged_;
        lines.push_back("DESYNC " + m.kind + " ref " + m.key + " host=" + m.host + " client=" + m.client);
    }
    return lines;
}

std::string Reporter::summary(uint64_t nowMs) {
    char b[96];
    std::snprintf(b, sizeof b, "summary %zu refs compared, %zu mismatching", windowRefs_.size(), windowBad_.size());
    windowRefs_.clear();
    windowBad_.clear();
    windowRounds_ = 0;
    nextSummary_ = nowMs + tol_.summaryMs;
    return b;
}

}  // namespace nvcoop::parity
