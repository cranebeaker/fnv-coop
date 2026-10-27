#include "fnvcoop/policy.h"

#include <cstdlib>
#include <sstream>
#include <vector>
#include "fnvcoop/mutations.h"
#include "fnvcoop/refop.h"
#include "fnvcoop/wire.h"

namespace fnvcoop {

VolatileKeys& volatileKeys() {
    static VolatileKeys k;
    return k;
}

uint64_t VolatileKeys::contentHash() const {
    uint64_t h = 0;   // sets are ordered, so a straight fold is deterministic
    auto fold = [&](const std::string& s) { h = hash64(s.data(), s.size(), h ^ 0x9E3779B97F4A7C15ull); };
    for (auto& [f, i] : vars_) if (!fromName_.count({f, i})) fold("v" + f.plugin + ":" + std::to_string(f.localId) + ":" + std::to_string(i));
    for (auto& f : owners_) fold("o" + f.plugin + ":" + std::to_string(f.localId));
    for (auto& f : globals_) fold("g" + f.plugin + ":" + std::to_string(f.localId));
    for (auto& [f, n] : declaredNames_) fold("n" + f.plugin + ":" + std::to_string(f.localId) + ":" + n);
    for (auto& [f, k] : fields_) fold("f" + f.plugin + ":" + std::to_string(f.localId) + ":" + std::to_string(k));
    for (auto& f : factions_) fold("r" + f.plugin + ":" + std::to_string(f.localId));
    return h;
}

bool VolatileKeys::isVolatileVar(const FormRef& owner, uint16_t index) const {
    if (mutations().ignoreVolatile) return false;
    return owners_.count(owner) || vars_.count({owner, index});
}
bool VolatileKeys::isVolatileGlobal(const FormRef& g) const {
    return !mutations().ignoreVolatile && globals_.count(g) > 0;
}

namespace {
bool parseForm(const std::vector<std::string>& t, size_t& pos, FormRef& out) {
    if (pos >= t.size()) return false;
    std::string plugin, hex;
    auto colon = t[pos].rfind(':');
    if (colon != std::string::npos) { plugin = t[pos].substr(0, colon); hex = t[pos].substr(colon + 1); ++pos; }
    else { if (pos + 1 >= t.size()) return false; plugin = t[pos]; hex = t[pos + 1]; pos += 2; }
    if (plugin.empty() || hex.empty()) return false;
    char* end = nullptr;
    unsigned long id = std::strtoul(hex.c_str(), &end, 16);
    if (!end || *end != 0) return false;
    out = FormRef::make(plugin, static_cast<uint32_t>(id));
    return true;
}
}  // namespace

bool VolatileKeys::isVolatile(const Effect& e) const {
    if (mutations().ignoreVolatile) return false;
    if (auto* v = std::get_if<SetScriptVar>(&e)) return isVolatileVar(v->owner, v->index);
    // an object / NPC script var is per-machine on the same terms as a quest var (`var <ref> <index>` or `owner <ref>`): before this a
    // timer on a ref (RefScriptVar 171CDC var 1) was baselined and replicated although each machine counts its own
    if (auto* r = std::get_if<RefScriptVar>(&e)) return isVolatileVar(r->ref, r->index);
    if (auto* g = std::get_if<SetGlobal>(&e)) return isVolatileGlobal(g->global);
    if (auto* x = std::get_if<StartQuest>(&e)) return fields_.count({x->quest, kFieldRunning}) > 0;
    if (auto* x = std::get_if<StopQuest>(&e)) return fields_.count({x->quest, kFieldRunning}) > 0;
    if (auto* x = std::get_if<SetStage>(&e)) return fields_.count({x->quest, kFieldStage}) > 0;
    if (auto* x = std::get_if<SetObjective>(&e)) return fields_.count({x->quest, kFieldObjectives}) > 0;
    // a DLC setup script (GuestLocalRun) ranks each machine's own player in the DLC's player faction: per-machine by definition
    if (auto* x = std::get_if<SetFactionRank>(&e)) return factions_.count(x->faction) > 0;
    return false;
}

size_t VolatileKeys::resolveNames(const std::function<bool(const FormRef& owner, const std::string& name, uint16_t& index)>& resolver) {
    size_t resolved = 0;
    for (auto it = names_.begin(); it != names_.end();) {
        uint16_t idx = 0;
        if (resolver(it->first, it->second, idx)) { vars_.insert({it->first, idx}); fromName_.insert({it->first, idx}); it = names_.erase(it); ++resolved; }
        else ++it;
    }
    return resolved;
}

bool VolatileKeys::loadFromText(const std::string& text, std::string* err) {
    std::istringstream in(text);
    std::string line;
    int n = 0;
    VolatileKeys tmp;
    auto fail = [&](const std::string& why) {
        if (err) *err = "line " + std::to_string(n) + ": " + why + ": " + line;
        return false;
    };
    while (std::getline(in, line)) {
        ++n;
        auto hash = line.find('#');
        if (hash != std::string::npos) line.resize(hash);
        std::istringstream ls(line);
        std::vector<std::string> t;
        for (std::string w; ls >> w;) t.push_back(w);
        if (t.empty()) continue;
        const std::string& kind = t[0];
        size_t pos = 1;
        FormRef f;
        if (!parseForm(t, pos, f)) return fail("bad form");
        if (kind == "owner" || kind == "global" || kind == "faction") {
            if (pos != t.size()) return fail("trailing tokens");
            if (kind == "owner") tmp.addOwner(f); else if (kind == "global") tmp.addGlobal(f); else tmp.addFaction(f);
        } else if (kind == "var") {
            if (pos + 1 != t.size()) return fail("expected one index or name");
            const std::string& v = t[pos];
            char* end = nullptr;
            unsigned long idx = std::strtoul(v.c_str(), &end, 10);
            if (end && *end == 0) {   // numeric index
                if (idx > 0xFFFF) return fail("index out of range");
                tmp.addVar(f, static_cast<uint16_t>(idx));
            } else {                  // variable name: needs resolveNames() once forms are loaded
                tmp.names_.insert({f, v});
            }
        } else if (kind == "field") {
            if (pos + 1 != t.size()) return fail("expected field name");
            const std::string& v = t[pos];
            uint8_t fld = v == "running" ? kFieldRunning : v == "stage" ? kFieldStage : v == "objectives" ? kFieldObjectives : 0;
            if (!fld) return fail("unknown field");
            tmp.fields_.insert({f, fld});
        } else {
            return fail("unknown entry kind");
        }
    }
    vars_.insert(tmp.vars_.begin(), tmp.vars_.end());
    owners_.insert(tmp.owners_.begin(), tmp.owners_.end());
    globals_.insert(tmp.globals_.begin(), tmp.globals_.end());
    names_.insert(tmp.names_.begin(), tmp.names_.end());
    declaredNames_.insert(tmp.names_.begin(), tmp.names_.end());
    fields_.insert(tmp.fields_.begin(), tmp.fields_.end());
    factions_.insert(tmp.factions_.begin(), tmp.factions_.end());
    return true;
}

bool isQuestRewardCause(CauseKind k) { return k == CauseKind::QuestResult || k == CauseKind::DialogueResult; }

uint8_t recipientMask(const Recipient& r, PlayerId origin) {
    switch (r.mode) {
        case Recipient::Mask: return r.mask;
        case Recipient::All: return 0xFF;
        case Recipient::Origin:
        case Recipient::Default:
        default: return origin < 8 ? static_cast<uint8_t>(1u << origin) : 0;
    }
}

namespace {
// Resolves `r` to an explicit mask under the policy. `shared` = the category's default is "everyone".
void resolve(Recipient& r, bool sharedByDefault, PlayerId origin, const SharePolicy& p) {
    if (r.mode == Recipient::Default) r.mode = sharedByDefault ? Recipient::All : Recipient::Origin;
    uint8_t m = 0;
    switch (r.mode) {
        case Recipient::All: m = p.allMask(); break;
        case Recipient::Origin: m = static_cast<uint8_t>(1u << origin); break;
        default: m = static_cast<uint8_t>(r.mask & p.allMask()); break;
    }
    r = Recipient::slots(m);
}
}  // namespace

size_t normalizeTxn(EffectTxn& txn, const SharePolicy& p) {
    if (txn.origin >= p.maxPlayers) return kNormalizeInvalid;
    const bool reward = isQuestRewardCause(txn.cause.kind) && p.questRewardsToAll;
    size_t dropped = 0;
    std::vector<Effect> kept;
    kept.reserve(txn.effects.size());
    for (auto& e : txn.effects) {
        bool keep = true;
        std::visit([&](auto& x) {
            using T = std::decay_t<decltype(x)>;
            if constexpr (std::is_same_v<T, SetStage> || std::is_same_v<T, SetObjective> ||
                          std::is_same_v<T, StartQuest> || std::is_same_v<T, StopQuest>) {
                keep = p.journal && !volatileKeys().isVolatile(Effect(x));
            } else if constexpr (std::is_same_v<T, SetScriptVar>) {
                keep = p.journal && !volatileKeys().isVolatileVar(x.owner, x.index);
            } else if constexpr (std::is_same_v<T, RefScriptVar>) {
                keep = !volatileKeys().isVolatileVar(x.ref, x.index);
            } else if constexpr (std::is_same_v<T, CompleteQuest> || std::is_same_v<T, FailQuest> || std::is_same_v<T, CompleteAllObjectives>) {
                keep = p.journal;
            } else if constexpr (std::is_same_v<T, SetPlayerControls>) {
                resolve(x.to, false, txn.origin, p);  // initiator unless addressed
                keep = x.to.mask != 0;
            } else if constexpr (std::is_same_v<T, AddTopic> || std::is_same_v<T, AddNote> || std::is_same_v<T, RemoveNote> ||
                                 std::is_same_v<T, SetMapMarker>) {
                keep = p.journal;   // journal topics / notes / map markers are shared
            } else if constexpr (std::is_same_v<T, ForceActiveQuest>) {
                resolve(x.to, mutations().activeQuestShared, txn.origin, p);   // UI focus is per player: the initiator unless addressed
                keep = x.to.mask != 0;
            } else if constexpr (std::is_same_v<T, SetPerk>) {
                resolve(x.to, reward, txn.origin, p);
                keep = x.to.mask != 0;
            } else if constexpr (std::is_same_v<T, SetGlobal>) {
                keep = p.globals && !volatileKeys().isVolatileGlobal(x.global);
            } else if constexpr (std::is_same_v<T, KillActor> || std::is_same_v<T, ResurrectActor>) {
                keep = p.kills;
            } else if constexpr (std::is_same_v<T, AddItem> || std::is_same_v<T, RemoveItem> ||
                                 std::is_same_v<T, ModActorValue>) {
                // Payment: a removal in a topic result is the speaker's cost (the initiator pays), while its rewards go to the party.
                bool shared = reward;
                if constexpr (std::is_same_v<T, RemoveItem>)
                    if (txn.cause.kind == CauseKind::DialogueResult && !mutations().paymentSharesAll) shared = false;
                resolve(x.to, shared, txn.origin, p);
                keep = x.to.mask != 0;
            } else if constexpr (std::is_same_v<T, RefCommand>) {
                // The capture addresses a non-player target (an actor, a container, a list, a quest) to All and the character-build calls to Origin; what
                // it leaves at Default is a command on the PLAYER (a spell, a conditioned item ...), which is a reward like AddItem: shared only when the
                // cause is a quest reward, otherwise the acting player's alone.
                resolve(x.to, reward || mutations().refCmdPlayerShared, txn.origin, p);
                keep = x.to.mask != 0 && refOpKnown(x.op);
            } else if constexpr (std::is_same_v<T, ContainerTransfer>) {
                // A quest's inventory stash and its hand-back apply to the whole party: each machine moves its own player's items. A
                // PerPlayerScene (character creation) narrows it to the acting player in the rules engine.
                resolve(x.to, !mutations().stashOriginOnly, txn.origin, p);
                keep = x.to.mask != 0;
            } else if constexpr (std::is_same_v<T, ModKarma>) {
                resolve(x.to, p.karma, txn.origin, p);
                keep = x.to.mask != 0;
            } else if constexpr (std::is_same_v<T, ModReputation>) {
                resolve(x.to, p.reputation, txn.origin, p);
                keep = x.to.mask != 0;
            }
        }, e);
        if (keep) kept.push_back(std::move(e)); else ++dropped;
    }
    txn.effects = std::move(kept);
    return dropped;
}

}  // namespace fnvcoop
