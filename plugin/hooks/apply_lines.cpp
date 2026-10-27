#include "apply_lines.h"

#include <cstdio>

namespace nvhooks {

#ifdef NVHOOKS_MUTANT
constexpr const char* kKill = "Kill";               // wrong spelling of the long name
constexpr const char* kMove = "MoveToQuestTarget";  // wrong command
#else
constexpr const char* kKill = "KillActor";
constexpr const char* kMove = "MoveTo";
#endif

std::string killCmd() { return kKill; }
std::string resurrectCmd(bool reset) { return std::string("ResurrectActor ") + (reset ? "1" : "0"); }
std::string moveToMarkerCmd(uint32_t marker, float x, float y, float z) {
    char b[120];
    std::snprintf(b, sizeof b, "%s %08X %.4f %.4f %.4f", kMove, marker, x, y, z);
    return b;
}

static std::string fmt1(const char* f, uint32_t a) {
    char b[80];
    std::snprintf(b, sizeof b, f, a);
    return b;
}
std::string setFactionRankCmd(uint32_t f, int rank) {
    char b[80];
    std::snprintf(b, sizeof b, "SetFactionRank %08X %d", f, rank);
    return b;
}
std::string removeFromFactionCmd(uint32_t f) { return fmt1("RemoveFromFaction %08X", f); }
#ifdef NVHOOKS_MUTANT
std::string addPerkCmd(uint32_t p) { return fmt1("AddPerks %08X", p); }
#else
std::string addPerkCmd(uint32_t p) { return fmt1("AddPerk %08X", p); }
#endif
std::string removePerkCmd(uint32_t p) { return fmt1("RemovePerk %08X", p); }
std::string lockCmd(int level) { return level < 0 ? std::string("Lock") : std::string("Lock ") + std::to_string(level); }
std::string unlockCmd() { return "UnLock"; }
std::string setOpenStateCmd(bool open) { return open ? "SetOpenState 1" : "SetOpenState 0"; }
std::string setDestroyedCmd(bool d) { return d ? "SetDestroyed 1" : "SetDestroyed 0"; }
std::string addNoteCmd(uint32_t n) { return fmt1("AddNote %08X", n); }
std::string removeNoteCmd(uint32_t n) { return fmt1("RemoveNote %08X", n); }
std::string addTopicCmd(uint32_t t) { return fmt1("AddTopic %08X", t); }
std::string setWeatherCmd(uint32_t w, bool force) { return fmt1(force ? "ForceWeather %08X" : "SetWeather %08X", w); }
std::string setTimeScaleCmd(float s) {
    char b[64];
    std::snprintf(b, sizeof b, "SetGlobalTimeMultiplier %.4f", s);
    return b;
}
std::string enableFastTravelCmd(bool enable) { return enable ? "EnableFastTravel 1" : "EnableFastTravel 0"; }
std::string setTeammateCmd(bool on) { return on ? "SetPlayerTeammate 1" : "SetPlayerTeammate 0"; }
bool companionForMe(uint8_t owner, uint8_t localSlot) { return owner == localSlot; }
std::string showMapCmd(uint32_t m, bool travel) {
    char b[64];
    std::snprintf(b, sizeof b, travel ? "ShowMap %08X 1" : "ShowMap %08X", m);
    return b;
}
std::string addItemCmd(uint32_t i, int c) {
    char b[80];
    std::snprintf(b, sizeof b, "AddItem %08X %d", i, c);
    return b;
}
std::string removeItemCmd(uint32_t i, int c) {
    char b[80];
    std::snprintf(b, sizeof b, "RemoveItem %08X %d", i, c);
    return b;
}
std::string removeAllItemsCmd() { return "RemoveAllItems"; }
std::string removeAllItemsToCmd(uint32_t target, bool toPlayer) {
    if (toPlayer) return "RemoveAllItems player";
    return target ? fmt1("RemoveAllItems %08X", target) : removeAllItemsCmd();
}
std::string saveCmd(unsigned kind) { return kind == 1 ? "SystemSave" : kind == 2 ? "ForceSave" : "Autosave"; }

std::string setPosCmd(char axis, float value) {
    char b[48];
    std::snprintf(b, sizeof b, "SetPos %c %.4f", axis, value);
    return b;
}

std::string setAngleCmd(char axis, float degrees) {
    char b[48];
    std::snprintf(b, sizeof b, "SetAngle %c %.4f", axis, degrees);
    return b;
}

int axisIndexOf(unsigned c) {
#ifdef NVHOOKS_MUTANT
    return c == 'X' ? 0 : c == 'Y' ? 1 : c == 'Z' ? 2 : -1;  // forgets lower case
#else
    switch (c) {
        case 'X': case 'x': return 0;
        case 'Y': case 'y': return 1;
        case 'Z': case 'z': return 2;
        default: return -1;
    }
#endif
}

bool isReferenceFormType(unsigned t) {
#ifdef NVHOOKS_MUTANT
    return t == 0x3A;  // forgets that the PLAYER and every NPC/creature reference are ACHR/ACRE, not REFR
#else
    return t == 0x3A || t == 0x3B || t == 0x3C;
#endif
}

}  // namespace nvhooks
