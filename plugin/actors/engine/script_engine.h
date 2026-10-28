// ScriptActorEngine: IActorEngine on top of the small IEngineHost, every write being ONE console line run with the actor as the calling
// reference (RunScriptLine2(line, ref), the bridge exec{cmd, ref} path; the console does not accept "<hexid>.Command").
// Only commands of the pinned vanilla command table (plugin/hooks/tools/re/command_table.json) are used: SetPos, SetAngle, MoveTo
// (= MoveToMarker), SetRestrained, StopCombat, StartCombat, EvaluatePackage, SetActorRefEssential, ForceActorValue, KillActor,
// ResurrectActor, Disable, Enable, MarkForDelete. The exact text of each line is pinned by tests/test_script_engine.cpp.
//
// Cost: every line is compiled by the script interpreter (Script::Run). The cost per line has NOT been measured; the port budgets lines
// per frame (Config::maxLinesPerFrame) and IEngineHost::directPose() is the hook for a cheaper direct write once someone finds one.
#pragma once
#include <string>

#include "actors/engine/engine_iface.h"

namespace nvcoop::actors::engine {

namespace lines {
std::string setPos(char axis, float value);              // "SetPos X 12.5000"
std::string setAngleZ(float yawRad);                     // "SetAngle Z -90.0000" (degrees, wrapped to (-180, 180])
std::string moveTo(RefId anchor, float ox, float oy, float oz);   // "MoveTo 00112233 1.0000 2.0000 3.0000" (offsets from the anchor)
std::string setRestrained(bool on);                      // "SetRestrained 1"
std::string stopCombat();                                // "StopCombat"
std::string startCombat(RefId target);                   // "StartCombat 00112233"
std::string evaluatePackage();                           // "EvaluatePackage"
std::string setEssential(bool on);                       // "SetActorRefEssential 1"
std::string forceAv(const char* avName, float value);    // "ForceActorValue Health 85"
std::string kill();                                      // "KillActor"
std::string resurrect();                                 // "ResurrectActor 0"
std::string setDisabled(bool disabled);                  // "Disable" / "Enable"
std::string markForDelete();                             // "MarkForDelete"
double yawDegrees(float yawRad);                         // wrapped to (-180, 180]
}  // namespace lines

class ScriptActorEngine : public IActorEngine {
public:
    explicit ScriptActorEngine(IEngineHost& host) : h_(host) {}

    bool worldReady() override { return h_.worldReady(); }
    uint32_t worldEpoch() override { return h_.worldEpoch(); }
    RefId playerRef() override { return h_.playerRef(); }
    bool read(RefId ref, ActorRaw& out) override { return h_.read(ref, out); }
    bool readPose(RefId ref, CellId& cell, live::Vec3& pos, float& rotZ) override { return h_.readPose(ref, cell, pos, rotZ); }
    bool actorValue(RefId ref, uint8_t av, float& out) override { return h_.actorValue(ref, av, out); }
    void loadedActors(std::vector<RefId>& out, size_t max) override { h_.loadedActors(out, max); }
    bool cellLoaded(CellId cell) override { return h_.cellLoaded(cell); }
    bool cellInfo(CellId cell, CellInfo& out) override { return h_.cellInfo(cell, out); }
    bool toFormRef(uint32_t formId, fnvcoop::FormRef& out) override { return h_.toFormRef(formId, out); }
    uint32_t fromFormRef(const fnvcoop::FormRef& r) override { return h_.fromFormRef(r); }
    RefId anchorInCell(CellId cell, live::Vec3& p) override { return h_.anchorInCell(cell, p); }

    RefId placeAtMe(RefId anchor, uint32_t baseId) override;
    bool setPos(RefId ref, const live::Vec3& p, uint8_t axisMask) override;
    bool setYaw(RefId ref, float yawRad) override;
    bool moveToCell(RefId ref, CellId cell, const live::Vec3& p, float yawRad) override;
    bool setRestrained(RefId ref, bool on) override;
    bool stopCombat(RefId ref) override;
    bool startCombat(RefId ref, RefId target) override;
    bool evaluatePackage(RefId ref) override;
    bool setEssential(RefId ref, bool on) override;
    bool forceAv(RefId ref, uint8_t av, float value) override;
    bool killActor(RefId ref) override;
    bool killActorBy(RefId ref, RefId killer) override;
    bool resurrect(RefId ref) override;
    bool setDisabled(RefId ref, bool disabled) override;
    bool markForDelete(RefId ref) override;

    uint64_t linesRun() const override { return run_; }
    uint64_t linesFailed() const override { return failed_; }

private:
    bool line(const std::string& text, RefId on);
    IEngineHost& h_;
    uint64_t run_ = 0, failed_ = 0;
};

}  // namespace nvcoop::actors::engine
