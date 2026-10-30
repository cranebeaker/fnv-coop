// A tiny trace line for the NPC sync code (actors, enemy-anim, area digest). Game-free: the module that owns the real log installs a sink
// (nvcoop::Log), the unit tests install a recording one, and without a sink every call is a no-op. Added 2026-10-30 after a playtest where
// the plugin log said nothing about an NPC that stayed seated on the guest (docs/tools/enemy-anim.md, "What the log shows").
//
// Every line starts with "npc:" so one grep gives the whole story of an actor: `grep "npc:" nvcoop.log | grep 00104BFA`.
// Lines about one actor are rate limited per (kind, actor) by Throttle: a state CHANGE is always worth a line, a repeating condition is not.
#pragma once
#include <cstdarg>
#include <cstdint>
#include <cstdio>
#include <functional>
#include <map>
#include <string>

namespace nvcoop::actors {

using TraceSink = std::function<void(const std::string&)>;

inline TraceSink& traceSinkRef() {
    static TraceSink s;
    return s;
}
inline void setTraceSink(TraceSink f) { traceSinkRef() = std::move(f); }
inline bool traceOn() { return static_cast<bool>(traceSinkRef()); }

// How an actor is named in a line: its NetId in hex and, when the module that owns the engine installed a namer, its reference and base form
// ("00ABCDEF(ref 00104BFA base 00104BF8)"). The lines are about NetIds because that is what the sync layers speak; the namer says which actor that is.
using TraceNamer = std::function<std::string(uint32_t netId)>;
inline TraceNamer& traceNamerRef() {
    static TraceNamer n;
    return n;
}
inline void setTraceNamer(TraceNamer f) { traceNamerRef() = std::move(f); }
inline std::string who(uint32_t netId) {
    char b[16];
    std::snprintf(b, sizeof b, "%08X", netId);
    std::string s = b;
    const TraceNamer& n = traceNamerRef();
    if (n) {
        const std::string x = n(netId);
        if (!x.empty()) s += "(" + x + ")";
    }
    return s;
}

#if defined(__GNUC__) || defined(__clang__)
__attribute__((format(printf, 1, 2)))
#endif
inline void trace(const char* fmt, ...) {
    const TraceSink& sink = traceSinkRef();
    if (!sink) return;
    char buf[512];
    va_list ap;
    va_start(ap, fmt);
    std::vsnprintf(buf, sizeof buf, fmt, ap);
    va_end(ap);
    buf[sizeof buf - 1] = 0;
    sink(std::string("npc: ") + buf);
}

// "At most one per `everyMs` for this key": the key is (kind << 32 | id). Wrap safe; the table is bounded (it forgets the oldest half when it grows).
class Throttle {
public:
    bool allow(uint32_t kind, uint32_t id, uint32_t nowMs, uint32_t everyMs) {
        const uint64_t key = (static_cast<uint64_t>(kind) << 32) | id;
        auto it = last_.find(key);
        if (it != last_.end() && static_cast<uint32_t>(nowMs - it->second) < everyMs) return false;
        if (last_.size() > 2048) {
            for (auto i = last_.begin(); i != last_.end();) {
                if (static_cast<uint32_t>(nowMs - i->second) > everyMs) i = last_.erase(i); else ++i;
            }
        }
        last_[key] = nowMs;
        return true;
    }
    void forget(uint32_t id) {
        for (auto i = last_.begin(); i != last_.end();) {
            if (static_cast<uint32_t>(i->first & 0xFFFFFFFFu) == id) i = last_.erase(i); else ++i;
        }
    }

private:
    std::map<uint64_t, uint32_t> last_;
};

}  // namespace nvcoop::actors
