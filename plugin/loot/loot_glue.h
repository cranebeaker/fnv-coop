// What dll-wiring (plugin/coop/live/live_glue.cpp) needs from plugin/loot, ONE LINE (nothing in coop_module.cpp or main.cpp changes):
//
//     #ifdef NVCOOP_WITH_LOOT
//     hub.add(&nvcoop::loot::service(), nvcoop::loot::enabled());      // the live hub owns the channel (0xB1), joins, leaves, ticks, loads
//     #endif
//
// plugin/loot is otherwise self-contained: its module (loot_module.cpp, registered with NVCOOP_MODULE) reads nvcoop.ini [loot], registers the
// bridge method coop.loot, installs the xNVSE OnDrop / OnActivate handlers and feeds the actor port's death events into the service.
// Everything is OFF unless nvcoop.ini has  [loot] enabled=1  (default 0).
#pragma once
#include <string>
#include "loot/loot_sync.h"

namespace nvcoop::loot {

bool enabled();                       // [loot] enabled=1 and the module initialised
LootSync& service();                  // valid for the life of the process (a service with enabled=false is inert)
bool glueRpc(const std::string& paramsJson, std::string& out);   // coop.loot
// Scarcity (plugin/rebalance, docs/rebalance.md): the hook every container or corpse this machine registers is passed through once, before its contents are shared.
// Kept across the service being rebuilt; an empty function removes it.
void setTrimHook(TrimHook hook);

}  // namespace nvcoop::loot
