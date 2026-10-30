// nvcoop.ini [parity] for the live sync parity checker. Game-free; the module reads real ini and environment through an injected getter.
//
//   [parity]
//   enabled=0          the module does nothing until this is 1 (the release ini ships 1)
//   intervalMs=2000    one sample of the local view this often                  (clamped 500 .. 60000)
//   sendEvery=2        every Nth sample a guest also sends to the host          (0 .. 30; 0 = never send, file only)
//   radius=6000        actors this close to the local player are listed         (500 .. 30000 game units; 70 units are about 1 m)
//   cap=40             at most this many actors per sample                      (4 .. 60)
//   netCap=30          at most this many actors in the datagram                 (4 .. 40)
//   file=nvcoop_parity.jsonl   the sample file, next to nvcoop.ini; "" = no file
//   maxFileMb=8        the file is moved to <file>.1 and restarted at this size (1 .. 64)
//   posTolUnits=210    position difference that is fine (about 3 m)            (35 .. 3500)
//   posHardUnits=1400  position difference that is reported at once             (pos tolerance .. 30000)
//   lagSeconds=0.9     interpolation lag a moving entity is excused for         (0 .. 5)
//   persistMs=3000     a difference must last this long before it is reported   (0 .. 60000)
//   logEveryMs=10000   one DESYNC line per reference and kind this often        (1000 .. 600000)
//   summaryMs=30000    summary line cadence                                      (5000 .. 600000)
// Environment override: NVCOOP_PARITY_<KEY> (upper case) beats the ini.
#pragma once
#include <cstdint>
#include <functional>
#include <string>

namespace nvcoop::parity {

using IniGetter = std::function<std::string(const char* section, const char* key, const char* def)>;

struct Tolerances {
    double posTolUnits = 210;
    double posHardUnits = 1400;
    double lagSeconds = 0.9;
    double skewSeconds = 0.15;       // how far apart in time the two samples are assumed to be when the host samples on arrival (one way latency)
    uint64_t persistMs = 3000;
    uint64_t logEveryMs = 10000;
    uint64_t summaryMs = 30000;
    uint64_t staleMs = 12000;        // a pending difference that was not seen again for this long starts over
    int hpBucketGap = 2;             // health buckets apart that count as different
    int actsGap = 2;                 // attack starts the two machines may disagree by
};

struct Settings {
    bool enabled = false;
    uint32_t intervalMs = 2000;
    uint32_t sendEvery = 2;
    uint32_t radius = 6000;
    uint32_t cap = 40;
    uint32_t netCap = 30;
    std::string file = "nvcoop_parity.jsonl";
    uint32_t maxFileMb = 8;
    Tolerances tol;
    std::string warnings;            // one line per value that was out of range and clamped
};

bool parseBool(const std::string& v);
Settings readSettings(const IniGetter& get);
std::string describe(const Settings& s);

}  // namespace nvcoop::parity
