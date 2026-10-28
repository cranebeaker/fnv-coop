// AreaService: assembles one machine's area desync module from its settings and its inputs (game-free, unit-tested end to end).
//   settings + actor facade + session shadow hooks  ->  sources (actors, containers, objects), story source, flight link, AreaRuntime.
// The thin dll glue (area_glue.cpp) only turns the coop Runtime into a FrameInfo and these inputs.
#pragma once
#include <deque>
#include <functional>
#include <memory>
#include <string>
#include "areadigest/actor_source.h"
#include "areadigest/area_config.h"
#include "areadigest/area_runtime.h"
#include "areadigest/shadow_source.h"

namespace fnvcoop::flight { class FlightRecorder; }

namespace nvcoop::areadigest {

// Heals and bug lines go into the flight recorder as kMarkHeal markers; the counters go into its header Meta (healCounters).
class FlightLink : public fnvcoop::areadigest::IAreaObserver {
public:
    explicit FlightLink(fnvcoop::flight::FlightRecorder* rec = nullptr) : rec_(rec) {}
    void setRecorder(fnvcoop::flight::FlightRecorder* rec) { rec_ = rec; }
    void onHeal(const fnvcoop::areadigest::HealNote& n) override;
    void onBug(const fnvcoop::areadigest::BugReport& b) override;
    // Writes the per-category counters of `sync` (a null sync exports nothing).
    void exportCounters(const fnvcoop::areadigest::AreaSync* sync, uint64_t nowMs);
    uint64_t healsRecorded() const { return healsRecorded_; }
    uint64_t bugsRecorded() const { return bugsRecorded_; }
    // The newest heals as text ("actors key=N cell=N <before> -> <after>"), kept even without a recorder, so a failed lab room can name the healed key (areadigest.status lastHeals).
    const std::deque<std::string>& lastHeals() const { return lastHeals_; }
    static constexpr size_t kLastHeals = 8;
private:
    fnvcoop::flight::FlightRecorder* rec_;
    uint64_t healsRecorded_ = 0, bugsRecorded_ = 0;
    std::deque<std::string> lastHeals_;
};

struct ServiceInputs {
    IActorFacade* actors = nullptr;                              // null: no actor category (the actor layer is not running)
    std::function<bool(ShadowStorySource::World&)> world;        // the session's shared-world shadow, its applied seq and world epoch
    std::function<void(fnvcoop::PlayerId)> repair;               // host: ask the story layer to repair this client now (HostSession::requestDigest)
    std::function<uint64_t()> repairCount;                       // host: HostStats::desyncRepairs
    CellOfRef cellOf;                                            // ref -> cell for containers / objects (optional)
    fnvcoop::flight::FlightRecorder* flight = nullptr;
};

class AreaService {
public:
    AreaService(const AreaSettings& settings, ServiceInputs in);
    ~AreaService();
    AreaService(const AreaService&) = delete;
    AreaService& operator=(const AreaService&) = delete;

    void frame(const FrameInfo& f);
    const AreaSettings& settings() const { return set_; }
    AreaRuntime& runtime() { return rt_; }
    fnvcoop::areadigest::AreaSync* sync() { return rt_.sync(); }
    ActorAreaSource* actorSource() { return actors_.get(); }
    FlightLink& flightLink() { return link_; }
    std::string statusJson(uint64_t nowMs);
    // flight.dump hook: exports the counters now (the periodic export runs every 5 s from frame()).
    void exportFlight(uint64_t nowMs) { link_.exportCounters(rt_.sync(), nowMs); }

private:
    AreaSettings set_;
    ServiceInputs in_;
    FlightLink link_;
    std::unique_ptr<ActorAreaSource> actors_;
    std::unique_ptr<ShadowRefSource> containers_, objects_;
    ShadowStorySource story_;
    AreaRuntime rt_;
    uint64_t nextExportAt_ = 0;
};

}  // namespace nvcoop::areadigest
