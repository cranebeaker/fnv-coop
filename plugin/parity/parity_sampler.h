// Takes one machine's Sample from an engine source (game-free; the dll side is parity_engine.cpp, the tests use a fake). Read-only: the source interface has no
// way to change the game. The sampler also counts attack animations starting on the local player and on every puppet, frame by frame, because a punch lasts
// less than a sample interval.
#pragma once
#include <cstdint>
#include <deque>
#include <map>
#include <utility>
#include <vector>

#include "parity/parity_config.h"
#include "parity/parity_types.h"

namespace nvcoop::parity {

struct RawPlayer {
    float x = 0, y = 0, z = 0;
    uint32_t cell = 0, ws = 0;
    bool menu = false;
};

struct RawActor {
    uint32_t ref = 0, base = 0;
    float x = 0, y = 0, z = 0;
    bool disabled = false;
};

struct RawState {
    bool ok = false;                 // the actor has animation data
    std::vector<int> groups;         // every animation group id playing (all AnimData sets, -1 entries dropped)
    bool dead = false;
    bool hpOk = false;
    float hp = 0;
    bool seatedOk = false;
    bool seated = false;
    bool weaponOut = false;
};

struct RawLook {
    bool ok = false;
    Look look;
    std::string name;
};

class IParitySource {
public:
    virtual ~IParitySource() = default;
    virtual bool ready() = 0;                                              // a world is loaded and the player exists
    virtual bool player(RawPlayer& out) = 0;
    virtual uint32_t playerRef() { return 0x14; }
    virtual void actors(std::vector<RawActor>& out) = 0;                   // every actor in the loaded cells (the player and the puppets included)
    virtual bool actor(uint32_t ref, RawActor& out) = 0;                   // one reference
    virtual bool groups(uint32_t ref, bool isPlayer, std::vector<int>& out) = 0;   // cheap: the animation groups only
    virtual bool state(uint32_t ref, bool isPlayer, RawState& out) = 0;
    virtual bool look(uint32_t ref, RawLook& out) = 0;
    virtual void puppets(std::vector<std::pair<uint8_t, uint32_t>>& out) = 0;   // (player slot, puppet reference), live ones only
    virtual uint8_t localSlot() = 0;
    virtual double gameTime() = 0;                                         // GameDaysPassed
};

class Sampler {
public:
    Sampler(const Settings& s, IParitySource& src) : set_(s), src_(src) {}
    // Every frame: watches the animation groups of the local player and the puppets (throttled to about 30 Hz).
    void frame(uint64_t nowMs);
    // One sample. False when the world is not up.
    bool take(uint64_t nowMs, uint64_t wallMs, Sample& out);
    uint32_t seq() const { return seq_; }
    uint32_t attacksOf(uint8_t slot) const;

private:
    struct Track { uint8_t prev[256] = {}; uint32_t count = 0; };
    struct Pt { float x, y, z; uint64_t ms; };
    uint8_t speedOf(uint32_t key, float x, float y, float z, uint64_t nowMs);
    void count(uint8_t slot, const std::vector<int>& groups);

    Settings set_;
    IParitySource& src_;
    uint32_t seq_ = 0;
    uint64_t lastFrameMs_ = 0;
    std::map<uint8_t, Track> tracks_;
    std::map<uint32_t, std::deque<Pt>> history_;
};

// The part of a sample that fits the datagram: the first netCap Npc entries and every player entity; `cov` shrinks to the distance of the nearest dropped one.
Sample netSample(const Sample& s, size_t netCap);

}  // namespace nvcoop::parity
