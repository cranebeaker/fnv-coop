// A fake game for the puppet driver tests. It models the engine behaviours the puppet traps come from:
//   * commands to a ref without 3D are refused (restrained / just spawned actors), counted as refusedNo3D;
//   * 3D appears a few frames after Spawn and is REBUILT (restraint lost, animation reset, generation bumps) by MoveToCell;
//   * the actor's AI can drop the restraint, strip equipment, put the default outfit back on;
//   * menus reset animation silently (CurrentGroup then reports -1 when reportsCurrentGroup is off);
//   * deleted ref ids may be reused by something else, cells unload refs, loads leave orphans from the old save;
//   * SetPos across cells puts the actor in the wrong cell (violation), equip needs ownership and replaces a slot.
// Header-only; compiled into the test executable only.
#pragma once
#include <algorithm>
#include <cmath>
#include <functional>
#include <map>
#include <set>
#include <string>
#include <vector>

#include "../avatar_groups.h"
#include "../puppet_engine.h"

namespace nvcoop::avatar {

class FakePuppetEngine : public IPuppetEngine {
public:
    struct FakeRef {
        RefId id = 0;
        FormId base = 0;
        CellId cell = 0;
        Vec3 pos;
        float yaw = 0;
        bool exists = false, tagged = false, disabled = false, deleted = false;
        bool restrained = false, nonCombatant = false, sneak = false, drawn = false, dead = false;
        int load3dLeft = 0;
        uint32_t gen3d = 1;
        int group = 0;
        std::map<FormId, int> inv;
        std::set<FormId> worn;
        std::map<FormId, uint8_t> mods;          // weapon form -> mod flags the puppet really has
        Appearance appearance;
        bool appearanceApplied = false;
    };

    // ---- knobs ----------------------------------------------------------------------------------------------------
    bool worldReady = true;
    uint32_t worldEpoch = 1;
    bool menuOpen = false;
    std::set<CellId> loadedCells;
    std::set<CellId> exterior;                 // cells of one shared exterior worldspace
    int load3DFrames = 2;                      // frames until 3D appears after Spawn / MoveToCell
    int spawnFailuresLeft = 0;
    bool reuseRefIds = false;
    bool refuseWithout3D = true;
    bool rebuild3DDropsRestraint = true;
    bool reportsCurrentGroup = true;
    bool menuResetsGroup = true;
    int playGroupRefusalsLeft = 0;
    bool playGroupSilent = false;              // PlayGroup says ok but the actor plays nothing (a group its state rejects)
    bool attackNeedsDrawn = false;             // an attack group plays only on an actor whose weapon is out (a sheathed puppet does not throw a punch)
    unsigned silentPlays = 0;                  // PlayGroup calls that were accepted and did nothing
    bool weaponDrawSupported = true;
    bool equippedKnown = true;
    bool requireItemOwned = true;
    int equipFailuresLeft = 0;
    // weapon modifications
    bool modsReadable = true;                  // WeaponMods answers (false: -1, the engine cannot tell)
    bool modsAccepted = true;                  // EnableWeaponMod works
    unsigned modCalls = 0, modReads = 0;
    std::vector<std::pair<FormId, int>> modLog;   // every accepted (weapon, slot)
    std::map<FormId, int> slotOf;              // item -> slot; equipping replaces the item in the same slot
    std::set<FormId> weaponItems;
    std::set<FormId> defaultOutfit;            // worn by every new puppet (the template NPC's own clothes)
    std::map<FormId, int> aiReequipLeft;       // the actor's AI puts this item back on, this many times
    std::function<CellId(const Vec3&)> cellOfPos;   // truth for "which cell is this position in"
    Vec3 menuDisplacement;                     // closing a menu leaves puppets this far from where they were (AI moved them)
    // The real player of this machine as a body. A puppet written (Spawn / SetPos / MoveToCell) closer than bodyRadius to it, at nearly the same
    // height, shoves the player away by the overlap: the engine pushing two capsules apart, counted in localShoves / localTravel.
    bool haveLocal = false;
    CellId localCell = 0;
    Vec3 localPos;
    float bodyRadius = 60.0f;
    unsigned localShoves = 0;
    float localTravel = 0;                     // total distance the real player was shoved

