#include "actors/owner_rpc.h"
#include "actors/state_parts.h"
#include <cctype>
#include <cstdlib>
#include "common/minijson.h"

namespace nvcoop::actors {

bool parseRef(const std::string& text, FormRef& out) {
    const size_t c = text.rfind(':');
    if (c == std::string::npos || c == 0 || c + 1 >= text.size()) return false;
    const std::string hex = text.substr(c + 1);
    for (char ch : hex) if (!std::isxdigit(static_cast<unsigned char>(ch)) && ch != 'x' && ch != 'X') return false;
    char* end = nullptr;
    const unsigned long id = std::strtoul(hex.c_str(), &end, 16);
    if (!end || *end) return false;
    out = FormRef::make(text.substr(0, c), static_cast<uint32_t>(id));
    return true;
}

std::string ownerJson(const OwnerInfo& i) {
    nvcoop::json::Value v = nvcoop::json::Value::object();
    v.set("owner", static_cast<int>(i.owner));
    v.set("epoch", static_cast<int64_t>(i.epoch));
    v.set("known", i.known);
    v.set("mine", i.mine);
    v.set("pending", i.pending);
    v.set("self", static_cast<int>(i.self));
    v.set("netId", static_cast<int64_t>(i.netId));
    v.set("ai", i.aiOn);
    v.set("standIn", i.standIn);
    v.set("dead", i.dead);
    return v.dump();
}

std::string statsJson(const ActorSync& sync) {
    const Stats& s = sync.stats();
    nvcoop::json::Value v = nvcoop::json::Value::object();
    auto n = [&](const char* k, uint64_t x) { v.set(k, static_cast<int64_t>(x)); };
    n("tracked", sync.trackedCount());
    n("published", s.published); n("snapshotsApplied", s.snapshotsApplied); n("snapshotsDropped", s.snapshotsDropped);
    n("staleEpochDropped", s.staleEpochDropped); n("staleSeqDropped", s.staleSeqDropped); n("notOwnerDropped", s.notOwnerDropped); n("futureEpoch", s.futureEpoch);
    n("finalsSent", s.finalsSent); n("finalsApplied", s.finalsApplied); n("finalsRepeated", s.finalsRepeated); n("acksSent", s.acksSent);
    n("takeovers", s.takeovers); n("handoffsOut", s.handoffsOut); n("aiHolds", s.aiHolds);
    n("avAudits", s.avAudits); n("avDriftFixed", s.avDriftFixed);
    // State parts (state_parts.h), by kind: {"parts":{"inventory":{"published":..,"applied":..,"rejected":..,"duplicate":..,"applyFailed":..},"equipment":{..}}}
    nvcoop::json::Value parts = nvcoop::json::Value::object();
    for (size_t k = 0; k < kPartCount; ++k) {
        const PartStats& ps = s.parts[k];
        nvcoop::json::Value pv = nvcoop::json::Value::object();
        pv.set("published", static_cast<int64_t>(ps.published)); pv.set("applied", static_cast<int64_t>(ps.applied));
        pv.set("rejected", static_cast<int64_t>(ps.rejected)); pv.set("duplicate", static_cast<int64_t>(ps.duplicate));
        pv.set("applyFailed", static_cast<int64_t>(ps.applyFailed));
        parts.set(partName(static_cast<PartKind>(k)), pv);
    }
    v.set("parts", parts);
    if (const fnvcoop::OwnershipService* o = sync.service()) {   // host only
        const fnvcoop::OwnershipStats& os = o->stats();
        n("grants", os.grants); n("transfers", os.transfers); n("commits", os.commits); n("forcedCommits", os.forcedCommits);
        n("immediateCommits", os.immediateCommits); n("abortedHandoffs", os.abortedHandoffs);
    }
    return v.dump();
}

bool ownerRpc(const ActorSync* sync, const std::string& paramsJson, std::string& out) {
    if (!sync) {
        out = "{\"unsupported\":true,\"reason\":\"actor sync is not running (no coop session)\"}";
        return true;
    }
    nvcoop::json::Value p;
    std::string err;
    if (!nvcoop::json::parse(paramsJson.empty() ? "{}" : paramsJson, p, err)) { out = "bad params: " + err; return false; }
    if (p.get("stats").asBool(false)) { out = statsJson(*sync); return true; }
    const std::string refText = p.get("ref").asString("");
    if (refText.empty()) { out = "missing ref (\"plugin.esm:00ABCD\")"; return false; }
    FormRef ref;
    if (!parseRef(refText, ref)) { out = "bad ref: " + refText; return false; }
    NetId id = 0;
    if (!sync->netIdOf(ref, id)) {
        nvcoop::json::Value v = nvcoop::json::Value::object();
        v.set("owner", 255);
        v.set("epoch", 0);
        v.set("known", false);
        v.set("self", static_cast<int>(sync->self()));
        v.set("tracked", static_cast<int64_t>(sync->trackedCount()));
        out = v.dump();
        return true;
    }
    out = ownerJson(sync->ownerOf(id));
    return true;
}

}  // namespace nvcoop::actors
