// Only the host loads saves (host policy [coop] clientload=block, the default). A joined guest that loads its own save would rewind its
// character against the shared world, so the game refuses it: the save manager's load (0x850760, BGSSaveLoadGame, thiscall, ret 0x10) is
// detoured in coop_module.cpp and asks guestLoadAllowed() first. The pause menu's Load, Continue, quickload and the death screen all reach
// that function; the console's LoadGame command does too, and it is how the mod itself loads (the launcher's start save and Continue,
// main.cpp LoadSave), so a load called from there is let through. Game-free.
#pragma once
#include <cstdint>

namespace nvcoop::coop {

constexpr uint32_t kConsoleLoadReturn = 0x005C554Eu;   // the return address inside Cmd_LoadGame (0x5C54D0) after its call to the load

// blockedGuest: this machine is a guest that joined a session whose host blocks guest loads (ClientSession::clientLoadAllowed() == false).
inline bool guestLoadAllowed(bool blockedGuest, uint32_t returnAddress) {
    return !blockedGuest || returnAddress == kConsoleLoadReturn;
}

}  // namespace nvcoop::coop