    // ---- observations ----------------------------------------------------------------------------------------------
    unsigned refusedNo3D = 0, commandsOnInvalid = 0, commandsOnForeign = 0, setPosCalls = 0, setYawCalls = 0, playGroupCalls = 0;
    unsigned moveToCellCalls = 0, spawnCalls = 0, deleteCalls = 0, equipCalls = 0, unequipCalls = 0, addCalls = 0, removeCalls = 0;
    unsigned resetHealthCalls = 0, isDeadCalls = 0;
    unsigned drawCalls = 0, drawOnCalls = 0, restrainCalls = 0, getEquippedCalls = 0, guardCalls = 0;
    unsigned Writes() const { return guardCalls + spawnCalls + deleteCalls; }
    std::vector<std::string> violations;
    std::vector<std::string> trace;
    std::vector<std::pair<RefId, int>> groupLog;   // every successful PlayGroup
    std::vector<int> groupFlags;                   // the flags of each entry of groupLog (bit 0 = hold)
    std::vector<float> yawLog;
    std::vector<RefId> deletedRefs;

    FakeRef* Get(RefId r) {
        auto it = refs_.find(r);
        return it == refs_.end() ? nullptr : &it->second;
    }
    const FakeRef* Get(RefId r) const {
        auto it = refs_.find(r);
        return it == refs_.end() ? nullptr : &it->second;
    }
    std::vector<RefId> LiveTagged() const {
        std::vector<RefId> v;
        for (const auto& kv : refs_)
            if (kv.second.exists && kv.second.tagged && !kv.second.deleted) v.push_back(kv.first);
        return v;
    }
    size_t LiveCount() const { return LiveTagged().size(); }

    // ---- test helpers (the "world" doing things) ------------------------------------------------------------------
    void Frame() {   // one engine frame
        for (auto& kv : refs_) {
            FakeRef& r = kv.second;
            if (!r.exists) continue;
            if (r.load3dLeft > 0) --r.load3dLeft;
            for (auto& ai : aiReequipLeft)
                if (ai.second > 0 && r.exists && r.inv.count(ai.first) && r.inv[ai.first] > 0 && !r.worn.count(ai.first)) {
                    r.worn.insert(ai.first);
                    --ai.second;
                }
        }
    }
    void Unrestrain(RefId r) { if (FakeRef* f = Get(r)) f->restrained = false; }
    void SilentGroupReset(RefId r) { if (FakeRef* f = Get(r)) f->group = 0; }
    void Rebuild3DInstant(RefId r) {   // 3D rebuilt without a gap (cell reload): generation bumps, animation is gone
        if (FakeRef* f = Get(r)) {
            ++f->gen3d;
            f->group = 0;
        }
    }
    void KillRef(RefId r) { if (FakeRef* f = Get(r)) { f->exists = false; } }   // cell unloaded / engine cleaned it up
    // The engine reuses the id of a deleted ref for something unrelated.
    void HijackId(RefId r) {
        if (FakeRef* f = Get(r)) {
            f->exists = true;
            f->tagged = false;
            f->deleted = false;
            f->base = 0xDEAD;
        }
    }
    RefId PlantOrphan(CellId cell, const Vec3& pos) {   // a puppet that was saved into the game
        FakeRef r;
        r.id = nextId_++;
        r.cell = cell;
        r.pos = pos;
        r.exists = true;
        r.tagged = true;
        refs_[r.id] = r;
        return r.id;
    }
    void SetMenu(bool open) {
        if (menuOpen && !open) {
            for (auto& kv : refs_) {
                if (menuResetsGroup) kv.second.group = 0;   // animation is reset by closing a menu
                kv.second.pos.x += menuDisplacement.x;
                kv.second.pos.y += menuDisplacement.y;
                kv.second.pos.z += menuDisplacement.z;
            }
        }
        menuOpen = open;
    }
    // What the save writer would serialize right now.
    std::vector<RefId> SaveImage() const { return LiveTagged(); }
    // A load: every ref is gone; the saved image comes back as fresh orphan refs.
    void Load(const std::vector<RefId>& image, CellId cell = 0) {
        for (auto& kv : refs_) kv.second.exists = false;
        ++worldEpoch;
        for (size_t i = 0; i < image.size(); ++i) PlantOrphan(cell, Vec3{});
    }
    // A body was put at `pos` in `cell`: push the real player out of it.
    void BodyContact(CellId cell, const Vec3& pos) {
        if (!haveLocal || cell != localCell || std::fabs(pos.z - localPos.z) > 110.0f) return;
        float dx = localPos.x - pos.x, dy = localPos.y - pos.y;
        float d = std::sqrt(dx * dx + dy * dy);
        if (d >= bodyRadius) return;
        if (d < 0.001f) {
            dx = 1.0f;
            dy = 0.0f;
            d = 1.0f;
        }
        const float push = bodyRadius - d;
        localPos.x += dx / d * push;
        localPos.y += dy / d * push;
        localTravel += push;
        ++localShoves;
    }
    void ClearTrace() {
        trace.clear();
        groupLog.clear();
        groupFlags.clear();
        yawLog.clear();
    }

