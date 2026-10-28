// Client/host side helper for anchor tethers (campaign gap G25): turns "where is my player" into the ReportNear / ReportFar transitions the RulesEngine waits for.
//
// A PartyTether with `anchor` + `radius` is satisfied when every joined player was reported within it. Nothing used to send those reports. The plugin feeds this class the
// anchors it should watch (position read from the placed ref, radius from the rule) and the local player's position every ~30 frames; it returns only the TRANSITIONS, with
// a little hysteresis so a player standing on the radius does not flap. The host reports itself with RulesEngine::onNearReported(0, ...), a guest sends
// ClientSession::submitAction(IntentKind::ReportNear / ReportFar, anchor). reset() forgets the state so a rejoin or a loaded save re-reports everything.
#pragma once
#include <cmath>
#include <vector>
#include "fnvcoop/mutations.h"
#include "fnvcoop/types.h"

namespace fnvcoop {

class AnchorWatch {
public:
    struct Anchor { FormRef id; float x = 0, y = 0, z = 0, radius = 0; };
    struct Change { FormRef anchor; bool within = false; };

    // Replaces the watched set. State of anchors that stay is kept; removed ones are forgotten.
    void set(const std::vector<Anchor>& anchors) {
        std::vector<Entry> next;
        for (auto& a : anchors) {
            Entry e{a, false, false};
            for (auto& old : entries_) if (old.a.id == a.id) { e.within = old.within; e.known = old.known; }
            next.push_back(e);
        }
        entries_ = std::move(next);
    }
    // Position of the player (any cell: a ref in another cell is simply far). Returns the anchors whose within/far state changed (the first call reports every anchor once).
    std::vector<Change> update(float px, float py, float pz) {
        std::vector<Change> out;
        for (auto& e : entries_) {
            const float dx = px - e.a.x, dy = py - e.a.y, dz = pz - e.a.z;
            const float d2 = dx * dx + dy * dy + dz * dz;
            const float inR = e.a.radius, outR = mutations().anchorWatchNoHysteresis ? e.a.radius : e.a.radius * 1.1f;
            const bool nowNear = e.within ? d2 <= outR * outR : d2 <= inR * inR;
            if (!e.known || nowNear != e.within) { e.known = true; e.within = nowNear; out.push_back({e.a.id, nowNear}); }
        }
        return out;
    }
    // The player is in a cell where none of the anchors can be (another interior): far from all of them.
    std::vector<Change> allFar() {
        std::vector<Change> out;
        for (auto& e : entries_) if (!e.known || e.within) { e.known = true; e.within = false; out.push_back({e.a.id, false}); }
        return out;
    }
    void reset() { for (auto& e : entries_) e.known = false; }
    bool isNear(const FormRef& id) const { for (auto& e : entries_) if (e.a.id == id) return e.within; return false; }
    size_t size() const { return entries_.size(); }

private:
    struct Entry { Anchor a; bool within = false; bool known = false; };
    std::vector<Entry> entries_;
};

}  // namespace fnvcoop
