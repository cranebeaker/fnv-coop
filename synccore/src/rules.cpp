#include "fnvcoop/rules.h"
#include <tuple>
#include "fnvcoop/wire.h"
#include <algorithm>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <fstream>
#include <limits>
#include <sstream>

namespace fnvcoop {

namespace {

// ---- minimal strict JSON (objects, arrays, strings, numbers, true/false/null) --------------------------------------
struct J {
    enum T { Null, Bool, Num, Str, Arr, Obj } t = Null;
    bool b = false;
    double n = 0;
    std::string s;
    std::vector<J> a;
    std::vector<std::pair<std::string, J>> o;
    const J* get(const std::string& k) const {
        for (auto& kv : o) if (kv.first == k) return &kv.second;
        return nullptr;
    }
};

class Parser {
public:
    Parser(const std::string& s) : s_(s) {}
    bool parse(J& out, std::string* err) {
        ws();
        if (!value(out, 0)) { fail(err); return false; }
        ws();
        if (p_ != s_.size()) { why_ = "trailing data"; fail(err); return false; }
        return true;
    }
private:
    const std::string& s_;
    size_t p_ = 0;
    std::string why_;
    void fail(std::string* err) { if (err) *err = "json: " + (why_.empty() ? std::string("syntax error") : why_) + " at byte " + std::to_string(p_); }
    void ws() { while (p_ < s_.size() && std::isspace(static_cast<unsigned char>(s_[p_]))) ++p_; }
    bool lit(const char* w) {
        size_t n = std::char_traits<char>::length(w);
        if (s_.compare(p_, n, w) != 0) return false;
        p_ += n;
        return true;
    }
    bool str(std::string& out) {
        if (p_ >= s_.size() || s_[p_] != '"') return false;
        ++p_;
        out.clear();
        while (p_ < s_.size()) {
            char c = s_[p_++];
            if (c == '"') return true;
            if (static_cast<unsigned char>(c) < 0x20) { why_ = "control char in string"; return false; }
            if (c != '\\') { out.push_back(c); continue; }
            if (p_ >= s_.size()) return false;
            char e = s_[p_++];
            switch (e) {
                case '"': out.push_back('"'); break;
                case '\\': out.push_back('\\'); break;
                case '/': out.push_back('/'); break;
                case 'b': out.push_back('\b'); break;
                case 'f': out.push_back('\f'); break;
                case 'n': out.push_back('\n'); break;
                case 'r': out.push_back('\r'); break;
                case 't': out.push_back('\t'); break;
                case 'u': {
                    if (p_ + 4 > s_.size()) return false;
                    unsigned cp = 0;
                    for (int i = 0; i < 4; ++i) {
                        char h = s_[p_++];
                        cp <<= 4;
                        if (h >= '0' && h <= '9') cp |= unsigned(h - '0');
                        else if (h >= 'a' && h <= 'f') cp |= unsigned(h - 'a' + 10);
                        else if (h >= 'A' && h <= 'F') cp |= unsigned(h - 'A' + 10);
                        else return false;
                    }
                    if (cp < 0x80) out.push_back(char(cp));
                    else if (cp < 0x800) { out.push_back(char(0xC0 | (cp >> 6))); out.push_back(char(0x80 | (cp & 0x3F))); }
                    else { out.push_back(char(0xE0 | (cp >> 12))); out.push_back(char(0x80 | ((cp >> 6) & 0x3F))); out.push_back(char(0x80 | (cp & 0x3F))); }
                    break;
                }
                default: why_ = "bad escape"; return false;
            }
        }
        return false;
    }
    bool value(J& v, int depth) {
        if (depth > 32) { why_ = "too deep"; return false; }
        ws();
        if (p_ >= s_.size()) return false;
        char c = s_[p_];
        if (c == '{') {
            ++p_; v.t = J::Obj; ws();
            if (p_ < s_.size() && s_[p_] == '}') { ++p_; return true; }
            for (;;) {
                ws();
                std::string k;
                if (!str(k)) return false;
                ws();
                if (p_ >= s_.size() || s_[p_++] != ':') return false;
                J child;
                if (!value(child, depth + 1)) return false;
                v.o.emplace_back(std::move(k), std::move(child));
                ws();
                if (p_ >= s_.size()) return false;
                char d = s_[p_++];
                if (d == '}') return true;
                if (d != ',') return false;
            }
        }
        if (c == '[') {
            ++p_; v.t = J::Arr; ws();
            if (p_ < s_.size() && s_[p_] == ']') { ++p_; return true; }
            for (;;) {
                J child;
                if (!value(child, depth + 1)) return false;
                v.a.push_back(std::move(child));
                ws();
                if (p_ >= s_.size()) return false;
                char d = s_[p_++];
                if (d == ']') return true;
                if (d != ',') return false;
            }
        }
        if (c == '"') { v.t = J::Str; return str(v.s); }
        if (lit("true")) { v.t = J::Bool; v.b = true; return true; }
        if (lit("false")) { v.t = J::Bool; v.b = false; return true; }
        if (lit("null")) { v.t = J::Null; return true; }
        // number
        size_t st = p_;
        if (p_ < s_.size() && (s_[p_] == '-' || s_[p_] == '+')) ++p_;
        while (p_ < s_.size() && (std::isdigit(static_cast<unsigned char>(s_[p_])) || s_[p_] == '.' || s_[p_] == 'e' || s_[p_] == 'E' || s_[p_] == '-' || s_[p_] == '+')) ++p_;
        if (p_ == st) return false;
        char* end = nullptr;
        std::string num = s_.substr(st, p_ - st);
        v.n = std::strtod(num.c_str(), &end);
        if (!end || *end != 0 || !std::isfinite(v.n)) { why_ = "bad number"; return false; }
        v.t = J::Num;
        return true;
    }
};

std::string scalarText(const J& j) {
    switch (j.t) {
        case J::Str: return j.s;
        case J::Bool: return j.b ? "true" : "false";
        case J::Num: { std::ostringstream o; o << j.n; return o.str(); }
        default: return "";
    }
}

std::string lower(std::string s) {
    for (auto& c : s) c = static_cast<char>(std::tolower(static_cast<unsigned char>(c)));
    return s;
}

bool truthy(double v) { return v != 0.0; }

bool parseUint(const std::string& s, uint32_t& out) {
    if (s.empty() || s.size() > 10) return false;
    uint64_t v = 0;
    for (char c : s) {
        if (c < '0' || c > '9') return false;
        v = v * 10 + uint64_t(c - '0');
    }
    if (v > 0xFFFFFFFFull) return false;
    out = uint32_t(v);
    return true;
}

// "Plugin.esm:HEX" / "Plugin.esm HEX" form reference, as used by ReadyCheck `infos`.
bool parseFormText(const std::string& s, FormRef& out) {
    size_t sep = s.find_last_of(": ");
    if (sep == std::string::npos || sep == 0 || sep + 1 >= s.size()) return false;
    uint32_t id = 0;
    for (size_t i = sep + 1; i < s.size(); ++i) {
        char c = s[i];
        uint32_t d = c >= '0' && c <= '9' ? uint32_t(c - '0') : c >= 'a' && c <= 'f' ? uint32_t(c - 'a' + 10) : c >= 'A' && c <= 'F' ? uint32_t(c - 'A' + 10) : 99u;
        if (d == 99u || (id >> 28)) return false;
        id = (id << 4) | d;
    }
    out = FormRef::make(s.substr(0, sep), id);
    return true;
}

bool isIdent(const std::string& s) {
    if (s.empty() || std::isdigit(static_cast<unsigned char>(s[0]))) return false;
    for (char c : s) if (!(std::isalnum(static_cast<unsigned char>(c)) || c == '_')) return false;
    return true;
}

// Kind-specific load-time checks. Returns an empty string when the rule is fine.
std::string validateRule(const std::string& edid, const Rule& r) {
    const std::string who = edid + "/" + r.kind;
    auto sl = r.lists.find("stages");
    if (sl != r.lists.end()) {
        for (auto& v : sl->second) { uint32_t n; if (!parseUint(v, n)) return who + ": stages must be whole numbers, got '" + v + "'"; }
    }
    if (r.scalars.count("stages")) return who + ": stages must be a list";
    if (r.kind == "PartyTether") {
        auto c = r.lists.find("cells");
        const bool cells = c != r.lists.end() && !c->second.empty();
        const std::string anchor = r.scalar("anchor", r.scalar("anchor_ref"));
        if (!cells && anchor.empty()) return who + ": a tether needs a non-empty cells list or anchor + radius (an empty cell list tethers nothing)";
        if (!anchor.empty() && !(r.number("radius", 0) > 0)) return who + ": anchor tethers need a radius > 0";
    } else if (r.kind == "ExclusiveStages") {
        if (sl == r.lists.end() || sl->second.size() < 2) return who + ": needs at least two stages";
        if (r.scalar("resolution", "first-wins") != "first-wins") return who + ": only resolution first-wins exists";
    } else if (r.kind == "ReadyCheck") {
        auto i = r.lists.find("infos");
        if (i == r.lists.end() || i->second.empty()) return who + ": needs infos";
        const std::string oc = r.scalar("on_timeout", "cancel");
        if (oc != "cancel" && oc != "proceed") return who + ": on_timeout must be cancel or proceed";
        if (r.scalar("confirm", "all-players") != "all-players") return who + ": only confirm all-players exists";
        if (r.scalars.count("timeout_s") && !(r.number("timeout_s", 0) > 0)) return who + ": timeout_s must be > 0";
    } else if (r.kind == "PerPlayerVars") {
        auto v = r.lists.find("vars");
        if (v == r.lists.end() || v->second.empty()) return who + ": needs vars";
    } else if (r.kind == "Barrier") {
        const std::string b = r.scalar("before");
        if (!b.empty() && b != "slideshow") return who + ": before may only be slideshow";
    } else if (r.kind == "ChoiceConsent") {
        auto t = r.lists.find("topics"), b = r.lists.find("boxes");
        if ((t == r.lists.end() || t->second.empty()) && (b == r.lists.end() || b->second.empty())) return who + ": needs topics (INFOs) or boxes (message boxes)";
        for (const char* key : {"topics", "boxes"}) {
            auto l = r.lists.find(key);
            if (l == r.lists.end()) continue;
            for (auto& v : l->second) { FormRef f; if (!parseFormText(v, f) && !isIdent(v)) return who + ": " + key + " entry '" + v + "' is neither Plugin.esm:HEX nor an EditorID"; }
        }
        const std::string m = r.scalar("mode", "vote");
        if (m != "vote" && m != "unanimous" && m != "host" && m != "initiator") return who + ": mode must be vote, unanimous, host or initiator";
        if (r.scalars.count("timeout_s") && !(r.number("timeout_s", 0) > 0)) return who + ": timeout_s must be > 0";
    } else if (r.kind == "PerkRecipient") {
        const std::string d = r.scalar("default");
        if (!d.empty() && d != "all" && d != "initiator" && d != "recruiter") return who + ": default must be all, initiator or recruiter";
        if (r.scalars.count("all") || r.scalars.count("initiator") || r.scalars.count("recruiter")) return who + ": all / initiator / recruiter must be lists";
        if (d.empty() && !r.lists.count("all") && !r.lists.count("initiator") && !r.lists.count("recruiter")) return who + ": names no perk and no default";
    } else if (r.kind == "FriendlyFireGuard") {
        auto a = r.lists.find("actors");
        if (a == r.lists.end() || a->second.empty()) return who + ": needs actors";
        if (r.scalars.count("deliberate_hits") && !(r.number("deliberate_hits", 0) >= 1)) return who + ": deliberate_hits must be >= 1";
        if (r.scalars.count("window_s") && !(r.number("window_s", 0) > 0)) return who + ": window_s must be > 0";
    } else if (r.kind == "PerPlayerState") {
        auto v = r.lists.find("vars");
        if ((v == r.lists.end() || v->second.empty()) && r.scalar("scope").empty()) return who + ": needs vars or a scope";
        const std::string dn = r.scalar("down");
        if (!dn.empty() && dn != "until-checkpoint" && dn != "timed") return who + ": down must be until-checkpoint or timed";
        if (r.scalars.count("scripts") || r.scalars.count("objvars") || r.scalars.count("checkpoint")) return who + ": scripts / objvars / checkpoint must be lists";
        auto ov = r.lists.find("objvars");
        if (ov != r.lists.end())
            for (auto& n : ov->second) { const size_t dot = n.find('.'); if (dot == std::string::npos || dot == 0 || dot + 1 >= n.size() || !isIdent(n.substr(0, dot)) || !isIdent(n.substr(dot + 1))) return who + ": objvars entry '" + n + "' is not Script.var"; }
        auto sc = r.lists.find("scripts");
        if (sc != r.lists.end())
            for (auto& n : sc->second) if (!isIdent(n)) return who + ": scripts entry '" + n + "' is not an EditorID";
    } else if (r.kind == "PooledItems") {
        auto it = r.lists.find("items");
        if (it == r.lists.end() || it->second.empty()) return who + ": needs items";
        for (auto& v : it->second) { FormRef f; if (!parseFormText(v, f) && !isIdent(v)) return who + ": items entry '" + v + "' is neither Plugin.esm:HEX nor an EditorID"; }
    } else if (r.kind == "OncePerEvent") {
        auto g = r.lists.find("globals"), e = r.lists.find("effects");
        if ((g == r.lists.end() || g->second.empty()) && (e == r.lists.end() || e->second.empty())) return who + ": needs globals or effects";
        if (r.scalars.count("window_s") && !(r.number("window_s", 0) > 0)) return who + ": window_s must be > 0";
        if (e != r.lists.end()) for (auto& n : e->second) if (n != "AdvanceTime" && n != "SetGlobal") return who + ": effects entry '" + n + "' must be AdvanceTime or SetGlobal";
    } else if (r.kind == "SetPlayerControls") {
        const std::string sc = r.scalar("scope", "all");
        if (sc != "all" && sc != "initiator") return who + ": scope must be all or initiator";
    } else if (r.kind == "PartyWarp") {
        if (r.scalars.count("range") && !(r.number("range", 0) > 0)) return who + ": range must be > 0";
    } else if (r.kind == "CompanionOwner") {
        const std::string o = r.scalar("owner", "recruiter");
        if (o != "recruiter" && o != "party") return who + ": owner must be recruiter or party";
    }
    return std::string();
}

}  // namespace

bool Rule::appliesToStage(uint32_t stage) const {
    auto it = lists.find("stages");
    if (it == lists.end() || it->second.empty() || mutations().stagesParamIgnored) return true;
    for (auto& s : it->second) { uint32_t n; if (parseUint(s, n) && n == stage) return true; }
    return false;
}

double Rule::number(const std::string& key, double dflt) const {
    auto it = scalars.find(key);
    if (it == scalars.end() || it->second.empty()) return dflt;
    char* end = nullptr;
    double v = std::strtod(it->second.c_str(), &end);
    return end && *end == 0 ? v : dflt;
}

// ================================ RulesDb ======================================================================

bool RulesDb::load(const std::string& text, std::string* err) {
    J root;
    Parser p(text);
    if (!p.parse(root, err)) return false;
    auto bad = [&](const std::string& why) { if (err) *err = "rules: " + why; return false; };
    if (root.t != J::Obj) return bad("root must be an object");
    const J* schema = root.get("schema");
    if (!schema || schema->t != J::Num || schema->n != 1) return bad("unsupported schema");
    const J* quests = root.get("quests");
    if (!quests || quests->t != J::Obj) return bad("missing quests");
    std::map<std::string, QuestRules> out;
    for (auto& [edid, q] : quests->o) {
        if (q.t != J::Obj) return bad("quest " + edid + " is not an object");
        QuestRules qr;
        qr.edid = edid;
        if (auto* t = q.get("tier")) qr.tier = scalarText(*t);
        if (auto* t = q.get("questline")) qr.questline = scalarText(*t);
        if (auto* t = q.get("curated")) qr.curated = t->t == J::Bool && t->b;
        const J* rules = q.get("rules");
        if (!rules || rules->t != J::Arr) return bad("quest " + edid + " has no rules array");
        for (auto& r : rules->a) {
            if (r.t != J::Obj) return bad("rule in " + edid + " is not an object");
            const J* kind = r.get("kind");
            if (!kind || kind->t != J::Str || kind->s.empty()) return bad("rule in " + edid + " has no kind");
            Rule rule;
            rule.kind = kind->s;
            if (auto* o = r.get("origin")) rule.origin = scalarText(*o);
            if (auto* params = r.get("params")) {
                if (params->t != J::Obj) return bad("params of " + edid + "/" + rule.kind + " must be an object");
                for (auto& [k, v] : params->o) {
                    if (v.t == J::Arr) {
                        for (auto& e : v.a) {
                            if (e.t == J::Arr || e.t == J::Obj) return bad("nested arrays in params of " + edid);
                            rule.lists[k].push_back(scalarText(e));
                        }
                        if (v.a.empty()) rule.lists[k];
                    } else if (v.t == J::Obj) {
                        if (k != "funcs") return bad("unexpected object param " + k + " in " + edid);
                        for (auto& [fk, fv] : v.o) rule.funcs[lower(fk)] = scalarText(fv);
                    } else {
                        rule.scalars[k] = scalarText(v);
                    }
                }
            }
            if (std::string why = validateRule(edid, rule); !why.empty()) return bad(why);
            qr.rules.push_back(std::move(rule));
        }
        out.emplace(edid, std::move(qr));
    }
    std::map<std::string, std::string> policy;
    if (const J* pol = root.get("policy")) {
        if (pol->t != J::Obj) return bad("policy must be an object");
        for (auto& [sec, vals] : pol->o) {
            if (vals.t != J::Obj) return bad("policy section " + sec + " must be an object");
            for (auto& [k, v] : vals.o) {
                if (v.t == J::Arr || v.t == J::Obj) return bad("policy " + sec + "." + k + " must be a scalar");
                policy[sec + "." + k] = scalarText(v);
            }
        }
    }
    quests_ = std::move(out);
    policy_ = std::move(policy);
    textHash_ = hash64(text.data(), text.size());
    return true;
}

bool RulesDb::loadFile(const std::string& path, std::string* err) {
    std::ifstream f(path, std::ios::binary);
    if (!f) { if (err) *err = "cannot open " + path; return false; }
    std::stringstream ss;
    ss << f.rdbuf();
    return load(ss.str(), err);
}

const QuestRules* RulesDb::find(const std::string& edid) const {
    auto it = quests_.find(edid);
    return it == quests_.end() ? nullptr : &it->second;
}

std::map<std::string, size_t> RulesDb::kindCounts() const {
    std::map<std::string, size_t> c;
    for (auto& [e, q] : quests_) for (auto& r : q.rules) ++c[r.kind];
    return c;
}

// ================================ RulesEngine ==================================================================

RulesEngine::RulesEngine(HostSession& host, const RulesDb& db, Resolver resolve) : RulesEngine(host, db, std::move(resolve), Options()) {}

RulesEngine::RulesEngine(HostSession& host, const RulesDb& db, Resolver resolve, Options opt)
    : host_(host), db_(db), opt_(opt), resolve_(std::move(resolve)) {
    static const std::set<std::string> handled = {"Barrier", "PartyTether", "SetPlayerControls", "SaveSuppress", "RewardAll", "QuestItemDup",
                                                  "RemoveItemPerOwner", "ShareKarmaRep", "CompanionOwner", "SlideshowEnd", "PartyAwareCond",
                                                  "ExcludeFromGate", "PerPlayerScene", "FastTravelLock", "InitiatorDialogue", "PartyWarp", "GuestLocalRun",
                                                  "PerPlayerVars", "ExclusiveStages", "ReadyCheck", "ChoiceConsent", "PerkRecipient", "FriendlyFireGuard",
                                                  "PerPlayerState", "PooledItems", "OncePerEvent"};
    for (auto& [edid, q] : db_.quests()) {
        FormRef f, f2;
        if (resolve_ && resolve_(edid, f)) byForm_[f] = &q;
        for (auto& r : q.rules) {
            if (!handled.count(r.kind)) unsupported_.insert(r.kind);
            if (r.kind == "ReadyCheck") {
                auto il = r.lists.find("infos");
                if (il != r.lists.end()) for (auto& txt : il->second) { FormRef f; if (parseFormText(txt, f)) { readyInfos_.insert(f); if (resolve_ && resolve_(edid, f2)) readyQuests_[f] = f2; } }
            } else if (r.kind == "ChoiceConsent" && resolve_ && resolve_(edid, f2)) {
                ChoiceInfo ci;
                ci.quest = f2;
                const std::string m = r.scalar("mode", "vote");
                ci.mode = m == "unanimous" ? ChoiceMode::Unanimous : m == "host" ? ChoiceMode::Host : m == "initiator" ? ChoiceMode::Initiator : ChoiceMode::Vote;
                ci.timeoutMs = static_cast<uint32_t>(r.number("timeout_s", 90) * 1000.0);
                for (const char* key : {"topics", "boxes"}) {
                    auto l = r.lists.find(key);
                    if (l != r.lists.end()) for (auto& txt : l->second) { FormRef f; if (parseFormText(txt, f) || (resolve_ && resolve_(txt, f))) choices_[f] = ci; }
                }
            } else if (r.kind == "PerPlayerState") {
                auto cl = r.lists.find("checkpoint");
                if (cl != r.lists.end()) for (auto& txt : cl->second) { FormRef f; if (parseFormText(txt, f) || (resolve_ && resolve_(txt, f))) checkpointCells_.insert(f); }
            } else if (r.kind == "FriendlyFireGuard") {
                auto al = r.lists.find("actors");
                if (al != r.lists.end())
                    for (auto& txt : al->second) { FormRef f; if (parseFormText(txt, f) || (resolve_ && resolve_(txt, f))) protected_[f] = &r; }
            }
        }
    }
    host_.txnFilter = [this](EffectTxn& t) { filter(t); };
    host_.isPartyCompanion = [this](const FormRef& npc) { return !mutations().partyOwnerIgnored && partyCompanions_.count(npc) > 0; };
}

RulesEngine::~RulesEngine() { host_.txnFilter = nullptr; host_.isPartyCompanion = nullptr; }

namespace {
// PerPlayerScene with run=every-machine and scope=local-machine: the scene belongs to each machine's own player (achievements, local
// fades, implant menus): it runs everywhere and nothing of it is replicated, exactly like GuestLocalRun.
bool runsOnEveryMachine(const QuestRules& q) {
    if (q.find("GuestLocalRun")) return true;
    const Rule* p = q.find("PerPlayerScene");
    return p && p->scalar("run") == "every-machine" && p->scalar("scope") == "local-machine";
}
}  // namespace

bool RulesEngine::guestLocalRun(const FormRef& quest) const {
    const QuestRules* q = rulesFor(quest);
    return q && runsOnEveryMachine(*q);
}
std::vector<FormRef> RulesEngine::guestLocalQuests() const {
    std::vector<FormRef> v;
    for (auto& [f, q] : byForm_) if (runsOnEveryMachine(*q)) v.push_back(f);
    return v;
}
std::vector<std::string> RulesEngine::guestLocalScripts() const {
    std::vector<std::string> v;
    for (auto& [edid, q] : db_.quests())
        if (const Rule* r = q.find("GuestLocalRun")) {
            auto it = r->lists.find("scripts");
            if (it != r->lists.end()) for (auto& s : it->second) v.push_back(s);
        }
    return v;
}

const QuestRules* RulesEngine::rulesFor(const FormRef& quest) const {
    auto it = byForm_.find(quest);
    return it == byForm_.end() ? nullptr : it->second;
}

bool RulesEngine::excludedFromGate(const FormRef& quest) const {
    auto* q = rulesFor(quest);
    return q && q->find("ExcludeFromGate");
}

FormRef RulesEngine::barrierId(const FormRef& quest, uint32_t stage) {
    char buf[16];
    std::snprintf(buf, sizeof buf, "%08X", quest.localId);
    return FormRef(quest.plugin + "#b" + buf, stage);
}

std::vector<FormRef> RulesEngine::guestBarrierPasses(const ShadowState& s, PlayerId slot) {
    std::vector<FormRef> out;
    if (slot >= 8) return out;
    const uint8_t me = static_cast<uint8_t>(1u << slot);
    for (auto& kv : s.barriers) {
        const BarrierState& b = kv.second;
        if (!mutations().guestPassIgnoresState && (b.released || (b.arrived & me) || !(b.required & me) || !b.arrived)) continue;
        const std::string& p = kv.first.plugin;   // barrierId: "<plugin>#b<8 hex>"
        const size_t at = p.rfind("#b");
        if (at == std::string::npos || p.size() != at + 10) continue;
        char* end = nullptr;
        const unsigned long id = std::strtoul(p.c_str() + at + 2, &end, 16);
        if (!end || *end) continue;
        FormRef q(p.substr(0, at), static_cast<uint32_t>(id));
        if (std::find(out.begin(), out.end(), q) == out.end()) out.push_back(q);
    }
    return out;
}

uint8_t RulesEngine::joinedMask() const {
    uint8_t m = 1u << kHostPlayer;
    for (auto& c : host_.clients()) if ((c.joined || mutations().departedHoldBarrier) && c.slot < 8) m = static_cast<uint8_t>(m | (1u << c.slot));
    return m;
}

bool RulesEngine::tetherSatisfied(const std::set<FormRef>& cells, const FormRef& anchor, bool hasAnchor) const {
    if (cells.empty() && !hasAnchor) return true;
    if (hasAnchor && cells.empty() && mutations().tetherIgnoresAnchor) return true;
    return tetherAbsent(cells, anchor, hasAnchor).empty();
}

// The joined slots that are NOT inside the tether (in one of its cells / its worldspace, or within the radius of its anchor).
std::vector<PlayerId> RulesEngine::tetherAbsent(const std::set<FormRef>& cells, const FormRef& anchor, bool hasAnchor) const {
    std::vector<PlayerId> out;
    uint8_t m = joinedMask();
    for (unsigned s = 0; s < kMaxSlots; ++s) {
        if (!(m >> s & 1u)) continue;
        auto it = cellOf_.find(PlayerId(s));
        if (it != cellOf_.end() && cells.count(it->second)) continue;
        auto wd = worldOf_.find(PlayerId(s));
        if (wd != worldOf_.end() && !mutations().tetherIgnoresWorld && cells.count(wd->second)) continue;   // an exterior tether names the worldspace
        auto nr = nearOf_.find(PlayerId(s));
        if (hasAnchor && !mutations().tetherIgnoresAnchor && nr != nearOf_.end() && nr->second.count(anchor)) continue;
        out.push_back(PlayerId(s));
    }
    return out;
}

StageDecision RulesEngine::onStageRequested(const FormRef& quest, uint32_t stage, PlayerId initiator) {
    if (mutations().rulesDisabled) return StageDecision::Proceed;
    const QuestRules* q = rulesFor(quest);
    if (!q) return StageDecision::Proceed;
    for (auto& p : pending_) if (p.quest == quest && p.stage == stage) return StageDecision::Deferred;   // already waiting
    if (const Rule* br = q->findFor("Barrier", stage)) {
        Pending p;
        p.quest = quest; p.stage = stage; p.initiator = initiator; p.barrier = barrierId(quest, stage); p.since = now_;
        p.slideshowAfter = br->scalar("before") == "slideshow";
        auto early = earlyPasses_.find(quest);
        pending_.push_back(std::move(p));
        ++stats_.stagesDeferred;
        syncTravelLock(quest);
        if (early != earlyPasses_.end()) {   // players that finished before the host asked
            std::set<PlayerId> slots = std::move(early->second);
            earlyPasses_.erase(early);
            for (PlayerId s : slots) onPlayerPassed(s, quest);
        }
        return StageDecision::Deferred;
    }
    if (const Rule* tr = q->findFor("PartyTether", stage)) {
        const std::string tetherMode = mutations().tetherModeIgnored ? std::string("on") : policy("tether.mode", "on");
        if (tetherMode == "off") { ++stats_.tethersIgnored; return StageDecision::Proceed; }   // owner switch: tethers are ignored
        std::set<FormRef> cells;
        auto it = tr->lists.find("cells");
        if (it != tr->lists.end())
            for (auto& edid : it->second) { FormRef f; if (resolve_ && resolve_(edid, f)) cells.insert(f); }
        FormRef anchor;
        bool hasAnchor = false;
        const std::string anchorEdid = tr->scalar("anchor", tr->scalar("anchor_ref"));
        if (!anchorEdid.empty() && resolve_ && resolve_(anchorEdid, anchor)) hasAnchor = true;
        if (tetherSatisfied(cells, anchor, hasAnchor)) {
            if (!cells.empty() || hasAnchor) { zone_ = cells.empty() ? anchor : *cells.begin(); hasZone_ = true; releaseCheckpointDowned(); }   // everybody is already there: that is the checkpoint
            return StageDecision::Proceed;
        }
        if (tetherMode == "warn") {   // never hold, never warp: only tell the players outside the zone
            ++stats_.tetherWarnings;
            if (tetherWarn) tetherWarn(quest, stage, tetherAbsent(cells, anchor, hasAnchor), cells.empty() ? anchor : *cells.begin());
            return StageDecision::Proceed;
        }
        Pending p;
        p.quest = quest; p.stage = stage; p.initiator = initiator; p.tether = true; p.since = now_; p.cells = cells;
        p.anchor = anchor; p.hasAnchor = hasAnchor;
        p.autoWarp = tr->scalar("gather") == "auto-warp";
        if (tr->scalars.count("timeout_s")) p.timeoutMs = static_cast<uint32_t>(tr->number("timeout_s", 0) * 1000.0);
        Cause c; c.kind = CauseKind::Script; c.source = quest; c.aux = static_cast<int32_t>(stage);
        host_.commit(kHostPlayer, c, {PartyTether{cells.empty() ? anchor : *cells.begin(), true}});   // clients gather / hold
        ++stats_.tetherGathers;
        pending_.push_back(std::move(p));
        ++stats_.stagesDeferred;
        syncTravelLock(quest);
        return StageDecision::Deferred;
    }
    return StageDecision::Proceed;
}

FormRef RulesEngine::timeSkipId() { return FormRef("rules", 0x7157EEu); }

void RulesEngine::onPlayerPassed(PlayerId slot, const FormRef& ref) {
    if (slot >= kMaxSlots) return;
    for (size_t i = 0; i < pending_.size(); ++i) {
        Pending& p = pending_[i];
        if (p.tether) continue;
        if (p.kind != Pending::Stage) {   // a consent vote: keyed by the INFO (ReadyCheck / ChoiceConsent) or by timeSkipId() (wait / sleep)
            if ((p.kind == Pending::TimeSkip ? timeSkipId() : p.info) != ref) continue;
            if (p.kind == Pending::Choice && p.declined.count(slot)) p.declined.erase(slot);   // a changed mind
            if (!p.passed.insert(slot).second) { ++stats_.ignoredPasses; return; }
            ++stats_.arrivals;
            checkPending();
            return;
        }
        if (p.quest != ref) continue;
        if (!p.passed.insert(slot).second) { ++stats_.ignoredPasses; return; }   // duplicate report
        ++stats_.arrivals;
        Cause c; c.kind = CauseKind::Script; c.source = ref; c.aux = static_cast<int32_t>(p.stage);
        host_.commit(slot, c, {BarrierArrive{p.barrier, joinedMask()}});
        const BarrierState* b = nullptr;
        auto it = host_.state().barriers.find(p.barrier);
        if (it != host_.state().barriers.end()) b = &it->second;
        bool done = b && b->released;
        if (mutations().barrierReleaseEarly) done = true;   // TEST ONLY: first arrival releases
        if (done) release(i, false);
        return;
    }
    if (ref == timeSkipId() || readyInfos_.count(ref) || choices_.count(ref)) return;   // a consent for a vote nobody asked for must not count for a later one
    earlyPasses_[ref].insert(slot);   // nobody asked for this barrier yet
}

void RulesEngine::onCellReported(PlayerId slot, const FormRef& cell, const FormRef& world) {
    cellOf_[slot] = cell;
    if (world.plugin.empty()) worldOf_.erase(slot); else worldOf_[slot] = world;
    checkPending();
    // G42: a living member standing in a checkpoint cell of a collar rule ends the downed state of the collar deaths
    if (!holdDowned_.empty() && !downed_.count(slot) && (checkpointCell(cell) || (!world.plugin.empty() && checkpointCell(world)))) {
        zone_ = checkpointCell(cell) ? cell : world; hasZone_ = true;
        releaseCheckpointDowned();
    }
}

void RulesEngine::onWorldReported(PlayerId slot, const FormRef& world) {
    if (world.plugin.empty()) worldOf_.erase(slot); else worldOf_[slot] = world;
    checkPending();
    if (!holdDowned_.empty() && !downed_.count(slot) && !world.plugin.empty() && checkpointCell(world)) { zone_ = world; hasZone_ = true; releaseCheckpointDowned(); }
}

bool RulesEngine::checkpointCell(const FormRef& cell) const { return checkpointCells_.count(cell) > 0; }

void RulesEngine::releaseCheckpointDowned() {
    if (holdDowned_.empty() || mutations().rulesDisabled) return;
    bool alive = false;                                    // a checkpoint revives only while somebody who is up reached it
    const uint8_t joined = joinedMask();
    for (unsigned s = 0; s < kMaxSlots; ++s) if ((joined >> s & 1u) && !downed_.count(PlayerId(s))) alive = true;
    if (!alive) return;
    const std::set<PlayerId> slots = holdDowned_;
    holdDowned_.clear();
    for (PlayerId s : slots) {
        downed_.erase(s);
        ++stats_.respawns; ++stats_.checkpointRevives;
        FormRef z; const bool has = zone(z);
        if (respawnPlayer) respawnPlayer(s, has ? z : FormRef());
    }
}

void RulesEngine::onNearReported(PlayerId slot, const FormRef& anchor, bool isNear) {
    if (slot >= kMaxSlots) return;
    if (isNear) nearOf_[slot].insert(anchor); else nearOf_[slot].erase(anchor);
    checkPending();
}

bool RulesEngine::handleAction(PlayerId slot, IntentKind kind, const FormRef& ref) {
    if (kind == IntentKind::ScenePass) {
        if (ref.plugin.size() > 2 && ref.plugin.compare(ref.plugin.size() - 2, 2, "#n") == 0) {   // choiceDeclineId: a "no" vote
            FormRef choice(ref.plugin.substr(0, ref.plugin.size() - 2), ref.localId);
            for (auto& p : pending_)
                if (p.kind == Pending::Choice && p.info == choice && slot < kMaxSlots) {
                    p.passed.erase(slot);
                    if (p.declined.insert(slot).second) { ++stats_.arrivals; checkPending(); } else ++stats_.ignoredPasses;
                    return true;
                }
            return true;   // a decline for a vote nobody asked: dropped
        }
        onPlayerPassed(slot, ref);
        return true;
    }
    if (kind == IntentKind::ReportCell) { onCellReported(slot, ref); return true; }
    if (kind == IntentKind::ReportNear) { onNearReported(slot, ref, true); return true; }
    if (kind == IntentKind::ReportFar) { onNearReported(slot, ref, false); return true; }
    return false;
}

void RulesEngine::startVote(Pending p) {
    p.since = now_;
    p.passed.insert(p.initiator);   // asking is agreeing
    pending_.push_back(std::move(p));
}

// True when nobody but the asker is present: there is no one to ask, the caller just proceeds.
bool RulesEngine::askerIsAlone(PlayerId asker) const {
    return (joinedMask() & ~(1u << asker)) == 0;
}

StageDecision RulesEngine::onReadyRequested(const FormRef& quest, const FormRef& info, PlayerId initiator) {
    if (mutations().rulesDisabled) return StageDecision::Proceed;
    const QuestRules* q = rulesFor(quest);
    if (!q) return StageDecision::Proceed;
    for (auto& p : pending_) if (p.kind == Pending::Ready && p.info == info) return StageDecision::Deferred;   // already asking
    for (auto& r : q->rules) {
        if (r.kind != "ReadyCheck") continue;
        auto il = r.lists.find("infos");
        if (il == r.lists.end()) continue;
        bool listed = false;
        for (auto& txt : il->second) { FormRef f; listed |= parseFormText(txt, f) && f == info; }
        if (!listed) continue;
        Pending p;
        p.kind = Pending::Ready; p.quest = quest; p.info = info; p.initiator = initiator;
        p.timeoutMs = static_cast<uint32_t>(r.number("timeout_s", 120) * 1000.0);
        p.proceedOnTimeout = r.scalar("on_timeout", "cancel") == "proceed";
        ++stats_.readyChecks;
        if (askerIsAlone(initiator) && !mutations().readyProceedsAlone) return StageDecision::Proceed;   // no one to ask
        startVote(std::move(p));
        return StageDecision::Deferred;
    }
    return StageDecision::Proceed;
}

StageDecision RulesEngine::onTimeSkipRequested(PlayerId initiator, uint32_t minutes) {
    if (mutations().rulesDisabled) return StageDecision::Proceed;
    for (auto& p : pending_) if (p.kind == Pending::TimeSkip) return StageDecision::Deferred;   // one wait / sleep question at a time
    Pending p;
    p.kind = Pending::TimeSkip; p.quest = timeSkipId(); p.initiator = initiator; p.minutes = minutes; p.timeoutMs = opt_.consentTimeoutMs;
    ++stats_.timeSkipsAsked;
    if (askerIsAlone(initiator) && !mutations().readyProceedsAlone) { ++stats_.timeSkipsGranted; return StageDecision::Proceed; }
    startVote(std::move(p));
    return StageDecision::Deferred;
}

void RulesEngine::holdForSlideshow(const FormRef& quest) {
    if (mutations().slideshowNoHold || host_.state().slideshowEnded) return;
    if (slideshowHold_.count(quest)) return;
    slideshowHold_[quest] = now_;
    ++stats_.slideshowHolds;
    Cause c; c.kind = CauseKind::Script; c.source = quest;
    host_.commit(kHostPlayer, c, {SetPlayerControls{kAllControls, false, Recipient::all()}});
}

void RulesEngine::release(size_t index, bool timedOut) {
    Pending p = std::move(pending_[index]);
    pending_.erase(pending_.begin() + static_cast<std::ptrdiff_t>(index));
    if (p.kind == Pending::Choice) {
        const bool ok = choiceDecision(p, timedOut) == 1;
        ++stats_.stagesReleased;
        if (ok) ++stats_.choicesAccepted; else ++stats_.choicesCancelled;
        if (onChoiceResolved) onChoiceResolved(p.quest, p.info, ok);
        return;
    }
    if (p.kind == Pending::Ready) {
        const bool ok = !timedOut || p.proceedOnTimeout;
        if (timedOut) ++stats_.readyTimeouts;
        ++stats_.stagesReleased;
        if (onReadyResolved) onReadyResolved(p.quest, p.info, ok);
        return;
    }
    if (p.kind == Pending::TimeSkip) {
        if (timedOut) {
            ++stats_.timeSkipsDenied;
            if (timeSkipDenied) timeSkipDenied(p.minutes);
        } else {
            ++stats_.timeSkipsGranted;
            if (applyTimeSkip) applyTimeSkip(p.minutes);
        }
        return;
    }
    if (p.tether) {
        Cause c; c.kind = CauseKind::Script; c.source = p.quest; c.aux = static_cast<int32_t>(p.stage);
        const FormRef target = p.cells.empty() ? p.anchor : *p.cells.begin();
        if (!p.cells.empty() || p.hasAnchor) {
            if (timedOut && p.autoWarp && gatherRest) {   // the gather ran out of time: the plugin warps whoever is not there (D10)
                std::vector<PlayerId> absent = tetherAbsent(p.cells, p.anchor, p.hasAnchor);
                if (!absent.empty()) { ++stats_.tetherAutoWarps; gatherRest(p.quest, p.stage, absent, target); }
            }
            if (!timedOut) { zone_ = target; hasZone_ = true; releaseCheckpointDowned(); }   // everybody gathered: this is the zone checkpoint
            host_.commit(kHostPlayer, c, {PartyTether{target, false}});
        }
    } else if (timedOut || !host_.state().barriers.count(p.barrier) || !host_.state().barriers.at(p.barrier).released) {
        Cause c; c.kind = CauseKind::Script; c.source = p.quest; c.aux = static_cast<int32_t>(p.stage);
        host_.commit(kHostPlayer, c, {BarrierRelease{p.barrier}});   // explicit release (timeout, or players left)
    }
    ++stats_.stagesReleased;
    syncTravelLock(p.quest);
    if (p.slideshowAfter) holdForSlideshow(p.quest);   // the ending's controls stay locked until the slideshow itself ends
    host_.requestDigest();   // verify every client's world right after a barrier / scene
    if (applyStage) applyStage(p.quest, p.stage);
}

bool RulesEngine::hasLock(const FormRef& quest) const {
    auto it = locks_.lower_bound({quest, PlayerId(0)});
    return it != locks_.end() && it->first.first == quest;
}

bool RulesEngine::desiredTravelLock(const FormRef& quest) const {
    if (hasLock(quest)) return true;
    for (auto& p : pending_) if (p.quest == quest) return true;
    return false;
}

void RulesEngine::syncTravelLock(const FormRef& quest) {
    const QuestRules* q = rulesFor(quest);
    if (!q || !q->find("FastTravelLock")) return;
    bool want = desiredTravelLock(quest), have = travelLocked_.count(quest) > 0;
    if (want == have) return;
    if (want) travelLocked_.insert(quest); else travelLocked_.erase(quest);
    ++stats_.travelLockChanges;
    Cause c; c.kind = CauseKind::Script; c.source = quest;
    host_.commit(kHostPlayer, c, {SetFastTravelLock{quest, want}});
}

void RulesEngine::checkPending() {
    for (size_t i = 0; i < pending_.size();) {
        Pending& p = pending_[i];
        bool go = false;
        if (p.tether) {
            go = tetherSatisfied(p.cells, p.anchor, p.hasAnchor);
        } else if (p.kind == Pending::Choice) {
            go = choiceDecision(p, false) != -1;
        } else {
            uint8_t joined = joinedMask(), arrived = 0;
            for (PlayerId s : p.passed) arrived = static_cast<uint8_t>(arrived | (1u << s));
            go = (arrived & joined) == joined;   // everyone still present has passed (leavers do not hold the party)
            if (p.kind != Pending::Stage && mutations().readyProceedsAlone) go = arrived != 0;
        }
        if (go) release(i, false); else ++i;
    }
}

void RulesEngine::tick(uint64_t nowMs) {
    now_ = nowMs;
    checkPending();
    for (size_t i = 0; i < pending_.size();) {
        const uint32_t limit = (pending_[i].kind == Pending::Stage && pending_[i].timeoutMs == 0) ? opt_.barrierTimeoutMs : pending_[i].timeoutMs;
        if (now_ >= pending_[i].since + limit) {
            if (pending_[i].kind == Pending::Stage) ++stats_.barrierTimeouts;
            release(i, true);
        } else {
            ++i;
        }
    }
    for (auto it = slideshowHold_.begin(); it != slideshowHold_.end();) {   // failsafe: a slideshow that never ends must not freeze the party forever
        if (now_ >= it->second + opt_.slideshowHoldMs) {
            FormRef quest = it->first;
            it = slideshowHold_.erase(it);
            Cause c; c.kind = CauseKind::Script; c.source = quest;
            host_.commit(kHostPlayer, c, {SetPlayerControls{kAllControls, true, Recipient::all()}});
        } else {
            ++it;
        }
    }
    for (auto it = locks_.begin(); it != locks_.end();) {
        if (now_ >= it->second.since + opt_.controlTimeoutMs) {
            ++stats_.controlTimeouts;
            FormRef quest = it->first.first;
            const PlayerId who = it->first.second;
            uint32_t mask = it->second.mask;
            it = locks_.erase(it);
            Cause c; c.kind = CauseKind::Script; c.source = quest;
            std::vector<Effect> eff;
            const uint32_t rel = mask & ~heldByOthers(quest, who);   // only this lock own bits
            if (rel) eff.push_back(SetPlayerControls{rel, true, who == kNoPlayer ? Recipient::all() : Recipient::slot(who)});
            if (auto* q = rulesFor(quest); q && q->find("SaveSuppress") && !hasLock(quest)) eff.push_back(SaveSuppress{false});
            if (!eff.empty()) host_.commit(kHostPlayer, c, std::move(eff));
            syncTravelLock(quest);
        } else {
            ++it;
        }
    }
    // downed players that nobody revived in time respawn at the zone checkpoint
    for (auto it = downed_.begin(); it != downed_.end();) {
        if (holdDowned_.count(it->first)) { ++it; continue; }   // a collar death waits for the checkpoint, not for the clock
        const double reviveMs = policy("death.mode", "downed") == "respawn" ? 0.0 : std::strtod(policy("death.revive_s", "60").c_str(), nullptr) * 1000.0;
        if (now_ >= it->second + static_cast<uint64_t>(reviveMs < 0 ? 0 : reviveMs)) {
            PlayerId slot = it->first;
            it = downed_.erase(it);
            ++stats_.respawns;
            FormRef z; const bool has = zone(z);
            if (respawnPlayer) respawnPlayer(slot, has ? z : FormRef());
        } else {
            ++it;
        }
    }
}

// ---- PerPlayerVars -------------------------------------------------------------------------------------------------------------

bool RulesEngine::isPerPlayerVar(const FormRef& quest, const std::string& var) const {
    const QuestRules* q = rulesFor(quest);
    if (!q) return false;
    for (auto& r : q->rules) {
        if (r.kind != "PerPlayerVars" && r.kind != "PerPlayerState") continue;
        auto it = r.lists.find("vars");
        if (it != r.lists.end() && std::find(it->second.begin(), it->second.end(), var) != it->second.end()) return true;
        auto ov = r.lists.find("objvars");
        if (r.kind == "PerPlayerState" && ov != r.lists.end() && !mutations().perPlayerScriptsIgnored && std::find(ov->second.begin(), ov->second.end(), var) != ov->second.end()) return true;
    }
    return false;
}

std::vector<std::string> RulesEngine::perPlayerVarNames(const FormRef& quest) const {
    std::vector<std::string> v;
    const QuestRules* q = rulesFor(quest);
    if (!q) return v;
    for (auto& r : q->rules) {
        if (r.kind != "PerPlayerVars" && r.kind != "PerPlayerState") continue;
        auto it = r.lists.find("vars");
        if (it != r.lists.end()) for (auto& n : it->second) if (std::find(v.begin(), v.end(), n) == v.end()) v.push_back(n);
    }
    return v;
}

std::vector<RulesEngine::VarValue> RulesEngine::beginActing(const FormRef& quest, PlayerId slot, const std::vector<VarValue>& live) {
    std::vector<VarValue> out;
    if (slot >= kMaxSlots || acting_ != kNoPlayer) return out;   // not nested
    std::vector<std::string> names = perPlayerVarNames(quest);
    for (auto& n : perPlayerObjVarNames(quest)) names.push_back(n);   // object-script variables ("Script.var") keep their copies the same way
    bool perSlotRun = false;                                           // a PerPlayerState that only declares scripts (the AutoDoc, the kitchen traps) still opens the acting context
    if (const QuestRules* qr = rulesFor(quest)) for (auto& r : qr->rules) if (r.kind == "PerPlayerState" && r.lists.count("scripts") && !mutations().perPlayerScriptsIgnored) perSlotRun = true;
    if (names.empty() && !perSlotRun) return out;
    auto& seed = seeds_[quest];
    auto liveOf = [&](const std::string& n, double& v) { for (auto& x : live) if (x.var == n) { v = x.value; return true; } return false; };
    for (auto& n : names) { double v; if (!seed.count(n) && liveOf(n, v)) seed[n] = v; }   // the shared initial state, the first time we see the quest
    auto ld = loadedSlot_.find(quest);
    if (ld != loadedSlot_.end() && ld->second != slot)
        for (auto& n : names) { double v; if (liveOf(n, v)) playerVars_[{quest, n, ld->second}] = v; }   // park whoever was in the variables
    if (!mutations().actingCopyShared) {
        for (auto& n : names) {
            double v = 0;
            auto it = playerVars_.find({quest, n, slot});
            if (it != playerVars_.end()) v = it->second;
            else if (seed.count(n)) v = seed[n];
            else if (!liveOf(n, v)) continue;
            out.push_back({n, v});
        }
    }
    loadedSlot_[quest] = slot;
    acting_ = slot; actingQuest_ = quest;
    ++stats_.actingRuns;
    return out;
}

void RulesEngine::endActing(const FormRef& quest, PlayerId slot, const std::vector<VarValue>& live) {
    if (acting_ == kNoPlayer || acting_ != slot || !(actingQuest_ == quest)) return;
    for (auto& x : live)
        if (isPerPlayerVar(quest, x.var)) playerVars_[{quest, x.var, slot}] = x.value;   // objvars ("Script.var") count too (isPerPlayerVar)
    acting_ = kNoPlayer; actingQuest_ = FormRef();
}

double RulesEngine::viewPlayerVar(const FormRef& quest, const std::string& var, PlayerId slot, double real) const {
    if (!isPerPlayerVar(quest, var) || mutations().actingCopyShared) return real;
    auto it = playerVars_.find({quest, var, slot});
    if (it != playerVars_.end()) return it->second;
    auto sq = seeds_.find(quest);
    if (sq != seeds_.end()) { auto sv = sq->second.find(var); if (sv != sq->second.end()) return sv->second; }
    return real;
}

void RulesEngine::writePlayerVar(const FormRef& quest, const std::string& var, PlayerId slot, double value) {
    if (slot >= kMaxSlots || !isPerPlayerVar(quest, var)) return;
    if (mutations().perPlayerVarsShared) { for (unsigned s = 0; s < kMaxSlots; ++s) playerVars_[{quest, var, PlayerId(s)}] = value; return; }
    playerVars_[{quest, var, slot}] = value;
}

bool RulesEngine::readPlayerVar(const FormRef& quest, const std::string& var, PlayerId slot, double& out) const {
    auto it = playerVars_.find({quest, var, slot});
    if (it == playerVars_.end()) return false;
    out = it->second;
    return true;
}

std::vector<std::pair<std::string, double>> RulesEngine::playerVarsOf(const FormRef& quest, PlayerId slot) const {
    std::vector<std::pair<std::string, double>> v;
    for (auto& [k, val] : playerVars_)
        if (std::get<0>(k) == quest && std::get<2>(k) == slot) v.emplace_back(std::get<1>(k), val);
    return v;
}

// Host-side rules state worth keeping across a host restart: per-slot var copies, the seeds, the ExclusiveStages winners and the party companions.
Bytes RulesEngine::saveState() const {
    Archive a = Archive::writer();
    uint32_t magic = 0x32525246; a.u32(magic);   // 'FRR2' ('FRRS' is the older layout without seeds / party companions, still read)
    size_t n = playerVars_.size(); a.count(n, 1u << 20);
    for (auto& [k, val] : playerVars_) {
        FormRef q = std::get<0>(k); std::string var = std::get<1>(k); uint8_t sl = std::get<2>(k); double v = val;
        io(a, q); a.str(var, 255); a.u8(sl); a.f64(v);
    }
    size_t m = exclusiveWinner_.size(); a.count(m, 1u << 20);
    for (auto& [q, st] : exclusiveWinner_) { FormRef f = q; uint32_t s = st; io(a, f); a.varu32(s); }
    size_t ns = 0; for (auto& [q, vars] : seeds_) ns += vars.size();
    a.count(ns, 1u << 20);
    for (auto& [q, vars] : seeds_)
        for (auto& [var, val] : vars) { FormRef f = q; std::string v = var; double d = val; io(a, f); a.str(v, 255); a.f64(d); }
    size_t np = partyCompanions_.size(); a.count(np, 1u << 16);
    for (auto& f : partyCompanions_) { FormRef g = f; io(a, g); }
    return a.finish();
}

bool RulesEngine::loadState(const Bytes& b) {
    Archive a = Archive::reader(b.data(), b.size());
    uint32_t magic = 0; a.u32(magic);
    const bool v2 = magic == 0x32525246;
    std::map<std::tuple<FormRef, std::string, PlayerId>, double> vars;
    std::map<FormRef, uint32_t> winners;
    std::map<FormRef, std::map<std::string, double>> seeds;
    std::set<FormRef> party;
    size_t n = 0;
    if (!a.count(n, 1u << 20)) return false;
    for (size_t i = 0; i < n && a.ok(); ++i) {
        FormRef q; std::string var; uint8_t sl = 0; double v = 0;
        io(a, q); a.str(var, 255); a.u8(sl); a.f64(v);
        if (a.ok() && sl < kMaxSlots) vars[{q, var, sl}] = v;
    }
    size_t m = 0;
    if (!a.count(m, 1u << 20)) return false;
    for (size_t i = 0; i < m && a.ok(); ++i) { FormRef q; uint32_t s = 0; io(a, q); a.varu32(s); if (a.ok()) winners[q] = s; }
    if (v2) {
        size_t ns = 0;
        if (!a.count(ns, 1u << 20)) return false;
        for (size_t i = 0; i < ns && a.ok(); ++i) { FormRef q; std::string var; double d = 0; io(a, q); a.str(var, 255); a.f64(d); if (a.ok()) seeds[q][var] = d; }
        size_t np = 0;
        if (!a.count(np, 1u << 16)) return false;
        for (size_t i = 0; i < np && a.ok(); ++i) { FormRef f; io(a, f); if (a.ok()) party.insert(f); }
    }
    if (!a.ok() || !a.atEnd() || (magic != 0x53525246 && !v2)) return false;
    playerVars_ = std::move(vars);
    exclusiveWinner_ = std::move(winners);
    if (v2) { seeds_ = std::move(seeds); partyCompanions_ = std::move(party); }
    return true;
}

double RulesEngine::reduceCondition(const FormRef& quest, const std::string& func, const std::vector<PartyValue>& values,
                                    PlayerId initiator, CondPhase phase) {
    auto initiatorValue = [&]() {
        for (auto& v : values) if (v.slot == initiator) return v.value;
        return values.empty() ? 0.0 : values.front().value;
    };
    if (values.empty()) return 0.0;
    std::string red = "initiator";
    if (!mutations().rulesDisabled) {
        if (const QuestRules* q = rulesFor(quest))
            if (const Rule* r = q->find("PartyAwareCond")) {
                auto it = r->funcs.find(lower(func));
                if (it != r->funcs.end()) red = it->second;
            }
    }
    if (red == "initiator") return initiatorValue();
    if (red == "host") {   // the host value (XP tiers follow the host level, D8): no per-player global exists
        if (mutations().hostLevelAsInitiator) return initiatorValue();
        for (auto& v : values) if (v.slot == kHostPlayer) return v.value;
        return initiatorValue();
    }
    if (red == "any-party-member" || red == "any") {
        for (auto& v : values) if (truthy(v.value)) return 1.0;
        return 0.0;
    }
    if (red == "min-over-party") {
        double m = std::numeric_limits<double>::infinity();
        for (auto& v : values) m = std::min(m, v.value);
        return m;
    }
    if (red == "max-over-party" || red == "sum-over-party") {
        if (mutations().sumAsAny) {   // TEST ONLY: the old any-party-member stand-in (0 or 1, so a later >= 2 compare can never pass)
            for (auto& v : values) if (truthy(v.value)) return 1.0;
            return 0.0;
        }
        if (red == "max-over-party") {
            double m = -std::numeric_limits<double>::infinity();
            for (auto& v : values) m = std::max(m, v.value);
            return m;
        }
        double sum = 0;
        for (auto& v : values) sum += v.value;
        return sum;
    }
    if (red == "entry:all-in,exit:any-in") {
        if (phase == CondPhase::Entry) {
            for (auto& v : values) if (!truthy(v.value)) return 0.0;
            return 1.0;
        }
        for (auto& v : values) if (truthy(v.value)) return 1.0;
        return 0.0;
    }
    ++stats_.unknownReductions;
    return initiatorValue();
}

bool RulesEngine::stageExcluded(const FormRef& quest, uint32_t stage) const {
    if (mutations().rulesDisabled || mutations().exclusiveLastWins) return false;
    const QuestRules* sq = rulesFor(quest);
    const Rule* ex = sq ? sq->findFor("ExclusiveStages", stage) : nullptr;
    if (!ex) return false;
    auto w = exclusiveWinner_.find(quest);
    if (w != exclusiveWinner_.end()) return w->second != stage;
    auto st = host_.state().stages.find(quest);   // an outcome committed before this engine started counts (same rule as filter)
    return st != host_.state().stages.end() && ex->appliesToStage(st->second) && st->second != stage;
}

bool RulesEngine::listedForm(const Rule& r, const std::string& key, const FormRef& f) const {
    auto it = r.lists.find(key);
    if (it == r.lists.end()) return false;
    for (auto& txt : it->second) {
        FormRef g;
        if ((parseFormText(txt, g) || (resolve_ && resolve_(txt, g))) && g == f) return true;
    }
    return false;
}

// PerkRecipient: who gets the perk (G14). initiator and recruiter both mean the player whose dialogue / action the txn came from.
// `acting`: the txn has a real acting player (a dialogue result, an intent, a pickup ...). A stage result or a timer runs as the host, so an initiator / recruiter route
// has nobody to point at there and the quest default (everyone, for a reward) stays.
bool RulesEngine::perkRecipient(const QuestRules& q, const FormRef& perk, bool acting, Recipient& out) const {
    if (mutations().perkRouteIgnored) return false;
    const Rule* r = q.find("PerkRecipient");
    if (!r) return false;
    if (listedForm(*r, "all", perk)) { out = Recipient::all(); return true; }
    if (acting && (listedForm(*r, "initiator", perk) || listedForm(*r, "recruiter", perk))) { out = Recipient::origin(); return true; }
    const std::string d = r->scalar("default");
    if (d == "all") { out = Recipient::all(); return true; }
    if (acting && (d == "initiator" || d == "recruiter")) { out = Recipient::origin(); return true; }
    return false;
}

// Bits of SetPlayerControls that OTHER sources still hold on a player this lock covers (G18: a release only frees its own bits).
uint32_t RulesEngine::heldByOthers(const FormRef& source, PlayerId who) const {
    if (mutations().controlsReleaseAll) return 0;
    uint32_t held = 0;
    for (auto& [k, l] : locks_) {
        if (k.first == source && k.second == who) continue;
        if (k.second == kNoPlayer || who == kNoPlayer || k.second == who) held |= l.mask;
    }
    return held;
}

bool RulesEngine::questOfCause(const Cause& c, FormRef& quest) const {
    if (mutations().causeNotResolved || c.source.plugin.empty()) return false;
    if (rulesFor(c.source)) { quest = c.source; return true; }
    auto it = causeQuests_.find(c.source);
    if (it == causeQuests_.end()) return false;
    quest = it->second;
    return true;
}

bool RulesEngine::loadCauseQuests(const std::string& text, std::string* err) {
    J root;
    Parser p(text);
    if (!p.parse(root, err)) return false;
    auto bad = [&](const std::string& why) { if (err) *err = "cause quests: " + why; return false; };
    if (root.t != J::Obj) return bad("root must be an object");
    const J* schema = root.get("schema");
    if (!schema || schema->t != J::Num || schema->n != 1) return bad("unsupported schema");
    // "keys": rules entries that are not quests (a trigger script with its own rules, shared by the scripts that map to it): EDID -> the form that stands for the entry.
    std::map<std::string, FormRef> keyForms;
    if (const J* ks = root.get("keys")) {
        if (ks->t != J::Obj) return bad("keys must be an object");
        for (auto& [edid, v] : ks->o) {
            FormRef f;
            if (v.t != J::Str || !parseFormText(v.s, f)) return bad("key '" + edid + "' is not Plugin.esm:HEX");
            keyForms[edid] = f;
        }
    }
    std::map<FormRef, FormRef> out;
    for (const char* sec : {"scripts", "infos", "refs"}) {
        const J* o = root.get(sec);
        if (!o) continue;
        if (o->t != J::Obj) return bad(std::string(sec) + " must be an object");
        for (auto& [key, v] : o->o) {
            FormRef from, q;
            if (v.t != J::Str || !parseFormText(key, from)) return bad(std::string(sec) + " entry '" + key + "' is not Plugin.esm:HEX -> quest");
            auto kf = keyForms.find(v.s);
            if (kf != keyForms.end()) q = kf->second;
            else if (!resolve_ || !resolve_(v.s, q)) continue;   // a quest this build cannot name: the entry is inert
            out[from] = q;
        }
    }
    for (auto& [edid, f] : keyForms)
        if (const QuestRules* qr = db_.find(edid)) byForm_[f] = qr;
    causeQuests_ = std::move(out);
    return true;
}

// ---- ChoiceConsent -------------------------------------------------------------------------------------------------------------------------

FormRef RulesEngine::choiceDeclineId(const FormRef& choice) { return FormRef(choice.plugin + "#n", choice.localId); }

bool RulesEngine::choiceQuest(const FormRef& choice, FormRef& quest) const {
    auto it = choices_.find(choice);
    if (it == choices_.end()) return false;
    quest = it->second.quest;
    return true;
}

RulesEngine::ChoiceMode RulesEngine::choiceMode(const FormRef& choice) const {
    auto it = choices_.find(choice);
    return it == choices_.end() ? ChoiceMode::Initiator : it->second.mode;
}

// 1 accepted, 0 cancelled, -1 still open.
int RulesEngine::choiceDecision(const Pending& p, bool timedOut) const {
    auto ci = choices_.find(p.info);
    const ChoiceMode mode = ci == choices_.end() ? ChoiceMode::Initiator : ci->second.mode;
    if (mode == ChoiceMode::Initiator || mutations().choiceInitiatorDecides) return 1;
    const uint8_t joined = joinedMask();
    int n = 0, yes = 0, no = 0;
    for (unsigned s = 0; s < kMaxSlots; ++s) {
        if (!(joined >> s & 1u)) continue;
        ++n;
        yes += p.passed.count(PlayerId(s)) ? 1 : 0;
        no += p.declined.count(PlayerId(s)) ? 1 : 0;
    }
    const bool hostYes = p.passed.count(kHostPlayer) > 0, hostNo = p.declined.count(kHostPlayer) > 0;
    if (mode == ChoiceMode::Host) return hostYes ? 1 : hostNo ? 0 : timedOut ? 0 : -1;
    if (mode == ChoiceMode::Unanimous) return no > 0 ? 0 : yes >= n ? 1 : timedOut ? 0 : -1;
    // Vote: a strict majority of the joined players wins; with everybody voted and no majority the host breaks the tie.
    if (yes * 2 > n) return 1;
    if (no * 2 > n) return 0;
    if (yes + no >= n || timedOut) return hostYes ? 1 : 0;
    return -1;
}

StageDecision RulesEngine::onChoiceRequested(const FormRef& choice, PlayerId initiator) {
    if (mutations().rulesDisabled) return StageDecision::Proceed;
    auto ci = choices_.find(choice);
    if (ci == choices_.end()) return StageDecision::Proceed;
    for (auto& p : pending_) if (p.kind == Pending::Choice && p.info == choice) return StageDecision::Deferred;   // already asking
    ++stats_.choicesAsked;
    const ChoiceMode mode = ci->second.mode;
    const bool alone = askerIsAlone(initiator);
    if (mode == ChoiceMode::Initiator || mutations().choiceInitiatorDecides || alone || (mode == ChoiceMode::Host && initiator == kHostPlayer)) {
        ++stats_.choicesAccepted;
        return StageDecision::Proceed;
    }
    Pending p;
    p.kind = Pending::Choice; p.quest = ci->second.quest; p.info = choice; p.initiator = initiator; p.timeoutMs = ci->second.timeoutMs;
    startVote(std::move(p));
    return StageDecision::Deferred;
}

bool RulesEngine::readyQuest(const FormRef& info, FormRef& quest) const {
    if (mutations().readyInfoUnmapped) return false;
    auto it = readyQuests_.find(info);
    if (it != readyQuests_.end()) { quest = it->second; return true; }
    return choiceQuest(info, quest);
}

StageDecision RulesEngine::onInfoResult(const FormRef& info, PlayerId initiator) {
    FormRef quest;
    if (mutations().readyInfoUnmapped) return StageDecision::Proceed;
    auto rq = readyQuests_.find(info);
    if (rq != readyQuests_.end()) return onReadyRequested(rq->second, info, initiator);
    if (isChoice(info)) return onChoiceRequested(info, initiator);
    return StageDecision::Proceed;
}

// ---- guest stage intents as passes ----------------------------------------------------------------------------------------------------------

bool RulesEngine::isPassStage(const FormRef& quest, uint32_t stage) const {
    const QuestRules* q = rulesFor(quest);
    return q && q->findFor("Barrier", stage) != nullptr;
}

StageDecision RulesEngine::onGuestStageIntent(PlayerId slot, const FormRef& quest, uint32_t stage) {
    if (mutations().rulesDisabled || mutations().passIntentIgnored || !isPassStage(quest, stage)) return StageDecision::Proceed;
    ++stats_.passIntents;
    const StageDecision d = onStageRequested(quest, stage, slot);   // idempotent: an already pending barrier answers Deferred
    onPlayerPassed(slot, quest);
    return d == StageDecision::Deferred ? StageDecision::Deferred : StageDecision::Proceed;
}

// ---- FriendlyFireGuard --------------------------------------------------------------------------------------------------------------------------

RulesEngine::FfVerdict RulesEngine::onProtectedHit(const FormRef& victim, PlayerId attacker, bool splash, uint64_t nowMs) {
    auto pr = protected_.find(victim);
    if (pr == protected_.end()) return FfVerdict::NotProtected;
    if (mutations().friendlyFireOff || mutations().rulesDisabled) return FfVerdict::Allow;
    const Rule& r = *pr->second;
    const uint64_t window = static_cast<uint64_t>(r.number("window_s", 10) * 1000.0);
    const size_t need = static_cast<size_t>(r.number("deliberate_hits", 3));
    FfState& st = ff_[{attacker, victim}];
    if (st.deliberate) { ++stats_.friendlyFireAllowed; return FfVerdict::Allow; }
    if (!splash) {
        st.hits.push_back(nowMs);
        while (!st.hits.empty() && nowMs > window && st.hits.front() < nowMs - window) st.hits.erase(st.hits.begin());
        if (st.hits.size() >= need) { st.deliberate = true; ++stats_.friendlyFireAllowed; return FfVerdict::Allow; }   // meant it
    }
    ++stats_.friendlyFireBlocked;
    const bool warn = !st.warned && r.scalar("warn", "true") != "false";
    st.warned = true;
    return warn ? FfVerdict::BlockWarn : FfVerdict::Block;
}

void RulesEngine::confirmDeliberate(const FormRef& victim, PlayerId attacker) {
    if (!protected_.count(victim)) return;
    ff_[{attacker, victim}].deliberate = true;
}

// ---- death, downed state and late joiners ---------------------------------------------------------------------------------------------------------

std::string RulesEngine::policy(const std::string& key, const std::string& dflt) const {
    auto it = opt_.policy.find(key);
    if (it != opt_.policy.end()) return it->second;
    return db_.policy(key, dflt);
}

RulesEngine::DeathVerdict RulesEngine::onPlayerDied(PlayerId slot, uint64_t nowMs) {
    DeathVerdict v;
    if (slot >= kMaxSlots || mutations().deathPolicyOff || mutations().rulesDisabled) return v;
    const std::string mode = policy("death.mode", "downed");
    if (mode == "normal") return v;
    v.hasCheckpoint = zone(v.checkpoint);
    downed_[slot] = nowMs;
    ++stats_.playersDowned;
    bool collar = false;
    if (auto ck = collarKilled_.find(slot); ck != collarKilled_.end()) { collar = nowMs <= ck->second + 30000 && !mutations().collarDownTimed; collarKilled_.erase(ck); }   // a stale drain must not label a later death
    const uint8_t joined = joinedMask();
    bool all = true;
    for (unsigned s = 0; s < kMaxSlots; ++s) if ((joined >> s & 1u) && !downed_.count(PlayerId(s))) all = false;
    if (all) {   // the whole party is down: nobody is left to revive anybody
        downed_.clear();
        holdDowned_.clear();
        if (policy("death.all_down", "reload-host-save") == "reload-host-save") {
            v.kind = DeathVerdict::ReloadSave;
            ++stats_.saveReloads;
            if (reloadHostSave) reloadHostSave();
            return v;
        }
        downed_[slot] = nowMs;   // all_down=respawn: they respawn one by one after the revive window
    }
    v.kind = DeathVerdict::Downed;
    if (collar) { holdDowned_.insert(slot); v.untilCheckpoint = true; v.respawnAfterMs = 0; ++stats_.checkpointDowns; }   // D9: down until the party reaches a checkpoint
    else if (mode == "respawn") v.respawnAfterMs = 0;   // tick() respawns it at once
    else {
        const double ms = std::strtod(policy("death.revive_s", "60").c_str(), nullptr) * 1000.0;
        v.respawnAfterMs = static_cast<uint32_t>(ms < 0 ? 0 : ms);
    }
    return v;
}

void RulesEngine::onPlayerRevived(PlayerId slot) { downed_.erase(slot); holdDowned_.erase(slot); collarKilled_.erase(slot); }

bool RulesEngine::sceneActive() const {
    for (auto& p : pending_) if (p.kind == Pending::Stage) return true;   // a barrier or a tether is holding a stage
    return !locks_.empty() || !slideshowHold_.empty();
}

RulesEngine::JoinVerdict RulesEngine::onPlayerJoining(PlayerId slot) {
    JoinVerdict v;
    if (slot >= kMaxSlots || mutations().deathPolicyOff || mutations().rulesDisabled) return v;
    if (sceneActive() && policy("late_join.during_scene", "wait") == "wait") {
        v.kind = JoinVerdict::WaitForScene;
        ++stats_.lateJoinHolds;
        return v;
    }
    if (hasZone_ && policy("late_join.after_scene", "warp-to-zone") == "warp-to-zone") {
        v.kind = JoinVerdict::WarpToZone;
        v.zone = zone_;
        ++stats_.lateJoinWarps;
    }
    return v;
}

// ---- G45 PooledItems: a hand-in of N items that the party holds together is charged to the players who hold them ----------------------------------------------------
// The sum reducer lets a quest see the pooled count; the RemoveItem that follows (the stage result or INFO end script of the hand-in) names N of the item with no slot, and
// would charge every player N (QuestItemDup) or only the initiator. Split it over the party instead: the initiator first, then the slots in order, each up to what it holds.
void RulesEngine::expandPooled(const QuestRules& q, EffectTxn& t) {
    if (mutations().pooledDebitOnOrigin || !opt_.itemCount) return;
    std::vector<Effect> out;
    bool changed = false;
    for (auto& e : t.effects) {
        auto* ri = std::get_if<RemoveItem>(&e);
        bool pooled = false;
        if (ri && ri->to.mode == Recipient::Default && ri->count > 0)
            for (auto& r : q.rules) if (r.kind == "PooledItems" && listedForm(r, "items", ri->item)) pooled = true;
        if (!pooled) { out.push_back(e); continue; }
        int64_t left = ri->count;
        const uint8_t joined = joinedMask();
        std::vector<PlayerId> order;
        if (t.origin < kMaxSlots && (joined >> t.origin & 1u)) order.push_back(t.origin);
        for (unsigned s = 0; s < kMaxSlots; ++s) if ((joined >> s & 1u) && PlayerId(s) != t.origin) order.push_back(PlayerId(s));
        std::vector<RemoveItem> parts;
        for (PlayerId s : order) {
            if (left <= 0) break;
            int64_t held = 0;
            if (!opt_.itemCount(s, ri->item, held) || held <= 0) continue;
            const int64_t take = held < left ? held : left;
            parts.push_back(RemoveItem{ri->item, static_cast<int32_t>(take), Recipient::slot(s)});
            left -= take;
        }
        if (left > 0) parts.push_back(RemoveItem{ri->item, static_cast<int32_t>(left), t.origin < kMaxSlots ? Recipient::slot(t.origin) : Recipient::origin()});   // not enough pooled: the rest stays with the initiator (the apply clamps)
        for (auto& p : parts) out.push_back(p);
        ++stats_.pooledDebits;
        changed = true;
    }
    if (changed) t.effects = std::move(out);
}

// ---- G44 OncePerEvent (D7): the clock is the host's; a time leap that a scripted door applies on every activation happens once per party event ------------------------------
void RulesEngine::dropRepeatedEvent(const QuestRules& q, EffectTxn& t) {
    const Rule* r = q.find("OncePerEvent");
    const uint64_t windowMs = static_cast<uint64_t>(r->number("window_s", 600) * 1000.0);
    auto matches = [&](const Effect& e) {
        if (auto* g = std::get_if<SetGlobal>(&e)) {
            auto ef = r->lists.find("effects");
            const bool all = ef != r->lists.end() && std::find(ef->second.begin(), ef->second.end(), "SetGlobal") != ef->second.end();
            return all || listedForm(*r, "globals", g->global);
        }
        if (std::holds_alternative<AdvanceTime>(e)) {
            auto ef = r->lists.find("effects");
            return ef != r->lists.end() && std::find(ef->second.begin(), ef->second.end(), "AdvanceTime") != ef->second.end();
        }
        return false;
    };
    bool any = false;
    for (auto& e : t.effects) any |= matches(e);
    if (!any) return;
    const FormRef key = t.cause.source;
    auto it = onceEvents_.find(key);
    const bool active = it != onceEvents_.end() && now_ < it->second.first + windowMs;
    if (!active || (it->second.second == t.origin && now_ <= it->second.first + 1000)) {   // the first commit of the event (or the rest of the same script run)
        if (!active) onceEvents_[key] = {now_, t.origin};
        return;
    }
    const size_t before = t.effects.size();
    t.effects.erase(std::remove_if(t.effects.begin(), t.effects.end(), matches), t.effects.end());
    stats_.onceEventsDropped += before - t.effects.size();
}

std::vector<std::string> RulesEngine::perPlayerScripts() const {
    std::vector<std::string> v;
    if (mutations().perPlayerScriptsIgnored) return v;
    for (auto& [edid, q] : db_.quests())
        for (auto& r : q.rules) {
            if (r.kind != "PerPlayerState") continue;
            auto it = r.lists.find("scripts");
            if (it != r.lists.end()) for (auto& n : it->second) if (std::find(v.begin(), v.end(), n) == v.end()) v.push_back(n);
        }
    return v;
}

bool RulesEngine::runsPerPlayer(const std::string& scriptEdid) const {
    for (auto& n : perPlayerScripts()) {
        if (n.size() != scriptEdid.size()) continue;
        bool same = true;
        for (size_t i = 0; i < n.size() && same; ++i) same = std::tolower(static_cast<unsigned char>(n[i])) == std::tolower(static_cast<unsigned char>(scriptEdid[i]));
        if (same) return true;
    }
    return false;
}

std::vector<std::string> RulesEngine::perPlayerObjVarNames(const FormRef& quest) const {
    std::vector<std::string> v;
    if (mutations().perPlayerScriptsIgnored) return v;
    const QuestRules* q = rulesFor(quest);
    if (!q) return v;
    for (auto& r : q->rules) {
        if (r.kind != "PerPlayerState") continue;
        auto it = r.lists.find("objvars");
        if (it != r.lists.end()) for (auto& n : it->second) if (std::find(v.begin(), v.end(), n) == v.end()) v.push_back(n);
    }
    return v;
}

void RulesEngine::filter(EffectTxn& t) {
    if (mutations().rulesDisabled) return;
    // G02: a quest own script, an INFO or a target ref commits txns whose cause.source is NOT the quest; map it back before any rule is looked up.
    if (!rulesFor(t.cause.source)) {
        FormRef owner;
        if (questOfCause(t.cause, owner) && !(owner == t.cause.source)) { t.cause.source = owner; ++stats_.causesResolved; }
    }
    // G24: a run the plugin does on behalf of a player (beginActing) belongs to that player.
    if (acting_ != kNoPlayer && t.cause.source == actingQuest_) t.origin = acting_;
    // ExclusiveStages: alternative outcomes (rescued / failed, three endings) are not progress. The first one committed wins in host order;
    // a later outcome of the same set is dropped (stage merge alone would let the numerically highest win).
    for (auto it = t.effects.begin(); it != t.effects.end();) {
        auto* ss = std::get_if<SetStage>(&*it);
        const QuestRules* sq = ss ? rulesFor(ss->quest) : nullptr;
        const Rule* ex = sq ? sq->findFor("ExclusiveStages", ss->stage) : nullptr;
        if (!ex || mutations().exclusiveLastWins) { ++it; continue; }
        auto w = exclusiveWinner_.find(ss->quest);
        if (w == exclusiveWinner_.end()) {
            auto st = host_.state().stages.find(ss->quest);   // an outcome committed before this engine started counts
            if (st != host_.state().stages.end() && ex->appliesToStage(st->second)) w = exclusiveWinner_.emplace(ss->quest, st->second).first;
        }
        if (w != exclusiveWinner_.end() && w->second != ss->stage) { it = t.effects.erase(it); ++stats_.exclusiveRejected; continue; }
        if (w == exclusiveWinner_.end()) exclusiveWinner_[ss->quest] = ss->stage;
        ++it;
    }
    // The ending slideshow releases the controls it held since the barrier.
    if (!slideshowHold_.empty() && !host_.state().slideshowEnded) {
        bool ends = false;
        for (auto& e : t.effects) ends |= std::holds_alternative<SlideshowEnd>(e);
        if (ends) {
            slideshowHold_.clear();
            t.effects.push_back(SetPlayerControls{kAllControls, true, Recipient::all()});
        }
    }
    const QuestRules* q = rulesFor(t.cause.source);
    if (!q) return;
    if (runsOnEveryMachine(*q) && !mutations().guestLocalReplicated) {
        // Per-machine scripts (achievements, rumble, DLC setup, fades) run locally on every machine: nothing of theirs is replicated.
        t.effects.clear();
        ++stats_.guestLocalDropped;
        return;
    }
    if (q->find("InitiatorDialogue") && t.cause.kind == CauseKind::DialogueResult && dialogueInitiator_ != kNoPlayer &&
        t.origin != dialogueInitiator_) {
        t.effects.clear();   // not your conversation: only the initiator's choices advance it
        ++stats_.dialogueRejected;
        return;
    }
    if (q->find("PartyWarp") && opt_.playerRef) {
        std::vector<Effect> out;
        const Rule* warpRule = q->find("PartyWarp");
        const double warpRange = mutations().warpRangeIgnored ? 0.0 : warpRule->number("range", 0);   // D10: the Teleporter takes the party members within `range` of the firer
        for (auto& e : t.effects) {
            auto* mv = std::get_if<MoveRef>(&e);
            bool isPlayer = false;
            unsigned firer = kMaxSlots;
            if (mv) for (unsigned s = 0; s < kMaxSlots && !isPlayer; ++s) { FormRef pr; isPlayer = opt_.playerRef(PlayerId(s), pr) && pr == mv->ref; if (isPlayer) firer = s; }
            if (!isPlayer) { out.push_back(e); continue; }
            ++stats_.warpsExpanded;
            uint8_t joined = joinedMask();
            unsigned k = 0;
            for (unsigned s = 0; s < kMaxSlots; ++s) {   // absent players are held where they are
                if (!(joined >> s & 1u)) continue;
                FormRef pr;
                if (!opt_.playerRef(PlayerId(s), pr)) continue;
                if (warpRange > 0 && s != firer) {
                    float d = 0;
                    if (!opt_.playerDistance || !opt_.playerDistance(PlayerId(firer), PlayerId(s), d) || d > warpRange) { ++stats_.rangeWarpsHeld; continue; }   // out of range (or unknown): stays
                }
                MoveRef m = *mv;
                m.ref = pr;
                m.x += opt_.warpSpacing * static_cast<float>(k++);
                m.partyWarp = true;
                out.push_back(m);
            }
        }
        t.effects = std::move(out);
    }
    const bool actingCause = t.cause.kind == CauseKind::DialogueResult || t.cause.kind == CauseKind::Intent || t.cause.kind == CauseKind::Loot ||
                             t.cause.kind == CauseKind::Console || t.cause.kind == CauseKind::Combat;
    const bool rewardAll = q->find("RewardAll") != nullptr;
    const bool itemDup = q->find("QuestItemDup") != nullptr;
    const bool perOwner = q->find("RemoveItemPerOwner") != nullptr;
    const bool shareKR = q->find("ShareKarmaRep") != nullptr;
    const Rule* ctrlRule = t.cause.kind == CauseKind::QuestResult ? q->findFor("SetPlayerControls", static_cast<uint32_t>(t.cause.aux)) : nullptr;
    if (!ctrlRule) ctrlRule = q->find("SetPlayerControls");
    const bool ctrlAll = ctrlRule != nullptr;
    const bool ctrlInitiator = ctrlRule && ctrlRule->scalar("scope", "all") == "initiator" && !mutations().controlsScopeIgnored;
    const PlayerId lockWho = ctrlInitiator ? t.origin : kNoPlayer;
    const bool perScene = (q->find("PerPlayerScene") != nullptr || q->find("PerPlayerState") != nullptr) && !mutations().perPlayerSceneShared;   // the scene belongs to the acting player
    if (q->find("PerPlayerState") && opt_.playerRef && !mutations().perPlayerStateKillsAll) {
        // G42: player.kill in a per-player state machine (collar, cloud, gas trap) kills THAT player: the KillActor of a player ref carries no slot, so it becomes a
        // health drain addressed to the slot whose ref it names (every other machine sees nothing).
        for (auto& e : t.effects) {
            auto* ka = std::get_if<KillActor>(&e);
            if (!ka) continue;
            for (unsigned s = 0; s < kMaxSlots; ++s) {
                FormRef pr;
                if (opt_.playerRef(PlayerId(s), pr) && pr == ka->actor) {
                    e = ModActorValue{kAvHealth, -kKillDrain, Recipient::slot(PlayerId(s))}; ++stats_.killsRerouted;
                    const Rule* ps = q->find("PerPlayerState");
                    if (ps && ps->scalar("down") == "until-checkpoint") collarKilled_[PlayerId(s)] = now_;   // the death that follows is a collar death (D9)
                    break;
                }
            }
        }
    }
    if (q->find("PooledItems")) expandPooled(*q, t);
    if (q->find("OncePerEvent") && !mutations().oncePerEventOff) dropRepeatedEvent(*q, t);
    const bool saveRule = q->find("SaveSuppress") != nullptr;
    bool lockedNow = false, unlockedNow = false;
    for (auto& e : t.effects) {
        std::visit([&](auto& x) {
            using T = std::decay_t<decltype(x)>;
            auto setTo = [&](Recipient& to, Recipient r) {
                if (to.mode == Recipient::Default) { to = r; ++stats_.recipientRewrites; }
            };
            auto scene = [&](Recipient& to) {
                if (to.mode != Recipient::Default) return;
                to = Recipient::origin();
                ++stats_.recipientRewrites; ++stats_.perPlayerRouted;
            };
            // An explicit sharing rule of the quest (RewardAll, QuestItemDup, RemoveItemPerOwner, ShareKarmaRep) wins; otherwise a
            // PerPlayerScene keeps the scene's effects with the acting player instead of the global quest-reward default.
            if constexpr (std::is_same_v<T, AddItem>) {
                if (rewardAll || itemDup) setTo(x.to, Recipient::all());
                else if (perScene) scene(x.to);
            } else if constexpr (std::is_same_v<T, ModActorValue>) {
                if (rewardAll) setTo(x.to, Recipient::all());
                else if (perScene) scene(x.to);
            } else if constexpr (std::is_same_v<T, RemoveItem>) {
                if (perOwner) setTo(x.to, Recipient::origin());
                else if (itemDup) setTo(x.to, Recipient::all());
                else if (perScene) scene(x.to);
            } else if constexpr (std::is_same_v<T, ModKarma> || std::is_same_v<T, ModReputation>) {
                if (shareKR) setTo(x.to, Recipient::all());
                else if (perScene) scene(x.to);
            } else if constexpr (std::is_same_v<T, SetPerk>) {
                Recipient pr;
                if (x.to.mode == Recipient::Default && perkRecipient(*q, x.perk, actingCause, pr)) { x.to = pr; ++stats_.recipientRewrites; ++stats_.perkRoutes; }
                else if (rewardAll) setTo(x.to, Recipient::all());
                else if (perScene) scene(x.to);
            } else if constexpr (std::is_same_v<T, SetPlayerControls>) {
                if (ctrlAll) {
                    setTo(x.to, ctrlInitiator ? Recipient::origin() : Recipient::all());
                    auto key = std::make_pair(t.cause.source, lockWho);
                    if (!x.enable) {
                        Lock& l = locks_[key];
                        if (l.mask == 0) { l.since = now_; ++stats_.locksTracked; }
                        l.mask |= x.controls;
                        lockedNow = true;
                    } else {
                        auto lk = locks_.find(key);
                        if (lk != locks_.end()) {
                            lk->second.mask &= ~x.controls;
                            if (lk->second.mask == 0) { locks_.erase(lk); if (!hasLock(t.cause.source)) unlockedNow = true; }
                        } else if (!hasLock(t.cause.source)) {
                            unlockedNow = true;   // an enable with nothing held (the old behaviour): still releases the save block
                        }
                        const uint32_t keep = heldByOthers(t.cause.source, lockWho);   // a release frees only this lock own bits
                        if (x.controls & keep) { x.controls &= ~keep; ++stats_.controlsHeldBack; }
                    }
                }
            } else if constexpr (std::is_same_v<T, CompanionOwner>) {
                if (const Rule* co = q->find("CompanionOwner")) {
                    if (co->scalar("owner", "recruiter") == "party" && !mutations().partyOwnerIgnored &&
                        (!co->lists.count("npcs") || listedForm(*co, "npcs", x.npc))) {
                        if (x.active) partyCompanions_.insert(x.npc); else partyCompanions_.erase(x.npc);   // the whole party owns it; the wire keeps the recruiter slot
                    }
                    x.owner = t.origin;   // owned by the recruiter
                }
            }
        }, e);
    }
    if (q->find("FastTravelLock")) {
        bool want = desiredTravelLock(t.cause.source), have = travelLocked_.count(t.cause.source) > 0;
        if (want != have) {
            if (want) travelLocked_.insert(t.cause.source); else travelLocked_.erase(t.cause.source);
            ++stats_.travelLockChanges;
            t.effects.push_back(SetFastTravelLock{t.cause.source, want});
        }
    }
    // a release that only had bits another source still holds frees nothing
    t.effects.erase(std::remove_if(t.effects.begin(), t.effects.end(), [](const Effect& e) {
        auto* sc = std::get_if<SetPlayerControls>(&e);
        return sc && sc->enable && sc->controls == 0;
    }), t.effects.end());
    // a scene that locks controls also blocks saving until it releases (or times out)
    if (saveRule && lockedNow && !unlockedNow) t.effects.push_back(SaveSuppress{true});
    if (saveRule && unlockedNow && !lockedNow) t.effects.push_back(SaveSuppress{false});
    // SlideshowEnd: the host runs it once
    if (q->find("SlideshowEnd") && host_.state().slideshowEnded)
        t.effects.erase(std::remove_if(t.effects.begin(), t.effects.end(), [](const Effect& e) { return std::holds_alternative<SlideshowEnd>(e); }), t.effects.end());
}

}  // namespace fnvcoop
