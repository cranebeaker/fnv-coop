#include "parity/parity_config.h"

#include <algorithm>
#include <cctype>
#include <cstdio>
#include <cstdlib>

#include "fnvcoop_mut.h"

namespace nvcoop::parity {

bool parseBool(const std::string& v) {
    size_t a = 0, b = v.size();
    while (a < b && std::isspace(static_cast<unsigned char>(v[a]))) ++a;
    while (b > a && std::isspace(static_cast<unsigned char>(v[b - 1]))) --b;
    std::string t;
    for (size_t i = a; i < b; ++i) t += static_cast<char>(std::tolower(static_cast<unsigned char>(v[i])));
    return t == "1" || t == "true" || t == "on" || t == "yes";
}

namespace {

struct Reader {
    const IniGetter& get;
    std::string warn;

    double number(const char* key, double def, double lo, double hi) {
        const std::string v = get("parity", key, "");
        if (v.empty()) return def;
        char* end = nullptr;
        const double x = std::strtod(v.c_str(), &end);
        if (end == v.c_str() || (end && *end != '\0' && !std::isspace(static_cast<unsigned char>(*end)))) {
            warn += std::string("parity ") + key + "='" + v + "' is not a number, using " + std::to_string(def) + "\n";
            return def;
        }
        if (FNV_MUT(1814)) return x;   // twin 1814: values are not clamped
        if (x < lo || x > hi) {
            const double c = std::min(hi, std::max(lo, x));
            warn += std::string("parity ") + key + "=" + v + " is outside " + std::to_string(lo) + ".." + std::to_string(hi) + ", using " + std::to_string(c) + "\n";
            return c;
        }
        return x;
    }
};

}  // namespace

Settings readSettings(const IniGetter& get) {
    Settings s;
    Reader r{get, {}};
    const std::string en = get("parity", "enabled", "0");
    s.enabled = !en.empty() && parseBool(en);
    s.intervalMs = static_cast<uint32_t>(r.number("intervalMs", s.intervalMs, 500, 60000));
    s.sendEvery = static_cast<uint32_t>(r.number("sendEvery", s.sendEvery, 0, 30));
    s.radius = static_cast<uint32_t>(r.number("radius", s.radius, 500, 30000));
    s.cap = static_cast<uint32_t>(r.number("cap", s.cap, 4, 60));
    s.netCap = static_cast<uint32_t>(r.number("netCap", s.netCap, 4, 40));
    s.maxFileMb = static_cast<uint32_t>(r.number("maxFileMb", s.maxFileMb, 1, 64));
    s.file = get("parity", "file", s.file.c_str());
    s.tol.posTolUnits = r.number("posTolUnits", s.tol.posTolUnits, 35, 3500);
    s.tol.posHardUnits = r.number("posHardUnits", s.tol.posHardUnits, s.tol.posTolUnits, 30000);
    s.tol.lagSeconds = r.number("lagSeconds", s.tol.lagSeconds, 0, 5);
    s.tol.persistMs = static_cast<uint64_t>(r.number("persistMs", static_cast<double>(s.tol.persistMs), 0, 60000));
    s.tol.logEveryMs = static_cast<uint64_t>(r.number("logEveryMs", static_cast<double>(s.tol.logEveryMs), 1000, 600000));
    s.tol.summaryMs = static_cast<uint64_t>(r.number("summaryMs", static_cast<double>(s.tol.summaryMs), 5000, 600000));
    if (s.netCap > s.cap) s.netCap = s.cap;
    s.warnings = r.warn;
    return s;
}

std::string describe(const Settings& s) {
    char b[400];
    std::snprintf(b, sizeof b, "enabled=%d interval=%ums sendEvery=%u radius=%u cap=%u netCap=%u file=%s maxFileMb=%u posTol=%.0f posHard=%.0f lag=%.2fs persist=%llums logEvery=%llums",
                  s.enabled ? 1 : 0, s.intervalMs, s.sendEvery, s.radius, s.cap, s.netCap, s.file.empty() ? "(none)" : s.file.c_str(), s.maxFileMb, s.tol.posTolUnits,
                  s.tol.posHardUnits, s.tol.lagSeconds, static_cast<unsigned long long>(s.tol.persistMs), static_cast<unsigned long long>(s.tol.logEveryMs));
    return b;
}

}  // namespace nvcoop::parity
