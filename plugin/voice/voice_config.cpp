#include "voice_config.h"

#include <algorithm>
#include <cctype>
#include <cstdio>
#include <cstdlib>
#include <cstring>

#include "fnvcoop_mut.h"
#include "voice_types.h"

namespace nvcoop::voice {

namespace {

std::string lower(std::string s) {
    for (char& c : s) c = static_cast<char>(std::tolower(static_cast<unsigned char>(c)));
    return s;
}
std::string trim(const std::string& s) {
    size_t a = 0, b = s.size();
    while (a < b && std::isspace(static_cast<unsigned char>(s[a]))) ++a;
    while (b > a && std::isspace(static_cast<unsigned char>(s[b - 1]))) --b;
    return s.substr(a, b - a);
}

struct KeyName { const char* name; int vk; };
const KeyName kKeys[] = {
    {"lalt", 0xA4}, {"alt", 0xA4}, {"ralt", 0xA5}, {"lctrl", 0xA2}, {"ctrl", 0xA2}, {"rctrl", 0xA3}, {"lshift", 0xA0}, {"shift", 0xA0},
    {"rshift", 0xA1}, {"tab", 0x09}, {"capslock", 0x14}, {"space", 0x20}, {"insert", 0x2D}, {"delete", 0x2E}, {"home", 0x24},
    {"end", 0x23}, {"pageup", 0x21}, {"pagedown", 0x22}, {"mouse3", 0x04}, {"mouse4", 0x05}, {"mouse5", 0x06}, {"scrolllock", 0x91},
    {"pause", 0x13}, {"numpadmultiply", 0x6A}, {"numpadadd", 0x6B}, {"numpadsubtract", 0x6D}, {"numpaddecimal", 0x6E}, {"numpaddivide", 0x6F},
};

bool parseBool(const std::string& v, bool def, bool* ok) {
    const std::string s = lower(trim(v));
    *ok = true;
    if (s.empty()) return def;
    if (s == "1" || s == "true" || s == "yes" || s == "on") return true;
    if (s == "0" || s == "false" || s == "no" || s == "off") return false;
    *ok = false;
    return def;
}

float parseFloat(const std::string& v, float def, float lo, float hi, bool* ok) {
    const std::string s = trim(v);
    *ok = true;
    if (s.empty()) return def;
    char* end = nullptr;
    const double d = std::strtod(s.c_str(), &end);
    if (end == s.c_str() || *end != 0 || d != d) {
        *ok = false;
        return def;
    }
    return static_cast<float>(std::min<double>(std::max<double>(d, lo), hi));
}

long parseLong(const std::string& v, long def, long lo, long hi, bool* ok) {
    const std::string s = trim(v);
    *ok = true;
    if (s.empty()) return def;
    char* end = nullptr;
    const long d = std::strtol(s.c_str(), &end, 10);
    if (end == s.c_str() || *end != 0) {
        *ok = false;
        return def;
    }
    return std::min(std::max(d, lo), hi);
}

}  // namespace

int parseKey(const std::string& raw) {
    const std::string s = lower(trim(raw));
    if (s.empty()) return 0;
    if (s.size() == 1) {
        const char c = s[0];
        if (c >= 'a' && c <= 'z') return 'A' + (c - 'a');
        if (c >= '0' && c <= '9') return c;
    }
    if (s[0] == 'f' && s.size() <= 3) {   // F1 .. F24
        char* end = nullptr;
        const long n = std::strtol(s.c_str() + 1, &end, 10);
        if (*end == 0 && end != s.c_str() + 1 && n >= 1 && n <= 24) return 0x6F + static_cast<int>(n);
    }
    if (s.compare(0, 3, "num") == 0 && s.size() == 4 && s[3] >= '0' && s[3] <= '9') return 0x60 + (s[3] - '0');
    for (const KeyName& k : kKeys) if (s == k.name) return k.vk;
    char* end = nullptr;
    const int base = (s.size() > 2 && s[0] == '0' && s[1] == 'x') ? 16 : 10;
    const long n = std::strtol(s.c_str(), &end, base);
    if (end != s.c_str() && *end == 0 && n >= 1 && n <= 0xFE) return static_cast<int>(n);
    return 0;
}

std::string keyName(int vk) {
    if (vk >= 'A' && vk <= 'Z') return std::string(1, static_cast<char>(vk));
    if (vk >= '0' && vk <= '9') return std::string(1, static_cast<char>(vk));
    if (vk >= 0x70 && vk <= 0x87) return "F" + std::to_string(vk - 0x6F);
    if (vk >= 0x60 && vk <= 0x69) return "Num" + std::to_string(vk - 0x60);
    static const char* names[][2] = {{"164", "LAlt"}, {"165", "RAlt"}, {"162", "LCtrl"}, {"163", "RCtrl"}, {"160", "LShift"}, {"161", "RShift"},
                                     {"9", "Tab"}, {"20", "CapsLock"}, {"32", "Space"}, {"45", "Insert"}, {"46", "Delete"}, {"36", "Home"},
                                     {"35", "End"}, {"33", "PageUp"}, {"34", "PageDown"}, {"4", "Mouse3"}, {"5", "Mouse4"}, {"6", "Mouse5"}};
    for (auto& n : names) if (std::atoi(n[0]) == vk) return n[1];
    char b[16];
    std::snprintf(b, sizeof b, "0x%02X", vk & 0xFF);
    return b;
}

VoiceSettings readSettings(const IniGetter& get) {
    VoiceSettings s;
    std::string warn;
    auto bad = [&warn](const char* key) { warn += std::string(warn.empty() ? "" : "; ") + "bad value for " + key; };
    auto str = [&get](const char* key, const char* def) { return get ? get("voice", key, def) : std::string(def); };
    bool ok = true;

    s.enabled = parseBool(str("enabled", "1"), true, &ok);
    if (!ok) bad("enabled");

    const std::string mode = lower(trim(str("mode", "ptt")));
    if (mode == "open" || mode == "openmic") s.mode = Mode::OpenMic;
    else if (mode == "ptt" || mode == "pushtotalk" || mode == "push" || mode.empty()) s.mode = Mode::PushToTalk;
    else bad("mode");

    const std::string key = str("key", "");
    if (!trim(key).empty()) {
        const int vk = parseKey(key);
        if (vk) s.key = vk;
        else bad("key");
    }
    s.volume = parseFloat(str("volume", "1"), 1.0f, 0.0f, 2.0f, &ok);
    if (!ok) bad("volume");
    s.micVolume = parseFloat(str("mic_volume", "1"), 1.0f, 0.0f, 4.0f, &ok);
    if (!ok) bad("mic_volume");
    s.range = parseFloat(str("range", "3000"), 3000.0f, 200.0f, 100000.0f, &ok);
    if (!ok) bad("range");
    s.fullRange = parseFloat(str("full_range", "400"), 400.0f, 0.0f, 100000.0f, &ok);
    if (!ok) bad("full_range");
    if (s.fullRange >= s.range && !FNV_MUT(361)) s.fullRange = s.range * 0.25f;   // twin 361: a full-volume radius past the range is kept
    s.bitrate = static_cast<int>(parseLong(str("bitrate", "24000"), 24000, 6000, 64000, &ok));
    if (!ok) bad("bitrate");
    s.openThreshold = parseFloat(str("open_threshold", "600"), 600.0f, 0.0f, 32767.0f, &ok);
    if (!ok) bad("open_threshold");
    s.tailMs = static_cast<uint32_t>(parseLong(str("tail_ms", "240"), 240, 0, 3000, &ok));
    if (!ok) bad("tail_ms");
    s.hangMs = static_cast<uint32_t>(parseLong(str("hang_ms", "400"), 400, 0, 5000, &ok));
    if (!ok) bad("hang_ms");
    s.jitterMs = static_cast<uint32_t>(parseLong(str("jitter_ms", "60"), 60, 20, 400, &ok));
    if (!ok) bad("jitter_ms");
    s.maxDelayMs = static_cast<uint32_t>(parseLong(str("max_delay_ms", "240"), 240, 60, 1000, &ok));
    if (!ok) bad("max_delay_ms");
    if (s.maxDelayMs < s.jitterMs + 40 && !FNV_MUT(362)) s.maxDelayMs = s.jitterMs + 40;   // the cap must leave room for the wait
    s.hud = parseBool(str("hud", "1"), true, &ok);
    if (!ok) bad("hud");
    s.globalFallback = parseBool(str("global_fallback", "1"), true, &ok);
    if (!ok) bad("global_fallback");
    s.micDevice = static_cast<int>(parseLong(str("mic_device", "-1"), -1, -1, 64, &ok));
    if (!ok) bad("mic_device");
    s.speakerDevice = static_cast<int>(parseLong(str("speaker_device", "-1"), -1, -1, 64, &ok));
    if (!ok) bad("speaker_device");

    const std::string muted = str("muted", "");
    size_t pos = 0;
    while (pos < muted.size()) {
        size_t comma = muted.find(',', pos);
        if (comma == std::string::npos) comma = muted.size();
        const std::string item = trim(muted.substr(pos, comma - pos));
        pos = comma + 1;
        if (item.empty()) continue;
        bool k = true;
        const long slot = parseLong(item, -1, -1, 255, &k);
        if (!k || slot < 0 || slot >= kMaxSlots) bad("muted");
        else s.muted.insert(static_cast<uint8_t>(slot));
    }
    s.warnings = warn;
    return s;
}

namespace {
uint32_t framesFor(uint32_t ms) { return (ms + kFrameMs - 1) / kFrameMs; }
}

TxConfig txConfigFrom(const VoiceSettings& s) {
    TxConfig t;
    t.mode = s.mode;
    t.tailFrames = FNV_MUT(363) ? s.tailMs : framesFor(s.tailMs);   // twin 363: milliseconds used as frames
    t.hangFrames = framesFor(s.hangMs);
    t.openThreshold = s.openThreshold;
    t.micGain = s.micVolume;
    return t;
}

JitterConfig jitterConfigFrom(const VoiceSettings& s) {
    JitterConfig j;
    j.startFrames = std::max<uint32_t>(1, framesFor(s.jitterMs));
    j.maxFrames = std::max<uint32_t>(j.startFrames + 1, framesFor(s.maxDelayMs));
    return j;
}

PlacementConfig placementConfigFrom(const VoiceSettings& s) {
    PlacementConfig p;
    p.range = s.range;
    p.fullRange = s.fullRange;
    return p;
}

std::string describe(const VoiceSettings& s) {
    char b[256];
    std::snprintf(b, sizeof b, "enabled=%d mode=%s key=%s volume=%.2f mic=%.2f range=%.0f/%.0f bitrate=%d jitter=%ums cap=%ums hud=%d global=%d muted=%zu",
                  s.enabled ? 1 : 0, s.mode == Mode::OpenMic ? "open" : "ptt", keyName(s.key).c_str(), s.volume, s.micVolume, s.fullRange, s.range,
                  s.bitrate, s.jitterMs, s.maxDelayMs, s.hud ? 1 : 0, s.globalFallback ? 1 : 0, s.muted.size());
    return b;
}

}  // namespace nvcoop::voice
