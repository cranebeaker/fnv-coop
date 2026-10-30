// A fake game behind IGameBackend, for the engine tests. It is deliberately STRICT about the console grammar: a line the real
// engine would not compile (wrong word, wrong argument count, a non-hex form id, an unknown group name) is refused and recorded in
// `badLines`, so a typo in puppet_game_engine.cpp fails a test instead of silently doing nothing in the game.
//
// What it models (each is an engine behaviour the puppet layer has to survive; none is measured in the real game):
//   * a reference has no 3D for `load3DFrames` frames after creation / MoveToMarker and refuses most commands meanwhile;
//   * PlaceAtMe puts a new actor in the calling reference's cell, with a rising id (or a re-used one), and can create extras;
//   * Disable / MarkForDelete flag the reference; it can vanish after a delay or stay (flagged deleted);
//   * MoveToMarker rebuilds the 3D (new scene object, restraint lost, animation reset);
//   * condition evaluators can be switched off one by one (pin mismatch), equipment can be unreadable;
//   * the player is reference 0x14 and can be moved / animated by the test.
// Header-only; compiled into the test executable only.
#pragma once
#include <algorithm>
#include <cctype>
#include <cmath>
#include <cstdlib>
#include <map>
#include <set>
#include <sstream>
#include <string>
#include <vector>

#include "../../avatar_groups.h"
#include "../game_backend.h"

namespace nvcoop::avatar::engine {

class FakeGameBackend : public IGameBackend {
public:
    struct FRef {
        RefId id = 0;
        FormId base = 0;
        CellId cell = 0;
        Vec3 pos, rot;
        bool exists = true, isActor = true, isPlayer = false, deleted = false, disabled = false;
        int load3dLeft = 0;
        uint32_t node = 0;
        bool restrained = false, ghost = false, ignoreFriendly = false, sneak = false, alert = false, aiming = false;
        int groupSlot = 1;
        int group = -1;                      // running group id (-1 none)
        int groupState = 1;                  // its play state as the engine reports it (1 = animated, 0 = a finished leftover in the slot)
        int groupPlayMs = -1;                // how long it has been playing (-1 = the backend cannot tell)
        // The local player's first-person AnimData (PlayerCharacter+0x690): a separate set that holds what he plays in first person.
        bool fpSet = false;                  // the set exists (reads as valid, with nothing in it until fpGroup is set)
        int fpGroup = -1, fpSlot = 1, fpState = 1, fpPlayMs = -1;
        std::map<FormId, int> inv;
        std::set<FormId> worn;
        std::map<FormId, uint8_t> mods;      // weapon form -> ExtraWeaponModFlags
        float scale = 1.0f;                  // SetScale
        int health = 100;                    // Hit() lowers it, ResetHealth restores it (a dead actor stays dead)
        int vanishIn = -1;                   // frames until a deleted reference disappears (-1 = never)
        uint32_t moverFlags = 0;
        bool haveMover = false, swimming = false, dead = false, knocked = false, firstPerson = false;
        bool weaponOutForced = false;        // the local player draws its weapon without SetAlert
        bool weaponFlag = false;             // the process's weapon-out flag (BaseProcess::SetWeaponOut)
        int sitSleep = 0;                    // BaseProcess::GetSitSleepState (kSitSleepState_*: 4 sitting, 9 sleeping)
        RefId usedFurniture = 0;             // the furniture an Activate put the actor in
        int level = 1;                       // GetLevel
        int sitState = 0;                    // GetSitting
        double inventoryWeight = 0;          // GetActorValue InventoryWeight (46); tests keep it in step with `inv` by hand
        std::set<int> actions;               // IsPlayerActionActive ids that are active
        bool has3D() const { return node != 0 && load3dLeft == 0; }
    };