    // ---- IPuppetEngine ---------------------------------------------------------------------------------------------
    bool WorldReady() override { return worldReady; }
    uint32_t WorldEpoch() override { return worldEpoch; }
    bool MenuOpen() override { return menuOpen; }
    bool CellLoaded(CellId c) override { return loadedCells.count(c) != 0; }
    bool SameSpace(CellId a, CellId b) override { return exterior.count(a) && exterior.count(b); }

    // An engine that builds each puppet from its player's look (GamePuppetEngine with a look copy): LookBase hands back a private base per call and keeps
    // the look it was asked for, so a test can see what a puppet was built from.
    bool looksByBase = false;
    std::vector<std::pair<Slot, Appearance>> lookBaseCalls;
    bool LooksByBase() override { return looksByBase; }
    FormId LookBase(Slot slot, FormId templateBase, const Appearance& a) override {
        if (!looksByBase) return templateBase;
        lookBaseCalls.push_back({slot, a});
        return templateBase;
    }

    RefId Spawn(FormId base, CellId cell, const Vec3& pos, float yaw) override {
        ++spawnCalls;
        trace.push_back("Spawn");
        if (spawnFailuresLeft > 0) {
            --spawnFailuresLeft;
            return 0;
        }
        if (!loadedCells.count(cell)) return 0;
        FakeRef r;
        if (reuseRefIds && !deletedRefs.empty()) {
            r.id = deletedRefs.back();
            deletedRefs.pop_back();
        } else {
            r.id = nextId_++;
        }
        r.base = base;
        r.cell = cell;
        r.pos = pos;
        r.yaw = yaw;
        r.exists = r.tagged = true;
        r.load3dLeft = load3DFrames;
        for (FormId f : defaultOutfit) {
            r.inv[f] = 1;
            r.worn.insert(f);
        }
        refs_[r.id] = r;
        BodyContact(cell, pos);
        return r.id;
    }
    void Delete(RefId ref) override {
        ++deleteCalls;
        trace.push_back("Delete");
        if (FakeRef* f = Get(ref)) {
            if (f->exists && !f->tagged) { ++commandsOnForeign; return; }   // never delete somebody else's ref
            f->deleted = true;
            f->exists = false;
            deletedRefs.push_back(ref);
        }
    }
    bool IsValid(RefId ref) override {
        const FakeRef* f = Get(ref);
        return f && f->exists && f->tagged && !f->deleted;
    }
    bool Has3D(RefId ref) override {
        const FakeRef* f = Get(ref);
        return f && f->exists && f->load3dLeft == 0 && !f->disabled;
    }
    uint32_t Generation3D(RefId ref) override {
        const FakeRef* f = Get(ref);
        return f ? f->gen3d : 0;
    }
    std::vector<RefId> FindTaggedPuppets() override { return LiveTagged(); }

    bool SetRestrained(RefId ref, bool on) override {
        ++restrainCalls;
        FakeRef* f = Guard(ref, "SetRestrained");
        if (!f) return false;
        trace.push_back("Restrain");
        f->restrained = on;
        return true;
    }
    bool IsRestrained(RefId ref) override {
        const FakeRef* f = Get(ref);
        return f && f->exists && f->restrained;
    }
    bool MakeNonCombatant(RefId ref) override {
        FakeRef* f = Guard(ref, "MakeNonCombatant");
        if (!f) return false;
        f->nonCombatant = true;
        return true;
    }
    bool ApplyAppearance(RefId ref, const Appearance& a) override {
        FakeRef* f = Guard(ref, "ApplyAppearance");
        if (!f) return false;
        f->appearance = a;
        f->appearanceApplied = true;
        return true;
    }

