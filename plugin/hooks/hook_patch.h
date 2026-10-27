// Entry-point detours with a fail-safe prologue check, ref-counted install/uninstall.
//
// Game-free: all memory access goes through MemoryOps, so the logic is unit-tested on a fake buffer
// (tests/test_hooks.cpp) and driven by Win32Memory inside the game. Addresses are 32-bit game addresses.
//
// Rules the patcher enforces (a violation => refuse, never write):
//   1. The first `expectedLen` bytes at `target` must equal `expected` (wrong exe version / another mod already
//      patched this function => PrologueMismatch / AlreadyHooked, zero bytes written).
//   2. `stolen` is 5..16, <= expectedLen, and the function must start with the standard frame prologue
//      (55 8B EC): we only hook function ENTRIES, so the stolen bytes never contain a rel32 branch that would need
//      relocating into the trampoline.
//   3. Trampoline = stolen bytes + `jmp target+stolen`. The detour calls the original through it.
//   4. Chaining (opt-in per acquire, `chainForeignJmp`): when the entry already holds another patcher's `jmp rel32` (xNVSE's
//      EventManager does this at MarkBaseExtraListScriptEvent 0x5AC750) and the rest of the signature is intact, we hook ON TOP of
//      it: only those 5 bytes are replaced, and the trampoline is that jump relocated, so the call order is game -> ours -> theirs
//      -> original. Release puts their jump back byte for byte. Their code never points into ours, so a core unload stays safe.
#pragma once
#include <cstddef>
#include <cstdint>
#include <map>
#include <mutex>
#include <string>
#include <vector>