    // ---- knobs ----------------------------------------------------------------------------------------------------
    uint32_t epoch = 1;
    bool menuOpen = false;
    int load3DFrames = 2;
    bool refuseWithout3D = true;
    bool refusePlaceAtMe = false;
    int placeExtraActors = 0;               // PlaceAtMe also creates this many extra actors of the same base
    bool placeDisabled = false;             // PlaceAtMe creates actors that already read as disabled (a pin that lies)
    bool reuseIds = false;
    int vanishAfterFrames = -1;             // deleted references disappear after this many frames (-1 = never)
    bool restraintLostOnRebuild = true;
    bool alertDrawsWeapon = true;
    bool directWeaponOut = true;            // BaseProcess::SetWeaponOut is available (the pin holds)
    bool attackNeedsWeaponOut = false;      // an attack group plays only on an actor whose process says "weapon out" (the sheathed variant has no sequence): else PlayGroup says ok and nothing plays
    bool sitSleepReadable = true;           // ReadSitSleep answers
    bool activateSeats = true;              // Activate <actor> 1 on a furniture seats the actor (off: accepted, nothing happens)
    bool equipRequiresOwned = true;
    bool equippedReadable = true, animReadable = true, cellsReadable = true;
    bool descendingEquipped = false;        // the engine lists worn items in descending form id order
    bool modsReadable = true;               // the extra data of a worn entry can be read (EquippedItem::modsKnown)
    bool enableModWorks = true;             // EnableWeaponMod <weapon> <slot> sets bit (slot-1) of the worn weapon; false: the engine refuses
    std::set<Cond> evalBroken;              // evaluators that are unavailable
    std::string throwOnPrefix;              // RunLine returns -1 for lines starting with this
    uint32_t lineCostUs = 40;               // virtual time one console line costs
    std::set<FormId> knownBases;            // PlaceAtMe works for these
    std::set<FormId> weaponItems;
    std::map<FormId, int> slotOf;           // item -> equip slot; equipping replaces the item in the same slot
    std::set<FormId> defaultOutfit;         // worn by every new actor
    std::map<FormId, Appearance> baseAppearance;

    // ---- observations ---------------------------------------------------------------------------------------------
    struct Line { RefId ctx; std::string text; int rc; };
    std::vector<Line> lines;
    std::vector<std::string> badLines;      // lines the grammar refused
    unsigned count(const std::string& cmd) const {
        unsigned n = 0;
        for (const Line& l : lines)
            if (l.text.compare(0, cmd.size(), cmd) == 0) ++n;
        return n;
    }
    unsigned linesTo(RefId ctx) const {
        unsigned n = 0;
        for (const Line& l : lines)
            if (l.ctx == ctx) ++n;
        return n;
    }
    unsigned eventsOn(RefId r) const { return linesTo(r); }
    uint64_t now = 1000;

    FakeGameBackend() {
        FRef p;
        p.id = kPlayerRef;
        p.base = 7;
        p.isPlayer = true;
        p.node = 0x9000;
        p.haveMover = true;
        refs_[p.id] = p;
        cells_[kInterior] = CellState{{true, true, true, 0}, {}};
        cells_[kExtA] = CellState{{true, true, false, kWorld}, {}};
        cells_[kExtB] = CellState{{true, true, false, kWorld}, {}};
        cells_[kOtherInterior] = CellState{{true, true, true, 0}, {}};
        cells_[kOtherWorldExt] = CellState{{true, true, false, kWorld + 1}, {}};
        refs_[kPlayerRef].cell = kExtA;
        AddStatic(kExtA, 0x500, Vec3{0, 0, 0});
        AddStatic(kExtB, 0x501, Vec3{4096, 0, 0});
        AddStatic(kInterior, 0x502, Vec3{100, 100, 0});
        AddStatic(kOtherInterior, 0x503, Vec3{-100, 50, 0});
        AddStatic(kOtherWorldExt, 0x504, Vec3{50, 50, 0});
    }

    static constexpr CellId kExtA = 0x100, kExtB = 0x101, kInterior = 0x200, kOtherInterior = 0x201, kOtherWorldExt = 0x300;
    static constexpr uint32_t kWorld = 0xA0;

