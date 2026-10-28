// ShadowState: the deterministic, game-free model of "story state" derived from the txn stream.
// Host and every client fold the same txns in seq order and must end with identical ShadowStates.
// Merge rules: stage = max, objective flags = OR, kills = union (Resurrect removes), globals = host wins
// (host order, last write), enable/disable/move/quest-running = host order, per-slot ledgers accumulate deltas.
#pragma once
#include <array>
#include <map>
#include <set>
#include <string>
#include <tuple>
#include <cstdio>
#include <cstdlib>
#include <unordered_map>
#include <vector>
#include "fnvcoop/effect.h"
#include "fnvcoop/refop.h"

namespace fnvcoop {

class Archive;

struct Placement {
    FormRef cell;
    float x = 0, y = 0, z = 0;
    bool operator==(const Placement& o) const { return cell == o.cell && x == o.x && y == o.y && z == o.z; }
};

// Net deltas granted to one player slot.
struct Ledger {
    std::map<FormRef, int64_t> items;
    std::map<uint32_t, int64_t> av;
    int64_t karma = 0;
    std::map<FormRef, std::pair<int64_t, int64_t>> rep;  // faction -> (fame, infamy)
    std::map<FormRef, int32_t> perks;                    // perk -> rank (0 erased)
    FormRef activeQuest; bool hasActive = false;         // ForceActiveQuest (per player)
    uint32_t controlsOff = 0;                            // disabled player-control bits
    bool empty() const { return items.empty() && av.empty() && karma == 0 && rep.empty() && perks.empty() && controlsOff == 0 && !hasActive; }
    bool operator==(const Ledger& o) const {
        return items == o.items && av == o.av && karma == o.karma && rep == o.rep && perks == o.perks &&
               controlsOff == o.controlsOff && hasActive == o.hasActive && (!hasActive || activeQuest == o.activeQuest);
    }
};

struct SpawnInfo {
    FormRef base, cell;
    float x = 0, y = 0, z = 0;
    bool leveled = false;
    bool operator==(const SpawnInfo& o) const { return base == o.base && cell == o.cell && x == o.x && y == o.y && z == o.z && leveled == o.leveled; }
};
struct ObjState {
    bool open = false, destroyed = false, activated = false;
    bool isDefault() const { return !open && !destroyed && !activated; }
    bool operator==(const ObjState& o) const { return open == o.open && destroyed == o.destroyed && activated == o.activated; }
};
struct RefInv {
    bool cleared = false;                       // RemoveAllItems happened
    std::map<FormRef, int64_t> delta;           // net AddItem - RemoveItem since then
    // Pre-image of the clear (metadata, not part of equality): the contents the container held when it was emptied, if the capturer sent them.
    std::map<FormRef, int64_t> pre;
    bool hasPre = false;
    bool operator==(const RefInv& o) const { return cleared == o.cleared && delta == o.delta; }
};

struct BarrierState {
    uint8_t arrived = 0, required = 0;
    bool released = false;
    bool operator==(const BarrierState& o) const { return arrived == o.arrived && required == o.required && released == o.released; }
};

// Folds one RefAngle into an (angles, originals) pair; returns true when the state changed. Shared by the shadow, the simulator oracle
// and the simulated world so all three agree. The first pre-image seen for an axis is its original; writing the original back removes the axis.
inline bool foldAngle(std::map<std::pair<FormRef, uint8_t>, float>& angles, std::map<std::pair<FormRef, uint8_t>, float>& orig, const RefAngle& e) {
    const std::pair<FormRef, uint8_t> key{e.ref, e.axis};
    auto it = angles.find(key);
    if (it == angles.end()) {
        if (e.hasBefore) {
            if (e.degrees == e.before) return false;      // rotated to where it already was
            orig[key] = e.before;
        }
        angles[key] = e.degrees;
        return true;
    }
    if (it->second == e.degrees) return false;
    auto o = orig.find(key);
    if (o != orig.end() && o->second == e.degrees) { angles.erase(it); orig.erase(o); return true; }
    it->second = e.degrees;
    return true;
}

// Incremental bucket digest (T1a): per-segment bucket sums, a segment recomputed only after an effect that can touch it.
// A copy starts empty (all dirty), so copying a ShadowState never copies stale sums.
struct BucketCache {
    static constexpr unsigned kSegments = 32;
    uint64_t dirty = ~0ull;                   // bit per segment
    uint32_t volSize = 0;                     // volatile-key count the sums were computed under
    uint32_t calls = 0, verifyFailures = 0;   // cached digests taken; times the periodic full check disagreed
    std::vector<uint32_t> sums;               // kSegments * 256, lazily sized
    uint32_t counts[kSegments] = {};
    // Single-key updates for the big maps (stages 3, globals 7, enabled 8, placed 10): the touched keys wait here and only those
    // entries are re-hashed; `terms` remembers each entry's last contribution to its bucket.
    static constexpr int kKeyedSlots = 4;
    static int keyedSlot(unsigned seg) { return seg == 3 ? 0 : seg == 7 ? 1 : seg == 8 ? 2 : seg == 10 ? 3 : -1; }
    uint32_t termsMask = 0;                                  // bit per segment whose `terms` are valid
    std::vector<std::pair<uint8_t, FormRef>> pending;        // (segment, key) touched since the last digest
    std::unordered_map<uint64_t, uint32_t> terms[kKeyedSlots];
    BucketCache() = default;
    BucketCache(const BucketCache&) {}
    BucketCache& operator=(const BucketCache&) { dirty = ~0ull; return *this; }
};

// Pre-images ("originals") of set-valued state, recorded the first time a slot leaves the value the machine started with, so a rebase after a
// host load can put it back. One entry per (kind, ref, index); the Origin holds the value to restore.
enum OriginKind : uint8_t { kOrgWeather = 1, kOrgTimeScale = 2, kOrgRefVar = 3, kOrgMarker = 4, kOrgLock = 5 };
using OriginKey = std::tuple<uint8_t, FormRef, uint16_t>;
struct Origin {
    FormRef f;        // weather
    double d = 0;     // time scale, ref script var
    int32_t n = 0;    // marker flags, lock level
    bool b = false;   // weather forced, lock locked
    bool operator==(const Origin& o) const { return f == o.f && d == o.d && n == o.n && b == o.b; }
};
using OriginMap = std::map<OriginKey, Origin>;

// Fold helpers shared by the shadow, the simulator oracle and the simulated world so all three agree. Each returns true when the state
// changed. Rule (same as foldAngle): the first pre-image seen for a slot is its original; writing the original back removes the slot.
inline bool foldWeather(FormRef& weather, bool& set, bool& forced, OriginMap& og, const SetWeather& e) {
    const OriginKey k{kOrgWeather, FormRef(), 0};
    if (!set) {
        if (e.hasBefore) {
            if (e.weather == e.before && e.force == e.beforeForce) return false;
            Origin o; o.f = e.before; o.b = e.beforeForce; og[k] = o;
        }
        weather = e.weather; set = true; forced = e.force;
        return true;
    }
    if (weather == e.weather && forced == e.force) return false;
    auto it = og.find(k);
    if (it != og.end() && it->second.f == e.weather && it->second.b == e.force) { weather = FormRef(); set = false; forced = false; og.erase(it); return true; }
    weather = e.weather; forced = e.force;
    return true;
}
inline bool foldTimeScale(float& scale, OriginMap& og, const SetTimeScale& e) {
    const OriginKey k{kOrgTimeScale, FormRef(), 0};
    if (scale == 0) {
        if (e.hasBefore && e.before != 0) {
            if (e.scale == e.before) return false;
            Origin o; o.d = e.before; og[k] = o;
        }
        scale = e.scale;
        return scale != 0;
    }
    if (scale == e.scale) return false;
    auto it = og.find(k);
    if (it != og.end() && float(it->second.d) == e.scale) { scale = 0; og.erase(it); return true; }
    scale = e.scale;
    return true;
}
inline bool foldRefVar(std::map<std::pair<FormRef, uint16_t>, double>& vars, OriginMap& og, const RefScriptVar& e) {
    const std::pair<FormRef, uint16_t> key{e.ref, e.index};
    const OriginKey k{kOrgRefVar, e.ref, e.index};
    auto it = vars.find(key);
    if (it == vars.end()) {
        if (e.hasBefore) {
            if (e.value == e.before) return false;
            Origin o; o.d = e.before; og[k] = o;
        }
        vars[key] = e.value;
        return true;
    }
    if (it->second == e.value) return false;
    auto o = og.find(k);
    if (o != og.end() && o->second.d == e.value) { vars.erase(it); og.erase(o); return true; }
    it->second = e.value;
    return true;
}
inline bool foldMarker(std::map<FormRef, uint8_t>& markers, OriginMap& og, const SetMapMarker& e) {
    const OriginKey k{kOrgMarker, e.marker, 0};
    auto it = markers.find(e.marker);
    if (it == markers.end()) {
        if (e.hasBefore) {
            if (e.flags == e.before) return false;
            Origin o; o.n = e.before; og[k] = o;
        }
        markers[e.marker] = e.flags;
        return true;
    }
    if (it->second == e.flags) return false;
    auto o = og.find(k);
    if (o != og.end() && o->second.n == e.flags) { markers.erase(it); og.erase(o); return true; }
    it->second = e.flags;
    return true;
}
// Lock of a ref (SetLock and the lock bit of ObjectState). Stored as (locked, level); a locked == false lock has level 0. Dumps list a lock
// for every ref, so an entry is never removed: restoring the original writes the original value (the origin stays as metadata).
inline bool foldLock(std::map<FormRef, std::pair<bool, int32_t>>& locks, OriginMap& og, const FormRef& ref, bool locked, int32_t level,
                     bool hasBefore, bool beforeLocked, int32_t beforeLevel) {
    const std::pair<bool, int32_t> v{locked, locked ? level : 0};
    auto it = locks.find(ref);
    if (it == locks.end()) {
        if (hasBefore) {
            Origin o; o.b = beforeLocked; o.n = beforeLocked ? beforeLevel : 0;
            og.emplace(OriginKey{kOrgLock, ref, 0}, o);
        }
        locks[ref] = v;
        return true;
    }
    if (it->second == v) return false;
    it->second = v;
    return true;
}

// RemoveAllItems on a ref inventory (clear, or undo of a clear with the contents it discarded). Returns true when the state changed.
inline bool foldRefClear(std::map<FormRef, RefInv>& invs, const RemoveAllItems& e, bool noClear = false) {
    if (e.undo) {
        auto it = invs.find(e.ref);
        if (it == invs.end() || !it->second.cleared) return false;
        it->second.cleared = false;   // the deltas added after the clear stay: their own reverts take them out
        it->second.pre.clear(); it->second.hasPre = false;
        if (it->second.delta.empty()) invs.erase(it);
        return true;
    }
    RefInv& inv = invs[e.ref];
    if (noClear) return false;
    if (!inv.cleared && e.hasBefore) {
        inv.pre.clear();
        for (auto& [f, c] : e.items) inv.pre[f] += c;
        inv.hasPre = true;
    }
    inv.cleared = true;
    inv.delta.clear();
    return true;
}

// State entries of the generic RefCommand: (group, ref, a, b) with the slots the op does not key on left empty (refop.h).
using RefOpKey = std::tuple<uint16_t, FormRef, FormRef, FormRef>;
inline RefOpKey refOpKeyOf(const RefCommand& c) {
    const RefOpInfo* i = refOpInfo(c.op);
    const uint16_t g = i ? i->group : c.op;
    const uint8_t k = i ? i->key : 0;
    return RefOpKey{g, (k & kKeyRef) ? c.ref : FormRef(), (k & kKeyA) ? c.a : FormRef(), (k & kKeyB) ? c.b : FormRef()};
}

class ShadowState {
public:
    mutable BucketCache bucketCache;
    uint64_t seq = 0;  // last txn folded in
    std::map<FormRef, uint32_t> stages;
    // The stage items the host ran, per quest: the live memory of applyEffect only (not compared, not serialized, not in any digest). A SetStage below the quest's
    // stage is still a stage the host ran (its done flag, GetStageDone, its journal entry): it is delivered once, the max merge does not swallow it.
    std::map<FormRef, std::set<uint32_t>> stagesRun;
    std::map<std::pair<FormRef, uint32_t>, uint8_t> objectives;
    std::map<FormRef, float> globals;
    std::map<FormRef, bool> enabled;
    std::set<FormRef> kills;
    std::map<FormRef, bool> questRunning;
    std::map<FormRef, Placement> placed;
    std::array<Ledger, kMaxSlots> ledger;
    // round 2
    std::map<std::pair<FormRef, uint16_t>, double> scriptVars;
    std::map<FormRef, bool> tethers;
    bool slideshowEnded = false, saveSuppress = false;
    std::map<FormRef, uint8_t> companionOwner;
    std::map<FormRef, BarrierState> barriers;
    std::set<FormRef> sayOnce, questCompleted, allObjectivesDone;
    std::set<FormRef> questFailed;   // subset of questCompleted (a failed quest is also completed)
    std::map<FormRef, std::pair<bool, int32_t>> locks;
    std::map<std::pair<FormRef, FormRef>, int32_t> factionRank;
    std::map<std::pair<FormRef, uint8_t>, FormRef> textures;
    std::map<FormRef, bool> fastTravelLock;   // scope -> locked
    // round 3
    std::map<FormRef, SpawnInfo> spawned;     // live created refs
    std::set<FormRef> despawned;              // tombstones (a replayed SpawnRef must not resurrect)
    uint32_t createdCounter = 0;              // highest created-ref number allocated so far
    std::map<FormRef, ObjState> objects;      // non-default object states only (lock lives in `locks`)
    std::map<std::pair<FormRef, uint16_t>, double> refVars;
    std::map<FormRef, RefInv> refInv;
    std::set<FormRef> topics, notes;
    std::map<FormRef, uint8_t> mapMarkers;
    std::map<FormRef, uint8_t> companionKind;
    FormRef weather; bool weatherSet = false, weatherForced = false;
    float timeScale = 0;
    std::map<std::pair<FormRef, uint8_t>, float> angles;      // SetAngle per (ref, axis), degrees; only axes that differ from their original
    std::map<std::pair<FormRef, uint8_t>, float> angleOrig;   // the original (pre-image) of every axis in `angles`, when known
    OriginMap origins;                                        // pre-images of weather, time scale, ref script vars, map markers and locks
    std::map<RefOpKey, RefCommand> refOps;                    // RefCommand state ops: the last command per key (no pre-image: a rebase cannot take one back)

    // Fold one committed txn. `me` = the local slot (kNoPlayer for the host's own bookkeeping);
    // `out` (optional) receives the post-merge effects the local world must actually perform:
    // shared effects only when they change state, player effects only when `me` is a recipient.
    // The txn seq must be exactly seq+1 for callers that care; this function only records it.
    void applyTxn(const EffectTxn& txn, PlayerId me = kNoPlayer, std::vector<Effect>* out = nullptr);

    // Effects that move a world currently equal to `from` into `to` for slot `me`.
    static void diffEffects(const ShadowState& from, const ShadowState& to, PlayerId me, std::vector<Effect>& out);

    bool operator==(const ShadowState& o) const;
    bool sameOrigins(const ShadowState& o) const;
    bool operator!=(const ShadowState& o) const { return !(*this == o); }
    std::string describeDifference(const ShadowState& o) const;  // first differing section (debug aid)

private:
    void applyEffect(const Effect& e, PlayerId origin, PlayerId me, std::vector<Effect>* out);
};

void io(Archive& a, ShadowState& s);

}  // namespace fnvcoop
