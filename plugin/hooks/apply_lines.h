// Console commands used to apply effects that have no single verified engine primitive. The command is run with the
// target reference as the console's CALLING REFERENCE (NVSEConsoleInterface::RunScriptLine2(line, thisObj)); the console
// does not accept `<hexFormID>.Command`, and `prid` does not survive across separate RunScriptLine2 calls.
// Names / parameter orders come from the engine command table (plugin/hooks/tools/re/command_table.json):
//   KillActor (short "kill")        params: [Actor opt] [int opt] [int opt]
//   ResurrectActor (short "resurrect") params: [int opt: reset inventory]
//   MoveToMarker (short "MoveTo")    params: ObjectReferenceID, [float x] [float y] [float z]
// Game-free so the exact text is pinned by unit tests.
#pragma once
#include <cstdint>
#include <set>
#include <string>

#include "fnvcoop/types.h"

namespace nvhooks {

std::string killCmd();
std::string enableFastTravelCmd(bool enable);
// Party-wide fast-travel lock bookkeeping for SetFastTravelLock{scope,on}: travel is enabled only while NO scope holds a lock.
class FastTravelLocks {
public:
    // Applies one lock/unlock; returns true if the effective travel state changed (then run enableFastTravelCmd(travelEnabled())).
    bool set(const fnvcoop::FormRef& scope, bool on) {
        bool before = travelEnabled();
        if (on) locks_.insert(scope); else locks_.erase(scope);
        return before != travelEnabled();
    }
    bool travelEnabled() const { return locks_.empty(); }
    size_t size() const { return locks_.size(); }

private:
    std::set<fnvcoop::FormRef> locks_;
};
std::string setTeammateCmd(bool on);           // run ON the actor
// CompanionOwner is applied only on the owner's machine (the engine's follower AI follows the LOCAL player); others see the NPC move.
bool companionForMe(uint8_t owner, uint8_t localSlot);
std::string resurrectCmd(bool resetInventory);
std::string moveToMarkerCmd(uint32_t markerRefID, float ox, float oy, float oz);

// Journal / world / faction / perk / lock commands (hex FormID arguments are accepted by the console parser).
std::string setFactionRankCmd(uint32_t factionRefID, int rank);
std::string removeFromFactionCmd(uint32_t factionRefID);
std::string addPerkCmd(uint32_t perkRefID);
std::string removePerkCmd(uint32_t perkRefID);
std::string lockCmd(int level);          // level < 0: "Lock" without argument
std::string unlockCmd();
std::string setOpenStateCmd(bool open);  // SetOpenState 1 = open, 0 = closed
std::string setDestroyedCmd(bool destroyed);
std::string addNoteCmd(uint32_t noteRefID);
std::string removeNoteCmd(uint32_t noteRefID);
std::string addTopicCmd(uint32_t topicRefID);
std::string setWeatherCmd(uint32_t weatherRefID, bool force);
std::string setTimeScaleCmd(float scale);
std::string showMapCmd(uint32_t markerRefID, bool travel);
std::string addItemCmd(uint32_t itemRefID, int count);     // run ON the container
std::string removeItemCmd(uint32_t itemRefID, int count);
std::string removeAllItemsCmd();
// ContainerTransfer, run ON the source (0x14 = this machine's player): into `targetRefID`, into the player, or a plain clear (target 0).
std::string removeAllItemsToCmd(uint32_t targetRefID, bool toPlayer);
// Scripted saves: kind 0 autosave, 1 systemsave, 2 forcesave.
std::string saveCmd(unsigned kind);

// SetPos axis value (params: Axis, Float): same-cell absolute move, one command per axis.
std::string setPosCmd(char axis /* 'X' 'Y' 'Z' */, float value);
// SetAngle axis degrees (params: Axis, Float): rotation of one axis in degrees, run ON the reference.
std::string setAngleCmd(char axis /* 'X' 'Y' 'Z' */, float degrees);
// The Axis parameter as ExtractArgs hands it back ('X' 'Y' 'Z', either case) -> 0 / 1 / 2, or -1 when it is none of them.
int axisIndexOf(unsigned axisChar);
// Form types that can be a console calling reference / MoveTo marker: REFR 0x3A, ACHR 0x3B, ACRE 0x3C.
bool isReferenceFormType(unsigned typeId);

}  // namespace nvhooks
