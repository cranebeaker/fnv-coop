// Everything the puppet driver needs from the game, behind one interface. Production implementation:
// ScriptPuppetEngine (script_engine.h, script-command text over an IScriptRunner plus IEngineReads); tests: FakePuppetEngine
// (testing/fake_engine.h). All calls happen on the game main thread. Write calls return true on success; a false return
// is an ordinary event (the engine refused: no 3D yet, restrained actor ignoring the command, ref gone) that the driver
// counts and retries with backoff, never an exception.
#pragma once
#include <vector>

#include "avatar_types.h"

namespace nvcoop::avatar {

class IPuppetEngine {
public:
    virtual ~IPuppetEngine() = default;

    // ---- world state --------------------------------------------------------------------------------------------
    virtual bool WorldReady() = 0;        // a game is loaded and the player ref exists (false in menus / while loading)
    virtual uint32_t WorldEpoch() = 0;    // changes on every load, new game and return to the main menu
    virtual bool MenuOpen() = 0;          // a menu that stops the world or resets animation (Pip-Boy, dialogue, container, loading)
    virtual bool CellLoaded(CellId cell) = 0;
    // True when both cells are exterior cells of one worldspace, so SetPos alone moves a ref between them.
    virtual bool SameSpace(CellId a, CellId b) = 0;

    // ---- life ---------------------------------------------------------------------------------------------------
    // Creates a puppet from `templateBase` at the given place. The implementation MUST tag the ref so FindTaggedPuppets
    // returns it after a load (dedicated base form, or an extra-data name). 0 = failed.
    virtual RefId Spawn(FormId templateBase, CellId cell, const Vec3& pos, float yawRad) = 0;
    virtual void Delete(RefId ref) = 0;   // disable + mark for delete; safe on an invalid ref
    virtual bool IsValid(RefId ref) = 0;  // the same object we spawned still exists (ids can be reused after a delete)
    virtual bool Has3D(RefId ref) = 0;
    virtual uint32_t Generation3D(RefId ref) = 0;   // changes when the 3D is rebuilt (cell reload); 0 = unknown
    virtual std::vector<RefId> FindTaggedPuppets() = 0;   // every loaded puppet, including orphans from an old save

    // ---- configuration (once after Has3D) -----------------------------------------------------------------------
    virtual bool SetRestrained(RefId ref, bool on) = 0;
    virtual bool IsRestrained(RefId ref) = 0;
    // Friendly to the players (their hits on it are ignored), no AI package. GamePuppetEngine leaves it HITTABLE by enemies on purpose: the
    // combat layer needs the engine's hit event on a puppet to forward the damage to the remote player it stands for (ScriptPuppetEngine
    // still makes it a ghost).
    virtual bool MakeNonCombatant(RefId ref) = 0;
    virtual bool ApplyAppearance(RefId ref, const Appearance& a) = 0;
    // The base the puppet of `slot` is spawned from: `templateBase` itself, or the engine's private copy of it with the look `a` written in
    // (sex, race, hair, face). With a copy, a look change needs a new puppet (the game builds the head and body from the base when it loads
    // the 3D), so the manager respawns it.
    virtual FormId LookBase(Slot /*slot*/, FormId templateBase, const Appearance& /*a*/) { return templateBase; }
    virtual bool LooksByBase() { return false; }
    // One line about what the last LookBase(slot) did ("look copy 0xFF000123 ..." / "template, ..."), for the log. Empty = nothing to say.
    virtual std::string LookReport(Slot /*slot*/) { return std::string(); }

    // ---- driving (every Tick) -----------------------------------------------------------------------------------
    virtual bool SetPos(RefId ref, const Vec3& pos) = 0;
    virtual bool SetYaw(RefId ref, float yawRad) = 0;
    virtual bool MoveToCell(RefId ref, CellId cell, const Vec3& pos, float yawRad) = 0;   // cell change; 3D is rebuilt
    // The ref's real place (physics or AI may have moved it). False when unknown.
    virtual bool GetPose(RefId ref, CellId& cell, Vec3& pos) = 0;
    // Where THIS machine's own player stands (cell and place). False when unknown (default: the driver then makes no room for it).
    virtual bool GetLocalPose(CellId& /*cell*/, Vec3& /*pos*/) { return false; }
    // flags bit 0: loop / hold the group (script PlayGroup flag 1).
    virtual bool PlayGroup(RefId ref, int baseGroup, int flags) = 0;
    virtual int CurrentGroup(RefId ref) = 0;       // -1 unknown (then only the heartbeat re-asserts)
    virtual bool SetSneak(RefId ref, bool on) = 0;
    virtual bool SetAlert(RefId ref, bool on) = 0;
    virtual bool SetWeaponDrawn(RefId ref, bool on) = 0;   // false = unsupported / refused
    virtual bool IsWeaponDrawn(RefId ref) = 0;

    // ---- furniture (default: unsupported, so older engines and fakes need no change) --------------------------------------
    // The puppet uses that furniture reference (a chair, a bench, a bed on this machine): the engine's own sit-down. False = unsupported / refused (an ordinary event).
    virtual bool SitIn(RefId /*ref*/, FormId /*furniture*/) { return false; }
    // Leave the furniture. attempt 0 is the gentle way (use the furniture again), higher ones are escalations that may let the actor's AI run (the restraint check puts the hold back).
    virtual bool StandUp(RefId /*ref*/, FormId /*furniture*/, unsigned /*attempt*/) { return false; }
    // The engine's sit / sleep state of the puppet, the xNVSE BaseProcess::kSitSleepState_* number (0 = not in furniture). -1 = cannot tell.
    virtual int SitState(RefId /*ref*/) { return -1; }

    // ---- the look the engine really built (default: unknown) -----------------------------------------------------------------
    // The look of the puppet's own base as the game has it now (race, sex, hair, eyes, body, face blocks, head parts), for the check after a spawn.
    virtual bool ReadLook(RefId /*ref*/, Appearance& /*out*/) { return false; }

    // ---- equipment ----------------------------------------------------------------------------------------------
    virtual bool GetEquipped(RefId ref, std::vector<FormId>& out) = 0;   // false = unknown
    virtual int ItemCount(RefId ref, FormId item) = 0;
    virtual bool AddItem(RefId ref, FormId item, int count) = 0;
    virtual bool RemoveItem(RefId ref, FormId item, int count) = 0;
    // Equip must set the engine's "prevent unequip" flag so the actor's own AI does not strip it again.
    virtual bool EquipItem(RefId ref, FormId item) = 0;
    virtual bool UnequipItem(RefId ref, FormId item) = 0;

    // ---- hit target (default: unsupported) ---------------------------------------------------------------------------------
    virtual bool IsDead(RefId /*ref*/) { return false; }       // the actor died (a stray enemy hit): the driver respawns it
    virtual bool ResetHealth(RefId /*ref*/) { return false; }  // back to full health (the puppet only stands in for a player; its health is not the player's)

    // ---- weapon modifications (default: unsupported, so older engines and fakes need no change) -------------------------
    // The mod flags (EquipEntry::mods) of the weapon `weapon` as the puppet really wears it. -1 = unknown (not worn, unreadable).
    virtual int WeaponMods(RefId /*ref*/, FormId /*weapon*/) { return -1; }
    // Asks the engine to put modification slot 1..3 on the puppet's weapon. false = refused / unsupported (an ordinary event).
    virtual bool EnableWeaponMod(RefId /*ref*/, FormId /*weapon*/, int /*slot*/) { return false; }
};

}  // namespace nvcoop::avatar
