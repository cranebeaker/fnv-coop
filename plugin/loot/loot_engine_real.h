// The real ILootEngine over xNVSE (x86, nvcoop.dll only). See loot_engine_real.cpp for the engine facts, pins and TODO-RE list.
#pragma once
#include <functional>
#include <map>
#include "loot/loot_types.h"

namespace nvcoop::loot {

class RealLootEngine : public ILootEngine {
public:
    RealLootEngine();
    bool pinsOk() const { return pinsOk_; }
    bool menuPinOk() const { return menuPinOk_; }
    void bumpEpoch() { ++epoch_; refOfNet_.clear(); boundNet_.clear(); netRef_.clear(); menuRef_ = 0; }
    // glue hooks for the pieces without a verified engine call
    void setCloseMenuHook(std::function<bool()> f) { closeHook_ = std::move(f); }
    void setAnchorHook(std::function<RefId(lc::CellId)> f) { anchor_ = std::move(f); }     // a loaded ref inside that cell (PlaceAtMe anchor); 0 = cell not loaded
    void setNewestOfHook(std::function<RefId(uint32_t baseId)> f) { newestOf_ = std::move(f); }   // the dynamic ref a PlaceAtMe of that base just made

    bool worldReady() override;
    uint32_t worldEpoch() override;
    bool describeContainer(RefId ref, ContainerDesc& out) override;
    RefId refOfContainer(NetId net) override;
    bool readContents(NetId net, Items& out, bool& exact) override;
    bool setContents(NetId net, const Items& want) override;
    RefId openMenuRef() override;
    bool closeMenu() override;
    bool adjustPlayer(const FormRef& base, int32_t delta) override;
    bool playerCount(const FormRef& base, int32_t& out) override;
    // Test helper (coop.loot {"grab":..}): the two halves of a quick-loot style take, with the guest gate lifted like every other call this engine makes.
    bool testGrab(RefId container, uint32_t itemId, int32_t count, bool remove, bool add, std::string* why);
    void notify(lc::Notice n, NetId id) override;
    void loadedContainers(std::vector<NetId>& out, size_t max) override;
    void nearbyContainers(std::vector<RefId>& out, size_t max) override;
    void nearbyCells(std::vector<lc::CellId>& out, size_t max) override;
    void containersInCell(lc::CellId cell, std::vector<RefId>& out, size_t max) override;
    bool describeItem(RefId ref, ItemDesc& out) override;
    RefId createItem(const lc::WorldItem& it) override;
    void bindItem(RefId ref, NetId net) override;
    bool removeItem(NetId net, const FormRef& placed) override;
    bool isNoteBase(const FormRef& base) override;
    bool addNote(const FormRef& note) override;
    bool readPose(NetId net, lc::Vec3& pos, lc::Vec3& rot) override;
    bool setPose(NetId net, const lc::Vec3& pos, const lc::Vec3& rot, bool moving) override;

private:
    uint32_t formAddr(RefId id);
    bool netOfRef(RefId ref, uint32_t obj, NetId* net);
    bool runOn(RefId ref, const std::string& line);

    bool pinsOk_ = false, menuPinOk_ = false;
    uint32_t epoch_ = 1;
    RefId menuRef_ = 0;
    std::map<NetId, RefId> refOfNet_;     // containers seen (describeContainer): net -> ref
    std::map<RefId, NetId> boundNet_;     // world items: dropped ref -> issued id
    std::map<NetId, RefId> netRef_;       // issued id -> ref
    std::function<bool()> closeHook_;
    std::function<RefId(lc::CellId)> anchor_;
    std::function<RefId(uint32_t)> newestOf_;
};

}  // namespace nvcoop::loot
