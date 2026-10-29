#include "chat/chat_core.h"

#include <algorithm>
#include <cctype>

#include "chat/chat_text.h"
#include "fnvcoop_mut.h"

// Twins: 1040 = the host does not rate-limit guests; 1041 = (limiter, see chat_limiter.cpp 1011); 1042 = the host also sends a guest's line back to that guest;
//        1043 = the host relays the text as it came (not sanitised); 1044 = a guest shows the text as the host sent it (not sanitised);
//        1045 = a guest shows a line that names itself as the origin (its own line twice); 1046 = the host does not drop a repeated Say;
//        1047 = no "left" line when a guest leaves; 1048 = two players with one name keep it; 1049 = say() does not check the local rate;
//        1050 = a guest says nothing to the host (its line stays on its own machine); 1051 = a system line the host makes is not sent to the guests;
//        1052 = the host takes a Say from a slot that is not joined; 1053 = the host's own line is not sent to the guests;
//        1054 = a rate-limited guest is not told; 1055 = a guest shows a Say (a datagram only a host should take); 1056 = Hello is not sent on link-up
namespace nvcoop::chat {

ChatCore::ChatCore(Config cfg, uint32_t idSeed)
    : cfg_(std::move(cfg)), log_(cfg_.historyLines), limiter_(cfg_.rateLines, cfg_.rateSeconds * 1000u), nextId_(idSeed ? idSeed : 1) {}

void ChatCore::setConfig(const Config& c) {
    cfg_ = c;
    log_.setCapacity(c.historyLines);
    limiter_ = RateLimiter(c.rateLines, c.rateSeconds * 1000u);
}

void ChatCore::begin(bool isHost, Slot self) {
    active_ = true;
    isHost_ = isHost;
    self_ = self;
    log_.clearSeen();
    limiter_.reset();
    names_.clear();
    announced_.clear();
    if (isHost) names_[self] = selfName();
}

void ChatCore::end() {
    active_ = false;
    names_.clear();
    announced_.clear();
}

std::string ChatCore::selfName() const {
    std::string n = sanitizeName(cfg_.name);
    return n.empty() ? defaultName(self_ == kSystemSlot ? Slot(0) : self_) : n;
}

std::string ChatCore::nameOf(Slot slot) const {
    if (slot == self_ && !isHost_) return selfName();
    auto it = names_.find(slot);
    return it != names_.end() ? it->second : defaultName(slot);
}

bool ChatCore::isJoined(Slot s) const {
    if (!ports_.joinedSlots) return false;
    const std::vector<Slot> j = ports_.joinedSlots();
    return std::find(j.begin(), j.end(), s) != j.end();
}

std::string ChatCore::uniqueName(Slot slot, std::string base) const {
    auto same = [](const std::string& a, const std::string& b) {
        if (a.size() != b.size()) return false;
        for (size_t i = 0; i < a.size(); ++i)
            if (std::tolower(static_cast<unsigned char>(a[i])) != std::tolower(static_cast<unsigned char>(b[i]))) return false;
        return true;
    };
    auto taken = [&](const std::string& n) {
        for (const auto& kv : names_)
            if (kv.first != slot && same(kv.second, n)) return true;
        return false;
    };
    if (FNV_MUT(1048) || !taken(base)) return base;
    for (int i = 2; i < 100; ++i) {
        const std::string suffix = " " + std::to_string(i);
        std::string cand = base.substr(0, kMaxNameBytes - suffix.size()) + suffix;
        if (!taken(cand)) return cand;
    }
    return base;
}

void ChatCore::push(Line l) {
    if (!log_.add(l)) { ++stats_.duplicates; return; }
    if (onLine_) onLine_(log_.lines().back());
}

void ChatCore::broadcast(const Bytes& data, Slot except) {
    if (!ports_.sendToSlot || !ports_.joinedSlots) return;
    for (Slot s : ports_.joinedSlots())
        if (s != except && s != kHostSlot) ports_.sendToSlot(s, data);
}

void ChatCore::linkUp() {
    if (isHost_ || !active_ || !ports_.sendToHost || FNV_MUT(1056)) return;
    ports_.sendToHost(encodeHello(sanitizeName(cfg_.name)));
}

void ChatCore::playerJoined(Slot slot) {
    if (!isHost_ || slot == kHostSlot || slot >= 8) return;
    if (!names_.count(slot)) names_[slot] = uniqueName(slot, defaultName(slot));
}

void ChatCore::playerLeft(Slot slot, uint64_t nowMs) {
    if (!isHost_ || slot == kHostSlot) return;
    const bool announced = announced_.erase(slot) != 0;
    const std::string name = nameOf(slot);
    names_.erase(slot);
    limiter_.forget(slot);
    log_.forgetSender(slot);
    if (announced && !FNV_MUT(1047)) {
        ++stats_.leavesAnnounced;
        system(name + " left", true, nowMs);
    }
}

SayResult ChatCore::say(std::string_view raw, uint64_t nowMs) {
    const std::string text = sanitizeText(raw, kMaxTextBytes);
    if (text.empty()) return SayResult::Empty;
    if (!active_) return SayResult::NoSession;
    if (!FNV_MUT(1049) && !limiter_.allow(self_, nowMs)) { ++stats_.rateLimited; return SayResult::RateLimited; }
    if (++nextId_ == 0) nextId_ = 1;
    Line l;
    l.slot = self_;
    l.msgId = nextId_;
    l.kind = LineKind::Player;
    l.name = selfName();
    l.text = text;
    l.atMs = nowMs;
    push(l);
    ++stats_.said;
    if (isHost_) {
        if (!FNV_MUT(1053)) broadcast(encodeLine(self_, l.msgId, LineKind::Player, l.name, l.text), kSystemSlot);
    } else if (ports_.sendToHost && !FNV_MUT(1050)) {
        ports_.sendToHost(encodeSay(l.msgId, l.text));
    }
    return SayResult::Sent;
}

void ChatCore::system(std::string_view raw, bool broadcastIt, uint64_t nowMs) {
    const std::string text = sanitizeText(raw, kMaxTextBytes);
    if (text.empty()) return;
    Line l;
    l.slot = kSystemSlot;
    l.kind = LineKind::System;
    l.text = text;
    l.atMs = nowMs;
    if (isHost_ && active_ && broadcastIt) {
        if (++nextId_ == 0) nextId_ = 1;
        l.msgId = nextId_;
        push(l);
        if (!FNV_MUT(1051)) broadcast(encodeLine(kSystemSlot, l.msgId, LineKind::System, std::string(), text), kSystemSlot);
    } else {
        push(l);
    }
}

void ChatCore::notice(std::string_view raw, uint64_t nowMs) {
    const std::string text = sanitizeText(raw, kMaxTextBytes);
    if (text.empty()) return;
    Line l;
    l.slot = kSystemSlot;
    l.kind = LineKind::System;
    l.text = text;
    l.atMs = nowMs;
    push(l);
}

void ChatCore::onDatagram(Slot from, const uint8_t* data, size_t size, uint64_t nowMs) {
    if (!active_) return;
    Message m;
    if (!decode(data, size, m)) { ++stats_.undecodable; return; }
    ++stats_.received;
    if (isHost_) {
        const bool known = from != kHostSlot && from < 8 && (isJoined(from) || FNV_MUT(1052));
        if (!known) { ++stats_.unknownSender; return; }
        if (m.kind == Kind::Hello) {
            std::string name = sanitizeName(m.name);
            if (name.empty()) name = defaultName(from);
            names_[from] = uniqueName(from, name);
            if (announced_.insert(from).second) {
                ++stats_.joinsAnnounced;
                system(names_[from] + " joined", true, nowMs);
            }
            return;
        }
        if (m.kind != Kind::Say) return;
        const std::string text = FNV_MUT(1043) ? m.text : sanitizeText(m.text, kMaxTextBytes);
        if (text.empty()) return;
        if (log_.seen(from, m.msgId) && !FNV_MUT(1046)) { ++stats_.duplicates; return; }
        if (!FNV_MUT(1040) && !limiter_.allow(from, nowMs)) {
            ++stats_.rateLimited;
            if (ports_.sendToSlot && !FNV_MUT(1054)) ports_.sendToSlot(from, encodeNotice(kNoticeRateLimited));
            return;
        }
        Line l;
        l.slot = from;
        l.msgId = m.msgId;
        l.kind = LineKind::Player;
        l.name = nameOf(from);
        l.text = text;
        l.atMs = nowMs;
        const uint32_t seqBefore = log_.lastSeq();
        push(l);
        if (log_.lastSeq() == seqBefore) return;   // a repeat the log refused
        ++stats_.relayed;
        broadcast(encodeLine(from, l.msgId, LineKind::Player, l.name, l.text), FNV_MUT(1042) ? kSystemSlot : from);
        return;
    }
    // guest: only the host speaks to it
    if (from != kHostSlot) { ++stats_.unknownSender; return; }
    if (m.kind == Kind::Notice) {
        if (m.code == kNoticeRateLimited) notice("You are sending messages too fast. The last one was not sent.", nowMs);
        return;
    }
    if (m.kind == Kind::Say && FNV_MUT(1055)) {
        Line l;
        l.slot = from; l.msgId = m.msgId; l.kind = LineKind::Player; l.name = nameOf(from); l.text = m.text; l.atMs = nowMs;
        push(l);
        return;
    }
    if (m.kind != Kind::Line) return;
    Line l;
    l.msgId = m.msgId;
    l.atMs = nowMs;
    l.text = FNV_MUT(1044) ? m.text : sanitizeText(m.text, kMaxTextBytes);
    if (l.text.empty()) return;
    if (m.lineKind == LineKind::System) {
        l.slot = kSystemSlot;
        l.kind = LineKind::System;
    } else {
        if (m.slot >= 8) { ++stats_.undecodable; return; }
        if (m.slot == self_ && !FNV_MUT(1045)) { ++stats_.ownEchoes; return; }
        l.slot = m.slot;
        l.kind = LineKind::Player;
        l.name = sanitizeName(m.name);
        if (l.name.empty()) l.name = defaultName(m.slot);
    }
    push(l);
}

}  // namespace nvcoop::chat
