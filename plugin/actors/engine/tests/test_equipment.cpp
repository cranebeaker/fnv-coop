// EquipmentAccess (equipment.h): what an actor reference holds and wears, read and written through console lines.
// Twins 78..84 live in equipment.cpp (see the FNV_MUT markers there).
#include <cstdio>
#include <map>
#include <set>
#include <string>
#include <vector>

#include "actors/engine/equipment.h"
#include "minitest.h"

using namespace nvcoop::actors;
using namespace nvcoop::actors::engine;

namespace {

const fnvcoop::FormRef kRifle("falloutnv.esm", 0x8ED04), kPistol("falloutnv.esm", 0x1234), kArmor("falloutnv.esm", 0x500A), kAmmo("falloutnv.esm", 0x8ED03);
const fnvcoop::FormRef kGone("lostmod.esp", 0x77);

// A tiny world: runtime id = 0x01000000 | local id; what each reference wears and carries; the console lines that were run.
class World : public IEngineHost {
public:
    std::map<uint32_t, uint8_t> types{{0x0108ED04, kFormTypeWeapon}, {0x01001234, kFormTypeWeapon}, {0x0100500A, kFormTypeArmor}, {0x0108ED03, 0x29}};
    std::set<uint32_t> worn;                // runtime ids worn by the one actor
    std::map<uint32_t, int> pack;           // runtime id -> count
    std::vector<std::string> lines;
    bool has3D = true, equipTakes = true, consoleOk = true, readOk = true, packKnown = true;
    std::vector<uint32_t> queued;           // equips the engine accepted but has not applied yet

    static uint32_t rt(const fnvcoop::FormRef& f) { return 0x01000000u | f.localId; }
    bool worldReady() override { return true; }
    uint32_t worldEpoch() override { return 1; }
    RefId playerRef() override { return 0x14; }
    bool read(RefId ref, ActorRaw& out) override { out = ActorRaw{}; out.valid = true; out.ref = ref; out.has3D = has3D; return true; }
    bool actorValue(RefId, uint8_t, float&) override { return false; }
    void loadedActors(std::vector<RefId>&, size_t) override {}
    bool cellLoaded(CellId) override { return true; }
    bool cellInfo(CellId, CellInfo&) override { return false; }
    bool toFormRef(uint32_t id, fnvcoop::FormRef& out) override {
        if ((id >> 24) != 1) return false;   // a created form
        out = fnvcoop::FormRef("falloutnv.esm", id & 0xFFFFFF);
        return true;
    }
    uint32_t fromFormRef(const fnvcoop::FormRef& f) override { return f.plugin == "falloutnv.esm" ? rt(f) : 0; }
    RefId anchorInCell(CellId, live::Vec3&) override { return 0; }
    RefId placeAtMe(RefId, uint32_t) override { return 0; }
    bool console(const std::string& line, RefId on) override {
        lines.push_back(line);
        if (!consoleOk) return false;
        char verb[32] = {};
        unsigned id = 0;
        if (std::sscanf(line.c_str(), "%31s %x", verb, &id) != 2) return false;
        const std::string v = verb;
        if (v == "AddItem") pack[id] += 1;
        else if (v == "EquipItem") { if (pack[id] > 0) { if (equipTakes) worn.insert(id); else queued.push_back(id); } }
        else if (v == "UnequipItem") worn.erase(id);
        return true;
    }
    void land() { for (uint32_t id : queued) worn.insert(id); queued.clear(); }

    EquipmentAccess access() {
        return EquipmentAccess(*this,
            [this](RefId, std::vector<WornEntry>& out) {
                if (!readOk) return false;
                out.clear();
                for (uint32_t id : worn) { auto it = types.find(id); out.push_back({id, it == types.end() ? uint8_t(0) : it->second}); }
                return true;
            },
            [this](RefId, const fnvcoop::FormRef& f) { if (!packKnown) return -1; auto it = pack.find(rt(f)); return it == pack.end() ? 0 : it->second; });
    }
};

constexpr RefId kRef = 0x00ABCDEF;

}  // namespace

TEST(equipment, the_line_texts_are_pinned) {
    CHECK(lines::equipItem(0x0108ED04) == "EquipItem 0108ED04 0 1");
    CHECK(lines::unequipItem(0x0108ED04) == "UnequipItem 0108ED04 0 1");
    CHECK(lines::addItem(0x0108ED04) == "AddItem 0108ED04 1 1");
}

TEST(equipment, read_gives_weapons_armour_and_clothing_only_sorted) {
    World w;
    w.worn = {World::rt(kRifle), World::rt(kAmmo), World::rt(kArmor), 0xFF000123u /* a created form */, 0x01FFFFFFu /* a type we do not know */};
    EquipmentAccess a = w.access();
    EquipList got;
    REQUIRE(a.read(kRef, got));
    CHECK(got.size() == 2 && got[0] < got[1]);
    CHECK(got == (EquipList{kArmor, kRifle}) || got == (EquipList{kRifle, kArmor}));
    w.readOk = false;
    CHECK(!a.read(kRef, got) && got.empty());
    CHECK(a.counters().reads == 2);
}

