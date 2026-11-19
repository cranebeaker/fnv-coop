#include "world_query.h"

#include <cstring>

namespace nvworld {

namespace {
// signature of the evaluators: cdecl bool(thisObj, p1, p2, double* result). Addresses: 4th field of the command table entry.
const Evaluator kEval[] = {
    {"GetRestrained", 0x59FF90, false},         {"IsEssential", 0x5A3980, false},    {"GetActorValue", 0x59C4F0, false},
    {"GetFactionRelation", 0x5A46D0, false},    {"GetLocked", 0x59D010, false},      {"GetLockLevel", 0x59D100, false},
    {"GetOpenState", 0x5A1C30, false},          {"GetDestroyed", 0x5A2330, false},   {"GetItemCount", 0x59D8E0, true},
    {"GetIgnoreCrime", 0x5A59A0, false},        {"GetPCEnemyofFaction", 0x5A21F0, false},
    {"GetActorFactionPlayerEnemy", 0x5A49F0, false},
    {"GetDead", 0x59D840, false},               // command table 0x1191040 (opcode 0x102E), read in game 2026-10-27
};

bool cleanPrologue(const unsigned char* b) { return b[0] == 0x55 && b[1] == 0x8B && b[2] == 0xEC; }
}  // namespace

const Evaluator* evaluators(size_t* count) {
    if (count) *count = sizeof kEval / sizeof kEval[0];
    return kEval;
}

const Evaluator* findEvaluator(const std::string& fn) {
    for (const Evaluator& q : kEval)
        if (fn == q.name) return &q;
    return nullptr;
}

EvalPlan resolveEvaluator(const Evaluator& q, const ReadMem& read, const TrampolineOf& trampolineOf) {
    EvalPlan p;
    unsigned char live[5] = {};
    if (!read || !read(q.addr, live, sizeof live)) { p.why = "evaluator entry unreadable"; return p; }
    if (cleanPrologue(live)) { p.kind = EvalCall::Live; p.addr = q.addr; return p; }
    if (live[0] != 0xE9) { p.why = "evaluator entry changed (not the original prologue, not a jump)"; return p; }
    // A jump: ours (the hook registry has its trampoline) or somebody else's.
    uint32_t t = trampolineOf ? trampolineOf(q.addr) : 0;
    if (FNV_MUT(45) && t == 0) {   // MUTANT 45: a foreign jump is called as if it were ours
        p.kind = EvalCall::Live; p.addr = q.addr; return p;
    }
    if (FNV_MUT(43)) t = 0;        // MUTANT 43: the trampoline is ignored (the old behaviour: every detoured evaluator is refused)
    if (t == 0) { p.why = "evaluator is detoured by someone other than us"; return p; }
    unsigned char orig[3] = {};
    if (!FNV_MUT(44)) {            // MUTANT 44: the trampoline is not checked for the original prologue
        if (!read(t, orig, sizeof orig) || !cleanPrologue(orig)) { p.why = "trampoline does not start with the original prologue"; return p; }
    }
    p.kind = EvalCall::Trampoline;
    p.addr = FNV_MUT(46) ? q.addr : t;   // MUTANT 46: calls the live (detoured) entry instead of the trampoline
    return p;
}

}  // namespace nvworld