    bool SetPos(RefId ref, const Vec3& pos) override {
        FakeRef* f = Guard(ref, "SetPos");
        if (!f) return false;
        ++setPosCalls;
        trace.push_back("SetPos");
        if (cellOfPos) {
            CellId truth = cellOfPos(pos);
            if (truth != f->cell && !SameSpace(truth, f->cell))
                violations.push_back("SetPos moved a ref from cell " + std::to_string(f->cell) + " to a position in cell " + std::to_string(truth));
        }
        f->pos = pos;
        BodyContact(f->cell, pos);
        return true;
    }
    bool SetYaw(RefId ref, float yaw) override {
        FakeRef* f = Guard(ref, "SetYaw");
        if (!f) return false;
        ++setYawCalls;
        trace.push_back("SetYaw");
        yawLog.push_back(yaw);
        f->yaw = yaw;
        return true;
    }
    bool MoveToCell(RefId ref, CellId cell, const Vec3& pos, float yaw) override {
        FakeRef* f = Guard(ref, "MoveToCell");
        if (!f) return false;
        if (!loadedCells.count(cell)) return false;
        ++moveToCellCalls;
        trace.push_back("MoveToCell");
        f->cell = cell;
        f->pos = pos;
        f->yaw = yaw;
        f->load3dLeft = load3DFrames;
        ++f->gen3d;
        f->group = 0;
        if (rebuild3DDropsRestraint) f->restrained = false;
        BodyContact(cell, pos);
        return true;
    }
    bool GetLocalPose(CellId& cell, Vec3& pos) override {
        if (!haveLocal) return false;
        cell = localCell;
        pos = localPos;
        return true;
    }
    bool GetPose(RefId ref, CellId& cell, Vec3& pos) override {
        const FakeRef* f = Get(ref);
        if (!f || !f->exists) return false;
        cell = f->cell;
        pos = f->pos;
        return true;
    }
    bool PlayGroup(RefId ref, int baseGroup, int flags) override {
        FakeRef* f = Guard(ref, "PlayGroup");
        if (!f) return false;
        if (!GroupName(baseGroup)) return false;
        if (playGroupRefusalsLeft > 0) {
            --playGroupRefusalsLeft;
            return false;
        }
        ++playGroupCalls;
        if (playGroupSilent || (attackNeedsDrawn && IsAttackGroup(baseGroup) && !f->drawn)) {
            ++silentPlays;
            return true;
        }
        f->group = baseGroup;
        groupLog.push_back({ref, baseGroup});
        groupFlags.push_back(flags);
        trace.push_back("PlayGroup:" + std::to_string(baseGroup));
        return true;
    }
    int CurrentGroup(RefId ref) override {
        const FakeRef* f = Get(ref);
        if (!f || !f->exists || !reportsCurrentGroup) return -1;
        return f->group;
    }
    bool SetSneak(RefId ref, bool on) override {
        FakeRef* f = Guard(ref, "SetSneak");
        if (!f) return false;
        f->sneak = on;
        return true;
    }
    bool SetAlert(RefId ref, bool) override { return Guard(ref, "SetAlert") != nullptr; }
    bool SetWeaponDrawn(RefId ref, bool on) override {
        FakeRef* f = Guard(ref, "SetWeaponDrawn");
        if (!f || !weaponDrawSupported) return false;
        ++drawCalls;
        if (on) {
            ++drawOnCalls;
            bool haveWeapon = false;
            for (FormId w : f->worn)
                if (weaponItems.count(w)) haveWeapon = true;
            if (!haveWeapon && !weaponItems.empty()) violations.push_back("weapon drawn with no weapon equipped");
        }
        f->drawn = on;
        return true;
    }
    bool IsWeaponDrawn(RefId ref) override {
        const FakeRef* f = Get(ref);
        return f && f->drawn;
    }

