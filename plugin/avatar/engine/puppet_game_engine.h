// GamePuppetEngine: the real IPuppetEngine. Game-free policy on top of IGameBackend (game_backend.h).
//
// What it adds over ScriptPuppetEngine (script_engine.h, which only formats script text):
//   * a guard in front of EVERY write: only an actor that carries one of our template base forms (and is not the player) is ever
//     touched, so a reused ref id, a stale ref or a bug upstream can never move, equip or delete a foreign actor;
//   * spawn that finds the new reference itself (a console PlaceAtMe cannot return it): before / after diff of the anchor cell's
//     actors, anchored on the local player when the target cell is the player's cell, else on a static in that cell;
//   * SetPos / SetAngle compare with where the reference REALLY is and send only the axes that differ (one console line per axis),
//     so a standing puppet costs nothing and a walking one costs two lines per frame;
//   * 3D tracking (Has3D, Generation3D from the scene object's identity), `CurrentGroup` from the running animation sequences
//     (a group counts as playing when ANY slot holds it, so the slot layout does not have to be known), weapon draw through
//     SetAlert, restraint read-back through the engine's own GetRestrained evaluator;
//   * cost accounting for every console line (p50 / p95 / max microseconds) so "what does a script line cost" is measured,
//     not guessed (docs/tools/avatar.md TODO-RE).
//
// Console text is pinned by tests (the same lines as ScriptPuppetEngine, tests/test_engine.cpp): commands come from the vanilla
// command table (plugin/hooks/tools/re/command_table.json); nothing here is an invented address.
#pragma once
#include <cstdint>
#include <map>
#include <string>
#include <vector>

#include "../puppet_engine.h"
#include "game_backend.h"

namespace nvcoop::avatar::engine {

struct EngineOptions {
    std::vector<FormId> taggedBases;   // template NPC base forms: a reference with one of these bases is a puppet
    float axisEpsilon = 0.25f;         // units: an axis closer than this to the target is not re-sent
    float yawEpsilonRad = 0.002f;      // heading closer than this is not re-sent
    bool ghostPuppets = false;         // SetGhost on puppets: immune to everything, but nothing can hit them either (see MakeNonCombatant)
    size_t costWindow = 256;           // console line costs kept for the percentiles
    unsigned maxSpawnsPerWindow = 8;   // circuit breaker: at most this many spawns ...
    uint64_t spawnWindowUs = 10000000; // ... in this window; a driver that keeps losing its refs must not flood the world with actors
};

struct EngineStats {
    uint64_t lines = 0, linesRefused = 0, linesException = 0, linesRefGone = 0;
    uint64_t evalFailures = 0;             // an evaluator was unavailable or faulted
    uint64_t refused3D = 0;                // a write was not sent because the ref has no 3D yet
    uint64_t foreignRefused = 0;           // a write / delete was refused because the ref is not one of ours
    uint64_t spawnCalls = 0, spawnNoAnchor = 0, spawnRefused = 0, spawnNoNewRef = 0, spawnAmbiguous = 0;
    uint64_t spawnThrottled = 0;           // refused by the circuit breaker
    uint64_t spawnRejected = 0;            // the new ref failed our own guard and was deleted again
    uint64_t setPosCalls = 0, setPosSkipped = 0, setYawCalls = 0, setYawSkipped = 0;
    uint64_t appearanceApproximate = 0;    // the face / hair of the template was kept (no engine path to change it)
    uint64_t scaleApplied = 0;             // SetScale sent to match the owner's height
    uint64_t lookCopies = 0, lookWrites = 0, lookFaces = 0, lookFailures = 0;   // LookBase: copies made, looks written, of them with the face morphs, refusals
    uint64_t lookReused = 0;               // LookBase: a respawn (after a save, a park) whose look had not changed took the copy it already had
    uint64_t weaponOutDirect = 0, weaponOutFallbacks = 0;   // BaseProcess::SetWeaponOut calls that took; of them, the flag did not read back and the alert state was raised as well
    uint64_t sitCalls = 0, standCalls = 0;  // furniture commands sent
    uint64_t modLines = 0, modReads = 0;   // EnableWeaponMod lines sent / weapon-mod reads that knew the answer
    uint64_t drift3D = 0;                  // the scene object was more than 20 units from the reference position
    float maxDrift3D = 0;
    uint32_t costP50Us = 0, costP95Us = 0, costMaxUs = 0;
};

class GamePuppetEngine : public IPuppetEngine {
public:
    GamePuppetEngine(IGameBackend& be, EngineOptions opt = {});

    void SetTaggedBases(std::vector<FormId> bases) { opt_.taggedBases = std::move(bases); }
    const EngineOptions& options() const { return opt_; }
    EngineStats stats() const;                 // includes the cost percentiles
    std::string StatsJson() const;             // {"lines":..,"costP95Us":..,...} for avatar.status

    // ---- IPuppetEngine ----------------------------------------------------------------------------------------------
    bool WorldReady() override;
    uint32_t WorldEpoch() override;
    bool MenuOpen() override;
    bool CellLoaded(CellId cell) override;
    bool SameSpace(CellId a, CellId b) override;

