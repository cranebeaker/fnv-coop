#include "script_npc_engine.h"

#include <cmath>
#include <cstdio>

#include "anim_groups.h"
#include "fnvcoop_mut.h"

namespace nvcoop::actors::anim {

std::string formatNumber(double v) {
    if (!std::isfinite(v)) v = 0;
    char buf[64];
    std::snprintf(buf, sizeof buf, "%.3f", v);
    for (char* c = buf; *c; ++c)
        if (*c == ',') *c = '.';   // a decimal comma must never leak into script text
    std::string s = buf;
    if (s == "-0.000") s = "0.000";
    return s;
}

double yawRadToDegrees(float yawRad) {
    constexpr double kPi = 3.14159265358979323846;
    if (!std::isfinite(yawRad)) return 0;
    double a = std::fmod(static_cast<double>(yawRad), 2 * kPi);
    if (a > kPi) a -= 2 * kPi;
    if (a <= -kPi) a += 2 * kPi;
    return a * 180.0 / kPi;
}

Result ScriptNpcAnimEngine::PlayGroup(NetId id, int baseGroup, bool hold) {
    const char* name = groupName(baseGroup);
    if (FNV_MUT(70) && (!name || !*name)) name = "Idle";   // twin 70: an unknown group is guessed (a wrong animation instead of none)
    if (!name || !*name) return Result::Refused;   // never guess a group name
    return line(id, std::string("PlayGroup ") + name + (hold ? " 1" : " 0"));
}

Result ScriptNpcAnimEngine::ReleaseHold(NetId id) {
    return line(id, "PlayGroup Idle 0");
}

Result ScriptNpcAnimEngine::SetSneak(NetId id, bool on) {
    return line(id, on ? "SetForceSneak 1" : "SetForceSneak 0");
}

Result ScriptNpcAnimEngine::SetWeaponDrawn(NetId id, bool on) {
    // The direct call sets the process flag AND notifies the actor (the draw / holster visuals); the console fallback only raises the alert
    // state, which makes an actor with working AI draw. Either way the driver reads IsWeaponOut back.
    const Result d = game_.SetWeaponOut(id, on);
    if (d == Result::Ok) return d;
    const Result l = line(id, on ? "SetAlert 1" : "SetAlert 0");
    if (l == Result::Ok) return l;
    return d == Result::Unsupported ? Result::Refused : d;
}

Result ScriptNpcAnimEngine::Kill(NetId id) { return line(id, "KillActor"); }

Result ScriptNpcAnimEngine::PlaceCorpse(NetId id, const Vec3& pos, float yawRad) {
    // One command per axis: SetPos takes a single axis letter.
    Result r = line(id, "SetPos X " + formatNumber(pos.x));
    if (r != Result::Ok) return r;
    r = line(id, "SetPos Y " + formatNumber(pos.y));
    if (r != Result::Ok) return r;
    r = line(id, "SetPos Z " + formatNumber(pos.z));
    if (r != Result::Ok) return r;
    return line(id, "SetAngle Z " + formatNumber(yawRadToDegrees(yawRad)));
}

Result ScriptNpcAnimEngine::useFurniture(NetId id, uint32_t furniture) {
    const uint32_t actor = game_.RefIdOf(id);
    if (!furniture || !actor) return Result::Refused;
    char buf[48];
    if (FNV_MUT(86)) {   // twin 86: the roles are swapped, the actor "activates" the furniture reference as its argument
        std::snprintf(buf, sizeof buf, "Activate %08X 1", furniture);
        return run_.ExecOnRef(actor, buf) ? Result::Ok : Result::Refused;
    }
    std::snprintf(buf, sizeof buf, "Activate %08X 1", actor);
    return run_.ExecOnRef(furniture, buf) ? Result::Ok : Result::Refused;
}

Result ScriptNpcAnimEngine::SitIn(NetId id, uint32_t furniture) { return useFurniture(id, furniture); }

Result ScriptNpcAnimEngine::StandUp(NetId id, uint32_t furniture, uint32_t attempt) {
    if (attempt == 0 && furniture) return useFurniture(id, furniture);
    // Escalation: let the actor's own AI run, it leaves furniture its package does not ask for. The driver puts the hold back (Restrain).
    Result r = line(id, "SetRestrained 0");
    if (r != Result::Ok) return r;
    return line(id, "EvaluatePackage");
}

Result ScriptNpcAnimEngine::Restrain(NetId id) { return line(id, "SetRestrained 1"); }

}  // namespace nvcoop::actors::anim
