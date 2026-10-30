#include "avatar_runtime.h"

#include <algorithm>
#include <cmath>
#include <cstdio>

#include "../live_bridge.h"
#include "fnvcoop_mut.h"

// Mutation twins in this file (tests/test_engine.cpp must fail for each):
// 60 loopback offset ignored            61 remote snapshots all go to the first remote   62 game events ignored
// 63 local publishing during a load     64 pose set before the first snapshot arrived    65 engine position not fed back to the buffer
// 66 status lacks the tagged count      67 equipment not forwarded to the loopback       68 loopback off leaves the puppet
// 69 local identity ignored             70 every snapshot is handed to the sink twice   71 a save that never reports back keeps puppets away for ever

namespace nvcoop::avatar::engine {

namespace {
EngineOptions MakeEngineOptions(const RuntimeConfig& c) {
    EngineOptions o = c.engine;
    for (const TemplateEntry& t : c.puppet.templates)
        if (t.base && std::find(o.taggedBases.begin(), o.taggedBases.end(), t.base) == o.taggedBases.end()) o.taggedBases.push_back(t.base);
    if (c.puppet.defaultTemplate && std::find(o.taggedBases.begin(), o.taggedBases.end(), c.puppet.defaultTemplate) == o.taggedBases.end())
        o.taggedBases.push_back(c.puppet.defaultTemplate);
    return o;
}
std::string Hex8(uint32_t v) {
    char b[16];
    std::snprintf(b, sizeof b, "0x%08X", v);
    return b;
}
}  // namespace

AvatarRuntime::AvatarRuntime(IGameBackend& be, RuntimeConfig cfg)
    : be_(be), cfg_(std::move(cfg)), eng_(be, MakeEngineOptions(cfg_)), pm_(eng_, cfg_.puppet), sampler_(cfg_.sampler), equip_(cfg_.equipPollMs) {}

void AvatarRuntime::SetLog(std::function<void(const std::string&)> log) {
    log_ = log;
    pm_.SetLog(std::move(log));
}

void AvatarRuntime::SetLocalIdentity(live::NetId netId, live::OwnerEpoch epoch) {
    netId_ = netId;
    if (!FNV_MUT(69)) sampler_.SetIdentity(netId, epoch);
}

// ---- remote players --------------------------------------------------------------------------------------------------

void AvatarRuntime::SetLocalSlot(Slot slot) {
    cfg_.puppet.localSlot = slot;
    if (!loopback_) cfg_.loopbackSlot = static_cast<Slot>(slot == 1 ? 2 : 1);
    pm_.SetLocalSlot(slot);
    remotes_.erase(slot);   // nobody streams the local player to himself
}

bool AvatarRuntime::AddRemote(Slot slot, const Appearance& a) {
    if (!pm_.AddPlayer(slot, a)) return false;
    remotes_[slot] = std::make_unique<Remote>(cfg_.interp);   // a re-added slot (reconnect) starts with an empty buffer
    return true;
}

void AvatarRuntime::RemoveRemote(Slot slot) {
    pm_.RemovePlayer(slot);
    remotes_.erase(slot);
}

bool AvatarRuntime::SetRemoteAppearance(Slot slot, const Appearance& a) {
    if (!remotes_.count(slot)) return false;
    return pm_.SetAppearance(slot, a);   // the interpolation buffer keeps running: the (new) puppet gets its pose on the next Tick
}

bool AvatarRuntime::ReadLocalAppearance(Appearance& out) { return be_.ReadAppearance(kPlayerRef, out); }

bool AvatarRuntime::ReadLocalEquipment(std::vector<EquipEntry>& out) {
    if (!equip_.hasPublished()) return false;
    out = equip_.published();
    return true;
}

bool AvatarRuntime::ReadLocalFacts(PlayerFacts& out) {
    PlayerFacts f;
    bool any = false;
    double v = 0;
    if (be_.Eval(Cond::GetLevel, kPlayerRef, 0, v) && v >= 0 && v < 65536) { f.level = static_cast<uint16_t>(v); any = true; }
    if (be_.Eval(Cond::GetSitting, kPlayerRef, 0, v) && v >= 0 && v < 256) { f.sitting = static_cast<uint8_t>(v); any = true; }
    for (unsigned id = 0; id < kFactActionIds && !FNV_MUT(77); ++id)   // twin 77: no action is ever sampled (IsPlayerActionActive stays unknown for a guest)
        if (be_.Eval(Cond::IsPlayerActionActive, kPlayerRef, id, v)) { any = true; if (v > 0.5) f.actions |= 1u << id; }
    if (be_.ReadPlayerRegions(f.regions, kMaxFactRegions)) any = true;
    if (!any) return false;
    out = std::move(f);
    return true;
}

// Twin 950: the weight is never read (the Jed gate sees no weight of a guest). 957: items the guest does not carry are listed with a zero.
bool AvatarRuntime::ReadLocalPack(const std::vector<FormId>& watch, PlayerPack& out) {
    PlayerPack p;
    p.watchId = packWatchId(watch);
    bool any = false;
    double v = 0;
    if (!FNV_MUT(950) && be_.Eval(Cond::GetActorValue, kPlayerRef, kAvInventoryWeight, v) && v >= 0 && v < 4e6) {
        p.haveWeight = true;
        p.weightMilli = static_cast<uint32_t>(v * 1000.0 + 0.5);
        any = true;
    }
    std::vector<FormId> forms = watch;
    std::sort(forms.begin(), forms.end());
    forms.erase(std::unique(forms.begin(), forms.end()), forms.end());
    for (FormId f : forms) {
        if (!f || p.items.size() >= kMaxPackItems) continue;
        if (!be_.Eval(Cond::GetItemCount, kPlayerRef, f, v)) continue;
        any = true;
        if (v >= 0.5 || FNV_MUT(957)) p.items.push_back(PackEntry{f, v > 2e9 ? 2000000000 : static_cast<int32_t>(v + 0.5)});
    }
    if (!any) return false;
    out = std::move(p);
    return true;
}

void AvatarRuntime::OnRemoteSnapshot(Slot slot, const live::ActorSnapshot& s, uint32_t arrivalMs) {
    auto it = remotes_.find(slot);
    if (FNV_MUT(61) && !remotes_.empty()) it = remotes_.begin();
    if (it == remotes_.end()) {
        ++droppedRemote_;
        return;
    }
    it->second->buf.push(s, arrivalMs);
    ++remoteSnaps_;
}

void AvatarRuntime::OnRemoteEquipment(Slot slot, const std::vector<EquipEntry>& wanted) { pm_.SetEquipment(slot, wanted); }

// ---- loopback ----------------------------------------------------------------------------------------------------------

bool AvatarRuntime::SetLoopback(bool on) {
    if (on == loopback_) return true;
    const Slot slot = cfg_.loopbackSlot;
    if (on) {
        if (slot == cfg_.puppet.localSlot) return false;
        Appearance a;
        be_.ReadAppearance(kPlayerRef, a);   // best effort: the template decides race / sex
        a.name = a.name.empty() ? "loopback" : a.name;
        if (!AddRemote(slot, a)) return false;
        if (!equip_.published().empty()) pm_.SetEquipment(slot, equip_.published());
        loopback_ = true;
        return true;
    }
    loopback_ = false;
    if (!FNV_MUT(68)) RemoveRemote(slot);
    return true;
}

// ---- per frame -----------------------------------------------------------------------------------------------------------

// The owner can read next run whether his own attacks were seen and handed to the net layer (nvcoop.log). Capped: an attack every few seconds is the normal rate.
void AvatarRuntime::LogLocalAction(const live::ActorSnapshot& s) {
    const LocalFrame& f = sampler_.lastFrame();
    // which animation sets can be read (a change is worth a line: 1 = only the process's, 2/3 = the first-person / alternate set is there too)
    const int setState = f.setCount * 4 + (f.firstPerson ? 1 : 0);
    if (setState != loggedSetState_) {
        loggedSetState_ = setState;
        Log("avatar: local attack source: " + std::to_string(f.setCount) + " animation set(s) readable, camera " + (f.firstPerson ? "first person" : "third person"));
    }
    const auto& c = sampler_.counters();
    if (c.actionEvents != loggedActionEvents_) {
        loggedActionEvents_ = c.actionEvents;
        const auto& e = sampler_.lastActionEvent();
        if (actionLogs_++ < 400) {
            const char* name = GroupName(e.group);
            const char* setName = e.set == kSetFirstPerson ? "first-person set" : e.set == kSetAlt ? "alternate set" : "third-person set";
            char buf[240];
            std::snprintf(buf, sizeof buf, "avatar: local attack seen: group %d (%s) from the %s, camera %s, process action %d -> sent as event %u in snapshot %u",
                          e.group, name ? name : "?", setName, e.firstPerson ? "first person" : "third person", e.procAction, (unsigned)s.anim.actionSeq, (unsigned)s.seq);
            Log(buf);
        }
    }
    // The engine's own "what the player is doing" value changing with no animation group behind it is the sign of an attack we cannot see
    if (f.procAction != loggedProcAction_) {
        const int before = loggedProcAction_;
        loggedProcAction_ = f.procAction;
        if (actionLogs_ < 400 && f.procAction >= 0 && f.actionGroup < 0) {
            ++actionLogs_;
            Log("avatar: local process action " + std::to_string(before) + " -> " + std::to_string(f.procAction) + " with no action group playing in any readable set");
        }
    }
}

void AvatarRuntime::PublishLocal(uint32_t nowMs) {
    if (loading_ && !FNV_MUT(63)) return;
    live::ActorSnapshot s;
    if (sampler_.Sample(be_, nowMs, s)) {
        lastSnap_ = s;
        haveSnap_ = true;
        if (sampler_.lastFrame().havePos3D) {
            const Vec3 a = sampler_.lastFrame().pos, b = sampler_.lastFrame().pos3D;
            stages_.localDrift3D.add(std::sqrt((a.x - b.x) * (a.x - b.x) + (a.y - b.y) * (a.y - b.y) + (a.z - b.z) * (a.z - b.z)));
        }
        LogLocalAction(s);
        traceLocal_.push_back(LocalPt{s.tick, s.pos.x, s.pos.y, s.pos.z, s.vel.x, s.vel.y, s.vel.z});
        if (traceLocal_.size() > kTraceLocal) traceLocal_.pop_front();
        if (snapSink_) {
            snapSink_(s);
            if (FNV_MUT(70)) snapSink_(s);
        }
        if (loopback_) {
            auto it = remotes_.find(cfg_.loopbackSlot);
            if (it != remotes_.end()) {
                live::ActorSnapshot l = s;
                if (!FNV_MUT(60)) {
                    l.pos.x += cfg_.loopbackOffset.x;
                    l.pos.y += cfg_.loopbackOffset.y;
                    l.pos.z += cfg_.loopbackOffset.z;
                }
                it->second->buf.push(l, nowMs);
            }
        }
    }
    std::vector<EquipEntry> eq;
    if (equip_.Poll(be_, nowMs, eq)) {
        if (equipSink_) equipSink_(eq);
        if (loopback_ && !FNV_MUT(67)) pm_.SetEquipment(cfg_.loopbackSlot, eq);
    }
}

void AvatarRuntime::DriveRemotes(uint32_t nowMs) {
    for (auto& kv : remotes_) {
        const Slot slot = kv.first;
        Remote& r = *kv.second;
        CellId c = 0;
        Vec3 p;
        live::ActualPose ap;
        bool haveActual = !FNV_MUT(65) && pm_.ActualPose(slot, c, p);
        if (haveActual) {
            ap.cell = c;
            ap.pos = ToLive(p);
        }
        live::InterpOut o = r.buf.sample(nowMs, haveActual ? &ap : nullptr);
        if (o.state == live::InterpState::Empty && !FNV_MUT(64)) continue;   // nothing received yet: no puppet at a made-up place
        pm_.SetPose(slot, FromInterp(o));
        {
            auto& tr = traceRemote_[slot];
            tr.push_back(RemotePt{nowMs, o.pos.x, o.pos.y, o.pos.z, haveActual ? ap.pos.x : 0.0f, haveActual ? ap.pos.y : 0.0f, haveActual ? ap.pos.z : 0.0f,
                                  static_cast<uint8_t>((haveActual ? 16 : 0) | static_cast<uint8_t>(o.state)),
                                  static_cast<float>(r.buf.diag().delayInUse.last), static_cast<float>(r.buf.diag().wantDelayMs), static_cast<float>(r.buf.diag().jitterMs),
                                  static_cast<float>(r.buf.diag().newestAgeMs)});
            if (tr.size() > kTraceRemote) tr.pop_front();
        }
        ++posesFed_;
        r.everSampled = true;
    }
}

void AvatarRuntime::NoteStage(NetStage st, double ms) {
    switch (st) {
        case NetStage::Send: stages_.send.add(ms); break;
        case NetStage::Rx: stages_.rx.add(ms); break;
        case NetStage::NetFrame: {
            const uint32_t now = static_cast<uint32_t>(ms);
            stages_.netPhase.add(static_cast<double>(static_cast<int32_t>(now - lastTick_)));
            if (haveNetFrame_) stages_.netGap.add(static_cast<double>(static_cast<int32_t>(now - lastNetFrame_)));
            lastNetFrame_ = now;
            haveNetFrame_ = true;
            break;
        }
    }
}

void AvatarRuntime::Tick(uint32_t nowMs) {
    lastTick_ = nowMs;
    const uint64_t u0 = be_.NowUs();
    if (haveTickUs_ && u0 >= lastTickUs_) stages_.frameGap.add(static_cast<double>(u0 - lastTickUs_) / 1000.0);
    lastTickUs_ = u0;
    haveTickUs_ = true;
    if (saveHeld_ && !FNV_MUT(71) && static_cast<int32_t>(nowMs - saveHeldAt_) >= static_cast<int32_t>(cfg_.saveHoldMs)) {
        Log("avatar: no SaveEnd after " + std::to_string(cfg_.saveHoldMs) + " ms, puppets may return");
        saveHeld_ = false;
        pm_.OnSaveEnd();
    }
    PublishLocal(nowMs);
    const uint64_t u1 = be_.NowUs();
    DriveRemotes(nowMs);
    const uint64_t u2 = be_.NowUs();
    pm_.Tick(nowMs);
    const uint64_t u3 = be_.NowUs();
    stages_.sampleUs.add(static_cast<double>(u1 - u0));
    stages_.driveUs.add(static_cast<double>(u2 - u1));
    stages_.applyUs.add(static_cast<double>(u3 - u2));
    const uint32_t sp = static_cast<uint32_t>(pm_.stats().setPos);
    if (sp != lastSetPosCount_) stages_.poseToSetPosMs.add(static_cast<double>(u3 - u0) / 1000.0);
    lastSetPosCount_ = sp;
}

void AvatarRuntime::OnEvent(GameEvent e) {
    if (FNV_MUT(62)) return;
    switch (e) {
        case GameEvent::SaveBegin:
            pm_.OnSaveBegin();
            saveHeld_ = true;
            saveHeldAt_ = lastTick_;
            break;
        case GameEvent::SaveEnd:
            saveHeld_ = false;
            pm_.OnSaveEnd();
            break;
        case GameEvent::LoadBegin:
            loading_ = true;
            pm_.OnLoadBegin();
            sampler_.Reset();
            equip_.Reset();
            for (auto& kv : remotes_) kv.second->buf.reset();   // positions from the old world mean nothing
            break;
        case GameEvent::LoadEnd:
            loading_ = false;
            pm_.OnLoadEnd();
            sampler_.Reset();
            break;
        case GameEvent::NewGame:
        case GameEvent::ExitToMenu:
            // the world is replaced without a Pre / PostLoad pair: drop every ref without touching the engine, purge when it is back
            pm_.OnLoadBegin();
            pm_.OnLoadEnd();
            sampler_.Reset();
            equip_.Reset();
            for (auto& kv : remotes_) kv.second->buf.reset();
            loading_ = false;
            break;
    }
}

// ---- status ----------------------------------------------------------------------------------------------------------------

std::string AvatarRuntime::StatusJson() {
    std::string s = pm_.StatusJson();
    if (!s.empty() && s.back() == '}') s.pop_back();
    char buf[400];
    std::snprintf(buf, sizeof buf, ",\"v\":1,\"enabled\":true,\"loopback\":%s,\"loopbackSlot\":%d,\"worldEpoch\":%u,\"remoteSnapshots\":%llu,\"droppedRemote\":%llu,\"posesFed\":%llu",
                  loopback_ ? "true" : "false", (int)cfg_.loopbackSlot, (unsigned)be_.WorldEpoch(), (unsigned long long)remoteSnaps_,
                  (unsigned long long)droppedRemote_, (unsigned long long)posesFed_);
    s += buf;
    // the playback mode the receivers were built with (the per-remote figure in "stages" is what each buffer really runs)
    s += std::string(",\"lowLatency\":") + (cfg_.interp.lowLatency ? "true" : "false") + ",\"samplerPeriodMs\":" + std::to_string(cfg_.sampler.periodMs);
    if (!FNV_MUT(66)) s += ",\"tagged\":" + std::to_string(eng_.FindTaggedPuppets().size());
    s += ",\"engine\":" + eng_.StatsJson();
    s += ",\"local\":" + LocalJson();
    s += ",\"stages\":" + StagesJson();
    s += "}";
    return s;
}

std::string AvatarRuntime::TraceJson(size_t n) const {
    std::string s = "{\"clock\":" + std::to_string(lastTick_) + ",\"local\":[";
    char b[160];
    const size_t l0 = traceLocal_.size() > n ? traceLocal_.size() - n : 0;
    for (size_t i = l0; i < traceLocal_.size(); ++i) {
        const LocalPt& p = traceLocal_[i];
        std::snprintf(b, sizeof b, "%s[%u,%.1f,%.1f,%.1f,%.1f,%.1f,%.1f]", i == l0 ? "" : ",", (unsigned)p.t, p.x, p.y, p.z, p.vx, p.vy, p.vz);
        s += b;
    }
    s += "],\"remote\":{";
    bool firstSlot = true;
    for (const auto& kv : traceRemote_) {
        if (!firstSlot) s += ",";
        firstSlot = false;
        s += "\"" + std::to_string((int)kv.first) + "\":[";
        const auto& d = kv.second;
        const size_t r0 = d.size() > n * 2 ? d.size() - n * 2 : 0;
        for (size_t i = r0; i < d.size(); ++i) {
            const RemotePt& p = d[i];
            std::snprintf(b, sizeof b, "%s[%u,%.1f,%.1f,%.1f,%.1f,%.1f,%.1f,%u,%.0f,%.0f,%.0f,%.0f]", i == r0 ? "" : ",", (unsigned)p.t, p.x, p.y, p.z, p.ax, p.ay, p.az, (unsigned)p.state,
                      p.delay, p.want, p.jitter, p.newestAge);
            s += b;
        }
        s += "]";
    }
    s += "}}";
    return s;
}

// avatar.status "stages": every figure is a gauge {n, last, mean, max, sum} (mean is since the start; the difference of two sums over the difference of two
// counts is the mean over the stretch between two readings). ms unless the name ends in Us.
//   send      sampling time -> handed to the transport (sender)            rx        sampling time -> in the other copy's network layer (shared clock)
//   hold      arrival stamp -> the first drive that could use it           delay     the buffer delay in use (render age minus the clock offset)
//   applyUs   engine calls of one tick                                      poseToSetPosMs  start of the tick -> end of the engine calls, ticks that moved a puppet
std::string AvatarRuntime::StagesJson() const {
    std::string s = "{\"send\":" + live::gaugeJson(stages_.send) + ",\"rx\":" + live::gaugeJson(stages_.rx) + ",\"netPhase\":" + live::gaugeJson(stages_.netPhase) +
                    ",\"netGap\":" + live::gaugeJson(stages_.netGap) +
                    ",\"frameGap\":" + live::gaugeJson(stages_.frameGap) + ",\"sampleUs\":" + live::gaugeJson(stages_.sampleUs) +
                    ",\"driveUs\":" + live::gaugeJson(stages_.driveUs) + ",\"applyUs\":" + live::gaugeJson(stages_.applyUs) +
                    ",\"localDrift3D\":" + live::gaugeJson(stages_.localDrift3D) + ",\"poseToSetPosMs\":" + live::gaugeJson(stages_.poseToSetPosMs) + ",\"remotes\":[";
    bool first = true;
    for (const auto& kv : remotes_) {
        const live::InterpDiag& d = kv.second->buf.diag();
        char b[420];
        std::snprintf(b, sizeof b,
                      "{\"slot\":%d,\"lowLatency\":%s,\"snapshots\":%zu,\"pushes\":%llu,\"samples\":%llu,\"offsetMs\":%d,\"jitterMs\":%.1f,\"frameMs\":%.1f,"
                      "\"wantDelayMs\":%.1f,\"lagErrMs\":%.1f,\"renderAgeMs\":%d,\"newestAgeMs\":%d,\"extrapolating\":%llu,\"holding\":%llu,\"snaps\":%llu,"
                      "\"teleportSnaps\":%llu,\"eased\":%llu,",
                      (int)kv.first, d.lowLatency ? "true" : "false", kv.second->buf.size(), (unsigned long long)d.pushes, (unsigned long long)d.samples,
                      (int)d.offsetMs, d.jitterMs, d.frameMs, d.wantDelayMs, d.lagErrMs, (int)d.renderAgeMs, (int)d.newestAgeMs,
                      (unsigned long long)d.extrapolating, (unsigned long long)d.holding, (unsigned long long)d.snaps, (unsigned long long)d.teleportSnaps,
                      (unsigned long long)d.eased);
        if (!first) s += ",";
        first = false;
        s += b;
        s += "\"delay\":" + live::gaugeJson(d.delayInUse) + ",\"hold\":" + live::gaugeJson(d.hold) + ",\"extrap\":" + live::gaugeJson(d.extrap) +
             ",\"arrivalGap\":" + live::gaugeJson(d.arrivalGap) + "}";
    }
    s += "]}";
    return s;
}

std::string AvatarRuntime::LocalJson() {
    const LocalFrame& f = sampler_.lastFrame();
    const Vec3 v = sampler_.lastVelocity();
    const auto& c = sampler_.counters();
    char buf[1000];
    std::snprintf(buf, sizeof buf,
                  "{\"have\":%s,\"cell\":\"%s\",\"pos\":[%.1f,%.1f,%.1f],\"rot\":[%.3f,%.3f,%.3f],\"vel\":[%.1f,%.1f,%.1f],\"loco\":%d,"
                  "\"weaponDrawn\":%s,\"sneak\":%s,\"jump\":%s,\"reload\":%s,\"aim\":%s,\"animReadable\":%s,\"published\":%llu,\"notReady\":%llu,"
                  "\"teleports\":%llu,\"cellChanges\":%llu,\"seq\":%u,\"rawAction\":%d,\"action\":%d,\"actionSeq\":%d,"
                  "\"actionSet\":%d,\"sets\":%d,\"firstPerson\":%s,\"procAction\":%d,\"actionEvents\":%llu}",
                  f.valid ? "true" : "false", Hex8(f.cell).c_str(), f.pos.x, f.pos.y, f.pos.z, f.rot.x, f.rot.y, f.rot.z, v.x, v.y, v.z,
                  (int)sampler_.lastLoco(), f.anim.weaponDrawn ? "true" : "false", f.anim.sneak ? "true" : "false",
                  f.anim.jump ? "true" : "false", f.anim.reload ? "true" : "false", f.anim.aim ? "true" : "false",
                  f.animReadable ? "true" : "false", (unsigned long long)c.published, (unsigned long long)c.notReady,
                  (unsigned long long)c.teleports, (unsigned long long)c.cellChanges, haveSnap_ ? (unsigned)lastSnap_.seq : 0u, f.actionGroup,
                  haveSnap_ ? (int)lastSnap_.anim.action : 0, haveSnap_ ? (int)lastSnap_.anim.actionSeq : 0, f.actionSet, f.setCount,
                  f.firstPerson ? "true" : "false", f.procAction, (unsigned long long)c.actionEvents);
    return buf;
}

}  // namespace nvcoop::avatar::engine
