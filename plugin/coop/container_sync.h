// Container contents for NV:MP. NV:MP does not carry what a player takes from or puts into a container or a corpse: the other players' copies
// kept every item (both could loot the same things). Each machine compares the container its player has open with what it held when the menu
// opened and sends the difference on mux channel kChannel; the host passes a guest's change on to the other guests, and every receiver applies
// it to its own copy of that container (later, when it is loaded there). Game-free: the wire and the diff; coop_module.cpp does the engine part.
// Contents rolled from leveled lists differ per machine (each game rolls its own): the FIRST player to open a container also sends its whole
// contents (full=1, counts instead of changes) and the other copies are set to exactly that, so everybody loots the same items.
// A leveled actor rolls its own creature on each machine too (one game's vicious dog is the other's mole rat): the sender names what the
// container ref is (base), and a copy whose ref is something else keeps its own contents (dogs dropped mole rat meat).
//   [0xD7][ver=3][slot u8][full u8][container: form][base: form, plugin empty = unknown][n u16 LE] n x ([item: form][count or delta i32 LE])   form = [plugin: u8 len][bytes][localId u32 LE]
#pragma once
#include <cstdint>
#include <map>
#include <string>
#include <vector>

#include "fnvcoop/types.h"

namespace nvcoop { namespace coop { namespace containers {

constexpr uint8_t kChannel = 0xD7;   // 0xD5 subtitles, 0xD6 shared XP
constexpr uint8_t kVersion = 3;
constexpr size_t kMaxItems = 512;

using Items = std::map<fnvcoop::FormRef, int32_t>;

struct Change {
    uint8_t slot = 0;                 // who moved the items
    fnvcoop::FormRef container;
    fnvcoop::FormRef base;            // what the container ref is on the sender's machine (empty = not known)
    bool full = false;                // delta holds the container's whole contents (counts > 0), not a change
    Items delta;                      // item -> count added (> 0) or taken (< 0); with full: item -> count
};

// what `now` holds compared with `before` (items whose count did not change are left out)
inline Items diff(const Items& before, const Items& now) {
    Items d;
    for (const auto& kv : now) {
        auto it = before.find(kv.first);
        const int32_t was = it == before.end() ? 0 : it->second;
        if (kv.second != was) d[kv.first] = kv.second - was;
    }
    for (const auto& kv : before)
        if (!now.count(kv.first) && kv.second) d[kv.first] = -kv.second;
    return d;
}

// `items` after `delta` was applied to them (a count never drops below 0)
inline void apply(Items& items, const Items& delta) {
    for (const auto& kv : delta) {
        int32_t& n = items[kv.first];
        n += kv.second;
        if (n <= 0) items.erase(kv.first);
    }
}

inline std::vector<uint8_t> encode(const Change& c) {
    std::vector<uint8_t> b{kChannel, kVersion, c.slot, static_cast<uint8_t>(c.full ? 1 : 0)};
    auto form = [&](const fnvcoop::FormRef& f) {
        const size_t n = f.plugin.size() < 255 ? f.plugin.size() : 255;
        b.push_back(static_cast<uint8_t>(n));
        b.insert(b.end(), f.plugin.begin(), f.plugin.begin() + static_cast<std::ptrdiff_t>(n));
        for (int i = 0; i < 4; ++i) b.push_back(static_cast<uint8_t>(f.localId >> (8 * i)));
    };
    form(c.container);
    form(c.base);
    const size_t n = c.delta.size() < kMaxItems ? c.delta.size() : kMaxItems;
    b.push_back(static_cast<uint8_t>(n & 0xFF));
    b.push_back(static_cast<uint8_t>(n >> 8));
    size_t k = 0;
    for (const auto& kv : c.delta) {
        if (k++ == n) break;
        form(kv.first);
        for (int i = 0; i < 4; ++i) b.push_back(static_cast<uint8_t>(static_cast<uint32_t>(kv.second) >> (8 * i)));
    }
    return b;
}

inline bool decode(const uint8_t* p, size_t n, Change* out) {
    size_t at = 0;
    auto u32 = [&](uint32_t* v) {
        if (n - at < 4) return false;
        *v = uint32_t(p[at]) | uint32_t(p[at + 1]) << 8 | uint32_t(p[at + 2]) << 16 | uint32_t(p[at + 3]) << 24;
        at += 4;
        return true;
    };
    auto form = [&](fnvcoop::FormRef* f) {
        if (at >= n) return false;
        const size_t len = p[at++];
        if (n - at < len) return false;
        f->plugin.assign(reinterpret_cast<const char*>(p + at), len);
        at += len;
        return u32(&f->localId);
    };
    if (n < 4 || p[0] != kChannel || p[1] != kVersion || p[3] > 1) return false;
    Change c;
    c.slot = p[2];
    c.full = p[3] == 1;
    at = 4;
    if (!form(&c.container) || c.container.plugin.empty() || !form(&c.base) || n - at < 2) return false;
    const size_t count = size_t(p[at]) | size_t(p[at + 1]) << 8;
    at += 2;
    if ((count == 0 && !c.full) || count > kMaxItems) return false;   // a full snapshot may be empty: the container holds nothing
    for (size_t i = 0; i < count; ++i) {
        fnvcoop::FormRef item;
        uint32_t d = 0;
        if (!form(&item) || item.plugin.empty() || !u32(&d) || static_cast<int32_t>(d) == 0 || (c.full && static_cast<int32_t>(d) < 0)) return false;
        c.delta[item] = static_cast<int32_t>(d);
    }
    if (at != n) return false;
    *out = std::move(c);
    return true;
}

}}}  // namespace nvcoop::coop::containers
