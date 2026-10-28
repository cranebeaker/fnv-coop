#include "vats/vats_policy.h"

#include "../../tools/mut/fnvcoop_mut.h"

namespace nvcoop::vats {

bool realtimeActive(const PolicyInput& in) {
    if (FNV_MUT(811)) return in.settings.realtime;   // twin: a solo game would also lose its VATS pause
    return in.settings.realtime && in.sessionActive;
}

bool menuModeForWorld(const PolicyInput& in, bool engineValue) {
    if (!engineValue) return false;
    if (!realtimeActive(in)) return engineValue;
    if (FNV_MUT(812)) return false;                                  // twin: every menu stops pausing the world (Pip-Boy, dialogue, barter ...)
    if (!in.vatsMenuOpen) return engineValue;                        // another menu is on top: it pauses as always
    if (FNV_MUT(813)) return isSelecting(in.mode) ? false : engineValue;   // twin: the playback still pauses
    return isActive(in.mode) ? false : engineValue;
}

bool interfaceUpdateOnMain(const PolicyInput& in, bool engineMenuMode) {
    if (FNV_MUT(818)) return false;                                  // twin: the worker keeps the update under the open VATS menu (the 0x800F66 crash)
    // only when the open menu is the one whose pause real time lifted (the VATS menu): any other kick with a menu open (a menu opened in the middle of a
    // frame) stays exactly as the game does it
    return engineMenuMode && !menuModeForWorld(in, true);
}

bool InterfaceHandoff::atKick(bool wantMain, bool workerSitesPatched) {
    const bool main = wantMain && (workerSitesPatched || FNV_MUT(819));   // twin: an unpatched worker would run the update a second time
    mainOwns_.store(main, std::memory_order_release);
    return main;
}

float scaleForVatsRequest(const PolicyInput& in, float requested) {
    if (FNV_MUT(814)) return requested;                              // twin: the engine's freeze / slow-mo request goes through
    if (!realtimeActive(in)) return requested;
    return desiredWorldScale(in);
}

double timeMultFor(const PolicyInput& in, double engineValue) {
    if (FNV_MUT(815)) return engineValue;                            // twin: the player still runs at 3x in playback
    if (!realtimeActive(in)) return engineValue;
    return 1.0;
}

float desiredWorldScale(const PolicyInput& in) {
    if (FNV_MUT(816)) return 1.0f;                                   // twin: the party slow-mo scale is never applied
    if (in.partySlowActive && in.settings.partySlowMo) return in.partyScale;
    if (FNV_MUT(817) && in.partySlowActive) return in.partyScale;    // twin: a window of a host with slow-mo turned off still slows (host switch ignored)
    return 1.0f;
}

}  // namespace nvcoop::vats
