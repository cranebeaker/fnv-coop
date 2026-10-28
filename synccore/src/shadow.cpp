#include "fnvcoop/shadow.h"
#include <algorithm>
#include <climits>
#include "fnvcoop/mutations.h"
#include "fnvcoop/wire.h"

namespace fnvcoop {

namespace {
template <class... Ts> struct Overloaded : Ts... { using Ts::operator()...; };
template <class... Ts> Overloaded(Ts...) -> Overloaded<Ts...>;

void bump(std::map<FormRef, int64_t>& m, const FormRef& k, int64_t d) {
    auto& v = m[k];
    v += d;
    if (v == 0) m.erase(k);
}
}  // namespace

void ShadowState::applyTxn(const EffectTxn& txn, PlayerId me, std::vector<Effect>* out) {
    for (const auto& e : txn.effects) applyEffect(e, txn.origin, me, out);
    seq = txn.seq;
}

namespace {
// Bucket-digest segments an effect can change (bit per BucketCache segment, see baseline.cpp walkSegment). Unknown kinds dirty everything.
constexpr uint64_t S(unsigned i) { return 1ull << i; }
// Namespace scope (not local to segmentsOf): the lambdas below use them, and the legacy MSVC lambda processor (cl without
// /permissive-, as the L7 snapshot suite builds it) rejects a local constexpr used in a non-capturing lambda (C3493).
constexpr uint64_t none = 0, all = ~0ull;
uint64_t segmentsOf(const Effect& eff) {
    return std::visit(Overloaded{
        [](const SetStage&) { return S(3); },
        [](const SetObjective&) { return S(4); },
        [](const SetGlobal&) { return S(7); },
        [](const EnableRef&) { return S(8); },
        [](const DisableRef&) { return S(8); },
        [](const StartQuest&) { return S(0); },
        [](const StopQuest&) { return S(0); },
        [](const MoveRef&) { return S(10); },
        [](const KillActor&) { return S(9); },
        [](const ResurrectActor&) { return S(9); },
        [](const SetScriptVar&) { return S(6); },
        [](const PartyTether&) { return S(22); },
        [](const SlideshowEnd&) { return S(26); },
        [](const SaveSuppress&) { return S(27); },
        [](const CompanionOwner&) { return S(13); },
        [](const SayOnceFlag&) { return S(25); },
        [](const CompleteQuest&) { return S(1); },
        [](const FailQuest&) { return S(1) | S(2); },
        [](const CompleteAllObjectives&) { return S(5); },
        [](const SetLock&) { return S(11); },
        [](const SetFactionRank&) { return S(23); },
        [](const SwapTexture&) { return S(12); },
        [](const SetFastTravelLock&) { return S(24); },
        [](const ObjectState&) { return S(14) | S(11); },
        [](const RefScriptVar&) { return S(15); },
        [](const AddTopic&) { return S(16); },
        [](const AddNote&) { return S(17); },
        [](const RemoveNote&) { return S(17); },
        [](const SetMapMarker&) { return S(18); },
        [](const SetWeather&) { return S(19); },
        [](const SetTimeScale&) { return S(20); },
        [](const RefAngle&) { return S(21); },
        [](const RefCommand& c) { return refOpIsState(c.op) ? S(12) : none; },   // state ops share the texture segment (baseline.cpp case 12)
        // ledger, event and spawn/inventory effects are not baseline entries
        [](const AddItem&) { return none; }, [](const RemoveItem&) { return none; }, [](const ModActorValue&) { return none; },
        [](const ModReputation&) { return none; }, [](const ModKarma&) { return none; }, [](const SetPlayerControls&) { return none; },
        [](const ForcedDialogue&) { return none; }, [](const BarrierArrive&) { return none; }, [](const BarrierRelease&) { return none; },
        [](const SetPerk&) { return none; }, [](const SpawnRef&) { return S(30) | S(31); }, [](const DespawnRef&) { return S(30) | S(31); },
        [](const RefAddItem&) { return S(28); }, [](const RefRemoveItem&) { return S(28); }, [](const RemoveAllItems&) { return S(28) | S(29); },
        [](const ForceActiveQuest&) { return none; }, [](const RequestSave&) { return none; }, [](const AdvanceTime&) { return none; },
        [](const auto&) { return all; },
    }, eff);
}

// Queue a single-key update for the big maps when their per-entry terms are valid; false => the caller dirties the whole segment.
bool noteKeyed(BucketCache& c, unsigned seg, const FormRef& ref) {
    if ((c.dirty >> seg & 1u) || !(c.termsMask >> seg & 1u) || c.pending.size() >= 512) return false;
    c.pending.emplace_back(static_cast<uint8_t>(seg), ref);
    return true;
}
bool noteKeyedEffect(BucketCache& c, const Effect& eff) {
    return std::visit(Overloaded{
        [&](const SetStage& e) { return noteKeyed(c, 3, e.quest); },
        [&](const SetGlobal& e) { return noteKeyed(c, 7, e.global); },
        [&](const EnableRef& e) { return noteKeyed(c, 8, e.ref); },
        [&](const DisableRef& e) { return noteKeyed(c, 8, e.ref); },
        [&](const MoveRef& e) { return noteKeyed(c, 10, e.ref); },
        [](const auto&) { return false; },
    }, eff);
}
}  // namespace

void ShadowState::applyEffect(const Effect& eff, PlayerId origin, PlayerId me, std::vector<Effect>* out) {
    if (!noteKeyedEffect(bucketCache, eff)) bucketCache.dirty |= segmentsOf(eff);
    auto emit = [&](const Effect& e) { if (out) out->push_back(e); };
    auto toMe = [&](const Recipient& r) {
        return me != kNoPlayer && (recipientMask(r, origin) >> me & 1u);
    };
    const Mutations& mut = mutations();

    std::visit(Overloaded{
        [&](const SetStage& e) {
            // Every stage the host runs is delivered, in the order it ran: the engine's SetStage raises the current stage only when it is higher but ALWAYS
            // marks the stage item done (GetStageDone, the journal). The max merge alone would swallow a stage below one that already came (a quest script
            // that jumps ahead, then the host walks the earlier stages): the copy then has the stage but not the done flags the host has.
            const bool firstRun = e.stage != 0 && stagesRun[e.quest].insert(e.stage).second;
            auto it = stages.find(e.quest);
            if (it == stages.end()) { stages[e.quest] = e.stage; emit(e); return; }
            uint32_t nv = mut.stageLastWins ? e.stage : std::max(it->second, e.stage);
            if (nv != it->second) { it->second = nv; emit(SetStage{e.quest, nv}); return; }
            if (firstRun && e.stage < it->second && !mut.lowerStageSwallowed) emit(e);   // a lower stage the host ran for the first time: its done flag still has to arrive
        },
        [&](const SetObjective& e) {
            // Host-authoritative last writer: a host-origin write is the exact flag state (so a hide, displayed 1 -> 0, sticks through resync and
            // late join). A guest-origin write can only add bits (a stale or local report never un-completes or un-hides what the host decided).
            auto key = std::make_pair(e.quest, e.objective);
            auto it = objectives.find(key);
            if (it == objectives.end()) { objectives[key] = e.flags; emit(e); return; }
            const bool exact = origin == kHostPlayer && !mut.objectiveOr;
            uint8_t nv = mut.flagsAnd ? static_cast<uint8_t>(it->second & e.flags)
                       : exact ? e.flags
                               : static_cast<uint8_t>(it->second | e.flags);
            if (nv != it->second) { it->second = nv; emit(SetObjective{e.quest, e.objective, nv}); }
        },
        [&](const SetGlobal& e) {
            auto it = globals.find(e.global);
            if (it == globals.end() || it->second != e.value) { globals[e.global] = e.value; emit(e); }
        },
        [&](const EnableRef& e) {
            auto it = enabled.find(e.ref);
            if (it == enabled.end() || !it->second) { enabled[e.ref] = true; emit(e); }
        },
        [&](const DisableRef& e) {
            auto it = enabled.find(e.ref);
            if (it == enabled.end() || it->second) { enabled[e.ref] = false; emit(e); }
        },
        [&](const AddItem& e) {
            uint8_t m = recipientMask(e.to, origin);
            for (unsigned s = 0; s < kMaxSlots; ++s) if (m >> s & 1u) bump(ledger[s].items, e.item, e.count);
            if (toMe(e.to)) emit(e);
        },
        [&](const RemoveItem& e) {
            uint8_t m = recipientMask(e.to, origin);
            for (unsigned s = 0; s < kMaxSlots; ++s) if (m >> s & 1u) bump(ledger[s].items, e.item, -static_cast<int64_t>(e.count));
            if (toMe(e.to)) emit(e);
        },
        [&](const ModActorValue& e) {
            uint8_t m = recipientMask(e.to, origin);
            for (unsigned s = 0; s < kMaxSlots; ++s) if (m >> s & 1u) {
                auto& v = ledger[s].av[e.av];
                v += e.delta;
                if (v == 0) ledger[s].av.erase(e.av);
            }
            if (toMe(e.to)) emit(e);
        },
        [&](const ModReputation& e) {
            uint8_t m = recipientMask(e.to, origin);
            for (unsigned s = 0; s < kMaxSlots; ++s) if (m >> s & 1u) {
                auto& v = ledger[s].rep[e.faction];
                v.first += e.fame;
                v.second += e.infamy;
                if (v.first == 0 && v.second == 0) ledger[s].rep.erase(e.faction);
            }
            if (toMe(e.to)) emit(e);
        },
        [&](const ModKarma& e) {
            uint8_t m = recipientMask(e.to, origin);
            for (unsigned s = 0; s < kMaxSlots; ++s) if (m >> s & 1u) ledger[s].karma += e.delta;
            if (toMe(e.to)) emit(e);
        },
        [&](const StartQuest& e) {
            auto it = questRunning.find(e.quest);
            if (it == questRunning.end() || !it->second) { questRunning[e.quest] = true; emit(e); }
        },
        [&](const StopQuest& e) {
            auto it = questRunning.find(e.quest);
            if (it == questRunning.end() || it->second) { questRunning[e.quest] = false; emit(e); }
        },
        [&](const MoveRef& e) {
            Placement p{e.cell, e.x, e.y, e.z};
            auto it = placed.find(e.ref);
            if (it == placed.end() || !(it->second == p) || e.partyWarp) { placed[e.ref] = p; emit(e); }
        },
        [&](const KillActor& e) {
            if (mut.killsReplace) kills.clear();
            if (kills.insert(e.actor).second) emit(e);
        },
        [&](const ResurrectActor& e) {
            if (kills.erase(e.actor)) emit(e);
        },
        [&](const SetScriptVar& e) {
            auto key = std::make_pair(e.owner, e.index);
            auto it = scriptVars.find(key);
            if (it == scriptVars.end() || it->second != e.value) { scriptVars[key] = e.value; emit(e); }
        },
        [&](const SetPlayerControls& e) {
            uint8_t m = recipientMask(e.to, origin);
            for (unsigned s = 0; s < kMaxSlots; ++s) if (m >> s & 1u) {
                uint32_t before = ledger[s].controlsOff;
                ledger[s].controlsOff = e.enable ? (before & ~e.controls) : (before | e.controls);
            }
            if (toMe(e.to)) emit(e);
        },
        [&](const PartyTether& e) {
            auto it = tethers.find(e.cell);
            if (it == tethers.end() || it->second != e.on) { tethers[e.cell] = e.on; emit(e); }
        },
        [&](const SlideshowEnd& e) { if (!slideshowEnded) { slideshowEnded = true; emit(e); } },
        [&](const SaveSuppress& e) { if (saveSuppress != e.on) { saveSuppress = e.on; emit(e); } },
        [&](const CompanionOwner& e) {
            if (!e.active) {   // dismissed
                if (companionOwner.erase(e.npc)) { companionKind.erase(e.npc); emit(e); }
                return;
            }
            auto it = companionOwner.find(e.npc);
            auto kt = companionKind.find(e.npc);
            bool changed = it == companionOwner.end() || it->second != e.owner || kt == companionKind.end() || kt->second != e.kind;
            if (changed) { companionOwner[e.npc] = e.owner; companionKind[e.npc] = e.kind; emit(e); }
        },
        [&](const ForcedDialogue& e) { emit(e); },  // event: no state
        [&](const BarrierArrive& e) {
            BarrierState& b = barriers[e.barrier];
            BarrierState before = b;
            b.required = static_cast<uint8_t>(b.required | e.required);
            if (origin < 8) b.arrived = static_cast<uint8_t>(b.arrived | (1u << origin));
            if (!(b == before)) emit(e);
            if (!b.released && b.required != 0 && (b.arrived & b.required) == b.required) {
                b.released = true;
                emit(BarrierRelease{e.barrier});
            }
        },
        [&](const BarrierRelease& e) {
            BarrierState& b = barriers[e.barrier];
            if (!b.released) { b.released = true; emit(e); }
        },
        [&](const SayOnceFlag& e) { if (sayOnce.insert(e.info).second) emit(e); },
        [&](const CompleteQuest& e) { if (questCompleted.insert(e.quest).second) emit(e); },
        [&](const FailQuest& e) {
            bool a = questFailed.insert(e.quest).second;
            bool b = mutations().failNotCompleted ? false : questCompleted.insert(e.quest).second;
            if (a || b) emit(e);
        },
        [&](const CompleteAllObjectives& e) { if (allObjectivesDone.insert(e.quest).second) emit(e); },
        [&](const SetLock& e) {
            if (foldLock(locks, origins, e.ref, e.locked, e.level, e.hasBefore, e.beforeLocked, e.beforeLevel)) emit(e);
        },
        [&](const SetFactionRank& e) {
            auto key = std::make_pair(e.actor, e.faction);
            int32_t r = e.rank < 0 ? -1 : e.rank;
            auto it = factionRank.find(key);
            if (it == factionRank.end() || it->second != r) { factionRank[key] = r; emit(e); }
        },
        [&](const SetPerk& e) {
            uint8_t m = recipientMask(e.to, origin);
            bool changedMe = false;
            for (unsigned s = 0; s < kMaxSlots; ++s) if (m >> s & 1u) {
                auto& pm = ledger[s].perks;
                auto it = pm.find(e.perk);
                int32_t cur = it == pm.end() ? 0 : it->second;
                int32_t nv = e.rank < 0 ? 0 : e.rank;
                if (nv == 0) pm.erase(e.perk); else pm[e.perk] = nv;
                if (s == me && cur != nv) changedMe = true;
            }
            if (changedMe) emit(e);
        },
        [&](const SetFastTravelLock& e) {
            // only locked scopes are stored (absent == unlocked), so the state stays canonical
            bool cur = fastTravelLock.count(e.scope) > 0;
            if (cur != e.on) {
                if (e.on) fastTravelLock[e.scope] = true; else fastTravelLock.erase(e.scope);
                emit(e);
            }
        },
        [&](const SpawnRef& e) {
            if (despawned.count(e.ref()) && !mut.spawnResurrects) return;
            const uint32_t id = e.id & 0x00FFFFFFu;
            if (id > createdCounter) createdCounter = id;
            SpawnInfo n{e.base, e.cell, e.x, e.y, e.z, e.leveled};
            auto it = spawned.find(e.ref());
            if (it == spawned.end() || !(it->second == n)) { spawned[e.ref()] = n; emit(e); }
        },
        [&](const DespawnRef& e) {
            if (e.forget) {
                bool had = spawned.erase(e.ref()) > 0;
                bool tomb = despawned.erase(e.ref()) > 0;
                if (had || tomb) emit(e);
                return;
            }
            bool had = spawned.erase(e.ref()) > 0;
            const uint32_t id = e.id & 0x00FFFFFFu;
            if (id > createdCounter) createdCounter = id;
            bool fresh = despawned.insert(e.ref()).second;
            if (had || fresh) emit(e);
        },
        [&](const ObjectState& e) {
            ObjState o;
            auto it = objects.find(e.ref);
            if (it != objects.end()) o = it->second;
            const ObjState before = o;
            bool changed = false;
            if (e.mask & kObjOpen) o.open = e.open;
            if (e.mask & kObjDestroyed) o.destroyed = mut.stickyObjectsReset ? e.destroyed : (o.destroyed || e.destroyed);       // sticky
            if (e.mask & kObjActivated) o.activated = mut.stickyObjectsReset ? e.activatedOnce : (o.activated || e.activatedOnce);   // sticky
            if (!(o == before)) changed = true;
            if (e.mask & kObjLock)
                if (foldLock(locks, origins, e.ref, e.locked, e.level, e.hasBefore, e.beforeLocked, e.beforeLevel)) changed = true;
            if (o.isDefault()) objects.erase(e.ref); else objects[e.ref] = o;
            if (changed) emit(e);
        },
        [&](const RefScriptVar& e) {
            auto key = std::make_pair(e.ref, e.index);
            if (foldRefVar(refVars, origins, e)) emit(e);
        },
        [&](const RefAddItem& e) {
            if (e.count == 0) return;
            RefInv& inv = refInv[e.ref];
            bump(inv.delta, e.item, e.count);
            if (inv.delta.empty() && !inv.cleared) refInv.erase(e.ref);
            emit(e);
        },
        [&](const RefRemoveItem& e) {
            if (e.count == 0) return;
            RefInv& inv = refInv[e.ref];
            bump(inv.delta, e.item, -static_cast<int64_t>(e.count));
            if (inv.delta.empty() && !inv.cleared) refInv.erase(e.ref);
            emit(e);
        },
        [&](const RemoveAllItems& e) { if (foldRefClear(refInv, e, mut.refInvNoClear)) emit(e); },
        [&](const AddTopic& e) { if (topics.insert(e.topic).second) emit(e); },
        [&](const AddNote& e) { if (notes.insert(e.note).second) emit(e); },
        [&](const RemoveNote& e) { if (notes.erase(e.note)) emit(e); },
        [&](const SetMapMarker& e) {
            if (foldMarker(mapMarkers, origins, e)) emit(e);
        },
        [&](const ForceActiveQuest& e) {
            uint8_t m = recipientMask(e.to, origin);
            bool changedMe = false;
            for (unsigned s = 0; s < kMaxSlots; ++s) if (m >> s & 1u) {
                Ledger& l = ledger[s];
                bool ch = !l.hasActive || !(l.activeQuest == e.quest);
                l.hasActive = true; l.activeQuest = e.quest;
                if (s == me && ch) changedMe = true;
            }
            if (changedMe) emit(e);
        },
        [&](const RequestSave& e) { emit(e); },   // event: no state
        [&](const SetWeather& e) {
            if (foldWeather(weather, weatherSet, weatherForced, origins, e)) emit(e);
        },
        [&](const SetTimeScale& e) { if (foldTimeScale(timeScale, origins, e)) emit(e); },
        [&](const AdvanceTime& e) { emit(e); },   // event: no state
        [&](const RefAngle& e) {
            if (mut.angleKeepsFirst && angles.count({e.ref, e.axis})) return;
            if (foldAngle(angles, angleOrig, e)) emit(e);
        },
        [&](const RefCommand& e) {
            if (!refOpKnown(e.op)) return;
            if (!refOpIsState(e.op) && !mut.refCmdAllState) { if (toMe(e.to)) emit(e); return; }   // event: no state, only its recipients perform it
            const RefOpKey key = refOpKeyOf(e);
            auto it = refOps.find(key);
            if (it != refOps.end() && (mut.refCmdKeepsFirst || it->second == e)) return;   // same command again (or, under the twin, a later one is ignored)
            refOps[key] = e;
            if (toMe(e.to)) emit(e);
        },
        [&](const SwapTexture& e) {
            auto key = std::make_pair(e.ref, e.slot);
            auto it = textures.find(key);
            if (it == textures.end() || !(it->second == e.texture)) { textures[key] = e.texture; emit(e); }
        },
    }, eff);
}

static const Origin* orgOf(const ShadowState& w, uint8_t kind, const FormRef& r, uint16_t idx) {
    auto it = w.origins.find(OriginKey{kind, r, idx});
    return it == w.origins.end() ? nullptr : &it->second;
}

void ShadowState::diffEffects(const ShadowState& from, const ShadowState& to, PlayerId me, std::vector<Effect>& out) {
    for (auto& [q, st] : to.stages) {
        auto it = from.stages.find(q);
        if (it == from.stages.end() || it->second != st) out.push_back(SetStage{q, st});
    }
    for (auto& [k, fl] : to.objectives) {
        auto it = from.objectives.find(k);
        uint8_t have = it == from.objectives.end() ? 0 : it->second;
        if (it == from.objectives.end() || have != fl) {
            out.push_back(SetObjective{k.first, k.second, fl});   // exact state (host-authoritative), not the added bits
        }
    }
    for (auto& [g, v] : to.globals) {
        auto it = from.globals.find(g);
        if (it == from.globals.end() || it->second != v) out.push_back(SetGlobal{g, v});
    }
    for (auto& [r, on] : to.enabled) {
        auto it = from.enabled.find(r);
        if (it == from.enabled.end() || it->second != on) {
            if (on) out.push_back(EnableRef{r}); else out.push_back(DisableRef{r});
        }
    }
    for (auto& k : to.kills) if (!from.kills.count(k)) out.push_back(KillActor{k});
    for (auto& k : from.kills) if (!to.kills.count(k)) out.push_back(ResurrectActor{k});
    for (auto& [q, on] : to.questRunning) {
        auto it = from.questRunning.find(q);
        if (it == from.questRunning.end() || it->second != on) {
            if (on) out.push_back(StartQuest{q}); else out.push_back(StopQuest{q});
        }
    }
    for (auto& [r, p] : to.placed) {
        auto it = from.placed.find(r);
        if (it == from.placed.end() || !(it->second == p)) out.push_back(MoveRef{r, p.cell, p.x, p.y, p.z});
    }
    for (auto& [k, v] : to.scriptVars) {
        auto it = from.scriptVars.find(k);
        if (it == from.scriptVars.end() || it->second != v) out.push_back(SetScriptVar{k.first, k.second, v});
    }
    for (auto& [c, on] : to.tethers) {
        auto it = from.tethers.find(c);
        if (it == from.tethers.end() || it->second != on) out.push_back(PartyTether{c, on});
    }
    if (to.slideshowEnded && !from.slideshowEnded) out.push_back(SlideshowEnd{});
    if (to.saveSuppress != from.saveSuppress) out.push_back(SaveSuppress{to.saveSuppress});
    for (auto& [n, o] : to.companionOwner) {
        auto it = from.companionOwner.find(n);
        auto kt = to.companionKind.find(n);
        uint8_t kind = kt == to.companionKind.end() ? 0 : kt->second;
        auto fk = from.companionKind.find(n);
        uint8_t fromKind = fk == from.companionKind.end() ? 0 : fk->second;
        if (it == from.companionOwner.end() || it->second != o || fromKind != kind) out.push_back(CompanionOwner{n, o, true, kind});
    }
    for (auto& [n, o] : from.companionOwner) if (!to.companionOwner.count(n)) out.push_back(CompanionOwner{n, o, false, 0});
    // round 3
    if (!preimageIgnored(kPiSpawn)) {   // spawned / despawned in `from` but not in `to`: take the spawn back, drop the tombstone (before any respawn below)
        for (auto& [r, info] : from.spawned) if (!to.spawned.count(r) && !to.despawned.count(r)) { DespawnRef f{r.localId}; f.forget = true; out.push_back(f); }
        for (auto& r : from.despawned) if (!to.despawned.count(r)) { DespawnRef f{r.localId}; f.forget = true; out.push_back(f); }
    }
    for (auto& [r, info] : to.spawned) {
        auto it = from.spawned.find(r);
        if (it == from.spawned.end() || !(it->second == info)) out.push_back(SpawnRef{r.localId, info.base, info.cell, info.x, info.y, info.z, info.leveled});
    }
    for (auto& r : to.despawned) if (!from.despawned.count(r)) out.push_back(DespawnRef{r.localId});
    {
        std::set<FormRef> refs;
        for (auto& [r, o] : to.objects) refs.insert(r);
        for (auto& [r, o] : from.objects) refs.insert(r);
        for (auto& r : refs) {
            ObjState a, b;
            if (auto it = from.objects.find(r); it != from.objects.end()) a = it->second;
            if (auto it = to.objects.find(r); it != to.objects.end()) b = it->second;
            ObjectState e; e.ref = r;
            if (a.open != b.open && !(a.open && !b.open && preimageIgnored(kPiObjectState))) { e.mask |= kObjOpen; e.open = b.open; }   // a.open && !b.open is the revert
            if (b.destroyed && !a.destroyed) { e.mask |= kObjDestroyed; e.destroyed = true; }
            if (b.activated && !a.activated) { e.mask |= kObjActivated; e.activatedOnce = true; }
            if (e.mask) out.push_back(e);
        }
    }
    for (auto& [k, v] : to.refVars) {
        auto it = from.refVars.find(k);
        if (it == from.refVars.end() || it->second != v) {
            RefScriptVar x{k.first, k.second, v};
            if (const Origin* o = orgOf(to, kOrgRefVar, k.first, k.second)) { x.hasBefore = true; x.before = o->d; }
            out.push_back(x);
        }
    }
    if (!preimageIgnored(kPiRefVar))
        for (auto& [k, v] : from.refVars)
            if (!to.refVars.count(k))
                if (const Origin* o = orgOf(from, kOrgRefVar, k.first, k.second)) out.push_back(RefScriptVar{k.first, k.second, o->d, true, o->d});
    {
        std::set<FormRef> refs;
        for (auto& [r, i] : to.refInv) refs.insert(r);
        for (auto& [r, i] : from.refInv) refs.insert(r);
        for (auto& r : refs) {
            RefInv a, b;
            if (auto it = from.refInv.find(r); it != from.refInv.end()) a = it->second;
            if (auto it = to.refInv.find(r); it != to.refInv.end()) b = it->second;
            if (b.cleared && !a.cleared) {
                RemoveAllItems c{r};
                if (b.hasPre && !preimageIgnored(kPiClearPre)) { c.hasBefore = true; for (auto& [f, n] : b.pre) c.items.emplace_back(f, static_cast<int32_t>(n)); }   // carry the pre-image so the receiver can undo the clear after a host load
                out.push_back(c);
                a.delta.clear();
            }
            else if (a.cleared && !b.cleared && a.hasPre && !preimageIgnored(kPiRefInv)) {   // cleared in `from`, not in `to`: put the contents back
                RemoveAllItems u; u.ref = r; u.undo = true;
                for (auto& [f, c] : a.pre) u.items.emplace_back(f, static_cast<int32_t>(c));
                out.push_back(u);
                a.cleared = false;
            }
            std::set<FormRef> items;
            for (auto& [i, d] : a.delta) items.insert(i);
            for (auto& [i, d] : b.delta) items.insert(i);
            for (auto& i : items) {
                int64_t x = a.delta.count(i) ? a.delta.at(i) : 0, y = b.delta.count(i) ? b.delta.at(i) : 0;
                int64_t d = y - x;
                if (y == 0 && preimageIgnored(kPiRefInv)) continue;   // twin: a delta only `from` holds is not taken back
                while (d != 0) {
                    int32_t c = static_cast<int32_t>(std::clamp<int64_t>(d, INT32_MIN + 1, INT32_MAX));
                    if (c > 0) out.push_back(RefAddItem{r, i, c}); else out.push_back(RefRemoveItem{r, i, -c});
                    d -= c;
                }
            }
        }
    }
    for (auto& t : to.topics) if (!from.topics.count(t)) out.push_back(AddTopic{t});
    for (auto& t : to.notes) if (!from.notes.count(t)) out.push_back(AddNote{t});
    for (auto& t : from.notes) if (!to.notes.count(t)) out.push_back(RemoveNote{t});
    for (auto& [m, f] : to.mapMarkers) {
        auto it = from.mapMarkers.find(m);
        if (it == from.mapMarkers.end() || it->second != f) {
            SetMapMarker x{m, f};
            if (const Origin* o = orgOf(to, kOrgMarker, m, 0)) { x.hasBefore = true; x.before = static_cast<uint8_t>(o->n); }
            out.push_back(x);
        }
    }
    if (!preimageIgnored(kPiMarker))
        for (auto& [m, f] : from.mapMarkers)
            if (!to.mapMarkers.count(m))
                if (const Origin* o = orgOf(from, kOrgMarker, m, 0)) out.push_back(SetMapMarker{m, static_cast<uint8_t>(o->n), true, static_cast<uint8_t>(o->n)});
    if (to.weatherSet && (!from.weatherSet || !(from.weather == to.weather) || from.weatherForced != to.weatherForced)) {
        SetWeather x{to.weather, to.weatherForced};
        if (const Origin* o = orgOf(to, kOrgWeather, FormRef(), 0)) { x.hasBefore = true; x.before = o->f; x.beforeForce = o->b; }
        out.push_back(x);
    } else if (!to.weatherSet && from.weatherSet && !preimageIgnored(kPiWeather)) {
        if (const Origin* o = orgOf(from, kOrgWeather, FormRef(), 0)) out.push_back(SetWeather{o->f, o->b, true, o->f, o->b});
    }
    if (to.timeScale != 0 && to.timeScale != from.timeScale) {
        SetTimeScale x{to.timeScale};
        if (const Origin* o = orgOf(to, kOrgTimeScale, FormRef(), 0)) { x.hasBefore = true; x.before = static_cast<float>(o->d); }
        out.push_back(x);
    } else if (to.timeScale == 0 && from.timeScale != 0 && !preimageIgnored(kPiTimeScale)) {
        if (const Origin* o = orgOf(from, kOrgTimeScale, FormRef(), 0)) out.push_back(SetTimeScale{static_cast<float>(o->d), true, static_cast<float>(o->d)});
    }
    for (auto& [k, deg] : to.angles) {
        auto it = from.angles.find(k);
        if (it == from.angles.end() || it->second != deg) {
            auto o = to.angleOrig.find(k);
            out.push_back(RefAngle{k.first, k.second, deg, o != to.angleOrig.end(), o != to.angleOrig.end() ? o->second : 0.f});
        }
    }
    if (!mutations().anglePreimageIgnored)
        for (auto& [k, deg] : from.angles) {   // rotated in `from` but not in `to`: restore the original
            auto o = from.angleOrig.find(k);
            if (!to.angles.count(k) && o != from.angleOrig.end()) out.push_back(RefAngle{k.first, k.second, o->second, true, o->second});
        }
    for (auto& [b, st] : to.barriers) {
        auto it = from.barriers.find(b);
        if (st.released && (it == from.barriers.end() || !it->second.released)) out.push_back(BarrierRelease{b});
    }
    for (auto& k : to.sayOnce) if (!from.sayOnce.count(k)) out.push_back(SayOnceFlag{k});
    for (auto& k : to.questFailed) if (!from.questFailed.count(k)) out.push_back(FailQuest{k});
    for (auto& k : to.questCompleted) if (!from.questCompleted.count(k) && !to.questFailed.count(k)) out.push_back(CompleteQuest{k});
    for (auto& k : to.allObjectivesDone) if (!from.allObjectivesDone.count(k)) out.push_back(CompleteAllObjectives{k});
    for (auto& [r, v] : to.locks) {
        auto it = from.locks.find(r);
        if (it == from.locks.end() || it->second != v) {
            SetLock l{r, v.first, v.second};
            if (const Origin* o = orgOf(to, kOrgLock, r, 0)) { l.hasBefore = true; l.beforeLocked = o->b; l.beforeLevel = o->n; }
            out.push_back(l);
        }
    }
    if (!preimageIgnored(kPiLock))
        for (auto& [r, v] : from.locks)   // locked / unlocked in `from` but never touched in `to`: restore the original
            if (!to.locks.count(r))
                if (const Origin* o = orgOf(from, kOrgLock, r, 0)) out.push_back(SetLock{r, o->b, o->n, true, o->b, o->n});
    for (auto& [k, rk] : to.factionRank) {
        auto it = from.factionRank.find(k);
        if (it == from.factionRank.end() || it->second != rk) out.push_back(SetFactionRank{k.first, k.second, rk});
    }
    for (auto& [k, t] : to.textures) {
        auto it = from.textures.find(k);
        if (it == from.textures.end() || !(it->second == t)) out.push_back(SwapTexture{k.first, t, k.second});
    }
    for (auto& [k, c] : to.refOps) {   // state commands the target has and `from` lacks or holds a different last value of (nothing takes one back: no pre-image)
        auto it = from.refOps.find(k);
        if ((it == from.refOps.end() || !(it->second == c)) && (me == kNoPlayer || c.to.mode != Recipient::Mask || c.to.includes(me))) out.push_back(c);
    }
    for (auto& [k, on] : to.fastTravelLock) {
        auto it = from.fastTravelLock.find(k);
        bool cur = it != from.fastTravelLock.end() && it->second;
        if (cur != on) out.push_back(SetFastTravelLock{k, on});
    }
    for (auto& [k, on] : from.fastTravelLock)
        if (on && !to.fastTravelLock.count(k)) out.push_back(SetFastTravelLock{k, false});
    if (me >= kMaxSlots) return;
    {
        const Ledger& fa = from.ledger[me];
        const Ledger& fb = to.ledger[me];
        if (fb.hasActive && (!fa.hasActive || !(fa.activeQuest == fb.activeQuest))) out.push_back(ForceActiveQuest{fb.activeQuest, Recipient::slot(me)});
        std::set<FormRef> pk;
        for (auto& [k, v] : fa.perks) pk.insert(k);
        for (auto& [k, v] : fb.perks) pk.insert(k);
        for (auto& k : pk) {
            auto ia = fa.perks.find(k), ib = fb.perks.find(k);
            int32_t x = ia == fa.perks.end() ? 0 : ia->second, y = ib == fb.perks.end() ? 0 : ib->second;
            if (x != y) out.push_back(SetPerk{k, y, Recipient::slot(me)});
        }
        if (fa.controlsOff != fb.controlsOff) {
            uint32_t turnOn = fa.controlsOff & ~fb.controlsOff, turnOff = fb.controlsOff & ~fa.controlsOff;
            if (turnOn) out.push_back(SetPlayerControls{turnOn, true, Recipient::slot(me)});
            if (turnOff) out.push_back(SetPlayerControls{turnOff, false, Recipient::slot(me)});
        }
    }
    const Ledger& a = from.ledger[me];
    const Ledger& b = to.ledger[me];
    auto chunks = [&](int64_t d, auto&& fn) {
        while (d != 0) {
            int32_t c = static_cast<int32_t>(std::clamp<int64_t>(d, INT32_MIN + 1, INT32_MAX));
            fn(c);
            d -= c;
        }
    };
    const Recipient self = Recipient::slot(me);
    std::set<FormRef> itemKeys;
    for (auto& [k, v] : a.items) itemKeys.insert(k);
    for (auto& [k, v] : b.items) itemKeys.insert(k);
    for (auto& k : itemKeys) {
        auto ia = a.items.find(k), ib = b.items.find(k);
        int64_t d = (ib == b.items.end() ? 0 : ib->second) - (ia == a.items.end() ? 0 : ia->second);
        chunks(d, [&](int32_t c) { out.push_back(AddItem{k, c, self}); });
    }
    std::set<uint32_t> avKeys;
    for (auto& [k, v] : a.av) avKeys.insert(k);
    for (auto& [k, v] : b.av) avKeys.insert(k);
    for (auto k : avKeys) {
        auto ia = a.av.find(k), ib = b.av.find(k);
        int64_t d = (ib == b.av.end() ? 0 : ib->second) - (ia == a.av.end() ? 0 : ia->second);
        chunks(d, [&](int32_t c) { out.push_back(ModActorValue{k, c, self}); });
    }
    chunks(b.karma - a.karma, [&](int32_t c) { out.push_back(ModKarma{c, self}); });
    std::set<FormRef> facs;
    for (auto& [k, v] : a.rep) facs.insert(k);
    for (auto& [k, v] : b.rep) facs.insert(k);
    for (auto& k : facs) {
        auto ia = a.rep.find(k), ib = b.rep.find(k);
        int64_t f = (ib == b.rep.end() ? 0 : ib->second.first) - (ia == a.rep.end() ? 0 : ia->second.first);
        int64_t i = (ib == b.rep.end() ? 0 : ib->second.second) - (ia == a.rep.end() ? 0 : ia->second.second);
        while (f != 0 || i != 0) {
            int32_t cf = static_cast<int32_t>(std::clamp<int64_t>(f, INT32_MIN + 1, INT32_MAX));
            int32_t ci = static_cast<int32_t>(std::clamp<int64_t>(i, INT32_MIN + 1, INT32_MAX));
            out.push_back(ModReputation{k, cf, ci, self});
            f -= cf; i -= ci;
        }
    }
}

// Origins are metadata (what a revert needs); entries created from a dump carry none, so they are not part of state equality.
bool ShadowState::sameOrigins(const ShadowState& o) const { return origins == o.origins; }

bool ShadowState::operator==(const ShadowState& o) const {
    return seq == o.seq && stages == o.stages && objectives == o.objectives && globals == o.globals &&
           enabled == o.enabled && kills == o.kills && questRunning == o.questRunning && placed == o.placed &&
           ledger == o.ledger && scriptVars == o.scriptVars && tethers == o.tethers && slideshowEnded == o.slideshowEnded &&
           saveSuppress == o.saveSuppress && companionOwner == o.companionOwner && barriers == o.barriers &&
           sayOnce == o.sayOnce && questCompleted == o.questCompleted && allObjectivesDone == o.allObjectivesDone && questFailed == o.questFailed &&
           locks == o.locks && factionRank == o.factionRank && textures == o.textures && fastTravelLock == o.fastTravelLock &&
           spawned == o.spawned && despawned == o.despawned && createdCounter == o.createdCounter && objects == o.objects &&
           refVars == o.refVars && refInv == o.refInv && topics == o.topics && notes == o.notes && mapMarkers == o.mapMarkers &&
           companionKind == o.companionKind && weatherSet == o.weatherSet && (!weatherSet || (weather == o.weather && weatherForced == o.weatherForced)) &&
           timeScale == o.timeScale && angles == o.angles && angleOrig == o.angleOrig && refOps == o.refOps;
}

std::string ShadowState::describeDifference(const ShadowState& o) const {
    if (seq != o.seq) return "seq " + std::to_string(seq) + " vs " + std::to_string(o.seq);
    if (stages != o.stages) return "stages";
    if (objectives != o.objectives) return "objectives";
    if (globals != o.globals) return "globals";
    if (enabled != o.enabled) return "enabled";
    if (kills != o.kills) return "kills";
    if (questRunning != o.questRunning) return "questRunning";
    if (placed != o.placed) return "placed";
    for (unsigned s = 0; s < kMaxSlots; ++s)
        if (!(ledger[s] == o.ledger[s])) return "ledger[" + std::to_string(s) + "]";
    if (scriptVars != o.scriptVars) return "scriptVars";
    if (barriers != o.barriers) return "barriers";
    if (spawned != o.spawned || despawned != o.despawned || createdCounter != o.createdCounter) return "spawned";
    if (objects != o.objects) return "objects";
    if (refVars != o.refVars) return "refVars";
    if (refInv != o.refInv) return "refInv";
    if (topics != o.topics || notes != o.notes || mapMarkers != o.mapMarkers) return "journal";
    if (weatherSet != o.weatherSet || timeScale != o.timeScale) return "weather/time";
    if (angles != o.angles || angleOrig != o.angleOrig) return "angles";
    if (refOps != o.refOps) return "refOps";
    return "round2-state";
}

// ---- wire ---------------------------------------------------------------------------------------

namespace {
constexpr size_t kMaxEntries = 1u << 20;

void ioLedger(Archive& a, Ledger& l) {
    // items
    {
        size_t n = l.items.size();
        if (a.count(n, kMaxEntries)) {
            if (a.reading()) {
                l.items.clear();
                for (size_t i = 0; i < n && a.ok(); ++i) { FormRef k; int64_t v = 0; io(a, k); a.vari(v); if (a.ok()) l.items[k] = v; }
            } else for (auto& [k, v] : l.items) { FormRef kk = k; int64_t vv = v; io(a, kk); a.vari(vv); }
        }
    }
    {
        size_t n = l.av.size();
        if (a.count(n, kMaxEntries)) {
            if (a.reading()) {
                l.av.clear();
                for (size_t i = 0; i < n && a.ok(); ++i) { uint32_t k = 0; int64_t v = 0; a.varu32(k); a.vari(v); if (a.ok()) l.av[k] = v; }
            } else for (auto& [k, v] : l.av) { uint32_t kk = k; int64_t vv = v; a.varu32(kk); a.vari(vv); }
        }
    }
    a.vari(l.karma);
    a.varu32(l.controlsOff);
    a.boolean(l.hasActive);
    if (l.hasActive) io(a, l.activeQuest);
    {
        size_t n = l.perks.size();
        if (a.count(n, kMaxEntries)) {
            if (a.reading()) {
                l.perks.clear();
                for (size_t i = 0; i < n && a.ok(); ++i) { FormRef k; int32_t v = 0; io(a, k); a.vari32(v); if (a.ok()) l.perks[k] = v; }
            } else for (auto& [k, v] : l.perks) { FormRef kk = k; int32_t vv = v; io(a, kk); a.vari32(vv); }
        }
    }
    {
        size_t n = l.rep.size();
        if (a.count(n, kMaxEntries)) {
            if (a.reading()) {
                l.rep.clear();
                for (size_t i = 0; i < n && a.ok(); ++i) { FormRef k; int64_t f = 0, inf = 0; io(a, k); a.vari(f); a.vari(inf); if (a.ok()) l.rep[k] = {f, inf}; }
            } else for (auto& [k, v] : l.rep) { FormRef kk = k; int64_t f = v.first, inf = v.second; io(a, kk); a.vari(f); a.vari(inf); }
        }
    }
}
}  // namespace

namespace {
template <class M, class KF, class VF>
void ioKV(Archive& a, M& m, KF kf, VF vf) {
    size_t n = m.size();
    if (!a.count(n, kMaxEntries)) { m.clear(); return; }
    if (a.reading()) {
        m.clear();
        for (size_t i = 0; i < n && a.ok(); ++i) {
            typename M::key_type k{};
            typename M::mapped_type v{};
            kf(k); vf(v);
            if (a.ok()) m[k] = v;
        }
    } else {
        for (auto& [k, v] : m) { auto kk = k; auto vv = v; kf(kk); vf(vv); }
    }
}
template <class S>
void ioSet(Archive& a, S& s) {
    size_t n = s.size();
    if (!a.count(n, kMaxEntries)) { s.clear(); return; }
    if (a.reading()) {
        s.clear();
        for (size_t i = 0; i < n && a.ok(); ++i) { FormRef k; io(a, k); if (a.ok()) s.insert(k); }
    } else {
        for (auto& k : s) { FormRef kk = k; io(a, kk); }
    }
}
void ioRound2(Archive& a, ShadowState& s) {
    ioKV(a, s.scriptVars,
         [&](std::pair<FormRef, uint16_t>& k) {
             io(a, k.first); uint32_t i = k.second; a.varu32(i);
             if (a.reading()) { if (i > 0xFFFF) a.fail(); else k.second = static_cast<uint16_t>(i); }
         },
         [&](double& v) { a.f64(v); });
    ioKV(a, s.tethers, [&](FormRef& k) { io(a, k); }, [&](bool& v) { a.boolean(v); });
    a.boolean(s.slideshowEnded);
    a.boolean(s.saveSuppress);
    ioKV(a, s.companionOwner, [&](FormRef& k) { io(a, k); }, [&](uint8_t& v) { a.u8(v); if (a.reading() && v >= kMaxSlots) a.fail(); });
    ioKV(a, s.barriers, [&](FormRef& k) { io(a, k); },
         [&](BarrierState& v) { a.u8(v.arrived); a.u8(v.required); a.boolean(v.released); });
    ioSet(a, s.sayOnce);
    ioSet(a, s.questCompleted);
    ioSet(a, s.allObjectivesDone);
    ioKV(a, s.locks, [&](FormRef& k) { io(a, k); }, [&](std::pair<bool, int32_t>& v) { a.boolean(v.first); a.vari32(v.second); });
    ioKV(a, s.factionRank, [&](std::pair<FormRef, FormRef>& k) { io(a, k.first); io(a, k.second); }, [&](int32_t& v) { a.vari32(v); });
    ioKV(a, s.fastTravelLock, [&](FormRef& k) { io(a, k); }, [&](bool& v) { a.boolean(v); });
    ioKV(a, s.textures, [&](std::pair<FormRef, uint8_t>& k) { io(a, k.first); a.u8(k.second); }, [&](FormRef& v) { io(a, v); });
    ioSet(a, s.questFailed);
    ioKV(a, s.spawned, [&](FormRef& k) { io(a, k); },
         [&](SpawnInfo& v) { io(a, v.base); io(a, v.cell); a.f32(v.x); a.f32(v.y); a.f32(v.z); a.boolean(v.leveled); });
    ioSet(a, s.despawned);
    a.varu32(s.createdCounter);
    ioKV(a, s.objects, [&](FormRef& k) { io(a, k); }, [&](ObjState& v) { a.boolean(v.open); a.boolean(v.destroyed); a.boolean(v.activated); });
    ioKV(a, s.refVars,
         [&](std::pair<FormRef, uint16_t>& k) {
             io(a, k.first);
             uint32_t i = k.second;
             a.varu32(i);
             if (a.reading()) { if (i > 0xFFFF) a.fail(); else k.second = static_cast<uint16_t>(i); }
         },
         [&](double& v) { a.f64(v); });
    ioKV(a, s.refInv, [&](FormRef& k) { io(a, k); },
         [&](RefInv& v) {
             a.boolean(v.cleared);
             ioKV(a, v.delta, [&](FormRef& k) { io(a, k); }, [&](int64_t& d) { a.vari(d); });
             a.boolean(v.hasPre);
             ioKV(a, v.pre, [&](FormRef& k) { io(a, k); }, [&](int64_t& d) { a.vari(d); });
         });
    ioSet(a, s.topics);
    ioSet(a, s.notes);
    ioKV(a, s.mapMarkers, [&](FormRef& k) { io(a, k); }, [&](uint8_t& v) { a.u8(v); });
    ioKV(a, s.companionKind, [&](FormRef& k) { io(a, k); }, [&](uint8_t& v) { a.u8(v); if (a.reading() && v > 1) a.fail(); });
    a.boolean(s.weatherSet);
    a.boolean(s.weatherForced);
    io(a, s.weather);
    a.f32(s.timeScale);
    auto angleKey = [&](std::pair<FormRef, uint8_t>& k) { io(a, k.first); a.u8(k.second); if (a.reading() && k.second > kAxisZ) a.fail(); };
    ioKV(a, s.angles, angleKey, [&](float& v) { a.f32(v); });
    ioKV(a, s.angleOrig, angleKey, [&](float& v) { a.f32(v); });
    ioKV(a, s.origins,
         [&](OriginKey& k) {
             uint8_t kind = std::get<0>(k);
             uint32_t idx = std::get<2>(k);
             a.u8(kind);
             io(a, std::get<1>(k));
             a.varu32(idx);
             if (a.reading()) { if (kind < kOrgWeather || kind > kOrgLock || idx > 0xFFFF) a.fail(); std::get<0>(k) = kind; std::get<2>(k) = static_cast<uint16_t>(idx); }
         },
         [&](Origin& o) { io(a, o.f); a.f64(o.d); a.vari32(o.n); a.boolean(o.b); });
    {   // RefCommand state ops. The key is derived from the command, so only the commands travel.
        size_t n = s.refOps.size();
        if (a.count(n, kMaxEntries)) {
            if (a.reading()) {
                s.refOps.clear();
                for (size_t i = 0; i < n && a.ok(); ++i) {
                    Effect e;
                    io(a, e);
                    auto* c = a.ok() ? std::get_if<RefCommand>(&e) : nullptr;
                    if (!c || !refOpIsState(c->op)) { a.fail(); break; }
                    s.refOps[refOpKeyOf(*c)] = *c;
                }
            } else {
                for (auto& [k, c] : s.refOps) { Effect e = c; io(a, e); }
            }
        }
    }
}
}  // namespace

void io(Archive& a, ShadowState& s) {
    a.varu(s.seq);
    auto rd = a.reading();
    if (rd) s.bucketCache.dirty = ~0ull;
    {
        size_t n = s.stages.size();
        if (a.count(n, kMaxEntries)) {
            if (rd) { s.stages.clear(); for (size_t i = 0; i < n && a.ok(); ++i) { FormRef k; uint32_t v = 0; io(a, k); a.varu32(v); if (a.ok()) s.stages[k] = v; } }
            else for (auto& [k, v] : s.stages) { FormRef kk = k; uint32_t vv = v; io(a, kk); a.varu32(vv); }
        }
    }
    {
        size_t n = s.objectives.size();
        if (a.count(n, kMaxEntries)) {
            if (rd) { s.objectives.clear(); for (size_t i = 0; i < n && a.ok(); ++i) { FormRef k; uint32_t o = 0; uint8_t f = 0; io(a, k); a.varu32(o); a.u8(f); if (a.ok()) s.objectives[{k, o}] = f; } }
            else for (auto& [k, v] : s.objectives) { FormRef kk = k.first; uint32_t o = k.second; uint8_t f = v; io(a, kk); a.varu32(o); a.u8(f); }
        }
    }
    {
        size_t n = s.globals.size();
        if (a.count(n, kMaxEntries)) {
            if (rd) { s.globals.clear(); for (size_t i = 0; i < n && a.ok(); ++i) { FormRef k; float v = 0; io(a, k); a.f32(v); if (a.ok()) s.globals[k] = v; } }
            else for (auto& [k, v] : s.globals) { FormRef kk = k; float vv = v; io(a, kk); a.f32(vv); }
        }
    }
    {
        size_t n = s.enabled.size();
        if (a.count(n, kMaxEntries)) {
            if (rd) { s.enabled.clear(); for (size_t i = 0; i < n && a.ok(); ++i) { FormRef k; bool v = false; io(a, k); a.boolean(v); if (a.ok()) s.enabled[k] = v; } }
            else for (auto& [k, v] : s.enabled) { FormRef kk = k; bool vv = v; io(a, kk); a.boolean(vv); }
        }
    }
    {
        size_t n = s.kills.size();
        if (a.count(n, kMaxEntries)) {
            if (rd) { s.kills.clear(); for (size_t i = 0; i < n && a.ok(); ++i) { FormRef k; io(a, k); if (a.ok()) s.kills.insert(k); } }
            else for (auto& k : s.kills) { FormRef kk = k; io(a, kk); }
        }
    }
    {
        size_t n = s.questRunning.size();
        if (a.count(n, kMaxEntries)) {
            if (rd) { s.questRunning.clear(); for (size_t i = 0; i < n && a.ok(); ++i) { FormRef k; bool v = false; io(a, k); a.boolean(v); if (a.ok()) s.questRunning[k] = v; } }
            else for (auto& [k, v] : s.questRunning) { FormRef kk = k; bool vv = v; io(a, kk); a.boolean(vv); }
        }
    }
    {
        size_t n = s.placed.size();
        if (a.count(n, kMaxEntries)) {
            if (rd) { s.placed.clear(); for (size_t i = 0; i < n && a.ok(); ++i) { FormRef k; Placement p; io(a, k); io(a, p.cell); a.f32(p.x); a.f32(p.y); a.f32(p.z); if (a.ok()) s.placed[k] = p; } }
            else for (auto& [k, p0] : s.placed) { FormRef kk = k; Placement p = p0; io(a, kk); io(a, p.cell); a.f32(p.x); a.f32(p.y); a.f32(p.z); }
        }
    }
    ioRound2(a, s);
    // Only non-empty ledgers: (slot, ledger)*
    {
        size_t n = 0;
        if (!rd) for (auto& l : s.ledger) if (!l.empty()) ++n;
        if (a.count(n, kMaxSlots)) {
            if (rd) {
                for (auto& l : s.ledger) l = Ledger();
                for (size_t i = 0; i < n && a.ok(); ++i) {
                    uint8_t slot = 0; a.u8(slot);
                    if (slot >= kMaxSlots) { a.fail(); break; }
                    ioLedger(a, s.ledger[slot]);
                }
            } else {
                for (unsigned i = 0; i < kMaxSlots; ++i) if (!s.ledger[i].empty()) { uint8_t slot = static_cast<uint8_t>(i); a.u8(slot); ioLedger(a, s.ledger[i]); }
            }
        }
    }
}

}  // namespace fnvcoop
