// What an NPC holds and wears (weapon, armour, clothing), read from and written to a live actor reference. Game-free logic over three small
// callables, so the whole of it is unit tested; the x86 pieces (the raw read of the worn entries of the actor's ExtraContainerChanges, the
// inventory count) live in equipment_real.cpp / engine_module.cpp and are handed in.
//
// Read : the worn entries of the reference -> runtime form ids -> FormRef (the host's resolver) -> sorted EquipList. Ammo and every other
//        form type are left out: the engine marks an ammo stack "worn" too, but it follows the weapon by itself.
// Write: make the reference wear exactly `want`: one `UnequipItem` per worn item that is not wanted, one `EquipItem` per wanted item that is
//        not worn (an `AddItem` first when the reference has none of it: its pack was changed by something the inventory part has not
//        copied yet). The lines run with the actor as the calling reference, as every other write of the actors engine. The result is
//        checked with a second read: false means "not wearing it yet", the caller retries.
#pragma once
#include <cstdint>
#include <functional>
#include <string>
#include <vector>

#include "actors/engine/engine_iface.h"

namespace nvcoop::actors::engine {

// TESForm::typeID of the forms an actor can hold in its hands or wear (xNVSE GameForms.h: ARMO 0x18, CLOT 0x1A, WEAP 0x28; AMMO 0x29 is skipped)
constexpr uint8_t kFormTypeArmor = 0x18, kFormTypeClothing = 0x1A, kFormTypeWeapon = 0x28;
inline bool isEquipmentType(uint8_t t) { return t == kFormTypeArmor || t == kFormTypeClothing || t == kFormTypeWeapon; }

// One worn entry of a reference's container changes: the runtime form id and the form's type id.
struct WornEntry {
    uint32_t form = 0;
    uint8_t formType = 0;
};
using WornReader = std::function<bool(RefId ref, std::vector<WornEntry>& out)>;   // false = cannot read it now
using ItemCounter = std::function<int(RefId ref, const fnvcoop::FormRef& form)>;  // how many the reference holds; < 0 = unknown

namespace lines {
std::string equipItem(uint32_t formId);      // "EquipItem 00112233 0 1"   (do not prevent a later unequip, silent)
std::string unequipItem(uint32_t formId);    // "UnequipItem 00112233 0 1"
std::string addItem(uint32_t formId);        // "AddItem 00112233 1 1"     (one, silent)
}  // namespace lines

struct EquipmentCounters {
    uint64_t reads = 0, writes = 0, writesDone = 0;     // calls / calls that left the reference wearing what was asked
    uint64_t equipLines = 0, unequipLines = 0, addLines = 0, lineFailures = 0;
    uint64_t unresolved = 0;                            // a wanted item whose plugin is not loaded here (skipped)
};

class EquipmentAccess {
public:
    EquipmentAccess(IEngineHost& host, WornReader worn, ItemCounter count) : h_(host), worn_(std::move(worn)), count_(std::move(count)) {}

    bool read(RefId ref, EquipList& out);
    bool write(RefId ref, const EquipList& want);
    const EquipmentCounters& counters() const { return c_; }

private:
    bool readWorn(RefId ref, EquipList& out);   // read() without the counter: write() reads too
    bool line(const std::string& text, RefId on);
    IEngineHost& h_;
    WornReader worn_;
    ItemCounter count_;
    EquipmentCounters c_;
};

}  // namespace nvcoop::actors::engine
