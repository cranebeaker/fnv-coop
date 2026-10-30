#include "animprobe_core.h"

#include "fnvcoop_mut.h"  // FNV_MUT(n): runtime mutation twins (env FNVCOOP_MUT=n); constant false in production builds

#include <algorithm>
#include <cstdio>
#include <cstdlib>

namespace animprobe {

using nvcoop::json::Value;

const char* const kDefaultBones[] = {
    "Bip01 Pelvis",   "Bip01 Spine1",   "Bip01 Spine2",   "Bip01 Neck1",    "Bip01 Head",
    "Bip01 L UpperArm", "Bip01 L Forearm", "Bip01 L Hand",
    "Bip01 R UpperArm", "Bip01 R Forearm", "Bip01 R Hand",
    "Bip01 L Calf",   "Bip01 L Foot",   "Bip01 R Calf",   "Bip01 R Foot",
    "Weapon",
};
const int kDefaultBoneCount = static_cast<int>(sizeof(kDefaultBones) / sizeof(kDefaultBones[0]));

std::string HexId(uint32_t v) {
    char b[16];
    std::snprintf(b, sizeof b, "0x%08X", v);
    return b;
}

namespace {
bool ParseId(const Value& v, uint32_t& out) {
    if (v.isNumber()) {
        out = static_cast<uint32_t>(v.asInt());
        return true;
    }
    if (!v.isString()) return false;
    const std::string& s = v.asString();
    if (s.empty()) return false;
    char* end = nullptr;
    unsigned long n = std::strtoul(s.c_str(), &end, 16);  // ids are always hex; "0x" prefix optional
    if (end == s.c_str() || *end != 0) return false;
    out = static_cast<uint32_t>(n);
    return true;
}

Value Vec(const Vec3& v) {
    Value a = Value::array();
    a.push(static_cast<double>(v.x)).push(static_cast<double>(v.y)).push(static_cast<double>(v.z));
    return a;
}
}  // namespace

bool ParseRequest(const std::string& paramsJson, Request& out, std::string& err) {
    out = Request{};
    if (paramsJson.empty() || paramsJson == "null") {
        out.nearby = true;  // default: everything near the player, plus the player
        out.includePlayer = true;
        return true;
    }
    Value p;
    if (!nvcoop::json::parse(paramsJson, p, err)) return false;
    if (!p.isObject()) {
        err = "params must be an object";
        return false;
    }
    if (const Value* r = p.find("refs")) {
        for (const Value& it : r->items()) {
            if (it.isString() && it.asString() == "player") {
                out.includePlayer = true;
                continue;
            }
            uint32_t id = 0;
            if (!ParseId(it, id) || id == 0) {
                err = "bad ref in refs";
                return false;
            }
            out.refs.push_back(id);
        }
    }
    if (const Value* n = p.find("near")) {
        out.nearby = true;
        if (n->isObject()) out.nearRadius = static_cast<float>(n->get("radius").asNumber(2000));
        else if (n->isNumber()) out.nearRadius = static_cast<float>(n->asNumber());
        if (out.nearRadius <= 0 || out.nearRadius > 100000) {
            err = "near.radius out of range";
            return false;
        }
    }
    if (const Value* b = p.find("bones")) out.bones = b->asBool(true);
    if (const Value* s = p.find("scan_animdata")) out.scanAnimData = s->asBool(false);
    if (const Value* m = p.find("max")) {
        int64_t mx = m->asInt(16);
        if (FNV_MUT(4) ? (mx < 1) : (mx < 1 || mx > kMaxActorsPerProbe)) {   // twin 4: the upper bound is not checked
            err = "max must be 1.." + std::to_string(kMaxActorsPerProbe);
            return false;
        }
        out.maxActors = static_cast<int>(mx);
    }
    if (const Value* bn = p.find("bone_names")) {
        for (const Value& it : bn->items())
            if (it.isString() && !it.asString().empty()) out.boneNames.push_back(it.asString());
        if (out.boneNames.size() > 40) {
            err = "at most 40 bone_names";
            return false;
        }
    }
    if (out.refs.empty() && !out.nearby && !out.includePlayer) {
        err = "give refs, near or player";
        return false;
    }
    return true;
}

Value ToJson(const ActorSample& a) {
    Value o = Value::object();
    o.set("ref", HexId(a.ref)).set("base", HexId(a.base));
    if (!FNV_MUT(6)) {
        if (a.isPlayer) o.set("player", true);
    }
    o.set("valid3d", a.valid3d).set("first_person", a.firstPerson).set("dead", a.dead).set("life", a.lifeState);
    o.set("pos", Vec(a.pos)).set("rot", Vec(a.rot)).set("cell", HexId(a.cell));
    if (a.haveAnim) {
        Value an = Value::object();
        an.set("action", a.action).set("sequence_raw", HexId(a.sequenceRaw));
        auto seqArray = [](const std::vector<SeqSample>& v) {
            Value sq = Value::array();
            for (const SeqSample& s : v) {
                Value so = Value::object();
                so.set("slot", s.slot).set("group", s.group).set("state", s.state);
                so.set("t", static_cast<double>(s.t)).set("begin", static_cast<double>(s.begin)).set("end", static_cast<double>(s.end));
                so.set("freq", static_cast<double>(s.freq)).set("weight", static_cast<double>(s.weight));
                if (s.seqPtr) so.set("ptr", HexId(s.seqPtr)).set("grp_ptr", HexId(s.grpPtr));   // only with scan_animdata
                sq.push(so);
            }
            return sq;
        };
        Value sq = seqArray(a.seqs);
        if (!a.fpSeqs.empty() && !FNV_MUT(7)) an.set("seqs_first_person", seqArray(a.fpSeqs));   // twin 7: the first-person set is never emitted
        if (!a.altSeqs.empty() && !FNV_MUT(7)) an.set("seqs_alt", seqArray(a.altSeqs));
        an.set("seqs", sq);
        o.set("anim", an);
    }
    if (a.haveMove) o.set("move_flags", static_cast<int64_t>(a.moveFlags));
    Value w = Value::object();
    w.set("out", a.weaponOut).set("aiming", a.aiming).set("form", HexId(a.weaponForm));
    o.set("weapon", w);
    Value st = Value::object();
    st.set("knocked", a.knocked).set("sit_sleep", a.sitSleep).set("swimming", a.swimming).set("ragdoll_ctrl", a.ragdollCtrl);
    o.set("stance", st);
    Value ai = Value::object();
    ai.set("in_combat", a.inCombat).set("combat_target", HexId(a.combatTarget)).set("package", HexId(a.package));
    o.set("ai", ai);
    if (FNV_MUT(5) ? false : !a.bones.empty()) {   // twin 5: bones are never emitted
        Value b = Value::object();
        for (const auto& kv : a.bones) b.set(kv.first, Vec(kv.second));
        o.set("bones", b);
    }
    o.set("nodes_visited", a.nodesVisited);
    return o;
}

std::vector<int> SelectNearest(const std::vector<float>& distSq, float radius, int maxActors) {
    std::vector<int> idx;
    const float r2 = radius * radius;
    for (int i = 0; i < static_cast<int>(distSq.size()); ++i) {
        if (FNV_MUT(1)) {
            idx.push_back(i);  // mutant: ignores the radius
        } else {
            if (distSq[i] <= r2) idx.push_back(i);
        }
    }
    if (!FNV_MUT(2)) {
        std::stable_sort(idx.begin(), idx.end(), [&](int a, int b) { return distSq[a] < distSq[b]; });
    }
    if (static_cast<int>(idx.size()) > maxActors) idx.resize(maxActors);
    return idx;
}

void CostStats::Add(double us, int actors) {
    us_[count_ % kWindow] = us;
    actors_[count_ % kWindow] = actors;
    ++count_;
    sum_ += us;
    if (us > max_) max_ = us;
}

double CostStats::Percentile(double p) const {
    const int n = static_cast<int>(std::min<uint64_t>(count_, kWindow));
    if (n == 0) return 0;
    std::vector<double> v(us_, us_ + n);
    std::sort(v.begin(), v.end());
    if (FNV_MUT(3)) {
        p = 1.0;  // mutant: every percentile is the maximum
    }
    int k = static_cast<int>(p * (n - 1) + 0.5);
    if (k < 0) k = 0;
    if (k >= n) k = n - 1;
    return v[k];
}

Value CostStats::ToJson() const {
    Value o = Value::object();
    const int n = static_cast<int>(std::min<uint64_t>(count_, kWindow));
    double perActor = 0;
    long actors = 0;
    double us = 0;
    for (int i = 0; i < n; ++i) {
        us += us_[i];
        actors += actors_[i];
    }
    if (actors > 0) perActor = us / static_cast<double>(actors);
    o.set("probes", static_cast<int64_t>(count_)).set("window", n);
    o.set("p50_us", Percentile(0.5)).set("p95_us", Percentile(0.95)).set("max_us", max_);
    o.set("mean_us", count_ ? sum_ / static_cast<double>(count_) : 0.0).set("us_per_actor", perActor);
    return o;
}

}  // namespace animprobe
