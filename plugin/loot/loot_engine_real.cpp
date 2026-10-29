// The real ILootEngine (x86, compiled into nvcoop.dll next to the module). NOTHING here has run in a game.
//
// Engine facts used (all from docs/re/dialogmenu-menus-vats-inventory.md, plugin/bots/bots_addrs.h and third_party/NVSE headers; prologues are
// pinned and compared at init, a mismatch disables the read / call that needs it and the reason shows in coop.loot):
//   0x4BF220  cdecl(ref) -> ExtraContainerChanges::Data*           creates the data when missing (mutates, accepted: the bots module does the same)
//   0x4C8F30  thiscall(Data*, TESForm*) -> TOTAL count, ret 4      base container + changes, equipped items included
//   0x4BFFB0  thiscall(Data*) -> TESContainer* (the base form's)   TESContainer::formCountList is a tList at +4: node {data, next}, FormCount {count, form}
//   0x714F00  thiscall(InterfaceManager*) -> top menu id           InterfaceManager* at 0x11D8A80, crosshairRef at +0xFC (xNVSE GameUI.h)
//   menu id 0x3F0 = container menu (xNVSE kMenuType_Container; the RE doc table has 0x3F0 / 0x3F1 swapped, see bot_engine.h).
// Writes are console lines run with the container (or the player, ref 0x14) as the calling reference:  AddItem <id> <n> 1  /  RemoveItem <id> <n> 1.
// TODO-RE (each has a working degradation, none is guessed):
//   * closing the container menu from code: no verified call. closeMenu() uses a hook the glue can set; without it the menu stays open and the
//     player's further moves are refused by the host (NotHolder) and rolled back. The host's Busy answer still blocks the transfer.
//   * which ref the container menu shows: taken from the crosshair ref at the moment the menu becomes the top menu.
//   * leveled entries in a base container list (type 0x34) are not resolved by the totals call: such a read reports exact=false.
//   * the Havok pose of a physics object: positions are the ref's posX..posZ / rotX..rotZ; a physics-driven move only shows there after the
//     engine synchronises the ref with its body. SetPos moves the body with the ref (the same assumption the actor port makes).
//   * per-instance item data (condition, mods) is not carried: `meta` stays 0.
#include <Windows.h>

#include <cstdio>
#include <cstring>
#include <map>
#include <set>
#include <string>
#include <vector>

#include "GameForms.h"
#include "GameObjects.h"
#include "actors/actor_types.h"
#include "actors/engine/actor_engine_api.h"
#include "common/nvcoop_api.h"
#include "engine_hooks.h"
#include "loot/loot_engine_real.h"

