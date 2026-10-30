#include "npc_anim_system.h"

#include <cstdio>

#include "fnvcoop_mut.h"

namespace nvcoop::actors::anim {

NpcAnimSystem::NpcAnimSystem(INpcAnimEngine& engine, IAnimNet& net, const SystemConfig& cfg)
    : eng_(engine), net_(net), cfg_(cfg), sampler_(engine, cfg.sampler), driver_(engine, cfg.driver), watcher_(cfg.corpse) {}

bool NpcAnimSystem::sampleOwned(NetId id, live::ActorSnapshot& snap, uint32_t nowMs) {
    ++stats_.sampledOwned;
    return sampler_.sample(id, snap, nowMs);
}

void NpcAnimSystem::drive(NetId id, const live::InterpOut& out, uint32_t nowMs) {
    ++stats_.drives;
    driver_.drive(id, out, nowMs);
}

void NpcAnimSystem::onDied(NetId id, bool mine, bool fromNetwork, uint32_t nowMs) {
    ++stats_.died;
    sampler_.forget(id);
    driver_.onDeath(id, nowMs, fromNetwork);
    if (mine || FNV_MUT(60)) {   // twin 60: every machine announces the corpse, not only the owner's
        ++stats_.diedMine;
        watcher_.begin(id, nowMs);   // this machine decides where the corpse rests
    }
}

void NpcAnimSystem::onLost(NetId id) {
    sampler_.forget(id);
    // A body that disappears while we are waiting for it to settle is announced where it was last seen.
    if (watcher_.watching(id)) {
        PendingCorpse pc{id, CorpsePose{}, 0};
        if (watcher_.flush(id, 0, &pc.pose)) outbox_.push_back(pc);
    }
    if (FNV_MUT(63)) driver_.forget(id); else driver_.unload(id);   // a death and the corpse pose we wait for survive the cell unloading (twin 63: they do not)
}

void NpcAnimSystem::onRevived(NetId id) {
    watcher_.end(id);
    driver_.revive(id);
    sampler_.forget(id);
}

void NpcAnimSystem::onNetEvent(NetId id, const std::vector<uint8_t>& payload, uint32_t nowMs) {
    ++stats_.netEvents;
    if (payload.empty()) { ++stats_.netEventsBad; return; }
    if (payload[0] == kAnimEventCorpse) {
        CorpsePose p;
        if (!decodeCorpse(payload, p)) { ++stats_.netEventsBad; return; }
        driver_.onCorpse(id, p, nowMs);
        return;
    }
    ++stats_.netEventsBad;   // unknown kind: from a newer build, ignored
}

void NpcAnimSystem::trySendCorpse(PendingCorpse& pc, uint32_t now, bool& done) {
    done = false;
    if (pc.since == 0) pc.since = now ? now : 1u;
    if (net_.sendEvent(pc.id, encodeCorpse(pc.pose))) { ++stats_.corpseSent; done = true; return; }
    ++stats_.corpseSendRefused;
    if (FNV_MUT(61) || elapsed(now, pc.since) > cfg_.corpseResendMs) { ++stats_.corpseDropped; done = true; }   // twin 61: a refused event is never retried
}

// One line every 10 s while any counter moves: the plugin log then shows whether the animation commands LAND (many mismatches / reasserts with commands
// "ok" is what a swallowed command looks like) and what the furniture ladder did, without a bridge query.
void NpcAnimSystem::summary(uint32_t now) {
    if (!nvcoop::actors::traceOn() || (nextSummaryMs_ && static_cast<int32_t>(now - nextSummaryMs_) < 0)) return;
    nextSummaryMs_ = now + 10000u;
    const DriverStats& d = driver_.stats();
    char buf[480];
    std::snprintf(buf, sizeof buf,
        "anim summary: owned samples=%llu | copies drives=%llu commands=%llu refused=%llu unsupported=%llu groups=%llu mismatches=%llu reasserts=%llu waiting3D=%llu | weapon changes=%llu confirmed=%llu verifyFail=%llu | "
        "furniture sits=%llu stands=%llu confirmed=%llu rungsFailed=%llu giveUps=%llu drifts=%llu drops=%llu leavingStuck=%llu resets=%llu/%llu divergences=%llu",
        (unsigned long long)stats_.sampledOwned, (unsigned long long)d.drives, (unsigned long long)d.commands, (unsigned long long)d.refused, (unsigned long long)d.unsupported,
        (unsigned long long)d.groupsPlayed, (unsigned long long)d.groupMismatches, (unsigned long long)d.reasserts, (unsigned long long)d.waiting3D,
        (unsigned long long)d.weaponChanges, (unsigned long long)d.weaponConfirmed, (unsigned long long)d.weaponVerifyFailures,
        (unsigned long long)d.sitsIssued, (unsigned long long)d.standsIssued, (unsigned long long)d.standsConfirmed, (unsigned long long)d.standRungsFailed, (unsigned long long)d.standGiveUps,
        (unsigned long long)d.sitDrifts, (unsigned long long)d.sitDrops, (unsigned long long)d.leavingStuck, (unsigned long long)d.resetsDone, (unsigned long long)d.resetsStarted,
        (unsigned long long)d.divergences);
    if (lastSummary_ == buf) return;
    lastSummary_ = buf;
    if (d.drives || stats_.sampledOwned) nvcoop::actors::trace("%s", buf);
}

void NpcAnimSystem::tick(uint32_t now, const std::function<bool(NetId)>& isMine) {
    driver_.tick(now);
    summary(now);

    // ---- owner side: watch the bodies this machine owns until they stop, then announce the resting place once ----
    for (NetId id : watcher_.ids()) {
        if (!watcher_.due(id, now)) continue;
        CellId cell = 0; Vec3 pos; float yaw = 0;
        NpcReads r;
        if (eng_.GetPose(id, cell, pos, yaw)) {
            unreadableSince_.erase(id);
            const bool rag = eng_.Read(id, r) && r.ragdoll;
            CorpsePose p;
            if (watcher_.feed(id, cell, pos, yaw, rag, now, &p)) outbox_.push_back({id, p, 0});
        } else {
            ++stats_.corpseUnreadable;
            auto it = unreadableSince_.find(id);
            if (it == unreadableSince_.end()) unreadableSince_[id] = now ? now : 1u;
            else if (elapsed(now, it->second) > 1500) {   // gone for good: say where it was last seen
                CorpsePose p;
                if (watcher_.flush(id, now, &p)) outbox_.push_back({id, p, 0});
                unreadableSince_.erase(id);
            }
        }
    }
    for (size_t i = 0; i < outbox_.size();) {
        bool done = false;
        trySendCorpse(outbox_[i], now, done);
        if (done) outbox_.erase(outbox_.begin() + static_cast<long>(i)); else ++i;
    }

    // ---- non-owner side: a copy nobody drives any more and that is now ours is released to its own AI ----
    if (isMine) {
        for (NetId id : driver_.driven()) {
            if (elapsed(now, driver_.lastDriveMs(id)) > cfg_.releaseAfterIdleMs && isMine(id)) {
                if (!FNV_MUT(62)) sampler_.seed(id, driver_.info(id).lastActionSeq);   // twin 62: the counter restarts at 0 after a handoff, the first events are "stale" to everybody
                driver_.release(id);
            }
        }
    }
}

std::string NpcAnimSystem::statsJson() const {
    const SamplerStats& s = sampler_.stats();
    const DriverStats& d = driver_.stats();
    char buf[3200];
    std::snprintf(buf, sizeof buf,
        "{\"sampler\":{\"samples\":%llu,\"unreadable\":%llu,\"actions_started\":%llu,\"actions_restarted\":%llu,\"flickers_absorbed\":%llu},"
        "\"driver\":{\"drives\":%llu,\"commands\":%llu,\"refused\":%llu,\"unsupported\":%llu,\"deferred_budget\":%llu,\"waiting_3d\":%llu,"
        "\"groups_played\":%llu,\"loop_changes\":%llu,\"one_shots_played\":%llu,\"one_shots_missed\":%llu,\"one_shots_stale\":%llu,"
        "\"reasserts\":%llu,\"group_mismatches\":%llu,\"tpose_fixes\":%llu,\"rebuilds_3d\":%llu,\"menu_reasserts\":%llu,\"world_resets\":%llu,"
        "\"weapon_changes\":%llu,\"weapon_confirmed\":%llu,\"weapon_verify_failures\":%llu,\"weapon_ladder_giveups\":%llu,\"weapon_drifts\":%llu,"
        "\"turns\":%llu,\"jumps\":%llu,\"rate_changes\":%llu,\"knockdowns\":%llu,\"knockdown_fallbacks\":%llu,\"get_ups\":%llu,"
        "\"deaths\":%llu,\"kill_retries\":%llu,\"kill_gave_up\":%llu,\"corpse_poses\":%llu,\"corpse_corrections\":%llu,"
        "\"corpse_within_tolerance\":%llu,\"corpse_gave_up\":%llu,\"corpse_missing\":%llu,\"corpse_late\":%llu,\"releases\":%llu,"
        "\"sits\":%llu,\"stands\":%llu,\"stands_confirmed\":%llu,\"stand_give_ups\":%llu,\"sit_drops\":%llu,\"sit_drifts\":%llu,\"sit_no_furniture\":%llu,\"restrains\":%llu,"
        "\"stand_rungs_failed\":%llu,\"leaving_stuck\":%llu,\"resets_started\":%llu,\"resets_done\":%llu,\"resets_refused\":%llu,\"divergences\":%llu},"
        "\"system\":{\"sampled_owned\":%llu,\"drives\":%llu,\"died\":%llu,\"died_mine\":%llu,\"corpse_sent\":%llu,\"corpse_send_refused\":%llu,"
        "\"corpse_unreadable\":%llu,\"corpse_dropped\":%llu,\"net_events\":%llu,\"net_events_bad\":%llu,\"driven\":%zu,\"watching\":%zu}}",
        (unsigned long long)s.samples, (unsigned long long)s.unreadable, (unsigned long long)s.actionsStarted, (unsigned long long)s.actionsRestarted, (unsigned long long)s.flickersAbsorbed,
        (unsigned long long)d.drives, (unsigned long long)d.commands, (unsigned long long)d.refused, (unsigned long long)d.unsupported, (unsigned long long)d.deferredBudget, (unsigned long long)d.waiting3D,
        (unsigned long long)d.groupsPlayed, (unsigned long long)d.loopChanges, (unsigned long long)d.oneShotsPlayed, (unsigned long long)d.oneShotsMissed, (unsigned long long)d.oneShotsStale,
        (unsigned long long)d.reasserts, (unsigned long long)d.groupMismatches, (unsigned long long)d.tposeFixes, (unsigned long long)d.rebuilds3D, (unsigned long long)d.menuReasserts, (unsigned long long)d.worldResets,
        (unsigned long long)d.weaponChanges, (unsigned long long)d.weaponConfirmed, (unsigned long long)d.weaponVerifyFailures, (unsigned long long)d.weaponLadderGiveUps, (unsigned long long)d.weaponDrifts,
        (unsigned long long)d.turnsPlayed, (unsigned long long)d.jumpsPlayed, (unsigned long long)d.rateChanges, (unsigned long long)d.knockdowns, (unsigned long long)d.knockdownFallbacks, (unsigned long long)d.getUps,
        (unsigned long long)d.deaths, (unsigned long long)d.killRetries, (unsigned long long)d.killGaveUp, (unsigned long long)d.corpsePoses, (unsigned long long)d.corpseCorrections,
        (unsigned long long)d.corpseWithinTolerance, (unsigned long long)d.corpseGaveUp, (unsigned long long)d.corpseMissing, (unsigned long long)d.corpseLate, (unsigned long long)d.releases,
        (unsigned long long)d.sitsIssued, (unsigned long long)d.standsIssued, (unsigned long long)d.standsConfirmed, (unsigned long long)d.standGiveUps,
        (unsigned long long)d.sitDrops, (unsigned long long)d.sitDrifts, (unsigned long long)d.sitNoFurniture, (unsigned long long)d.restrains,
        (unsigned long long)d.standRungsFailed, (unsigned long long)d.leavingStuck, (unsigned long long)d.resetsStarted, (unsigned long long)d.resetsDone, (unsigned long long)d.resetsRefused, (unsigned long long)d.divergences,
        (unsigned long long)stats_.sampledOwned, (unsigned long long)stats_.drives, (unsigned long long)stats_.died, (unsigned long long)stats_.diedMine, (unsigned long long)stats_.corpseSent, (unsigned long long)stats_.corpseSendRefused,
        (unsigned long long)stats_.corpseUnreadable, (unsigned long long)stats_.corpseDropped, (unsigned long long)stats_.netEvents, (unsigned long long)stats_.netEventsBad,
        driver_.driven().size(), watcher_.count());
    return buf;
}

}  // namespace nvcoop::actors::anim
