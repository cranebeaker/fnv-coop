#include "social/companions.h"

#include <algorithm>

#include "../../tools/mut/fnvcoop_mut.h"

namespace fnvcoop::social {

namespace {

uint64_t fnv(uint64_t h, const void* p, size_t n) {
    const uint8_t* b = static_cast<const uint8_t*>(p);
    for (size_t i = 0; i < n; ++i) { h ^= b[i]; h *= 1099511628211ull; }
    return h;
}
uint64_t fnvU(uint64_t h, uint64_t v) { return fnv(h, &v, sizeof v); }
uint64_t fnvForm(uint64_t h, const FormRef& f) {
    h = fnv(h, f.plugin.data(), f.plugin.size());
    return fnvU(h, f.localId);
}

bool slotOk(PlayerId p) { return p < kMaxSlots; }

}  // namespace

const char* toString(RecruitReason r) {
    static const char* n[] = {"Ok", "AlreadyYours", "OwnedByOther", "OwnerCap", "PartyCap", "BadOwner"};
    return static_cast<size_t>(r) < sizeof n / sizeof *n ? n[static_cast<size_t>(r)] : "?";
}
const char* toString(CmdKind k) {
    static const char* n[] = {"?", "Wait", "Follow", "SetStance", "SetRange", "OpenWheel", "OpenInventory", "Equip", "Heal", "Talk", "Summon", "Dismiss"};
    return static_cast<size_t>(k) < sizeof n / sizeof *n ? n[static_cast<size_t>(k)] : "?";
}
const char* toString(CmdReason r) {
    static const char* n[] = {"Ok", "Duplicate", "NotACompanion", "NotOwner", "BadSender", "BadArg", "UnknownKind", "TooFast"};
    return static_cast<size_t>(r) < sizeof n / sizeof *n ? n[static_cast<size_t>(r)] : "?";
}
const char* toString(InvReason r) {
    static const char* n[] = {"Ok", "Duplicate", "NotACompanion", "NotOwner", "OutOfRange", "ZeroDelta"};
    return static_cast<size_t>(r) < sizeof n / sizeof *n ? n[static_cast<size_t>(r)] : "?";
}

// ---- wire ----------------------------------------------------------------------------------------------------------------------------
void io(Archive& a, CompanionState& s) {
    uint8_t m = static_cast<uint8_t>(s.mode), st = static_cast<uint8_t>(s.stance), r = static_cast<uint8_t>(s.range);
    a.u8(m); a.u8(st); a.u8(r);
    a.varu32(s.rev);
    if (a.reading()) {
        if (m > 1 || st > 2 || r > 2) { a.fail(); return; }
        s.mode = static_cast<FollowMode>(m); s.stance = static_cast<Stance>(st); s.range = static_cast<Range>(r);
    }
}
void io(Archive& a, Companion& c) {
    io(a, c.npc);
    a.u8(c.owner);
    uint8_t k = static_cast<uint8_t>(c.kind);
    a.u8(k);
    if (a.reading()) {
        if (k > 1 || c.owner >= kMaxSlots) { a.fail(); return; }
        c.kind = static_cast<CompanionKind>(k);
    }
    io(a, c.state);
    uint8_t flags = c.party ? 1 : 0;   // bit 0: party companion (G34)
    a.u8(flags);
    if (a.reading()) {
        if (flags > 1) { a.fail(); return; }
        c.party = flags == 1;
    }
}
void io(Archive& a, CompanionCmd& c) {
    a.u64(c.id);
    a.u8(c.from);
    io(a, c.npc);
    uint8_t k = static_cast<uint8_t>(c.kind);
    a.u8(k);
    a.u8(c.arg);
    if (a.reading()) {
        if (k == 0 || k >= static_cast<uint8_t>(CmdKind::Count_)) { a.fail(); return; }
        c.kind = static_cast<CmdKind>(k);
    }
}
void io(Archive& a, InvDelta& d) {
    a.u64(d.txn);
    io(a, d.npc);
    io(a, d.item);
    a.vari32(d.delta);
}
void io(Archive& a, InvFull& f) {
    io(a, f.npc);
    size_t n = f.items.size();
    if (!a.count(n, 512)) return;
    if (a.reading()) f.items.assign(n, {});
    for (auto& it : f.items) {
        io(a, it.first);
        a.vari32(it.second);
        if (a.reading() && (it.second == 0 || it.second > kMaxItemCount || it.second < -kMaxItemCount)) { a.fail(); return; }
    }
}

// ---- book ----------------------------------------------------------------------------------------------------------------------------
PlayerId CompanionBook::ownerOf(const FormRef& npc) const {
    auto it = list_.find(npc);
    return it == list_.end() ? kNoPlayer : it->second.owner;
}
const Companion* CompanionBook::find(const FormRef& npc) const {
    auto it = list_.find(npc);
    return it == list_.end() ? nullptr : &it->second;
}
std::vector<FormRef> CompanionBook::companionsOf(PlayerId owner) const {
    std::vector<FormRef> v;
    for (auto& kv : list_) if (kv.second.owner == owner) v.push_back(kv.first);
    return v;
}
size_t CompanionBook::countOf(PlayerId owner) const {
    size_t n = 0;
    for (auto& kv : list_) if (kv.second.owner == owner) ++n;
    return n;
}
size_t CompanionBook::countOf(PlayerId owner, CompanionKind kind) const {
    size_t n = 0;
    for (auto& kv : list_) if (kv.second.owner == owner && kv.second.kind == kind) ++n;
    return n;
}
size_t CompanionBook::capCountOf(PlayerId owner, CompanionKind kind) const {
    size_t n = 0;
    for (auto& kv : list_) if (kv.second.owner == owner && kv.second.kind == kind && (!kv.second.party || FNV_MUT(884))) ++n;   // BAD (884): a party companion uses its recruiter's follower slot
    return n;
}
size_t CompanionBook::capTotal() const {
    size_t n = 0;
    for (auto& kv : list_) if (!kv.second.party || FNV_MUT(884)) ++n;
    return n;
}
bool CompanionBook::overCap(PlayerId owner) const {
    if (FNV_MUT(828)) return countOf(owner) > static_cast<size_t>(caps_.humanoidPerOwner) + caps_.critterPerOwner + 100;   // BAD (828): kinds not counted apart
    return capCountOf(owner, CompanionKind::Humanoid) > caps_.humanoidPerOwner || capCountOf(owner, CompanionKind::Critter) > caps_.critterPerOwner;
}

bool CompanionBook::wantsParty(const FormRef& npc) const {
    if (fixedParty_.count(npc)) return true;
    return oracle_ && oracle_(npc);
}
std::vector<FormRef> CompanionBook::refreshParty() {
    std::vector<FormRef> changed;
    for (auto& kv : list_) {
        const bool want = wantsParty(kv.first);
        if (kv.second.party != want) { kv.second.party = want; changed.push_back(kv.first); }
    }
    return changed;
}
std::vector<FormRef> CompanionBook::handParty(PlayerId from, PlayerId to) {
    std::vector<FormRef> handed;
    if (FNV_MUT(889) || !slotOk(to)) return handed;   // BAD (889): the party companion keeps a recruiter who is gone
    for (auto& kv : list_)
        if (kv.second.party && kv.second.owner == from) { kv.second.owner = to; handed.push_back(kv.first); }
    return handed;
}

RecruitVerdict CompanionBook::check(const FormRef& npc, PlayerId owner, CompanionKind kind) const {
    if (!slotOk(owner)) return {false, RecruitReason::BadOwner};
    auto it = list_.find(npc);
    if (it != list_.end()) {
        if (it->second.owner == owner) return {true, RecruitReason::AlreadyYours};
        if (!FNV_MUT(803)) return {false, RecruitReason::OwnedByOther};   // BAD (803): a companion can be taken from its recruiter by a recruit
    }
    if ((wantsParty(npc) || (it != list_.end() && it->second.party)) && !FNV_MUT(884)) return {true, RecruitReason::Ok};   // a party companion is nobody's follower slot (BAD 884: it is)
    const size_t mine = capCountOf(owner, kind);
    const size_t cap = FNV_MUT(827) ? caps_.humanoidPerOwner : (kind == CompanionKind::Humanoid ? caps_.humanoidPerOwner : caps_.critterPerOwner);   // BAD (827): critters use the humanoid cap
    if (mine >= cap && !FNV_MUT(801)) return {false, RecruitReason::OwnerCap};                          // BAD (801): the per-owner cap is not checked
    if (capTotal() - (it != list_.end() && !it->second.party ? 1 : 0) >= caps_.partyTotal && !FNV_MUT(802)) return {false, RecruitReason::PartyCap};   // BAD (802): no party cap
    return {true, RecruitReason::Ok};
}

RecruitVerdict CompanionBook::recruit(const FormRef& npc, PlayerId owner, CompanionKind kind) {
    RecruitVerdict v = check(npc, owner, kind);
    if (!v.ok || v.reason == RecruitReason::AlreadyYours) return v;
    Companion c;
    auto it = list_.find(npc);
    if (it != list_.end()) c = it->second;   // taken over (only with the 803 twin): keep the state
    c.npc = npc; c.owner = owner; c.kind = kind; c.party = wantsParty(npc);
    list_[npc] = c;
    return v;
}

CompanionBook::Change CompanionBook::applyEffect(const Effect& e) {
    Change ch;
    const auto* c = std::get_if<CompanionOwner>(&e);
    if (!c) return ch;
    ch.npc = c->npc;
    auto it = list_.find(c->npc);
    if (c->active) {
        if (c->owner >= kMaxSlots) return ch;
        const CompanionKind kind = c->kind == 1 ? CompanionKind::Critter : CompanionKind::Humanoid;
        if (it != list_.end()) {
            ch.from = it->second.owner;
            if (it->second.owner == c->owner && it->second.kind == kind) return ch;   // idempotent
            if (FNV_MUT(805)) { it->second.kind = kind; return ch; }                  // BAD (805): a recruit by another player does not take the NPC over
            it->second.owner = c->owner; it->second.kind = kind; it->second.party = wantsParty(c->npc);
            it->second.state = CompanionState{};   // a new recruiter starts from the default behaviour
        } else {
            Companion n;
            n.npc = c->npc; n.owner = c->owner; n.kind = kind; n.party = wantsParty(c->npc);
            list_[c->npc] = n;
        }
        ch.changed = true; ch.to = c->owner;
    } else {
        if (it == list_.end()) return ch;
        if (FNV_MUT(804)) return ch;   // BAD (804): a dismiss effect is ignored
        ch.from = it->second.owner; ch.to = kNoPlayer; ch.changed = true;
        list_.erase(it);
    }
    return ch;
}

bool CompanionBook::dismiss(const FormRef& npc, PlayerId by, PlayerId host) {
    auto it = list_.find(npc);
    if (it == list_.end()) return false;
    if (it->second.owner != by && !(it->second.party && !FNV_MUT(891)) && by != host && !FNV_MUT(807)) return false;   // BAD (807): anybody can dismiss anybody's companion (BAD 891: only the recruiter may dismiss a party companion)
    list_.erase(it);
    return true;
}

std::vector<FormRef> CompanionBook::playerLeft(PlayerId p) {
    std::vector<FormRef> gone;
    if (FNV_MUT(806)) return gone;   // BAD (806): the companions of a leaver stay flagged as theirs
    for (auto it = list_.begin(); it != list_.end();) {
        if (it->second.owner == p && (!it->second.party || FNV_MUT(888))) { gone.push_back(it->first); it = list_.erase(it); }   // BAD (888): the party companion goes with its recruiter (ED-E vanishes from the book)
        else ++it;
    }
    return gone;
}

bool CompanionBook::setState(const FormRef& npc, const CompanionState& s) {
    auto it = list_.find(npc);
    if (it == list_.end()) return false;
    if (s.rev <= it->second.state.rev && !FNV_MUT(808)) return false;   // BAD (808): a stale revision overwrites a newer one
    it->second.state = s;
    return true;
}

uint64_t CompanionBook::digest() const {
    uint64_t h = 1469598103934665603ull;
    for (auto& kv : list_) {   // std::map order = deterministic
        h = fnvForm(h, kv.first);
        if (!FNV_MUT(825)) h = fnvU(h, kv.second.owner);   // BAD (825): the owner is not part of the digest
        h = fnvU(h, static_cast<uint64_t>(kv.second.kind) | (kv.second.party && !FNV_MUT(890) ? 0x100u : 0u));   // BAD (890): the party flag is not part of the digest
        h = fnvU(h, static_cast<uint64_t>(kv.second.state.mode) | (static_cast<uint64_t>(kv.second.state.stance) << 8) |
                        (static_cast<uint64_t>(kv.second.state.range) << 16) | (static_cast<uint64_t>(kv.second.state.rev) << 24));
    }
    return h;
}

std::vector<Companion> CompanionBook::roster() const {
    std::vector<Companion> v;
    for (auto& kv : list_) v.push_back(kv.second);
    return v;
}

void CompanionBook::replaceRoster(const std::vector<Companion>& all) {
    if (!FNV_MUT(826)) list_.clear();   // BAD (826): a resync merges into the old table, so a dismissed companion survives it
    for (const Companion& c : all) if (slotOk(c.owner)) list_[c.npc] = c;
}

// ---- commands ----------------------------------------------------------------------------------------------------------------------------
CmdVerdict CommandJudge::judge(const CompanionCmd& c, PlayerId sender, uint64_t nowMs, const CompanionBook& book) {
    CmdVerdict v;
    auto no = [&](CmdReason r) { v.apply = false; v.reason = r; return v; };
    if (c.kind == CmdKind::Count_ || static_cast<uint8_t>(c.kind) == 0 || static_cast<uint8_t>(c.kind) >= static_cast<uint8_t>(CmdKind::Count_)) return no(CmdReason::UnknownKind);
    if (c.from != sender && !FNV_MUT(810)) return no(CmdReason::BadSender);   // BAD (810): the message may claim another issuer
    if (ids_.count(c.id) && !FNV_MUT(811)) return no(CmdReason::Duplicate);   // BAD (811): a replayed command runs again
    const Companion* comp = book.find(c.npc);
    if (!comp) return no(CmdReason::NotACompanion);
    if (comp->owner != sender && !(comp->party && !FNV_MUT(885)) && !FNV_MUT(809)) return no(CmdReason::NotOwner);   // (BAD 885: anybody but the recruiter is refused for a party companion)   // BAD (809): commands from a non-owner are obeyed
    if (c.kind == CmdKind::SetStance && c.arg > 2) return no(CmdReason::BadArg);
    if (c.kind == CmdKind::SetRange && c.arg > 2) return no(CmdReason::BadArg);
    auto last = last_.find(sender);
    if (!FNV_MUT(812) && last != last_.end() && nowMs >= last->second && nowMs - last->second < rules_.minGapMs) return no(CmdReason::TooFast);   // BAD (812): no rate limit
    // accepted: remember it (a refused command does not spend the id, so a legitimate retry still works)
    ids_.insert(c.id); order_.push_back(c.id);
    while (order_.size() > rules_.remember) { ids_.erase(order_.front()); order_.erase(order_.begin()); }
    last_[sender] = nowMs;
    v.apply = true;
    v.reason = CmdReason::Ok;
    v.newState = comp->state;
    switch (c.kind) {
        case CmdKind::Wait: v.newState.mode = FollowMode::Wait; break;
        case CmdKind::Follow: v.newState.mode = FollowMode::Follow; break;
        case CmdKind::SetStance: v.newState.stance = static_cast<Stance>(c.arg); break;
        case CmdKind::SetRange: v.newState.range = static_cast<Range>(c.arg); break;
        case CmdKind::Dismiss: if (!FNV_MUT(813)) v.dismiss = true; break;   // BAD (813): a dismiss command does not dismiss
        default: break;
    }
    v.stateChanged = changesState(c.kind) && !(v.newState.mode == comp->state.mode && v.newState.stance == comp->state.stance && v.newState.range == comp->state.range);
    if (v.stateChanged && !FNV_MUT(814)) v.newState.rev = comp->state.rev + 1;   // BAD (814): a state change keeps its revision, so mirrors ignore it
    return v;
}

ActivationVerdict gateActivation(const CompanionBook& book, PlayerId who, const FormRef& npc, ForeignPolicy policy) {
    ActivationVerdict v;
    const Companion* c = book.find(npc);
    if (!c) return v;
    v.owner = c->owner;
    if (c->owner == who) { v.action = ActivateAction::OwnerOpens; return v; }
    if (c->party && !FNV_MUT(886)) { v.action = ActivateAction::PartyOpens; return v; }   // BAD (886): the wheel of a party companion opens for its recruiter only
    if (policy == ForeignPolicy::Allow || FNV_MUT(815)) { v.action = ActivateAction::AllowForeign; return v; }   // BAD (815): the wheel opens for anybody
    v.action = ActivateAction::DenyForeign;
    return v;
}

// ---- inventory ---------------------------------------------------------------------------------------------------------------------------
bool CompanionInventory::remember(uint64_t id) {
    if (ids_.count(id)) return false;
    ids_.insert(id); order_.push_back(id);
    while (order_.size() > cap_) { ids_.erase(order_.front()); order_.erase(order_.begin()); }
    return true;
}

std::vector<InvDelta> CompanionInventory::sample(const FormRef& npc, const ItemCounts& now, PlayerId owner, uint64_t& counter) {
    std::vector<InvDelta> out;
    ItemCounts clean;
    for (auto& kv : now) if (kv.second != 0) clean[kv.first] = kv.second;
    auto it = last_.find(npc);
    if (it == last_.end()) {
        last_[npc] = clean;
        if (FNV_MUT(819)) {   // BAD (819): the first sample is treated as a change from empty, so the whole inventory is added a second time
            for (auto& kv : clean) { InvDelta d; d.txn = (static_cast<uint64_t>(owner) << 48) | ++counter; d.npc = npc; d.item = kv.first; d.delta = kv.second; out.push_back(d); }
        }
        return out;
    }
    ItemCounts& before = it->second;
    for (auto& kv : clean) {
        auto b = before.find(kv.first);
        const int32_t was = b == before.end() ? 0 : b->second;
        if (kv.second != was) { InvDelta d; d.txn = (static_cast<uint64_t>(owner) << 48) | ++counter; d.npc = npc; d.item = kv.first; d.delta = kv.second - was; out.push_back(d); }
    }
    for (auto& kv : before) {
        if (clean.count(kv.first) == 0) { InvDelta d; d.txn = (static_cast<uint64_t>(owner) << 48) | ++counter; d.npc = npc; d.item = kv.first; d.delta = -kv.second; out.push_back(d); }
    }
    before = clean;
    return out;
}

InvVerdict CompanionInventory::judge(const InvDelta& d, PlayerId sender, const CompanionBook& book) {
    InvVerdict v;
    const Companion* c = book.find(d.npc);
    if (!c) { v.reason = InvReason::NotACompanion; return v; }
    if (d.delta == 0) { v.reason = InvReason::ZeroDelta; return v; }
    if (c->owner != sender && !(c->party && !FNV_MUT(887)) && !FNV_MUT(816)) { v.reason = InvReason::NotOwner; return v; }   // (BAD 887: a party companion's inventory takes its recruiter's changes only)   // BAD (816): anybody can change anybody's companion's inventory
    if (ids_.count(d.txn) && !FNV_MUT(817)) { v.reason = InvReason::Duplicate; return v; }   // BAD (817): a replayed delta applies again
    const int64_t result = static_cast<int64_t>(count(d.npc, d.item)) + d.delta;
    if ((result > kMaxItemCount || result < -static_cast<int64_t>(kMaxItemCount)) && !FNV_MUT(818)) { v.reason = InvReason::OutOfRange; v.resync = true; return v; }   // BAD (818): a count can run away without bound
    remember(d.txn);
    v.apply = true;
    return v;
}

int32_t CompanionInventory::apply(const InvDelta& d) {
    ItemCounts& m = model_[d.npc];
    int32_t& n = m[d.item];
    n += d.delta;
    if (n == 0) { m.erase(d.item); if (m.empty()) model_.erase(d.npc); return 0; }
    return n;
}

std::vector<InvDelta> CompanionInventory::replace(const InvFull& f, PlayerId asOwner, uint64_t& counter) {
    std::vector<InvDelta> out;
    ItemCounts now;
    for (auto& it : f.items) if (it.second != 0) now[it.first] += it.second;
    ItemCounts old;
    auto mi = model_.find(f.npc);
    if (mi != model_.end()) old = mi->second;
    for (auto& kv : now) {
        const int32_t was = old.count(kv.first) ? old[kv.first] : 0;
        if (kv.second != was) { InvDelta d; d.txn = (static_cast<uint64_t>(asOwner) << 48) | ++counter; d.npc = f.npc; d.item = kv.first; d.delta = kv.second - was; out.push_back(d); }
    }
    if (!FNV_MUT(829)) {   // BAD (829): items the owner no longer has are never removed from a mirror
        for (auto& kv : old) if (!now.count(kv.first)) { InvDelta d; d.txn = (static_cast<uint64_t>(asOwner) << 48) | ++counter; d.npc = f.npc; d.item = kv.first; d.delta = -kv.second; out.push_back(d); }
    }
    if (now.empty()) model_.erase(f.npc); else model_[f.npc] = now;
    last_[f.npc] = now;
    return out;
}

InvFull CompanionInventory::fullOf(const FormRef& npc) const {
    InvFull f;
    f.npc = npc;
    auto it = model_.find(npc);
    if (it != model_.end()) for (auto& kv : it->second) f.items.push_back(kv);
    return f;
}
int32_t CompanionInventory::count(const FormRef& npc, const FormRef& item) const {
    auto it = model_.find(npc);
    if (it == model_.end()) return 0;
    auto j = it->second.find(item);
    return j == it->second.end() ? 0 : j->second;
}
uint64_t CompanionInventory::digestOf(const FormRef& npc) const {
    uint64_t h = 1469598103934665603ull;
    auto it = model_.find(npc);
    if (it == model_.end()) return h;
    for (auto& kv : it->second) { h = fnvForm(h, kv.first); h = fnvU(h, static_cast<uint64_t>(kv.second)); }
    return h;
}

// ---- teammates ---------------------------------------------------------------------------------------------------------------------------
TeammateFix reconcileTeammates(const CompanionBook& book, PlayerId self, const std::set<FormRef>& flagged, const std::set<FormRef>& loaded) {
    TeammateFix fix;
    for (const FormRef& n : flagged) {
        if (book.ownerOf(n) != self && !FNV_MUT(820)) fix.unset.push_back(n);   // BAD (820): a foreign companion stays flagged here (it would follow the wrong player)
    }
    for (const Companion& c : book.roster()) {
        if (c.owner != self || FNV_MUT(821)) continue;   // BAD (821): my own companions are never flagged here
        if (flagged.count(c.npc) == 0 && loaded.count(c.npc) != 0) fix.set.push_back(c.npc);
    }
    return fix;
}

// ---- the party ---------------------------------------------------------------------------------------------------------------------------
HitPolicy companionHitPolicy(const Combatant& attacker, const Combatant& victim, bool friendlyFire) {
    if (friendlyFire) return HitPolicy::Allow;
    if (!attacker.party() || !victim.party()) return HitPolicy::Allow;
    // inside the party
    if (attacker.kind == Combatant::Player && victim.kind == Combatant::Companion && FNV_MUT(822)) return HitPolicy::Allow;      // BAD (822): players may shoot companions
    if (attacker.kind == Combatant::Companion && victim.kind == Combatant::Companion && FNV_MUT(823)) return HitPolicy::Allow;   // BAD (823): companions fight each other
    return HitPolicy::Neutralize;
}

bool vatsTargetAllowed(const Combatant& target) {
    if (FNV_MUT(824)) return true;   // BAD (824): party members can be picked as VATS targets
    return !target.party();
}

}  // namespace fnvcoop::social
