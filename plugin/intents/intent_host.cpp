#include "intents/intent_host.h"

#include <algorithm>
#include <cstdio>
#include <sstream>

#include "fnvcoop_mut.h"

namespace nvcoop::intents {

namespace {

// Twin 312 runs the interaction without an acting scope: nothing is captured, the player context is the host's.
class NoScope : public IActingScope {
public:
    bool record(const fnvcoop::Effect&) override { return false; }
};

constexpr size_t kRecentVerdicts = 32;

}  // namespace

IntentHost::IntentHost(HostDeps deps, Config cfg, ContainerRedirect redirect, StorageBook* stash)
    : deps_(std::move(deps)), cfg_(std::move(cfg)), redirect_(std::move(redirect)), stash_(stash) {}

Verdict IntentHost::verdictOf(const Intent& in, Result r) const {
    Verdict v;
    v.session = in.session;
    v.id = in.id;
    v.result = r;
    return v;
}

void IntentHost::count(const Intent& in, Result r) {
    if (r == Result::Executed) {
        ++stats_.executed;
        ++stats_.byKind[static_cast<size_t>(in.kind)];
    } else if (r == Result::Duplicate) {
        ++stats_.duplicates;
    } else {
        ++stats_.rejected[static_cast<size_t>(r)];
    }
}

namespace {
constexpr uint64_t kRefusalLogGapMs = 1000;

std::string refText(const FormRef& r) {
    char b[24];
    std::snprintf(b, sizeof b, "%X", static_cast<unsigned>(r.localId));
    return r.plugin.empty() ? std::string("?") : r.plugin + ":" + b;
}
std::string posText(const Vec3& v) {
    char b[80];
    std::snprintf(b, sizeof b, "(%.0f,%.0f,%.0f)", v.x, v.y, v.z);
    return b;
}
}  // namespace

// What the log line says about the sides of a refusal: the target as the host sees it (class, loaded, cell, position) and the guest's acting player
// (puppet / ghost ref, its position and cell) and the distance between the two. Read-only: nothing here may change what the host does next.
std::string IntentHost::refusalDetail(PlayerId slot, const Intent& in) {
    std::ostringstream o;
    TargetInfo t;
    const bool known = in.kind != Kind::SetStage && deps_.eng && deps_.eng->describe(in.target, t);
    o << "target " << refText(in.target);
    if (known) {
        o << " class " << className(t.cls) << (t.loaded ? " loaded" : " NOT-loaded") << (t.disabled ? " disabled" : "") << (t.dead ? " dead" : "") << " cell " << std::hex << t.cell << std::dec;
        if (t.hasPos) o << " pos " << posText(t.pos);
    } else if (in.kind != Kind::SetStage) {
        o << " (not resolvable on the host)";
    }
    RefId actor = deps_.puppetOf ? deps_.puppetOf(slot) : 0;
    const char* what = "puppet";
    if (!actor && deps_.ghostOf) { actor = deps_.ghostOf(slot); what = "ghost"; }
    if (!actor) {
        o << "; no " << what << " for slot " << int(slot);
        return o.str();
    }
    Vec3 p;
    uint32_t pcell = 0;
    if (deps_.eng && deps_.eng->position(actor, p, &pcell)) {
        o << "; " << what << " " << std::hex << actor << std::dec << " pos " << posText(p) << " cell " << std::hex << pcell << std::dec;
        if (known && t.hasPos) o << "; distance " << static_cast<int>(distance(p, t.pos)) << " (reach " << static_cast<int>(reachOf(cfg_, in.kind)) << ")";
    } else {
        o << "; " << what << " " << std::hex << actor << std::dec << " has no position here";
    }
    return o.str();
}

void IntentHost::logRefusal(PlayerId slot, const Intent& in, Result r) {
    if (!deps_.log || r == Result::Duplicate || FNV_MUT(416)) return;   // MUTANT 416: a refused intent leaves no trace in the log
    const bool ran = r == Result::Executed;
    if (ran && (in.kind == Kind::TriggerEnter || in.kind == Kind::TriggerLeave || in.kind == Kind::TriggerStay)) return;   // triggers are frequent and uninteresting when they run
    const uint64_t t = now();
    if (refusalGate_.size() > 256) refusalGate_.clear();
    RefusalGate& g = refusalGate_[std::make_pair(static_cast<uint8_t>(in.kind), static_cast<uint8_t>(r))];
    if (g.lastMs && t - g.lastMs < kRefusalLogGapMs && !FNV_MUT(417)) {   // MUTANT 417: no rate limit (a guest in a trigger volume writes a line per frame)
        ++g.skipped; ++stats_.refusalsQuiet; return; }
    std::ostringstream o;
    o << "intents: " << (ran ? "ran " : "REFUSED ") << kindName(in.kind) << " from slot " << int(slot) << ": " << resultName(r) << "; " << refusalDetail(slot, in);
    if (g.skipped) o << "; (" << g.skipped << " more of this kind and reason in the last second)";
    g.lastMs = t;
    g.skipped = 0;
    ++stats_.refusalsLogged;
    deps_.log(o.str());
}

void IntentHost::onDatagram(PlayerId from, const uint8_t* data, size_t size) {
    Intent in;
    if (decode(data, size, &in, nullptr) != WireType::Intent) { ++stats_.badDatagrams; return; }
    handle(from, in);
}

bool IntentHost::rateOk(SlotState& s, uint64_t t) {
    if (FNV_MUT(307)) return true;   // MUTANT 307: no rate limit (a guest can flood the host's script engine)
    if (!s.init) { s.tokens = cfg_.rateBurst; s.refilledAt = t; s.init = true; }
    if (t > s.refilledAt) {
        s.tokens = std::min<double>(cfg_.rateBurst, s.tokens + (t - s.refilledAt) / 1000.0 * cfg_.ratePerSec);
        s.refilledAt = t;
    }
    if (s.tokens < 1.0) return false;
    s.tokens -= 1.0;
    return true;
}

Verdict IntentHost::handle(PlayerId slot, const Intent& in) {
    ++stats_.received;
    if (!deps_.joined || !deps_.joined(slot)) {
        Verdict v = verdictOf(in, Result::NotJoined);
        count(in, v.result);
        logRefusal(slot, in, v.result);
        return v;   // not a player: no answer, nothing to answer to
    }
    SlotState& s = slots_[slot];
    if (in.session != s.session) { s.session = in.session; s.lastId = 0; s.recent.clear(); }   // a new join: ids start again
    if (in.id == 0 || (in.id <= s.lastId && !FNV_MUT(308))) {   // MUTANT 308: no exactly-once check (a resend runs the interaction twice)
        ++stats_.duplicates;
        Verdict v = verdictOf(in, in.id == 0 ? Result::Stale : Result::Duplicate);
        for (const Verdict& old : s.recent)
            if (old.id == in.id) { v = old; break; }   // the same answer again; nothing runs twice
        if (deps_.send && deps_.send(slot, encode(v))) ++stats_.verdictsSent; else ++stats_.verdictsFailed;
        return v;
    }
    s.lastId = in.id;
    Verdict v;
    if (!rateOk(s, now())) v = verdictOf(in, Result::RateLimited);
    else v = decide(slot, in, false);
    count(in, v.result);
    logRefusal(slot, in, v.result);
    s.recent.push_back(v);
    while (s.recent.size() > kRecentVerdicts) s.recent.pop_front();
    if (!FNV_MUT(318)) {   // MUTANT 318: the guest never hears back (its pending list fills up and times out)
        if (deps_.send && deps_.send(slot, encode(v))) ++stats_.verdictsSent; else ++stats_.verdictsFailed;
    }
    return v;
}

Result IntentHost::runLegacy(PlayerId slot, fnvcoop::IntentKind kind, const FormRef& ref) {
    ++stats_.legacy;
    Intent in;
    in.target = ref;
    switch (kind) {
        case fnvcoop::IntentKind::ActivateRef: in.kind = Kind::Activate; break;
        case fnvcoop::IntentKind::TriggerEnter: in.kind = Kind::TriggerEnter; break;
        case fnvcoop::IntentKind::PickUp: in.kind = Kind::PickUp; break;
        default: return Result::BadTarget;
    }
    if (!deps_.joined || !deps_.joined(slot)) { count(in, Result::NotJoined); logRefusal(slot, in, Result::NotJoined); return Result::NotJoined; }
    SlotState& s = slots_[slot];
    Result r = rateOk(s, now()) ? decide(slot, in, true).result : Result::RateLimited;
    count(in, r);
    logRefusal(slot, in, r);
    return r;
}

void IntentHost::noteActor(PlayerId slot, ActorMode mode, RefId actor) {
    auto it = lastMode_.find(slot);
    if (it != lastMode_.end() && it->second == mode && !FNV_MUT(330)) return;   // MUTANT 330: reported on every action, not on a change
    lastMode_[slot] = mode;
    if (mode == ActorMode::StandIn && FNV_MUT(331)) return;   // MUTANT 331: a stand-in is never reported (the silent host-player run)
    if (deps_.actorMode) deps_.actorMode(slot, mode, actor);
}

bool IntentHost::pickActor(PlayerId slot, Kind k, TargetClass cls, Actor* out) {
    RefId p = deps_.puppetOf ? deps_.puppetOf(slot) : 0;
    if (FNV_MUT(311)) p = 0;   // MUTANT 311: the puppet is never used (everything runs as the host's player)
    if (p) { out->ref = p; out->puppet = true; noteActor(slot, ActorMode::Puppet, p); return true; }
    RefId g = deps_.ghostOf ? deps_.ghostOf(slot) : 0;
    if (g && !FNV_MUT(311)) { out->ref = g; out->ghost = true; noteActor(slot, ActorMode::Ghost, g); return true; }
    if (!standInAllowed(cfg_, k, cls)) return false;
    out->ref = kHostPlayerRef;
    out->standIn = true;
    noteActor(slot, ActorMode::StandIn, kHostPlayerRef);
    return true;
}

Result IntentHost::checkReach(const Actor& a, const TargetInfo& t, Kind k) {
    const float reach = reachOf(cfg_, k);
    Vec3 p;
    if (!cfg_.checkReach || a.standIn || !t.loaded || FNV_MUT(313) || reach <= 0.f || !t.hasPos || !deps_.eng->position(a.ref, p, nullptr)) {   // MUTANT 313: never checked
        ++stats_.reachUnchecked;
        return Result::Executed;
    }
    ++stats_.reachChecked;
    return distance(p, t.pos) > reach ? Result::OutOfRange : Result::Executed;
}

Result IntentHost::runScoped(PlayerId slot, const Actor& a, Kind k, const FormRef& target, const std::function<Result(IActingScope&)>& fn, bool stash) {
    Result r = Result::Failed;
    Cause c;
    c.kind = CauseKind::Intent;
    c.source = target;
    c.aux = static_cast<int32_t>(k);
    bool ran = deps_.runAsGuest && deps_.runAsGuest(slot, [&] {
        StashHold hold;
        if (stash) hold = stashEnter(slot);   // the shared gear containers hold this player's stash while the interaction runs (outside the capture scope: nothing is recorded)
        {
            std::unique_ptr<IActingScope> scope = FNV_MUT(312) ? std::unique_ptr<IActingScope>(new NoScope()) : deps_.eng->beginActing(a.ref, slot, c);
            if (!scope) r = Result::Failed;
            else r = fn(*scope);
        }   // the scope is closed here (its txn emitted) while the origin is still the guest
        if (stash) stashExit(hold);
    });
    if (!ran) return Result::Failed;
    if (r == Result::Executed) {
        if (a.puppet) ++stats_.actedAsPuppet;
        else if (a.ghost) ++stats_.actedAsGhost;
        else ++stats_.actedAsStandIn;
    }
    return r;
}

// A script event for a guest. Three things make a block run as that guest:
//  * the event's object is the host's player (0x14), not the puppet: `OnTrigger Player` / `OnTriggerEnter Player` / `OnAdd Player` compare the object of the marked event with the
//    form they name (MarkBaseExtraListScriptEvent adds the mask to the entry of the object and to the entry of "any object"), and a puppet is not that form. Only the blocks that
//    name no object matched the puppet; the Dead Money return trigger (`OnTrigger Player`) never did.
//  * the target's script runs now, inside the acting scope: the mark is a flag, the engine runs the block on the reference's next update, when no scope is open and `player`
//    (and the origin of everything the block does) is the host's.
//  * a mark the host's own player left pending is run first, outside the scope, so that the guest's run does not eat it (both marks share the 0x14 entry).
bool IntentHost::raiseNow(const Actor& a, const FormRef& target, uint32_t mask) {
    if (!deps_.eng->raiseEvent(FNV_MUT(421) ? a.ref : kHostPlayerRef, target, mask)) return false;   // MUTANT 421: the puppet is the event's object (the `Player` filter never matches it)
    if (FNV_MUT(422)) return true;                                                                                        // MUTANT 422: the block is left to the engine's next update (no scope, `player` is the host)
    if (deps_.eng->runScript(target)) ++stats_.eventScriptRuns; else ++stats_.eventScriptMissed;
    return true;
}

Result IntentHost::raiseFor(PlayerId slot, const Actor& a, Kind k, const FormRef& target, uint32_t mask, bool stash) {
    if (!FNV_MUT(423) && deps_.eng->eventPending(0, target, 0xFFFFFFFFu)) {   // MUTANT 423: a mark somebody else left pending (the host's own player in the same volume) is eaten by the guest's run
        deps_.eng->runScript(target);
        ++stats_.eventsFlushed;
    }
    return runScoped(slot, a, k, target, [&](IActingScope&) { return raiseNow(a, target, mask) ? Result::Executed : Result::Failed; }, stash);
}

// ---- gear stashes (storage_book.h) -------------------------------------------------------------------------------------------------------------

IntentHost::StashHold IntentHost::stashEnter(PlayerId slot) {
    StashHold hold;
    hold.slot = slot;
    if (!stash_ || stash_->empty() || !cfg_.storage || !deps_.eng) return hold;
    if (slot == kHostPlayer && !FNV_MUT(410)) return hold;   // MUTANT 410: the host's own player is swapped too (its container goes empty, its gear is "parked")
    if (stashDepth_ > 0) return hold;                        // nested: the outer window already holds the right contents
    ++stashDepth_;
    hold.active = true;
    for (const StorageBook::Entry& e : stash_->entries()) {
        StashHold::One one;
        one.container = e.container;
        std::vector<StoredItem> now;
        if (!deps_.eng->readItems(e.container, &now)) continue;   // not loaded / not readable here: this container is not part of the interaction
        now = normalizeItems(std::move(now));
        const std::vector<StoredItem>* mine = stash_->peek(e.container, slot);
        if (now.empty() && !mine) { hold.held.push_back(one); ++stats_.stashKept; continue; }   // nothing parked, nothing to load: only the way back is checked
        std::vector<StoredItem> load = mine ? *mine : std::vector<StoredItem>();
        if (FNV_MUT(405)) load = now;   // MUTANT 405: nothing is swapped in (the guest's script sees whatever is in the container, the host's gear included)
        if (!deps_.eng->replaceItems(e.container, load)) { ++stats_.stashFailed; continue; }   // untouched: leave it out of the way back too
        stash_->take(e.container, slot);   // the contents now live in the container
        one.parked = std::move(now);
        one.loaded = std::move(load);
        one.swapped = true;
        ++stats_.stashSwaps;
        hold.held.push_back(std::move(one));
    }
    return hold;
}

void IntentHost::stashExit(StashHold& hold) {
    if (!hold.active) return;
    hold.active = false;
    --stashDepth_;
    for (StashHold::One& one : hold.held) {
        std::vector<StoredItem> mine;
        if (!deps_.eng->readItems(one.container, &mine)) {
            ++stats_.stashFailed;
            mine = FNV_MUT(414) ? std::vector<StoredItem>() : one.loaded;   // MUTANT 414: a failed read is taken as "empty" (the guest's gear is wiped). Real: assume nothing changed, still give the parked contents back
        }
        mine = normalizeItems(std::move(mine));
        if (!one.swapped && mine.empty()) continue;   // nothing was parked and the interaction left it empty
        if (!FNV_MUT(411)) stash_->put(one.container, hold.slot, mine);   // MUTANT 411: what the guest left in the container is not filed (it is lost to the parked contents)
        if (FNV_MUT(404)) continue;   // MUTANT 404: the parked contents are not put back (the host's own gear stays gone / the guest's stays in the shared container)
        if (!deps_.eng->replaceItems(one.container, one.parked)) {
            ++stats_.stashFailed;
            stash_->put(one.container, hold.slot, mine);   // could not clean up: the guest's stash is still filed, the container keeps a copy
        }
    }
}

Verdict IntentHost::decide(PlayerId slot, const Intent& in, bool /*legacy*/) {
    if (!kindEnabled(cfg_, in.kind) && !FNV_MUT(315)) return verdictOf(in, Result::Disabled);   // MUTANT 315: a switched-off kind still runs
    if (in.kind == Kind::ContainerClose) return doContainerClose(slot, in);
    if (in.kind == Kind::SetStage) return doSetStage(slot, in);   // the target is a quest form, not a reference: no describe()
    TargetInfo t;
    if (!deps_.eng->describe(in.target, t)) return verdictOf(in, Result::NotLoaded);
    if (!t.loaded) {
        // A guest alone in an area the host never loaded (story run 2026-10-28: the Broc flower far from the host): the object's form and its script are still in memory.
        // An activator's script is all the host has to run, and the guest's own game vouches for the reach, so it runs instead of being lost with a half-done activation
        // on the guest (XP, no item). Everything that needs the 3D world (a door, a container, a terminal, an NPC) still needs the cell.
        const bool scriptOnly = cfg_.runUnloaded && t.resident && in.kind == Kind::Activate && (t.cls == TargetClass::Activator || t.cls == TargetClass::Unknown) && !FNV_MUT(418);   // MUTANT 418: an unloaded target is refused as before (the guest's activation is half done)
        if (!scriptOnly) return verdictOf(in, Result::NotLoaded);
        ++stats_.ranUnloaded;
    }
    // plugin/companions: somebody else's companion is not for this guest to use (its wheel opens on its owner's machine only)
    if ((in.kind == Kind::Activate || in.kind == Kind::DialogueStart) && deps_.allowActivation && !FNV_MUT(327)   // MUTANT 327: the question is never asked
        && !deps_.allowActivation(slot, in.target)) {
        ++stats_.activationsRefused;
        return verdictOf(in, Result::Denied);
    }
    Verdict v = dispatch(slot, in, t);
    // ... and what the interaction touched (a take, an owned or locked thing) is judged by the host on the guest's behalf
    const bool opened = in.kind == Kind::Activate || in.kind == Kind::PickUp || in.kind == Kind::ContainerOpen;
    if (deps_.interacted && opened && !FNV_MUT(328) && (v.result == Result::Executed || FNV_MUT(329)) && !v.hasRedirect) {   // 328: never told; 329: told of refused ones too
        ++stats_.interactionsReported;
        deps_.interacted(slot, in.kind == Kind::PickUp || (in.kind == Kind::Activate && t.cls == TargetClass::Item), in.target);
    }
    return v;
}

Verdict IntentHost::dispatch(PlayerId slot, const Intent& in, const TargetInfo& t) {
    switch (in.kind) {
        case Kind::Activate: return doActivate(slot, in, t);
        case Kind::PickUp: return doPickUp(slot, in, t);
        case Kind::ContainerOpen: return doContainerOpen(slot, in, t);
        case Kind::TriggerEnter: return doEvent(slot, in, t, ev::OnTriggerEnter);
        case Kind::TriggerLeave: return doEvent(slot, in, t, ev::OnTriggerLeave);
        case Kind::TriggerStay: return doStay(slot, in, t);
        case Kind::StartCombat: return doEvent(slot, in, t, ev::OnStartCombat);
        case Kind::Hit: return doEvent(slot, in, t, ev::OnHit);
        case Kind::TerminalUse: return doTerminal(slot, in, t);
        case Kind::DialogueStart: {
            if (t.cls != TargetClass::Npc) return verdictOf(in, Result::BadTarget);
            if (t.dead || t.inCombat) return verdictOf(in, Result::Denied);
            if (deps_.dialogueStart) deps_.dialogueStart(in.target, slot);
            return verdictOf(in, Result::Executed);
        }
        default: return verdictOf(in, Result::BadTarget);
    }
}

// The stand-in retry runs a script's OnActivate a second time, with the host's own player in it. That is only safe for the scripts the generator listed (retry_list.h): the
// ones that test `GetActionRef == player` and do nothing that leaves a trace on the host's screen or outside the capture. Every other activator keeps the puppet's run alone.
bool IntentHost::retryAllowed(const TargetInfo& t) const {
    if (cfg_.requirePuppet || !cfg_.standIn) return false;
    if (FNV_MUT(424)) return t.cls == TargetClass::Activator;   // MUTANT 424: every activator is redone (a vending machine's message box pops on the host, a switch is thrown twice)
    switch (t.cls) {   // the list vouches for the script, whatever form the engine classed the plant as (a flora is no TESObjectACTI); a thing that opens a menu on the host is never redone
        case TargetClass::Container: case TargetClass::Terminal: case TargetClass::Npc: case TargetClass::Creature: case TargetClass::Item: return false;
        default: break;
    }
    return deps_.retryList && deps_.retryList->contains(t.script);
}

Verdict IntentHost::doActivate(PlayerId slot, const Intent& in, const TargetInfo& t) {
    TargetClass cls = t.cls;
    if ((cls == TargetClass::Npc || cls == TargetClass::Creature) && t.dead) cls = TargetClass::Container;
    if (cls == TargetClass::Item) return doPickUp(slot, in, t);          // a legacy ActivateRef on a world item is a claim
    if (cls == TargetClass::Container) return doContainerOpen(slot, in, t);
    if (cls == TargetClass::Npc || cls == TargetClass::Creature || cls == TargetClass::Static) return verdictOf(in, Result::BadTarget);   // talk: 0xD1
    Actor a;
    if (!pickActor(slot, Kind::Activate, cls, &a)) return verdictOf(in, Result::NoPuppet);
    if (Result r = checkReach(a, t, Kind::Activate); r != Result::Executed) return verdictOf(in, r);
    bool noEffect = false;
    Result r = runScoped(slot, a, Kind::Activate, in.target, [&](IActingScope& sc) {
        const uint64_t before = sc.recorded();
        const bool ok = deps_.eng->activate(a.ref, in.target);
        noEffect = ok && sc.recorded() == before;
        return ok ? Result::Executed : Result::Failed;
    });
    // A puppet is not `player` for the object's script: the Broc flower of Goodsprings is `if State == 0 && GetActionRef == player` and the puppet changed nothing at all
    // (story run 2026-10-28, S07: the guest got the XP of its own half but no ingredient). Once more with the host's own player standing in, an activator being something
    // that opens no menu; what that player gains belongs to the guest (the script's `player.AddItem` is captured with the guest as origin) and is taken back from the host.
    if (r == Result::Executed && noEffect && !a.standIn && cfg_.standInRetry && retryAllowed(t) && !FNV_MUT(419)) {   // MUTANT 419: a puppet's activation to no effect stays lost
        ++stats_.standInRetries;
        Actor s;
        s.ref = kHostPlayerRef;
        s.standIn = true;
        const bool watched = deps_.eng->markHostInventory();
        r = runScoped(slot, s, Kind::Activate, in.target, [&](IActingScope&) { return deps_.eng->activate(s.ref, in.target) ? Result::Executed : Result::Failed; });
        if (watched && !FNV_MUT(420)) stats_.standInGainsReverted += static_cast<uint64_t>(deps_.eng->revertHostInventory());   // MUTANT 420: the host keeps what the guest's activation gave "the player" (a duplicate)
    }
    return verdictOf(in, r);
}

Verdict IntentHost::doPickUp(PlayerId slot, const Intent& in, const TargetInfo& t) {
    if (t.disabled) { ++stats_.claimsLost; return verdictOf(in, Result::Taken); }
    if (t.base.plugin.empty() || t.count <= 0) return verdictOf(in, Result::BadTarget);   // nothing the grant could name
    Actor a;
    if (!pickActor(slot, Kind::PickUp, TargetClass::Item, &a)) return verdictOf(in, Result::NoPuppet);
    if (Result r = checkReach(a, t, Kind::PickUp); r != Result::Executed) return verdictOf(in, r);
    auto it = claims_.find(in.target);
    if (it != claims_.end() && !FNV_MUT(309)) {   // MUTANT 309: no claim table (two players both get the item)
        ++stats_.claimsLost;
        return verdictOf(in, Result::Taken);
    }
    claims_[in.target] = Claim{slot, now()};
    claimOrder_.push_back(in.target);
    while (claimOrder_.size() > cfg_.claimMemory) { claims_.erase(claimOrder_.front()); claimOrder_.pop_front(); }
    const bool note = t.note && !FNV_MUT(403);   // MUTANT 403: a note is granted as an inventory item (the pipboy never gets it, GetHasNote stays false)
    Result r = runScoped(slot, a, Kind::PickUp, in.target, [&](IActingScope& sc) {
        if (note) {
            // A note is a journal entry, not an inventory item (G10): the host's journal gains it (so GetHasNote on the host, which quest scripts poll, is true) and the
            // engine's AddNote hook records the shared AddNote effect into this scope, which reaches the guest and every other copy. First, so that a refusal leaves the
            // note lying in the world for the next try.
            if (!deps_.eng->addNote(t.base)) return Result::Failed;
            if (!deps_.eng->disableRef(in.target)) return Result::Failed;   // already in the journal; the AddNote is idempotent, the ref stays for a retry
            ++stats_.noteGrants;
        } else {
            if (!deps_.eng->disableRef(in.target)) return Result::Failed;   // it stays in the world for everybody
            fnvcoop::AddItem grant;
            grant.item = t.base;
            grant.count = t.count;
            grant.to = fnvcoop::Recipient::origin();   // the guest's inventory only, never the host's
            sc.record(grant);
        }
        raiseNow(a, in.target, ev::OnAdd);   // OnAdd blocks of the item (quest items) run now, with the guest as `player`
        return Result::Executed;
    });
    if (r == Result::Executed) {
        ++stats_.claimsWon;
    } else {
        claims_.erase(in.target);   // nothing happened: the item is free for the next try
        claimOrder_.erase(std::remove(claimOrder_.begin(), claimOrder_.end(), in.target), claimOrder_.end());
    }
    return verdictOf(in, r);
}

Verdict IntentHost::doContainerOpen(PlayerId slot, const Intent& in, const TargetInfo& t) {
    if (redirect_.isShared(in.target)) {
        FormRef mine;
        if (!redirect_.resolve(in.target, slot, &mine)) return verdictOf(in, Result::Denied);   // no storage for this player there
        Verdict v = verdictOf(in, Result::Executed);
        v.hasRedirect = true;
        v.redirect = mine;
        ++stats_.redirects;
        return v;
    }
    const uint64_t t0 = now();
    auto it = locks_.find(in.target);
    if (it != locks_.end() && t0 - it->second.refreshedMs > cfg_.lockLeaseMs) { locks_.erase(it); ++stats_.lockExpired; it = locks_.end(); }
    if (it != locks_.end() && it->second.slot != slot && !FNV_MUT(310)) {   // MUTANT 310: no in-use lock (two players in one chest, contents race)
        ++stats_.lockBusy;
        Verdict v = verdictOf(in, Result::Busy);
        v.holder = it->second.slot;
        return v;
    }
    Actor a;
    const bool have = pickActor(slot, Kind::ContainerOpen, t.cls, &a);
    if (have) {
        if (Result r = checkReach(a, t, Kind::ContainerOpen); r != Result::Executed) return verdictOf(in, r);
    }
    if (it == locks_.end()) { locks_[in.target] = Lock{slot, t0, t0}; ++stats_.lockGrants; }
    else it->second.refreshedMs = t0;
    if (have) {
        // the OnActivate half (scripted chests, trapped lids): the engine's Activate with the puppet as activator opens no menu for anybody
        runScoped(slot, a, Kind::ContainerOpen, in.target, [&](IActingScope&) { return deps_.eng->activate(a.ref, in.target) ? Result::Executed : Result::Failed; });
    } else {
        ++stats_.scriptsSkipped;
    }
    return verdictOf(in, Result::Executed);
}

Verdict IntentHost::doContainerClose(PlayerId slot, const Intent& in) {
    auto it = locks_.find(in.target);
    if (it != locks_.end() && it->second.slot == slot) { locks_.erase(it); ++stats_.lockReleased; }
    return verdictOf(in, Result::Executed);   // closing something you do not hold is not an error
}

Verdict IntentHost::doEvent(PlayerId slot, const Intent& in, const TargetInfo& t, uint32_t mask) {
    if (mask == ev::OnStartCombat) {
        if (t.cls != TargetClass::Npc && t.cls != TargetClass::Creature) return verdictOf(in, Result::BadTarget);
        if (t.dead && !FNV_MUT(408)) return verdictOf(in, Result::Denied);   // MUTANT 408: a dead actor "starts combat" (its blocks run on a corpse)
    }
    if (mask == ev::OnHit && (t.cls == TargetClass::Npc || t.cls == TargetClass::Creature) && !FNV_MUT(316))   // MUTANT 316: actor hits run as well (double OnHit)
        return verdictOf(in, Result::Denied);
    if (mask == ev::OnTriggerEnter || mask == ev::OnStartCombat) {
        auto key = std::make_pair(slot, in.target);
        auto it = lastTrigger_.find(key);
        if (it != lastTrigger_.end() && now() - it->second < cfg_.triggerCooldownMs && !FNV_MUT(317))   // MUTANT 317: no cooldown (standing in a volume re-fires)
            return verdictOf(in, Result::Duplicate);
    }
    Actor a;
    if (!pickActor(slot, in.kind, t.cls, &a)) return verdictOf(in, Result::NoPuppet);
    if (Result r = checkReach(a, t, in.kind); r != Result::Executed) return verdictOf(in, r);
    Result r = raiseFor(slot, a, in.kind, in.target, mask);
    if (r == Result::Executed && (mask == ev::OnTriggerEnter || mask == ev::OnStartCombat)) {
        if (lastTrigger_.size() > 2048) lastTrigger_.clear();
        lastTrigger_[std::make_pair(slot, in.target)] = now();
    }
    if (r == Result::Executed && mask == ev::OnTriggerEnter && cfg_.triggerStay) {   // from now on the host re-raises OnTrigger for this puppet every frame, like the engine does
        auto key = std::make_pair(slot, in.target);
        if (presence_.count(key) || presence_.size() < cfg_.stayMax) {
            if (!presence_.count(key)) ++stats_.stayStarted;
            presence_[key] = Presence{now(), 0};
        }
    }
    if (mask == ev::OnTriggerLeave && !FNV_MUT(407)) presence_.erase(std::make_pair(slot, in.target));   // MUTANT 407: a player who left keeps getting OnTrigger
    return verdictOf(in, r);
}

// The guest's per-frame OnTrigger events arrive as one TriggerStay per `stay_keepalive_ms`. The host does not run the block once per message: it keeps the
// (player, trigger) presence and raises OnTrigger for the puppet itself, every host frame (tick), until the guest says it left, stops refreshing (lease) or
// leaves the session. Nothing runs here but the bookkeeping.
Verdict IntentHost::doStay(PlayerId slot, const Intent& in, const TargetInfo& t) {
    auto key = std::make_pair(slot, in.target);
    auto it = presence_.find(key);
    if (it == presence_.end()) {
        if (presence_.size() >= cfg_.stayMax && !FNV_MUT(412)) {   // MUTANT 412: the table is unbounded (a flood makes the host raise thousands of events a frame)
            ++stats_.stayDropped;
            return verdictOf(in, Result::RateLimited);
        }
        Actor a;
        if (!pickActor(slot, Kind::TriggerStay, t.cls, &a)) return verdictOf(in, Result::NoPuppet);
        if (Result r = checkReach(a, t, Kind::TriggerStay); r != Result::Executed) return verdictOf(in, r);
        presence_[key] = Presence{now(), 0};
        ++stats_.stayStarted;
    } else {
        it->second.lastSeenMs = now();
    }
    return verdictOf(in, Result::Executed);
}

void IntentHost::raisePresence() {
    if (presence_.empty()) return;
    const uint64_t t = now();
    std::vector<std::pair<PlayerId, FormRef>> gone;
    for (auto& kv : presence_) {
        Presence& p = kv.second;
        if (t - p.lastSeenMs > cfg_.stayLeaseMs && !FNV_MUT(406)) { gone.push_back(kv.first); ++stats_.stayExpired; continue; }   // MUTANT 406: no lease (a guest whose Leave was lost is "inside" for ever)
        if (cfg_.stayRaiseMs && p.lastRaiseMs && t - p.lastRaiseMs < cfg_.stayRaiseMs && !FNV_MUT(409)) continue;   // MUTANT 409: the interval is ignored
        const PlayerId slot = kv.first.first;
        const FormRef target = kv.first.second;
        TargetInfo ti;
        if (!deps_.eng->describe(target, ti) || !ti.loaded || ti.disabled) { gone.push_back(kv.first); ++stats_.stayDropped; continue; }
        Actor a;
        if (!pickActor(slot, Kind::TriggerStay, ti.cls, &a)) { gone.push_back(kv.first); ++stats_.stayDropped; continue; }
        raiseFor(slot, a, Kind::TriggerStay, target, ev::OnTrigger, false);   // no stash swap: this runs every frame
        p.lastRaiseMs = t;
        ++stats_.stayRaises;
    }
    for (const auto& k : gone) presence_.erase(k);
}

Verdict IntentHost::doTerminal(PlayerId slot, const Intent& in, const TargetInfo& t) {
    if (t.cls != TargetClass::Terminal || in.path.empty() || in.path.size() > kMaxPath) return verdictOf(in, Result::BadTarget);
    Actor a;
    if (!pickActor(slot, Kind::TerminalUse, t.cls, &a)) return verdictOf(in, Result::NoPuppet);
    if (Result r = checkReach(a, t, Kind::TerminalUse); r != Result::Executed) return verdictOf(in, r);
    bool supported = true;
    Result r = runScoped(slot, a, Kind::TerminalUse, in.target, [&](IActingScope&) {
        bool ok = deps_.eng->runTerminalEntry(a.ref, in.target, in.path, &supported);
        return !supported ? Result::Unsupported : ok ? Result::Executed : Result::Failed;
    });
    return verdictOf(in, r);
}

// A guest's script or console tried to advance a quest. The guest applied nothing. The host judges it like the engine would judge its own script:
//   - the stage must be a real advance (the shadow merge is max and the engine's SetStage only raises); a request for a stage the quest already
//     reached is the host's own copy of the same script having got there first, and it must not run the stage's result scripts a second time;
//   - the host's rules get their say through the engine's stage gate (barrier / tether: Deferred, released by the host later) and through allowStage
//     (ExclusiveStages: a losing outcome is refused up front, Denied);
//   - the stage runs as the guest (acting player = their puppet), so rewards and journal entries route by origin like any other intent.
// Every refusal is counted and reported through HostDeps::stageRefused (the module writes a log line), never silent.
Verdict IntentHost::doSetStage(PlayerId slot, const Intent& in) {
    if (in.value <= 0 || in.value > 255) { ++stats_.stagesBad; return verdictOf(in, Result::BadTarget); }
    const uint32_t stage = static_cast<uint32_t>(in.value);
    int32_t cur = 0;
    if (!deps_.eng->questStage(in.target, &cur)) return verdictOf(in, Result::NotLoaded);
    auto refuse = [&](Result r) {
        if (deps_.stageRefused && !FNV_MUT(336)) deps_.stageRefused(slot, in.target, stage, r);   // MUTANT 336: a refused guest stage is dropped without a word
        return verdictOf(in, r);
    };
    if (cur >= static_cast<int32_t>(stage) && !FNV_MUT(333)) {   // MUTANT 333: a stage the quest already reached runs its result scripts again (double rewards)
        ++stats_.stagesNoAdvance;
        return refuse(Result::NoAdvance);
    }
    if (deps_.guestStagePass && !FNV_MUT(345)) {   // MUTANT 345: the rules engine is never told that a guest reached its barrier stage (the pass is lost, the stage runs alone)
        const StagePass pass = deps_.guestStagePass(slot, in.target, stage);
        if (pass == StagePass::Deferred && !FNV_MUT(346)) {   // MUTANT 346: a held pass stage is run anyway (the party is not waited for)
            ++stats_.stagesDeferred;
            return refuse(Result::Deferred);
        }
    }
    if (deps_.allowStage && !FNV_MUT(334) && !deps_.allowStage(slot, in.target, stage)) {   // MUTANT 334: the policy question is never asked
        ++stats_.stagesRefused;
        return refuse(Result::Denied);
    }
    Actor a;
    if (!pickActor(slot, Kind::SetStage, TargetClass::Unknown, &a)) return verdictOf(in, Result::NoPuppet);
    IIntentEngine::StageOutcome out = IIntentEngine::StageOutcome::Failed;
    Result r = runScoped(slot, a, Kind::SetStage, in.target, [&](IActingScope&) {
        out = deps_.eng->setStage(in.target, stage);
        return out == IIntentEngine::StageOutcome::Failed ? Result::Failed : Result::Executed;
    });
    if (r == Result::Executed && out == IIntentEngine::StageOutcome::Deferred && !FNV_MUT(335)) {   // MUTANT 335: a held stage is reported as done
        ++stats_.stagesDeferred;
        return refuse(Result::Deferred);
    }
    if (r == Result::Executed) { ++stats_.stagesApplied; return verdictOf(in, Result::Executed); }
    ++stats_.stagesFailed;
    return refuse(r);
}

void IntentHost::tick() {
    const uint64_t t = now();
    for (auto it = locks_.begin(); it != locks_.end();) {
        if (t - it->second.refreshedMs > cfg_.lockLeaseMs) { it = locks_.erase(it); ++stats_.lockExpired; }
        else ++it;
    }
    raisePresence();
}

void IntentHost::playerLeft(PlayerId slot) {
    if (!FNV_MUT(314)) {   // MUTANT 314: a leaver's locks stay until the lease runs out (the chest is dead for everyone for two minutes)
        for (auto it = locks_.begin(); it != locks_.end();) {
            if (it->second.slot == slot) { it = locks_.erase(it); ++stats_.lockReleased; }
            else ++it;
        }
    }
    slots_.erase(slot);
    lastMode_.erase(slot);
    for (auto it = presence_.begin(); it != presence_.end();) {
        if (it->first.first == slot) it = presence_.erase(it);
        else ++it;
    }
    for (auto it = lastTrigger_.begin(); it != lastTrigger_.end();) {
        if (it->first.first == slot) it = lastTrigger_.erase(it);
        else ++it;
    }
}

void IntentHost::worldReset() {
    claims_.clear();
    claimOrder_.clear();
    locks_.clear();
    lastTrigger_.clear();
    presence_.clear();
}

std::vector<IntentHost::LockInfo> IntentHost::locks() const {
    std::vector<LockInfo> v;
    for (const auto& kv : locks_) v.push_back({kv.first, kv.second.slot, kv.second.sinceMs});
    return v;
}

std::string IntentHost::statusJson() const {
    std::ostringstream o;
    o << "{\"role\":\"host\",\"received\":" << stats_.received << ",\"executed\":" << stats_.executed << ",\"legacy\":" << stats_.legacy
      << ",\"duplicates\":" << stats_.duplicates << ",\"badDatagrams\":" << stats_.badDatagrams << ",\"verdictsSent\":" << stats_.verdictsSent
      << ",\"verdictsFailed\":" << stats_.verdictsFailed << ",\"actedAs\":{\"puppet\":" << stats_.actedAsPuppet << ",\"ghost\":" << stats_.actedAsGhost
      << ",\"standIn\":" << stats_.actedAsStandIn << "},\"reach\":{\"checked\":" << stats_.reachChecked << ",\"unchecked\":" << stats_.reachUnchecked
      << "},\"stage\":{\"applied\":" << stats_.stagesApplied << ",\"noAdvance\":" << stats_.stagesNoAdvance << ",\"refused\":" << stats_.stagesRefused
      << ",\"deferred\":" << stats_.stagesDeferred << ",\"failed\":" << stats_.stagesFailed << ",\"bad\":" << stats_.stagesBad
      << "},\"stay\":{\"present\":" << presence_.size() << ",\"started\":" << stats_.stayStarted << ",\"raises\":" << stats_.stayRaises << ",\"expired\":" << stats_.stayExpired
      << ",\"dropped\":" << stats_.stayDropped << "},\"events\":{\"scriptRuns\":" << stats_.eventScriptRuns << ",\"scriptMissed\":" << stats_.eventScriptMissed << ",\"flushed\":" << stats_.eventsFlushed << "},\"notes\":" << stats_.noteGrants << ",\"stash\":{\"containers\":" << (stash_ ? stash_->entries().size() : 0u)
      << ",\"filed\":" << (stash_ ? stash_->stashes() : 0u) << ",\"swaps\":" << stats_.stashSwaps << ",\"kept\":" << stats_.stashKept << ",\"failed\":" << stats_.stashFailed
      << "},\"scriptsSkipped\":" << stats_.scriptsSkipped << ",\"claims\":{\"won\":" << stats_.claimsWon << ",\"lost\":" << stats_.claimsLost
      << ",\"held\":" << claims_.size() << "},\"locks\":{\"granted\":" << stats_.lockGrants << ",\"busy\":" << stats_.lockBusy << ",\"expired\":"
      << stats_.lockExpired << ",\"released\":" << stats_.lockReleased << ",\"held\":" << locks_.size() << "},\"redirects\":" << stats_.redirects
      << ",\"byKind\":{";
    bool first = true;
    for (size_t k = 1; k < static_cast<size_t>(Kind::Count_); ++k) {
        if (!stats_.byKind[k]) continue;
        o << (first ? "" : ",") << '"' << kindName(static_cast<Kind>(k)) << "\":" << stats_.byKind[k];
        first = false;
    }
    o << "},\"rejected\":{";
    first = true;
    for (size_t r = 2; r < static_cast<size_t>(Result::Count_); ++r) {
        if (!stats_.rejected[r]) continue;
        o << (first ? "" : ",") << '"' << resultName(static_cast<Result>(r)) << "\":" << stats_.rejected[r];
        first = false;
    }
    o << "}}";
    return o.str();
}

}  // namespace nvcoop::intents
