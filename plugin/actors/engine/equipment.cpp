#include "actors/engine/equipment.h"

#include <algorithm>
#include <cstdio>
#include <string>

#include "fnvcoop_mut.h"

namespace nvcoop::actors::engine {

namespace lines {
namespace {
std::string withId(const char* verb, uint32_t formId, const char* tail) {
    char b[48];
    std::snprintf(b, sizeof b, "%s %08X %s", verb, formId, tail);
    return b;
}
}  // namespace
std::string equipItem(uint32_t formId) { return withId("EquipItem", formId, "0 1"); }
std::string unequipItem(uint32_t formId) { return withId("UnequipItem", formId, "0 1"); }
std::string addItem(uint32_t formId) { return withId("AddItem", formId, "1 1"); }
}  // namespace lines

bool EquipmentAccess::read(RefId ref, EquipList& out) {
    ++c_.reads;
    return readWorn(ref, out);
}

bool EquipmentAccess::readWorn(RefId ref, EquipList& out) {
    out.clear();
    std::vector<WornEntry> worn;
    if (!worn_ || !worn_(ref, worn)) return false;
    for (const WornEntry& w : worn) {
        // MUTANT 78: ammo (and anything else the engine marks worn) is published; the copy would be told to equip a stack of bullets
        if (!isEquipmentType(w.formType) && !FNV_MUT(78)) continue;
        fnvcoop::FormRef f;
        if (!h_.toFormRef(w.form, f)) continue;   // a created form has no name every machine agrees on
        out.push_back(f);
    }
    std::sort(out.begin(), out.end());
    out.erase(std::unique(out.begin(), out.end()), out.end());
    return true;
}

bool EquipmentAccess::line(const std::string& text, RefId on) {
    if (h_.console(text, on)) return true;
    ++c_.lineFailures;
    return false;
}

bool EquipmentAccess::write(RefId ref, const EquipList& want) {
    ++c_.writes;
    EquipList cur;
    if (!readWorn(ref, cur)) return false;

    // The wanted items this machine can name; the rest (a plugin that is not loaded here) can never be worn and must not fail the call.
    EquipList local;
    for (const fnvcoop::FormRef& f : want) {
        if (h_.fromFormRef(f)) local.push_back(f);
        else ++c_.unresolved;
    }
    std::sort(local.begin(), local.end());
    local.erase(std::unique(local.begin(), local.end()), local.end());
    const bool same = std::equal(cur.begin(), cur.end(), local.begin(), local.end());
    if (same) { ++c_.writesDone; return true; }

    // An actor without a 3D model refuses the equip lines (the puppet engine of plugin/avatar learned the same); try again when it has one.
    ActorRaw raw;
    if (h_.read(ref, raw) && !raw.has3D && !FNV_MUT(84)) return false;   // MUTANT 84: the lines run on an actor that cannot take them

    bool ok = true;
    if (!FNV_MUT(79)) {   // MUTANT 79: what the owner no longer holds stays in the copy's hands
        for (const fnvcoop::FormRef& f : cur) {
            if (std::binary_search(local.begin(), local.end(), f)) continue;
            ok = line(lines::unequipItem(h_.fromFormRef(f)), ref) && ok;
            ++c_.unequipLines;
        }
    }
    for (const fnvcoop::FormRef& f : local) {
        const bool worn = std::binary_search(cur.begin(), cur.end(), f);
        if (worn && !FNV_MUT(83)) continue;   // MUTANT 83: everything is equipped again, worn or not
        const uint32_t id = h_.fromFormRef(f);
        // The copy may not own the item (the owner's AI took it from a corpse, a script gave it): add one, silently. An unknown count adds nothing.
        const int have = count_ ? count_(ref, f) : -1;
        if ((have == 0 && !FNV_MUT(80)) || (have > 0 && FNV_MUT(81))) {   // MUTANTS 80 (never added) and 81 (always added: the copy ends up with one too many)
            ok = line(lines::addItem(id), ref) && ok;
            ++c_.addLines;
        }
        ok = line(lines::equipItem(id), ref) && ok;
        ++c_.equipLines;
    }
    if (FNV_MUT(82)) return ok;   // MUTANT 82: the result is not checked, a write that did not take is reported as done

    // Did it take? (An equip can be queued behind the actor's current animation: then the next call finds the work done and returns true.)
    EquipList after;
    const bool done = readWorn(ref, after) && after == local;
    if (done) ++c_.writesDone;
    return done;
}

}  // namespace nvcoop::actors::engine