    // ---- the world, driven by the test ----------------------------------------------------------------------------
    FRef& Player() { return refs_[kPlayerRef]; }
    // Never null: a missing id gives a blank, non-existing scratch ref (so a broken twin fails a check instead of crashing the test).
    FRef* Get(RefId r) {
        auto it = refs_.find(r);
        if (it != refs_.end()) return &it->second;
        none_ = FRef{};
        none_.exists = false;
        return &none_;
    }
    RefId AddStatic(CellId cell, RefId id, const Vec3& pos) {
        FRef s;
        s.id = id;
        s.cell = cell;
        s.pos = pos;
        s.isActor = false;
        s.node = 0x100 + id;
        refs_[id] = s;
        cells_[cell].statics.push_back(id);
        return id;
    }
    RefId AddActor(FormId base, CellId cell, const Vec3& pos, bool with3D = true, RefId fixedId = 0) {
        FRef a;
        a.id = fixedId ? fixedId : NewId();
        a.base = base;
        a.cell = cell;
        a.pos = pos;
        a.node = with3D ? ++nextNode_ : 0;
        a.worn = defaultOutfit;
        refs_[a.id] = a;
        return a.id;
    }
    void SetCell(CellId c, bool attached, bool interior, uint32_t world) { cells_[c].info = CellInfo{true, attached, interior, world}; }
    void RemoveCell(CellId c) { cells_.erase(c); }
    void Frame() {
        std::vector<RefId> gone;
        for (auto& kv : refs_) {
            FRef& r = kv.second;
            if (!r.exists || r.isPlayer) continue;
            if (r.load3dLeft > 0 && --r.load3dLeft == 0 && !r.node) r.node = ++nextNode_;
            if (r.deleted && r.vanishIn > 0 && --r.vanishIn == 0) gone.push_back(kv.first);
        }
        for (RefId g : gone) refs_[g].exists = false;
    }
    void Kill(RefId r) { if (FRef* f = Get(r)) f->exists = false; }
    void Rebuild3D(RefId r) {   // 3D replaced without a gap (cell reload)
        if (FRef* f = Get(r)) {
            f->node = ++nextNode_;
            f->group = -1;
            if (restraintLostOnRebuild) f->restrained = false;
        }
    }
    void Lose3D(RefId r, int frames) {
        if (FRef* f = Get(r)) {
            f->node = 0;
            f->load3dLeft = frames;
            f->group = -1;
            if (restraintLostOnRebuild) f->restrained = false;
        }
    }
    void Push(RefId r, const Vec3& to) { if (FRef* f = Get(r)) f->pos = to; }   // physics or AI moved it
    void HijackId(RefId r, FormId newBase) {   // the id now belongs to another actor
        if (FRef* f = Get(r)) {
            f->exists = true;
            f->deleted = false;
            f->base = newBase;
            f->node = ++nextNode_;
        }
    }
    void Load() { ++epoch; }
    // An enemy hit on an actor: health drops, at zero the actor is dead (GetDead = 1, the process reports it too).
    void Hit(RefId r, int damage) {
        if (FRef* f = Get(r)) {
            if (f->dead) return;
            f->health -= damage;
            if (f->health <= 0) {
                f->health = 0;
                f->dead = true;
            }
        }
    }

