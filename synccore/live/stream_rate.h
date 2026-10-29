// The player stream's rate planner (docs/entity-core-plan.md S10): moved here from plugin/avatar/stream_rate.h so that the avatar layer and the
// actor module (ActorSync's player stream profile) plan the stream with the same code. plugin/avatar/stream_rate.h keeps the old names as aliases.
//
// How often each player's avatar stream goes to each other player (FULL-SYNC-PLAN section 2: 20 Hz near, 5 Hz far; the shipped near rate is 30 Hz,
// engine/avatar_tuning.h sets nearPeriodMs and nvcoop.ini [avatar] nearHz overrides it).
//
//   Near  same cell, within nearRadius (2800 units, about 40 m, with 1.25x hysteresis so nobody flaps at the border)   every snapshot (20 Hz)
//   Far   same cell beyond that, or an adjacent cell (the caller's adjacency test)                                      one per farPeriodMs (5 Hz)
//   Out   anywhere else (another interior / worldspace)                                                                 one per outPeriodMs (1 Hz)
//
// "Out" is NOT silence: the ownership and spawn services want every player's place even when no puppet can exist here, so the stream
// trickles. A place that is not known yet counts as Near (a newcomer is never starved).
//
// Latest wins: a snapshot that arrives before its turn is kept, and poll() releases it when the turn comes, so the final position of a
// player who stopped (or left the area) is never lost behind the rate limit. Game-free, header only, uses only synccore/live types.
//
// The same planner serves both directions: the host decides what each CLIENT gets of every other player (relay); a client decides what the
// HOST gets of the client (uplink: the highest tier any other player has for it, because the host relays that stream onward).
#pragma once
#include <cmath>
#include <cstdint>
#include <functional>
#include <map>
#include <set>
#include <utility>
#include <vector>

#include "live/live.h"

namespace fnvcoop::live {

struct StreamRateConfig {
    bool enabled = true;            // false: every snapshot goes to everybody (the old behaviour)
    uint32_t nearPeriodMs = 50;     // 20 Hz
    uint32_t farPeriodMs = 200;     // 5 Hz
    uint32_t outPeriodMs = 1000;    // 1 Hz
    uint32_t slackMs = 10;          // a snapshot this much early (or a sixteenth of the period, whichever is more) still counts as on time
    float nearRadius = 2800.0f;     // units
    float hysteresis = 1.25f;
    // A snapshot whose place is further than this from the last one sent on the link (or in another cell) is a teleport: it goes at once, whatever the tier's
    // period says, and carries the new place to the observer (whose tier for this stream is decided by the places). Held for a far or out period (200 ms, 1 s)
    // it was the time the other copy's puppet stood at the old place (lab spawn room, "arrives 650 to 2700 ms late"). 0 = off.
    float jumpUnits = 350.0f;
};

enum class StreamTier : uint8_t { Out = 0, Far = 1, Near = 2 };

class StreamRatePlanner {
public:
    using Slot = uint8_t;
    static constexpr Slot kUplink = 0xFE;   // pseudo observer: "whoever the host relays me to"

    explicit StreamRatePlanner(StreamRateConfig cfg = {}) : cfg_(cfg) {}
    const StreamRateConfig& config() const { return cfg_; }
    void setConfig(const StreamRateConfig& c) { cfg_ = c; }
    // Optional: exterior cells that are loaded next to each other count as Far instead of Out.
    void setAdjacency(std::function<bool(CellId, CellId)> f) { adjacent_ = std::move(f); }

    void setPlace(Slot slot, CellId cell, const Vec3& pos) { places_[slot] = Place{cell, pos}; }
    void removePlace(Slot slot) {
        places_.erase(slot);
        for (auto it = links_.begin(); it != links_.end();) {
            if (it->first.first == slot || it->first.second == slot) it = links_.erase(it);
            else ++it;
        }
    }
    bool hasPlace(Slot slot) const { return places_.count(slot) != 0; }
    void clear() { places_.clear(); links_.clear(); }

    // The tier `observer` has for `subject` right now (with hysteresis against the previous answer).
    StreamTier tierOf(Slot observer, Slot subject) {
        auto o = places_.find(observer);
        auto s = places_.find(subject);
        if (o == places_.end() || s == places_.end()) return StreamTier::Near;
        Link& L = links_[{observer, subject}];
        StreamTier t = StreamTier::Out;
        if (o->second.cell == s->second.cell) {
            const float d = distance(o->second.pos, s->second.pos);
            const float limit = L.tier == StreamTier::Near ? cfg_.nearRadius * cfg_.hysteresis : cfg_.nearRadius;
            t = d <= limit ? StreamTier::Near : StreamTier::Far;
        } else if (adjacent_ && adjacent_(o->second.cell, s->second.cell)) {
            t = StreamTier::Far;
        }
        L.tier = t;
        return t;
    }
    uint32_t periodOf(StreamTier t) const {
        return t == StreamTier::Near ? cfg_.nearPeriodMs : (t == StreamTier::Far ? cfg_.farPeriodMs : cfg_.outPeriodMs);
    }

