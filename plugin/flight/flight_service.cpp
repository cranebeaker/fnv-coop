#include "flight_service.h"
#include <windows.h>
#include <algorithm>
#include <cstdlib>
#include <ctime>
#include <vector>
#include "common/minijson.h"

namespace nvcoop::flight {
namespace {
LPTOP_LEVEL_EXCEPTION_FILTER g_prev = nullptr;
volatile LONG g_inCrash = 0;

LONG WINAPI crashFilter(EXCEPTION_POINTERS* ep) {
    if (InterlockedCompareExchange(&g_inCrash, 1, 0) == 0) {
        try { service().crashDumpNow(); } catch (...) {}
    }
    return g_prev ? g_prev(ep) : EXCEPTION_CONTINUE_SEARCH;
}

std::string trim(std::string s) {
    size_t a = s.find_first_not_of(" \t\r");
    size_t b = s.find_last_not_of(" \t\r");
    return a == std::string::npos ? std::string() : s.substr(a, b - a + 1);
}
}  // namespace

bool parseServiceConfig(const std::string& text, ServiceConfig* out) {
    ServiceConfig c = *out;
    bool inSection = false;
    size_t pos = 0;
    while (pos <= text.size()) {
        size_t nl = text.find('\n', pos);
        std::string line = trim(text.substr(pos, nl == std::string::npos ? std::string::npos : nl - pos));
        pos = nl == std::string::npos ? text.size() + 1 : nl + 1;
        if (line.empty() || line[0] == ';' || line[0] == '#') continue;
        if (line[0] == '[') { inSection = line == "[flight]"; continue; }
        if (!inSection) continue;
        size_t eq = line.find('=');
        if (eq == std::string::npos) continue;
        std::string k = trim(line.substr(0, eq)), v = trim(line.substr(eq + 1));
        if (k == "enable") c.enable = v != "0";
        else if (k == "dir") c.dir = v;
        else if (k == "label") c.label = v;
        else if (k == "crash") c.crashDump = v != "0";
        else if (k == "keepMb") { char* end = nullptr; long n = std::strtol(v.c_str(), &end, 10); if (end != v.c_str() && n >= 0 && n <= 1000000) c.keepMb = static_cast<uint32_t>(n); }
        else if (k == "ringMb") { int n = std::atoi(v.c_str()); if (n >= 1 && n <= 256) c.ringMb = static_cast<uint32_t>(n); }
    }
    *out = c;
    return true;
}

int pruneDumps(const std::string& dir, uint32_t keepMb) {
    if (dir.empty() || keepMb == 0) return 0;
    struct Dump { uint64_t written, bytes; std::string path; };
    std::vector<Dump> dumps;
    WIN32_FIND_DATAA fd;
    HANDLE h = FindFirstFileA((dir + "\\*.nvfr").c_str(), &fd);
    if (h == INVALID_HANDLE_VALUE) return 0;
    do {
        if (fd.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY) continue;
        dumps.push_back({(uint64_t(fd.ftLastWriteTime.dwHighDateTime) << 32) | fd.ftLastWriteTime.dwLowDateTime,
                         (uint64_t(fd.nFileSizeHigh) << 32) | fd.nFileSizeLow, dir + "\\" + fd.cFileName});
    } while (FindNextFileA(h, &fd));
    FindClose(h);
    std::sort(dumps.begin(), dumps.end(), [](const Dump& a, const Dump& b) { return a.written > b.written; });   // newest first
    const uint64_t budget = uint64_t(keepMb) << 20;
    uint64_t kept = 0;
    int deleted = 0;
    for (const Dump& d : dumps) {
        if (kept + d.bytes <= budget) { kept += d.bytes; continue; }
        if (DeleteFileA(d.path.c_str())) ++deleted;
    }
    return deleted;
}

bool Service::init(const ServiceConfig& cfg, uint8_t slot, uint8_t role) {
    if (rec_) return true;
    cfg_ = cfg;
    if (!cfg_.enable) return false;
    pruneDumps(cfg_.dir, cfg_.keepMb);
    fnvcoop::flight::Config fc;
    fc.ringBytes = static_cast<size_t>(cfg_.ringMb) << 20;
    fc.slot = slot;
    fc.role = role;
    fc.label = cfg_.label;
    fc.dir = cfg_.dir;
    rec_.reset(new fnvcoop::flight::FlightRecorder(fc));
    rec_->setWallStart(static_cast<uint64_t>(std::time(nullptr)) * 1000ull);
    if (cfg_.crashDump) installCrashHandler();
    return true;
}

void Service::installCrashHandler() {
    if (crashInstalled_) return;
    g_prev = SetUnhandledExceptionFilter(&crashFilter);
    crashInstalled_ = true;
}

void Service::removeCrashHandler() {
    if (!crashInstalled_) return;
    SetUnhandledExceptionFilter(g_prev);
    g_prev = nullptr;
    crashInstalled_ = false;
    InterlockedExchange(&g_inCrash, 0);
}

std::string Service::crashDumpNow() { return rec_ ? rec_->dumpCrash() : std::string(); }

std::string Service::statusJson() const {
    nvcoop::json::Value o = nvcoop::json::Value::object();
    o.set("enabled", rec_ != nullptr).set("dir", cfg_.dir);
    if (rec_) {
        auto s = rec_->stats();
        o.set("records", (int64_t)s.records).set("dropped", (int64_t)s.dropped).set("dumps", (int64_t)s.dumps)
            .set("dumpFailures", (int64_t)s.dumpFailures).set("frame", (int64_t)rec_->frame()).set("ringBytesUsed", (int64_t)s.bytesRing);
    }
    return o.dump();
}

bool Service::dumpJson(const std::string& params, std::string& out) {
    if (!rec_) { out = "flight recorder is off ([flight] enable=0)"; return false; }
    std::string path;
    nvcoop::json::Value p;
    std::string err;
    if (!params.empty() && nvcoop::json::parse(params, p, err)) path = p.get("path").asString("");
    if (path.empty()) path = rec_->dumpToDir(fnvcoop::flight::kDumpManual);
    else if (!rec_->dump(path, fnvcoop::flight::kDumpManual)) path.clear();
    if (path.empty()) { out = "dump failed (no dir configured or not writable)"; return false; }
    nvcoop::json::Value o = nvcoop::json::Value::object();
    o.set("path", path);
    out = o.dump();
    return true;
}

void Service::shutdown() {
    if (!rec_) return;
    rec_->dumpToDir(fnvcoop::flight::kDumpShutdown);
    removeCrashHandler();
    rec_.reset();
}

Service& service() {
    static Service s;
    return s;
}

}  // namespace nvcoop::flight