namespace nvcoop::loot {

namespace {

constexpr uint32_t kGetContainerData = 0x4BF220, kItemTotal = 0x4C8F30, kBaseContainer = 0x4BFFB0, kTopMenuId = 0x714F00;
constexpr uint32_t kInterfaceMgrPtr = 0x11D8A80;
constexpr uint32_t kFormDeleted = 0x20, kFormDisabled = 0x800;   // TESForm::flags (+8): a picked-up world item shows kFormDeleted (ref.info 0x408 -> 0x4032A)
constexpr uint32_t kMenuContainer = 0x3F0;
const uint8_t kPinGetContainerData[16] = {0x55, 0x8b, 0xec, 0x6a, 0xff, 0x68, 0x13, 0x12, 0xef, 0x00, 0x64, 0xa1, 0x00, 0x00, 0x00, 0x00};
const uint8_t kPinItemTotal[16] = {0x55, 0x8b, 0xec, 0x83, 0xec, 0x14, 0x89, 0x4d, 0xf0, 0xc7, 0x45, 0xf4, 0x00, 0x00, 0x00, 0x00};
const uint8_t kPinBaseContainer[16] = {0x55, 0x8b, 0xec, 0x83, 0xec, 0x08, 0x89, 0x4d, 0xfc, 0x8b, 0x45, 0xfc, 0x83, 0x78, 0x04, 0x00};
const uint8_t kPinTopMenuId[16] = {0x55, 0x8b, 0xec, 0x83, 0xec, 0x08, 0x89, 0x4d, 0xf8, 0xc7, 0x45, 0xfc, 0x00, 0x00, 0x00, 0x00};

// one tiny function per raw access so __try never meets a C++ object
bool Rd32(uint32_t addr, uint32_t* out) {
    __try {
        *out = *reinterpret_cast<const uint32_t*>(addr);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool Rd8(uint32_t addr, uint8_t* out) {
    __try {
        *out = *reinterpret_cast<const uint8_t*>(addr);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool RdF(uint32_t addr, float* out) {
    __try {
        *out = *reinterpret_cast<const float*>(addr);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool Rd16(uint32_t addr, int16_t* out) {
    __try {
        *out = *reinterpret_cast<const int16_t*>(addr);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
// one tiny function so __try never meets a C++ object: the stack size of a world item (ExtraCount, extra 0x24, count at +0x0C)
int StackCountOf(uint32_t obj) {
    __try {
        auto* r = reinterpret_cast<TESObjectREFR*>(obj);
        BSExtraData* x = r->extraDataList.GetByType(kExtraData_Count);
        int16_t c = 0;
        if (x && Rd16(reinterpret_cast<uint32_t>(x) + 0x0C, &c) && c > 0) return c;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
    }
    return 0;
}
bool PinOk(uint32_t addr, const uint8_t* want) {
    uint8_t got[16];
    __try {
        std::memcpy(got, reinterpret_cast<const void*>(addr), 16);
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
    return std::memcmp(got, want, 16) == 0;
}
bool CallGetContainerData(uint32_t ref, uint32_t* out) {
    __try {
        *out = reinterpret_cast<uint32_t(__cdecl*)(uint32_t)>(kGetContainerData)(ref);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool CallThis1(uint32_t fn, uint32_t self, uint32_t* out) {
    __try {
        *out = reinterpret_cast<uint32_t(__thiscall*)(uint32_t)>(fn)(self);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool CallThis2(uint32_t fn, uint32_t self, uint32_t arg, int32_t* out) {
    __try {
        *out = reinterpret_cast<int32_t(__thiscall*)(uint32_t, uint32_t)>(fn)(self, arg);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

constexpr uint8_t kTypeCell = 0x39;   // TESObjectCELL (xNVSE kFormType_TESObjectCELL)
constexpr uint8_t kTypeCont = 0x1B, kTypeLevItem = 0x34, kTypeListForm = 0x55, kTypeRefr = 0x3A, kTypeNpcRef = 0x3B, kTypeCreatureRef = 0x3C;
constexpr uint8_t kTypeAmmo = 0x29, kTypeAlch = 0x2F, kTypeNpc = 0x2A;
constexpr uint32_t kAlchFlagsOffset = 0xBC;                          // AlchemyItem::alchFlags (xNVSE GameForms.h), the ENIT flags byte
constexpr uint8_t kAlchFood = 0x02, kAlchMedicine = 0x04;            // GECK wiki GetIngestibleFlag: 2 food (drink included), 4 medicine
constexpr uint32_t kAcbsUniqueNpc = 0x20;                            // TESActorBaseData flags, plugin/actors/engine/engine_real.cpp

// A unique (named) NPC base form; creatures and levelled characters are generic. Plain data only inside the __try.
bool IsUniqueNpcBase(uint32_t base) {
    __try {
        TESForm* f = reinterpret_cast<TESForm*>(base);
        if (f->typeID != kTypeNpc) return false;
        return (static_cast<TESActorBase*>(f)->baseData.flags & kAcbsUniqueNpc) != 0;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool IsInventoryType(uint8_t t) {
    switch (t) {
    case 0x18: case 0x19: case 0x1A: case 0x1F: case 0x28: case 0x29: case 0x2E: case 0x2F: case 0x31: case 0x32: case 0x1D: case 0x1E:   // ARMO BOOK CLOT MISC WEAP AMMO KEYM ALCH NOTE CHIP INGR LIGH
        return true;
    default: return false;
    }
}

}  // namespace

RealLootEngine::RealLootEngine() {
    pinsOk_ = PinOk(kGetContainerData, kPinGetContainerData) && PinOk(kItemTotal, kPinItemTotal) && PinOk(kBaseContainer, kPinBaseContainer);
    menuPinOk_ = PinOk(kTopMenuId, kPinTopMenuId);
    if (!pinsOk_) nvcoop::Log("loot: inventory pins do not match this exe: container reads report exact=false and refuse");
    if (!menuPinOk_) nvcoop::Log("loot: top-menu pin does not match this exe: the container menu is not detected");
}

bool RealLootEngine::worldReady() {
    uint32_t player = 0;
    return Rd32(0x11DEA3C, &player) && player != 0;
}

uint32_t RealLootEngine::worldEpoch() { return epoch_; }

uint32_t RealLootEngine::formAddr(RefId id) { return reinterpret_cast<uint32_t>(nvhooks::resolver().lookup(id)); }

bool RealLootEngine::netOfRef(RefId ref, uint32_t obj, NetId* net) {
    FormRef fr;
    if (nvhooks::resolver().toRef(ref, &fr) && !fr.plugin.empty()) {
        *net = nvcoop::actors::deriveNetId(fr);
        return *net != 0;
    }
    *net = nvcoop::actors::engine::netIdOf(ref);   // a created actor the actor port tracks
    (void)obj;
    return *net != 0;
}

bool RealLootEngine::describeContainer(RefId ref, ContainerDesc& out) {
    out = ContainerDesc();
    const uint32_t obj = formAddr(ref);
    if (!obj) return false;
    uint8_t type = 0;
    if (!Rd8(obj + 4, &type)) return false;
    bool corpse = false;
    if (type == kTypeRefr) {
        uint32_t base = 0;
        uint8_t bt = 0;
        if (!Rd32(obj + 0x20, &base) || !base || !Rd8(base + 4, &bt) || bt != kTypeCont) return false;
    } else if (type == kTypeNpcRef || type == kTypeCreatureRef) {
        uint32_t life = 0;
        if (!Rd32(obj + 0x108, &life) || (life != 1 && life != 2)) return false;   // Actor+0x108: 1 dying, 2 dead (docs/tools/actor-port.md)
        corpse = true;
        uint32_t base = 0;
        out.uniqueActor = Rd32(obj + 0x20, &base) && base && IsUniqueNpcBase(base);
    } else {
        return false;
    }
    NetId net = 0;
    if (!netOfRef(ref, obj, &net)) return false;
    uint32_t cell = 0, cellId = 0;
    if (Rd32(obj + 0x40, &cell) && cell) Rd32(cell + 0x0C, &cellId);
    out.net = net;
    out.kind = corpse ? lc::ContainerKind::Corpse : lc::ContainerKind::Container;
    out.cell = cellId;
    RdF(obj + 0x30, &out.pos.x); RdF(obj + 0x34, &out.pos.y); RdF(obj + 0x38, &out.pos.z);
    refOfNet_[net] = ref;
    return true;
}

RefId RealLootEngine::refOfContainer(NetId net) {
    auto it = refOfNet_.find(net);
    if (it != refOfNet_.end()) {
        if (formAddr(it->second)) return it->second;
        refOfNet_.erase(it);
    }
    const RefId r = nvcoop::actors::engine::refOf(net);   // a tracked actor (corpse)
    if (r && formAddr(r)) { refOfNet_[net] = r; return r; }
    return 0;
}

bool RealLootEngine::readContents(NetId net, Items& out, bool& exact) {
    out.clear();
    exact = true;
    if (!pinsOk_) { exact = false; return false; }
    const RefId ref = refOfContainer(net);
    if (!ref) return false;
    uint32_t data = 0;
    if (!CallGetContainerData(ref, &data) || !data) return false;
    std::set<uint32_t> forms;
    // changes: ExtraContainerChanges::Data.objList (a tList<EntryData>*), EntryData { extendData, countDelta, type }
    uint32_t objList = 0;
    if (Rd32(data, &objList) && objList) {
        uint32_t node = objList;   // head node is embedded: data@0, next@4
        for (int guard = 0; node && guard < 4096; ++guard) {
            uint32_t entry = 0, type = 0, next = 0;
            if (!Rd32(node, &entry) || !Rd32(node + 4, &next)) break;
            if (entry && Rd32(entry + 8, &type) && type) forms.insert(type);
            node = next;
        }
    }
    // base container: formCountList at TESContainer+4
    uint32_t cont = 0;
    if (CallThis1(kBaseContainer, data, &cont) && cont) {
        uint32_t node = cont + 4;
        for (int guard = 0; node && guard < 4096; ++guard) {
            uint32_t fc = 0, next = 0, form = 0;
            if (!Rd32(node, &fc) || !Rd32(node + 4, &next)) break;
            if (fc && Rd32(fc + 4, &form) && form) forms.insert(form);
            node = next;
        }
    }
    for (uint32_t form : forms) {
        uint8_t t = 0;
        if (!Rd8(form + 4, &t)) { exact = false; continue; }
        if (t == kTypeLevItem || t == kTypeListForm) { exact = false; continue; }   // an unresolved list: not a real item
        if (!IsInventoryType(t)) continue;
        int32_t total = 0;
        if (!CallThis2(kItemTotal, data, form, &total)) { exact = false; continue; }
        if (total <= 0) continue;
        uint32_t id = 0;
        FormRef fr;
        if (!Rd32(form + 0x0C, &id) || !nvhooks::resolver().toRef(id, &fr)) { exact = false; continue; }   // a created form: no identity on the wire
        out[fr] += total;
    }
    return true;
}

// Ammo (AMMO), and the supplies a scarcity rule thins: an ALCH that is flagged food (drink included) or medicine. Chems and the rest of the ALCH forms are not.
ItemClass RealLootEngine::classifyItem(const FormRef& base) {
    const uint32_t id = nvhooks::resolver().fromRef(base);
    const uint32_t form = id ? formAddr(id) : 0;
    uint8_t t = 0;
    if (!form || !Rd8(form + 4, &t)) return ItemClass::Other;
    if (t == kTypeAmmo) return ItemClass::Ammo;
    if (t != kTypeAlch) return ItemClass::Other;
    uint8_t flags = 0;
    return Rd8(form + kAlchFlagsOffset, &flags) && (flags & (kAlchFood | kAlchMedicine)) ? ItemClass::Supply : ItemClass::Other;
}

bool RealLootEngine::runOn(RefId ref, const std::string& line) {
    nvhooks::ApplyScope allow;   // a guest blocks script-driven inventory changes; ours are not scripts
    return nvhooks::execConsoleLineOn(line, ref);
}

bool RealLootEngine::setContents(NetId net, const Items& want) {
    Items have;
    bool exact = true;
    if (!readContents(net, have, exact)) return false;
    const RefId ref = refOfContainer(net);
    if (!ref) return false;
    bool ok = true;
    for (const auto& d : lc::diffItems(have, want)) {
        const uint32_t id = nvhooks::resolver().fromRef(d.base);
        if (!id) { ok = false; continue; }
        char line[96];
        if (d.b > d.a) std::snprintf(line, sizeof line, "AddItem %08X %d 1", id, d.b - d.a);
        else std::snprintf(line, sizeof line, "RemoveItem %08X %d 1", id, d.a - d.b);
        if (!runOn(ref, line)) ok = false;
    }
    return ok;
}

RefId RealLootEngine::openMenuRef() {
    uint32_t mgr = 0;
    if (!menuPinOk_ || !Rd32(kInterfaceMgrPtr, &mgr) || !mgr) { menuRef_ = 0; return 0; }
    uint32_t top = 0;
    if (!CallThis1(kTopMenuId, mgr, &top) || top != kMenuContainer) { menuRef_ = 0; return 0; }
    if (menuRef_) return menuRef_;   // still the same menu
    uint32_t cross = 0;
    if (Rd32(mgr + 0xFC, &cross) && cross) {
        uint32_t id = 0;
        if (Rd32(cross + 0x0C, &id)) menuRef_ = id;
    }
    return menuRef_;
}

bool RealLootEngine::closeMenu() {
    if (closeHook_) return closeHook_();
    nvcoop::Log("loot: close of the container menu requested; TODO-RE: no verified call (docs/tools/loot.md), the menu stays open");
    return false;
}

bool RealLootEngine::adjustPlayer(const FormRef& base, int32_t delta) {
    const uint32_t id = nvhooks::resolver().fromRef(base);
    if (!id || delta == 0) return false;
    char line[96];
    if (delta > 0) std::snprintf(line, sizeof line, "AddItem %08X %d 1", id, delta);
    else std::snprintf(line, sizeof line, "RemoveItem %08X %d 1", id, -delta);
    return runOn(kPlayerRef, line);
}

bool RealLootEngine::playerCount(const FormRef& base, int32_t& out) {
    out = 0;
    if (!pinsOk_) return false;
    const uint32_t id = nvhooks::resolver().fromRef(base);
    const uint32_t form = id ? formAddr(id) : 0;   // the resolver answers any form, not only references
    uint32_t data = 0;
    int32_t total = 0;
    if (!form || !CallGetContainerData(kPlayerRef, &data) || !data || !CallThis2(kItemTotal, data, form, &total)) return false;   // same totals call readContents() uses
    out = total < 0 ? 0 : total;
    return true;
}

bool RealLootEngine::testGrab(RefId container, uint32_t itemId, int32_t count, bool remove, bool add, std::string* why) {
    auto fail = [&](const char* m) { if (why) *why = m; return false; };
    if (count <= 0 || !itemId) return fail("bad item or count");
    char line[96];
    if (remove) {
        if (!container || !formAddr(container)) return fail("the container ref is not loaded");
        std::snprintf(line, sizeof line, "RemoveItem %08X %d 1", itemId, count);
        if (!runOn(container, line)) return fail("RemoveItem on the container was refused");
    }
    if (add) {
        std::snprintf(line, sizeof line, "AddItem %08X %d 1", itemId, count);
        if (!runOn(kPlayerRef, line)) return fail("AddItem on the player was refused");
    }
    return true;
}

void RealLootEngine::notify(lc::Notice n, NetId id) {
    char b[96];
    std::snprintf(b, sizeof b, "{\"notice\":\"%s\",\"id\":%u}", lc::toString(n), id);
    nvcoop::PushEvent("loot.notice", b);
    nvcoop::Log("loot: notice %s (%u)", lc::toString(n), id);   // TODO-RE: an on-screen message (no verified HUD call yet)
}

void RealLootEngine::loadedContainers(std::vector<NetId>& out, size_t max) {
    out.clear();
    for (auto it = refOfNet_.begin(); it != refOfNet_.end() && out.size() < max;) {
        if (formAddr(it->second)) { out.push_back(it->first); ++it; }
        else it = refOfNet_.erase(it);
    }
}

// The object list of a cell (TESObjectCELL + 0xAC: a tList, the head node is {data, next}): every REFR whose base form is a CONT.
static void containersOfCell(uint32_t cell, std::vector<RefId>& out, size_t max) {
    uint32_t node = cell + 0xAC;
    for (int guard = 0; node && guard < 30000 && out.size() < max; ++guard) {
        uint32_t data = 0, next = 0;
        if (!Rd32(node, &data) || !Rd32(node + 4, &next)) break;
        node = next;
        uint8_t type = 0, baseType = 0;
        uint32_t base = 0, id = 0;
        if (!data || !Rd8(data + 4, &type) || type != kTypeRefr) continue;
        if (!Rd32(data + 0x20, &base) || !base || !Rd8(base + 4, &baseType) || baseType != kTypeCont) continue;
        if (Rd32(data + 0x0C, &id) && id) out.push_back(id);
    }
}

// The player's parent cell (player + 0x40).
void RealLootEngine::nearbyContainers(std::vector<RefId>& out, size_t max) {
    out.clear();
    uint32_t player = 0, cell = 0;
    if (!Rd32(0x11DEA3C, &player) || !player || !Rd32(player + 0x40, &cell) || !cell) return;
    containersOfCell(cell, out, max);
}

// The cells of the player's worldspace the engine holds attached (TESWorldSpace + 0x30 cellMap, an NiTPointerMap: numBuckets @ +4, buckets @ +8, entries { next, key, data };
// TESObjectCELL: cellState @ +0x26 (6 = attached), worldSpace @ +0xC0, refID @ +0x0C), the same walk plugin/avatar's LoadedCells does. An interior (no worldspace) has none.
void RealLootEngine::nearbyCells(std::vector<lc::CellId>& out, size_t max) {
    out.clear();
    uint32_t player = 0, pcell = 0, ws = 0, map = 0, pid = 0;
    if (!Rd32(0x11DEA3C, &player) || !player || !Rd32(player + 0x40, &pcell) || !pcell || !Rd32(pcell + 0x0C, &pid) || !pid) return;
    out.push_back(pid);
    if (!Rd32(pcell + 0xC0, &ws) || !ws || !Rd32(ws + 0x30, &map) || !map) return;
    uint32_t nb = 0, buckets = 0;
    if (!Rd32(map + 4, &nb) || !Rd32(map + 8, &buckets) || !buckets || nb == 0 || nb > 4096) return;
    int seen = 0;
    for (uint32_t b = 0; b < nb && seen < 4000 && out.size() < max; ++b) {
        uint32_t e = 0;
        if (!Rd32(buckets + 4u * b, &e)) continue;
        for (int g = 0; e && g < 64 && out.size() < max; ++g, ++seen) {
            uint32_t next = 0, data = 0, id = 0;
            uint8_t type = 0, state = 0;
            if (!Rd32(e, &next) || !Rd32(e + 8, &data)) break;
            e = next;
            if (!data || data == pcell || !Rd8(data + 4, &type) || type != kTypeCell || !Rd8(data + 0x26, &state) || state != 6) continue;   // attached cells only
            if (Rd32(data + 0x0C, &id) && id) out.push_back(id);
        }
    }
}

void RealLootEngine::containersInCell(lc::CellId cellId, std::vector<RefId>& out, size_t max) {
    out.clear();
    const uint32_t cell = cellId ? formAddr(cellId) : 0;   // the resolver answers any form, a cell too
    uint8_t type = 0;
    if (!cell || !Rd8(cell + 4, &type) || type != kTypeCell) return;
    containersOfCell(cell, out, max);
}

bool RealLootEngine::describeItem(RefId ref, ItemDesc& out) {
    out = ItemDesc();
    const uint32_t obj = formAddr(ref);
    uint8_t type = 0;
    if (!obj || !Rd8(obj + 4, &type) || type != kTypeRefr) return false;
    uint32_t base = 0, baseId = 0;
    uint8_t bt = 0;
    if (!Rd32(obj + 0x20, &base) || !base || !Rd8(base + 4, &bt) || !IsInventoryType(bt) || !Rd32(base + 0x0C, &baseId)) return false;
    FormRef bfr;
    if (!nvhooks::resolver().toRef(baseId, &bfr)) return false;
    out.item.base = bfr;
    out.item.count = 1;
    // ExtraCount (extra 0x24): the stack size of a world item
    const int stack = StackCountOf(obj);
    if (stack > 0) out.item.count = stack;
    uint32_t cell = 0, cellId = 0;
    if (Rd32(obj + 0x40, &cell) && cell) Rd32(cell + 0x0C, &cellId);
    out.item.cell = cellId;
    RdF(obj + 0x30, &out.item.pos.x); RdF(obj + 0x34, &out.item.pos.y); RdF(obj + 0x38, &out.item.pos.z);
    RdF(obj + 0x24, &out.item.rot.x); RdF(obj + 0x28, &out.item.rot.y); RdF(obj + 0x2C, &out.item.rot.z);
    auto b = boundNet_.find(ref);
    if (b != boundNet_.end()) {
        out.net = b->second;
        out.boundDrop = true;
    } else {
        NetId net = 0;
        FormRef fr;
        if ((ref >> 24) != 0xFF && nvhooks::resolver().toRef(ref, &fr) && !fr.plugin.empty()) {   // a placed item
            net = nvcoop::actors::deriveNetId(fr);
            out.placedRef = fr;   // the net id is a hash: the others find this ref from plugin:localId, nothing maps the id back
        }
        out.net = net;
    }
    out.item.id = out.net;
    return true;
}

RefId RealLootEngine::createItem(const lc::WorldItem& it) {
    // an unbound ref of the same base at (nearly) the same place is the dropped object itself (the host reloaded its save): bind it
    const uint32_t baseId = nvhooks::resolver().fromRef(it.base);
    if (!baseId) return 0;
    const RefId anchor = anchor_ ? anchor_(it.cell) : 0;
    if (!anchor) return 0;   // the cell is not loaded here: the caller retries
    char line[160];
    std::snprintf(line, sizeof line, "PlaceAtMe %08X 1", baseId);
    // TODO-RE: PlaceAtMe's result ref is not readable from a console line (the glue supplies newestOf), and the stack size (ExtraCount) of the new ref is not set.
    if (!runOn(anchor, line)) return 0;
    const RefId made = newestOf_ ? newestOf_(baseId) : 0;
    if (!made) return 0;
    char p[160];
    std::snprintf(p, sizeof p, "SetPos X %f", it.pos.x); runOn(made, p);
    std::snprintf(p, sizeof p, "SetPos Y %f", it.pos.y); runOn(made, p);
    std::snprintf(p, sizeof p, "SetPos Z %f", it.pos.z); runOn(made, p);
    boundNet_[made] = it.id;
    netRef_[it.id] = made;
    return made;
}

void RealLootEngine::bindItem(RefId ref, NetId net) {
    boundNet_[ref] = net;
    netRef_[net] = ref;
}

bool RealLootEngine::removeItem(NetId net, const FormRef& placed) {
    auto it = netRef_.find(net);
    RefId ref = it != netRef_.end() ? it->second : 0;
    if (!ref) {
        for (const auto& kv : boundNet_) if (kv.second == net) { ref = kv.first; break; }
    }
    if (!ref && !placed.plugin.empty()) {
        // A placed item (a persistent ref of a plugin): the net id is a hash of plugin:localId, so only the ref the sender named can be found. (The actor
        // registry behind actors::engine::refOf knows actors only: asking it for a Stimpak on the floor answers 0, which is why a pickup never reached
        // the other copies.)
        const uint32_t id = nvhooks::resolver().fromRef(placed);
        if (id && formAddr(id)) ref = id;
    }
    const uint32_t obj = ref ? formAddr(ref) : 0;
    boundNet_.erase(ref);
    netRef_.erase(net);
    if (!obj) return false;
    uint32_t flags = 0;
    if (Rd32(obj + 8, &flags) && (flags & (kFormDeleted | kFormDisabled))) return true;    // gone already (the machine that took it, or a repeat): done. (false means: no such ref HERE yet, the caller asks again)
    return runOn(ref, "Disable") && runOn(ref, "MarkForDelete");
}

bool RealLootEngine::isNoteBase(const FormRef& base) {
    const uint32_t id = nvhooks::resolver().fromRef(base);
    const uint32_t form = id ? formAddr(id) : 0;
    uint8_t t = 0;
    return form && Rd8(form + 4, &t) && t == 0x31;   // BGSNote
}

bool RealLootEngine::addNote(const FormRef& note) {
    const uint32_t id = nvhooks::resolver().fromRef(note);
    if (!id) return false;
    nvhooks::CaptureHub* hub = nvhooks::hub();
    char line[64];
    std::snprintf(line, sizeof line, "AddNote %08X", id);   // the same line the remote applier uses; the engine's AddNote hook records the effect into the open scope
    if (!hub) return nvhooks::execConsoleLine(line);
    fnvcoop::Cause c;
    c.kind = fnvcoop::CauseKind::Intent;
    c.source = note;
    nvhooks::ScopedCapture scope(*hub, c);
    return nvhooks::execConsoleLine(line);
}

bool RealLootEngine::readPose(NetId net, lc::Vec3& pos, lc::Vec3& rot) {
    auto it = netRef_.find(net);
    if (it == netRef_.end()) return false;
    const uint32_t obj = formAddr(it->second);
    if (!obj) return false;
    return RdF(obj + 0x30, &pos.x) && RdF(obj + 0x34, &pos.y) && RdF(obj + 0x38, &pos.z) && RdF(obj + 0x24, &rot.x) && RdF(obj + 0x28, &rot.y) && RdF(obj + 0x2C, &rot.z);
}

bool RealLootEngine::setPose(NetId net, const lc::Vec3& pos, const lc::Vec3& rot, bool moving) {
    (void)moving;
    auto it = netRef_.find(net);
    if (it == netRef_.end() || !formAddr(it->second)) return false;
    char l[96];
    bool ok = true;
    std::snprintf(l, sizeof l, "SetPos X %f", pos.x); ok &= runOn(it->second, l);
    std::snprintf(l, sizeof l, "SetPos Y %f", pos.y); ok &= runOn(it->second, l);
    std::snprintf(l, sizeof l, "SetPos Z %f", pos.z); ok &= runOn(it->second, l);
    (void)rot;   // TODO-RE: SetAngle in degrees per axis; resting objects keep the owner's rotation only roughly until then
    return ok;
}

}  // namespace nvcoop::loot
