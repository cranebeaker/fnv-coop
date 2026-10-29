#include "mk_service.h"

#include <algorithm>
#include <cstdio>

namespace nvmk {

MarkerService::MarkerService(IMarkerPort& port, IMarkerHud* hud, ServiceConfig cfg)
    : port_(port), hud_(hud), cfg_(cfg), board_(cfg.markers), auth_(cfg.revive), hold_(cfg.hold) {}

void MarkerService::setConfig(const ServiceConfig& c) {
    cfg_ = c;
    board_.setConfig(c.markers);
    auth_.setRules(c.revive);
    hold_.setConfig(c.hold);
}

uint32_t MarkerService::localWindowLeftMs(uint64_t nowMs) const {
    if (!localDowned_ || localWindowMs_ == 0) return 0;
    const uint64_t gone = FNV_MUT(1418) ? 0 : nowMs - localDownedAt_;           // twin 1418: the timer never counts down
    return gone >= localWindowMs_ ? 0u : static_cast<uint32_t>(localWindowMs_ - gone);
}

std::string MarkerService::nameOf(Slot s) const {
    if (s == port_.localSlot() && !myName_.empty()) return myName_;
    if (const Member* m = board_.find(s)) if (!m->name.empty()) return m->name;
    auto it = rows_.find(s);
    if (it != rows_.end() && !it->second.b.name.empty()) return it->second.b.name;
    return "player " + std::to_string((int)s);
}

PlaceFn MarkerService::placeFn(const LocalState& me) const {
    const Slot local = port_.localSlot();
    const Place here = me.valid ? me.at : meAt_;
    return [this, local, here](Slot s, Place& out) {
        if (s == local) { out = here; return here.valid; }
        IMarkerPort& p = port_;
        if (p.puppetPlace(s, out) && out.valid) return true;
        auto it = rows_.find(s);
        if (it == rows_.end() || !it->second.b.at.valid) return false;
        out = it->second.b.at;
        return true;
    };
}

void MarkerService::onSessionEnd() {
    if (!FNV_MUT(1417)) board_.clear();                                          // twin 1417: friends of the last session stay on the map
    rows_.clear();
    auth_.clear();
    hold_.reset();
    localDowned_ = false;
    sentBeacon_ = sentTable_ = false;
    haveSeq_ = false;
    selfRow_ = Member{};
    frame_ = HudFrame{};
    if (hud_ && builtHud_) hud_->show(frame_);
    builtHud_ = false;
}

void MarkerService::onLocalDowned(uint32_t respawnMs, uint64_t nowMs) {
    localDowned_ = true;
    localDownedAt_ = nowMs;
    localWindowMs_ = respawnMs;
    hold_.reset();
}

void MarkerService::onLocalUp(const std::string& byName) {
    const bool was = localDowned_;
    localDowned_ = false;
    localWindowMs_ = 0;
    selfRow_ = Member{};
    hold_.reset();
    if (was && !byName.empty()) port_.notify(byName + " got you back on your feet.");
}

// ---- beacons and the table --------------------------------------------------------------------------------------------------------------------
void MarkerService::sendBeacon(const LocalState& me, uint64_t now) {
    if (FNV_MUT(1401)) return;                                                    // twin 1401: a guest never says where he is
    if (sentBeacon_ && now - lastBeaconMs_ < cfg_.beaconMs) return;
    Beacon b;
    b.name = me.name;
    b.at = me.at;
    b.ext = me.ext;
    if (port_.sendToHost(encodeBeacon(b))) {
        ++stats_.beaconsSent;
        sentBeacon_ = true;
        lastBeaconMs_ = now;
    }
}

std::vector<Member> MarkerService::hostTable(const LocalState& me, uint64_t now) const {
    std::vector<Member> out;
    auto addState = [&](Member& m) {
        m.downed = auth_.isDowned(m.slot) && !FNV_MUT(1402);                     // twin 1402: the table never says who is down
        if (m.downed) {
            m.checkpointOnly = auth_.checkpointOnly(m.slot);
            m.windowLeftMs = auth_.windowLeftMs(m.slot, now);
            m.reviver = auth_.reviverOf(m.slot);
            m.holdLeftMs = auth_.holdLeftMs(m.slot, now);
        }
    };
    Member self;
    self.slot = port_.localSlot();
    self.name = me.name;
    self.at = me.at;
    self.ext = me.ext;
    addState(self);
    out.push_back(self);
    for (const auto& kv : rows_) {
        if (now - kv.second.seenMs > cfg_.markers.staleMs * 2 && !FNV_MUT(1411)) continue;   // twin 1411: a guest who stopped reporting stays in the table
        Member m;
        m.slot = kv.first;
        m.name = kv.second.b.name;
        m.at = kv.second.b.at;
        m.ext = kv.second.b.ext;
        addState(m);
        out.push_back(m);
    }
    return out;
}

void MarkerService::hostFrame(const LocalState& me, uint64_t now) {
    const Slot local = port_.localSlot();
    // guests who left
    std::vector<Slot> joined = port_.guests();
    for (auto it = rows_.begin(); it != rows_.end();) {
        if (std::find(joined.begin(), joined.end(), it->first) == joined.end() && !FNV_MUT(1412)) {   // twin 1412: a guest who left keeps his row
            board_.remove(it->first);
            auth_.forget(it->first);
            it = rows_.erase(it);
        } else {
            ++it;
        }
    }
    // the rules engine's downed list into the authority
    const uint32_t window = port_.reviveWindowMs();
    auth_.sync(local, port_.ruleDowned(local), port_.ruleCheckpointOnly(local), window, now);
    for (Slot s : joined) auth_.sync(s, port_.ruleDowned(s), port_.ruleCheckpointOnly(s), window, now);
    const PlaceFn where = placeFn(me);
    auth_.tick(now, where);
    processRevived(now);

    // the host's board: everybody else, with the state the authority holds
    const std::vector<Member> table = hostTable(me, now);
    selfRow_ = table.front();
    selfRowAt_ = now;
    for (const Member& m : table)
        if (m.slot != local) board_.set(m, now);
    std::vector<Slot> silent;   // on the board but no longer in the table: stopped reporting
    for (const auto& kv : board_.members()) {
        bool inTable = false;
        for (const Member& m : table) inTable = inTable || m.slot == kv.first;
        if (!inTable) silent.push_back(kv.first);
    }
    for (Slot s : silent) board_.remove(s);
    if (!sentTable_ || now - lastTableMs_ >= cfg_.tableMs) {
        PartyTable t;
        t.seq = ++seq_;
        t.members = table;
        const std::string json = encodeParty(t);
        for (Slot s : joined)
            if (port_.sendToGuest(s, json)) ++stats_.tablesSent;
        sentTable_ = true;
        lastTableMs_ = now;
    }
}

void MarkerService::processRevived(uint64_t now) {
    (void)now;
    for (const ReviveAuthority::Revived& r : auth_.takeRevived()) {
        ++stats_.revivesGranted;
        if (!FNV_MUT(1405)) port_.reviveInPlace(FNV_MUT(1406) ? r.reviver : r.target, auth_.rules().healthFraction, nameOf(r.reviver));   // twin 1405: granted, never done; twin 1406: the wrong player gets up
        if (FNV_MUT(1416)) continue;                                              // twin 1416: the friend is not told
        const std::string line = "You got " + nameOf(r.target) + " back on their feet.";
        if (r.reviver == port_.localSlot()) port_.notify(line);
        else port_.notifyGuest(r.reviver, line);
    }
}

std::string MarkerService::onHostOrder(Slot from, const std::string& json, uint64_t now) {
    if (!port_.isHost()) return "NotHost";
    // each decoder checks the op name itself (tiny messages)
    Beacon b;
    ReviveMsg r;
    if (decodeBeacon(json, b)) {
        const std::vector<Slot> joined = port_.guests();
        if ((from == port_.localSlot() || std::find(joined.begin(), joined.end(), from) == joined.end()) && !FNV_MUT(1403)) {   // twin 1403: any slot can write a row, the host's own too
            ++stats_.badOrders;
            return "NotAGuest";
        }
        Row& row = rows_[from];
        row.b = b;
        row.seenMs = now;
        ++stats_.beaconsGot;
        return "Ok";
    }
    if (decodeRevive(json, r)) {
        if (!cfg_.revive.enabled && !FNV_MUT(1419)) { lastVerdict_ = Verdict::Disabled; return verdictName(Verdict::Disabled); }   // twin 1419: the switch only hides the prompt
        const Slot reviver = FNV_MUT(1404) ? r.target : from;                    // twin 1404: the sender is whoever the message says
        lastVerdict_ = auth_.request(reviver, r.phase, r.target, now, placeFn(LocalState{}));
        if (r.phase == RevivePhase::Done) {
            if (lastVerdict_ != Verdict::Ok) ++stats_.revivesRefused;
            processRevived(now);
        }
        return lastVerdict_ == Verdict::Ok ? "Ok" : verdictName(lastVerdict_);
    }
    ++stats_.badOrders;
    return "BadOrder";
}

void MarkerService::applyTable(const PartyTable& t, uint64_t now) {
    const Slot local = port_.localSlot();
    std::vector<Slot> present;
    for (const Member& m : t.members) {
        present.push_back(m.slot);
        if (m.slot == local) { selfRow_ = m; selfRowAt_ = now; continue; }
        board_.set(m, now);
    }
    std::vector<Slot> gone;
    for (const auto& kv : board_.members())
        if (std::find(present.begin(), present.end(), kv.first) == present.end()) gone.push_back(kv.first);
    for (Slot s : gone) board_.remove(s);
}

std::string MarkerService::onGuestOrder(const std::string& json, uint64_t now) {
    PartyTable t;
    if (!decodeParty(json, t)) {
        ++stats_.badOrders;
        return "BadOrder";
    }
    if (haveSeq_ && static_cast<int32_t>(t.seq - lastSeq_) <= 0 && !FNV_MUT(1407)) return "Ok";   // twin 1407: an older table that arrives late wins
    haveSeq_ = true;
    lastSeq_ = t.seq;
    ++stats_.tablesGot;
    applyTable(t, now);
    return "Ok";
}

// ---- revive: the friend's hold ---------------------------------------------------------------------------------------------------------------
void MarkerService::sendRevive(RevivePhase ph, Slot target, uint64_t now) {
    if (port_.isHost()) {
        if (FNV_MUT(1409)) return;                                                // twin 1409: the host's own revive goes nowhere
        lastVerdict_ = auth_.request(port_.localSlot(), ph, target, now, placeFn(LocalState{}));
        if (ph == RevivePhase::Done) {
            if (lastVerdict_ != Verdict::Ok) ++stats_.revivesRefused;
            processRevived(now);
        }
        return;
    }
    port_.sendToHost(encodeRevive({ph, target}));
}

void MarkerService::reviveFrame(const LocalState& me, uint64_t now) {
    if (!cfg_.revive.enabled && !FNV_MUT(1419)) { hold_.reset(); return; }
    const Slot local = port_.localSlot();
    HoldInput in;
    in.now = now;
    in.selfUp = !localDowned_ && me.alive;
    in.activateHeld = me.activateHeld;
    in.blocked = me.uiBlocked;
    for (const auto& kv : board_.members()) {
        const Member& m = kv.second;
        if (m.slot == local) continue;
        Place at = m.at;
        port_.puppetPlace(m.slot, at);
        if (!at.valid) at = m.at;
        if (!sameSpace(me.at, at) && !FNV_MUT(1413)) continue;                    // twin 1413: a friend elsewhere is offered for a revive
        HoldCandidate c;
        c.slot = m.slot;
        c.distUnits = sameSpace(me.at, at) ? distance3(me.at, at) : 0.0f;
        c.revivable = revivable(m);
        c.busyWithOther = m.reviver != kNoSlot && m.reviver != local && !FNV_MUT(1414);   // twin 1414: two friends hold on the same downed player
        in.candidates.push_back(c);
    }
    const HoldOut out = hold_.step(in);
    if (out.ev == HoldEvent::Begin) { ++stats_.holdsBegun; sendRevive(RevivePhase::Begin, out.target, now); }
    else if (out.ev == HoldEvent::Cancel) { ++stats_.holdsCancelled; sendRevive(RevivePhase::Cancel, out.target, now); }
    else if (out.ev == HoldEvent::Done) { ++stats_.holdsDone; sendRevive(RevivePhase::Done, out.target, now); }
}

// ---- HUD ---------------------------------------------------------------------------------------------------------------------------------------
void MarkerService::buildHud(const LocalState& me, uint64_t now) {
    HudFrame f;
    f.local = localDowned_;
    if (cfg_.markers.enabled) {
        MarkerBoard::Puppets pup;
        for (const auto& kv : board_.members()) {
            Place p;
            if (port_.puppetPlace(kv.first, p) && p.valid) pup.at[kv.first] = p;
        }
        const std::vector<MarkerView> views = board_.views(port_.localSlot(), me.at, me.ext, me.heading, now, pup);
        for (const MarkerView& v : views) {
            if (v.tag && !me.uiBlocked) f.tags.push_back({v.slot, v.tagText, v.tagAlpha, v.tagAt});
            if (v.compass) f.compass.push_back({v.slot, v.compassGlyph, v.compassX, v.edge, v.downed});
            if (v.localMap || v.worldMap) f.map.push_back({v.slot, v.label, v.localMap, v.worldMap, v.mapAt, v.downed});
            std::string line = v.label;
            if (const Member* m = board_.find(v.slot); m && revivable(*m)) line += " [" + clockText(m->windowLeftMs) + "]";
            f.list.push_back(line);
        }
    }
    // the banner: my own downed timer, the hold I am making, or the prompt
    if (cfg_.revive.enabled) {
        if (localDowned_) {
            uint32_t left = localWindowLeftMs(now);
            if (selfRow_.slot != kNoSlot && selfRow_.downed) {   // the host's figure, counted down since it arrived
                const uint64_t age = now - selfRowAt_;
                left = age >= selfRow_.windowLeftMs ? 0u : static_cast<uint32_t>(selfRow_.windowLeftMs - age);
            }
            const bool can = left > 0 && !selfRow_.checkpointOnly;
            if (can) {
                f.banner = "You are down. A friend can get you up: " + clockText(left);
                if (selfRow_.reviver != kNoSlot) {
                    f.banner += " - " + nameOf(selfRow_.reviver) + " is helping";
                    const uint32_t total = std::max<uint32_t>(1, cfg_.hold.holdMs);
                    f.bannerProgress = 1.0f - std::min(1.0f, static_cast<float>(selfRow_.holdLeftMs) / static_cast<float>(total));
                }
                if (FNV_MUT(1410)) f.banner = "You are down.";                     // twin 1410: no timer
            } else {
                f.banner = "You are down. You get up at the party's last checkpoint.";
            }
        } else if (hold_.active()) {
            const float p = hold_.progress(now);
            f.banner = "Reviving " + nameOf(hold_.target()) + "... " + std::to_string((int)(p * 100.0f)) + "%";
            f.bannerProgress = p;
        } else if (hold_.candidate() != kNoSlot) {
            std::string s = "Hold the activate key to get " + nameOf(hold_.candidate()) + " up";
            if (const Member* m = board_.find(hold_.candidate())) s += " (" + clockText(m->windowLeftMs) + " left)";
            f.banner = s;
        }
    }
    frame_ = std::move(f);
    if (hud_) hud_->show(frame_);
    builtHud_ = true;
}

void MarkerService::tick(uint64_t now) {
    LocalState me;
    if (!port_.readLocal(me) || !me.valid) {
        if (builtHud_ && hud_) { frame_ = HudFrame{}; hud_->show(frame_); builtHud_ = false; }
        return;
    }
    myName_ = me.name;
    meAt_ = me.at;
    if (!cfg_.markers.enabled && !cfg_.revive.enabled) {
        if (builtHud_ && hud_) { frame_ = HudFrame{}; hud_->show(frame_); builtHud_ = false; }
        return;
    }
    if (port_.isHost()) hostFrame(me, now);
    else sendBeacon(me, now);
    reviveFrame(me, now);
    buildHud(me, now);
}

std::string MarkerService::statusJson(uint64_t now) const {
    char b[512];
    std::snprintf(b, sizeof b,
        "{\"members\":%u,\"localDowned\":%s,\"windowLeftMs\":%u,\"holding\":%d,\"beaconsSent\":%llu,\"beaconsGot\":%llu,\"tablesSent\":%llu,\"tablesGot\":%llu,"
        "\"badOrders\":%llu,\"revivesGranted\":%llu,\"revivesRefused\":%llu,\"holdsBegun\":%llu,\"holdsDone\":%llu,\"holdsCancelled\":%llu,\"lastVerdict\":\"%s\"}",
        (unsigned)board_.members().size(), localDowned_ ? "true" : "false", (unsigned)localWindowLeftMs(now), hold_.active() ? (int)hold_.target() : -1,
        (unsigned long long)stats_.beaconsSent, (unsigned long long)stats_.beaconsGot, (unsigned long long)stats_.tablesSent, (unsigned long long)stats_.tablesGot,
        (unsigned long long)stats_.badOrders, (unsigned long long)stats_.revivesGranted, (unsigned long long)stats_.revivesRefused,
        (unsigned long long)stats_.holdsBegun, (unsigned long long)stats_.holdsDone, (unsigned long long)stats_.holdsCancelled, verdictName(lastVerdict_));
    return b;
}

}  // namespace nvmk