    // ---- IGameBackend -----------------------------------------------------------------------------------------------
    uint32_t WorldEpoch() override { return epoch; }
    bool MenuOpen() override { return menuOpen; }
    bool ReadCell(CellId cell, CellInfo& out) override {
        if (!cellsReadable) return false;
        auto it = cells_.find(cell);
        if (it == cells_.end()) {
            out = CellInfo{};
            return true;
        }
        out = it->second.info;
        return true;
    }
    RefId AnchorRef(CellId cell, Vec3& pos) override {
        auto it = cells_.find(cell);
        if (it == cells_.end()) return 0;
        for (RefId s : it->second.statics) {
            FRef* f = Get(s);
            if (f && f->exists) {
                pos = f->pos;
                return s;
            }
        }
        return 0;
    }
    void ListActors(CellId cell, std::vector<ActorEntry>& out) override {
        out.clear();
        for (auto& kv : refs_)
            if (kv.second.exists && kv.second.isActor && kv.second.cell == cell) out.push_back(ActorEntry{kv.first, kv.second.base});
    }
    void LoadedCells(std::vector<CellId>& out) override {
        out.clear();
        CellId pc = refs_[kPlayerRef].cell;
        out.push_back(pc);
        for (auto& kv : cells_)
            if (kv.first != pc && kv.second.info.attached) out.push_back(kv.first);
    }
    bool ReadRef(RefId ref, RefInfo& out) override {
        out = RefInfo{};
        FRef* f = Get(ref);
        if (!f || !f->exists) return true;   // a guarded read that finds nothing is not an error
        out.exists = true;
        out.isActor = f->isActor;
        out.isPlayer = f->isPlayer;
        out.deleted = f->deleted;
        out.disabled = f->disabled;
        out.base = f->base;
        out.cell = f->cell;
        out.pos = f->pos;
        out.rot = f->rot;
        out.has3D = f->has3D();
        out.node3D = f->node;
        out.havePos3D = f->has3D();
        out.pos3D = f->pos;
        return true;
    }
    bool ReadAnim(RefId ref, AnimRead& out) override {
        out = AnimRead{};
        FRef* f = Get(ref);
        if (!animReadable || !f || !f->exists || !f->isActor || !f->has3D()) return false;
        out.valid = true;
        if (f->group >= 0) {
            out.groups[f->groupSlot] = f->group;
            out.state[f->groupSlot] = f->groupState;
            out.playMs[f->groupSlot] = f->groupPlayMs;
            out.nGroups = 1;
        }
        if (f->fpSet || f->fpGroup >= 0) {
            out.alt[0].valid = true;
            if (f->fpGroup >= 0) {
                out.alt[0].groups[f->fpSlot] = f->fpGroup;
                out.alt[0].state[f->fpSlot] = f->fpState;
                out.alt[0].playMs[f->fpSlot] = f->fpPlayMs;
                out.alt[0].nGroups = 1;
            }
        }
        out.aiming = f->aiming;
        out.weaponOut = WeaponOut(*f);
        out.haveMover = f->haveMover;
        out.moverFlags = f->moverFlags;
        out.swimming = f->swimming;
        out.dead = f->dead;
        out.knocked = f->knocked;
        out.firstPerson = f->firstPerson;
        return true;
    }
    bool ReadEquipped(RefId ref, std::vector<EquippedItem>& out) override {
        out.clear();
        FRef* f = Get(ref);
        if (!equippedReadable || !f || !f->exists) return false;
        for (FormId w : f->worn) {
            EquippedItem e{w, static_cast<uint8_t>(weaponItems.count(w) ? kFormTypeWeapon : 0x18), 0, false};
            auto m = f->mods.find(w);
            e.mods = m == f->mods.end() ? 0 : m->second;
            e.modsKnown = modsReadable;
            out.push_back(e);
        }
        if (descendingEquipped) std::reverse(out.begin(), out.end());
        return true;
    }
    // ---- look copies (CloneNpcBase / WriteNpcLook): the model of TESNPC::CopyFrom (the copy starts as the template's look) and of the writer ----------------
    bool cloneWorks = false;                // CloneNpcBase makes a copy (off: it returns 0, as an engine without the pins does)
    int cloneRefusals = 0;                  // the next N CloneNpcBase calls return 0 even when cloneWorks
    bool faceArraysOwned = true;            // the copy owns its FaceGen arrays (CopyFrom allocated them); off: they are shared with the template and never written
    bool headPartsKnown = true;             // the head part forms of a look are known on this machine
    unsigned clones = 0;
    std::map<FormId, FormId> copyOf;        // copy -> template
    std::map<FormId, Appearance> copyLook;  // the look a copy has now
    bool CanCloneNpcBase() override { return cloneWorks; }
    FormId CloneNpcBase(FormId base) override {
        if (!cloneWorks || !knownBases.count(base)) return 0;
        if (cloneRefusals > 0) {
            --cloneRefusals;
            return 0;
        }
        const FormId id = 0xFF100000u + static_cast<FormId>(++clones);
        copyOf[id] = base;
        knownBases.insert(id);
        auto it = baseAppearance.find(base);
        copyLook[id] = it == baseAppearance.end() ? Appearance{} : it->second;   // CopyFrom: the copy begins as the template
        return id;
    }
    bool WriteNpcLook(FormId base, FormId tmpl, const Appearance& a, bool* faceWritten) override {
        if (faceWritten) *faceWritten = false;
        auto it = copyLook.find(base);
        if (it == copyLook.end() || base == tmpl) return false;   // never the template itself
        Appearance& c = it->second;
        c.female = a.female;
        if (a.race) c.race = a.race;
        if (a.hair) c.hair = a.hair;
        if (a.eyes) c.eyes = a.eyes;
        if (a.hairColor) c.hairColor = a.hairColor;
        if (a.height) c.height = a.height;
        if (a.weight) c.weight = a.weight;
        bool face = false;
        if (!a.faceCoeffs.empty() && faceArraysOwned) {
            c.faceCoeffs = a.faceCoeffs;
            face = true;
        }
        if (faceWritten) *faceWritten = face;
        bool parts = false;
        if (!a.headParts.empty() && headPartsKnown) {
            c.headParts = a.headParts;
            parts = true;
        }
        lookNote_ = std::string("face ") + (face ? "written" : "not written") + ", head parts " + (parts ? "written" : "not written");
        return true;
    }
    std::string LastLookNote() override { return lookNote_; }
    std::string lookNote_;

