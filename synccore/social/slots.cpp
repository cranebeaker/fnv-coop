#include "social/slots.h"

#include <algorithm>
#include <cctype>
#include <cstdlib>
#include <sstream>

#include "../../tools/mut/fnvcoop_mut.h"

namespace fnvcoop::social {

namespace {
std::string lower(std::string s) {
    for (auto& c : s) c = static_cast<char>(std::tolower(static_cast<unsigned char>(c)));
    return s;
}
bool parseList(const std::string& tok, std::vector<FormRef>& out) {
    size_t pos = 0;
    while (pos < tok.size()) {
        const size_t comma = tok.find(',', pos);
        FormRef r;
        if (!parseSetRef(tok.substr(pos, comma == std::string::npos ? std::string::npos : comma - pos), r)) return false;
        out.push_back(r);
        pos = comma == std::string::npos ? tok.size() : comma + 1;
    }
    return !out.empty();
}
}  // namespace

bool parseSetRef(const std::string& tok, FormRef& out) {
    const size_t c = tok.rfind(':');
    if (c == std::string::npos || c == 0 || c + 1 >= tok.size()) return false;
    char* end = nullptr;
    const unsigned long v = std::strtoul(tok.c_str() + c + 1, &end, 16);
    if (!end || *end) return false;
    out = FormRef(lower(tok.substr(0, c)), static_cast<uint32_t>(v));
    return true;
}

SlotRules::LoadStats SlotRules::load(const std::string& text) {
    LoadStats st;
    std::istringstream in(text);
    std::string line;
    while (std::getline(in, line)) {
        if (!line.empty() && line.back() == '\r') line.pop_back();
        if (line.empty() || line[0] == '#') continue;
        std::istringstream ls(line);
        std::string kind;
        ls >> kind;
        if (kind == "group") {
            std::string name, refs;
            std::vector<FormRef> v;
            if (!(ls >> name >> refs) || !parseList(refs, v)) { ++st.bad; continue; }
            SlotGroup g;
            g.name = name;
            g.members.insert(v.begin(), v.end());
            addGroup(std::move(g));
            ++st.groups;
        } else if (kind == "party") {
            std::string refs;
            std::vector<FormRef> v;
            if (!(ls >> refs) || !parseList(refs, v)) { ++st.bad; continue; }
            party_.insert(v.begin(), v.end());
            ++st.party;
        } else if (kind == "view-party") {
            std::string edid;
            if (!(ls >> edid)) { ++st.bad; continue; }
            viewParty_.insert(lower(edid));
            ++st.viewParty;
        } else {
            ++st.bad;
        }
    }
    return st;
}

void SlotRules::addGroup(SlotGroup g) { groups_.push_back(std::move(g)); }

const SlotGroup* SlotRules::groupOf(const FormRef& npc) const {
    for (const SlotGroup& g : groups_) if (g.members.count(npc)) return &g;
    return nullptr;
}

bool SlotRules::viewsParty(const std::string& scriptEdid) const { return viewParty_.count(lower(scriptEdid)) != 0; }

PlayerId viewSlotFor(const SlotRules& rules, const std::string& scriptEdid, PlayerId acting) {
    if (rules.viewsParty(scriptEdid) && !FNV_MUT(900)) return kViewParty;   // BAD (900): the movers read the acting player's flags only, so a guest's companions stay behind
    return acting;
}

FormRef currentMember(const CompanionBook& book, const SlotRules& rules, const std::string& group, PlayerId owner) {
    for (const SlotGroup& g : rules.groups()) {
        if (g.name != group) continue;
        for (const FormRef& m : g.members) {
            const Companion* c = book.find(m);
            if (c && (c->owner == owner || FNV_MUT(899))) return m;   // BAD (899): another player's companion answers for this player
        }
    }
    return FormRef();
}

TxnVet vetCompanionTxn(const CompanionBook& book, const SlotRules& rules, const std::vector<Effect>& effects) {
    TxnVet out;
    std::vector<const CompanionOwner*> recruits, dismissals;
    for (const Effect& e : effects) {
        const auto* c = std::get_if<CompanionOwner>(&e);
        if (!c) continue;
        (c->active ? recruits : dismissals).push_back(c);
    }
    // 1. dismissals
    std::set<FormRef> revertedNpc;
    for (const CompanionOwner* d : dismissals) {
        const Companion* cur = book.find(d->npc);
        const SlotGroup* g = cur ? rules.groupOf(d->npc) : nullptr;
        bool casualty = false;
        if (g && !FNV_MUT(892)) {   // BAD (892): the swap guard is off, so one player's hire fires another player's companion
            for (const CompanionOwner* r : recruits) {
                if (r->npc == d->npc || rules.groupOf(r->npc) != g) continue;
                if (r->owner != cur->owner || FNV_MUT(894)) { casualty = true; break; }   // BAD (894): the player's own swap is reverted too
            }
        }
        if (casualty) {
            ++out.swapsReverted;
            revertedNpc.insert(d->npc);
            CompanionOwner fix;
            fix.npc = cur->npc; fix.owner = cur->owner; fix.active = true; fix.kind = cur->kind == CompanionKind::Critter ? 1 : 0;
            out.repairs.push_back(fix);
        } else {
            out.learn.push_back(*d);
        }
    }
    // 2. recruits
    for (const CompanionOwner* r : recruits) {
        const SlotGroup* g = rules.groupOf(r->npc);
        bool full = false;
        if (g && !FNV_MUT(893)) {   // BAD (893): the single slot is not enforced
            for (const FormRef& m : g->members) {
                if (m == r->npc) continue;
                const Companion* c = book.find(m);
                if (!c || c->owner != r->owner) continue;
                bool leaving = false;   // the same transaction lets that member go (the player's own swap): the slot is free again
                if (!FNV_MUT(895)) for (const CompanionOwner* d : dismissals) if (d->npc == m && !revertedNpc.count(m)) { leaving = true; break; }   // BAD (895): the player's own swap is refused
                if (!leaving) { full = true; break; }
            }
        }
        if (full) {
            ++out.slotDenied;
            CompanionOwner undo = *r;
            undo.active = false;
            out.repairs.push_back(undo);
        } else {
            out.learn.push_back(*r);
        }
    }
    return out;
}

}  // namespace fnvcoop::social