TEST(equipment, a_wanted_weapon_in_the_pack_is_equipped_with_one_line) {
    World w;
    w.pack[World::rt(kRifle)] = 1;
    EquipmentAccess a = w.access();
    CHECK(a.write(kRef, EquipList{kRifle}));
    CHECK(w.lines.size() == 1 && w.lines[0] == "EquipItem 0108ED04 0 1");
    CHECK(w.worn.count(World::rt(kRifle)) == 1 && w.pack[World::rt(kRifle)] == 1);   // nothing was added: it had one
    CHECK(a.counters().equipLines == 1 && a.counters().addLines == 0 && a.counters().writesDone == 1);
}

TEST(equipment, an_item_the_copy_does_not_own_is_added_first_and_never_twice) {
    World w;
    EquipmentAccess a = w.access();
    CHECK(a.write(kRef, EquipList{kRifle}));
    CHECK(w.lines.size() == 2 && w.lines[0].rfind("AddItem ", 0) == 0 && w.lines[1].rfind("EquipItem ", 0) == 0);
    CHECK(w.pack[World::rt(kRifle)] == 1 && w.worn.count(World::rt(kRifle)) == 1);
    w.lines.clear();
    CHECK(a.write(kRef, EquipList{kRifle}) && w.lines.empty());   // already wearing it: no line at all
    w.worn.clear();
    CHECK(a.write(kRef, EquipList{kRifle}));                      // it has one now: equip only
    CHECK(w.lines.size() == 1 && w.lines[0].rfind("EquipItem ", 0) == 0 && w.pack[World::rt(kRifle)] == 1);
}

TEST(equipment, an_unknown_pack_count_adds_nothing) {
    World w;
    w.packKnown = false;
    EquipmentAccess a = w.access();
    CHECK(!a.write(kRef, EquipList{kRifle}));   // the equip found nothing to wear: reported, not hidden
    CHECK(w.lines.size() == 1 && w.lines[0].rfind("EquipItem ", 0) == 0 && a.counters().addLines == 0);
}

TEST(equipment, what_the_owner_no_longer_holds_is_taken_off) {
    World w;
    w.pack[World::rt(kRifle)] = 1; w.pack[World::rt(kPistol)] = 1; w.pack[World::rt(kArmor)] = 1;
    w.worn = {World::rt(kPistol), World::rt(kArmor)};
    EquipmentAccess a = w.access();
    CHECK(a.write(kRef, EquipList{kArmor, kRifle}));
    CHECK(w.worn == (std::set<uint32_t>{World::rt(kArmor), World::rt(kRifle)}));
    CHECK(w.lines.size() == 2 && w.lines[0].rfind("UnequipItem ", 0) == 0 && w.lines[1].rfind("EquipItem ", 0) == 0);   // the armour it already wears is left alone
    w.lines.clear();
    CHECK(a.write(kRef, EquipList{}));
    CHECK(w.worn.empty() && w.lines.size() == 2);
}

TEST(equipment, an_equip_that_has_not_landed_is_reported_then_found_done) {
    World w;
    w.pack[World::rt(kRifle)] = 1;
    w.equipTakes = false;
    EquipmentAccess a = w.access();
    CHECK(!a.write(kRef, EquipList{kRifle}) && a.counters().writesDone == 0);   // accepted but not worn: the caller retries
    w.land();
    w.lines.clear();
    CHECK(a.write(kRef, EquipList{kRifle}) && w.lines.empty());                  // the retry finds it done and runs nothing
}

TEST(equipment, an_actor_without_3d_gets_no_line) {
    World w;
    w.pack[World::rt(kRifle)] = 1;
    w.has3D = false;
    EquipmentAccess a = w.access();
    CHECK(!a.write(kRef, EquipList{kRifle}) && w.lines.empty());
    w.has3D = true;
    CHECK(a.write(kRef, EquipList{kRifle}));
}

TEST(equipment, an_item_of_a_plugin_that_is_not_loaded_here_is_skipped) {
    World w;
    w.pack[World::rt(kRifle)] = 1;
    EquipmentAccess a = w.access();
    CHECK(a.write(kRef, EquipList{kGone, kRifle}));   // the rest is worn; the missing one cannot fail the call forever
    CHECK(w.worn == (std::set<uint32_t>{World::rt(kRifle)}) && a.counters().unresolved == 1);
}

TEST(equipment, a_refused_console_line_fails_the_write_and_is_counted) {
    World w;
    w.pack[World::rt(kRifle)] = 1;
    w.consoleOk = false;
    EquipmentAccess a = w.access();
    CHECK(!a.write(kRef, EquipList{kRifle}) && a.counters().lineFailures == 1 && w.worn.empty());
    w.readOk = false;
    CHECK(!a.write(kRef, EquipList{kRifle}));         // an unreadable actor is not touched
    CHECK(w.lines.size() == 1);
}
