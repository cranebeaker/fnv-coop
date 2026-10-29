// The rebalance director of ONE machine: reads the campaign's setting and the party size, works out the profile, moves the game's numbers to it smoothly,
// switches friendly fire, says which mode the campaign uses, and (host only) runs the bigger fights. Game-free: everything it needs from the game comes in
// through DirectorEnv, so the tests run a host and guests against fakes. docs/rebalance.md.
//
// Who applies what (so nothing is applied twice on two machines):
//   * the game's numbers (damage to / by players, carry weight, skill points, kill XP): every machine sets them to the same absolute values worked out from the
//     same inputs (the campaign setting and the party size, both told by the host). A shot is computed by the engine of the player who fires it, a hit on a
//     player by that player's engine, so each hit is scaled once, by one machine. Setting an absolute value twice is the same as once.
//   * Stimpak strength (the first effect of the Stimpak forms, EffectApplier): the same absolute value on every machine, from the form's original.
//   * supplies and ammo (trimLoot): thinned ONCE per container or corpse, by the machine that registers it with the loot service (plugin/loot), before its
//     contents become the shared ones; every other machine gets the thinned list through the loot sync.
//   * friendly fire: every machine sets the combat module's switch to the campaign's value.
//   * reinforcements: the host only; the extra is a host-made actor and the guests get it through the normal sync.
#pragma once
#include <cstdint>
#include <functional>
#include <string>
#include <vector>

#include "rebalance/balance.h"
#include "rebalance/campaign_settings.h"
#include "rebalance/effect_applier.h"
#include "rebalance/fights.h"
#include "rebalance/gmst_applier.h"
#include "rebalance/ramp.h"
#include "rebalance/scarcity.h"

namespace nvcoop::rebalance {

struct DirectorConfig {
    bool enabled = true;
    bool extras = true;                // reinforcements
    uint32_t rampMs = 6000;
    uint32_t applyEveryMs = 100;
    uint32_t fightsEveryMs = 1000;
    uint32_t sizeNoticeMinMs = 4000;   // between two "now scaled for N players" lines
};

struct DirectorEnv {
    std::function<bool()> worldReady;                    // a game is loaded
    std::function<bool()> isHost;                        // this machine holds the world
    std::function<CampaignSettings()> settings;          // the campaign's (host: its own; guest: what the host said)
    std::function<int()> players;                        // the party size (0: not known yet)
    std::function<uint64_t()> nowMs;
    std::function<void(int ff)> setFriendlyFire;         // 1 on, 0 off, -1 the combat module's own ini value
    std::function<void(const std::string&)> hud;         // one line on screen
    std::function<void(const std::string&)> log;
    std::function<std::vector<Fighter>()> fighters;      // host only
    FightDirector::Env fight;
};

class Director {
public:
    Director(const DirectorConfig& cfg, DirectorEnv env, IGmst& gmst, IEffectPort& effects);
    void tick();                                  // every frame
    // The flag other modules read: this campaign plays Co-op balanced (and the director is on).
    bool balancedOn() const { return cfg_.enabled && settings_.balanced(); }
    const CampaignSettings& settings() const { return settings_; }
    int players() const { return players_; }
    const Profile& profile() const { return ramp_.current(); }
    const Profile& target() const { return ramp_.target(); }
    std::string statusJson() const;
    GmstApplier& applier() { return applier_; }
    EffectApplier& effects() { return effects_; }
    FightDirector& fights() { return fights_; }
    uint64_t hudLines() const { return hudLines_; }
    // The loot hook (plugin/loot calls it for a container or corpse this machine registers, once): thins the stacks for the campaign's TARGET profile. Returns how
    // many items went; 0 (and nothing touched) in Normal, in an unchosen campaign, and while the director is off.
    int32_t trimLoot(const LootSource& src, std::vector<LootStack>& stacks);
    uint64_t trimmedSources() const { return trimmedSources_; }
    uint64_t trimmedItems() const { return trimmedItems_; }
private:
    void say(const std::string& line);
    DirectorConfig cfg_;
    DirectorEnv env_;
    GmstApplier applier_;
    EffectApplier effects_;
    ProfileRamp ramp_;
    FightDirector fights_;
    CampaignSettings settings_;
    int players_ = 0;
    bool seenSettings_ = false;     // the first settings after the game loaded are applied at once (nobody sees a ramp)
    bool wasReady_ = false;
    bool applied_ = true;           // the last apply() got every original
    int lastFf_ = -2;
    CampaignSettings announced_;
    bool haveAnnounced_ = false;
    int announcedPlayers_ = -1;
    uint64_t lastSizeNotice_ = 0, nextApply_ = 0, nextFights_ = 0, hudLines_ = 0, trimmedSources_ = 0, trimmedItems_ = 0;
};

}  // namespace nvcoop::rebalance
