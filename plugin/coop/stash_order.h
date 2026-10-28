// The "stash" order (G35): the host tells a guest's machine to move THAT player's own gear into / out of its copy of a shared container (the Fort's storage, the Dead
// Money equipment container). Game-free; coop_module.cpp sends it from the PlayerStashEvent of nvhooks and runs the console line on the guest.
//
//   player.RemoveAllItems container                       {"op":"stash","into":false,"container":"deadmoney.esm:00efcc","retain":false}
//   container.RemoveAllItems player                       {"op":"stash","into":true, ...}
//   player.RemoveAllTypedItems container own msg type [holdout]   the same with "type" (a form type: 40 = weapons, 24 = armour), "noMsg", "holdout" (W3d item 4: the Dead
//                                                         Money failure ending strips weapons only, the Fort's 131 calls strip by type)
#pragma once
#include <cstdint>
#include <cstdio>
#include <string>

#include "common/minijson.h"
#include "fnvcoop_mut.h"

namespace nvcoop::coop {

struct StashOrder {
    bool into = false;          // the gear comes OUT of the container into the player
    std::string container;      // nvhooks::refToStr form
    bool retain = false;        // "Ownership Added"
    int32_t type = -1;          // < 0: every item (RemoveAllItems)
    bool noMsg = false;
    std::string holdout;        // form list that stays on the player, empty = none
};

inline std::string stashOrderJson(const StashOrder& s) {
    nvcoop::json::Value o = nvcoop::json::Value::object();
    o.set("op", "stash").set("into", s.into).set("container", s.container).set("retain", s.retain);
    if (s.type >= 0 && !FNV_MUT(44)) {   // MUTANT 44: a typed strip travels as a plain RemoveAllItems (a guest loses its armour with its weapons)
        o.set("type", static_cast<int64_t>(s.type)).set("noMsg", s.noMsg);
        if (!s.holdout.empty()) o.set("holdout", s.holdout);
    }
    return o.dump();
}

inline StashOrder stashOrderFrom(const nvcoop::json::Value& v) {
    StashOrder s;
    s.into = v.get("into").asBool(false);
    s.container = v.get("container").asString("");
    s.retain = v.get("retain").asBool(false);
    s.type = static_cast<int32_t>(v.get("type").asInt(-1));
    s.noMsg = v.get("noMsg").asBool(false);
    s.holdout = v.get("holdout").asString("");
    return s;
}

struct StashLine {
    std::string text;
    uint32_t onRef = 0;   // the reference the console line runs on
};

// `cid`: runtime id of the container on this machine; `holdoutId`: runtime id of the holdout form list (0 = none).
inline StashLine stashLine(const StashOrder& s, uint32_t cid, uint32_t holdoutId) {
    const int retain = s.retain ? 1 : 0;
    char b[96];
    StashLine l;
    if (cid == 0 && !s.into && s.type >= 0) {   // `player.RemoveAllTypedItems` with no container (type, holdout kept): the guest's items of that type are destroyed on the guest's machine
        if (FNV_MUT(140)) return l;             // MUTANT 140: the typed strip without a container is never ordered (the guest keeps what the script took from the host's player)
        if (holdoutId) std::snprintf(b, sizeof b, "RemoveAllTypedItems 0 %d %d %d %08X", retain, s.noMsg ? 1 : 0, s.type, holdoutId);
        else std::snprintf(b, sizeof b, "RemoveAllTypedItems 0 %d %d %d", retain, s.noMsg ? 1 : 0, s.type);
        l.text = b;
        l.onRef = 0x14u;
        return l;
    }
    if (cid == 0 && !s.into && s.type < 0) {   // `player.RemoveAllItems` with no container: the guest's items are destroyed, on the guest's machine (the host's player keeps its own)
        l.text = "RemoveAllItems";
        l.onRef = 0x14u;
        return l;
    }
    if (s.type >= 0) {
        const uint32_t other = s.into ? 0x14u : cid;   // the argument: where the gear goes
        if (holdoutId) std::snprintf(b, sizeof b, "RemoveAllTypedItems %08X %d %d %d %08X", other, retain, s.noMsg ? 1 : 0, s.type, holdoutId);
        else std::snprintf(b, sizeof b, "RemoveAllTypedItems %08X %d %d %d", other, retain, s.noMsg ? 1 : 0, s.type);
    } else {
        std::snprintf(b, sizeof b, "RemoveAllItems %08X %d", s.into ? 0x14u : cid, retain);
    }
    l.text = b;
    l.onRef = s.into ? cid : 0x14u;
    return l;
}

}  // namespace nvcoop::coop
