// Party saves, game-free half. See party_save.h and docs/party-save.md. Twins (FNVCOOP_MUT=n, tests/test_party_save.cpp): 1100..1110.
#include "party_save.h"

#include <algorithm>
#include <cctype>
#include <cstdio>
#include <cstdlib>
#include <cstring>

namespace nvcoop::coop::party {

namespace {
using nvcoop::json::Value;

std::string lower(std::string s) {
    for (char& c : s) c = static_cast<char>(std::tolower(static_cast<unsigned char>(c)));
    return s;
}
bool isHex(char c) { return std::isxdigit(static_cast<unsigned char>(c)) != 0; }

// "Co-op ab12cd " (and a "Host " marker after it) is what this code puts in front of a name.
size_t partyPrefixLen(const std::string& n) {
    if (n.size() < 13 || n.compare(0, 6, "Co-op ") != 0 || n[12] != ' ') return 0;
    for (size_t i = 6; i < 12; ++i)
        if (!isHex(n[i])) return 0;
    size_t len = 13;
    if (n.compare(len, 5, "Host ") == 0) len += 5;
    return len;
}

std::string clean1(const std::string& s) {   // tabs and line breaks never go into the index or the sidecars
    std::string o = s;
    for (char& c : o) if (c == '\t' || c == '\r' || c == '\n') c = ' ';
    return o;
}
}  // namespace

// ---- campaign id and names ---------------------------------------------------------------------------------------------------------------------
std::string campaignHex(uint64_t id) {
    char b[24];
    std::snprintf(b, sizeof b, "%016llx", static_cast<unsigned long long>(id));
    return b;
}
bool parseCampaign(const std::string& t, uint64_t* id) {
    if (t.size() != 16) return false;
    for (char c : t) if (!isHex(c)) return false;
    const uint64_t v = std::strtoull(t.c_str(), nullptr, 16);
    if (!v) return false;
    if (id) *id = v;
    return true;
}
std::string shortCampaign(uint64_t id) { return campaignHex(id).substr(0, 6); }

std::string saveBaseName(const std::string& p) {
    size_t s = p.find_last_of("\\/");
    std::string n = s == std::string::npos ? p : p.substr(s + 1);
    if (n.size() >= 4 && lower(n.substr(n.size() - 4)) == ".fos") n.resize(n.size() - 4);
    while (!n.empty() && n.back() == ' ') n.pop_back();
    size_t b = 0;
    while (b < n.size() && n[b] == ' ') ++b;
    return n.substr(b);
}

std::string cleanName(const std::string& raw, size_t maxLen) {
    std::string o;
    for (char c : raw) {
        const unsigned char u = static_cast<unsigned char>(c);
        o += (u < 32 || u == 127 || std::strchr("\\/:*?\"<>|", c)) ? '_' : c;
    }
    size_t b = 0;
    while (b < o.size() && o[b] == ' ') ++b;
    o = o.substr(b);
    if (o.size() > maxLen) o.resize(maxLen);
    while (!o.empty() && (o.back() == ' ' || o.back() == '.')) o.pop_back();
    return o;
}

std::string stripPartyPrefix(const std::string& name) {
    if (FNV_MUT(1106)) return name;   // MUTANT 1106: a name made here is wrapped again ("Co-op a Co-op a Save 4")
    return name.substr(partyPrefixLen(name));
}

static std::string partyName(uint64_t campaign, const std::string& base, const char* marker) {
    std::string b = cleanName(stripPartyPrefix(saveBaseName(base)), kMaxNamePart);
    if (b.empty()) b = "save";
    return "Co-op " + shortCampaign(campaign) + " " + marker + b;
}
std::string guestSaveName(uint64_t campaign, const std::string& hostSave) { return partyName(campaign, hostSave, ""); }
// The "Host " marker keeps the host's save apart from the save another guest makes for it when the machines share one save folder (two game copies on a PC).
std::string hostSaveNameForRequest(uint64_t campaign, const std::string& guestSave) { return partyName(campaign, guestSave, "Host "); }
bool sameName(const std::string& a, const std::string& b) { return lower(a) == lower(b); }

// ---- sidecar -------------------------------------------------------------------------------------------------------------------------------------
std::string metaText(const Meta& m) {
    std::string t = std::string("FNVCOOP-PARTY 1\ncampaign=") + campaignHex(m.campaign) + "\nrole=" + (m.host ? "host" : "guest") + "\nhostSave=" + clean1(m.hostSave) + "\n";
    if (!FNV_MUT(1260)) t += nvcoop::rebalance::sidecarLines(m.settings);   // MUTANT 1260: the campaign's rebalance setting is not kept with the save
    return t;
}
bool parseMeta(const std::string& text, Meta* out) {
    Meta m;
    size_t pos = 0;
    bool first = true, haveCampaign = false;
    while (pos <= text.size()) {
        size_t e = text.find('\n', pos);
        if (e == std::string::npos) e = text.size();
        std::string line = text.substr(pos, e - pos);
        pos = e + 1;
        while (!line.empty() && (line.back() == '\r' || line.back() == ' ')) line.pop_back();
        if (line.empty()) continue;
        if (first) {
            if (line != "FNVCOOP-PARTY 1") return false;
            first = false;
            continue;
        }
        const size_t eq = line.find('=');
        if (eq == std::string::npos) continue;
        const std::string k = line.substr(0, eq), v = line.substr(eq + 1);
        if (k == "campaign") haveCampaign = parseCampaign(v, &m.campaign);
        else if (k == "role") m.host = v == "host";
        else if (k == "hostSave") m.hostSave = v;
        else nvcoop::rebalance::applySidecarKey(m.settings, k, v);
    }
    if (first || !haveCampaign) return false;
    if (out) *out = m;
    return true;
}

// ---- index ---------------------------------------------------------------------------------------------------------------------------------------
Index Index::parse(const std::string& text) {
    Index ix;
    size_t pos = 0;
    bool first = true;
    while (pos <= text.size()) {
        size_t e = text.find('\n', pos);
        if (e == std::string::npos) e = text.size();
        std::string line = text.substr(pos, e - pos);
        pos = e + 1;
        while (!line.empty() && line.back() == '\r') line.pop_back();
        if (first) {
            if (line != "FNVCOOP-PARTY-INDEX 1") return ix;
            first = false;
            continue;
        }
        std::vector<std::string> f;
        size_t b = 0;
        for (;;) {
            size_t t = line.find('\t', b);
            if (t == std::string::npos) { f.push_back(line.substr(b)); break; }
            f.push_back(line.substr(b, t - b));
            b = t + 1;
        }
        IndexEntry en;
        if (f.size() != 4 || !parseCampaign(f[0], &en.campaign) || f[1].empty() || f[2].empty() || f[3].empty()) continue;
        en.hostSave = f[1];
        en.guestSave = f[2];
        en.time = std::strtoll(f[3].c_str(), nullptr, 10);
        ix.e_.push_back(en);
    }
    return ix;
}
std::string Index::text() const {
    std::string o = "FNVCOOP-PARTY-INDEX 1\n";
    for (const IndexEntry& e : e_)
        o += campaignHex(e.campaign) + "\t" + clean1(e.hostSave) + "\t" + clean1(e.guestSave) + "\t" + std::to_string(e.time) + "\n";
    return o;
}
void Index::add(const IndexEntry& e) {
    if (!FNV_MUT(1109))   // MUTANT 1109: a host save that is saved again adds a second entry (the launcher would pick between two)
        e_.erase(std::remove_if(e_.begin(), e_.end(), [&](const IndexEntry& o) { return o.campaign == e.campaign && sameName(o.hostSave, e.hostSave); }), e_.end());
    e_.push_back(e);
    if (e_.size() > 400) {   // keep the newest
        std::stable_sort(e_.begin(), e_.end(), [](const IndexEntry& a, const IndexEntry& b) { return a.time > b.time; });
        e_.resize(300);
    }
}
const IndexEntry* Index::pick(uint64_t campaign, const std::string& hostSave, const std::function<bool(const std::string&)>& exists) const {
    const IndexEntry* exact = nullptr;
    const IndexEntry* newest = nullptr;
    for (const IndexEntry& e : e_) {
        if (e.campaign != campaign || (exists && !exists(e.guestSave))) continue;
        if (!newest || e.time >= newest->time) newest = &e;
        if (sameName(e.hostSave, hostSave) && (!exact || e.time >= exact->time)) exact = &e;
    }
    if (FNV_MUT(1103)) return newest;   // MUTANT 1103: the host save is ignored (the guest loads a newer save than the host's)
    return exact ? exact : newest;
}

// ---- orders --------------------------------------------------------------------------------------------------------------------------------------
std::string campaignOrderJson(uint64_t c, const nvcoop::rebalance::CampaignSettings& settings, int players) {
    Value o = Value::object();
    o.set("op", "campaign").set("id", campaignHex(c));
    nvcoop::rebalance::addJson(o, settings);
    if (players > 0) o.set("players", players);
    return o.dump();
}
std::string saveOrderJson(const SaveOrder& s) {
    Value o = Value::object();
    o.set("op", "partySave").set("campaign", campaignHex(s.campaign)).set("hostSave", s.hostSave).set("seq", s.seq);
    if (s.reqSlot >= 0) o.set("reqSlot", s.reqSlot).set("reqId", s.reqId);
    return o.dump();
}
bool parseSaveOrder(const Value& v, SaveOrder* out) {
    SaveOrder s;
    if (!parseCampaign(v.get("campaign").asString(""), &s.campaign)) return false;
    s.hostSave = v.get("hostSave").asString("");
    const int64_t seq = v.get("seq").asInt(0);
    if (s.hostSave.empty() || seq <= 0) return false;
    s.seq = static_cast<uint64_t>(seq);
    s.reqSlot = static_cast<int>(v.get("reqSlot").asInt(-1));
    s.reqId = static_cast<uint64_t>(v.get("reqId").asInt(0));
    if (out) *out = s;
    return true;
}
std::string saveWaitJson(const std::string& text) {
    Value o = Value::object();
    o.set("op", "partySaveWait").set("text", text);
    return o.dump();
}
std::string ackJson(uint64_t seq, const std::string& save, bool skipped) {
    Value o = Value::object();
    o.set("op", "partySaved").set("seq", seq).set("save", save);
    if (skipped) o.set("skipped", true);
    return o.dump();
}
std::string requestJson(uint64_t id, const std::string& save) {
    Value o = Value::object();
    o.set("op", "partySaveRequest").set("id", id).set("save", save);
    return o.dump();
}

// ---- host ----------------------------------------------------------------------------------------------------------------------------------------
int HostParty::partySize() const { return players_; }

uint64_t HostParty::ensureCampaign() {
    if (campaign_) return campaign_;
    uint64_t id = env_.random64 ? env_.random64() : 0;
    campaign_ = id ? id : 1;
    settings_ = pending_;   // the campaign keeps the mode the host picked when it started
    if (env_.log) env_.log("party save: new campaign " + campaignHex(campaign_) + ", rebalance: " + nvcoop::rebalance::describe(settings_));
    return campaign_;
}

uint64_t HostParty::nextSeq() {
    if (!run_) {   // a host that restarts must not reuse a number a guest still remembers: the high part is random per run
        run_ = (env_.random64 ? env_.random64() : 1) & 0xFFFFFFFull;
        if (!run_) run_ = 1;
    }
    return (run_ << 24) | (++counter_ & 0xFFFFFFull);
}

void HostParty::sendOrResend(uint8_t slot, const std::string& json) {
    if (env_.sendOrder && env_.sendOrder(slot, json)) return;
    resend_.push_back({slot, json, (env_.nowMs ? env_.nowMs() : 0) + 30000});
    ++stats_.resent;
}

uint64_t HostParty::onHostSaved(const std::string& savePathOrName) {
    const std::string base = saveBaseName(savePathOrName);
    if (base.empty()) return 0;
    ensureCampaign();
    SaveOrder o;
    o.campaign = campaign_;
    o.hostSave = base;
    o.seq = nextSeq();
    if (expect_.active && sameName(expect_.name, base)) {   // the save the host just made for a guest's hand save
        o.reqSlot = expect_.req.slot;
        o.reqId = expect_.req.id;
        expect_.active = false;
        if (!requests_.empty() && requests_.front().slot == expect_.req.slot && requests_.front().id == expect_.req.id) requests_.pop_front();
        ++stats_.requestSaves;
    }
    const std::string json = saveOrderJson(o);
    if (env_.joined && !FNV_MUT(1100))   // MUTANT 1100: the host saves and tells nobody
        for (uint8_t slot : env_.joined()) sendOrResend(slot, json);
    ++stats_.orders;
    if (env_.log) env_.log("party save: host saved '" + base + "' -> sequence " + std::to_string(o.seq) + (o.reqSlot >= 0 ? " (for a guest's hand save)" : ""));
    return o.seq;
}

void HostParty::onGuestRequest(uint8_t slot, uint64_t id, const std::string& guestSave) {
    for (const Request& r : requests_)
        if (r.slot == slot && r.id == id) return;   // a resend
    if (expect_.active && expect_.req.slot == slot && expect_.req.id == id) return;
    Request r;
    r.slot = slot;
    r.id = id;
    r.guestSave = saveBaseName(guestSave);
    if (r.guestSave.empty()) return;
    requests_.push_back(r);
    ++stats_.requests;
    if (env_.log) env_.log("party save: guest slot " + std::to_string(slot) + " saved '" + r.guestSave + "' by hand and asks for a party save");
}

void HostParty::onAck(uint8_t slot, uint64_t seq, const std::string& guestSave, bool skipped) {
    acked_[seq].insert(slot);
    while (acked_.size() > 64) acked_.erase(acked_.begin());
    ++stats_.acks;
    if (skipped) ++stats_.skipped;
    if (env_.log) env_.log("party save: guest slot " + std::to_string(slot) + (skipped ? " skipped" : " saved '" + guestSave + "' for") + " sequence " + std::to_string(seq));
}

void HostParty::serveRequest() {
    const uint64_t now = env_.nowMs ? env_.nowMs() : 0;
    if (expect_.active) {   // the engine said "saved" and the SaveGame message has not come: do not wait for ever
        if (now - expect_.since > 5000) {
            if (env_.log) env_.log("party save: no SaveGame message for '" + expect_.name + "', request dropped");
            if (!requests_.empty()) requests_.pop_front();
            expect_.active = false;
        }
        return;
    }
    if (requests_.empty()) return;
    Request& r = requests_.front();
    if (r.lastTry && now - r.lastTry < kRetryMs) return;
    const bool can = FNV_MUT(1107) ? true : (env_.canSaveNow && env_.canSaveNow());   // MUTANT 1107: the host saves into a menu / scene the engine refuses
    if (!can) {
        if (!r.notified) {   // told once; the party save follows as soon as the host can
            r.notified = true;
            ++stats_.waits;
            if (env_.sendOrder) env_.sendOrder(r.slot, saveWaitJson("The host cannot save right now. Your party save follows as soon as the host can."));
            if (env_.log) env_.log("party save: the host cannot save now; guest slot " + std::to_string(r.slot) + " is waiting");
        }
        return;
    }
    ensureCampaign();
    r.lastTry = now;
    expect_.active = true;
    expect_.name = hostSaveNameForRequest(campaign_, r.guestSave);
    expect_.req = r;
    expect_.since = now;
    const std::string name = expect_.name;
    const bool saved = env_.saveNow && env_.saveNow(name);   // the SaveGame message (onHostSaved) may run inside this call: `r` is not touched after it
    if (saved) return;
    expect_.active = false;
    if (!requests_.empty() && ++requests_.front().failures >= kMaxFailures) {
        if (env_.log) env_.log("party save: the host could not save '" + name + "' " + std::to_string(kMaxFailures) + " times, request dropped");
        requests_.pop_front();
    }
}

void HostParty::tick() {
    const uint64_t now = env_.nowMs ? env_.nowMs() : 0;
    const std::vector<uint8_t> joined = env_.joined ? env_.joined() : std::vector<uint8_t>();
    for (auto it = told_.begin(); it != told_.end();)   // a guest that left is told again when it comes back
        it = std::find(joined.begin(), joined.end(), it->first) == joined.end() ? told_.erase(it) : std::next(it);
    players_ = static_cast<int>(joined.size()) + 1;
    if (campaign_ && !FNV_MUT(1105)) {   // MUTANT 1105: a guest is never told which campaign it plays
        for (uint8_t slot : joined) {
            auto t = told_.find(slot);
            // told again when the campaign, its rebalance setting or the party size changed (the size only matters in Co-op balanced)
            const int size = settings_.balanced() ? players_ : 0;
            if (t != told_.end() && t->second.campaign == campaign_ && t->second.settings == settings_ && (t->second.players == size || FNV_MUT(1262))) continue;   // MUTANT 1262: a guest is not told when the party size changes
            if (env_.sendOrder && env_.sendOrder(slot, campaignOrderJson(campaign_, settings_, players_))) told_[slot] = Told{campaign_, settings_, size};
        }
    }
    for (auto it = resend_.begin(); it != resend_.end();) {
        if (now > it->until || (env_.sendOrder && env_.sendOrder(it->slot, it->json))) it = resend_.erase(it);
        else ++it;
    }
    serveRequest();
}

// ---- guest ---------------------------------------------------------------------------------------------------------------------------------------
void GuestParty::markHandled(uint64_t seq) {
    if (done_.insert(seq).second) doneOrder_.push_back(seq);
    while (doneOrder_.size() > 128) { done_.erase(doneOrder_.front()); doneOrder_.pop_front(); }
}

void GuestParty::ack(uint64_t seq, const std::string& save, bool skipped) {
    outbox_.push_back(ackJson(seq, save, skipped));
    ++stats_.acks;
}

void GuestParty::remember(const std::string& hostSave, const std::string& guestSave) {
    Index ix = Index::parse(env_.readIndex ? env_.readIndex() : std::string());
    IndexEntry e;
    e.campaign = campaign_;
    e.hostSave = hostSave;
    e.guestSave = guestSave;
    e.time = env_.wallSeconds ? env_.wallSeconds() : 0;
    ix.add(e);
    if (env_.writeIndex && !env_.writeIndex(ix.text()) && env_.log) env_.log("party save: the index could not be written");
}

void GuestParty::onPartySaveOrder(const SaveOrder& o) {
    if (o.campaign) campaign_ = o.campaign;
    if (!FNV_MUT(1101) && (handled(o.seq) || (pending_ && order_.seq == o.seq))) {   // MUTANT 1101: a repeated order saves a second time
        ++stats_.duplicates;
        return;
    }
    if (haveRequest_ && o.reqSlot >= 0 && o.reqSlot == (env_.mySlot ? env_.mySlot() : -1) && o.reqId == request_.id && !FNV_MUT(1104)) {
        // The host made this save for the hand save we just made: that save IS our party save. A second one for the same sequence would be a lone duplicate.
        remember(o.hostSave, request_.save);   // MUTANT 1104 skips this branch: the requester saves again
        markHandled(o.seq);
        ack(o.seq, request_.save, false);
        haveRequest_ = false;
        ++stats_.adopted;
        if (env_.notify) env_.notify("Party save made. Your save '" + request_.save + "' belongs to it.");
        if (env_.log) env_.log("party save: hand save '" + request_.save + "' adopted for host save '" + o.hostSave + "'");
        return;
    }
    if (pending_) {   // a newer host save while an older one is still waiting: the newest state is the one that matches
        ack(order_.seq, "", true);
        ++stats_.superseded;
    }
    pending_ = true;
    order_ = o;
    pendingSince_ = env_.nowMs ? env_.nowMs() : 0;
    lastTry_ = 0;
    failures_ = 0;
    announced_ = false;
}

bool GuestParty::onLocalSave(const std::string& savePathOrName) {
    const std::string base = saveBaseName(savePathOrName);
    if (base.empty() || own_.count(lower(base))) return false;
    if (!campaign_ || (env_.mySlot ? env_.mySlot() : -1) < 0) {
        if (env_.log) env_.log("party save: hand save '" + base + "' outside a co-op session: not a party save");
        return false;
    }
    if (FNV_MUT(1110)) return false;   // MUTANT 1110: the hand save stays a lone save (the host is never asked)
    haveRequest_ = true;
    request_.id = ++nextRequest_;
    request_.save = base;
    request_.sent = false;
    ++stats_.requests;
    if (env_.notify) env_.notify("Saved. The host makes a party save so your save and the world match.");
    if (env_.log) env_.log("party save: hand save '" + base + "' -> asking the host for a party save (request " + std::to_string(request_.id) + ")");
    return true;
}

std::string GuestParty::ownHostSave(const std::string& savePathOrName) const {
    auto it = own_.find(lower(saveBaseName(savePathOrName)));
    return it == own_.end() ? std::string() : it->second;
}

void GuestParty::tick() {
    const uint64_t now = env_.nowMs ? env_.nowMs() : 0;
    for (auto it = outbox_.begin(); it != outbox_.end();)   // what the host has not taken yet
        it = (env_.sendToHost && env_.sendToHost(*it)) ? outbox_.erase(it) : std::next(it);
    if (haveRequest_ && !request_.sent && env_.sendToHost && env_.sendToHost(requestJson(request_.id, request_.save))) request_.sent = true;
    if (!pending_) return;
    if (now - pendingSince_ > kPendingGiveUpMs) {
        ack(order_.seq, "", true);
        pending_ = false;
        ++stats_.failed;
        if (env_.log) env_.log("party save: could not save for sequence " + std::to_string(order_.seq) + " in time, given up");
        return;
    }
    const bool can = FNV_MUT(1102) ? true : (env_.canSaveNow && env_.canSaveNow());   // MUTANT 1102: saves into a menu / scene the engine refuses
    if (!can) {
        if (FNV_MUT(1108)) { pending_ = false; return; }   // MUTANT 1108: a save that cannot happen now is forgotten (the guest never gets one)
        if (!announced_) {
            announced_ = true;
            ++stats_.deferred;
            if (env_.log) env_.log("party save: cannot save now (menu or scene), will save as soon as possible");
        }
        return;
    }
    if (lastTry_ && now - lastTry_ < kRetryMs) return;
    lastTry_ = now;
    const std::string name = guestSaveName(order_.campaign ? order_.campaign : campaign_, order_.hostSave);
    own_[lower(name)] = order_.hostSave;
    if (own_.size() > 64) own_.erase(own_.begin());
    if (env_.saveNow && env_.saveNow(name)) {
        ++stats_.saves;
        remember(order_.hostSave, name);
        markHandled(order_.seq);
        ack(order_.seq, name, false);
        pending_ = false;
        if (env_.log) env_.log("party save: saved '" + name + "' for host save '" + order_.hostSave + "'");
        return;
    }
    own_.erase(lower(name));
    if (++failures_ >= kMaxFailures) {
        ack(order_.seq, "", true);
        pending_ = false;
        ++stats_.failed;
        if (env_.notify) env_.notify("Could not save your game for the party save.");
        if (env_.log) env_.log("party save: the engine refused the save " + std::to_string(kMaxFailures) + " times, given up");
    }
}

// ---- dispatchers -----------------------------------------------------------------------------------------------------------------------------------
bool isGuestOp(const std::string& op) { return op == "campaign" || op == "partySave" || op == "partySaveWait"; }
bool isHostOp(const std::string& op) { return op == "partySaveRequest" || op == "partySaved"; }

std::string applyGuestOrder(GuestParty& g, const Value& v) {
    const std::string op = v.get("op").asString("");
    if (op == "campaign") {
        uint64_t id = 0;
        if (!parseCampaign(v.get("id").asString(""), &id)) return "BadCampaign";
        g.setCampaign(id);
        g.setSettings(nvcoop::rebalance::fromJson(v));
        g.setPlayers(static_cast<int>(v.get("players").asInt(0)));
        return "Ok";
    }
    if (op == "partySave") {
        SaveOrder o;
        if (!parseSaveOrder(v, &o)) return "BadOrder";
        g.onPartySaveOrder(o);
        return "Ok";
    }
    if (op == "partySaveWait") {
        g.onWait(v.get("text").asString(""));
        return "Ok";
    }
    return "UnknownOrder";
}

std::string handleHostOrder(HostParty& h, uint8_t from, const Value& v) {
    const std::string op = v.get("op").asString("");
    if (op == "partySaveRequest") {
        const int64_t id = v.get("id").asInt(0);
        if (id <= 0) return "BadRequest";
        h.onGuestRequest(from, static_cast<uint64_t>(id), v.get("save").asString(""));
        return "Ok";
    }
    if (op == "partySaved") {
        const int64_t seq = v.get("seq").asInt(0);
        if (seq <= 0) return "BadAck";
        h.onAck(from, static_cast<uint64_t>(seq), v.get("save").asString(""), v.get("skipped").asBool(false));
        return "Ok";
    }
    return "UnknownOrder";
}

}  // namespace nvcoop::coop::party
