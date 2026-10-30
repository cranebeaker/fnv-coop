// T7 animprobe, game-free core: sample structs, JSON encoding, request parsing, probe cost statistics.
// The engine reads live in animprobe_game.cpp; nothing here touches the game, so it is unit tested without it.
//
// One probe result (the "anim.probe" reply) looks like:
//   {"v":1,"frame":N,"t_ms":F,"cost_us":F,"cell":"0x..","actors":[ <ActorSample> ... ],"skipped":[...]}
// Field meanings are documented in docs/tools/T7-anim.md.
#pragma once
#include <cstdint>
#include <string>
#include <utility>
#include <vector>

#include "common/minijson.h"

namespace animprobe {

constexpr int kSchemaVersion = 1;
constexpr int kMaxSeqSlots = 9;      // AnimData::animSequence[8] + animSeq100
constexpr int kMaxActorsPerProbe = 64;

struct Vec3 {
    float x = 0, y = 0, z = 0;
};

// One slot of AnimData::animSequence (a playing or last-played BSAnimGroupSequence).
struct SeqSample {
    int slot = 0;        // index in AnimData::animSequence, 8 = animSeq100
    int group = -1;      // TESAnimGroup::animGroup (base group id; names in tools/animcheck/animgroups.py)
    uint32_t seqPtr = 0, grpPtr = 0;   // dev (scan_animdata): the sequence object and the pointer read at +0x68, to locate the group in game
    int state = -1;      // NiControllerSequence::state (0 inactive, 1 animating, others unknown)
    float t = 0;         // NiControllerSequence::last (time of the last update, in sequence seconds)
    float begin = 0, end = 0;
    float freq = 1, weight = 0;
};

struct ActorSample {
    uint32_t ref = 0;
    uint32_t base = 0;
    bool isPlayer = false;
    bool valid3d = false;
    bool firstPerson = false;
    bool dead = false;
    int lifeState = -1;
    Vec3 pos;
    Vec3 rot;            // radians, as stored on the reference (rotX, rotY, rotZ)
    uint32_t cell = 0;

    // animation
    bool haveAnim = false;
    int action = -1;             // HighProcess::GetCurrentAnimAction
    uint32_t sequenceRaw = 0;    // HighProcess::GetCurrentSequence (raw dword, meaning unverified)
    std::vector<SeqSample> seqs;
    // The LOCAL PLAYER only: his other AnimData objects (PlayerCharacter+0x690 first person, +0x6A0 alternate). A player in first person plays his swings and shots on the first-person
    // set, not on the process's (`seqs`), so a probe that reads only `seqs` never sees a human's attack. Emitted as anim.seqs_first_person / anim.seqs_alt when non-empty.
    std::vector<SeqSample> fpSeqs, altSeqs;

    // locomotion (raw engine flags; bits decoded by animcheck)
    bool haveMove = false;
    uint32_t moveFlags = 0;      // ActorMover::GetMovementFlags (bit7 walk, bit8 run, bit9 sneak, bit11 swim)

    // weapon
    bool weaponOut = false;
    bool aiming = false;
    uint32_t weaponForm = 0;

    // stance
    int knocked = 0;             // BaseProcess::GetKnockedState
    int sitSleep = 0;            // BaseProcess::GetSitSleepState
    bool swimming = false;       // Actor+0x14D
    bool ragdollCtrl = false;    // Actor::ragDollController != null

    // AI
    bool inCombat = false;       // GetCombatTarget() != null
    uint32_t combatTarget = 0;
    uint32_t package = 0;

    // skeleton
    std::vector<std::pair<std::string, Vec3>> bones;
    int nodesVisited = 0;
};

// Bones sampled by default. Names are the New Vegas human skeleton; missing names are simply left out
// (creatures use other names, the request can override the list with "bone_names").
extern const char* const kDefaultBones[];
extern const int kDefaultBoneCount;

struct Request {
    std::vector<uint32_t> refs;      // explicit refs
    bool includePlayer = false;
    bool nearby = false;
    float nearRadius = 2000.f;
    bool bones = true;
    bool scanAnimData = false;       // dev: report which AnimData dwords look like sequences
    int maxActors = 16;
    std::vector<std::string> boneNames;  // empty = defaults
};

// Parses the bridge params. Returns false and fills err for malformed input (refs that are not hex/number...).
bool ParseRequest(const std::string& paramsJson, Request& out, std::string& err);

// "0x00ABCDEF" style
std::string HexId(uint32_t v);

nvcoop::json::Value ToJson(const ActorSample& a);

// Distance filter used by "near": keep at most maxActors, nearest first. Returns indices into `distSq`.
std::vector<int> SelectNearest(const std::vector<float>& distSq, float radius, int maxActors);

// Cost statistics over the last probes (microseconds per probe call). Thread-safe enough for main-thread use.
class CostStats {
public:
    void Add(double us, int actors);
    nvcoop::json::Value ToJson() const;
    uint64_t count() const { return count_; }
    double Percentile(double p) const;  // 0..1 over the retained window
private:
    static constexpr int kWindow = 256;
    double us_[kWindow] = {};
    int actors_[kWindow] = {};
    uint64_t count_ = 0;
    double max_ = 0, sum_ = 0;
};

}  // namespace animprobe
