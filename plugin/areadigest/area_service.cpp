#include "areadigest/area_service.h"

#include <sstream>
#include "flight/flight.h"
#include "fnvcoop_mut.h"

namespace nvcoop::areadigest {

using fnvcoop::areadigest::BugReport;
using fnvcoop::areadigest::HealNote;
using fnvcoop::areadigest::kCats;

// ---------------------------------------------------------------------------------------------------------------- flight link
void FlightLink::onHeal(const HealNote& n) {
    const std::string line = std::string(toString(n.cat)) + " key=" + std::to_string(n.key) + " cell=" + std::to_string(n.cell) + " " + n.text;
    if (!FNV_MUT(82)) {   // twin 82: the status never names the healed key
        lastHeals_.push_back(line);
        while (lastHeals_.size() > kLastHeals) lastHeals_.pop_front();
    }
    if (!rec_ || FNV_MUT(80)) return;   // twin 80: a healed key leaves no marker in the trace
    ++healsRecorded_;
    rec_->healed(static_cast<uint8_t>(n.cat), "key=" + std::to_string(n.key) + " cell=" + std::to_string(n.cell) + " " + n.text);
}

void FlightLink::onBug(const BugReport& b) {
    if (!rec_) return;
    ++bugsRecorded_;
    // a repeated mismatch is a bug in the sync layer: both values go in the trace, and the recorder dumps (rate limited) so it can be analysed
    rec_->marker(fnvcoop::flight::kMarkHeal, std::string("BUG repeated ") + toString(b.cat) + " key=" + std::to_string(b.key) + " x" + std::to_string(b.count) +
                                                 " authority{" + b.authority + "} other{" + b.other + "}");
    rec_->onDesync(std::string("area digest: repeated mismatch ") + toString(b.cat) + " key=" + std::to_string(b.key));
}

void FlightLink::exportCounters(const fnvcoop::areadigest::AreaSync* sync, uint64_t nowMs) {
    if (!rec_ || !sync || FNV_MUT(81)) return;   // twin 81: the counters never reach the recorder
    std::vector<fnvcoop::flight::FlightRecorder::HealRow> rows;
    for (const auto& r : sync->catRows(nowMs)) {
        fnvcoop::flight::FlightRecorder::HealRow h;
        h.category = static_cast<uint8_t>(r.cat);
        h.total = r.total;
        h.perMinute = r.perMinute;
        h.mismatches = r.mismatches;
        rows.push_back(h);
    }
    rec_->healCounters(rows);
}

// ---------------------------------------------------------------------------------------------------------------- service
AreaService::AreaService(const AreaSettings& settings, ServiceInputs in)
    : set_(settings), in_(std::move(in)), link_(in_.flight), rt_(settings.cfg) {
    std::vector<fnvcoop::areadigest::IAreaSource*> sources;
    if (set_.actors && in_.actors) {
        actors_.reset(new ActorAreaSource(*in_.actors, set_.cfg, set_.maxActors));
        sources.push_back(actors_.get());
    }
    WorldGetter wg = [this]() -> const fnvcoop::ShadowState* {
        ShadowStorySource::World w;
        return in_.world && in_.world(w) ? w.state : nullptr;
    };
    if (set_.containers) {
        containers_.reset(new ShadowRefSource(fnvcoop::areadigest::Cat::Containers, wg, in_.cellOf, set_.cfg));
        sources.push_back(containers_.get());
    }
    if (set_.objects) {
        objects_.reset(new ShadowRefSource(fnvcoop::areadigest::Cat::Objects, wg, in_.cellOf, set_.cfg));
        sources.push_back(objects_.get());
    }
    story_.world = in_.world;
    story_.repair = in_.repair;
    story_.repairCount = in_.repairCount;
    rt_.setSources(std::move(sources));
    rt_.setStory(in_.world ? &story_ : nullptr);   // also with story=0: delegated categories still need its repair hook (cfg.compareStory gates the digest)
    rt_.setObserver(&link_);
}

AreaService::~AreaService() = default;

void AreaService::frame(const FrameInfo& f) {
    rt_.frame(f);
    if (f.active && f.nowMs >= nextExportAt_) {
        nextExportAt_ = f.nowMs + 5000;
        link_.exportCounters(rt_.sync(), f.nowMs);
    }
}

std::string AreaService::statusJson(uint64_t nowMs) {
    const fnvcoop::areadigest::AreaSync* s = rt_.sync();
    std::ostringstream o;
    o << "{\"running\":" << (s ? "true" : "false") << ",\"settings\":\"" << describe(set_) << "\",\"rebuilds\":" << rt_.rebuilds()
      << ",\"droppedUnknownPeer\":" << rt_.droppedUnknownPeer();
    if (!set_.warnings.empty()) {
        std::string w = set_.warnings;
        for (char& c : w) if (c == '\n' || c == '"' || c == '\\') c = ' ';
        o << ",\"warnings\":\"" << w << "\"";
    }
    if (actors_) {
        const auto& st = actors_->stats();
        o << ",\"actorSource\":{\"scans\":" << st.scans << ",\"snaps\":" << st.snaps << ",\"smooths\":" << st.smooths << ",\"smoothing\":" << actors_->smoothing()
          << ",\"hpSet\":" << st.hpSet << ",\"deathRequests\":" << st.deathRequests << ",\"refused\":" << st.refused << ",\"missing\":" << st.missing
          << ",\"capped\":" << st.capped << "}";
    }
    o << ",\"flight\":{\"healsRecorded\":" << link_.healsRecorded() << ",\"bugsRecorded\":" << link_.bugsRecorded() << "}";
    o << ",\"lastHeals\":[";
    {
        bool first = true;
        for (const std::string& h : link_.lastHeals()) {
            std::string t = h;
            for (char& c : t) if (c == '"' || c == '\\' || static_cast<unsigned char>(c) < 0x20) c = ' ';
            o << (first ? "" : ",") << "\"" << t << "\"";
            first = false;
        }
    }
    o << "]";
    if (s) {
        const std::string inner = s->statusJson(nowMs);   // {"enabled":true,"role":...}: splice its members in
        if (inner.size() > 2) o << "," << inner.substr(1, inner.size() - 2);
    } else {
        o << ",\"enabled\":true";
    }
    o << "}";
    return o.str();
}

}  // namespace nvcoop::areadigest
