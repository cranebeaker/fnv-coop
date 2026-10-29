// The marker + revive service of ONE machine (game-free). It owns the board, the revive authority (host), the hold tracker (friend) and the wire
// between machines, and talks to the game only through IMarkerPort / IMarkerHud. plugin/markers/mk_glue.cpp implements both over the engine;
// the tests implement them with fakes and join several machines through an in-memory network.
//
// Per frame (tick): read the local player; send a beacon (guest) or build and send the party table (host); on the host, mirror the rules
// engine's downed list into the revive authority and carry out the revives it granted; compute the views and hand the HUD one frame.
//
// Mutation twins 1401..1440.
#pragma once
#include <functional>
#include <map>
#include <string>
#include <vector>

#include "mk_board.h"
#include "mk_types.h"
#include "mk_wire.h"
#include "rv_authority.h"
#include "rv_hold.h"

namespace nvmk {

struct ServiceConfig {
    MarkerConfig markers;
    ReviveRules revive;
    HoldConfig hold;
    uint32_t beaconMs = 500;       // a guest's place to the host
    uint32_t tableMs = 500;        // the host's table to the guests
};

// What the service needs from the game and the session.
class IMarkerPort {
public:
    virtual ~IMarkerPort() = default;
    virtual Slot localSlot() = 0;
    virtual bool isHost() = 0;
    virtual bool readLocal(LocalState& out) = 0;
    // The exact place of a loaded puppet (empty when the puppet is not loaded on this machine).
    virtual bool puppetPlace(Slot, Place&) { return false; }
    virtual std::vector<Slot> guests() { return {}; }                  // host: the joined guests
    virtual bool sendToHost(const std::string& json) = 0;              // guest
    virtual bool sendToGuest(Slot, const std::string& json) = 0;       // host
    virtual void notify(const std::string& text) = 0;                  // a line on this machine's HUD
    virtual void notifyGuest(Slot, const std::string& text) { (void)text; }   // host: a line on a guest's HUD
    // host: the death rules (the rules engine)
    virtual bool ruleDowned(Slot) { return false; }
    virtual bool ruleCheckpointOnly(Slot) { return false; }
    virtual uint32_t reviveWindowMs() { return 0; }                    // how long a downed player waits for a revive; 0 = no revive (death.mode respawn)
    // host: the rules engine forgets the death of `slot` and the player gets up where he lies with `healthFraction` of his health.
    virtual void reviveInPlace(Slot slot, float healthFraction, const std::string& byName) { (void)slot; (void)healthFraction; (void)byName; }
};

struct HudTag { Slot slot = kNoSlot; std::string text; float alpha = 0; Place at; };
struct HudCompassMark { Slot slot = kNoSlot; std::string glyph; float x = 0; int edge = 0; bool downed = false; };
struct HudMapMark { Slot slot = kNoSlot; std::string label; bool local = false; bool world = false; Place at; bool downed = false; };
struct HudFrame {
    std::vector<HudTag> tags;
    std::vector<HudCompassMark> compass;
    std::vector<HudMapMark> map;
    std::vector<std::string> list;     // "Alex, 340 m - Mojave Wasteland, sector 3,-2" for everybody (also what the notify fallback prints)
    std::string banner;                // the prompt, the revive progress, or the downed timer; "" = none
    float bannerProgress = -1;         // 0..1 while a revive is held, else < 0
    bool local = false;                // the local player is down
};

class IMarkerHud {
public:
    virtual ~IMarkerHud() = default;
    virtual void show(const HudFrame& f) = 0;
};

class MarkerService {
public:
    MarkerService(IMarkerPort& port, IMarkerHud* hud, ServiceConfig cfg = {});
    void setConfig(const ServiceConfig& c);
    const ServiceConfig& config() const { return cfg_; }

    void tick(uint64_t nowMs);
    // The order channel.
    std::string onHostOrder(Slot from, const std::string& json, uint64_t nowMs);   // host: "beacon" / "revive"; returns "Ok" or why not
    std::string onGuestOrder(const std::string& json, uint64_t nowMs);             // guest: "party"
    // This machine's own player was downed (the rules engine's order reached it) / got up. respawnMs 0 = no revive window.
    void onLocalDowned(uint32_t respawnMs, uint64_t nowMs);
    void onLocalUp(const std::string& byName = std::string());
    void onSessionEnd();                                                           // the session is gone or the world was reloaded

    // introspection
    const MarkerBoard& board() const { return board_; }
    const ReviveAuthority& authority() const { return auth_; }
    const HudFrame& lastFrame() const { return frame_; }
    bool localDowned() const { return localDowned_; }
    uint32_t localWindowLeftMs(uint64_t nowMs) const;
    struct Stats { uint64_t beaconsSent = 0, beaconsGot = 0, tablesSent = 0, tablesGot = 0, badOrders = 0, revivesGranted = 0, revivesRefused = 0, holdsBegun = 0, holdsDone = 0, holdsCancelled = 0; };
    const Stats& stats() const { return stats_; }
    Verdict lastVerdict() const { return lastVerdict_; }
    std::string statusJson(uint64_t nowMs) const;

private:
    struct Row { Beacon b; uint64_t seenMs = 0; };
    void sendBeacon(const LocalState& me, uint64_t now);
    void hostFrame(const LocalState& me, uint64_t now);
    std::vector<Member> hostTable(const LocalState& me, uint64_t now) const;
    void applyTable(const PartyTable& t, uint64_t now);
    void processRevived(uint64_t now);
    void reviveFrame(const LocalState& me, uint64_t now);
    void buildHud(const LocalState& me, uint64_t now);
    PlaceFn placeFn(const LocalState& me) const;
    void sendRevive(RevivePhase ph, Slot target, uint64_t now);
    std::string nameOf(Slot s) const;

    IMarkerPort& port_;
    IMarkerHud* hud_;
    ServiceConfig cfg_;
    MarkerBoard board_;
    ReviveAuthority auth_;
    ReviveHold hold_;
    std::map<Slot, Row> rows_;           // host: the last beacon of each guest
    uint64_t lastBeaconMs_ = 0, lastTableMs_ = 0;
    bool sentBeacon_ = false, sentTable_ = false, builtHud_ = false;
    uint32_t seq_ = 0, lastSeq_ = 0;     // seq_: the host's tables; lastSeq_: the newest table a guest has applied
    bool haveSeq_ = false;
    Member selfRow_;                     // what the host's table says about this machine's own player (downed, window, who is helping)
    uint64_t selfRowAt_ = 0;
    bool localDowned_ = false;
    uint64_t localDownedAt_ = 0;
    uint32_t localWindowMs_ = 0;
    HudFrame frame_;
    Stats stats_;
    Verdict lastVerdict_ = Verdict::Ok;
    Place meAt_;
    std::string myName_;
};

}  // namespace nvmk
