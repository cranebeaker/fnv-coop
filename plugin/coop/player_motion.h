// Watches the position of THIS machine's own player (game-free) and says in the log every time it was moved by something other than the player's legs.
//
// Why. The 2026-10-30 playtest (0.1.6, host and guest copies on one PC): "on the CLIENT I was constantly running and RUBBER BANDING back". The log of that
// run had nothing about it: the mod logged no move and no hold of the local player, and the position of the guest's player was not in the log at all, so
// the party tether, the late-join warp, the baseline / scene control locks, a stock script's SetPos, a scripted AI package and the engine's own push of an
// avatar puppet could not be told apart. This class is the missing instrument:
//
//   * every displacement that is too fast for a walking, running or sprinting player (horizontal speed above kJumpSpeed and further than kJumpUnits in one
//     step) is one log line with where from, where to, in how long, in which cell, and whether the MOD did it (noteModMove was called shortly before: a warp,
//     a respawn, a join warp, a committed MoveRef) or not (the game: a script, a package, a door, the physics, a puppet standing in the way);
//   * the caller's `context` rides on every line (control bits, VCG01 stage, the distance to the other player's puppet): the next log says which one it was;
//   * three unattributed set-backs within a few seconds are one "RUBBER BAND" line (once per half minute): the symptom itself, with the numbers;
//   * a cell change is a line of its own (doors and travel are the usual cause of a big jump, and are not a set-back).
// The step runs on every frame of the game; it only reads. Lines are rate limited so a bad minute cannot flood the log.
//
// Mutation twins (FNVCOOP_MUT, tests/test_player_motion.cpp must fail for each): 1700 an unattributed jump is not logged   1701 a mod move is logged as unattributed
// 1702 a cell change is logged as a jump   1703 the rubber-band summary is never said   1704 the context is dropped from the line   1705 the rate limit is gone
#pragma once
#include <cmath>
#include <cstdint>
#include <cstdio>
#include <deque>
#include <string>
#include <type_traits>
#include <vector>

#include "fnvcoop_mut.h"

namespace nvcoop::coop::motion {

struct Config {
    float jumpUnits = 60.0f;          // a step shorter than this is never a jump (a few frames of a sprint)
    float jumpSpeed = 900.0f;         // horizontal units per second: far above a sprint (about 350), far below a teleport
    uint64_t attributeMs = 3000;      // a move the mod announced explains the jumps of this long after it (a parked MoveRef lands late)
    int tugJumps = 3;                 // this many unattributed jumps ...
    uint64_t tugWindowMs = 12000;     // ... within this long are a rubber band
    uint64_t tugRepeatMs = 30000;     // the summary is said at most this often
    int maxLinesPerMinute = 24;       // the log is not for a flood
};

struct Sample {
    uint64_t nowMs = 0;
    uint32_t cell = 0, world = 0;     // 0 = unknown
    float x = 0, y = 0, z = 0;
};

class MotionWatch {
public:
    explicit MotionWatch(Config c = {}) : cfg_(c) {}

    // The mod is about to move (or has just moved) the local player: `what` is the log text of the move ("late join warp", "respawn at the checkpoint").
    // The jump that follows is then reported as the mod's, with this text.
    void noteModMove(uint64_t nowMs, const std::string& what) {
        modAt_ = nowMs;
        modWhat_ = what;
        haveMod_ = true;
    }

