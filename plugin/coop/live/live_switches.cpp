#include "live_switches.h"

#include <cctype>

#include "fnvcoop_mut.h"   // FNV_MUT(n): twins 1..4 of the live wiring suite (constant false in the shipped DLL)

namespace nvcoop::livewire {

bool parseSwitchValue(const std::string& raw) {
    size_t b = 0, e = raw.size();
    while (b < e && std::isspace(static_cast<unsigned char>(raw[b]))) ++b;
    while (e > b && std::isspace(static_cast<unsigned char>(raw[e - 1]))) --e;
    std::string v;
    for (size_t i = b; i < e; ++i) v += static_cast<char>(std::tolower(static_cast<unsigned char>(raw[i])));
    if (FNV_MUT(1)) return !v.empty() && v != "0";   // MUTANT 1: any non-empty text is "on" (a typo or "off" would switch the layer on)
    return v == "1" || v == "true" || v == "on" || v == "yes";
}

const char* toString(EntityMode m) { return m == EntityMode::Unified ? "unified" : "legacy"; }

EntityMode parseEntityMode(const std::string& raw, EntityMode def) {
    size_t b = 0, e = raw.size();
    while (b < e && std::isspace(static_cast<unsigned char>(raw[b]))) ++b;
    while (e > b && std::isspace(static_cast<unsigned char>(raw[e - 1]))) --e;
    std::string v;
    for (size_t i = b; i < e; ++i) v += static_cast<char>(std::tolower(static_cast<unsigned char>(raw[i])));
    if (v == "unified") return EntityMode::Unified;
    if (v == "legacy") return EntityMode::Legacy;
    return def;
}

bool LiveSwitches::partsUnified() const {
    if (FNV_MUT(1177)) return entities.parts == EntityMode::Unified;   // MUTANT 1177: unified without actors / avatars (nobody replicates the parts, nobody sends the old messages)
    return entities.parts == EntityMode::Unified && actors && avatars;
}

bool LiveSwitches::streamUnified() const {
    return entities.stream == EntityMode::Unified && actors && avatars;
}

LiveSwitches resolveSwitches(LiveSwitches s) {
    if (s.combat && !s.actors && !FNV_MUT(2)) {   // MUTANT 2: combat without actors is allowed (hits would resolve through nobody)
        s.combat = false;
        s.warnings += "combat=1 ignored: it needs actors=1 (a hit resolves through the victim's owner)\n";
    }
    if (s.spawns && !s.actors && !FNV_MUT(3)) {   // MUTANT 3: spawns without actors is allowed
        s.spawns = false;
        s.warnings += "spawns=1 ignored: it needs actors=1 (a spawned actor is an owned actor)\n";
    }
    if (s.social && !s.actors && !FNV_MUT(65)) {   // MUTANT 65: social without actors is allowed (companions are owned actors; nobody owns them)
        s.social = false;
        s.warnings += "social=1 ignored: it needs actors=1 (companions are owned actors)\n";
    }
    return s;
}

LiveSwitches readLiveSwitches(const IniGetter& get) {
    LiveSwitches s;
    auto on = [&](const char* key) { return parseSwitchValue(get("live", key, "0")); };
    s.actors = on("actors");
    s.avatars = on("avatars");
    s.combat = on("combat");
    s.spawns = on("spawns");
    s.social = FNV_MUT(66) ? on("combat") : on("social");   // MUTANT 66: social follows the combat switch
    if (FNV_MUT(4)) { s.actors = true; s.avatars = true; s.combat = true; s.spawns = true; s.social = true; }   // MUTANT 4: default ON (layer runs unproven)
    s.entities.parts = parseEntityMode(get("entities", "parts", "unified"), EntityMode::Unified);
    s.entities.stream = parseEntityMode(get("entities", "stream", toString(kDefaultStreamMode)), kDefaultStreamMode);
    if (FNV_MUT(1176)) s.entities.parts = EntityMode::Legacy;   // MUTANT 1176: the parts default (and the ini) is legacy
    return resolveSwitches(s);
}

std::string describeSwitches(const LiveSwitches& s) {
    return std::string("actors=") + (s.actors ? "1" : "0") + " avatars=" + (s.avatars ? "1" : "0") + " combat=" + (s.combat ? "1" : "0") +
           " spawns=" + (s.spawns ? "1" : "0") + (s.social ? " social=1" : "");
}

std::string describeEntities(const LiveSwitches& s) {
    return std::string("parts=") + (s.partsUnified() ? "unified" : "legacy") + " stream=" + (s.streamUnified() ? "unified" : "legacy");
}

}  // namespace nvcoop::livewire
