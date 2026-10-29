#include "rebalance/director.h"

#include <cstdio>

namespace nvcoop::rebalance {

Director::Director(const DirectorConfig& cfg, DirectorEnv env, IGmst& gmst, IEffectPort& effects)
    : cfg_(cfg), env_(std::move(env)), applier_(gmst), effects_(effects), ramp_(cfg.rampMs), fights_(env_.fight) {}

void Director::say(const std::string& line) {
    ++hudLines_;
    if (env_.hud) env_.hud(line);
    if (env_.log) env_.log("rebalance: " + line);
}

void Director::tick() {
    if (!cfg_.enabled) return;
    const uint64_t now = env_.nowMs ? env_.nowMs() : 0;
    const bool ready = env_.worldReady ? env_.worldReady() : false;
    if (!ready) {   // a game that is loading or at the main menu: the next one is applied at once
        if (wasReady_) { seenSettings_ = false; haveAnnounced_ = false; announcedPlayers_ = -1; fights_.reset(); }
        wasReady_ = false;
        return;
    }
    const bool justReady = !wasReady_;
    wasReady_ = true;

    const CampaignSettings s = env_.settings ? env_.settings() : CampaignSettings();
    const int n = env_.players ? env_.players() : 0;
    settings_ = s;
    players_ = n;

    // friendly fire: the campaign's choice, or the combat module's own ini value for a campaign that has none
    int ff = s.chosen ? (s.friendlyFire ? 1 : 0) : -1;
    if (FNV_MUT(1251)) ff = -1;   // MUTANT 1251: the campaign's friendly fire choice never reaches combat
    if (ff != lastFf_ || justReady) {
        lastFf_ = ff;
        if (env_.setFriendlyFire) env_.setFriendlyFire(ff);
    }

    // the numbers
    ramp_.setTarget(profileFor(s, n < 1 ? 1 : n), now);
    if (!seenSettings_ && (s.chosen || justReady)) {
        seenSettings_ = true;
        if (!FNV_MUT(1252)) ramp_.snap();   // MUTANT 1252: the first settings after a load are ramped too
    }
    if (now >= nextApply_) {
        nextApply_ = now + cfg_.applyEveryMs;
        const Profile& stepped = ramp_.step(now);
        applied_ = applier_.apply(stepped);
        applied_ = effects_.apply(stepped) && applied_;
        if (!applied_) nextApply_ = now + 1000;   // the originals could not be read yet: not every frame
    }

    // which mode this campaign uses, on screen
    if (!haveAnnounced_ || announced_ != s) {
        if (s.chosen) {
            say(hudLine(s, n));
            announcedPlayers_ = n;
            lastSizeNotice_ = now;
            haveAnnounced_ = true;
        }
        announced_ = s;
    } else if (s.balanced() && n > 0 && n != announcedPlayers_ && now - lastSizeNotice_ >= cfg_.sizeNoticeMinMs) {
        say("Co-op balanced is now scaled for " + std::to_string(n) + (n == 1 ? " player." : " players."));
        announcedPlayers_ = n;
        lastSizeNotice_ = now;
    }

    // bigger fights: the host only
    if (env_.isHost && env_.isHost() && now >= nextFights_) {
        nextFights_ = now + cfg_.fightsEveryMs;
        const bool on = cfg_.extras && s.balanced();
        fights_.tick(on && env_.fighters ? env_.fighters() : std::vector<Fighter>(), n, on);
    }
}

int32_t Director::trimLoot(const LootSource& src, std::vector<LootStack>& stacks) {
    if (!cfg_.enabled || !settings_.balanced()) return 0;
    const int32_t removed = rebalance::trimLoot(src, stacks, FNV_MUT(1476) ? ramp_.current() : ramp_.target());   // MUTANT 1476: the point of the smooth change, not where it is heading
    ++trimmedSources_;
    if (removed > 0) trimmedItems_ += static_cast<uint64_t>(removed);
    return removed;
}

std::string Director::statusJson() const {
    const Profile& p = ramp_.current();
    char b[1280];
    std::snprintf(b, sizeof b,
        "{\"enabled\":%s,\"chosen\":%s,\"mode\":\"%s\",\"friendlyFire\":%s,\"balanced\":%s,\"players\":%d,\"settled\":%s,"
        "\"profile\":{\"blend\":%.3f,\"damageToPlayers\":%.3f,\"damageByPlayers\":%.3f,\"killXp\":%.3f,\"suppliesKept\":%.3f,\"bodyAmmoKept\":%.3f,\"stimpakHeal\":%.3f},"
        "\"gmstWrites\":%llu,\"gmstReadsFailed\":%llu,\"stimpak\":{\"writes\":%llu,\"readsFailed\":%llu,\"changed\":%s},"
        "\"scarcity\":{\"sources\":%llu,\"itemsRemoved\":%llu},"
        "\"extras\":{\"alive\":%zu,\"spawned\":%llu,\"removed\":%llu,\"counted\":%llu},\"hudLines\":%llu}",
        cfg_.enabled ? "true" : "false", settings_.chosen ? "true" : "false", modeName(settings_.mode), settings_.friendlyFire ? "true" : "false",
        balancedOn() ? "true" : "false", players_, ramp_.settled() ? "true" : "false", p.blend, p.damageToPlayers, p.damageByPlayers, p.killXp, p.suppliesKept,
        p.bodyAmmoKept, p.stimpakHeal,
        static_cast<unsigned long long>(applier_.writes()), static_cast<unsigned long long>(applier_.readsFailed()),
        static_cast<unsigned long long>(effects_.writes()), static_cast<unsigned long long>(effects_.readsFailed()), effects_.changedAnything() ? "true" : "false",
        static_cast<unsigned long long>(trimmedSources_), static_cast<unsigned long long>(trimmedItems_), fights_.extrasKnown(),
        static_cast<unsigned long long>(fights_.spawned()), static_cast<unsigned long long>(fights_.removed()),
        static_cast<unsigned long long>(fights_.counted()), static_cast<unsigned long long>(hudLines_));
    return b;
}

}  // namespace nvcoop::rebalance