    // ---- the process's weapon flag, sit state, furniture ----------------------------------------------------------------------------------------
    std::vector<RefId> furniture;           // placed furniture references (AddFurniture)
    unsigned weaponOutCalls = 0;
    RefId AddFurniture(CellId cell, RefId id, const Vec3& pos) {
        AddStatic(cell, id, pos);
        furniture.push_back(id);
        return id;
    }
    bool SetWeaponOut(RefId ref, bool on) override {
        FRef* f = Get(ref);
        if (!directWeaponOut || !f || !f->exists || !f->isActor) return false;
        ++weaponOutCalls;
        f->weaponFlag = on;
        return true;
    }
    bool ReadSitSleep(RefId ref, int& out) override {
        out = 0;
        FRef* f = Get(ref);
        if (!sitSleepReadable || !f || !f->exists || !f->isActor) return false;
        out = f->sitSleep;
        return true;
    }
    RefId NearestFurniture(RefId ref) override {
        FRef* me = Get(ref);
        if (!me || !me->exists) return 0;
        RefId best = 0;
        float bestD2 = 140.0f * 140.0f;
        for (RefId id : furniture) {
            FRef* f = Get(id);
            if (!f || !f->exists || f->cell != me->cell) continue;
            const float dz = f->pos.z - me->pos.z;
            if (dz > 120.0f || dz < -120.0f) continue;
            const float dx = f->pos.x - me->pos.x, dy = f->pos.y - me->pos.y;
            const float d2 = dx * dx + dy * dy;
            if (d2 < bestD2) {
                bestD2 = d2;
                best = id;
            }
        }
        return best;
    }

