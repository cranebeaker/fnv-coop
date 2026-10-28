#include "intents/intent_client.h"

#include <cstdio>

#include <sstream>

#include "fnvcoop_mut.h"

namespace nvcoop::intents {

namespace {

std::string cellHex(uint32_t c) {
    char b[16];
    std::snprintf(b, sizeof b, "%x", static_cast<unsigned>(c));
    return b;
}
std::string posText(const Vec3& v) {
    char b[80];
    std::snprintf(b, sizeof b, "(%.0f,%.0f,%.0f)", v.x, v.y, v.z);
    return b;
}
constexpr size_t kMaxPending = 64;
constexpr uint64_t kNeverSawMenuMs = 3000;   // the menu never showed up (the activation failed locally): release the lock anyway

std::string explain(Kind k, Result r, PlayerId holder) {
    switch (r) {
        case Result::Busy: return holder == kNoPlayer ? "Someone else is using that." : "Player " + std::to_string(static_cast<int>(holder) + 1) + " is using that.";
        case Result::Taken: return "Somebody else got there first.";
        case Result::OutOfRange: return k == Kind::PickUp ? "You are too far away to take that." : "You are too far away.";
        case Result::NotLoaded: return "The host cannot see that yet.";
        case Result::Denied: return "You cannot use that.";
        case Result::NoPuppet: return "The host could not act for you yet. Try again.";
        case Result::RateLimited: return "Slow down.";
        default: return "That did not work.";
    }
}

}  // namespace

IntentClient::IntentClient(ClientDeps deps, Config cfg) : deps_(std::move(deps)), cfg_(std::move(cfg)) {}

void IntentClient::syncSession() {
    const uint32_t s = deps_.session ? deps_.session() : 0;
    if (s == session_) return;
    session_ = s;
    nextId_ = 1;
    if (!FNV_MUT(322)) pending_.clear();   // MUTANT 322: intents of the old session stay pending forever
    recent_.clear();
    watching_ = false;
}

Obs IntentClient::finish(const GuestDecision& d, const FormRef& target) {
    if (d.viaDialogue) {
        if (deps_.startDialogue) { deps_.startDialogue(target); ++stats_.dialogueStarts; }
        ++stats_.proceeded;
        return Obs::Proceed;
    }
    if (!d.forward) { ++stats_.proceeded; return Obs::Proceed; }
    const uint64_t t = now();
    const auto key = std::make_pair(d.kind, target);
    auto it = recent_.find(key);
    // a TriggerStay is a keep-alive of a presence the host re-raises every frame by itself: one per stay_keepalive_ms, not one per engine frame
    const uint32_t window = d.kind == Kind::TriggerStay && !FNV_MUT(402) ? cfg_.stayKeepaliveMs : cfg_.coalesceMs;   // MUTANT 402: the guest sends a keep-alive for every engine frame
    if (it != recent_.end() && window > 0 && t - it->second < window && !FNV_MUT(321)) {   // MUTANT 321: a double click sends two intents
        ++stats_.coalesced;
        if (d.suppressLocal) { ++stats_.suppressed; return Obs::Suppress; }
        ++stats_.proceeded;
        return Obs::Proceed;
    }
    if (submit(d.kind, target) == 0) { ++stats_.proceeded; return Obs::Proceed; }   // could not forward: the engine call goes on as before
    if (recent_.size() > 256) recent_.clear();
    recent_[key] = t;
    if (d.suppressLocal) { ++stats_.suppressed; return Obs::Suppress; }
    ++stats_.proceeded;
    return Obs::Proceed;
}

Obs IntentClient::onActivate(const FormRef& target) {
    ++stats_.observedActivate;
    if (inObserver_) { ++stats_.proceeded; return Obs::Proceed; }
    TargetInfo t;
    if (!deps_.eng->describe(target, t)) { ++stats_.proceeded; return Obs::Proceed; }
    const GuestDecision d = decideActivate(cfg_, t);
    if (deps_.log) {
        char b[64];
        std::snprintf(b, sizeof b, "%X", static_cast<unsigned>(target.localId));
        deps_.log(std::string("intents: guest activates ") + target.plugin + ":" + b + " class " + className(t.cls) + (t.loaded ? "" : " (NOT loaded)") + (t.disabled ? " (disabled)" : "") + " cell " + cellHex(t.cell) + " pos " + posText(t.pos) + ": " + (d.viaDialogue ? "talk via the dialogue channel" : d.forward ? std::string("forwarded as ") + kindName(d.kind) + (d.suppressLocal ? ", local activation suppressed" : ", local activation goes on") : std::string("not forwarded (") + d.why + "), local activation goes on"));
    }
    return finish(d, target);
}

Obs IntentClient::onEvent(uint32_t mask, const FormRef& target) {
    ++stats_.observedEvent;
    if (inObserver_) { ++stats_.proceeded; return Obs::Proceed; }
    TargetInfo t;
    if (!deps_.eng->describe(target, t)) { ++stats_.proceeded; return Obs::Proceed; }
    return finish(decideEvent(cfg_, mask, t), target);
}

bool IntentClient::onStageBlocked(const FormRef& quest, uint32_t stage) {
    ++stats_.observedStage;
    if (!cfg_.enable || !cfg_.stage) { ++stats_.stageOff; return false; }
    if (stage == 0 || stage > 255) { ++stats_.stageBad; return false; }
    const uint64_t t = now();
    const auto key = std::make_pair(quest, stage);
    auto it = recentStage_.find(key);
    if (it != recentStage_.end() && cfg_.coalesceMs > 0 && t - it->second < cfg_.coalesceMs && !FNV_MUT(332)) {   // MUTANT 332: a script that retries every frame sends 60 intents a second
        ++stats_.coalesced;
        return false;
    }
    if (submit(Kind::SetStage, quest, nullptr, static_cast<int32_t>(stage)) == 0) return false;
    if (recentStage_.size() > 256) recentStage_.clear();
    recentStage_[key] = t;
    return true;
}

uint32_t IntentClient::submit(Kind kind, const FormRef& target, const FormRef* aux, int32_t value, std::vector<uint8_t> path) {
    syncSession();
    if (!deps_.ready || !deps_.ready()) { ++stats_.notReady; return 0; }
    if (pending_.size() >= kMaxPending) { ++stats_.pendingCapped; return 0; }
    Intent in;
    in.session = session_;
    in.id = nextId_;
    in.kind = kind;
    in.target = target;
    if (aux) { in.aux = *aux; in.hasAux = true; }
    in.value = value;
    in.path = std::move(path);
    if (in.path.size() > kMaxPath) in.path.resize(kMaxPath);
    if (!deps_.sendToHost || !deps_.sendToHost(encode(in))) { ++stats_.sendFailed; return 0; }
    ++nextId_;
    ++stats_.forwarded;
    ++stats_.byKind[static_cast<size_t>(kind)];
    pending_[in.id] = Pending{in, now()};
    return in.id;
}

void IntentClient::onDatagram(const uint8_t* data, size_t size) {
    Verdict v;
    if (decode(data, size, nullptr, &v) != WireType::Verdict) { ++stats_.badDatagrams; return; }
    syncSession();
    auto it = pending_.find(v.id);
    if (v.session != session_ || it == pending_.end()) { ++stats_.unknownVerdicts; return; }
    Pending p = std::move(it->second);
    pending_.erase(it);
    ++stats_.verdicts;
    if (v.result != Result::Executed) ++stats_.rejected[static_cast<size_t>(v.result)];
    onVerdict(v, p);
}

void IntentClient::onVerdict(const Verdict& v, const Pending& p) {
    const Kind k = p.in.kind;
    if (k == Kind::ContainerOpen) {
        if (v.result == Result::Executed) {
            if (v.hasRedirect) {
                deps_.eng->closeMenus();   // the shared storage opened locally; the player's own is the one to show
                inObserver_ = true;
                deps_.eng->openContainer(v.redirect);
                inObserver_ = false;
                ++stats_.redirectsOpened;
            } else if (deps_.containerMenuOpen) {
                watching_ = true;
                sawOpen_ = false;
                watched_ = p.in.target;
                watchSince_ = now();
            }
        } else if (v.result != Result::Duplicate) {
            if (!(v.result == Result::Busy && FNV_MUT(319))) deps_.eng->closeMenus();   // MUTANT 319: a busy container stays open on the guest
            deps_.eng->notify(explain(k, v.result, v.holder));
        }
        return;
    }
    if (k == Kind::PickUp && v.result != Result::Executed && v.result != Result::Duplicate) deps_.eng->notify(explain(k, v.result, v.holder));
}

void IntentClient::tick() {
    syncSession();
    const uint64_t t = now();
    if (!FNV_MUT(323)) {   // MUTANT 323: a verdict that never comes keeps the intent pending for ever
        for (auto it = pending_.begin(); it != pending_.end();) {
            if (t - it->second.sentMs > cfg_.pendingTimeoutMs) { ++stats_.timeouts; it = pending_.erase(it); }
            else ++it;
        }
    }
    if (watching_ && deps_.containerMenuOpen && !FNV_MUT(320)) {   // MUTANT 320: the lock is never released (the chest stays "in use" for the lease)
        const bool open = deps_.containerMenuOpen();
        if (open) sawOpen_ = true;
        else if (sawOpen_ || t - watchSince_ > kNeverSawMenuMs) {
            watching_ = false;
            if (submit(Kind::ContainerClose, watched_) != 0) ++stats_.closesSent;
        }
    }
}

std::string IntentClient::statusJson() const {
    std::ostringstream o;
    o << "{\"role\":\"guest\",\"session\":" << session_ << ",\"pending\":" << pending_.size() << ",\"observedActivate\":" << stats_.observedActivate
      << ",\"observedStage\":" << stats_.observedStage << ",\"stageOff\":" << stats_.stageOff << ",\"stageBad\":" << stats_.stageBad
      << ",\"observedEvent\":" << stats_.observedEvent << ",\"forwarded\":" << stats_.forwarded << ",\"suppressed\":" << stats_.suppressed
      << ",\"proceeded\":" << stats_.proceeded << ",\"coalesced\":" << stats_.coalesced << ",\"notReady\":" << stats_.notReady
      << ",\"verdicts\":" << stats_.verdicts << ",\"unknownVerdicts\":" << stats_.unknownVerdicts << ",\"timeouts\":" << stats_.timeouts
      << ",\"sendFailed\":" << stats_.sendFailed << ",\"redirectsOpened\":" << stats_.redirectsOpened << ",\"closesSent\":" << stats_.closesSent
      << ",\"watching\":" << (watching_ ? "true" : "false") << ",\"rejected\":{";
    bool first = true;
    for (size_t r = 1; r < static_cast<size_t>(Result::Count_); ++r) {
        if (!stats_.rejected[r]) continue;
        o << (first ? "" : ",") << '"' << resultName(static_cast<Result>(r)) << "\":" << stats_.rejected[r];
        first = false;
    }
    o << "}}";
    return o.str();
}

}  // namespace nvcoop::intents
