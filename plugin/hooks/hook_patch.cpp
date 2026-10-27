#include "hook_patch.h"

#include <atomic>
#include <cstdlib>
#include <cstring>

namespace nvhooks {

// NVHOOKS_MUTANT builds re-introduce classic bugs; tests/test_hooks.cpp must fail against them.
#ifdef NVHOOKS_MUTANT
constexpr bool kMutSkipPrologueCheck = true;  // patch without verifying the bytes
constexpr bool kMutRefcountIgnored = true;    // first release restores even if other users remain
constexpr bool kMutNoLiveCount = true;        // live counter never decremented (leak accounting lies)
constexpr bool kMutSuspendNoop = true;        // suspendAll claims success but leaves the patch in place
constexpr bool kMutDtorKeepsHooks = true;     // destructor forgets the hooks instead of restoring them
constexpr bool kMutTrampolineOfLive = true;    // trampolineOf answers the hooked entry itself (a caller would run our detour, not the original)
#else
constexpr bool kMutSkipPrologueCheck = false;
constexpr bool kMutRefcountIgnored = false;
constexpr bool kMutNoLiveCount = false;
constexpr bool kMutSuspendNoop = false;
constexpr bool kMutDtorKeepsHooks = false;
constexpr bool kMutTrampolineOfLive = false;
#endif

// Chaining twins (rule 4), picked at run time from NVHOOKS_CHAIN_MUT, compiled in only for the nvcoop_hooks_core_twins test library
// (plugin/hooks/CMakeLists.txt): 1 = the foreign jump is copied into the trampoline without relocating it, 2 = a foreign jump is
// chained over even when the caller did not opt in, 3 = the rest of the signature is not checked before chaining.
#ifdef NVHOOKS_CHAIN_TWINS
static int chainMut() {
    static const int v = [] {
        const char* e = std::getenv("NVHOOKS_CHAIN_MUT");
        return e ? std::atoi(e) : 0;
    }();
    return v;
}
#else
static constexpr int chainMut() { return 0; }
#endif

namespace {
std::mutex g_regsMu;
std::vector<HookRegistry*>& allRegs() {
    static std::vector<HookRegistry*>* v = new std::vector<HookRegistry*>();
    return *v;
}
std::atomic<int> g_live{0};
std::atomic<WriteGuardBegin> g_guardBegin{nullptr};
std::atomic<WriteGuardEnd> g_guardEnd{nullptr};
struct GuardScope {
    GuardScope(Addr target, uint8_t stolen, Addr tramp, bool installing) {
        if (auto b = g_guardBegin.load()) { b(target, stolen, tramp, installing); active = true; }
    }
    ~GuardScope() {
        if (active)
            if (auto e = g_guardEnd.load()) e();
    }
    bool active = false;
};
std::atomic<int> g_tampered{0};
}  // namespace
void setWriteGuard(WriteGuardBegin b, WriteGuardEnd e) {
    g_guardBegin = b;
    g_guardEnd = e;
}
int liveHookCount() { return g_live.load(); }
int tamperedCount() { return g_tampered.load(); }

const char* statusName(PatchStatus s) {
    switch (s) {
        case PatchStatus::Ok: return "Ok";
        case PatchStatus::AlreadyInstalled: return "AlreadyInstalled";
        case PatchStatus::PrologueMismatch: return "PrologueMismatch";
        case PatchStatus::AlreadyHooked: return "AlreadyHooked";
        case PatchStatus::BadSpec: return "BadSpec";
        case PatchStatus::ReadFailed: return "ReadFailed";
        case PatchStatus::WriteFailed: return "WriteFailed";
        case PatchStatus::AllocFailed: return "AllocFailed";
        case PatchStatus::NotInstalled: return "NotInstalled";
        case PatchStatus::Tampered: return "Tampered";
        case PatchStatus::Released: return "Released";
        case PatchStatus::Restored: return "Restored";
    }
    return "?";
}

bool specIsValid(const HookSpec& s, std::string* why) {
    auto fail = [&](const char* m) {
        if (why) *why = m;
        return false;
    };
    if (!s.target) return fail("null target");
    if (s.stolen < 5 || s.stolen > 16) return fail("stolen must be 5..16");
    if (s.expectedLen < s.stolen || s.expectedLen > 16) return fail("expectedLen must cover stolen bytes");
    if (!(s.expected[0] == 0x55 && s.expected[1] == 0x8B && s.expected[2] == 0xEC))
        return fail("only standard-frame function entries (55 8B EC) may be hooked");
    return true;
}

static void putRel32(uint8_t* p, Addr from /*addr of the E9 byte*/, Addr to) {
    uint32_t rel = to - (from + 5);
    p[0] = 0xE9;
    std::memcpy(p + 1, &rel, 4);
}

// Rule 4: is `cur` (the live entry bytes) another patcher's `jmp rel32` laid over this function? The bytes after the stolen region
// must still be the function's own; the bytes between the jump and `stolen` may be the original ones or NOP padding a detour left
// behind. Returns the jump target, or 0.
static Addr foreignJmpTarget(const HookSpec& spec, const uint8_t* cur, Addr detour) {
    if (cur[0] != 0xE9) return 0;
    if (chainMut() != 3) {
        for (unsigned i = spec.stolen; i < spec.expectedLen; ++i)
            if (cur[i] != spec.expected[i]) return 0;
        for (unsigned i = 5; i < spec.stolen; ++i)
            if (cur[i] != spec.expected[i] && cur[i] != 0x90) return 0;
    }
    uint32_t rel;
    std::memcpy(&rel, cur + 1, 4);
    const Addr to = spec.target + 5 + rel;
    if (to == detour) return 0;                                              // our own jump (bookkeeping lost): never chain to ourselves
    if (to >= spec.target && to < spec.target + spec.expectedLen) return 0;  // a jump into the patched bytes is not a detour
    return to;
}

// The expected bytes past the stolen ones only fingerprint the exe build. Another plugin (ShowOff, Stewie's Tweaks under
// Tale of Two Wastelands) may have re-pointed a call or jump there; that leaves the opcode in place and moves the target
// out of the exe image. Such a difference is accepted; a target inside the image (another exe build) is not.
static bool prologueMatches(const HookSpec& spec, const uint8_t* cur, MemoryOps& mem) {
    for (unsigned i = 0; i < spec.expectedLen; ++i) {
        if (cur[i] == spec.expected[i]) continue;
        if (i < spec.stolen) return false;
        // i must be a rel32 byte of an E8/E9 that starts past the stolen bytes and is still there
        unsigned op = 0;
        bool found = false;
        for (unsigned j = i; j-- > spec.stolen && j + 4 >= i;) {
            if ((spec.expected[j] == 0xE8 || spec.expected[j] == 0xE9) && cur[j] == spec.expected[j]) {
                op = j;
                found = true;
                break;
            }
        }
        uint32_t rel;   // read from memory: the operand may run past the expected bytes (E8 at index 12 of 16)
        if (!found || !mem.read(spec.target + op + 1, &rel, 4)) return false;
        if (!mem.foreignCode(spec.target + op + 5 + rel)) return false;
        i = op + 4;   // the whole rel32 is accounted for
    }
    return true;
}

PatchStatus HookRegistry::acquire(const HookSpec& spec, Addr detour, Addr* trampolineOut, Addr* origSlot, bool chainForeignJmp) {
    std::string why;
    if (!specIsValid(spec, &why)) {
        lastError_ = spec.name + std::string(": ") + why;
        return PatchStatus::BadSpec;
    }
    auto it = hooks_.find(spec.target);
    if (it != hooks_.end()) {
        if (it->second.detour != detour) {
            lastError_ = std::string(spec.name) + ": already hooked with another detour";
            return PatchStatus::BadSpec;
        }
        ++it->second.refs;
        if (trampolineOut) *trampolineOut = it->second.trampoline;
        return PatchStatus::AlreadyInstalled;
    }

    uint8_t cur[16] = {};
    if (!mem_.read(spec.target, cur, spec.expectedLen)) {
        lastError_ = std::string(spec.name) + ": read failed";
        return PatchStatus::ReadFailed;
    }
    Addr chainTo = 0;
    if (!kMutSkipPrologueCheck && !prologueMatches(spec, cur, mem_)) {
        if (chainForeignJmp || chainMut() == 2) chainTo = foreignJmpTarget(spec, cur, detour);
        if (!chainTo) {
            lastError_ = std::string(spec.name) + ": prologue mismatch (wrong exe version or already patched)";
            return (cur[0] == 0xE9 || cur[0] == 0xE8) ? PatchStatus::AlreadyHooked : PatchStatus::PrologueMismatch;
        }
    }
    // Chained (rule 4): only the foreign 5-byte jump is ours to replace; whatever follows it stays as the other patcher left it.
    const uint8_t stolen = chainTo ? 5 : spec.stolen;

    Installed h;
    h.detour = detour;
    h.stolen = stolen;
    h.chainedTo = chainTo;
    std::memcpy(h.original, cur, 16);
    h.trampSize = stolen + 5u;
    h.trampoline = mem_.allocExec(h.trampSize);
    if (!h.trampoline) {
        lastError_ = std::string(spec.name) + ": trampoline alloc failed";
        return PatchStatus::AllocFailed;
    }

    // Build + write the trampoline first: if anything fails past this point the game code is still untouched.
    uint8_t tramp[32] = {};
    std::memcpy(tramp, cur, stolen);
    if (chainTo && chainMut() != 1) putRel32(tramp, h.trampoline, chainTo);   // their jump, relocated to run from the trampoline
    putRel32(tramp + stolen, h.trampoline + stolen, spec.target + stolen);
    if (!mem_.write(h.trampoline, tramp, h.trampSize)) {
        mem_.freeExec(h.trampoline, h.trampSize);
        lastError_ = std::string(spec.name) + ": trampoline write failed";
        return PatchStatus::WriteFailed;
    }

    std::memset(h.patch, 0x90, sizeof h.patch);
    putRel32(h.patch, spec.target, detour);
    if (origSlot) *origSlot = h.trampoline;
    bool patched;
    {
        GuardScope gs(spec.target, stolen, h.trampoline, true);
        patched = mem_.write(spec.target, h.patch, stolen);
    }
    if (!patched) {
        if (origSlot) *origSlot = 0;
        mem_.freeExec(h.trampoline, h.trampSize);
        lastError_ = std::string(spec.name) + ": patch write failed";
        return PatchStatus::WriteFailed;
    }
    h.refs = 1;
    if (trampolineOut) *trampolineOut = h.trampoline;
    hooks_[spec.target] = h;
    ++g_live;
    return PatchStatus::Ok;
}

PatchStatus HookRegistry::release(const HookSpec& spec) {
    auto it = hooks_.find(spec.target);
    if (it == hooks_.end()) return PatchStatus::NotInstalled;
    Installed& h = it->second;
    if (!kMutRefcountIgnored && --h.refs > 0) return PatchStatus::Released;

    uint8_t cur[16] = {};
    // A suspended hook already has the original bytes back: "ours" then means the entry still equals the original.
    const uint8_t* want = h.suspended ? h.original : h.patch;
    bool ours = mem_.read(spec.target, cur, h.stolen) && std::memcmp(cur, want, h.stolen) == 0;
    if (!ours) {
        // Another patcher chained over us. Restoring would clobber theirs and leave their jump pointing into a
        // freed trampoline, so keep the trampoline alive and leave the code alone.
        lastError_ = std::string(spec.name) + ": bytes changed since install; not restoring";
        hooks_.erase(it);
        if (!kMutNoLiveCount) --g_live;
        ++g_tampered;
        return PatchStatus::Tampered;
    }
    bool restoredOk = true;
    if (!h.suspended) {
        GuardScope gs(spec.target, h.stolen, h.trampoline, false);
        restoredOk = mem_.write(spec.target, h.original, h.stolen);
    }
    if (!restoredOk) {
        lastError_ = std::string(spec.name) + ": restore failed";
        return PatchStatus::WriteFailed;
    }
    // Read back: a write that "succeeded" but did not land would leave the game jumping into freed memory after a reload.
    uint8_t back[16] = {};
    if (!mem_.read(spec.target, back, h.stolen) || std::memcmp(back, h.original, h.stolen) != 0) {
        lastError_ = std::string(spec.name) + ": restore did not stick (read-back differs)";
        return PatchStatus::WriteFailed;  // trampoline kept, entry kept: caller sees it is still live
    }
    mem_.freeExec(h.trampoline, h.trampSize);
    hooks_.erase(it);
    if (!kMutNoLiveCount) --g_live;
    return PatchStatus::Restored;
}

int HookRegistry::refCount(const HookSpec& spec) const {
    auto it = hooks_.find(spec.target);
    return it == hooks_.end() ? 0 : it->second.refs;
}

HookRegistry::HookRegistry(MemoryOps& mem) : mem_(mem) {
    std::lock_guard<std::mutex> g(g_regsMu);
    allRegs().push_back(this);
}

int HookRegistry::suspendAll() {
    int bad = 0;
    for (auto& [target, h] : hooks_) {
        if (h.suspended) continue;
        uint8_t cur[16] = {};
        if (kMutSuspendNoop) { h.suspended = true; continue; }
        bool ok = mem_.read(target, cur, h.stolen) && std::memcmp(cur, h.patch, h.stolen) == 0;
        if (ok) {
            GuardScope gs(target, h.stolen, h.trampoline, false);
            ok = mem_.write(target, h.original, h.stolen);
        }
        if (!ok) {
            ++bad;  // chained over or unwritable: left alone
            continue;
        }
        h.suspended = true;
    }
    return bad;
}

int HookRegistry::resumeAll() {
    int bad = 0;
    for (auto& [target, h] : hooks_) {
        if (!h.suspended) continue;
        bool ok;
        {
            GuardScope gs(target, h.stolen, h.trampoline, true);
            ok = mem_.write(target, h.patch, h.stolen);
        }
        if (!ok) { ++bad; continue; }
        h.suspended = false;
    }
    return bad;
}

void HookRegistry::collectTrampolines(std::vector<std::pair<Addr, size_t>>* out) const {
    for (const auto& [target, h] : hooks_) out->push_back({h.trampoline, h.trampSize});
}

Addr HookRegistry::trampolineOf(Addr target) const {
    auto it = hooks_.find(target);
    if (it == hooks_.end()) return 0;
    return kMutTrampolineOfLive ? target : it->second.trampoline;
}

Addr HookRegistry::chainedOver(Addr target) const {
    auto it = hooks_.find(target);
    return it == hooks_.end() ? 0 : it->second.chainedTo;
}

Addr trampolineOfAny(Addr target) {
    std::lock_guard<std::mutex> g(g_regsMu);
    for (auto* r : allRegs())
        if (Addr t = r->trampolineOf(target)) return t;
    return 0;
}

int suspendAllHooks() {
    std::lock_guard<std::mutex> g(g_regsMu);
    int bad = 0;
    for (auto* r : allRegs()) bad += r->suspendAll();
    return bad;
}
int resumeAllHooks() {
    std::lock_guard<std::mutex> g(g_regsMu);
    int bad = 0;
    for (auto* r : allRegs()) bad += r->resumeAll();
    return bad;
}
void collectAllTrampolines(std::vector<std::pair<Addr, size_t>>* out) {
    std::lock_guard<std::mutex> g(g_regsMu);
    for (auto* r : allRegs()) r->collectTrampolines(out);
}

int HookRegistry::auditTampered(std::vector<Addr>* which) const {
    int bad = 0;
    for (const auto& [target, h] : hooks_) {
        if (h.suspended) continue;
        uint8_t cur[16] = {};
        if (!mem_.read(target, cur, h.stolen) || std::memcmp(cur, h.patch, h.stolen) != 0) {
            ++bad;
            if (which) which->push_back(target);
        }
    }
    return bad;
}

int HookRegistry::restoreAllAndVerify() {
    int bad = 0;
    while (!hooks_.empty()) {
        auto it = hooks_.begin();
        it->second.refs = 1;
        HookSpec s{};
        s.target = it->first;
        s.name = "(restoreAll)";
        PatchStatus st = release(s);
        if (st != PatchStatus::Restored) {
            ++bad;
            if (hooks_.count(s.target)) {  // WriteFailed leaves the entry: give up on it so the loop ends
                hooks_.erase(s.target);
                --g_live;
            }
        }
    }
    return bad;
}

HookRegistry::~HookRegistry() {
    if (!kMutDtorKeepsHooks) restoreAllAndVerify();
    std::lock_guard<std::mutex> g(g_regsMu);
    auto& v = allRegs();
    for (size_t i = 0; i < v.size(); ++i)
        if (v[i] == this) { v.erase(v.begin() + static_cast<long>(i)); break; }
}

}  // namespace nvhooks
