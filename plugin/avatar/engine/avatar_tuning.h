// The constants that decide how far behind a remote player's puppet appears (round of 2026-10-27, "avatar lag"), in one place so the
// game module, the network wiring and the simulation test all read the same numbers.
//
//   LagTuning         = what the sender publishes (LocalSampler: rate, velocity filter), how often the host / client relays it (StreamRateConfig,
//                       near tier) and how the receiver plays it back (live::InterpConfig, low-latency mode).
//   LegacyLagTuning() = the behaviour before that round: a fixed 100 ms buffer, interpolation only, 20 Hz. The simulation (tests/test_lag_sim.cpp)
//                       measures it next to DefaultLagTuning(), so the improvement is a number that is checked on every run, not a claim.
//   DefaultLagTuning()= what ships.
//   ApplyLagIni()     = nvcoop.ini [avatar] overrides; `get(key, default)` returns the raw text (the game module and live_glue.cpp each pass their own
//                       ini reader). Every key is optional; a bad value keeps the default.
//
//   [avatar] lowLatency=1        0 = the old fixed-delay buffer (the whole round switched off for the receiver)
//            nearHz=30           near-tier rate (sender and relay): 10..60. The far (5 Hz) and out (1 Hz) rates are not touched.
//            minDelayMs=20       floor of the render delay
//            jitterFactor=1.0    delay = minDelayMs + jitterFactor * measured arrival jitter
//            maxDelayMs=120      ceiling of that delay
//            extrapolateTauMs=150 dead reckoning displacement never exceeds speed * this
//            maxExtrapolateMs=250
//            blendMs=70          fade-out time of a prediction miss (0 = pop)
//            slewPerMille=100    the render clock may move this many thousandths of a ms per ms
//            velAlpha=0.8        sender velocity filter (1 = none; the old value was 0.6)
//            legacyRules=0       1 = the teleport / jump / silence rules of 2026-10-28 switched off (for before / after runs)
#pragma once
#include <cstdint>
#include <cstdlib>
#include <functional>
#include <string>

#include "../stream_rate.h"
#include "live/live.h"
#include "local_sampler.h"

namespace nvcoop::avatar::engine {

struct LagTuning {
    ::fnvcoop::live::InterpConfig interp;
    LocalSamplerConfig sampler;
    StreamRateConfig rate;
};

inline LagTuning LegacyLagTuning() {
    LagTuning t;   // every struct default is the legacy value, except that the low-latency mode is off in InterpConfig
    t.interp.lowLatency = false;
    return t;
}

inline LagTuning DefaultLagTuning() {
    LagTuning t;
    t.interp.lowLatency = true;
    t.sampler.periodMs = 33;       // 30 Hz near: the stream is about 20 bytes a snapshot, so the rate costs hundreds of bytes a second
    t.sampler.velAlpha = 0.8f;     // the sender's velocity lags its motion less (the receiver dead-reckons on it)
    t.rate.nearPeriodMs = 33;
    return t;
}

using IniGet = std::function<std::string(const char* key, const char* def)>;

namespace tuning_detail {
inline bool Num(const IniGet& get, const char* key, double lo, double hi, double& out) {
    const std::string s = get(key, "");
    if (s.empty()) return false;
    char* end = nullptr;
    const double v = std::strtod(s.c_str(), &end);
    if (end == s.c_str() || !(v >= lo && v <= hi)) return false;
    out = v;
    return true;
}
}  // namespace tuning_detail

inline void ApplyLagIni(LagTuning& t, const IniGet& get) {
    using tuning_detail::Num;
    double v = 0;
    if (Num(get, "lowLatency", 0, 1, v)) t.interp.lowLatency = v != 0;
    if (Num(get, "nearHz", 10, 60, v)) {
        const uint32_t period = static_cast<uint32_t>(1000.0 / v + 0.5);
        t.sampler.periodMs = period;
        t.rate.nearPeriodMs = period;
    }
    if (Num(get, "minDelayMs", 0, 500, v)) t.interp.minDelayMs = static_cast<uint32_t>(v);
    if (Num(get, "jitterFactor", 0, 10, v)) t.interp.jitterFactor = static_cast<float>(v);
    if (Num(get, "maxDelayMs", 0, 1000, v)) t.interp.maxLowLatencyDelayMs = static_cast<uint32_t>(v);
    if (Num(get, "extrapolateTauMs", 0, 1000, v)) t.interp.extrapolateTauMs = static_cast<uint32_t>(v);
    if (Num(get, "maxExtrapolateMs", 0, 2000, v)) t.interp.maxExtrapolateMs = static_cast<uint32_t>(v);
    if (Num(get, "blendMs", 0, 1000, v)) t.interp.blendMs = static_cast<uint32_t>(v);
    if (Num(get, "slewPerMille", 0, 1000, v)) t.interp.slewPerMille = static_cast<uint32_t>(v);
    if (Num(get, "velAlpha", 0.05, 1, v)) t.sampler.velAlpha = static_cast<float>(v);
    if (Num(get, "legacyRules", 0, 1, v) && v != 0) {   // before / after runs: the teleport, jump and silence rules of 2026-10-28 off
        t.interp.teleportRules = false;
        t.interp.legacyInterval = true;
        t.sampler.speedTeleport = false;
        t.rate.jumpUnits = 0;
    }
    if (t.interp.maxLowLatencyDelayMs < t.interp.minDelayMs) t.interp.maxLowLatencyDelayMs = t.interp.minDelayMs;
}

}  // namespace nvcoop::avatar::engine
