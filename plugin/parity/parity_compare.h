// Comparing what two machines see (game-free). The host holds its own sample taken the moment a guest's sample arrived and the guest's sample; this finds what
// disagrees. A difference only counts when it is not timing: positions get a tolerance that grows with speed, one-shot animations are not compared, and everything
// else must last persistMs before it is reported. tools/parity/compare.py runs the same rules over the two sample files of a run.
#pragma once
#include <cstdint>
#include <map>
#include <set>
#include <string>
#include <vector>

#include "parity/parity_config.h"
#include "parity/parity_types.h"

namespace nvcoop::parity {

// Kinds: pos seated anim dead hp look action absent disabled
struct Mismatch {
    std::string kind;
    std::string key;         // "0x0010A1B2" (an actor reference) or "player1" (a player slot)
    std::string host, client;
};

struct KindCount { uint64_t checks = 0, bad = 0; };

struct CompareResult {
    std::vector<Mismatch> confirmed;
    uint32_t refs = 0, refsBad = 0;                 // references (actors and players) compared this round / with at least one confirmed difference
    uint32_t players = 0, playersBad = 0;           // the same for player entities only
    std::vector<std::string> keysCompared, keysBad;
    bool skipped = false;                           // not comparable now (a menu is open, or the two players are in different places)
    std::string skipReason;
};

// How far apart two copies of the same moving entity may be: base + average speed * (skew + lag). The speeds are speed16 values (units/s / 16).
double posTolerance(const Tolerances& t, uint8_t speedA16, uint8_t speedB16);
bool sameArea(const Sample& a, const Sample& b);
double dist3(const Ent& a, const Ent& b);

class Comparer {
public:
    explicit Comparer(Tolerances t = {}) : tol_(t) {}
    CompareResult compare(const Sample& host, const Sample& client, uint64_t nowMs);
    const std::map<std::string, KindCount>& kinds() const { return kinds_; }
    const Tolerances& tolerances() const { return tol_; }
    void forget() { pending_.clear(); acts_.clear(); }

private:
    struct Pending { uint64_t first = 0, last = 0; };
    struct ActsState { bool have = false; int64_t base = 0; uint32_t lastH = 0, lastC = 0; };
    bool observe(const std::string& id, bool bad, bool hard, uint64_t now);
    void check(CompareResult& r, bool& anyBad, const char* kind, const std::string& key, bool bad, bool hard, uint64_t now, const std::string& host,
               const std::string& client);
    void comparePair(CompareResult& r, const Ent& h, const Ent& c, const std::string& key, bool players, uint64_t now);

    Tolerances tol_;
    std::map<std::string, Pending> pending_;
    std::map<uint8_t, ActsState> acts_;
    std::map<std::string, KindCount> kinds_;
};

// Rate limit and summary line (the host logs through this).
class Reporter {
public:
    explicit Reporter(Tolerances t = {}) : tol_(t) {}
    // The lines (without the "parity: " prefix) to log now for these confirmed differences: one per key and kind per logEveryMs. Also feeds the summary window.
    std::vector<std::string> report(const CompareResult& r, uint64_t nowMs);
    bool summaryDue(uint64_t nowMs) const { return nowMs >= nextSummary_ && (windowRefs_.size() || windowRounds_); }
    std::string summary(uint64_t nowMs);   // "summary N refs compared, M mismatching" and starts a new window
    uint64_t suppressed() const { return suppressed_; }
    uint64_t logged() const { return logged_; }

private:
    Tolerances tol_;
    std::map<std::string, uint64_t> lastLogged_;
    std::set<std::string> windowRefs_, windowBad_;
    uint64_t windowRounds_ = 0, nextSummary_ = 0, suppressed_ = 0, logged_ = 0;
    bool started_ = false;
};

}  // namespace nvcoop::parity
