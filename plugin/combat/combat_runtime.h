// CombatRuntime: owns the CombatSync of the running co-op session and plugs it into the coop runtime's MuxTransport (first byte kChannelCombat).
// Game-free, the same shape as plugin/actors ActorRuntime: the glue (or a test) calls frame() every frame with the facts it knows; this class builds /
// drops the CombatSync when the session starts / ends, maps slots to transport peers, forwards datagrams and reports joins / leaves.
#pragma once
#include <memory>
#include <set>
#include <vector>
#include "combat/combat_sync.h"
#include "fnvcoop/transport_util.h"

namespace nvcoop::combat {

struct ClientLink { PlayerId slot = kNoPlayer; fnvcoop::PeerId peer = 0; bool joined = false; };

struct FrameInfo {
    bool active = false;                  // the coop session is running
    bool isHost = false;
    PlayerId self = kNoPlayer;            // client: slot assigned by the host (kNoPlayer until joined)
    bool hostLinkUp = false;              // client: joined and not rebaselining
    std::vector<ClientLink> clients;      // host: clients known to the session
    fnvcoop::MuxTransport* mux = nullptr; // the session's mux; a new pointer = a new session
    uint64_t nowMs = 0;
};

class CombatRuntime {
public:
    CombatRuntime(IWorldView& world, ICombatEngine& engine);
    ~CombatRuntime();
    CombatRuntime(const CombatRuntime&) = delete;
    CombatRuntime& operator=(const CombatRuntime&) = delete;

    void frame(const FrameInfo& f);
    CombatSync* sync() { return sync_.get(); }
    const CombatSync* sync() const { return sync_.get(); }
    Config& config() { return cfg_; }       // set before the session starts (rules, friendly fire, xp mode, crime sync)
    void setFriendlyFire(bool on) { cfg_.friendlyFire = on; if (sync_) sync_->setFriendlyFire(on); }   // the campaign's choice, any time (plugin/rebalance)
    uint64_t rebuilds() const { return rebuilds_; }
    uint64_t droppedUnknownPeer() const { return droppedUnknownPeer_; }

private:
    class MuxNet;
    void teardown();
    PlayerId slotOfPeer(fnvcoop::PeerId p) const;

    IWorldView& world_;
    ICombatEngine& engine_;
    Config cfg_;
    std::shared_ptr<bool> alive_;
    std::unique_ptr<MuxNet> net_;
    std::unique_ptr<CombatSync> sync_;
    fnvcoop::MuxTransport* mux_ = nullptr;
    bool isHost_ = false;
    PlayerId self_ = kNoPlayer;
    bool linkUp_ = false;
    std::vector<ClientLink> clients_;
    std::set<PlayerId> known_;
    uint64_t rebuilds_ = 0, droppedUnknownPeer_ = 0;
};

}  // namespace nvcoop::combat
