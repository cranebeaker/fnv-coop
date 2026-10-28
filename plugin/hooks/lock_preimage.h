// Lock pre-image for the scripted Lock / UnLock commands (P5 request 2). ObjectState.hasBefore / beforeLocked / beforeLevel let a host rebase
// restore a lock the loaded save never changed; the world-time poll sets them for engine-driven changes, this does it for script ones. Game-free;
// the engine reads (GetLocked 0x59D010, GetLockLevel 0x59D100: the condition evaluators, cdecl bool(thisObj, p1, p2, double*)) are in engine_hooks.cpp.
#pragma once
#include "fnvcoop/effect.h"

namespace nvhooks {

// `wasLocked` / `wasLevel` were read just BEFORE the original command ran. Only a change that touches the lock bit carries a pre-image.
inline void applyLockPreImage(fnvcoop::ObjectState* os, bool wasLocked, int wasLevel) {
    if (!(os->mask & fnvcoop::kObjLock)) return;
    os->hasBefore = true;
#ifdef NVHOOKS_MUTANT
    os->beforeLocked = os->locked;   // mutant: the pre-image is the new state, so a rebase restores nothing
    os->beforeLevel = os->level;
#else
    os->beforeLocked = wasLocked;
    os->beforeLevel = wasLevel;
#endif
}

}  // namespace nvhooks
