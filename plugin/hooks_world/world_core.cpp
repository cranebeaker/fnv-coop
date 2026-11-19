#include "world_core.h"

#include <cmath>
#include <set>
#include <type_traits>
#include <variant>
#include <cstdio>
#include <utility>

#include "common/minijson.h"
#include "effect_json.h"

namespace nvworld {

using fnvcoop::FormRef;
namespace json = nvcoop::json;

const char* worldEffectName(const WorldEffect& e) {
    switch (e.index()) {
        case 0: return "SetActorValueRef";
        case 1: return "SetEssential";
        case 2: return "SetRestrained";
        case 3: return "SetRelation";
        case 4: return "SetScriptPackage";
        case 5: return "SetIgnoreCrime";
        case 6: return "SetFactionPlayerEnemy";
        case 7: return "SetOwner";
        case 8: return "SetCellOwner";
        case 9: return "SetCellPublic";
    }
    return "?";
}

// ------------------------------------------------------------------------------------------------ CreatedRefMap
void CreatedRefMap::bind(const FormRef& host, uint32_t localRefID) {
    auto it = toLocal_.find(host);
    if (it != toLocal_.end()) {
        if (FNV_MUT(1)) {
            return;  // MUTANT: first binding wins
        }
        toHost_.erase(it->second);
    }
    toLocal_[host] = localRefID;
    toHost_[localRefID] = host;
}

uint32_t CreatedRefMap::local(const FormRef& host) const {
    auto it = toLocal_.find(host);
    return it == toLocal_.end() ? 0 : it->second;
}

bool CreatedRefMap::hostOf(uint32_t localRefID, FormRef* out) const {
    auto it = toHost_.find(localRefID);
    if (it == toHost_.end()) return false;
    if (out) *out = it->second;
    return true;
}

bool CreatedRefMap::forget(const FormRef& host) {
    auto it = toLocal_.find(host);
    if (it == toLocal_.end()) return false;
    if (!FNV_MUT(2)) {
        toHost_.erase(it->second);
    }
    toLocal_.erase(it);
    return true;
}

void CreatedRefMap::clear() {
    toLocal_.clear();
    toHost_.clear();
}

// ------------------------------------------------------------------------------------------------ WorldResolver
bool WorldResolver::toRef(uint32_t refID, FormRef* out) {
    if ((refID >> 24) != 0xFFu) return inner_.toRef(refID, out);
    if (host_) {
        *out = FormRef(fnvcoop::createdRefPlugin(), refID);
        return true;
    }
    if (FNV_MUT(3)) {
        return false;  // MUTANT: client never translates its local created refs back to host refs
    }
    return map_.hostOf(refID, out);
}

uint32_t WorldResolver::fromRef(const FormRef& r) {
    if (r.plugin != fnvcoop::createdRefPlugin()) return inner_.fromRef(r);
    if (host_) return r.localId;
    return map_.local(r);
}

// ------------------------------------------------------------------------------------------------ created-ref targets
bool createdTargetOf(const fnvcoop::Effect& e, FormRef* out) {
    const FormRef* r = nullptr;
    std::visit(
        [&](const auto& x) {
            using T = std::decay_t<decltype(x)>;
            if constexpr (std::is_same_v<T, fnvcoop::EnableRef> || std::is_same_v<T, fnvcoop::DisableRef> ||
                          std::is_same_v<T, fnvcoop::MoveRef> || std::is_same_v<T, fnvcoop::SetLock> ||
                          std::is_same_v<T, fnvcoop::SwapTexture> || std::is_same_v<T, fnvcoop::ObjectState> ||
                          std::is_same_v<T, fnvcoop::RefScriptVar> || std::is_same_v<T, fnvcoop::RefAddItem> ||
                          std::is_same_v<T, fnvcoop::RefRemoveItem> || std::is_same_v<T, fnvcoop::RemoveAllItems>)
                r = &x.ref;
            else if constexpr (std::is_same_v<T, fnvcoop::KillActor> || std::is_same_v<T, fnvcoop::ResurrectActor> ||
                               std::is_same_v<T, fnvcoop::SetFactionRank>)
                r = &x.actor;
            else if constexpr (std::is_same_v<T, fnvcoop::SetScriptVar>)
                r = &x.owner;
        },
        e);
    if (!r || r->plugin != fnvcoop::createdRefPlugin()) return false;
    if (out) *out = *r;
    return true;
}

bool createdTargetOf(const WorldEffect& e, FormRef* out) {
    const FormRef* r = nullptr;
    std::visit(
        [&](const auto& x) {
            using T = std::decay_t<decltype(x)>;
            if constexpr (std::is_same_v<T, SetActorValueRef> || std::is_same_v<T, SetEssential> || std::is_same_v<T, SetRestrained> ||
                          std::is_same_v<T, SetScriptPackage> || std::is_same_v<T, SetIgnoreCrime>)
                r = &x.actor;
            else if constexpr (std::is_same_v<T, SetOwner>)
                r = &x.ref;
        },
        e);
    if (!r || r->plugin != fnvcoop::createdRefPlugin()) return false;
    if (out) *out = *r;
    return true;
}

Route routeEffect(const fnvcoop::Effect& e, const CreatedRefMap& m, FormRef* waitOn) {
    FormRef t;
    if (!createdTargetOf(e, &t)) return Route::Apply;
    if (FNV_MUT(28)) {
        return Route::Apply;  // MUTANT: follow-ups on a not-yet-created ref are applied (and lost) instead of held
    }
    if (m.local(t)) return Route::Apply;
    if (waitOn) *waitOn = t;
    return Route::HoldForCreated;
}

Route routeEffect(const WorldEffect& e, const CreatedRefMap& m, FormRef* waitOn) {
    FormRef t;
    if (!createdTargetOf(e, &t)) return Route::Apply;
    if (m.local(t)) return Route::Apply;
    if (waitOn) *waitOn = t;
    return Route::HoldForCreated;
}

bool retriable(const fnvcoop::Effect& e, Outcome o) {
    if (o == Outcome::Missing) return true;
    // the variable list exists once the object's script has been instantiated (cell attached); a rejected write may succeed later
    return o == Outcome::Rejected && std::holds_alternative<fnvcoop::RefScriptVar>(e);
}
bool retriable(const WorldEffect&, Outcome o) { return o == Outcome::Missing; }

void DeferredQueue::hold(HeldEffect h) {
    ++holds_;
    if (q_.size() >= cap_ && cap_ > 0) {
        q_.erase(q_.begin());
        ++dropped_;
    }
    q_.push_back(std::move(h));
}

std::vector<HeldEffect> DeferredQueue::takeBound(const CreatedRefMap& m) {
    std::vector<HeldEffect> out, keep;
    for (auto& h : q_) {
        // MUTANT 29: released before the ref is bound
        bool ready = FNV_MUT(29) ? h.onCreated : (h.onCreated && m.local(h.waitOn) != 0);
        (ready ? out : keep).push_back(std::move(h));
    }
    q_ = std::move(keep);
    return out;
}

std::vector<HeldEffect> DeferredQueue::takeRetries(int maxAttempts) {
    std::vector<HeldEffect> out, keep;
    for (auto& h : q_) {
        ++h.attempts;
        int limit = h.onCreated ? maxAttempts * 4 : maxAttempts;
        if (FNV_MUT(30)) {
            limit = 1 << 30;  // MUTANT: nothing ever expires
        }
        if (h.attempts > limit) {
            ++expired_;
            continue;
        }
        (h.onCreated ? keep : out).push_back(std::move(h));
    }
    q_ = std::move(keep);
    return out;
}

size_t DeferredQueue::dropFor(const FormRef& host) {
    size_t n = 0;
    for (auto it = q_.begin(); it != q_.end();) {
        if (it->onCreated && it->waitOn == host) {
            it = q_.erase(it);
            ++n;
        } else {
            ++it;
        }
    }
    return n;
}

// ------------------------------------------------------------------------------------------------ SpawnLedger
SpawnAction SpawnLedger::onSpawn(const fnvcoop::SpawnRef& s, bool cellReady) {
    if (!FNV_MUT(4)) {
        if (map_.local(s.ref())) return SpawnAction::Duplicate;
        for (const auto& p : pending_)
            if (p.id == s.id) return SpawnAction::Duplicate;
    }
    if (cellReady) return SpawnAction::Create;
    pending_.push_back(s);
    return SpawnAction::Queued;
}

DespawnResult SpawnLedger::onDespawn(const fnvcoop::DespawnRef& d) {
    DespawnResult r;
    if (!FNV_MUT(31)) {
        deferred_.dropFor(d.ref());  // MUTANT 31: held follow-ups of a despawned ref survive and hit a recycled id later
    }
    if (!FNV_MUT(5)) {
        for (auto it = pending_.begin(); it != pending_.end(); ++it) {
            if (it->id == d.id) {
                pending_.erase(it);
                r.action = DespawnAction::CancelledPending;
                return r;
            }
        }
    }
    uint32_t id = map_.local(d.ref());
    if (!id) return r;  // Unknown: never created here (or already gone)
    map_.forget(d.ref());
    r.action = DespawnAction::DeleteLocal;
    r.localRefID = id;
    return r;
}

std::vector<uint32_t> SpawnLedger::reconcile(const std::vector<fnvcoop::SpawnRef>& live) {
    std::set<uint32_t> alive;
    for (const auto& s : live) alive.insert(s.id);
    std::vector<uint32_t> stale;
    std::vector<FormRef> forget;
    if (!FNV_MUT(32)) {
        for (const auto& kv : map_.all()) {  // MUTANT 32: a rejoining client keeps spawns the host already despawned
            if (!alive.count(kv.first.localId)) {
                stale.push_back(kv.second);
                forget.push_back(kv.first);
            }
        }
    }
    for (const auto& f : forget) {
        map_.forget(f);
        deferred_.dropFor(f);
    }
    for (auto it = pending_.begin(); it != pending_.end();) {
        if (!alive.count(it->id)) it = pending_.erase(it);
        else ++it;
    }
    return stale;
}

std::vector<fnvcoop::SpawnRef> SpawnLedger::takeReady(const std::function<bool(const fnvcoop::SpawnRef&)>& ready) {
    std::vector<fnvcoop::SpawnRef> out, keep;
    for (auto& p : pending_) (ready(p) ? out : keep).push_back(p);
    if (FNV_MUT(6)) {
        return out;  // MUTANT: the queue is not drained, so the same spawn is created again on every flush
    }
    pending_ = std::move(keep);
    return out;
}

// ------------------------------------------------------------------------------------------------ RefVarTracker
static uint64_t varKey(uint32_t ref, uint16_t idx) { return (uint64_t(ref) << 16) | idx; }

bool RefVarTracker::observe(uint32_t refID, uint16_t index, double value) {
    auto k = varKey(refID, index);
    auto it = last_.find(k);
    if (it == last_.end()) {
        last_[k] = value;
        if (FNV_MUT(7)) {
            return true;  // MUTANT: the baseline value is reported as a change
        }
        return false;
    }
    if (FNV_MUT(9)) {
        ++reported_;
        it->second = value;
        return true;  // MUTANT: unchanged values are reported too
    }
    if (it->second == value) return false;
    if (!FNV_MUT(8)) {
        if (reported_ >= budget_) return false;  // deferred: the tracker keeps the old value, so a later poll reports it
    }
    it->second = value;
    ++reported_;
    return true;
}

void RefVarTracker::forgetRef(uint32_t refID) {
    if (FNV_MUT(10)) {
        return;  // MUTANT: unloaded refs keep stale baselines
    }
    for (auto it = last_.begin(); it != last_.end();) {
        if ((it->first >> 16) == refID) it = last_.erase(it);
        else ++it;
    }
}

// ------------------------------------------------------------------------------------------------ WeatherTimeSync
std::vector<fnvcoop::Effect> WeatherTimeSync::poll(const FormRef& weather, bool haveWeather, float timeScale) {
    std::vector<fnvcoop::Effect> out;
    if (haveWeather) {
        if (FNV_MUT(11)) {
            weather_ = weather;
            out.push_back(fnvcoop::SetWeather{weather, false});  // MUTANT: every poll re-sends the weather
        } else {
            if (!weather_ || !(*weather_ == weather)) {
                weather_ = weather;
                out.push_back(fnvcoop::SetWeather{weather, false});
            }
        }
    }
    if (timeScale > 0 && (!scale_ || *scale_ != timeScale)) {
        scale_ = timeScale;
        out.push_back(fnvcoop::SetTimeScale{timeScale});
    }
    return out;
}

void WeatherTimeSync::noteSent(const fnvcoop::Effect& e) {
    if (FNV_MUT(12)) {
        return;  // MUTANT: script-driven changes are sent a second time by the next poll
    }
    if (auto* w = std::get_if<fnvcoop::SetWeather>(&e)) weather_ = w->weather;
    else if (auto* t = std::get_if<fnvcoop::SetTimeScale>(&e)) scale_ = t->scale;
}

bool WeatherTimeSync::blockLocal(bool guest, bool applying) {
    if (FNV_MUT(13)) {
        return false;  // MUTANT: guests keep their own weather
    }
    return guest && !applying;
}

// ------------------------------------------------------------------------------------------------ actor values
std::optional<SetActorValueRef> makeAvEffect(const FormRef& actor, uint32_t av, AvCmdKind kind, float before, float after) {
    SetActorValueRef e;
    e.actor = actor;
    e.av = av;
    if (!FNV_MUT(15)) {
        if (before == after) return std::nullopt;
    }
    switch (kind) {
        case AvCmdKind::Set:
        case AvCmdKind::Force:
            e.mode = AvMode::Set;
            e.value = after;  // absolute: idempotent on replay
            break;
        case AvCmdKind::Mod:
            e.mode = AvMode::Mod;
            e.value = after - before;
            break;
        case AvCmdKind::Damage:
            e.mode = AvMode::Damage;
            if (FNV_MUT(14)) {
                e.value = after - before;  // MUTANT: negative damage
            } else {
                e.value = before - after;
            }
            break;
        case AvCmdKind::Restore:
            e.mode = AvMode::Restore;
            e.value = after - before;
            break;
    }
    return e;
}

const char* avName(uint32_t av) {
    // NVSE ActorValueCode order (FalloutNV.exe 1.4.0.525); console accepts these names.
    static const char* const kNames[] = {
        "Aggression", "Confidence", "Energy", "Responsibility", "Mood", "Strength", "Perception", "Endurance", "Charisma",
        "Intelligence", "Agility", "Luck", "ActionPoints", "CarryWeight", "CritChance", "HealRate", "Health", "MeleeDamage",
        "DamageResist", "PoisonResist", "RadResist", "SpeedMult", "Fatigue", "Karma", "XP", "PerceptionCondition",
        "EnduranceCondition", "LeftAttackCondition", "RightAttackCondition", "LeftMobilityCondition", "RightMobilityCondition",
        "BrainCondition", "Barter", "BigGuns", "EnergyWeapons", "Explosives", "Lockpick", "Medicine", "MeleeWeapons", "Repair",
        "Science", "Guns", "Sneak", "Speech", "Survival", "Unarmed", "InventoryWeight", "Paralysis", "Invisibility", "Chameleon",
        "NightEye", "Turbo", "FireResist", "WaterBreathing", "RadiationRads", "BloodyMess", "UnarmedDamage", "Assistance",
        "ElectricResist", "FrostResist", "EnergyResist", "EmpResist", "Variable01", "Variable02", "Variable03", "Variable04",
        "Variable05", "Variable06", "Variable07", "Variable08", "Variable09", "Variable10", "IgnoreCrippledLimbs", "Dehydration",
        "Hunger", "SleepDeprivation", "DamageThreshold"};   // 46..76: FO3/TTW scripts drive NPCs through Variable01..10
    return av < sizeof(kNames) / sizeof(kNames[0]) ? kNames[av] : "";
}

// ------------------------------------------------------------------------------------------------ console lines
static std::string hex(uint32_t v) {
    char b[16];
    std::snprintf(b, sizeof b, "%08X", v);
    return b;
}
static std::string num(float v) {
    char b[48];
    std::snprintf(b, sizeof b, "%.4f", v);
    // Shortest form ("9", "2.5"): console commands whose value parameter is an int ignore a "9.0000" token (found in game:
    // SetActorValue/ModActorValue with "9.0000" did nothing, the same value as "9" works).
    std::string r = b;
    if (r.find('.') != std::string::npos) {
        while (r.back() == '0') r.pop_back();
        if (r.back() == '.') r.pop_back();
    }
    return r;
}

std::string markForDeleteLine() { return "MarkForDelete"; }

std::string setEssentialLine(const SetEssential& e, uint32_t id) {
    const bool v = FNV_MUT(16) ? !e.essential : e.essential;  // MUTANT 16: inverted flag
    if (e.refOnly) return std::string("SetActorRefEssential ") + (v ? "1" : "0");
    return "SetEssential " + hex(id) + (v ? " 1" : " 0");
}

std::string setRestrainedLine(bool on) { return on ? "SetRestrained 1" : "SetRestrained 0"; }

std::string setRelationLine(Relation kind, uint32_t a, uint32_t b) {
    if (FNV_MUT(20)) {
        std::swap(a, b);  // MUTANT
    }
    const char* cmd = kind == Relation::Ally ? "SetAlly " : "SetEnemy ";
    // Neutral: SetEnemy with flags 0 (engine clears the relation); enemy/ally set both directions' default flags.
    return cmd + hex(a) + " " + hex(b) + (kind == Relation::Neutral ? " 0" : "");
}

std::string scriptPackageLine(bool add, uint32_t packageID) {
    return std::string(add ? "AddScriptPackage " : "RemoveScriptPackage ") + hex(packageID);
}

std::string ignoreCrimeLine(bool on) {
    if (FNV_MUT(38)) {
        on = !on;  // MUTANT
    }
    return on ? "IgnoreCrime 1" : "IgnoreCrime 0";
}

std::string setPCEnemyOfFactionLine(uint32_t factionID, bool on) {
    if (FNV_MUT(33)) {
        on = !on;  // MUTANT
    }
    return "SetPCEnemyOfFaction " + hex(factionID) + (on ? " 1" : " 0");
}

std::string setOwnershipLine(uint32_t ownerID) {
    if (FNV_MUT(47)) ownerID = 0;   // MUTANT 47: the owner argument is dropped (the player ends up owning the thing)
    return ownerID ? "SetOwnership " + hex(ownerID) : std::string("SetOwnership");
}

std::string clearOwnershipLine() { return "ClearOwnership"; }

std::string setCellOwnershipLine(uint32_t cellID, uint32_t ownerID) {
    if (FNV_MUT(48)) std::swap(cellID, ownerID);   // MUTANT 48: arguments swapped
    return "SetCellOwnership " + hex(cellID) + " " + hex(ownerID);
}

std::string setCellPublicFlagLine(uint32_t cellID, bool on) { return "SetCellPublicFlag " + hex(cellID) + (on ? " 1" : " 0"); }

std::string avLine(const SetActorValueRef& e) {
    const char* n = avName(e.av);
    if (!*n) return "";
    const char* cmd = nullptr;
    switch (e.mode) {
        case AvMode::Set: cmd = FNV_MUT(17) ? "ForceActorValue " : "SetActorValue "; break;  // MUTANT 17: Set must change the base value, not the modified one
        case AvMode::Mod: cmd = "ModActorValue "; break;
        case AvMode::Damage: cmd = "DamageActorValue "; break;
        case AvMode::Restore: cmd = "RestoreActorValue "; break;
    }
    return std::string(cmd) + n + " " + num(e.value);
}

std::string placeAtMeLine(uint32_t baseID, int count) {
    if (FNV_MUT(21)) {
        (void)count;
        return "PlaceAtMe " + hex(baseID);  // MUTANT: count dropped
    }
    return "PlaceAtMe " + hex(baseID) + " " + std::to_string(count);
}

std::string setPosLine(char axis, float v) { return std::string("SetPos ") + axis + " " + num(v); }

// ------------------------------------------------------------------------------------------------ DeathWatch
std::vector<uint32_t> DeathWatch::update(const std::vector<std::pair<uint32_t, bool>>& seen) {
    std::vector<uint32_t> died;
    for (const auto& [id, dead] : seen) {
        auto it = alive_.find(id);
        if (!dead) { alive_[id] = true; continue; }
        if (it != alive_.end() && it->second) { died.push_back(id); it->second = false; }
        else if (it == alive_.end()) alive_[id] = FNV_MUT(53);   // MUTANT 53: a corpse seen for the first time counts as alive (reported)
    }
    return died;
}

// ------------------------------------------------------------------------------------------------ WorldLog
namespace {
// Same target of a state effect (cell, ref, actor, faction pair...): a later write of it replaces an earlier one. Never for actor values.
bool sameStateKey(const WorldEffect& w, const WorldEffect& e) {
    if (w.index() != e.index()) return false;
    if (auto* x = std::get_if<SetEssential>(&e)) return std::get<SetEssential>(w).actor == x->actor && std::get<SetEssential>(w).refOnly == x->refOnly;
    if (auto* x = std::get_if<SetRestrained>(&e)) return std::get<SetRestrained>(w).actor == x->actor;
    if (auto* x = std::get_if<SetIgnoreCrime>(&e)) return std::get<SetIgnoreCrime>(w).actor == x->actor;
    if (auto* x = std::get_if<SetFactionPlayerEnemy>(&e)) return std::get<SetFactionPlayerEnemy>(w).faction == x->faction;
    if (auto* x = std::get_if<SetOwner>(&e)) return std::get<SetOwner>(w).ref == x->ref;
    if (auto* x = std::get_if<SetCellOwner>(&e)) return std::get<SetCellOwner>(w).cell == x->cell;
    if (auto* x = std::get_if<SetCellPublic>(&e)) return std::get<SetCellPublic>(w).cell == x->cell;
    if (auto* x = std::get_if<SetRelation>(&e)) return std::get<SetRelation>(w).a == x->a && std::get<SetRelation>(w).b == x->b;
    if (auto* x = std::get_if<SetScriptPackage>(&e))
        return std::get<SetScriptPackage>(w).actor == x->actor && std::get<SetScriptPackage>(w).package == x->package;
    return false;
}
}  // namespace

void WorldLog::push(const WorldEffect& e) {
    // A script that sets the same state every frame (Megaton Clinic's "SetCellPublicFlag MegatonClinic 0" ran each frame on the host) sent
    // it every frame: 10 000 applies on the guest in a few minutes. A state write equal to the last one pushed for its key is dropped.
    if (!std::holds_alternative<SetActorValueRef>(e)) {
        auto it = std::find_if(sent_.begin(), sent_.end(), [&](const WorldEffect& w) { return sameStateKey(w, e); });
        if (it != sent_.end() && *it == e) { ++repeats_; return; }
        if (it != sent_.end()) *it = e;
        else sent_.push_back(e);
    }
    if (!FNV_MUT(18)) {
        for (auto& q : q_) {
            if (q.index() != e.index()) continue;
            bool same = false;
            if (auto* a = std::get_if<SetActorValueRef>(&e)) {
                auto& b = std::get<SetActorValueRef>(q);
                same = a->mode == AvMode::Set && b.mode == AvMode::Set && a->actor == b.actor && a->av == b.av;
            } else if (auto* c = std::get_if<SetEssential>(&e)) {
                auto& d = std::get<SetEssential>(q);
                same = c->actor == d.actor && c->refOnly == d.refOnly;
            } else if (auto* f = std::get_if<SetRestrained>(&e)) {
                same = f->actor == std::get<SetRestrained>(q).actor;
            } else if (auto* g = std::get_if<SetIgnoreCrime>(&e)) {
            if (FNV_MUT(39)) {
                    same = false;  // MUTANT: IgnoreCrime never coalesces
            } else {
                    same = g->actor == std::get<SetIgnoreCrime>(q).actor;
            }
            } else if (auto* h = std::get_if<SetFactionPlayerEnemy>(&e)) {
                same = h->faction == std::get<SetFactionPlayerEnemy>(q).faction;
            } else if (auto* o = std::get_if<SetOwner>(&e)) {
                same = o->ref == std::get<SetOwner>(q).ref;
            } else if (auto* co = std::get_if<SetCellOwner>(&e)) {
                same = co->cell == std::get<SetCellOwner>(q).cell;
            } else if (auto* cp = std::get_if<SetCellPublic>(&e)) {
                same = cp->cell == std::get<SetCellPublic>(q).cell;
            }
            if (same) {
                q = e;
                return;
            }
        }
    }
    q_.push_back(e);
}

std::vector<WorldEffect> WorldLog::drain() {
    std::vector<WorldEffect> out;
    out.swap(q_);
    return out;
}

// ------------------------------------------------------------------------------------------------ WorldBaseline
void WorldBaseline::purge(const FormRef& ref) {
    for (auto it = vars_.begin(); it != vars_.end();) {
        if (it->first.first == ref) it = vars_.erase(it);
        else ++it;
    }
    for (auto it = world_.begin(); it != world_.end();) {
        FormRef t;
        if (createdTargetOf(*it, &t) && t == ref) it = world_.erase(it);
        else ++it;
    }
}

void WorldBaseline::onEffect(const fnvcoop::Effect& e) {
    if (auto* sp = std::get_if<fnvcoop::SpawnRef>(&e)) {
        for (auto& x : spawns_)
            if (x.id == sp->id) return;  // a replayed txn
        spawns_.push_back(*sp);
    } else if (auto* d = std::get_if<fnvcoop::DespawnRef>(&e)) {
        for (auto it = spawns_.begin(); it != spawns_.end(); ++it)
            if (it->id == d->id) {
                spawns_.erase(it);
                break;
            }
        if (!FNV_MUT(34)) {
            purge(d->ref());  // MUTANT 34: state of a despawned ref stays in the baseline
        }
    } else if (auto* v = std::get_if<fnvcoop::RefScriptVar>(&e)) {
        vars_[{v->ref, v->index}] = v->value;
    }
}

namespace {
// Folds one world effect into the net-state list.
void fold(std::vector<WorldEffect>& q, const WorldEffect& e) {
    if (auto* a = std::get_if<SetActorValueRef>(&e)) {
        auto sameKey = [&](const WorldEffect& w) {
            auto* b = std::get_if<SetActorValueRef>(&w);
            return b && b->actor == a->actor && b->av == a->av;
        };
        if (a->mode == AvMode::Set) {
            if (!FNV_MUT(35)) {
                for (auto it = q.begin(); it != q.end();)  // MUTANT 35: deltas from before the Set survive and are applied on top of it
                    if (sameKey(*it)) it = q.erase(it);
                    else ++it;
            }
            q.push_back(e);
            return;
        }
        for (size_t i = q.size(); i-- > 0;) {
            if (!sameKey(q[i])) continue;
            auto& b = std::get<SetActorValueRef>(q[i]);
            if (b.mode == a->mode) {
                if (FNV_MUT(36)) {
                    b.value = a->value;  // MUTANT: the latest delta replaces the sum
                } else {
                    b.value += a->value;
                }
                if (b.value == 0 && b.mode == AvMode::Mod) q.erase(q.begin() + i);
                return;
            }
            break;  // a different kind was last: keep order, append
        }
        q.push_back(e);
        return;
    }
    // everything else: last write per key wins, in place
    auto same = [&](const WorldEffect& w) {
        if (w.index() != e.index()) return false;
        if (auto* x = std::get_if<SetEssential>(&e)) return std::get<SetEssential>(w).actor == x->actor && std::get<SetEssential>(w).refOnly == x->refOnly;
        if (auto* x = std::get_if<SetRestrained>(&e)) return std::get<SetRestrained>(w).actor == x->actor;
        if (auto* x = std::get_if<SetIgnoreCrime>(&e)) return std::get<SetIgnoreCrime>(w).actor == x->actor;
        if (auto* x = std::get_if<SetFactionPlayerEnemy>(&e)) return std::get<SetFactionPlayerEnemy>(w).faction == x->faction;
        if (auto* x = std::get_if<SetOwner>(&e)) return std::get<SetOwner>(w).ref == x->ref;
        if (auto* x = std::get_if<SetCellOwner>(&e)) return std::get<SetCellOwner>(w).cell == x->cell;
        if (auto* x = std::get_if<SetCellPublic>(&e)) return std::get<SetCellPublic>(w).cell == x->cell;
        if (auto* x = std::get_if<SetRelation>(&e)) return std::get<SetRelation>(w).a == x->a && std::get<SetRelation>(w).b == x->b;
        if (auto* x = std::get_if<SetScriptPackage>(&e))
            return std::get<SetScriptPackage>(w).actor == x->actor && std::get<SetScriptPackage>(w).package == x->package;
        return false;
    };
    for (auto& w : q)
        if (same(w)) {
            w = e;
            return;
        }
    q.push_back(e);
}
}  // namespace

void WorldBaseline::onWorld(const WorldEffect& e) { fold(world_, e); }

BaselineSnapshot WorldBaseline::snapshot() const {
    BaselineSnapshot b;
    for (const auto& s : spawns_) b.effects.push_back(s);
    for (const auto& kv : vars_) b.effects.push_back(fnvcoop::RefScriptVar{kv.first.first, kv.first.second, kv.second});
    b.world = world_;
    return b;
}

std::vector<FormRef> WorldBaseline::liveSpawnRefs() const {
    std::vector<FormRef> out;
    for (const auto& s : spawns_) out.push_back(s.ref());
    return out;
}

void WorldBaseline::clear() {
    spawns_.clear();
    vars_.clear();
    world_.clear();
}

// ------------------------------------------------------------------------------------------------ JSON
static json::Value J(const FormRef& r) { return json::Value(nvhooks::refToStr(r)); }

std::string worldEffectToJson(const WorldEffect& e) {
    json::Value o = json::Value::object();
    o.set("type", worldEffectName(e));
    if (auto* x = std::get_if<SetActorValueRef>(&e)) {
        o.set("actor", J(x->actor)).set("av", (int64_t)x->av).set("value", (double)x->value).set("mode", (int64_t)x->mode);
    } else if (auto* y = std::get_if<SetEssential>(&e)) {
        o.set("actor", J(y->actor)).set("essential", y->essential);
        if (!FNV_MUT(19)) {
            o.set("refOnly", y->refOnly);
        }
    } else if (auto* z = std::get_if<SetRestrained>(&e)) {
        o.set("actor", J(z->actor)).set("on", z->on);
    } else if (auto* r = std::get_if<SetRelation>(&e)) {
        o.set("a", J(r->a)).set("b", J(r->b)).set("kind", (int64_t)r->kind);
    } else if (auto* p = std::get_if<SetScriptPackage>(&e)) {
        o.set("actor", J(p->actor)).set("package", J(p->package)).set("add", p->add);
    } else if (auto* c = std::get_if<SetIgnoreCrime>(&e)) {
        o.set("actor", J(c->actor));
        if (!FNV_MUT(40)) {
            o.set("on", c->on);  // MUTANT 40: the flag is not serialised
        }
    } else if (auto* f = std::get_if<SetFactionPlayerEnemy>(&e)) {
        o.set("faction", J(f->faction)).set("on", f->on);
    } else if (auto* ow = std::get_if<SetOwner>(&e)) {
        o.set("ref", J(ow->ref)).set("clear", ow->clear);
        if (!ow->owner.plugin.empty()) o.set("owner", J(ow->owner));
    } else if (auto* co = std::get_if<SetCellOwner>(&e)) {
        o.set("cell", J(co->cell)).set("owner", J(co->owner));
    } else if (auto* cp = std::get_if<SetCellPublic>(&e)) {
        o.set("cell", J(cp->cell)).set("on", cp->on);
    }
    return o.dump();
}

bool worldEffectFromJson(const std::string& text, WorldEffect* out, std::string* err) {
    json::Value v;
    std::string perr;
    if (!json::parse(text, v, perr)) {
        if (err) *err = perr;
        return false;
    }
    auto ref = [&](const char* k, FormRef* r) { return nvhooks::strToRef(v.get(k).asString(""), r); };
    std::string t = v.get("type").asString("");
    if (t == "SetActorValueRef") {
        SetActorValueRef x;
        if (!ref("actor", &x.actor)) { if (err) *err = "bad actor"; return false; }
        x.av = (uint32_t)v.get("av").asInt();
        x.value = (float)v.get("value").asNumber();
        x.mode = (AvMode)v.get("mode").asInt();
        *out = x;
    } else if (t == "SetEssential") {
        SetEssential x;
        if (!ref("actor", &x.actor)) { if (err) *err = "bad actor"; return false; }
        x.essential = v.get("essential").asBool();
        x.refOnly = v.get("refOnly").asBool();
        *out = x;
    } else if (t == "SetRestrained") {
        SetRestrained x;
        if (!ref("actor", &x.actor)) { if (err) *err = "bad actor"; return false; }
        x.on = v.get("on").asBool();
        *out = x;
    } else if (t == "SetRelation") {
        SetRelation x;
        if (!ref("a", &x.a) || !ref("b", &x.b)) { if (err) *err = "bad faction"; return false; }
        x.kind = (Relation)v.get("kind").asInt();
        *out = x;
    } else if (t == "SetScriptPackage") {
        SetScriptPackage x;
        if (!ref("actor", &x.actor) || !ref("package", &x.package)) { if (err) *err = "bad ref"; return false; }
        x.add = v.get("add").asBool(true);
        *out = x;
    } else if (t == "SetIgnoreCrime") {
        SetIgnoreCrime x;
        if (!ref("actor", &x.actor)) { if (err) *err = "bad actor"; return false; }
        x.on = v.get("on").asBool();
        *out = x;
    } else if (t == "SetFactionPlayerEnemy") {
        SetFactionPlayerEnemy x;
        if (!ref("faction", &x.faction)) { if (err) *err = "bad faction"; return false; }
        x.on = v.get("on").asBool();
        *out = x;
    } else if (t == "SetOwner") {
        SetOwner x;
        if (!ref("ref", &x.ref)) { if (err) *err = "bad ref"; return false; }
        x.clear = v.get("clear").asBool();
        if (!v.get("owner").asString("").empty() && !ref("owner", &x.owner)) { if (err) *err = "bad owner"; return false; }
        *out = x;
    } else if (t == "SetCellOwner") {
        SetCellOwner x;
        if (!ref("cell", &x.cell) || !ref("owner", &x.owner)) { if (err) *err = "bad cell or owner"; return false; }
        *out = x;
    } else if (t == "SetCellPublic") {
        SetCellPublic x;
        if (!ref("cell", &x.cell)) { if (err) *err = "bad cell"; return false; }
        x.on = v.get("on").asBool();
        *out = x;
    } else {
        if (err) *err = "unknown world effect type: " + t;
        return false;
    }
    return true;
}

}  // namespace nvworld
