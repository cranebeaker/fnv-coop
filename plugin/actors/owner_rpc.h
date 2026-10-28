// coop.owner {ref:"falloutnv.esm:00ABCD"} -> who owns that actor on THIS machine and at which epoch (FULL-SYNC-PLAN 3.2).
// Game-free; coop_module.cpp calls ownerRpc() with the running ActorSync (or null).
//   {"owner":1,"epoch":3,"known":true,"mine":false,"pending":false,"self":0,"netId":..., "ai":false,"standIn":false}
//   unknown ref  : {"owner":255,"epoch":0,"known":false,"self":0,"tracked":N}      (255 = nobody)
//   no sync      : {"unsupported":true,"reason":"..."}
//   {stats:true} instead of a ref: the actor module's counters (and the host's ownership service's: forcedCommits, immediateCommits).
// Once ownership has settled, every machine answers with the same (owner, epoch).
#pragma once
#include <string>
#include "actors/actor_sync.h"

namespace nvcoop::actors {

bool parseRef(const std::string& text, FormRef& out);      // "plugin.esm:00ABCD" (plugin lower-cased); false if malformed
std::string ownerJson(const OwnerInfo& info);
std::string statsJson(const ActorSync& sync);   // coop.owner {stats:true}: the module's counters (finals repeated, audits, the host's forced / immediate commits ...)
// Returns false (with a message in `out`) only for bad parameters; "not running" is a normal answer.
bool ownerRpc(const ActorSync* sync, const std::string& paramsJson, std::string& out);

}  // namespace nvcoop::actors
