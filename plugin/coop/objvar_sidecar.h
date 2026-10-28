// The per-slot object variable copies of the scripts the rules run once per player (W3d item 1c) travel with the host's save game (follow-ups 1, item 3).
//
// h_ScriptExec re-runs a PerPlayerState script for every guest, and each guest's object variables (the speaker's bInRadius, the collar timer ...) are copies kept per script
// instance and per slot (nvhooks::ObjVarCopies). They lived in memory only, so loading a host save sent every guest's timers back to the seed. The key of an instance is the
// script and the reference it runs on (not an address), so a copy found in the save of an earlier session belongs to the same speaker after the load.
//
// Text next to the save as `<save>.fnvobjvars` (SaveSidecars kind), one value per line, names never contain a blank:
//   fnvobjvars 1
//   S <instance, 16 hex digits> <variable> <value>              the seed: what every slot starts from
//   C <instance, 16 hex digits> <slot> <variable> <value>       a slot's copy
// A save without the file resets the copies (restore("")), never keeps the previous save's.
#pragma once
#include <cstdio>
#include <cstdlib>
#include <sstream>
#include <string>
#include <vector>

#include "fnvcoop_mut.h"
#include "per_slot_run.h"

namespace nvcoop::coop {

// "" when there is nothing to store.
inline std::string objVarSnapshot(const nvhooks::ObjVarCopies& c) {
    std::vector<nvhooks::ObjVarCopies::Copy> copies = c.exportCopies();
    std::vector<nvhooks::ObjVarCopies::Seed> seeds = c.exportSeeds();
    if (FNV_MUT(45)) copies.clear();   // MUTANT 45: the slots' copies are not written (only the seeds): every guest's timers restart after a load
    if (FNV_MUT(46)) seeds.clear();    // MUTANT 46: the seeds are not written: a slot that has not run yet starts from whatever the host's variable holds after the load
    if (copies.empty() && seeds.empty()) return std::string();
    std::ostringstream o;
    o << "fnvobjvars 1\n";
    char buf[96];
    for (const auto& s : seeds) {
        std::snprintf(buf, sizeof buf, "S %016llx ", static_cast<unsigned long long>(s.instance));
        o << buf << s.name;
        std::snprintf(buf, sizeof buf, " %.17g\n", s.value);
        o << buf;
    }
    for (const auto& k : copies) {
        std::snprintf(buf, sizeof buf, "C %016llx %u ", static_cast<unsigned long long>(k.instance), static_cast<unsigned>(k.slot));
        o << buf << k.name;
        std::snprintf(buf, sizeof buf, " %.17g\n", k.value);
        o << buf;
    }
    return o.str();
}

// Puts back what a save held. "" or an unreadable header: the copies are emptied. A line that does not parse is skipped.
inline void objVarRestore(nvhooks::ObjVarCopies& c, const std::string& text) {
    std::vector<nvhooks::ObjVarCopies::Copy> copies;
    std::vector<nvhooks::ObjVarCopies::Seed> seeds;
    std::istringstream in(text);
    std::string line;
    bool header = false;
    while (std::getline(in, line)) {
        if (!line.empty() && line.back() == '\r') line.pop_back();
        if (!header) { header = line == "fnvobjvars 1"; if (!header) break; continue; }
        std::istringstream ls(line);
        std::string kind, inst, name;
        if (!(ls >> kind >> inst)) continue;
        const unsigned long long instance = std::strtoull(inst.c_str(), nullptr, 16);
        if (kind == "S") {
            double v = 0;
            if (ls >> name >> v) seeds.push_back({instance, name, v});
        } else if (kind == "C") {
            unsigned slot = 0;
            double v = 0;
            if (ls >> slot >> name >> v && slot < 256) copies.push_back({instance, name, static_cast<uint8_t>(FNV_MUT(47) ? 0u : slot), v});   // MUTANT 47: every copy is restored into slot 0
        }
    }
    if (FNV_MUT(48) && copies.empty() && seeds.empty()) return;   // MUTANT 48: a save without the file keeps the copies of the save loaded before it
    c.importAll(copies, seeds);
}

}  // namespace nvcoop::coop