    RefId Spawn(FormId templateBase, CellId cell, const Vec3& pos, float yawRad) override;
    void Delete(RefId ref) override;
    bool IsValid(RefId ref) override;
    bool Has3D(RefId ref) override;
    uint32_t Generation3D(RefId ref) override;
    std::vector<RefId> FindTaggedPuppets() override;

    bool SetRestrained(RefId ref, bool on) override;
    bool IsRestrained(RefId ref) override;
    bool MakeNonCombatant(RefId ref) override;
    bool ApplyAppearance(RefId ref, const Appearance& a) override;
    FormId LookBase(Slot slot, FormId templateBase, const Appearance& a) override;
    bool LooksByBase() override { return looksByBase_ && be_.CanCloneNpcBase(); }
    std::string LookReport(Slot slot) override {
        auto it = lookNotes_.find(slot);
        return it == lookNotes_.end() ? std::string() : it->second;
    }

    bool SetPos(RefId ref, const Vec3& pos) override;
    bool SetYaw(RefId ref, float yawRad) override;
    bool MoveToCell(RefId ref, CellId cell, const Vec3& pos, float yawRad) override;
    bool GetPose(RefId ref, CellId& cell, Vec3& pos) override;
    bool GetLocalPose(CellId& cell, Vec3& pos) override;
    bool PlayGroup(RefId ref, int baseGroup, int flags) override;
    int CurrentGroup(RefId ref) override;
    bool SetSneak(RefId ref, bool on) override;
    bool SetAlert(RefId ref, bool on) override;
    bool SetWeaponDrawn(RefId ref, bool on) override;
    bool IsWeaponDrawn(RefId ref) override;
    bool SitIn(RefId ref, FormId furniture) override;
    bool StandUp(RefId ref, FormId furniture, unsigned attempt) override;
    int SitState(RefId ref) override;
    bool ReadLook(RefId ref, Appearance& out) override;

    bool GetEquipped(RefId ref, std::vector<FormId>& out) override;
    int ItemCount(RefId ref, FormId item) override;
    bool AddItem(RefId ref, FormId item, int count) override;
    bool RemoveItem(RefId ref, FormId item, int count) override;
    bool EquipItem(RefId ref, FormId item) override;
    bool UnequipItem(RefId ref, FormId item) override;
    bool IsDead(RefId ref) override;
    bool ResetHealth(RefId ref) override;
    int WeaponMods(RefId ref, FormId weapon) override;
    bool EnableWeaponMod(RefId ref, FormId weapon, int slot) override;

    // Extra introspection for the bridge / tests.
    bool IsTaggedBase(FormId base) const;
    bool IsCreatedRef(RefId ref) const;   // 0xFFxxxxxx: made by PlaceAtMe, not placed in a plugin
    size_t OwnedCount() const { return owned_.size(); }
    bool Owns(RefId ref) const { return owned_.count(ref) != 0; }

private:
    struct RefState {
        FormId base = 0;
        uint32_t node = 0;
        uint32_t gen = 0;
        bool lost3D = false;
        int lastGroup = -1;
        bool restrainedKnown = false, restrained = false;   // what we last asked for (fallback when the evaluator is unavailable)
        std::map<FormId, int> added;                        // items we added (fallback for ItemCount when the evaluator is unavailable)
    };

    bool SendAxes(RefId ref, const Vec3& actual, const Vec3& target);
    bool Place(RefId ref, const Vec3& pos, float yawRad);   // best effort right after PlaceAtMe (the ref may not have 3D yet)

    bool Run(RefId ctx, const std::string& line);
    // The reference must exist, be an actor, not the player, not deleted, and carry a tagged base (and the base we recorded for it).
    bool Ours(RefId ref, RefInfo& ri, bool count = true);
    bool OursWith3D(RefId ref, RefInfo& ri);
    void Observe3D(RefId ref, const RefInfo& ri);
    RefState& StateOf(RefId ref, const RefInfo& ri);
    static std::string Hex(FormId f);

    IGameBackend& be_;
    EngineOptions opt_;
    mutable EngineStats st_;
    std::map<RefId, RefState> state_;
    std::map<RefId, FormId> owned_;   // spawned this session and not deleted yet: ref -> base
    std::vector<uint64_t> spawnTimes_;   // NowUs of the recent spawns (circuit breaker)
    struct LookCopy { FormId copy = 0, from = 0; uint32_t epoch = 0; Appearance written; };
    std::map<Slot, LookCopy> lookCopies_;   // per remote player: the private copy of its template with its look written in (LookBase)
    bool looksByBase_ = true;               // false once the game refused to make a copy three times in a row (then every puppet is the template, as before)
    unsigned cloneFailStreak_ = 0;
    std::map<Slot, std::string> lookNotes_; // LookReport
    std::vector<uint32_t> costs_;     // ring of the last console line costs (us)
    size_t costNext_ = 0;
    uint32_t costMax_ = 0;
};

}  // namespace nvcoop::avatar::engine
