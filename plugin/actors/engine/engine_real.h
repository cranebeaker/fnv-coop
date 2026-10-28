// RealEngineHost: IEngineHost over the running FalloutNV.exe 1.4.0.525 (x86 only, compiled against the xNVSE headers).
//
// Reads use the xNVSE layouts (Actor, TESObjectREFR, TESObjectCELL, ActorProcessManager) plus the BaseProcess / Actor vtable slots and raw
// offsets that plugin/animprobe/animprobe_game.cpp checked statically against a reference exe (tools/animcheck/verify_engine.py). Every raw
// access is SEH guarded: a wrong offset yields a missing field, never a crash. Calls into the engine are only made after the callee's prologue
// bytes were compared with the recorded ones (plugin/hooks/engine_addrs.h, plugin/hooks_world/world_addrs.h).
//
// NEVER RUN IN A GAME YET. The static facts it rests on are listed in docs/tools/actor-port.md; everything marked TODO-RE there is a guess
// with a working fallback.
#pragma once
#include <string>
#include <vector>

#include "actors/engine/engine_iface.h"
#include "actors/engine/pin_check.h"

namespace nvcoop::actors::engine {

class RealEngineHost : public IEngineHost {
public:
    RealEngineHost();

    // The module tells the host when the world is replaced (PostLoadGame, NewGame, ExitToMainMenu) and when a load is under way.
    void bumpEpoch() { ++epoch_; }
    void setLoading(bool on) { loading_ = on; }

    bool worldReady() override;
    uint32_t worldEpoch() override { return epoch_; }
    RefId playerRef() override { return kLocalPlayerRef; }
    bool read(RefId ref, ActorRaw& out) override;
    bool readPose(RefId ref, CellId& cell, live::Vec3& pos, float& rotZ) override;
    bool actorValue(RefId ref, uint8_t av, float& out) override;
    void loadedActors(std::vector<RefId>& out, size_t max) override;
    bool cellLoaded(CellId cell) override;
    bool cellInfo(CellId cell, CellInfo& out) override;
    bool toFormRef(uint32_t formId, fnvcoop::FormRef& out) override;
    uint32_t fromFormRef(const fnvcoop::FormRef& r) override;
    RefId anchorInCell(CellId cell, live::Vec3& anchorPos) override;
    RefId placeAtMe(RefId anchor, uint32_t baseId) override;
    bool console(const std::string& line, RefId on) override;
    bool killWithAttribution(RefId victim, RefId killer) override;

    // diagnostics (bridge method actors.port)
    struct Pins {
        bool placeAtMe = false, getActorValue = false, isDisabled = false;
    };
    Pins pins() const { return pins_; }
    uint64_t sehFaults() const { return sehFaults_; }

private:
    uint32_t epoch_ = 1;
    bool loading_ = false;
    Pins pins_;
    uint64_t sehFaults_ = 0;
    PinPolicy pinPolicy_;
    void checkPins();
    void ensurePins();   // first use, and again while a pin is refused (pin_check.h)
};

}  // namespace nvcoop::actors::engine
