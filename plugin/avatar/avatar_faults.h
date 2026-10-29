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
    bool adoptPhysicsPush = false;       // ActualPose reports the pushed place, so the buffer eases the puppet back from it (teleport out and back)
    bool noBodyRoom = false;             // a puppet is put inside the local player's body (the engine shoves the real player)
    // the owner's one-shots (attack, block, stagger ...; AnimState::action / actionSeq)
    bool noOneShots = false;             // the owner's attacks and reactions are never played on the puppet
    bool oneShotReplays = false;         // the counter is ignored: a one-shot still in the stream is played again on every Tick
    bool oneShotNoBaseline = false;      // the first look at the counter is played (an old attack replays when a puppet appears)
    bool oneShotKeepsBaseline = false;   // the counter is not re-read when the puppet (re)enters Driving (an attack that went by meanwhile is played late)
    bool oneShotNoWrap = false;          // the counter does not wrap: 255 -> 0 reads as older
    bool oneShotAdoptsStale = false;     // an older counter (a duplicate, a reordered snapshot) is taken as the new baseline: the newest one then plays again
    bool oneShotNoResync = false;        // an owner whose counter restarted below ours is ignored for good
    bool oneShotNextOnly = false;        // only seq + 1 is played: a lost datagram makes the newest attack vanish
    bool oneShotWhileFrozen = false;     // a dry stream still plays its last one-shot
    bool noOneShotReassert = false;      // the movement group is not asserted again after a one-shot (the puppet stays in the stagger pose)
    bool noOneShotMismatchGuard = false; // the readback re-assert runs while a one-shot plays (it cuts the swing off)
    bool reloadGroupFixed = false;       // reload always plays the default group, not the one the owner's weapon uses
    bool heldNeverEnds = false;          // a held action (block, automatic fire) is not ended when the owner stops reporting it
    bool bridgeDropsAction = false;      // the live <-> avatar conversion forgets action / actionSeq / block
    // the pose override (a downed friend's puppet keeps lying; PuppetManager::SetPoseOverride)
    bool poseOverrideIgnored = false;    // the override is stored but the planned group still plays
    bool poseReplaysPlanned = false;     // the periodic replay (heartbeat / readback mismatch) plays the planned group, not the override
    bool poseHeartbeats = false;         // the held pose is re-issued every heartbeat (a lying puppet gets up and falls again)
    bool poseNoDeathGuard = false;       // a puppet the engine calls dead is respawned although it is overridden
    bool poseOneShotsPlay = false;       // the owner's attacks / reactions play on the lying puppet (it pops up)
    bool poseClearIgnored = false;       // clearing the override does nothing: the puppet stays down
    bool poseLostOnRespawn = false;      // a new puppet for the slot (look change, reconnect) forgets the override
    bool poseSurvivesRemove = false;     // a player who left and came back is still down
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
