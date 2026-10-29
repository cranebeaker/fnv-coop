// The co-op rebalance setting of a campaign (docs/rebalance.md): the game as it is (Normal) or balanced for a group (Co-op balanced), and friendly fire on or
// off. The host decides when the campaign starts and the campaign keeps it: it is written in the party-save sidecar (plugin/coop/party_save.h Meta) and every
// guest gets it from the host in the campaign order. Header-only and game-free, so plugin/coop can use it without linking this module.
#pragma once
#include <cstdint>
#include <string>

#include "common/minijson.h"
#include "fnvcoop_mut.h"

namespace nvcoop::rebalance {

enum class Mode : uint8_t { Normal = 0, Balanced = 1 };

struct CampaignSettings {
    bool chosen = false;          // false: a campaign from before this setting existed, or none yet: it plays Normal and friendly fire follows nvcoop.ini [combat]
    Mode mode = Mode::Normal;
    bool friendlyFire = false;
    bool balanced() const { return chosen && mode == Mode::Balanced; }
    bool operator==(const CampaignSettings& o) const { return chosen == o.chosen && mode == o.mode && friendlyFire == o.friendlyFire; }
    bool operator!=(const CampaignSettings& o) const { return !(*this == o); }
};

inline const char* modeName(Mode m) { return m == Mode::Balanced ? "balanced" : "normal"; }
inline bool parseMode(const std::string& text, Mode* out) {
    std::string t;
    for (char c : text) t += static_cast<char>(c >= 'A' && c <= 'Z' ? c - 'A' + 'a' : c);
    if (t == "balanced" || t == "coop" || t == "co-op" || t == "coop_balanced") { *out = Mode::Balanced; return true; }
    if (t == "normal" || t == "vanilla") { *out = Mode::Normal; return true; }
    return false;
}

// What the host picks for a new campaign: always a decision (chosen). Unreadable text is Normal.
inline CampaignSettings chooseSettings(const std::string& modeText, bool friendlyFire) {
    CampaignSettings s;
    s.chosen = true;
    s.mode = Mode::Normal;
    parseMode(modeText, &s.mode);
    s.friendlyFire = friendlyFire;
    return s;
}

// ---- the sidecar lines (after the party-save header lines): "rebalance=balanced" and "friendlyFire=1"; nothing when not chosen ----
inline std::string sidecarLines(const CampaignSettings& s) {
    if (!s.chosen) return std::string();
    if (FNV_MUT(1200)) return std::string("rebalance=") + modeName(s.mode) + "\n";   // MUTANT 1200: friendly fire is not kept with the save
    return std::string("rebalance=") + modeName(s.mode) + "\nfriendlyFire=" + (s.friendlyFire ? "1" : "0") + "\n";
}
// One sidecar key. Returns true when the key was one of ours. A bad value leaves the settings as they were.
inline bool applySidecarKey(CampaignSettings& s, const std::string& key, const std::string& value) {
    if (key == "rebalance") {
        Mode m;
        if (parseMode(value, &m)) { s.mode = m; s.chosen = true; }
        return true;
    }
    if (key == "friendlyFire") {
        if (value == "0" || value == "1") s.friendlyFire = value == "1";
        return true;
    }
    return false;
}

// ---- the campaign order fields: "mode":"balanced","ff":true (omitted when not chosen) ----
inline void addJson(nvcoop::json::Value& o, const CampaignSettings& s) {
    if (!s.chosen) return;
    o.set("mode", modeName(s.mode)).set("ff", s.friendlyFire);
}
inline CampaignSettings fromJson(const nvcoop::json::Value& v) {
    CampaignSettings s;
    Mode m;
    if (parseMode(v.get("mode").asString(""), &m)) {
        s.chosen = true;
        s.mode = m;
        s.friendlyFire = v.get("ff").asBool(false) && !FNV_MUT(1201);   // MUTANT 1201: a guest never gets friendly fire from the host
    }
    return s;
}

// ---- the lines a player reads ----
inline std::string describe(const CampaignSettings& s) {
    if (!s.chosen) return "Normal";
    return std::string(s.mode == Mode::Balanced ? "Co-op balanced" : "Normal") + ", friendly fire " + (s.friendlyFire ? "on" : "off");
}
// The HUD line: the campaign's mode, and in Co-op balanced how many players it is scaled for.
inline std::string hudLine(const CampaignSettings& s, int players) {
    std::string t = "This campaign plays " + describe(s);
    if (s.balanced() && players > 0) t += " (" + std::to_string(players) + (players == 1 ? " player)" : " players)");
    return t + ".";
}

}  // namespace nvcoop::rebalance
