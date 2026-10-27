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
    quests_ = std::move(out);
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
                                                  "PerPlayerVars", "ExclusiveStages", "ReadyCheck"};
    for (auto& [edid, q] : db_.quests()) {
        FormRef f;
        if (resolve_ && resolve_(edid, f)) byForm_[f] = &q;
        for (auto& r : q.rules) {
            if (!handled.count(r.kind)) unsupported_.insert(r.kind);
            if (r.kind == "ReadyCheck") {
                auto il = r.lists.find("infos");
                if (il != r.lists.end()) for (auto& txt : il->second) { FormRef f; if (parseFormText(txt, f)) readyInfos_.insert(f); }
            }
        }
    }
    host_.txnFilter = [this](EffectTxn& t) { filter(t); };
}

RulesEngine::~RulesEngine() { host_.txnFilter = nullptr; }

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

uint8_t RulesEngine::joinedMask() const {
    uint8_t m = 1u << kHostPlayer;
    for (auto& c : host_.clients()) if ((c.joined || mutations().departedHoldBarrier) && c.slot < 8) m = static_cast<uint8_t>(m | (1u << c.slot));
    return m;
}

bool RulesEngine::tetherSatisfied(const std::set<FormRef>& cells, const FormRef& anchor, bool hasAnchor) const {
    if (cells.empty() && !hasAnchor) return true;
    if (hasAnchor && cells.empty() && mutations().tetherIgnoresAnchor) return true;
    uint8_t m = joinedMask();
    for (unsigned s = 0; s < kMaxSlots; ++s) {
        if (!(m >> s & 1u)) continue;
        auto it = cellOf_.find(PlayerId(s));
        if (it != cellOf_.end() && cells.count(it->second)) continue;
        auto nr = nearOf_.find(PlayerId(s));
        if (hasAnchor && !mutations().tetherIgnoresAnchor && nr != nearOf_.end() && nr->second.count(anchor)) continue;
        return false;
    }
    return true;
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
        std::set<FormRef> cells;
        auto it = tr->lists.find("cells");
        if (it != tr->lists.end())
            for (auto& edid : it->second) { FormRef f; if (resolve_ && resolve_(edid, f)) cells.insert(f); }
        FormRef anchor;
        bool hasAnchor = false;
        const std::string anchorEdid = tr->scalar("anchor", tr->scalar("anchor_ref"));
        if (!anchorEdid.empty() && resolve_ && resolve_(anchorEdid, anchor)) hasAnchor = true;
        if (tetherSatisfied(cells, anchor, hasAnchor)) return StageDecision::Proceed;
        Pending p;
        p.quest = quest; p.stage = stage; p.initiator = initiator; p.tether = true; p.since = now_; p.cells = cells;
        p.anchor = anchor; p.hasAnchor = hasAnchor;
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
        if (p.kind != Pending::Stage) {   // a consent vote: keyed by the INFO (ReadyCheck) or by timeSkipId() (wait / sleep)
            if ((p.kind == Pending::Ready ? p.info : timeSkipId()) != ref) continue;
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
    if (ref == timeSkipId() || readyInfos_.count(ref)) return;   // a consent for a vote nobody asked for must not count for a later one
    earlyPasses_[ref].insert(slot);   // nobody asked for this barrier yet
}

void RulesEngine::onCellReported(PlayerId slot, const FormRef& cell) {
    cellOf_[slot] = cell;
    checkPending();
}

void RulesEngine::onNearReported(PlayerId slot, const FormRef& anchor, bool near) {
    if (slot >= kMaxSlots) return;
    if (near) nearOf_[slot].insert(anchor); else nearOf_[slot].erase(anchor);
    checkPending();
}

bool RulesEngine::handleAction(PlayerId slot, IntentKind kind, const FormRef& ref) {
    if (kind == IntentKind::ScenePass) { onPlayerPassed(slot, ref); return true; }
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
        if (!p.cells.empty() || p.hasAnchor) host_.commit(kHostPlayer, c, {PartyTether{p.cells.empty() ? p.anchor : *p.cells.begin(), false}});
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

bool RulesEngine::desiredTravelLock(const FormRef& quest) const {
    if (locks_.count(quest)) return true;
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
        const uint32_t limit = pending_[i].kind == Pending::Stage ? opt_.barrierTimeoutMs : pending_[i].timeoutMs;
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
            FormRef quest = it->first;
            uint32_t mask = it->second.mask;
            it = locks_.erase(it);
            Cause c; c.kind = CauseKind::Script; c.source = quest;
            std::vector<Effect> eff{SetPlayerControls{mask, true, Recipient::all()}};
            if (auto* q = rulesFor(quest); q && q->find("SaveSuppress")) eff.push_back(SaveSuppress{false});
            host_.commit(kHostPlayer, c, std::move(eff));
            syncTravelLock(quest);
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
        if (r.kind != "PerPlayerVars") continue;
        auto it = r.lists.find("vars");
        if (it != r.lists.end() && std::find(it->second.begin(), it->second.end(), var) != it->second.end()) return true;
    }
    return false;
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

// Host-side rules state worth keeping across a host restart: per-slot var copies and the ExclusiveStages winners.
Bytes RulesEngine::saveState() const {
    Archive a = Archive::writer();
    uint32_t magic = 0x53525246; a.u32(magic);   // 'FRRS'
    size_t n = playerVars_.size(); a.count(n, 1u << 20);
    for (auto& [k, val] : playerVars_) {
        FormRef q = std::get<0>(k); std::string var = std::get<1>(k); uint8_t sl = std::get<2>(k); double v = val;
        io(a, q); a.str(var, 255); a.u8(sl); a.f64(v);
    }
    size_t m = exclusiveWinner_.size(); a.count(m, 1u << 20);
    for (auto& [q, st] : exclusiveWinner_) { FormRef f = q; uint32_t s = st; io(a, f); a.varu32(s); }
    return a.finish();
}

bool RulesEngine::loadState(const Bytes& b) {
    Archive a = Archive::reader(b.data(), b.size());
    uint32_t magic = 0; a.u32(magic);
    std::map<std::tuple<FormRef, std::string, PlayerId>, double> vars;
    std::map<FormRef, uint32_t> winners;
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
    if (!a.ok() || !a.atEnd() || magic != 0x53525246) return false;
    playerVars_ = std::move(vars);
    exclusiveWinner_ = std::move(winners);
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

void RulesEngine::filter(EffectTxn& t) {
    if (mutations().rulesDisabled) return;
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
        for (auto& e : t.effects) {
            auto* mv = std::get_if<MoveRef>(&e);
            bool isPlayer = false;
            if (mv) for (unsigned s = 0; s < kMaxSlots && !isPlayer; ++s) { FormRef pr; isPlayer = opt_.playerRef(PlayerId(s), pr) && pr == mv->ref; }
            if (!isPlayer) { out.push_back(e); continue; }
            ++stats_.warpsExpanded;
            uint8_t joined = joinedMask();
            unsigned k = 0;
            for (unsigned s = 0; s < kMaxSlots; ++s) {   // absent players are held where they are
                if (!(joined >> s & 1u)) continue;
                FormRef pr;
                if (!opt_.playerRef(PlayerId(s), pr)) continue;
                MoveRef m = *mv;
                m.ref = pr;
                m.x += opt_.warpSpacing * static_cast<float>(k++);
                m.partyWarp = true;
                out.push_back(m);
            }
        }
        t.effects = std::move(out);
    }
    const bool rewardAll = q->find("RewardAll") != nullptr;
    const bool itemDup = q->find("QuestItemDup") != nullptr;
    const bool perOwner = q->find("RemoveItemPerOwner") != nullptr;
    const bool shareKR = q->find("ShareKarmaRep") != nullptr;
    const bool ctrlAll = q->find("SetPlayerControls") != nullptr;
    const bool perScene = q->find("PerPlayerScene") != nullptr && !mutations().perPlayerSceneShared;   // the scene belongs to the acting player
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
                if (rewardAll) setTo(x.to, Recipient::all());
                else if (perScene) scene(x.to);
            } else if constexpr (std::is_same_v<T, SetPlayerControls>) {
                if (ctrlAll) {
                    setTo(x.to, Recipient::all());
                    Lock& l = locks_[t.cause.source];
                    if (!x.enable) {
                        if (l.mask == 0) { l.since = now_; ++stats_.locksTracked; }
                        l.mask |= x.controls;
                        lockedNow = true;
                    } else {
                        l.mask &= ~x.controls;
                        if (l.mask == 0) { locks_.erase(t.cause.source); unlockedNow = true; }
                    }
                }
            } else if constexpr (std::is_same_v<T, CompanionOwner>) {
                if (q->find("CompanionOwner")) x.owner = t.origin;   // owned by the recruiter
            } else if constexpr (std::is_same_v<T, ContainerTransfer>) {
                if (perScene) scene(x.to);   // a character-creation strip (CG03's age jumps) stays with the acting player
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
    // a scene that locks controls also blocks saving until it releases (or times out)
    if (saveRule && lockedNow && !unlockedNow) t.effects.push_back(SaveSuppress{true});
    if (saveRule && unlockedNow && !lockedNow) t.effects.push_back(SaveSuppress{false});
    // SlideshowEnd: the host runs it once
    if (q->find("SlideshowEnd") && host_.state().slideshowEnded)
        t.effects.erase(std::remove_if(t.effects.begin(), t.effects.end(), [](const Effect& e) { return std::holds_alternative<SlideshowEnd>(e); }), t.effects.end());
}

}  // namespace fnvcoop
