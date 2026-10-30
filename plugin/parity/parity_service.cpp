#include "parity/parity_service.h"

#include <cstdio>

#include "common/minijson.h"
#include "fnvcoop_mut.h"

// Twins: 1818 = a new session (new mux) keeps the old channel handler and the old comparers; 1824 = a datagram from a peer that is not a joined client is
//        taken for client slot 1; 1825 = the sample is stamped with the slot the sender claims instead of the slot of the peer it came from;
//        1826 = a guest sends a sample every interval whatever sendEvery says; 1827 = the host does not take a fresh sample when a guest's arrives, so there is nothing to compare it with
namespace nvcoop::parity {

ParityService::ParityService(const Settings& s, IParitySource& src, std::function<void(const std::string&)> log, const std::string& filePath)
    : set_(s), src_(src), log_(std::move(log)), sampler_(set_, src), alive_(std::make_shared<uint64_t>(0)) {
    if (!filePath.empty()) file_.open(filePath, uint64_t(set_.maxFileMb) * 1024u * 1024u, true);
}

ParityService::~ParityService() { *alive_ = 0; }

void ParityService::endSession() {
    *alive_ = 0;
    up_ = false;
    mux_ = nullptr;
    pending_.clear();
    comparers_.clear();
    reporters_.clear();
    nextTickAt_ = 0;
    tickCount_ = 0;
}

uint8_t ParityService::slotOfPeer(fnvcoop::PeerId p) const {
    for (const Link& c : clients_) if (c.peer == p && c.joined) return c.slot;
    return kNoSlot;
}

void ParityService::startSession(const FrameInfo& f) {
    if (up_) endSession();
    ++st_.sessions;
    up_ = true;
    mux_ = f.mux;
    isHost_ = f.isHost;
    self_ = f.isHost ? 0 : f.self;
    *alive_ = ++gen_;
    std::shared_ptr<uint64_t> alive = alive_;
    const uint64_t gen = gen_;
    f.mux->registerChannel(kChannelParity, [this, alive, gen](fnvcoop::PeerId from, fnvcoop::Bytes&& data) {
        if (*alive != gen && !FNV_MUT(1818)) return;
        onDatagram(from, std::move(data));
    });
}

void ParityService::onDatagram(fnvcoop::PeerId from, fnvcoop::Bytes&& data) {
    if (!isHost_) return;   // only the host compares; a guest has nothing to do with a sample
    uint8_t slot = slotOfPeer(from);
    if (slot == kNoSlot) {
        if (!FNV_MUT(1824)) {
            ++st_.droppedUnknownPeer;
            return;
        }
        slot = 1;
    }
    Sample s;
    if (!decodeSample(data.data(), data.size(), s)) {
        ++st_.rejected;
        return;
    }
    ++st_.received;
    if (!FNV_MUT(1825)) s.slot = slot;   // the peer it came from, never what the datagram claims
    pending_.push_back(Pending{slot, std::move(s)});
    while (pending_.size() > 8) pending_.pop_front();
}

void ParityService::write(const Sample& s, const LineMeta& m) {
    if (!file_.isOpen()) return;
    file_.append(sampleJson(s, m));
}

void ParityService::tick(const FrameInfo& f) {
    if (f.nowMs < nextTickAt_) return;
    nextTickAt_ = f.nowMs + set_.intervalMs;
    Sample s;
    if (!sampler_.take(f.nowMs, f.wallMs, s)) return;
    ++st_.samples;
    ++tickCount_;
    LineMeta m;
    m.role = isHost_ ? "host" : "guest";
    m.why = "tick";
    write(s, m);
    if (!isHost_ && f.hostLinkUp && mux_ && (FNV_MUT(1826) || (set_.sendEvery > 0 && tickCount_ % set_.sendEvery == 0))) {
        const Sample n = netSample(s, set_.netCap);
        mux_->sendEx(0, encodeSample(n), fnvcoop::ITransport::Channel::Reliable);   // a guest's only peer is the host
        ++st_.sent;
    }
}

void ParityService::compareAll(const FrameInfo& f) {
    while (!pending_.empty()) {
        Pending p = std::move(pending_.front());
        pending_.pop_front();
        Sample mine;
        if (!FNV_MUT(1827)) {
            if (!sampler_.take(f.nowMs, f.wallMs, mine)) continue;
            ++st_.samples;
            LineMeta m;
            m.role = "host";
            m.why = "cmp";
            m.cmpSlot = p.slot;
            m.cmpSeq = p.s.seq;
            write(mine, m);
        } else {
            continue;   // twin 1827
        }
        auto cit = comparers_.find(p.slot);
        if (cit == comparers_.end()) cit = comparers_.emplace(p.slot, Comparer(set_.tol)).first;
        auto rit = reporters_.find(p.slot);
        if (rit == reporters_.end()) rit = reporters_.emplace(p.slot, Reporter(set_.tol)).first;
        const CompareResult r = cit->second.compare(mine, p.s, f.nowMs);
        ++st_.rounds;
        if (r.skipped) {
            ++st_.skipped;
            continue;
        }
        const uint64_t before = rit->second.suppressed();
        for (const std::string& line : rit->second.report(r, f.nowMs)) {
            ++st_.linesLogged;
            if (log_) log_("parity: " + line + " slot=" + std::to_string(p.slot));
        }
        st_.linesSuppressed += rit->second.suppressed() - before;
    }
    for (auto& kv : reporters_) {
        if (kv.second.summaryDue(f.nowMs)) {
            lastSummary_ = kv.second.summary(f.nowMs) + " slot=" + std::to_string(kv.first);
            if (log_) log_("parity: " + lastSummary_);
        }
    }
}

void ParityService::frame(const FrameInfo& f) {
    if (!set_.enabled) return;
    lastNowMs_ = f.nowMs;
    lastWallMs_ = f.wallMs;
    if (!f.active || !f.mux || (!f.isHost && f.self == kNoSlot)) {
        if (up_) endSession();
        return;
    }
    const bool newMux = FNV_MUT(1818) ? false : f.mux != mux_;
    if (!up_ || newMux || f.isHost != isHost_ || (!f.isHost && f.self != self_)) startSession(f);
    clients_ = f.clients;
    sampler_.frame(f.nowMs);
    tick(f);
    if (isHost_) compareAll(f);
}

bool ParityService::takeNow(uint64_t nowMs, uint64_t wallMs, std::string& jsonLine) {
    Sample s;
    if (!sampler_.take(nowMs, wallMs, s)) return false;
    LineMeta m;
    m.role = !up_ ? "solo" : isHost_ ? "host" : "guest";
    m.why = "bridge";
    jsonLine = sampleJson(s, m);
    return true;
}

std::map<std::string, KindCount> ParityService::kinds() const {
    std::map<std::string, KindCount> out;
    for (const auto& c : comparers_)
        for (const auto& k : c.second.kinds()) {
            out[k.first].checks += k.second.checks;
            out[k.first].bad += k.second.bad;
        }
    return out;
}

std::string ParityService::statusJson() const {
    nvcoop::json::Value v = nvcoop::json::Value::object();
    v.set("enabled", set_.enabled);
    v.set("up", up_);
    v.set("role", !up_ ? "none" : isHost_ ? "host" : "guest");
    v.set("samples", static_cast<int64_t>(st_.samples));
    v.set("sent", static_cast<int64_t>(st_.sent));
    v.set("received", static_cast<int64_t>(st_.received));
    v.set("rejected", static_cast<int64_t>(st_.rejected));
    v.set("rounds", static_cast<int64_t>(st_.rounds));
    v.set("skipped", static_cast<int64_t>(st_.skipped));
    v.set("lines_logged", static_cast<int64_t>(st_.linesLogged));
    v.set("lines_suppressed", static_cast<int64_t>(st_.linesSuppressed));
    v.set("dropped_unknown_peer", static_cast<int64_t>(st_.droppedUnknownPeer));
    v.set("file", file_.path());
    v.set("file_bytes", static_cast<int64_t>(file_.bytes()));
    v.set("file_rotations", static_cast<int64_t>(file_.rotations()));
    v.set("last_summary", lastSummary_);
    nvcoop::json::Value k = nvcoop::json::Value::object();
    for (const auto& kv : kinds()) {
        nvcoop::json::Value e = nvcoop::json::Value::object();
        e.set("checks", static_cast<int64_t>(kv.second.checks));
        e.set("bad", static_cast<int64_t>(kv.second.bad));
        k.set(kv.first, e);
    }
    v.set("kinds", k);
    return v.dump();
}

}  // namespace nvcoop::parity
