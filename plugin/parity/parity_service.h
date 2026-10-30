// ParityService: the whole checker of one machine (game-free, unit-tested end to end with a fake engine source and an in-memory network).
//   every intervalMs   take a sample of the local view, append it to the sample file; a guest also sends every sendEvery-th one to the host (channel 0x9E)
//   host, on a sample  take a fresh sample of its own at once (so both describe nearly the same moment), write it too, compare, log the differences
// The glue (parity_module.cpp) only turns the coop Runtime into a FrameInfo and supplies the engine source. Read-only: nothing here changes the game.
#pragma once
#include <deque>
#include <functional>
#include <map>
#include <memory>
#include <string>
#include <vector>

#include "fnvcoop/transport_util.h"
#include "parity/parity_compare.h"
#include "parity/parity_config.h"
#include "parity/parity_json.h"
#include "parity/parity_sampler.h"
#include "parity/parity_wire.h"

namespace nvcoop::parity {

struct Link { uint8_t slot = kNoSlot; fnvcoop::PeerId peer = 0; bool joined = false; };

struct FrameInfo {
    bool active = false;                          // the co-op session is running
    bool isHost = false;
    uint8_t self = kNoSlot;                       // guest: the slot the host gave (kNoSlot until joined)
    bool hostLinkUp = false;                      // guest: joined and not in the baseline transfer
    std::vector<Link> clients;                    // host: the clients the session knows
    fnvcoop::MuxTransport* mux = nullptr;         // the session's mux; a new pointer = a new session
    uint64_t nowMs = 0;                           // monotonic
    uint64_t wallMs = 0;                          // unix time
};

struct ServiceStats {
    uint64_t samples = 0, sent = 0, received = 0, rejected = 0, droppedUnknownPeer = 0, rounds = 0, skipped = 0, linesLogged = 0, linesSuppressed = 0;
    uint64_t sessions = 0;
};

class ParityService {
public:
    // filePath "" = no sample file. log gets every line already prefixed with "parity: ".
    ParityService(const Settings& s, IParitySource& src, std::function<void(const std::string&)> log, const std::string& filePath);
    ~ParityService();
    ParityService(const ParityService&) = delete;
    ParityService& operator=(const ParityService&) = delete;

    void frame(const FrameInfo& f);
    bool takeNow(uint64_t nowMs, uint64_t wallMs, std::string& jsonLine);   // bridge: one sample now (not written to the file)
    const ServiceStats& stats() const { return st_; }
    const Settings& settings() const { return set_; }
    std::string statusJson() const;
    std::string lastSummary() const { return lastSummary_; }
    std::map<std::string, KindCount> kinds() const;
    RotatingFile& file() { return file_; }

private:
    struct Pending { uint8_t slot; Sample s; };
    void startSession(const FrameInfo& f);
    void endSession();
    void onDatagram(fnvcoop::PeerId from, fnvcoop::Bytes&& data);
    uint8_t slotOfPeer(fnvcoop::PeerId p) const;
    void tick(const FrameInfo& f);
    void compareAll(const FrameInfo& f);
    void write(const Sample& s, const LineMeta& m);

    Settings set_;
    IParitySource& src_;
    std::function<void(const std::string&)> log_;
    Sampler sampler_;
    RotatingFile file_;
    ServiceStats st_;
    std::shared_ptr<uint64_t> alive_;
    uint64_t gen_ = 0;
    fnvcoop::MuxTransport* mux_ = nullptr;
    bool up_ = false, isHost_ = false;
    uint8_t self_ = kNoSlot;
    std::vector<Link> clients_;
    uint64_t nextTickAt_ = 0, tickCount_ = 0;
    uint64_t lastNowMs_ = 0, lastWallMs_ = 0;
    std::deque<Pending> pending_;
    std::map<uint8_t, Comparer> comparers_;
    std::map<uint8_t, Reporter> reporters_;
    std::map<std::string, KindCount> kindTotals_;
    std::string lastSummary_;
};

}  // namespace nvcoop::parity
