#include "chat/chat_config.h"

#include <cctype>
#include <cstdlib>

#include "chat/chat_text.h"
#include "fnvcoop_mut.h"

// Twins: 1110 = enabled is read inverted; 1111 = the key name is not parsed (always the default); 1112 = values are not clamped; 1113 = the name is not sanitised
namespace nvcoop::chat {

namespace {
std::string trim(std::string s) {
    while (!s.empty() && (s.back() == ' ' || s.back() == '\t' || s.back() == '\r' || s.back() == '\n')) s.pop_back();
    size_t i = 0;
    while (i < s.size() && (s[i] == ' ' || s[i] == '\t')) ++i;
    return s.substr(i);
}
uint32_t number(const IniGetter& get, const char* key, uint32_t def, uint32_t lo, uint32_t hi, std::string& warn) {
    const std::string v = trim(get("chat", key, std::to_string(def).c_str()));
    char* end = nullptr;
    const long n = std::strtol(v.c_str(), &end, 10);
    if (v.empty() || !end || *end != 0 || n < 0) {
        warn += std::string(warn.empty() ? "" : "; ") + key + "=" + v + " is not a number (kept " + std::to_string(def) + ")";
        return def;
    }
    if (FNV_MUT(1112)) return static_cast<uint32_t>(n);
    if (static_cast<uint32_t>(n) < lo || static_cast<uint32_t>(n) > hi) {
        const uint32_t c = static_cast<uint32_t>(n) < lo ? lo : hi;
        warn += std::string(warn.empty() ? "" : "; ") + key + "=" + v + " is outside " + std::to_string(lo) + ".." + std::to_string(hi) + " (using " + std::to_string(c) + ")";
        return c;
    }
    return static_cast<uint32_t>(n);
}
}  // namespace

ChatSettings readSettings(const IniGetter& get) {
    ChatSettings s;
    Config& c = s.cfg;
    const std::string en = trim(get("chat", "enabled", "1"));
    c.enabled = FNV_MUT(1110) ? (en == "0") : (en != "0");
    const std::string key = trim(get("chat", "key", "Y"));
    if (!key.empty() && !FNV_MUT(1111)) {
        const int vk = parseKeyName(key);
        if (vk) c.openKey = vk;
        else s.warnings += "key=" + key + " is not a key name (kept " + keyName(c.openKey) + ")";
    }
    c.name = FNV_MUT(1113) ? trim(get("chat", "name", "")) : sanitizeName(get("chat", "name", ""));
    std::string d = trim(get("chat", "display", "auto"));
    for (char& ch : d) ch = static_cast<char>(std::tolower(static_cast<unsigned char>(ch)));
    if (d == "auto" || d == "hud" || d == "overlay") c.display = d;
    else s.warnings += std::string(s.warnings.empty() ? "" : "; ") + "display=" + d + " is not auto, hud or overlay (kept auto)";
    c.lineSeconds = number(get, "seconds", c.lineSeconds, 2, 60, s.warnings);
    c.visibleLines = number(get, "lines", c.visibleLines, 1, 12, s.warnings);
    c.historyLines = number(get, "history", c.historyLines, 10, 200, s.warnings);
    c.rateLines = number(get, "rate_lines", c.rateLines, 1, 20, s.warnings);
    c.rateSeconds = number(get, "rate_seconds", c.rateSeconds, 2, 120, s.warnings);
    return s;
}

std::string describe(const Config& c) {
    return std::string("chat ") + (c.enabled ? "on" : "off") + " key=" + keyName(c.openKey) + " display=" + c.display + " seconds=" + std::to_string(c.lineSeconds) +
           " lines=" + std::to_string(c.visibleLines) + " history=" + std::to_string(c.historyLines) + " rate=" + std::to_string(c.rateLines) + "/" +
           std::to_string(c.rateSeconds) + "s" + (c.name.empty() ? "" : " named");
}

}  // namespace nvcoop::chat
