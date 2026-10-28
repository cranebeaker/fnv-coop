// Decision of the SetPlayerControls hook (W3a item 5, P6 request 1), game-free. While a GUEST is the acting player and the effect is captured, the session's router says
// whether the lock concerns that guest's player only (rules scope=initiator). Then the host's own engine call is skipped: the host is never locked for a guest's scene (before this
// the host locked itself and the runtime gave the bits back when the committed effect excluded slot 0).
#pragma once
#include <cstdint>
#include <functional>

namespace nvhooks {

inline bool skipHostControlsCall(bool captured, uint8_t actingSlot, bool disable, const std::function<bool(uint8_t, bool)>& router) {
#ifdef NVHOOKS_MUTANT
    (void)captured; (void)actingSlot; (void)disable; (void)router;
    return false;   // mutant: the host always makes the call
#else
    return captured && actingSlot != 0 && router && router(actingSlot, disable);
#endif
}

}  // namespace nvhooks
