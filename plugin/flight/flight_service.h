// Plugin side of the flight recorder (T1a): owns the process-wide FlightRecorder, the crash handler and the
// bridge-facing helpers. Game-free (Windows API only, no NVSE). The coop Runtime records into the recorder
// (Runtime::setFlight); flight_module.cpp wires the ini, the bridge methods and the module lifecycle.
#pragma once
#include <memory>
#include <string>
#include "flight/flight.h"

namespace nvcoop::flight {

struct ServiceConfig {
    bool enable = true;            // [flight] enable=1 (on by default: the ring costs well under 0.1 ms per frame)
    std::string dir;               // [flight] dir= (default <plugin dir>\nvcoop_flight)
    std::string label = "nvcoop";  // file name prefix, e.g. host / guest
    uint32_t ringMb = 16;          // [flight] ringMb=
    bool crashDump = true;         // [flight] crash=1: dump from an unhandled exception, then pass it on
    uint32_t keepMb = 300;         // [flight] keepMb=: at startup the oldest .nvfr files in dir go until the rest fit (0 keeps all)
};

// Parses the text of an ini file's [flight] section. Unknown keys are ignored; bad values keep the default.
bool parseServiceConfig(const std::string& iniText, ServiceConfig* out);
// Deletes the oldest *.nvfr files in `dir` until the rest total at most keepMb (each session writes one of up to ringMb, so without
// this the folder grew without limit: 8.7 GB, 1461 files in a test copy, 2026-11-18). Returns the number deleted.
int pruneDumps(const std::string& dir, uint32_t keepMb);

class Service {
public:
    ~Service() { removeCrashHandler(); }   // the filter must never run against a destroyed recorder
    // Creates the recorder (once). Returns false when disabled.
    bool init(const ServiceConfig& cfg, uint8_t slot, uint8_t role);
    fnvcoop::flight::FlightRecorder* recorder() { return rec_.get(); }
    const ServiceConfig& config() const { return cfg_; }
    void installCrashHandler();      // idempotent; chains to the previous filter
    void removeCrashHandler();
    // JSON for the bridge: {"enabled":..,"records":..,"dropped":..,"dumps":..,"dir":..}
    std::string statusJson() const;
    // flight.dump: writes a dump now, returns {"path":"..."} or an error text with ok=false.
    bool dumpJson(const std::string& paramsJson, std::string& out);
    // Called by the crash filter; public so a test can drive it without crashing.
    std::string crashDumpNow();
    void shutdown();                 // dump (reason shutdown) and drop the recorder

private:
    ServiceConfig cfg_;
    std::unique_ptr<fnvcoop::flight::FlightRecorder> rec_;
    bool crashInstalled_ = false;
};

Service& service();   // process singleton

}  // namespace nvcoop::flight
