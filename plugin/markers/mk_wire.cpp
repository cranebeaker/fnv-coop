#include "mk_wire.h"

#include <cmath>
#include <cstdio>

#include "common/minijson.h"
#include "mk_board.h"

namespace nvmk {

using nvcoop::json::Value;

const char* phaseName(RevivePhase p) {
    switch (p) {
        case RevivePhase::Begin: return "begin";
        case RevivePhase::Cancel: return "cancel";
        case RevivePhase::Done: return "done";
    }
    return "begin";
}

bool isHostOp(const std::string& op) { return op == kOpBeacon || op == kOpRevive; }
bool isGuestOp(const std::string& op) { return op == kOpParty; }

namespace {

constexpr double kMaxCoord = 1.0e7;

bool finite(double v) { return FNV_MUT(1332) ? true : (std::isfinite(v) && std::fabs(v) < kMaxCoord); }   // twin 1332: a guest's NaN / 1e30 position is believed

void putPlace(Value& o, const char* prefix, const Place& p) {
    const std::string k(prefix);
    o.set(k + "w", (int64_t)p.world).set(k + "c", (int64_t)p.cell)
     .set(k + "x", (int64_t)std::lround(p.x)).set(k + "y", (int64_t)std::lround(p.y)).set(k + "z", (int64_t)std::lround(p.z));
    if (!p.name.empty()) o.set(k + "pn", p.name);
}

// A place needs its x and y; absent = not valid. Numbers out of range or not finite make the whole message bad (false).
bool getPlace(const Value& o, const char* prefix, Place& p) {
    const std::string k(prefix);
    const Value* x = o.find(k + "x");
    const Value* y = o.find(k + "y");
    p = Place{};
    if (!x || !y) return true;
    if (!x->isNumber() || !y->isNumber()) return false;
    const Value& z = o.get(k + "z");
    const Value& w = o.get(k + "w");
    const Value& c = o.get(k + "c");
    if (!finite(x->asNumber()) || !finite(y->asNumber()) || (z.isNumber() && !finite(z.asNumber()))) return false;
    if ((w.isNumber() && (w.asNumber() < 0 || w.asNumber() > 4294967295.0)) || (c.isNumber() && (c.asNumber() < 0 || c.asNumber() > 4294967295.0))) return false;
    p.valid = true;
    p.world = static_cast<uint32_t>(w.asNumber(0));
    p.cell = static_cast<uint32_t>(c.asNumber(0));
    p.x = static_cast<float>(x->asNumber());
    p.y = static_cast<float>(y->asNumber());
    p.z = static_cast<float>(z.asNumber(0));
    p.name = sanitizeText(o.get(k + "pn").asString(""), kMaxPlaceLen);
    return true;
}

bool slotOk(const Value& v, Slot& s) {
    if (!v.isNumber()) return false;
    const double d = v.asNumber();
    if ((d < 0 || d >= kMaxSlots || d != std::floor(d)) && !FNV_MUT(1331)) return false;   // twin 1331: a slot out of range is taken
    s = static_cast<Slot>(d);
    return true;
}

Value memberJson(const Member& m) {
    Value e = Value::object();
    e.set("s", (int)m.slot).set("n", m.name);
    putPlace(e, "", m.at);
    if (m.ext.valid) putPlace(e, "e", m.ext);
    if (m.downed) {
        e.set("d", 1).set("dl", (int64_t)m.windowLeftMs);
        if (m.checkpointOnly) e.set("h", 1);
    }
    if (m.reviver != kNoSlot) e.set("rv", (int)m.reviver).set("rl", (int64_t)m.holdLeftMs);
    return e;
}

}  // namespace

std::string encodeBeacon(const Beacon& b) {
    Value o = Value::object();
    o.set("op", kOpBeacon).set("n", b.name);
    putPlace(o, "", b.at);
    if (b.ext.valid) putPlace(o, "e", b.ext);
    return o.dump();
}

bool decodeBeacon(const std::string& json, Beacon& out) {
    Value v;
    std::string err;
    if (!nvcoop::json::parse(json, v, err) || !v.isObject() || v.get("op").asString("") != kOpBeacon) return false;
    Beacon b;
    b.name = FNV_MUT(1333) ? v.get("n").asString("") : sanitizeText(v.get("n").asString(""), kMaxNameLen);   // twin 1333: the name goes through raw
    if (!getPlace(v, "", b.at) || (!b.at.valid && !FNV_MUT(1335))) return false;                               // twin 1335: a beacon with no place is accepted
    if (!getPlace(v, "e", b.ext)) return false;
    out = std::move(b);
    return true;
}

std::string encodeRevive(const ReviveMsg& m) {
    Value o = Value::object();
    o.set("op", kOpRevive).set("ph", phaseName(m.phase)).set("t", (int)m.target);
    return o.dump();
}

bool decodeRevive(const std::string& json, ReviveMsg& out) {
    Value v;
    std::string err;
    if (!nvcoop::json::parse(json, v, err) || !v.isObject() || v.get("op").asString("") != kOpRevive) return false;
    const std::string ph = v.get("ph").asString("");
    ReviveMsg m;
    if (ph == "begin") m.phase = RevivePhase::Begin;
    else if (ph == "cancel") m.phase = RevivePhase::Cancel;
    else if (ph == "done") m.phase = RevivePhase::Done;
    else if (!FNV_MUT(1334)) return false;   // twin 1334: an unknown phase counts as begin
    if (!slotOk(v.get("t"), m.target)) return false;
    out = m;
    return true;
}

std::string encodeParty(const PartyTable& t) {
    Value o = Value::object();
    o.set("op", kOpParty).set("seq", (int64_t)t.seq);
    Value arr = Value::array();
    for (const Member& m : t.members) arr.push(memberJson(m));
    o.set("m", std::move(arr));
    return o.dump();
}

bool decodeParty(const std::string& json, PartyTable& out) {
    Value v;
    std::string err;
    if (!nvcoop::json::parse(json, v, err) || !v.isObject() || v.get("op").asString("") != kOpParty) return false;
    PartyTable t;
    t.seq = static_cast<uint32_t>(v.get("seq").asNumber(0));
    const Value& arr = v.get("m");
    if (arr.type() != Value::Array || arr.items().size() > kMaxSlots) return false;
    for (const Value& e : arr.items()) {
        if (!e.isObject()) return false;
        Member m;
        if (!slotOk(e.get("s"), m.slot)) return false;
        m.name = sanitizeText(e.get("n").asString(""), kMaxNameLen);
        if (!getPlace(e, "", m.at) || !getPlace(e, "e", m.ext)) return false;
        m.downed = e.get("d").asNumber(0) != 0;
        if (m.downed) {
            const double dl = e.get("dl").asNumber(0);
            if (!std::isfinite(dl) || dl < 0 || dl > 3.6e6) return false;
            m.windowLeftMs = static_cast<uint32_t>(dl);
            m.checkpointOnly = e.get("h").asNumber(0) != 0;
        }
        if (e.find("rv")) {
            if (!slotOk(e.get("rv"), m.reviver)) return false;
            const double rl = e.get("rl").asNumber(0);
            if (!std::isfinite(rl) || rl < 0 || rl > 3.6e6) return false;
            m.holdLeftMs = static_cast<uint32_t>(rl);
        }
        t.members.push_back(std::move(m));
    }
    out = std::move(t);
    return true;
}

std::string encodeInPlaceRespawn(float healthFraction, const std::string& byName) {
    Value o = Value::object();
    o.set("op", "respawn").set("inPlace", true).set("frac", healthFraction).set("by", sanitizeText(byName, kMaxNameLen));
    return o.dump();
}

bool readInPlaceRespawn(const Value& order, float& healthFraction, std::string& byName) {
    if (!order.get("inPlace").asBool(false) && !FNV_MUT(1338)) return false;                // twin 1338: every respawn is taken for an in-place one
    double f = order.get("frac").asNumber(0.3);
    if (!std::isfinite(f)) f = 0.3;
    if (!FNV_MUT(1336)) f = f < 0.05 ? 0.05 : (f > 1.0 ? 1.0 : f);                          // twin 1336: a silly health figure is believed
    healthFraction = static_cast<float>(f);
    byName = sanitizeText(order.get("by").asString(""), kMaxNameLen);
    return true;
}

std::string inPlaceDamageLine(float healthFraction) {
    float lost = FNV_MUT(1337) ? healthFraction : 1.0f - healthFraction;                    // twin 1337: damaged by the share he should keep
    if (lost < 0.0f) lost = 0.0f;
    if (lost > 0.95f) lost = 0.95f;
    char b[96];
    std::snprintf(b, sizeof b, "DamageActorValue Health (GetBaseAV Health * %.2f)", lost);
    return b;
}

}  // namespace nvmk