    bool ReadAppearance(RefId ref, Appearance& out) override {
        FRef* f = Get(ref);
        if (!f || !f->exists) return false;
        auto cp = copyLook.find(f->base);
        if (cp != copyLook.end()) {
            out = cp->second;
            return true;
        }
        auto it = baseAppearance.find(f->base);
        if (it == baseAppearance.end()) return false;
        out = it->second;
        return true;
    }
    int RunLine(RefId ctx, const std::string& line) override {
        now += lineCostUs;
        FRef* c = Get(ctx);
        int rc = 0;
        if (!c || !c->exists) {
            rc = -2;
        } else if (!throwOnPrefix.empty() && line.compare(0, throwOnPrefix.size(), throwOnPrefix) == 0) {
            rc = -1;
        } else {
            rc = Exec(*c, line);
        }
        lines.push_back(Line{ctx, line, rc});
        return rc;
    }
    bool Eval(Cond fn, RefId ref, FormId param, double& out) override {
        out = 0;
        if (evalBroken.count(fn)) return false;
        FRef* f = Get(ref);
        if (!f || !f->exists) return false;
        if (!f->isActor) return true;
        switch (fn) {
            case Cond::GetRestrained: out = f->restrained ? 1 : 0; break;
            case Cond::IsWeaponOut: out = WeaponOut(*f) ? 1 : 0; break;
            case Cond::IsSneaking: out = f->sneak || (f->moverFlags & (1u << 9)) ? 1 : 0; break;
            case Cond::GetDead: out = f->dead ? 1 : 0; break;
            case Cond::GetItemCount: {
                auto it = f->inv.find(param);
                out = it == f->inv.end() ? 0 : it->second;
                break;
            }
            case Cond::GetEquipped: out = f->worn.count(param) ? 1 : 0; break;
            case Cond::GetLevel: out = f->level; break;
            case Cond::GetSitting: out = f->sitState; break;
            case Cond::IsPlayerActionActive: out = f->actions.count(static_cast<int>(param)) ? 1 : 0; break;
            case Cond::GetActorValue: out = param == 46 ? f->inventoryWeight : 0; break;   // only InventoryWeight is modelled
        }
        return true;
    }
    std::vector<FormId> playerRegions;       // PlayerCharacter::regionsList
    bool regionsReadable = true;
    bool ReadPlayerRegions(std::vector<FormId>& out, size_t max) override {
        out.clear();
        if (!regionsReadable) return false;
        for (FormId r : playerRegions) if (out.size() < max) out.push_back(r);
        return true;
    }
    uint64_t NowUs() override { return now; }

private:
    struct CellState {
        CellInfo info;
        std::vector<RefId> statics;
    };

    RefId NewId() {
        if (reuseIds && !freeIds_.empty()) {
            RefId r = freeIds_.back();
            freeIds_.pop_back();
            return r;
        }
        return 0xFF000000u + (++nextId_);
    }
    bool WeaponOut(const FRef& f) const {
        if (f.weaponOutForced || f.weaponFlag) return true;
        if (!f.alert || !alertDrawsWeapon) return false;
        for (FormId w : f.worn)
            if (weaponItems.count(w)) return true;
        return false;
    }
    static std::vector<std::string> Split(const std::string& s) {
        std::vector<std::string> v;
        std::istringstream is(s);
        std::string w;
        while (is >> w) v.push_back(w);
        return v;
    }
    static bool Ieq(const std::string& a, const char* b) {
        size_t n = std::char_traits<char>::length(b);
        if (a.size() != n) return false;
        for (size_t i = 0; i < n; ++i)
            if (std::tolower(static_cast<unsigned char>(a[i])) != std::tolower(static_cast<unsigned char>(b[i]))) return false;
        return true;
    }
    static bool IsHex8(const std::string& s) {
        if (s.size() != 8) return false;
        for (char ch : s)
            if (!std::isxdigit(static_cast<unsigned char>(ch))) return false;
        return true;
    }
    static bool IsNum(const std::string& s) {
        if (s.empty()) return false;
        char* end = nullptr;
        std::strtod(s.c_str(), &end);
        return end && *end == 0 && s.find(',') == std::string::npos;
    }
    static bool IsInt(const std::string& s) {
        if (s.empty()) return false;
        size_t i = s[0] == '-' ? 1 : 0;
        if (i >= s.size()) return false;
        for (; i < s.size(); ++i)
            if (!std::isdigit(static_cast<unsigned char>(s[i]))) return false;
        return true;
    }
    int Bad(const std::string& line) {
        badLines.push_back(line);
        return 0;
    }