    bool GetEquipped(RefId ref, std::vector<FormId>& out) override {
        ++getEquippedCalls;
        const FakeRef* f = Get(ref);
        if (!f || !f->exists || !equippedKnown) return false;
        out.assign(f->worn.begin(), f->worn.end());
        return true;
    }
    bool IsDead(RefId ref) override {
        ++isDeadCalls;
        const FakeRef* f = Get(ref);
        return f && f->exists && f->dead;
    }
    bool ResetHealth(RefId ref) override {
        FakeRef* f = Guard(ref, "ResetHealth");
        if (!f) return false;
        ++resetHealthCalls;
        return !f->dead;
    }
    int WeaponMods(RefId ref, FormId weapon) override {
        ++modReads;
        const FakeRef* f = Get(ref);
        if (!modsReadable || !f || !f->exists || !f->worn.count(weapon)) return -1;
        auto it = f->mods.find(weapon);
        return it == f->mods.end() ? 0 : it->second;
    }
    bool EnableWeaponMod(RefId ref, FormId weapon, int slot) override {
        FakeRef* f = Guard(ref, "EnableWeaponMod");
        ++modCalls;
        if (!f || !modsAccepted || slot < 1 || slot > 3 || !f->worn.count(weapon)) return false;
        f->mods[weapon] = static_cast<uint8_t>(f->mods[weapon] | (1u << (slot - 1)));
        modLog.emplace_back(weapon, slot);
        return true;
    }
    int ItemCount(RefId ref, FormId item) override {
        const FakeRef* f = Get(ref);
        if (!f || !f->exists) return 0;
        auto it = f->inv.find(item);
        return it == f->inv.end() ? 0 : it->second;
    }
    bool AddItem(RefId ref, FormId item, int count) override {
        FakeRef* f = Guard(ref, "AddItem");
        if (!f || EquipFail()) return false;
        ++addCalls;
        f->inv[item] += count;
        return true;
    }
    bool RemoveItem(RefId ref, FormId item, int count) override {
        FakeRef* f = Guard(ref, "RemoveItem");
        if (!f || EquipFail()) return false;
        ++removeCalls;
        trace.push_back("RemoveItem");
        int& n = f->inv[item];
        n = std::max(0, n - count);
        if (n == 0) f->worn.erase(item);
        return true;
    }
    bool EquipItem(RefId ref, FormId item) override {
        FakeRef* f = Guard(ref, "EquipItem");
        if (!f || EquipFail()) return false;
        ++equipCalls;
        if (requireItemOwned && ItemCount(ref, item) <= 0) return false;
        auto s = slotOf.find(item);
        if (s != slotOf.end())
            for (auto it = f->worn.begin(); it != f->worn.end();) {
                auto o = slotOf.find(*it);
                if (o != slotOf.end() && o->second == s->second && *it != item) it = f->worn.erase(it);
                else ++it;
            }
        f->worn.insert(item);
        return true;
    }
    bool UnequipItem(RefId ref, FormId item) override {
        FakeRef* f = Guard(ref, "UnequipItem");
        if (!f || EquipFail()) return false;
        ++unequipCalls;
        trace.push_back("Unequip");
        if (!f->worn.erase(item)) return false;
        return true;
    }

private:
    bool EquipFail() {
        if (equipFailuresLeft > 0) {
            --equipFailuresLeft;
            return true;
        }
        return false;
    }
    // Common gate for commands that need a live, loaded actor.
    FakeRef* Guard(RefId ref, const char* what) {
        ++guardCalls;
        FakeRef* f = Get(ref);
        if (!f || !f->exists) {
            ++commandsOnInvalid;
            violations.push_back(std::string(what) + " on a ref that no longer exists");
            return nullptr;
        }
        if (!f->tagged) {
            ++commandsOnForeign;
            violations.push_back(std::string(what) + " on a ref that is not our puppet (reused id)");
            return nullptr;
        }
        if (refuseWithout3D && f->load3dLeft > 0) {
            ++refusedNo3D;
            return nullptr;
        }
        return f;
    }
    std::map<RefId, FakeRef> refs_;
    RefId nextId_ = 0xFF000100;
};

}  // namespace nvcoop::avatar
