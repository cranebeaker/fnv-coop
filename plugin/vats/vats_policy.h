// Real-time VATS policy: the pure decisions behind the engine hooks (vats_module.cpp). Game-free and stateless, so every rule is unit-tested.
//
// Three things stop or slow a shared world in vanilla VATS. Each has one function here, and each hook in the module is a one-line call to it:
//   1. the menu pause:   the main loop fills the byte 0x11DEA2B from MenuMode(); the VATS menu is a menu, so the whole world stops.
//        -> menuModeForWorld()
//   2. the time scale:   the VATS camera code asks TimeGlobal::SetTimeScale for 1e-4 (freeze), 0.3 (kill shot), a per-action factor and 1.0.
//        -> scaleForVatsRequest()
//   3. the multipliers:  while the world is slowed the engine speeds the player (and the VATS target) up again (GetPlayerTimeMult / GetTargetTimeMult,
//        fVATSPlayerTimeUpdateMult = 3 in playback). With a world that runs at normal speed that would make the player 3x faster than everybody else.
//        -> timeMultFor()
// A "co-op session" is required for all three: without one VATS is the vanilla game (solo play, the main menu, tests of the base game).
#pragma once
#include <atomic>

#include "vats/vats_types.h"

namespace nvcoop::vats {

struct PolicyInput {
    Settings settings;
    bool sessionActive = false;      // a co-op session (host, or a client that has joined) is running on this machine
    Mode mode = Mode::None;          // the engine's VATS mode right now
    bool vatsMenuOpen = false;       // the VATS menu is the top menu
    bool partySlowActive = false;    // a host-granted party slow-mo window is running (vats_slowmo.h)
    float partyScale = 1.0f;         // its world time scale
};

// Real-time rules apply on this machine right now.
bool realtimeActive(const PolicyInput& in);

// What the main loop should believe MenuMode() said. `engineValue` is the engine's own answer. Only the world-pause decision is changed: the input
// side of the VATS menu still sees the real MenuMode(), because only the two main-loop call sites are redirected.
bool menuModeForWorld(const PolicyInput& in, bool engineValue);

// The scale the world should have. Real time: 1.0, or the party slow-mo scale while a window runs. Outside real time the request goes through.
float scaleForVatsRequest(const PolicyInput& in, float requested);

// The value GetPlayerTimeMult / GetTargetTimeMult should return. Real time: exactly 1 (nobody is sped up to compensate for a slow-down that does not
// happen, and during a party slow-mo everyone, VATS user included, is slowed together).
double timeMultFor(const PolicyInput& in, double engineValue);

// Who runs the InterfaceManager update (0x86FD70) in a frame whose worker threads are kicked: true = the main thread, just before the kick; false = the
// worker thread, as the engine does. `engineMenuMode` is the real MenuMode() at the kick; only the menu whose pause menuModeForWorld() lifted counts. Vanilla never kicks the workers with a menu open (it pauses
// the world), so the update and the menu's own drawing never run at the same time. Real-time VATS kicks them under the open VATS menu; the update then
// belongs on the main thread, or the worker clears the VATS highlight target while the main thread draws it (crash at 0x800F66, vats_addrs.h).
bool interfaceUpdateOnMain(const PolicyInput& in, bool engineMenuMode);

// The per-frame handoff of that update between the kick (main thread) and the worker's stage 0. The kick decides before it resumes the workers, so the
// worker reads the decision of its own frame (the kick resets and sets events, a full barrier). The update runs exactly once per kicked frame: the main
// thread takes it only when every worker call site is redirected, otherwise the worker runs it as in the game.
class InterfaceHandoff {
public:
    // main thread, at the kick: true = run the update NOW, before the kick
    bool atKick(bool wantMain, bool workerSitesPatched);
    // worker thread, stage 0: true = run the update (the main thread did not take it this frame)
    bool workerRuns() const { return !mainOwns_.load(std::memory_order_acquire); }
    void reset() { mainOwns_.store(false, std::memory_order_release); }

private:
    std::atomic<bool> mainOwns_{false};
};

// The world scale this machine wants to HOLD (for the per-frame re-assert and for the window edges): the party scale while a window runs, else 1.
float desiredWorldScale(const PolicyInput& in);

}  // namespace nvcoop::vats