    // Offers the newest snapshot of `subject` for `observer`. true = send `out` now. false = kept for poll().
    bool offer(Slot observer, Slot subject, const ActorSnapshot& snap, uint32_t nowMs, ActorSnapshot& out) {
        return offerAt(observer, subject, tierOf(observer, subject), snap, nowMs, out);
    }
    // Same as offer() with the tier decided by the caller.
    bool offerAtTier(Slot observer, Slot subject, StreamTier t, const ActorSnapshot& snap, uint32_t nowMs,
                     ActorSnapshot& out) {
        return offerAt(observer, subject, t, snap, nowMs, out);
    }
    // The uplink of the local player: the highest tier of any observer in `observers` (empty = Near).
    bool offerUplink(Slot subject, const std::vector<Slot>& observers, const ActorSnapshot& snap, uint32_t nowMs,
                     ActorSnapshot& out) {
        StreamTier best = observers.empty() ? StreamTier::Near : StreamTier::Out;
        for (Slot o : observers) {
            StreamTier t = tierOf(o, subject);
            if (static_cast<uint8_t>(t) > static_cast<uint8_t>(best)) best = t;
        }
        return offerAt(kUplink, subject, best, snap, nowMs, out);
    }

    struct Release {
        Slot observer = 0, subject = 0;
        ActorSnapshot snap;
    };
    // Snapshots that were kept and whose turn has come. Call once per tick.
    void poll(uint32_t nowMs, std::vector<Release>& out) {
        for (auto& kv : links_) {
            Link& L = kv.second;
            if (!L.hasPending) continue;
            if (!dueAt(L, L.pendingTier, nowMs)) continue;
            L.hasPending = false;
            L.sent = true;
            L.last = nowMs;
            L.lastCell = L.pending.cell;
            L.lastPos = L.pending.pos;
            out.push_back(Release{kv.first.first, kv.first.second, L.pending});
        }
    }
    uint64_t kept() const { return kept_; }
    uint64_t pending() const {
        uint64_t n = 0;
        for (const auto& kv : links_) n += kv.second.hasPending ? 1 : 0;
        return n;
    }

private:
    struct Place { CellId cell = 0; Vec3 pos; };
    struct Link {
        StreamTier tier = StreamTier::Out;
        bool sent = false;
        uint32_t last = 0;
        bool hasPending = false;
        StreamTier pendingTier = StreamTier::Near;
        ActorSnapshot pending;
        CellId lastCell = 0;     // where the last snapshot that was sent stood
        Vec3 lastPos;
    };

    bool jumped(const Link& L, const ActorSnapshot& snap) const {
        if (!L.sent || cfg_.jumpUnits <= 0 || !cfg_.enabled) return false;
        return snap.cell != L.lastCell || distance(snap.pos, L.lastPos) > cfg_.jumpUnits;
    }

    bool dueAt(const Link& L, StreamTier t, uint32_t now) const {
        if (!cfg_.enabled || !L.sent) return true;
        const uint32_t period = periodOf(t);
        if (period == 0) return true;
        // a hop that relays an already throttled stream sees its arrivals jitter: allow a sixteenth of the period (at least slackMs) early
        const uint32_t slack = period / 16 > cfg_.slackMs ? period / 16 : cfg_.slackMs;
        return static_cast<uint32_t>(now - L.last) + slack >= period;
    }
    bool offerAt(Slot observer, Slot subject, StreamTier t, const ActorSnapshot& snap, uint32_t now,
                 ActorSnapshot& out) {
        Link& L = links_[{observer, subject}];
        if (dueAt(L, t, now) || jumped(L, snap)) {
            L.sent = true;
            L.last = now;
            L.hasPending = false;
            L.lastCell = snap.cell;
            L.lastPos = snap.pos;
            out = snap;
            return true;
        }
        L.hasPending = true;
        L.pendingTier = t;
        L.pending = snap;
        ++kept_;
        return false;
    }

    StreamRateConfig cfg_;
    std::function<bool(CellId, CellId)> adjacent_;
    std::map<Slot, Place> places_;
    std::map<std::pair<Slot, Slot>, Link> links_;
    uint64_t kept_ = 0;
};

}  // namespace fnvcoop::live