    // `context` is a string or a callable returning one: it is only evaluated when a line is written (the step itself runs on every frame).
    template <class Ctx>
    std::vector<std::string> step(const Sample& s, Ctx&& contextArg) {
        std::string contextText;
        bool haveContext = false;
        auto contextOf = [&]() -> const std::string& {
            if (!haveContext) {
                haveContext = true;
                if constexpr (std::is_invocable_v<Ctx&>) contextText = contextArg();
                else contextText = std::string(contextArg);
            }
            return contextText;
        };
        std::vector<std::string> out;
        if (!have_) {
            have_ = true;
            prev_ = s;
            return out;
        }
        const bool sameSpace = s.cell == prev_.cell && s.world == prev_.world;
        const Sample p = prev_;
        prev_ = s;
        if (!sameSpace) {
            if (!FNV_MUT(1702)) {
                say(out, s.nowMs, "coop: local player changed cell " + hex(p.cell) + " -> " + hex(s.cell) + " at " + pos(s) + (contextOf().empty() || FNV_MUT(1704) ? std::string() : " [" + contextOf() + "]"));
                return out;
            }
        }
        const float dx = s.x - p.x, dy = s.y - p.y, dz = s.z - p.z;
        const float xy = std::sqrt(dx * dx + dy * dy);
        const uint64_t dt = s.nowMs > p.nowMs ? s.nowMs - p.nowMs : 1;
        const float speed = xy * 1000.0f / static_cast<float>(dt);
        if (xy < cfg_.jumpUnits || speed < cfg_.jumpSpeed) return out;
        const bool byMod = haveMod_ && s.nowMs >= modAt_ && s.nowMs - modAt_ <= cfg_.attributeMs && !FNV_MUT(1701);
        if (!byMod && FNV_MUT(1700)) return out;
        char b[256];
        std::snprintf(b, sizeof b, "coop: local player %s %.0f units in %llu ms (%.0f u/s, rise %.0f) from %s to %s", byMod ? "MOVED by the mod:" : "JUMPED, not by the mod's own moves:", xy,
                      (unsigned long long)dt, speed, dz, pos(p).c_str(), pos(s).c_str());
        std::string line = b;
        if (byMod) line += " (" + modWhat_ + ")";
        if (!contextOf().empty() && !FNV_MUT(1704)) line += " [" + contextOf() + "]";
        say(out, s.nowMs, line);
        if (!byMod) {
            jumps_.push_back(s.nowMs);
            while (!jumps_.empty() && s.nowMs - jumps_.front() > cfg_.tugWindowMs) jumps_.pop_front();
            if (static_cast<int>(jumps_.size()) >= cfg_.tugJumps && !FNV_MUT(1703) && (!tugSaid_ || s.nowMs - tugAt_ >= cfg_.tugRepeatMs)) {
                tugSaid_ = true;
                tugAt_ = s.nowMs;
                std::snprintf(b, sizeof b, "coop: RUBBER BAND: the local player was set back %d times in %.1f s and none of it was the mod's own move",
                              static_cast<int>(jumps_.size()), static_cast<double>(s.nowMs - jumps_.front()) / 1000.0);
                line = b;
                if (!contextOf().empty() && !FNV_MUT(1704)) line += " [" + contextOf() + "]";
                out.push_back(line);   // the summary is never held back by the line limit
            }
        }
        return out;
    }

    int jumpsInWindow() const { return static_cast<int>(jumps_.size()); }

private:
    static std::string hex(uint32_t v) {
        char b[16];
        std::snprintf(b, sizeof b, "%X", (unsigned)v);
        return b;
    }
    static std::string pos(const Sample& s) {
        char b[64];
        std::snprintf(b, sizeof b, "(%.0f,%.0f,%.0f)", s.x, s.y, s.z);
        return b;
    }
    void say(std::vector<std::string>& out, uint64_t nowMs, const std::string& line) {
        if (!FNV_MUT(1705)) {
            while (!said_.empty() && nowMs - said_.front() > 60000) said_.pop_front();
            if (static_cast<int>(said_.size()) >= cfg_.maxLinesPerMinute) return;
            said_.push_back(nowMs);
        }
        out.push_back(line);
    }

    Config cfg_;
    bool have_ = false, haveMod_ = false, tugSaid_ = false;
    Sample prev_;
    uint64_t modAt_ = 0, tugAt_ = 0;
    std::string modWhat_;
    std::deque<uint64_t> jumps_, said_;
};

}  // namespace nvcoop::coop::motion
