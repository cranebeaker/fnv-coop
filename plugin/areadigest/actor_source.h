// The actors category of the area digest: reads the synced actors of this machine and applies heals to them.
//
// Everything engine-facing is behind IActorFacade (engine_facade.cpp is the real one over plugin/actors/engine; tests use a fake). The rules
// the source adds on top of the core:
//   * a heal changes position (snap now / smooth over a few hundred ms), health and, never directly, death;
//   * death: "the authority says dead, this copy is alive" is handed back to the actor module (requestOwnerResync), which owns the sanctioned
//     death path (owner declares it, a named NPC only through the story KillActor). The reverse (alive at the authority, dead here) is a
//     resurrect, which is a story effect: refused;
//   * a named / essential actor is never killed or revived from here;
//   * a smoothing job is dropped when its actor goes away, dies, or another heal replaces it.
// Status: unit-tested with a fake facade; the real facade has never run in a game.
#pragma once
#include <map>
#include <vector>
#include "areadigest/areadigest.h"

namespace nvcoop::areadigest {

using fnvcoop::areadigest::Cat;
using fnvcoop::areadigest::CellId;
using fnvcoop::areadigest::Config;
using fnvcoop::areadigest::Diff;
using fnvcoop::areadigest::HealResult;
using fnvcoop::areadigest::Item;
using fnvcoop::areadigest::Key;
using fnvcoop::areadigest::OwnerView;
using fnvcoop::areadigest::Vec3;

struct ActorRead {
    uint32_t netId = 0;
    bool standIn = false;
    bool named = false;        // story / unique / essential: never killed or revived from here
    bool alive = true;
    CellId cell = 0;
    float hp = 0;
    Vec3 pos;
};

class IActorFacade {
public:
    virtual ~IActorFacade() = default;
    virtual bool ready() = 0;                                              // a world is loaded and the actor layer runs
    virtual void tracked(std::vector<ActorRead>& out, size_t max) = 0;     // synced actors with a loaded local reference (real or stand-in)
    virtual bool read(uint32_t netId, ActorRead& out) = 0;
    virtual bool owner(uint32_t netId, OwnerView& out) = 0;
    virtual bool playerCell(CellId& out) = 0;                              // the local player's cell (a loaded area even when no actor is in it)
    virtual bool setHealth(uint32_t netId, float hp) = 0;
    virtual bool setPos(uint32_t netId, const Vec3& pos) = 0;              // same cell
    virtual bool moveToCell(uint32_t netId, CellId cell, const Vec3& pos) = 0;
    // Ask the actor module for the owner's full state of this actor (the sanctioned way to learn a death). false = not available.
    virtual bool requestOwnerResync(uint32_t netId) = 0;
    // The copy is being held in place while it gets out of furniture (the actor port's pose lock): a position heal would drag the seated body across the room.
    virtual bool moveHeld(uint32_t /*netId*/) { return false; }
};

class ActorAreaSource : public fnvcoop::areadigest::IAreaSource {
public:
    ActorAreaSource(IActorFacade& f, const Config& cfg, size_t maxActors = 400);

    Cat category() const override { return Cat::Actors; }
    void collect(uint64_t nowMs, std::vector<Item>& items, std::vector<CellId>& loadedCells) override;
    bool find(Key key, Item& out, uint64_t nowMs) override;
    HealResult heal(const Item& authoritative, const Diff& plan, uint64_t nowMs) override;
    bool owner(Key key, OwnerView& out) override;
    void tick(uint64_t nowMs) override;

    size_t smoothing() const { return smooth_.size(); }
    struct Stats {
        uint64_t snaps = 0, smooths = 0, smoothSteps = 0, hpSet = 0, deathRequests = 0, refused = 0, missing = 0, moveFailed = 0, scans = 0;
        uint64_t capped = 0, heldByPoseLock = 0;
    };
    const Stats& stats() const { return st_; }
    void setConfig(const Config& c) { cfg_ = c; tr_.setConfig(c); }

private:
    IActorFacade& f_;
    Config cfg_;
    size_t maxActors_;
    fnvcoop::areadigest::ChangeTracker tr_;
    std::map<uint32_t, fnvcoop::areadigest::Smoother> smooth_;
    Stats st_;
};

}  // namespace nvcoop::areadigest
