#include "actors/engine/script_engine.h"

#include <cmath>
#include <cstdio>

#include "apply_lines.h"      // plugin/hooks: setPosCmd / setAngleCmd / killCmd / resurrectCmd / moveToMarkerCmd (one text builder for the whole plugin)
#include "fnvcoop_mut.h"

namespace nvcoop::actors::engine {

const std::vector<uint8_t>& watchedAvs() {
    static const std::vector<uint8_t> v = {kAvHealth, kAvFatigue, kAvHead, kAvTorso, kAvLeftArm, kAvRightArm, kAvLeftLeg, kAvRightLeg, kAvBrain};
    return v;
}

const char* avConsoleName(uint8_t av) {
    switch (av) {
        case kAvHealth: return "Health";
        case kAvFatigue: return "Fatigue";
        case kAvActionPoints: return "ActionPoints";
        case kAvHead: return FNV_MUT(4) ? "Head" : "PerceptionCondition";
        case kAvTorso: return "EnduranceCondition";
        case kAvLeftArm: return "LeftAttackCondition";
        case kAvRightArm: return "RightAttackCondition";
        case kAvLeftLeg: return "LeftMobilityCondition";
        case kAvRightLeg: return "RightMobilityCondition";
        case kAvBrain: return "BrainCondition";
        default: return nullptr;
    }
}

namespace lines {

double yawDegrees(float yawRad) {
    double d = static_cast<double>(yawRad) * 57.29577951308232;
    if (FNV_MUT(2)) return d;   // BAD: not wrapped
    d = std::fmod(d, 360.0);
    if (d > 180.0) d -= 360.0;
    if (d <= -180.0) d += 360.0;
    return d;
}

std::string setPos(char axis, float value) { return nvhooks::setPosCmd(axis, value); }
std::string setAngleZ(float yawRad) { return nvhooks::setAngleCmd('Z', static_cast<float>(yawDegrees(yawRad))); }
std::string moveTo(RefId anchor, float ox, float oy, float oz) { return nvhooks::moveToMarkerCmd(anchor, ox, oy, oz); }
std::string setRestrained(bool on) { return on ? "SetRestrained 1" : "SetRestrained 0"; }
std::string stopCombat() { return "StopCombat"; }
std::string startCombat(RefId target) { char b[48]; std::snprintf(b, sizeof b, "StartCombat %08X", target); return b; }
std::string evaluatePackage() { return "EvaluatePackage"; }
std::string setEssential(bool on) { return on ? "SetActorRefEssential 1" : "SetActorRefEssential 0"; }
std::string forceAv(const char* avName, float value) {
    // The Amount parameter is an integer: "ForceActorValue Health 85.00" does not compile ("Unknown variable '85.00' for parameter Amount")
    // yet the console answers ok, so the old "%.2f" line changed nothing in game (found 2026-10-22).
    char b[96];
    if (FNV_MUT(40)) std::snprintf(b, sizeof b, "ForceActorValue %s %.2f", avName, static_cast<double>(value));   // twin 40: the decimal form
    else std::snprintf(b, sizeof b, "ForceActorValue %s %ld", avName, std::lround(static_cast<double>(value)));
    return b;
}
std::string kill() { return FNV_MUT(5) ? "Kill" : nvhooks::killCmd(); }
std::string resurrect() { return nvhooks::resurrectCmd(false); }
std::string setDisabled(bool disabled) { return disabled ? "Disable" : "Enable"; }
std::string markForDelete() { return "MarkForDelete"; }

}  // namespace lines

bool ScriptActorEngine::line(const std::string& text, RefId on) {
    ++run_;
    const bool ok = h_.console(text, on);
    if (!ok && !FNV_MUT(6)) ++failed_;   // (twin 6: a refused line is not counted)
    return ok;
}

RefId ScriptActorEngine::placeAtMe(RefId anchor, uint32_t baseId) { return h_.placeAtMe(anchor, baseId); }

bool ScriptActorEngine::setPos(RefId ref, const live::Vec3& p, uint8_t axisMask) {
    if (FNV_MUT(1)) axisMask = 7;   // BAD: every axis is written, the mask is ignored
    if (axisMask & 7u) {
        if (h_.directPose(ref, p, axisMask)) return true;
    }
    bool ok = true;
    if (axisMask & 1u) ok = line(lines::setPos('X', p.x), ref) && ok;
    if (axisMask & 2u) ok = line(lines::setPos('Y', p.y), ref) && ok;
    if (axisMask & 4u) ok = line(lines::setPos('Z', p.z), ref) && ok;
    return ok;
}

bool ScriptActorEngine::setYaw(RefId ref, float yawRad) { return line(lines::setAngleZ(yawRad), ref); }

bool ScriptActorEngine::moveToCell(RefId ref, CellId cell, const live::Vec3& p, float yawRad) {
    live::Vec3 a;
    const RefId anchor = h_.anchorInCell(cell, a);
    if (!anchor) return false;
    const float sign = FNV_MUT(3) ? -1.0f : 1.0f;   // BAD: the offset is taken from the wrong side
    if (!line(lines::moveTo(anchor, sign * (p.x - a.x), sign * (p.y - a.y), sign * (p.z - a.z)), ref)) return false;
    return line(lines::setAngleZ(yawRad), ref);
}

bool ScriptActorEngine::setRestrained(RefId ref, bool on) { return line(lines::setRestrained(on), ref); }
bool ScriptActorEngine::stopCombat(RefId ref) { return line(lines::stopCombat(), ref); }
bool ScriptActorEngine::startCombat(RefId ref, RefId target) { return line(lines::startCombat(target), ref); }
bool ScriptActorEngine::evaluatePackage(RefId ref) { return line(lines::evaluatePackage(), ref); }
bool ScriptActorEngine::setEssential(RefId ref, bool on) { return line(lines::setEssential(on), ref); }

bool ScriptActorEngine::forceAv(RefId ref, uint8_t av, float value) {
    const char* name = avConsoleName(av);
    if (!name) return false;   // not an actor value the port mirrors: no line, no guess
    return line(lines::forceAv(name, value), ref);
}

bool ScriptActorEngine::killActor(RefId ref) { return line(lines::kill(), ref); }
bool ScriptActorEngine::killActorBy(RefId ref, RefId killer) {
    // MUTANT 75 (engine_port tests): the attribution is dropped, the plain kill runs (the victim has no killer, IsKiller is false for the guest)
    if (killer && !FNV_MUT(75) && h_.killWithAttribution(ref, killer)) return true;
    return killActor(ref);
}
bool ScriptActorEngine::resurrect(RefId ref) { return line(lines::resurrect(), ref); }
bool ScriptActorEngine::setDisabled(RefId ref, bool disabled) { return line(lines::setDisabled(disabled), ref); }
bool ScriptActorEngine::markForDelete(RefId ref) { return line(lines::markForDelete(), ref); }

}  // namespace nvcoop::actors::engine
