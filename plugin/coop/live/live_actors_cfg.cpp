#include "live_actors_cfg.h"

#include <cstdlib>

#include "fnvcoop_mut.h"   // FNV_MUT(n): twins 61..64 of the live wiring suite

namespace nvcoop::livewire {

using nvcoop::actors::engine::Config;

namespace {

struct Reader {
    const IniGetter& get;
    std::string* notes;
    void note(const std::string& s) { if (notes) *notes += s + "\n"; }

    bool has(const char* key, std::string& out) {
        out = get("actors", key, "");
        size_t b = out.find_first_not_of(" \t\r\n"), e = out.find_last_not_of(" \t\r\n");
        if (b == std::string::npos) return false;
        out = out.substr(b, e - b + 1);
        return true;
    }
    void u32(const char* key, uint32_t& field, uint32_t lo, uint32_t hi) {
        std::string v;
        if (!has(key, v)) return;
        char* end = nullptr;
        long long n = std::strtoll(v.c_str(), &end, 10);
        if (end == v.c_str() || *end != 0) { note(std::string("[actors] ") + key + "=" + v + " is not a number: default kept"); return; }
        long long c = n;
        if (!FNV_MUT(62)) {   // MUTANT 62: no clamping (scanMs=0 would enumerate every actor every frame)
            if (c < static_cast<long long>(lo)) c = lo;
            if (c > static_cast<long long>(hi)) c = hi;
            if (c != n) note(std::string("[actors] ") + key + "=" + v + " clamped to " + std::to_string(c));
        } else if (c < 0) {
            c = 0;
        }
        field = static_cast<uint32_t>(c);
    }
    void f32(const char* key, float& field, float lo, float hi) {
        std::string v;
        if (!has(key, v)) return;
        char* end = nullptr;
        double n = std::strtod(v.c_str(), &end);
        if (end == v.c_str() || *end != 0) { note(std::string("[actors] ") + key + "=" + v + " is not a number: default kept"); return; }
        double c = n < lo ? lo : n > hi ? hi : n;
        if (c != n) note(std::string("[actors] ") + key + "=" + v + " clamped");
        field = static_cast<float>(c);
    }
    void flag(const char* key, bool& field) {
        std::string v;
        if (!has(key, v)) return;
        field = parseSwitchValue(v);
    }
};

}  // namespace

Config readActorsConfig(const IniGetter& get, Config c, std::string* notes) {
    Reader r{get, notes, };
    r.flag("engine", c.enabled);
    r.u32("scanMs", c.scanMs, 50, 10000);
    r.u32("maxScanActors", c.maxScanActors, 16, 2000);
    r.u32("maxLinesPerFrame", c.maxLinesPerFrame, 1, 200);
    r.u32("driveMinMs", c.driveMinMs, 0, 1000);
    r.f32("posEpsilonXY", c.posEpsilonXY, 0.0f, 100.0f);
    r.f32("posEpsilonZ", c.posEpsilonZ, 0.0f, 100.0f);
    r.f32("yawEpsilonDeg", c.yawEpsilonDeg, 0.0f, 90.0f);
    r.u32("pollDeathMs", c.pollDeathMs, 16, 5000);
    r.u32("deathGraceMs", c.deathGraceMs, 0, 30000);
    if (!FNV_MUT(61)) r.flag("restrain", c.restrainNonOwned);   // the name the port module always used; MUTANT 61: it is ignored
    r.flag("restrainNonOwned", c.restrainNonOwned);
    r.flag("stopCombatOnRestrain", c.stopCombatOnRestrain);
    r.flag("startCombatOnTakeover", c.startCombatOnTakeover);
    if (!FNV_MUT(63)) r.flag("persistLedger", c.persistLedger);   // MUTANT 63: persistLedger is never read (a save/load could leave actors frozen)
    r.flag("standIns", c.standIns);
    r.u32("maxStandIns", c.maxStandIns, 0, 400);
    r.flag("swapLeveled", c.swapLeveled);
    r.u32("saveGraceMs", c.saveGraceMs, 0, 60000);
    if (FNV_MUT(64)) c.enabled = true;   // MUTANT 64: engine=0 does not switch the port off
    return c;
}

}  // namespace nvcoop::livewire
