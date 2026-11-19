// Test-only fault switches (mutation twins, switchable at run time, same pattern as synccore/live LiveFaults).
// Every flag deliberately breaks ONE puppet trap handler; tests/test_avatar.cpp runs the whole suite once per flag
// and requires at least one failing check each time. All flags are false in production and are never set outside tests.
#pragma once

namespace nvcoop::avatar {

struct AvatarFaults {
    bool noRestrainReassert = false;     // a lost restraint is never noticed (AI takes the puppet over)
    bool noMenuReassert = false;         // animation is not replayed after a menu closed
    bool noWait3D = false;               // commands are sent before the puppet has 3D (engine refuses them)
    bool trustLocoCode = false;          // owner's locomotion code is trusted even when velocity disagrees (slides)
    bool no3DGenReassert = false;        // animation / pose are not re-sent after the 3D was rebuilt
    bool saveKeepsPuppets = false;       // puppets are not despawned when the game saves (they get saved)
    bool noLoadPurge = false;            // orphan puppets are not purged after a load
    bool usesPitchAsYaw = false;         // the pitch angle is applied as the heading (tilted / spinning body)
    bool noEquipAddFirst = false;        // EquipItem without making sure the puppet owns the item
    bool noEquipBackoff = false;         // failed equipment writes are retried every interval forever
    bool noEquipFlapGuard = false;       // equip/unequip fights with the engine are never broken
    bool weaponDrawBeforeEquip = false;  // weapon is drawn before it is equipped
    bool allowLocalSlot = false;         // the local player gets a puppet of itself
    bool duplicateSlotSpawns = false;    // a second AddPlayer for a slot spawns a second puppet
    bool noStaleRefCheck = false;        // a deleted / reused ref id is still driven
    bool neverPark = false;              // puppet stays when the remote's cell is not loaded (floats in the void)
    bool crossCellSetPos = false;        // cell change done with SetPos instead of MoveToCell
    bool frozenKeepsRunning = false;     // a dry interpolation buffer keeps the run animation going (skating)
    bool noYawNormalize = false;         // yaw is not wrapped to (-pi, pi]
    bool noAnimHeartbeat = false;        // the current group is never re-issued
    bool noJumpLand = false;             // jump never ends in the landing group
    bool ignoreBudget = false;           // per-Tick engine call budget is ignored
    bool staleMapEntries = false;        // slot -> ref map keeps entries for despawned puppets
    bool noPostMenuSnap = false;         // after a menu the puppet is not moved to the latest pose at once
    bool noSaveWait = false;             // puppets respawn while the save is still being written
    bool noWeaponMods = false;           // weapon modifications are never put on the puppet's weapon
    bool modsIgnoredInSet = false;       // a changed mod set alone does not count as a changed wanted set
    bool modsRetryForever = false;       // a refused modification is retried every interval without a limit
    bool apiLocalGetsPuppet = false;     // puppetRefOf(local slot) answers a puppet (the local player is treated like a remote one)
    bool apiPlayerIsPuppet = false;      // slotOfPuppet(0x14) answers the local slot (the real player counts as a puppet)
    bool apiActingNeedsPuppet = false;   // actingRefOf(local slot) answers 0 instead of the player's own reference
    bool hitsForwardEmpty = false;       // a hit without damage on a puppet is forwarded to the player as well
    bool puppetHitProceeds = false;      // the local puppet takes the damage itself (can die of a prediction)
    bool noPuppetHeal = false;           // a damaged puppet is never topped up (it dies from a few stray hits)
    bool noPuppetDeathRespawn = false;   // a puppet that died stays in the world as a corpse
    bool appearanceUpdateRespawns = false;   // a look change that keeps race and sex still deletes and respawns the puppet
    bool noFailedRetry = false;          // a slot that gave up stays Failed for the whole session
    bool adoptPhysicsPush = false;       // ActualPose reports the pushed place, so the buffer eases the puppet back from it (teleport out and back)
};

inline AvatarFaults& avatarFaults() {
    static AvatarFaults f;
    return f;
}

struct AvatarFaultScope {
    AvatarFaults saved;
    explicit AvatarFaultScope(const AvatarFaults& f) : saved(avatarFaults()) { avatarFaults() = f; }
    ~AvatarFaultScope() { avatarFaults() = saved; }
};

}  // namespace nvcoop::avatar
