// The stack arguments after the activator of TESObjectREFR::Activate (0x573170, thiscall, ret 0x10) when the bridge method `activate.ref` activates a reference as the
// player. The console's `Activate` command pushes (activator, 0, 0, 1): `push 1; push 0; push 0; push activator` at the engine's own call sites (docs/engine-addresses.md
// section 7). The last argument must be 1: with 0 the engine DELETES a placed item (a Stimpak) and gives it to nobody (lab finding, fix wave 1b).
#pragma once
#include <cstdint>

namespace nvhooks {

struct ActivateArgs {
    uint8_t flagA;
    uint32_t flagB;
    uint32_t last;
};

inline ActivateArgs consoleActivateArgs() {
#ifdef NVHOOKS_MUTANT
    return {0, 0, 0};   // mutant: the bridge activation passes 0 last, the engine deletes a placed item
#else
    return {0, 0, 1};
#endif
}

}  // namespace nvhooks