namespace nvhooks {

using Addr = uint32_t;

struct HookSpec {
    const char* name;
    Addr target;
    uint8_t expected[16];
    uint8_t expectedLen;
    uint8_t stolen;
    int16_t retBytes = -1;  // callee-popped stack bytes (`ret N`; 0 = cdecl/`ret`), -1 = unknown. Pinned against detour signatures.
};

struct MemoryOps {
    virtual ~MemoryOps() = default;
    virtual bool read(Addr a, void* out, size_t n) = 0;
    virtual bool write(Addr a, const void* in, size_t n) = 0;  // handles page protection / icache
    virtual Addr allocExec(size_t n) = 0;                      // 0 on failure
    virtual void freeExec(Addr a, size_t n) = 0;
    // True when `a` is code outside FalloutNV.exe's image (another xNVSE plugin's detour). acquire() then accepts an
    // E8/E9 past the stolen bytes whose target was moved there. Default false: every expected byte must match.
    virtual bool foreignCode(Addr) { return false; }
};

enum class PatchStatus {
    Ok,                // newly installed
    AlreadyInstalled,  // our own hook, refcount incremented
    PrologueMismatch,  // bytes differ from the expected signature: nothing written
    AlreadyHooked,     // first byte is E9/E8 (somebody else detoured it): nothing written
    BadSpec,           // spec violates rule 2 (or a second detour for the same target)
    ReadFailed,
    WriteFailed,
    AllocFailed,
    NotInstalled,      // release() of an unknown hook
    Tampered,          // release(): bytes no longer ours, left untouched
    Released,          // refcount decremented, still installed
    Restored,          // last ref dropped, original bytes restored
};
const char* statusName(PatchStatus s);

// Process-wide accounting for hot reload (docs/tools/T3b-state.md). liveHookCount() is the number of entry patches that are
// installed right now across every HookRegistry in this module; it must be 0 after a core DLL's shutdown or FreeLibrary would
// leave game code jumping into freed memory. tamperedCount() counts hooks we could not restore because another patcher chained
// over ours: those keep their trampoline alive, and a reload must refuse to unload while it is non-zero.
int liveHookCount();
int tamperedCount();
inline bool installed(PatchStatus s) { return s == PatchStatus::Ok || s == PatchStatus::AlreadyInstalled; }

class HookRegistry {
public:
    explicit HookRegistry(MemoryOps& mem);
    ~HookRegistry();  // restores everything still installed
    // On success *trampolineOut is the address to call for the original function.
    // `origSlot` (optional, 32-bit): written with the trampoline address BEFORE the entry is patched, so a thread that enters the detour
    // the instant the patch lands already finds a valid "original" pointer (the detours call through it).
    // `chainForeignJmp`: see rule 4 at the top. Off = a foreign jump at the entry is refused (AlreadyHooked), as before.
    PatchStatus acquire(const HookSpec& spec, Addr detour, Addr* trampolineOut, Addr* origSlot = nullptr, bool chainForeignJmp = false);
    // The foreign jump target an installed hook was chained over (rule 4), 0 = not chained / not installed.
    Addr chainedOver(Addr target) const;
    PatchStatus release(const HookSpec& spec);
    int refCount(const HookSpec& spec) const;
    size_t installedCount() const { return hooks_.size(); }
    std::string lastError() const { return lastError_; }
    // Reload support: release every hook (ignoring refcounts), then read each patched entry back and compare it with the bytes
    // we saved at install time. Returns the number of hooks that could NOT be proven restored (0 = clean).
    int restoreAllAndVerify();
    // Read-only preflight for a reload: how many installed hooks no longer have our patch bytes at the entry
    // (another patcher chained over it). Such a hook cannot be restored safely, so the reload must be refused.
    int auditTampered(std::vector<Addr>* which = nullptr) const;   // which: the targets whose bytes are no longer ours
    // Reload drain, step 1: put the original bytes back at every entry but keep the trampolines and the bookkeeping, so no thread
    // can ENTER a detour any more while threads already inside it finish. Returns how many hooks could not be suspended (0 = all).
    // resumeAll() re-patches every suspended hook (used when the drain times out and the reload is refused).
    int suspendAll();
    int resumeAll();
    // Appends [trampoline, trampoline+size) of every installed hook (threads may still be executing inside them).
    void collectTrampolines(std::vector<std::pair<Addr, size_t>>* out) const;
    // Address to CALL for the original function of an installed hook on `target` (its trampoline: the stolen bytes + a jump back).
    // 0 = no hook of this registry on that target. A suspended hook (original bytes back at the entry) still answers: the trampoline is valid.
    Addr trampolineOf(Addr target) const;

private:
    struct Installed {
        Addr detour = 0;
        Addr trampoline = 0;
        size_t trampSize = 0;
        uint8_t stolen = 0;
        uint8_t original[16] = {};
        uint8_t patch[16] = {};
        int refs = 0;
        bool suspended = false;
        Addr chainedTo = 0;   // rule 4: the foreign jump target we installed on top of (0 = a normal entry hook)
    };
    MemoryOps& mem_;
    std::map<Addr, Installed> hooks_;
    std::string lastError_;
};

// Cross-thread safe patching. Writing a jump over code another thread is executing can leave that thread running torn bytes.
// The reload layer installs a guard that freezes every other thread around each entry write and moves any thread that is
// stopped inside the patched bytes between the original entry and its trampoline copy (same offsets). Default: none.
//   begin(target, stolen, trampoline, installing): installing = the entry is about to get the jump; false = original bytes go back.
//   end(): resume everything. Between the two calls only the memory write runs (no allocation, no locks).
using WriteGuardBegin = void (*)(Addr target, uint8_t stolen, Addr trampoline, bool installing);
using WriteGuardEnd = void (*)();
void setWriteGuard(WriteGuardBegin begin, WriteGuardEnd end);

// Process-wide helpers over every live HookRegistry (L5, L11 and L12 each own one). Used by the reload drain (plugin/reload).
int suspendAllHooks();
int resumeAllHooks();
void collectAllTrampolines(std::vector<std::pair<Addr, size_t>>* out);
// trampolineOf over every live HookRegistry in this module (L5, L11, L12 each own one). 0 = not hooked by us.
// Use it to call an engine function that one of OUR detours has patched without going through the detour (queries, diagnostics).
Addr trampolineOfAny(Addr target);

// Validate a spec without touching memory (rule 2).
bool specIsValid(const HookSpec& s, std::string* why = nullptr);

}  // namespace nvhooks