    int Exec(FRef& c, const std::string& line) {
        std::vector<std::string> w = Split(line);
        if (w.empty()) return Bad(line);
        const std::string& cmd = w[0];
        const bool needs3D = refuseWithout3D && !c.has3D();
        if (Ieq(cmd, "PlaceAtMe")) {
            if (w.size() != 5 || !IsHex8(w[1]) || !IsInt(w[2]) || !IsNum(w[3]) || !IsNum(w[4])) return Bad(line);
            FormId base = static_cast<FormId>(std::strtoul(w[1].c_str(), nullptr, 16));
            if (refusePlaceAtMe || !knownBases.count(base)) return 0;
            for (int i = 0; i <= placeExtraActors; ++i) {
                RefId id = AddActor(base, c.cell, c.pos, false);
                FRef& n = refs_[id];
                n.load3dLeft = load3DFrames;
                n.disabled = placeDisabled;
                if (load3DFrames == 0) n.node = ++nextNode_;
                if (vanishAfterFrames >= 0) n.vanishIn = vanishAfterFrames;
            }
            return 1;
        }
        if (Ieq(cmd, "Disable")) {
            if (w.size() != 1) return Bad(line);
            c.disabled = true;
            return 1;
        }
        if (Ieq(cmd, "MarkForDelete")) {
            if (w.size() != 1) return Bad(line);
            c.deleted = true;
            c.vanishIn = vanishAfterFrames;
            if (c.vanishIn == 0) c.exists = false;
            if (reuseIds) freeIds_.push_back(c.id);
            return 1;
        }
        // everything below needs a loaded 3D (the engine refuses a restrained / just placed actor otherwise)
        if (Ieq(cmd, "SetPos")) {
            if (w.size() != 3 || w[1].size() != 1 || !IsNum(w[2])) return Bad(line);
            if (needs3D) return 0;
            float v = static_cast<float>(std::atof(w[2].c_str()));
            char a = static_cast<char>(std::toupper(static_cast<unsigned char>(w[1][0])));
            if (a == 'X') c.pos.x = v;
            else if (a == 'Y') c.pos.y = v;
            else if (a == 'Z') c.pos.z = v;
            else return Bad(line);
            return 1;
        }
        if (Ieq(cmd, "SetAngle")) {
            if (w.size() != 3 || w[1].size() != 1 || !IsNum(w[2])) return Bad(line);
            if (needs3D) return 0;
            float v = static_cast<float>(std::atof(w[2].c_str())) * 3.14159265f / 180.0f;
            char a = static_cast<char>(std::toupper(static_cast<unsigned char>(w[1][0])));
            if (a == 'Z') c.rot.z = v;
            else if (a == 'X') c.rot.x = v;
            else if (a == 'Y') c.rot.y = v;
            else return Bad(line);
            return 1;
        }
        if (Ieq(cmd, "PlayGroup")) {
            if (w.size() != 3 || !IsInt(w[2])) return Bad(line);
            int id = -1;
            for (int g = 0; g < 245; ++g) {   // every group the layer can name (the locomotion table and the action groups)
                const char* n = GroupName(g);
                if (n && Ieq(w[1], n)) id = g;
            }
            if (id < 0) return Bad(line);
            if (needs3D) return 0;
            if (attackNeedsWeaponOut && id >= 26 && id <= 167 && !WeaponOut(c)) return 1;   // accepted, nothing plays
            c.group = id;
            return 1;
        }
        if (Ieq(cmd, "SetRestrained") || Ieq(cmd, "SetGhost") || Ieq(cmd, "SetIgnoreFriendlyHits") || Ieq(cmd, "SetForceSneak") ||
            Ieq(cmd, "SetAlert")) {
            if (w.size() != 2 || (w[1] != "0" && w[1] != "1")) return Bad(line);
            if (needs3D) return 0;
            bool on = w[1] == "1";
            if (Ieq(cmd, "SetRestrained")) c.restrained = on;
            else if (Ieq(cmd, "SetGhost")) c.ghost = on;
            else if (Ieq(cmd, "SetIgnoreFriendlyHits")) c.ignoreFriendly = on;
            else if (Ieq(cmd, "SetForceSneak")) c.sneak = on;
            else c.alert = on;
            return 1;
        }
        if (Ieq(cmd, "AddItem") || Ieq(cmd, "RemoveItem")) {
            if (w.size() != 4 || !IsHex8(w[1]) || !IsInt(w[2]) || (w[3] != "0" && w[3] != "1")) return Bad(line);
            FormId it = static_cast<FormId>(std::strtoul(w[1].c_str(), nullptr, 16));
            int n = std::atoi(w[2].c_str());
            if (n <= 0) return Bad(line);
            if (Ieq(cmd, "AddItem")) {
                c.inv[it] += n;
            } else {
                int& have = c.inv[it];
                have = std::max(0, have - n);
                if (have == 0) c.worn.erase(it);
            }
            return 1;
        }
        if (Ieq(cmd, "EquipItem") || Ieq(cmd, "UnequipItem")) {
            if (w.size() != 4 || !IsHex8(w[1]) || (w[2] != "0" && w[2] != "1") || (w[3] != "0" && w[3] != "1")) return Bad(line);
            if (needs3D) return 0;
            FormId it = static_cast<FormId>(std::strtoul(w[1].c_str(), nullptr, 16));
            if (Ieq(cmd, "EquipItem")) {
                if (equipRequiresOwned && c.inv[it] <= 0) return 0;
                auto s = slotOf.find(it);
                if (s != slotOf.end())
                    for (auto itw = c.worn.begin(); itw != c.worn.end();) {
                        auto s2 = slotOf.find(*itw);
                        if (s2 != slotOf.end() && s2->second == s->second) itw = c.worn.erase(itw);
                        else ++itw;
                    }
                c.worn.insert(it);
            } else {
                c.worn.erase(it);
            }
            return 1;
        }
        if (Ieq(cmd, "ResetHealth")) {
            if (w.size() != 1) return Bad(line);
            if (needs3D) return 0;
            if (!c.dead) c.health = 100;
            return 1;
        }
        if (Ieq(cmd, "SetScale")) {
            if (w.size() != 2 || !IsNum(w[1])) return Bad(line);
            if (needs3D) return 0;
            c.scale = static_cast<float>(std::atof(w[1].c_str()));
            return 1;
        }
        if (Ieq(cmd, "EnableWeaponMod")) {
            if (w.size() != 3 || !IsHex8(w[1]) || !IsInt(w[2])) return Bad(line);
            if (needs3D) return 0;
            FormId wp = static_cast<FormId>(std::strtoul(w[1].c_str(), nullptr, 16));
            int slot = std::atoi(w[2].c_str());
            if (!enableModWorks || slot < 1 || slot > 3 || !c.worn.count(wp) || !weaponItems.count(wp)) return 0;
            c.mods[wp] = static_cast<uint8_t>(c.mods[wp] | (1u << (slot - 1)));
            return 1;
        }
        if (Ieq(cmd, "Activate")) {   // furniture.Activate <actor> 1: the actor uses the furniture (or, if it is in it already, gets out)
            if (w.size() != 3 || !IsHex8(w[1]) || w[2] != "1") return Bad(line);
            if (std::find(furniture.begin(), furniture.end(), c.id) == furniture.end()) return Bad(line);   // the line only makes sense on a furniture
            FRef* a = Get(static_cast<RefId>(std::strtoul(w[1].c_str(), nullptr, 16)));
            if (!a || !a->exists || !a->isActor || !a->has3D()) return 0;
            if (activateSeats) {
                if (a->sitSleep == 0) {
                    a->sitSleep = 4;
                    a->usedFurniture = c.id;
                } else {
                    a->sitSleep = 0;
                    a->usedFurniture = 0;
                }
            }
            return 1;
        }
        if (Ieq(cmd, "EvaluatePackage")) {
            if (w.size() != 1) return Bad(line);
            if (needs3D) return 0;
            c.sitSleep = 0;   // the AI lets it out of the furniture
            return 1;
        }
        if (Ieq(cmd, "MoveToMarker")) {
            if (w.size() != 5 || !IsHex8(w[1]) || !IsNum(w[2]) || !IsNum(w[3]) || !IsNum(w[4])) return Bad(line);
            RefId target = static_cast<RefId>(std::strtoul(w[1].c_str(), nullptr, 16));
            FRef* t = Get(target);
            if (!t || !t->exists) return 0;
            c.cell = t->cell;
            c.pos = t->pos;
            c.node = 0;
            c.load3dLeft = load3DFrames;
            if (restraintLostOnRebuild) c.restrained = false;
            c.group = -1;
            return 1;
        }
        return Bad(line);
    }

    std::map<RefId, FRef> refs_;
    std::map<CellId, CellState> cells_;
    uint32_t nextId_ = 0, nextNode_ = 0x2000;
    std::vector<RefId> freeIds_;
    FRef none_;
};

}  // namespace nvcoop::avatar::engine
