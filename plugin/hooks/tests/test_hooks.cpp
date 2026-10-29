// Unit tests for the hook patcher (fake memory) and the capture hub. Exit code != 0 on any failure.
// Built twice: against the real sources, and against NVHOOKS_MUTANT sources (the latter MUST fail).
#include <cstdarg>
#include <cstdio>
#include <cstring>
#include <string>
#include <map>
#include <set>
#include <unordered_set>
#include <vector>

#include "../capture_hub.h"
#include "../effect_json.h"
#include "../engine_addrs.h"
#include "../extract_pad.h"
#include "../hook_patch.h"
#include "../apply_lines.h"
#include "../change_tracker.h"
#include "../inventory_diff.h"
#include "../suppress_policy.h"
#include "../guest_item.h"
#include "../guest_local_run.h"
#include "../guest_objvars.h"
#include "../activate_args.h"
#include "../kill_attrib.h"
#include "../ref_var.h"
#include "../controls_route.h"
#include "../player_kill.h"
#include "../player_facts.h"
#include "../lock_preimage.h"
#include "../faction_removal.h"
#include "../party_cond.h"
#include "../rules_reducer.h"
#include "../per_slot_run.h"
#include "../info_scope.h"
#include "fnvcoop/session.h"
#include "fnvcoop/transport.h"

using namespace nvhooks;

static int g_fail = 0, g_checks = 0;
#define CHECK(c)                                                                  \
    do {                                                                          \
        ++g_checks;                                                               \
        if (!(c)) {                                                               \
            ++g_fail;                                                             \
            std::printf("  FAIL %s:%d  %s\n", __FILE__, __LINE__, #c);            \
        }                                                                         \
    } while (0)
#define TEST(name) static void name()
static void run(const char* n, void (*f)()) {
    int before = g_fail;
    f();
    std::printf("[%s] %s\n", g_fail == before ? " ok " : "FAIL", n);
}

// ---- fake game memory: one "image" region at 0x00400000 + a bump allocator region for trampolines ----
struct FakeMemory : MemoryOps {
    static constexpr Addr kImage = 0x00400000, kImageSize = 0x1000, kHeap = 0x20000000;
    std::vector<uint8_t> image = std::vector<uint8_t>(kImageSize, 0xCC);
    std::map<Addr, std::vector<uint8_t>> heap;
    Addr next = kHeap;
    int writes = 0;          // total successful writes
    int imageWrites = 0;     // writes into the image (game code)
    bool failAlloc = false, failTrampWrite = false, failImageWrite = false;
    bool dropImageWrite = false;  // write() claims success but changes nothing (a restore that does not stick)
    int freed = 0;

    uint8_t* at(Addr a, size_t n) {
        if (a >= kImage && a + n <= kImage + kImageSize) return &image[a - kImage];
        for (auto& [b, v] : heap)
            if (a >= b && a + n <= b + v.size()) return &v[a - b];
        return nullptr;
    }
    bool read(Addr a, void* out, size_t n) override {
        uint8_t* p = at(a, n);
        if (!p) return false;
        std::memcpy(out, p, n);
        return true;
    }
    bool write(Addr a, const void* in, size_t n) override {
        bool inImage = a >= kImage && a < kImage + kImageSize;
        if (inImage && failImageWrite) return false;
        if (inImage && dropImageWrite) return true;
        if (!inImage && failTrampWrite) return false;
        uint8_t* p = at(a, n);
        if (!p) return false;
        std::memcpy(p, in, n);
        ++writes;
        if (inImage) ++imageWrites;
        return true;
    }
    Addr allocExec(size_t n) override {
        if (failAlloc) return 0;
        Addr a = next;
        next += static_cast<Addr>((n + 15) & ~size_t(15));
        heap[a] = std::vector<uint8_t>(n, 0);
        return a;
    }
    void freeExec(Addr a, size_t) override {
        heap.erase(a);
        ++freed;
    }
    // Lay a function (the spec's expected bytes) down at spec.target.
    void place(const HookSpec& s) { std::memcpy(&image[s.target - kImage], s.expected, s.expectedLen); }
    std::vector<uint8_t> bytes(Addr a, size_t n) {
        std::vector<uint8_t> v(n);
        read(a, v.data(), n);
        return v;
    }
};

static HookSpec makeSpec(Addr target = 0x00400100, uint8_t stolen = 6) {
    HookSpec s{"fake::Fn", target, {0x55, 0x8B, 0xEC, 0x83, 0xEC, 0x08, 0x89, 0x4D, 0xF8, 0x8B, 0x4D, 0xF8, 0xE8, 0x11, 0x22, 0x33}, 16, stolen};
    return s;
}
static uint32_t rel32At(const std::vector<uint8_t>& v, size_t off) {
    uint32_t r;
    std::memcpy(&r, &v[off], 4);
    return r;
}

TEST(install_patches_entry_and_builds_trampoline) {
    FakeMemory m;
    HookSpec s = makeSpec();
    m.place(s);
    HookRegistry reg(m);
    Addr tramp = 0, detour = 0x30000000;
    CHECK(reg.acquire(s, detour, &tramp) == PatchStatus::Ok);
    CHECK(tramp != 0);
    auto code = m.bytes(s.target, 16);
    CHECK(code[0] == 0xE9);
    CHECK(s.target + 5 + rel32At(code, 1) == detour);
    CHECK(code[5] == 0x90);                                   // stolen=6: one NOP of padding
    CHECK(std::memcmp(&code[6], &s.expected[6], 10) == 0);    // bytes beyond `stolen` untouched
    auto t = m.bytes(tramp, 11);
    CHECK(std::memcmp(t.data(), s.expected, 6) == 0);          // stolen bytes first
    CHECK(t[6] == 0xE9);
    CHECK(tramp + 6 + 5 + rel32At(t, 7) == s.target + 6);      // jumps back to the unstolen remainder
    CHECK(m.imageWrites == 1);
}

TEST(wrong_prologue_refuses_and_writes_nothing) {
    FakeMemory m;
    HookSpec s = makeSpec();
    m.place(s);
    m.image[s.target - FakeMemory::kImage + 13] ^= 0xFF;  // different call displacement => different exe build
    auto before = m.image;
    HookRegistry reg(m);
    Addr tramp = 0xDEAD;
    CHECK(reg.acquire(s, 0x30000000, &tramp) == PatchStatus::PrologueMismatch);
    CHECK(m.writes == 0);
    CHECK(m.image == before);
    CHECK(m.heap.empty());          // no trampoline leaked
    CHECK(reg.installedCount() == 0);
    CHECK(tramp == 0xDEAD);         // output untouched on failure
}

TEST(unrelated_bytes_refuse) {
    FakeMemory m;  // image is all 0xCC: nothing like a prologue
    HookSpec s = makeSpec();
    HookRegistry reg(m);
    CHECK(reg.acquire(s, 0x30000000, nullptr) == PatchStatus::PrologueMismatch);
    CHECK(m.writes == 0);
}

TEST(foreign_detour_is_reported_not_overwritten) {
    FakeMemory m;
    HookSpec s = makeSpec();
    m.place(s);
    m.image[s.target - FakeMemory::kImage] = 0xE9;  // someone else already detoured the entry
    HookRegistry reg(m);
    CHECK(reg.acquire(s, 0x30000000, nullptr) == PatchStatus::AlreadyHooked);
    CHECK(m.writes == 0);
}

TEST(bad_specs_are_rejected) {
    FakeMemory m;
    HookRegistry reg(m);
    HookSpec a = makeSpec(0x00400100, 4);  // too few bytes for a jmp rel32
    m.place(a);
    CHECK(reg.acquire(a, 1, nullptr) == PatchStatus::BadSpec);
    HookSpec b = makeSpec();
    b.expected[0] = 0x8B;  // not a standard frame entry: stolen bytes could hold a rel32 branch
    CHECK(reg.acquire(b, 1, nullptr) == PatchStatus::BadSpec);
    HookSpec c = makeSpec();
    c.expectedLen = 5;  // signature shorter than the stolen region
    CHECK(reg.acquire(c, 1, nullptr) == PatchStatus::BadSpec);
    CHECK(m.writes == 0);
}

TEST(refcount_install_release_restore) {
    FakeMemory m;
    HookSpec s = makeSpec();
    m.place(s);
    auto pristine = m.image;
    HookRegistry reg(m);
    Addr t1 = 0, t2 = 0;
    CHECK(reg.acquire(s, 0x30000000, &t1) == PatchStatus::Ok);
    CHECK(reg.acquire(s, 0x30000000, &t2) == PatchStatus::AlreadyInstalled);
    CHECK(t1 == t2);
    CHECK(reg.refCount(s) == 2);
    CHECK(m.imageWrites == 1);  // second acquire does not re-patch
    CHECK(reg.release(s) == PatchStatus::Released);
    CHECK(m.image != pristine);  // still hooked for the remaining user
    CHECK(m.bytes(s.target, 1)[0] == 0xE9);
    CHECK(reg.release(s) == PatchStatus::Restored);
    CHECK(m.image == pristine);
    CHECK(m.freed == 1);
    CHECK(reg.release(s) == PatchStatus::NotInstalled);
}

TEST(different_detour_for_same_target_refused) {
    FakeMemory m;
    HookSpec s = makeSpec();
    m.place(s);
    HookRegistry reg(m);
    CHECK(reg.acquire(s, 0x30000000, nullptr) == PatchStatus::Ok);
    CHECK(reg.acquire(s, 0x30000040, nullptr) == PatchStatus::BadSpec);
    CHECK(reg.refCount(s) == 1);
}

TEST(tampered_code_not_restored) {
    FakeMemory m;
    HookSpec s = makeSpec();
    m.place(s);
    HookRegistry reg(m);
    CHECK(reg.acquire(s, 0x30000000, nullptr) == PatchStatus::Ok);
    m.image[s.target - FakeMemory::kImage + 1] ^= 0x55;  // another hook chained over ours
    auto after = m.image;
    CHECK(reg.release(s) == PatchStatus::Tampered);
    CHECK(m.image == after);
    CHECK(m.freed == 0);  // their jump may still target our trampoline
}

TEST(alloc_or_write_failure_leaves_game_code_untouched) {
    {
        FakeMemory m;
        HookSpec s = makeSpec();
        m.place(s);
        auto p = m.image;
        m.failAlloc = true;
        HookRegistry reg(m);
        CHECK(reg.acquire(s, 1, nullptr) == PatchStatus::AllocFailed);
        CHECK(m.image == p);
    }
    {
        FakeMemory m;
        HookSpec s = makeSpec();
        m.place(s);
        auto p = m.image;
        m.failTrampWrite = true;
        HookRegistry reg(m);
        CHECK(reg.acquire(s, 1, nullptr) == PatchStatus::WriteFailed);
        CHECK(m.image == p);
        CHECK(m.heap.empty());
    }
    {
        FakeMemory m;
        HookSpec s = makeSpec();
        m.place(s);
        m.failImageWrite = true;
        HookRegistry reg(m);
        CHECK(reg.acquire(s, 1, nullptr) == PatchStatus::WriteFailed);
        CHECK(m.heap.empty());  // trampoline freed again
    }
}

TEST(registry_destructor_restores_everything) {
    FakeMemory m;
    HookSpec a = makeSpec(0x00400100), b = makeSpec(0x00400200, 5);
    m.place(a);
    m.place(b);
    auto pristine = m.image;
    {
        HookRegistry reg(m);
        CHECK(reg.acquire(a, 1, nullptr) == PatchStatus::Ok);
        CHECK(reg.acquire(b, 2, nullptr) == PatchStatus::Ok);
        CHECK(m.image != pristine);
    }
    CHECK(m.image == pristine);
}

TEST(reload_cycles_restore_every_byte_and_balance_live_count) {
    FakeMemory m;
    HookSpec a = makeSpec(0x00400100), b = makeSpec(0x00400200, 5), c = makeSpec(0x00400300, 8);
    m.place(a);
    m.place(b);
    m.place(c);
    auto pristine = m.image;
    int live0 = liveHookCount();
    for (int i = 0; i < 100; ++i) {
        HookRegistry reg(m);
        CHECK(reg.acquire(a, 1, nullptr) == PatchStatus::Ok);
        CHECK(reg.acquire(b, 2, nullptr) == PatchStatus::Ok);
        CHECK(reg.acquire(c, 3, nullptr) == PatchStatus::Ok);
        CHECK(reg.acquire(c, 3, nullptr) == PatchStatus::AlreadyInstalled);  // refcount 2: shutdown must ignore it
        CHECK(liveHookCount() == live0 + 3);
        CHECK(reg.restoreAllAndVerify() == 0);
        CHECK(liveHookCount() == live0);
        CHECK(m.image == pristine);
        CHECK(m.heap.empty());
    }
}

TEST(reload_restore_failure_is_reported_and_never_hangs) {
    int live0 = liveHookCount();
    {
        FakeMemory m;
        HookSpec s = makeSpec();
        m.place(s);
        HookRegistry reg(m);
        CHECK(reg.acquire(s, 1, nullptr) == PatchStatus::Ok);
        m.failImageWrite = true;
        CHECK(reg.restoreAllAndVerify() == 1);  // could not restore: reported, loop ends
        CHECK(reg.installedCount() == 0);
    }
    CHECK(liveHookCount() == live0);
    {
        FakeMemory m;
        HookSpec s = makeSpec();
        m.place(s);
        HookRegistry reg(m);
        CHECK(reg.acquire(s, 1, nullptr) == PatchStatus::Ok);
        m.dropImageWrite = true;  // the write "worked" but the old bytes are still the jump
        CHECK(reg.release(s) == PatchStatus::WriteFailed);
        CHECK(m.heap.size() == 1);  // trampoline must stay: game code still jumps to our detour
    }
    CHECK(liveHookCount() == live0);
}

TEST(reload_preflight_finds_chained_hooks_without_touching_anything) {
    FakeMemory m;
    HookSpec a = makeSpec(0x00400100), b = makeSpec(0x00400200);
    m.place(a);
    m.place(b);
    HookRegistry reg(m);
    CHECK(reg.acquire(a, 1, nullptr) == PatchStatus::Ok);
    CHECK(reg.acquire(b, 2, nullptr) == PatchStatus::Ok);
    CHECK(reg.auditTampered() == 0);
    m.image[b.target - FakeMemory::kImage + 2] ^= 0x10;
    auto before = m.image;
    int writes = m.writes;
    CHECK(reg.auditTampered() == 1);
    CHECK(m.image == before && m.writes == writes);
    m.image[b.target - FakeMemory::kImage + 2] ^= 0x10;  // put it back so the destructor can restore
}

TEST(reload_suspend_resume_keeps_trampolines_and_release_still_restores) {
    FakeMemory m;
    HookSpec a = makeSpec(0x00400100), b = makeSpec(0x00400200, 5);
    m.place(a);
    m.place(b);
    auto pristine = m.image;
    HookRegistry reg(m);
    CHECK(reg.acquire(a, 1, nullptr) == PatchStatus::Ok);
    CHECK(reg.acquire(b, 2, nullptr) == PatchStatus::Ok);
    auto hooked = m.image;
    CHECK(reg.suspendAll() == 0);
    CHECK(m.image == pristine);          // nobody can enter a detour any more
    CHECK(m.heap.size() == 2);           // trampolines stay: threads already inside may still return through them
    std::vector<std::pair<Addr, size_t>> ranges;
    reg.collectTrampolines(&ranges);
    CHECK(ranges.size() == 2);
    CHECK(reg.auditTampered() == 0);     // suspended hooks are not "tampered"
    CHECK(reg.resumeAll() == 0);
    CHECK(m.image == hooked);            // refused reload: hooks are back exactly as they were
    CHECK(reg.suspendAll() == 0);
    int t0 = tamperedCount();
    CHECK(reg.restoreAllAndVerify() == 0);  // shutdown after a successful drain
    CHECK(m.image == pristine && m.heap.empty() && tamperedCount() == t0);
}

TEST(trampoline_lookup_names_the_original_code_of_installed_hooks_only) {
    // dll-wiring: world.query calls an evaluator that our own detour patched THROUGH its trampoline (plugin/hooks_world/world_query.cpp)
    FakeMemory m;
    HookSpec a = makeSpec(0x00400100), b = makeSpec(0x00400200, 5);
    m.place(a);
    m.place(b);
    HookRegistry reg(m);
    Addr ta = 0, tb = 0;
    CHECK(reg.trampolineOf(a.target) == 0);   // nothing installed yet
    CHECK(reg.acquire(a, 1, &ta) == PatchStatus::Ok);
    CHECK(reg.acquire(b, 2, &tb) == PatchStatus::Ok);
    CHECK(ta != 0 && tb != 0 && ta != tb && ta != a.target && tb != b.target);
    CHECK(reg.trampolineOf(a.target) == ta && reg.trampolineOf(b.target) == tb);
    CHECK(reg.trampolineOf(a.target + 4) == 0);                              // an address inside the function is not an entry
    CHECK(trampolineOfAny(a.target) == ta && trampolineOfAny(b.target) == tb);   // process-wide
    CHECK(trampolineOfAny(0x00400300) == 0);
    auto code = m.bytes(tb, b.stolen);
    CHECK(std::memcmp(code.data(), b.expected, b.stolen) == 0);               // the trampoline begins with the original bytes
    CHECK(reg.suspendAll() == 0);                                             // reload drain: original bytes are back at the entry, the trampoline stays valid
    CHECK(reg.trampolineOf(a.target) == ta);
    CHECK(reg.resumeAll() == 0);
    CHECK(reg.release(a) == PatchStatus::Restored);
    CHECK(reg.trampolineOf(a.target) == 0 && trampolineOfAny(a.target) == 0);
    CHECK(reg.trampolineOf(b.target) == tb);
}

TEST(reload_suspend_leaves_chained_hooks_alone_and_process_wide_helpers_see_every_registry) {
    FakeMemory m1, m2;
    HookSpec a = makeSpec(0x00400100);
    m1.place(a);
    m2.place(a);
    HookRegistry r1(m1), r2(m2);
    CHECK(r1.acquire(a, 1, nullptr) == PatchStatus::Ok);
    CHECK(r2.acquire(a, 1, nullptr) == PatchStatus::Ok);
    auto p1 = m1.image;
    m2.image[a.target - FakeMemory::kImage + 1] ^= 0x33;  // chained over in registry 2
    CHECK(suspendAllHooks() >= 1);
    CHECK(m1.bytes(a.target, 1)[0] != 0xE9);               // registry 1 suspended
    std::vector<std::pair<Addr, size_t>> ranges;
    collectAllTrampolines(&ranges);
    CHECK(ranges.size() >= 2);
    CHECK(resumeAllHooks() == 0);
    CHECK(m1.bytes(a.target, 1)[0] == 0xE9);
    m2.image[a.target - FakeMemory::kImage + 1] ^= 0x33;  // restore so the destructor is clean
}

TEST(reload_tampered_hook_is_counted_so_the_loader_can_refuse_to_unload) {
    FakeMemory m;
    HookSpec s = makeSpec();
    m.place(s);
    int t0 = tamperedCount();
    {
        HookRegistry reg(m);
        CHECK(reg.acquire(s, 1, nullptr) == PatchStatus::Ok);
        m.image[s.target - FakeMemory::kImage + 1] ^= 0x55;
        CHECK(reg.restoreAllAndVerify() == 1);
    }
    CHECK(tamperedCount() == t0 + 1);
}

// ---- hook_patch rule 4: chaining on top of another patcher's entry jump (xNVSE's EventManager hook at 0x5AC750) ----
// FakeXnvse models EventManager's lazy hook: the first handler registration for a game event writes `jmp MarkEventHook` over the
// first 5 bytes (WriteRelJump), then the installer pointer is cleared and it never writes there again. Its stub re-runs the prologue
// and jumps to target+6, so it never calls whatever was at the entry before.
struct FakeXnvse {
    static constexpr Addr kStub = 0x0B6A1000;   // MarkEventHook inside nvse_1_4.dll (never executed by the tests)
    FakeMemory& m;
    Addr target;
    bool installerUsed = false;
    void setEventHandler() {
        if (installerUsed) return;
        installerUsed = true;
        uint8_t j[5];
        uint32_t rel = kStub - (target + 5);
        j[0] = 0xE9;
        std::memcpy(j + 1, &rel, 4);
        std::memcpy(&m.image[target - FakeMemory::kImage], j, 5);   // WriteRelJump: no prologue check, no chaining
    }
};

TEST(chain_over_foreign_entry_jump_relocates_it_and_release_puts_it_back) {
    FakeMemory m;
    HookSpec s = makeSpec();
    m.place(s);
    FakeXnvse x{m, s.target};
    x.setEventHandler();
    const auto theirs = m.image;
    HookRegistry reg(m);
    const Addr detour = 0x30000000;
    CHECK(reg.acquire(s, detour, nullptr) == PatchStatus::AlreadyHooked);   // not opted in: refused, nothing written
    CHECK(m.image == theirs);
    Addr tramp = 0;
    CHECK(reg.acquire(s, detour, &tramp, nullptr, true) == PatchStatus::Ok);
    CHECK(reg.chainedOver(s.target) == FakeXnvse::kStub);
    auto code = m.bytes(s.target, 16);
    CHECK(code[0] == 0xE9 && s.target + 5 + rel32At(code, 1) == detour);   // game -> ours
    CHECK(code[5] == s.expected[5]);                                          // only their 5 bytes were replaced
    CHECK(std::memcmp(&code[6], &s.expected[6], 10) == 0);
    auto t = m.bytes(tramp, 10);
    CHECK(t[0] == 0xE9 && tramp + 5 + rel32At(t, 1) == FakeXnvse::kStub);   // ours -> theirs (relocated, not copied verbatim)
    CHECK(t[5] == 0xE9 && tramp + 10 + rel32At(t, 6) == s.target + 5);
    CHECK(reg.auditTampered() == 0);                                          // a hot reload may unhook
    CHECK(reg.release(s) == PatchStatus::Restored);
    CHECK(m.image == theirs);                                                 // their jump back, byte for byte
}

TEST(xnvse_primed_before_install_keeps_the_hook_and_the_swap_alive) {
    HookSpec s = makeSpec();
    {   // the old order: we hook first, the bots/loot OnDrop registration makes xNVSE write over us -> reload refused
        FakeMemory m;
        m.place(s);
        FakeXnvse x{m, s.target};
        HookRegistry reg(m);
        CHECK(reg.acquire(s, 0x30000000, nullptr, nullptr, true) == PatchStatus::Ok);
        x.setEventHandler();
        CHECK(reg.auditTampered() == 1);
        m.place(s);   // let the destructor restore cleanly
    }
    {   // the fixed order: prime xNVSE, then hook (chained), then the later registrations write nothing
        FakeMemory m;
        m.place(s);
        FakeXnvse x{m, s.target};
        x.setEventHandler();   // PrimeXnvseEventHook
        const auto theirs = m.image;
        for (int cycle = 0; cycle < 3; ++cycle) {   // core load, hot reload, hot reload
            HookRegistry reg(m);
            CHECK(reg.acquire(s, 0x30000000 + 0x40u * cycle, nullptr, nullptr, true) == PatchStatus::Ok);
            x.setEventHandler();   // bots InstallDropHandler: installer already used
            CHECK(reg.auditTampered() == 0);
            CHECK(reg.restoreAllAndVerify() == 0);
            CHECK(m.image == theirs);
        }
    }
    {   // a process where an older core was overwritten by xNVSE: the entry is their jump plus our NOP padding byte
        FakeMemory m;
        m.place(s);
        FakeXnvse x{m, s.target};
        {
            HookRegistry old(m);
            CHECK(old.acquire(s, 0x30000000, nullptr) == PatchStatus::Ok);
            x.setEventHandler();
            CHECK(old.restoreAllAndVerify() == 1);   // tampered: left alone
        }
        CHECK(m.image[s.target - FakeMemory::kImage + 5] == 0x90);
        HookRegistry reg(m);
        CHECK(reg.acquire(s, 0x30000080, nullptr, nullptr, true) == PatchStatus::Ok);
        CHECK(reg.chainedOver(s.target) == FakeXnvse::kStub);
    }
}

TEST(chain_refuses_a_foreign_jump_over_a_different_function_or_into_ourselves) {
    HookSpec s = makeSpec();
    {   // the body after the stolen bytes changed: wrong exe or a patch we do not understand
        FakeMemory m;
        m.place(s);
        FakeXnvse x{m, s.target};
        x.setEventHandler();
        m.image[s.target - FakeMemory::kImage + 9] ^= 0x01;
        const auto before = m.image;
        HookRegistry reg(m);
        CHECK(reg.acquire(s, 0x30000000, nullptr, nullptr, true) == PatchStatus::AlreadyHooked);
        CHECK(m.image == before && reg.installedCount() == 0);
    }
    {   // the jump already goes to this detour (bookkeeping lost): chaining would loop forever
        FakeMemory m;
        m.place(s);
        const Addr detour = 0x30000000;
        uint32_t rel = detour - (s.target + 5);
        m.image[s.target - FakeMemory::kImage] = 0xE9;
        std::memcpy(&m.image[s.target - FakeMemory::kImage + 1], &rel, 4);
        HookRegistry reg(m);
        CHECK(reg.acquire(s, detour, nullptr, nullptr, true) == PatchStatus::AlreadyHooked);
    }
}

// ---- extract_pad.h: ExtractArgs with more parameters in the script line than the hook has out pointers ----
// fakeExtract models xNVSE's v_ExtractArgsEx: one va_arg pointer per parameter PRESENT in the line; an int is written as 4 bytes, a
// string is copied whole. A null pointer stands for "a stray stack word": in the game that write would land at a random address.
struct FakeLine {
    int n = 0;
    char kind[8] = {};
    int ival[8] = {};
    const char* sval[8] = {};
};
static FakeLine g_fakeLine;
static int g_strayWrites = 0;
static bool __cdecl fakeExtract(void*, void*, uint32_t* off, void*, void*, void*, void* ev, ...) {
    va_list ap;
    va_start(ap, ev);
    for (int i = 0; i < g_fakeLine.n; ++i) {
        void* p = va_arg(ap, void*);
        if (!p) { ++g_strayWrites; continue; }
        if (g_fakeLine.kind[i] == 's') std::memcpy(p, g_fakeLine.sval[i], std::strlen(g_fakeLine.sval[i]) + 1);
        else std::memcpy(p, &g_fakeLine.ival[i], 4);
        ++*off;
    }
    va_end(ap);
    return true;
}

TEST(extract_args_extra_parameters_land_in_spare_slots_not_on_the_stack) {
    {   // AddItem <item> 300 1: the hook asks for item and count only (the old itemCmd)
        g_fakeLine = FakeLine{};
        g_fakeLine.n = 3;
        g_fakeLine.kind[0] = g_fakeLine.kind[1] = g_fakeLine.kind[2] = 'i';
        g_fakeLine.ival[0] = 0x1F276; g_fakeLine.ival[1] = 300; g_fakeLine.ival[2] = 1;
        g_strayWrites = 0;
        struct { uint32_t a = 0xA5A5A5A5; int item = 0; int count = 0; uint32_t b = 0x5A5A5A5A; } f;
        uint32_t off = 0;
        CHECK(extractPadded(&fakeExtract, nullptr, nullptr, &off, nullptr, nullptr, nullptr, nullptr, {&f.item, &f.count}));
        CHECK(g_strayWrites == 0);
        CHECK(f.item == 0x1F276 && f.count == 300 && off == 3);
        CHECK(f.a == 0xA5A5A5A5 && f.b == 0x5A5A5A5A);
        CHECK(std::memcmp(extractSpare(2), &g_fakeLine.ival[2], 4) == 0);   // the extra flag went to scratch
    }
    {   // a line whose extra parameter is text ("angl" in an EIP is what a stray string write leaves behind)
        static const std::string longText = "player.setangle z " + std::string(0x1F0, 'x');
        g_fakeLine = FakeLine{};
        g_fakeLine.n = 2;
        g_fakeLine.kind[0] = 'i'; g_fakeLine.kind[1] = 's';
        g_fakeLine.ival[0] = 7;
        g_fakeLine.sval[1] = longText.c_str();
        g_strayWrites = 0;
        int first = 0;
        uint32_t off = 0;
        CHECK(longText.size() + 1 <= kExtractSpareBytes);
        CHECK(extractPadded(&fakeExtract, nullptr, nullptr, &off, nullptr, nullptr, nullptr, nullptr, {&first}));
        CHECK(g_strayWrites == 0 && first == 7);
        CHECK(std::string(extractSpare(1)) == longText);
    }
    {   // more outs than slots is refused before ExtractArgs runs
        int x = 0;
        uint32_t off = 0;
        CHECK(!extractPadded(&fakeExtract, nullptr, nullptr, &off, nullptr, nullptr, nullptr, nullptr,
                             {&x, &x, &x, &x, &x, &x, &x, &x, &x, &x, &x, &x, &x}));
    }
}

TEST(engine_address_table_is_sane) {
    std::map<Addr, const char*> seen;
    for (const HookSpec* s : addr::kAll) {
        std::string why;
        CHECK(specIsValid(*s, &why));
        CHECK(s->target >= 0x401000 && s->target < 0xFDE58B);  // inside .text of 1.4.0.525
        CHECK(seen.emplace(s->target, s->name).second);         // no duplicate targets
    }
    CHECK(addr::SetStage.target == 0x60D510);
    CHECK(addr::kAll[0] != nullptr);
}

TEST(engine_table_signature_roundtrip_on_fake_image) {
    // Every table entry must install cleanly on an image containing exactly its signature, and refuse on 1 flipped bit.
    for (const HookSpec* s : addr::kAll) {
        FakeMemory m;
        HookSpec t = *s;
        // remap target into the fake image
        t.target = FakeMemory::kImage + 0x100 + (s->target & 0x3FF);
        m.place(t);
        HookRegistry reg(m);
        CHECK(reg.acquire(t, 0x30000000, nullptr) == PatchStatus::Ok);
        reg.release(t);
        m.place(t);
        m.image[t.target - FakeMemory::kImage + s->expectedLen - 1] ^= 1;
        CHECK(reg.acquire(t, 0x30000000, nullptr) == PatchStatus::PrologueMismatch);
    }
}

// ---------------------------------------------------------------- capture hub
using namespace fnvcoop;
static Effect stage(uint32_t id, uint32_t st) { return SetStage{FormRef("fallout3.esm", id), st}; }
static Cause qcause(uint32_t id, int st) {
    Cause c;
    c.kind = CauseKind::QuestResult;
    c.source = FormRef("falloutnv.esm", id);
    c.aux = st;
    return c;
}

TEST(no_scope_no_record) {
    CaptureHub h;
    h.setEnabled(true);
    std::vector<EffectTxn> out;
    h.setSink([&](EffectTxn&& t) { out.push_back(std::move(t)); });
    CHECK(!h.record(stage(1, 10)));
    CHECK(h.stats().droppedNoScope == 1);
    h.begin(qcause(1, 10));
    h.end();
    CHECK(out.empty());  // the ambient mutation must not leak into a later scope
    CHECK(h.stats().emptyScopes == 1);
}

TEST(scope_emits_one_txn_with_cause_and_order) {
    CaptureHub h;
    h.setEnabled(true);
    std::vector<EffectTxn> out;
    h.setSink([&](EffectTxn&& t) { out.push_back(std::move(t)); });
    {
        ScopedCapture sc(h, qcause(0x104C, 30));
        CHECK(h.active());
        CHECK(h.record(stage(0x104C, 30)));
        CHECK(h.record(AddItem{FormRef("falloutnv.esm", 0xF), 100, Recipient::dflt()}));
        CHECK(h.record(ModKarma{5, Recipient::dflt()}));
        CHECK(out.empty());  // nothing emitted until the scope closes
    }
    CHECK(out.size() == 1);
    CHECK(out[0].cause.kind == CauseKind::QuestResult && out[0].cause.aux == 30);
    CHECK(out[0].effects.size() == 3);
    CHECK(std::holds_alternative<SetStage>(out[0].effects[0]));
    CHECK(std::holds_alternative<AddItem>(out[0].effects[1]));
    CHECK(std::holds_alternative<ModKarma>(out[0].effects[2]));
    CHECK(!h.active());
}

TEST(nested_scopes_merge_into_outermost_txn) {
    CaptureHub h;
    h.setEnabled(true);
    std::vector<EffectTxn> out;
    h.setSink([&](EffectTxn&& t) { out.push_back(std::move(t)); });
    Cause topic;
    topic.kind = CauseKind::DialogueResult;
    topic.source = FormRef("falloutnv.esm", 0x77);
    h.begin(topic);
    h.record(EnableRef{FormRef("falloutnv.esm", 1)});
    h.begin(qcause(0x104C, 40));  // topic result -> SetStage -> stage item result script
    h.record(stage(0x104C, 40));
    h.record(DisableRef{FormRef("falloutnv.esm", 2)});
    h.end();
    CHECK(out.empty());
    h.record(StartQuest{FormRef("falloutnv.esm", 3)});
    h.end();
    CHECK(out.size() == 1);
    CHECK(out[0].cause.kind == CauseKind::DialogueResult);
    CHECK(out[0].effects.size() == 4);
    CHECK(std::holds_alternative<EnableRef>(out[0].effects[0]));
    CHECK(std::holds_alternative<SetStage>(out[0].effects[1]));
    CHECK(std::holds_alternative<DisableRef>(out[0].effects[2]));
    CHECK(std::holds_alternative<StartQuest>(out[0].effects[3]));
}

TEST(echo_guard_blocks_capture_of_remote_apply) {
    CaptureHub h;
    h.setEnabled(true);
    std::vector<EffectTxn> out;
    h.setSink([&](EffectTxn&& t) { out.push_back(std::move(t)); });
    h.begin(qcause(1, 1));
    {
        fnvcoop::EchoGuard g(&h.echo());
        CHECK(!h.record(stage(1, 5)));  // remote apply re-enters the hooks: must not be captured
    }
    CHECK(h.record(stage(1, 6)));
    h.end();
    CHECK(out.size() == 1 && out[0].effects.size() == 1);
    CHECK(std::get<SetStage>(out[0].effects[0]).stage == 6);
    CHECK(h.stats().droppedEcho == 1);
}

TEST(disabled_hub_is_inert) {
    CaptureHub h;  // clients: disabled by default
    std::vector<EffectTxn> out;
    h.setSink([&](EffectTxn&& t) { out.push_back(std::move(t)); });
    h.begin(qcause(1, 1));
    CHECK(!h.active());
    CHECK(!h.record(stage(1, 1)));
    h.end();
    CHECK(out.empty() && h.stats().recorded == 0);
}

TEST(unbalanced_end_is_ignored) {
    CaptureHub h;
    h.setEnabled(true);
    h.end();
    CHECK(h.depth() == 0);
}

TEST(scope_observers_run_on_outermost_only_and_can_record) {
    CaptureHub h;
    h.setEnabled(true);
    std::vector<EffectTxn> out;
    h.setSink([&](EffectTxn&& t) { out.push_back(std::move(t)); });
    int begins = 0, ends = 0;
    h.setScopeObservers([&] { ++begins; },
                        [&] {
                            ++ends;
                            h.record(SetGlobal{FormRef("falloutnv.esm", 0x99), 3.0f});  // e.g. diffed TESGlobal
                        });
    h.begin(qcause(1, 1));
    h.begin(qcause(1, 2));
    h.end();
    CHECK(begins == 1 && ends == 0);  // nested scope: observers silent
    h.record(stage(1, 2));
    h.end();
    CHECK(begins == 1 && ends == 1);
    CHECK(out.size() == 1 && out[0].effects.size() == 2);
    CHECK(std::holds_alternative<SetGlobal>(out[0].effects[1]));  // observer effect lands inside the txn, after the others
}

TEST(script_vars_become_setscriptvar_effects) {
    CaptureHub h;
    h.setEnabled(true);
    std::vector<EffectTxn> out;
    h.setSink([&](EffectTxn&& t) { out.push_back(std::move(t)); });
    CHECK(!h.recordScriptVar({FormRef("falloutnv.esm", 5), 2, 1.0}));  // no scope
    CHECK(!h.recordScriptVar({FormRef("falloutnv.esm", 5), 70000, 1.0}));  // index does not fit uint16
    h.begin(qcause(1, 1));
    CHECK(h.recordScriptVar({FormRef("falloutnv.esm", 5), 2, 7.0}));
    {
        fnvcoop::EchoGuard g(&h.echo());
        CHECK(!h.recordScriptVar({FormRef("falloutnv.esm", 5), 3, 9.0}));
    }
    h.end();
    CHECK(out.size() == 1 && out[0].effects.size() == 1);
    auto* v = std::get_if<SetScriptVar>(&out[0].effects[0]);
    CHECK(v && v->index == 2 && v->value == 7.0 && v->owner == FormRef("falloutnv.esm", 5));
}

TEST(multiple_sinks_all_see_every_txn) {
    CaptureHub h;
    h.setEnabled(true);
    int a = 0, b = 0, aEff = 0, bEff = 0;
    h.setSink([&](EffectTxn&& t) { ++a; aEff += (int)t.effects.size(); });
    h.addSink([&](EffectTxn&& t) { ++b; bEff += (int)t.effects.size(); });
    h.setSink([&](EffectTxn&& t) { ++a; aEff += (int)t.effects.size(); });  // replaces slot 0 only
    h.begin(qcause(1, 1));
    h.record(stage(1, 3));
    h.end();
    CHECK(a == 1 && b == 1 && aEff == 1 && bEff == 1);
}

TEST(ref_angle_json_and_capture) {
    FormRef r("falloutnv.esm", 0x1234);
    nvcoop::json::Value bad = effectToJson(RefAngle{r, kAxisY, 45.f});
    Effect e;
    std::string err;
    CHECK(effectFromJson(bad, &e, &err) && e == Effect(RefAngle{r, kAxisY, 45.f}));
    bad.set("axis", (int64_t)3);
    CHECK(!effectFromJson(bad, &e, &err) && !err.empty());   // out-of-range axis is rejected, not clamped
    CaptureHub h;
    h.setEnabled(true);
    std::vector<EffectTxn> out;
    h.setSink([&](EffectTxn&& t) { out.push_back(std::move(t)); });
    h.begin(qcause(1, 1));
    CHECK(h.record(RefAngle{r, kAxisZ, 180.f}));
    h.end();
    CHECK(out.size() == 1 && out[0].effects.size() == 1 && std::holds_alternative<RefAngle>(out[0].effects[0]));
    CHECK(std::string(effectName(Effect(RefAngle{}))) == "RefAngle");
}

TEST(effect_json_roundtrip_every_type) {
    FormRef q("falloutnv.esm", 0x10A214);
    std::vector<Effect> all = {SetStage{q, 10}, SetObjective{q, 20, 3}, SetGlobal{q, 2.5f}, EnableRef{q}, DisableRef{q},
        AddItem{q, 5, Recipient::all()}, RemoveItem{q, 1, Recipient::slots(3)}, ModActorValue{24, 100, Recipient::origin()},
        ModReputation{q, 2, -1, Recipient::dflt()}, ModKarma{-5, Recipient::dflt()}, StartQuest{q}, StopQuest{q},
        MoveRef{q, FormRef("falloutnv.esm", 0x99), 1.5f, -2.f, 3.f}, KillActor{q}, ResurrectActor{q},
        SetScriptVar{q, 7, 3.25}, CompleteQuest{q}, CompleteAllObjectives{q}, SetLock{q, true, 50},
        SetFactionRank{q, FormRef("falloutnv.esm", 0x42), -1}, SetPerk{q, 2, Recipient::all()}, FailQuest{q}, SetPlayerControls{0x0B, false, Recipient::origin()},
        RefAddItem{q, q, 3}, RefRemoveItem{q, q, 1}, RemoveAllItems{q}, RefScriptVar{q, 4, 2.5}, AddTopic{q}, AddNote{q}, RemoveNote{q},
        RequestSave{kSaveForce}, SetWeather{q, true}, SetTimeScale{30.f}, RefAngle{q, kAxisZ, 90.5f}, RefAngle{q, kAxisX, -12.25f}, DespawnRef{0xFF000001u}, CompanionOwner{q, 0xFF}, SetMapMarker{q, kMarkerVisible | kMarkerTravel}};
    {
        ObjectState os;
        os.ref = q; os.mask = kObjLock | kObjOpen; os.locked = true; os.open = true; os.level = 50;
        all.push_back(os);
        SpawnRef sp;
        sp.id = 0xFF000042u; sp.base = q; sp.cell = FormRef("falloutnv.esm", 0x99); sp.x = 1; sp.y = 2; sp.z = 3; sp.leveled = true;
        all.push_back(sp);
    }
    EffectTxn t;
    t.cause = qcause(0x10A214, 10);
    t.effects = all;
    std::string text = txnToJson(t).dump();
    nvcoop::json::Value v;
    std::string err;
    CHECK(nvcoop::json::parse(text, v, err));
    EffectTxn back;
    CHECK(txnFromJson(v, &back, &err));
    CHECK(back.effects.size() == all.size());
    CHECK(back.effects == all);
    for (size_t i = 0; i < all.size() && i < back.effects.size(); ++i)
        if (!(all[i] == back.effects[i])) std::printf("    mismatch at effect #%zu (variant index %zu)\n", i, all[i].index());
    CHECK(back.cause == t.cause);
    CHECK(text.find("\"quest\":\"falloutnv.esm:10A214\"") != std::string::npos);  // oracle dict form
    nvcoop::json::Value bad = nvcoop::json::Value::object();
    bad.set("type", "Nope");
    Effect e;
    CHECK(!effectFromJson(bad, &e, &err));
}

TEST(suppress_host_never_blocks) {
    SuppressPolicy p;  // host / client: guest=false
    p.enterScript();
    CHECK(!p.blockMutation());
    CHECK(!p.blockResultScript());
    CHECK(!p.skipScript(kScriptTypeQuest));
    CHECK(p.stats().mutations == 0 && p.stats().questScripts == 0);
}

TEST(suppress_guest_blocks_script_mutations_only_inside_scripts) {
    SuppressPolicy p;
    p.setGuest(true);
    CHECK(!p.blockMutation());  // console / load path: no script running -> allowed
    p.enterScript();
    CHECK(p.blockMutation());   // OnDeath script of a locally killed ghost NPC tries SetStage/AddItem/Lock...
    p.leaveScript();
    CHECK(!p.blockMutation());
    CHECK(p.stats().mutations == 1);
}

TEST(suppress_guest_skips_quest_scripts_but_not_object_scripts) {
    SuppressPolicy p;
    p.setGuest(true);
    CHECK(p.skipScript(kScriptTypeQuest));
    CHECK(!p.skipScript(kScriptTypeObject));
    CHECK(!p.skipScript(kScriptTypeMagic));
    CHECK(p.stats().questScripts == 1);
    p.setQuestGate(false);
    CHECK(!p.skipScript(kScriptTypeQuest));
}

TEST(suppress_guest_result_scripts_never_run_locally) {
    SuppressPolicy p;
    p.setGuest(true);
    CHECK(p.blockResultScript());
    CHECK(p.stats().resultScripts == 1);
}

TEST(guest_console_line_on_own_player_is_local) {
    // per-machine: the guest's own player
    CHECK(SuppressPolicy::isLocalPlayerConsoleLine("player.moveto 104C70"));
    CHECK(SuppressPolicy::isLocalPlayerConsoleLine("  Player.SetAV Health 5000"));
    CHECK(SuppressPolicy::isLocalPlayerConsoleLine("PLAYER.setpos x 100\r\n"));
    // shared state stays blocked on a guest
    CHECK(!SuppressPolicy::isLocalPlayerConsoleLine("setstage VMS01 10"));
    CHECK(!SuppressPolicy::isLocalPlayerConsoleLine("104C70.moveto player"));
    CHECK(!SuppressPolicy::isLocalPlayerConsoleLine("playerref.disable"));
    CHECK(!SuppressPolicy::isLocalPlayerConsoleLine("player.moveto 104C70\nsetstage VMS01 10"));
    CHECK(!SuppressPolicy::isLocalPlayerConsoleLine(""));
    CHECK(!SuppressPolicy::isLocalPlayerConsoleLine(nullptr));
}

// ---- explicit local-player scope (SuppressPolicy::enterLocalPlayerScope, nvhooks::LocalPlayerScope) ----
// What h_ScriptExec does for one script run, game-free: the per-machine decision, then the bracket it opens, then what a mutator called inside sees.
namespace {
constexpr uint32_t kConsoleTemp = 0xFF000123u;   // a console / bridge line is a temporary script: refID 0 or 0xFF......
constexpr uint32_t kEsmScript = 0x00123456u;
bool mutationBlockedInRun(SuppressPolicy& p, uint32_t scriptRefID, const char* text) {
    const bool local = p.runsLocal(scriptRefID, [&] { return text; });
    if (local) p.enterLocalScript(scriptRefID); else p.enterScript();
    const bool blocked = p.blockMutation();
    if (local) p.leaveLocalScript(scriptRefID); else p.leaveScript();
    return blocked;
}
}  // namespace

TEST(local_scope_guest_character_line_passes_inside_and_is_dropped_outside) {
    SuppressPolicy p;
    p.setGuest(true);
    // outside the scope the old rules hold: a bare line (SetAV, SetSPECIALPoints, SetTagSkills) is a shared-state mutation and is dropped
    CHECK(mutationBlockedInRun(p, kConsoleTemp, "SetSPECIALPoints 40"));
    CHECK(mutationBlockedInRun(p, 0, "SetAV Strength 5"));
    CHECK(!p.inLocalPlayerScope());
    p.enterLocalPlayerScope();
    CHECK(p.inLocalPlayerScope());
    CHECK(!mutationBlockedInRun(p, kConsoleTemp, "SetSPECIALPoints 40"));
    CHECK(!mutationBlockedInRun(p, 0, "SetTagSkills 3 1"));
    CHECK(!mutationBlockedInRun(p, kConsoleTemp, "SetAV Strength 5"));
    p.leaveLocalPlayerScope();
    CHECK(!p.inLocalPlayerScope());
    CHECK(mutationBlockedInRun(p, kConsoleTemp, "SetAV Strength 5"));   // and the old rules are back
}

TEST(local_scope_run_is_per_machine_so_the_host_never_captures_it) {
    // The hook mutes the capture hub for every run this answers true for (engine_hooks.cpp: `local` -> pushMute), on a host as on a guest.
    SuppressPolicy host;   // guest = false
    CHECK(!host.runsLocal(kConsoleTemp, [] { return "SetAV Strength 5"; }));   // no scope: an ordinary run, captured as before
    host.enterLocalPlayerScope();
    CHECK(host.runsLocal(kConsoleTemp, [] { return "SetAV Strength 5"; }));
    CHECK(host.runsLocal(0, [] { return "ShowRaceMenu"; }));
    host.leaveLocalPlayerScope();
    CHECK(!host.runsLocal(kConsoleTemp, [] { return "SetAV Strength 5"; }));
    // the hook's side of it on a host: the run is muted, so what the line mutates is not captured (and so not sent); outside the scope it is captured
    CaptureHub h;
    h.setEnabled(true);
    std::vector<EffectTxn> out;
    h.setSink([&](EffectTxn&& t) { out.push_back(std::move(t)); });
    FormRef q("falloutnv.esm", 0x104EAE);
    auto runLine = [&](SuppressPolicy& p) {
        const bool local = p.runsLocal(kConsoleTemp, [] { return "SetObjective"; });
        if (local) h.pushMute();
        h.begin(qcause(1, 1));
        h.record(SetObjective{q, 70, 2});
        h.end();
        if (local) h.popMute();
    };
    runLine(host);
    CHECK(out.size() == 1);   // no scope
    host.enterLocalPlayerScope();
    runLine(host);
    CHECK(out.size() == 1);   // in the scope: nothing new captured
    host.leaveLocalPlayerScope();
    runLine(host);
    CHECK(out.size() == 2);
}

TEST(local_scope_exempts_console_lines_only_not_esm_scripts) {
    SuppressPolicy p;
    p.setGuest(true);
    p.enterLocalPlayerScope();
    // an object script an event raised inside the scope is not the guest's own character work: it keeps the rules of any other script
    CHECK(mutationBlockedInRun(p, kEsmScript, nullptr));
    CHECK(!p.runsLocal(kEsmScript, [] { return "player.SetAV Strength 5"; }));
    p.leaveLocalPlayerScope();
}

TEST(local_scope_nests_and_unbalanced_leave_is_harmless) {
    SuppressPolicy p;
    p.setGuest(true);
    p.enterLocalPlayerScope();
    p.enterLocalPlayerScope();
    p.leaveLocalPlayerScope();
    CHECK(p.inLocalPlayerScope());   // the outer scope is still open
    CHECK(!mutationBlockedInRun(p, kConsoleTemp, "SetAV Luck 5"));
    p.leaveLocalPlayerScope();
    CHECK(!p.inLocalPlayerScope());
    CHECK(mutationBlockedInRun(p, kConsoleTemp, "SetAV Luck 5"));
    p.leaveLocalPlayerScope();   // unbalanced
    CHECK(!p.inLocalPlayerScope());
    p.enterLocalPlayerScope();
    CHECK(p.inLocalPlayerScope());   // the counter did not go negative
    p.leaveLocalPlayerScope();
}

TEST(local_scope_leaves_the_player_prefix_rule_and_the_other_exemptions_alone) {
    SuppressPolicy p;
    p.setGuest(true);
    p.setGuestLocalRun({kEsmScript});
    // a typed "player." line still passes, scope or not; anything else typed on a guest console stays blocked
    CHECK(!mutationBlockedInRun(p, kConsoleTemp, "player.SetAV Health 5000"));
    CHECK(mutationBlockedInRun(p, kConsoleTemp, "SetStage VMS01 10"));
    p.enterLocalPlayerScope();
    CHECK(!mutationBlockedInRun(p, kConsoleTemp, "player.SetAV Health 5000"));
    CHECK(!mutationBlockedInRun(p, kEsmScript, nullptr));   // a GuestLocalRun script is still per-machine
    p.leaveLocalPlayerScope();
    CHECK(!mutationBlockedInRun(p, kEsmScript, nullptr));
}

TEST(suppress_lifted_while_applying_host_txn) {
    SuppressPolicy p;
    p.setGuest(true);
    p.enterScript();
    p.beginApply();
    CHECK(!p.blockMutation());
    CHECK(!p.blockResultScript());
    CHECK(!p.skipScript(kScriptTypeQuest));
    p.beginApply();  // nested apply
    p.endApply();
    CHECK(p.applying());
    p.endApply();
    CHECK(!p.applying());
    CHECK(p.blockMutation());  // and back on afterwards
    p.endApply();              // unbalanced end is harmless
    CHECK(!p.applying());
}

TEST(change_tracker_reports_first_sight_and_changes_only) {
    ChangeTracker t;
    uint64_t k = ChangeTracker::varKey(0x10A214, 3), g = ChangeTracker::globalKey(0x35);
    CHECK(t.update(k, 1.0));    // baseline
    CHECK(!t.update(k, 1.0));   // unchanged -> silent
    CHECK(t.update(k, 2.0));
    CHECK(!t.update(k, 2.0));
    CHECK(t.update(g, 5.0));    // global namespace is separate from vars
    CHECK(k != g && ChangeTracker::varKey(1, 2) != ChangeTracker::varKey(2, 1));
    CHECK(t.size() == 2);
    t.clear();
    CHECK(t.update(k, 2.0));
}

TEST(change_tracker_priming_takes_the_present_world_as_known) {
    // A host without a world baseline: the first poll pass must only record what is already there (no flood of every quest var), later
    // passes report changes. Before this the host did not poll at all without a baseline.
    ChangeTracker t;
    const uint64_t ganger = ChangeTracker::varKey(0x104EAE, 1), idle = ChangeTracker::varKey(0x104EAE, 2), g = ChangeTracker::globalKey(0x35);
    t.beginPrime();
    CHECK(t.priming());
    CHECK(!t.update(ganger, 0));
    CHECK(!t.update(idle, 5));
    CHECK(!t.update(g, 12.0));
    t.endPrime();
    CHECK(!t.priming());
    CHECK(!t.update(ganger, 0) && !t.update(idle, 5) && !t.update(g, 12.0));   // nothing changed since the priming pass
    CHECK(t.update(ganger, 1));                                                   // a ganger died on the host
    CHECK(!t.update(ganger, 1));
    CHECK(t.update(ganger, 2));
    CHECK(t.update(g, 13.0));
    // a value that moved DURING the priming pass is also only recorded, and the next change is still seen
    ChangeTracker u;
    u.beginPrime();
    CHECK(!u.update(ganger, 0));
    CHECK(!u.update(ganger, 3));
    u.endPrime();
    CHECK(!u.update(ganger, 3) && u.update(ganger, 4));
    // without priming the first pass reports everything (the baseline host: the replicator drops what the baseline already holds)
    ChangeTracker v;
    CHECK(v.update(ganger, 0) && v.update(idle, 5));
}

TEST(guest_objvars_table_lists_the_quest_vars_that_object_scripts_write) {
    const ObjectVarWrites& t = ObjectVarWrites::builtin();
    CHECK(t.scripts() > 500 && t.rows() > 1500);   // generated from the game's own scripts (tools/gen_guest_objvars.py)
    // the VMS16 gangers' OnDeath scripts: `set VMS16.nGangerDeathCount to VMS16.nGangerDeathCount + 1`
    int gangerWriters = 0;
    for (uint32_t id : {0x104C69u, 0x105D4Du}) {
        const std::vector<ObjVarWrite>* w = t.of(FormRef("falloutnv.esm", id));
        CHECK(w != nullptr);
        if (!w) continue;
        for (const ObjVarWrite& x : *w)
            if (x.quest == FormRef("falloutnv.esm", 0x104EAE) && x.var == "ngangerdeathcount") ++gangerWriters;
    }
    CHECK(gangerWriters == 2);
    CHECK(t.of(FormRef("falloutnv.esm", 0x104EAE)) == nullptr);   // a quest is not an object script
    CHECK(t.of(FormRef("nosuch.esm", 0x104C69)) == nullptr);
    // every row is well formed: lower-case variable, real forms
    ObjectVarWrites mini;
    mini.add(FormRef("a.esm", 5), FormRef("a.esm", 9), "bflag");
    mini.add(FormRef("a.esm", 5), FormRef("a.esm", 9), "bflag");   // duplicate rows collapse
    mini.add(FormRef("a.esm", 5), FormRef("a.esm", 9), "ncount");
    mini.add(FormRef("a.esm", 2), FormRef("b.esm", 1), "x");
    CHECK(mini.scripts() == 2 && mini.rows() == 3);
    CHECK(mini.of(FormRef("a.esm", 5)) && mini.of(FormRef("a.esm", 5))->size() == 2 && mini.of(FormRef("a.esm", 2))->size() == 1);
    CHECK(mini.of(FormRef("a.esm", 3)) == nullptr);
}

TEST(var_rollback_puts_back_only_what_a_script_changed) {
    double a = 2, b = 7, c = 0;
    VarRollback rb;
    rb.remember(&a);
    rb.remember(&b);
    rb.remember(nullptr);   // an unresolved slot is ignored
    CHECK(rb.size() == 2);
    a = 3;   // the script wrote one of them
    CHECK(rb.restore() == 1);
    CHECK(a == 2 && b == 7 && c == 0);
    CHECK(rb.size() == 0 && rb.restore() == 0);   // spent
    // nested scripts (an outer script calls an inner one that writes the same var): the outermost restore wins
    VarRollback outer, inner;
    outer.remember(&a);
    inner.remember(&a);
    a = 9;
    CHECK(inner.restore() == 1 && a == 2);
    a = 11;
    CHECK(outer.restore() == 1 && a == 2);
}

TEST(guest_ganger_death_scripts_do_not_count_on_the_guest_the_host_value_wins) {
    // The retest: three gangers die; their OnDeath scripts run on the host AND on the guest (object scripts still run on a guest). The host's
    // count is polled and replicated; the guest's own increments must not stay (it ended at 1..2 / 0 while the host had 3).
    double hostVar = 0, guestVar = 0;
    int rolledBack = 0;
    auto onDeath = [](double& v) { v = v + 1; };   // set VMS16.nGangerDeathCount to VMS16.nGangerDeathCount + 1
    for (int death = 1; death <= 3; ++death) {
        onDeath(hostVar);
        VarRollback rb;                 // the guest brackets the same script (engine_hooks.cpp h_ScriptExec)
        rb.remember(&guestVar);
        onDeath(guestVar);
        rolledBack += static_cast<int>(rb.restore());
        CHECK(guestVar == double(death - 1));   // never ahead of the last replicated host value
        guestVar = hostVar;                     // the replicated SetScriptVar arrives (RemoteApplier writes the slot directly)
    }
    CHECK(rolledBack == 3);
    CHECK(hostVar == 3 && guestVar == 3);
}

TEST(apply_lines_pin_command_names_and_parameter_order) {
    CHECK(killCmd() == "KillActor");
    CHECK(refOpNeedsModel(fnvcoop::kOpPlayGroup) && refOpNeedsModel(fnvcoop::kOpPlayIdle));   // story fix 2026-10-28: no animation order on a reference without a model (the S08 crash)
    CHECK(!refOpNeedsModel(fnvcoop::kOpModFactionRank) && !refOpNeedsModel(fnvcoop::kOpSetCriticalStage));
    CHECK(resurrectCmd(false) == "ResurrectActor 0");
    CHECK(resurrectCmd(true) == "ResurrectActor 1");
    CHECK(moveToMarkerCmd(0x32, 1.5f, -2.f, 0.f) == "MoveTo 00000032 1.5000 -2.0000 0.0000");
    CHECK(setPosCmd('Z', 12.25f) == "SetPos Z 12.2500");
    CHECK(setAngleCmd('Z', 90.5f) == "SetAngle Z 90.5000" && setAngleCmd('X', -12.25f) == "SetAngle X -12.2500");
    CHECK(axisIndexOf('X') == 0 && axisIndexOf('y') == 1 && axisIndexOf('Z') == 2 && axisIndexOf('z') == 2 && axisIndexOf(0) == -1 && axisIndexOf('Q') == -1);
    CHECK(setFactionRankCmd(0x104C22, -1) == "SetFactionRank 00104C22 -1");
    CHECK(removeFromFactionCmd(0x104C22) == "RemoveFromFaction 00104C22");
    CHECK(addPerkCmd(0x1234) == "AddPerk 00001234" && removePerkCmd(0x1234) == "RemovePerk 00001234");
    CHECK(lockCmd(-1) == "Lock" && lockCmd(50) == "Lock 50" && unlockCmd() == "UnLock");
    CHECK(setOpenStateCmd(true) == "SetOpenState 1" && setDestroyedCmd(true) == "SetDestroyed 1");
    CHECK(addNoteCmd(0x77) == "AddNote 00000077" && removeNoteCmd(0x77) == "RemoveNote 00000077" && addTopicCmd(0x9) == "AddTopic 00000009");
    CHECK(setWeatherCmd(0x5, false) == "SetWeather 00000005" && setWeatherCmd(0x5, true) == "ForceWeather 00000005");
    CHECK(setTimeScaleCmd(30.f) == "SetGlobalTimeMultiplier 30.0000");
    CHECK(showMapCmd(0x55, false) == "ShowMap 00000055" && showMapCmd(0x55, true) == "ShowMap 00000055 1");
    CHECK(addItemCmd(0xF, 100) == "AddItem 0000000F 100" && removeItemCmd(0xF, 3) == "RemoveItem 0000000F 3" && removeAllItemsCmd() == "RemoveAllItems");
    CHECK(saveCmd(0) == "Autosave" && saveCmd(1) == "SystemSave" && saveCmd(2) == "ForceSave");
    CHECK(isReferenceFormType(0x3A) && isReferenceFormType(0x3B) && isReferenceFormType(0x3C));  // player (0x14) is an ACHR
    CHECK(!isReferenceFormType(0x39) && !isReferenceFormType(0x06));
}

TEST(engine_table_pins_stack_bytes_of_hooked_functions) {
    // Same numbers the x86 glue static_asserts against its detour signatures (engine_hooks.cpp, PIN_F / PIN_C).
    CHECK(addr::ScriptExecute.retBytes == 16);
    CHECK(addr::SetStage.retBytes == 4);
    CHECK(addr::TopicInfoRunResult.retBytes == 8);
    CHECK(addr::ObjectiveSetState.retBytes == 4);
    CHECK(addr::RepAdd.retBytes == 8 && addr::RepSet.retBytes == 8);
    CHECK(addr::PlayerRewardKarma.retBytes == 4);
    CHECK(addr::RefSetDisabled.retBytes == 4);
    CHECK(addr::RefActivate.retBytes == 16);
    CHECK(addr::MarkScriptEvent.retBytes == 0);
    CHECK(addr::ActorSetTeammate.retBytes == 4);
    CHECK(addr::QuestCompleteAllObjectives.retBytes == 4);
    CHECK(addr::PlayerSetControlsDisabled.retBytes == 8);
    CHECK(addr::Cmd_AddItem.retBytes == 0 && addr::Cmd_KillActor.retBytes == 0);
    CHECK(addr::ConsolePrint.retBytes == 8);  // ConsoleManager::Print(this, fmt, va_list): the function we once mis-hooked as cdecl
}

TEST(stage_gate_defers_unless_applying_or_bypassed) {
    StageGate g;
    int asked = 0;
    CHECK(g.allow(1, 5, false));  // no gate installed
    g.set([&](uint32_t, uint32_t s) { ++asked; return s != 20; });  // defer stage 20 (barrier)
    CHECK(g.allow(1, 10, false));
    CHECK(!g.allow(1, 20, false));
    CHECK(g.deferred() == 1);
    CHECK(g.allow(1, 20, true));   // our own remote apply is never gated
    g.beginBypass();
    CHECK(g.allow(1, 20, false));  // the owner re-issuing the deferred stage
    g.beginBypass();
    g.endBypass();
    CHECK(g.allow(1, 20, false));  // nested bypass
    g.endBypass();
    CHECK(!g.allow(1, 20, false)); // and gated again afterwards
    g.endBypass();                 // unbalanced end is harmless
    CHECK(asked == 3);             // bypass/applying paths do not even consult the callback
}

TEST(stale_scope_is_reaped_at_frame_boundary) {
    CaptureHub h;
    h.setEnabled(true);
    std::vector<EffectTxn> out;
    h.setSink([&](EffectTxn&& t) { out.push_back(std::move(t)); });
    h.begin(qcause(0x100, 36));
    h.begin(qcause(0x100, 37));  // nested; neither end() ever runs (destructor skipped by an engine longjmp/SEH unwind)
    CHECK(h.record(stage(0x100, 36)));
    CHECK(h.record(StartQuest{FormRef("falloutnv.esm", 0x100)}));
    CHECK(out.empty());  // the VCG00-36 symptom: recorded 2, emitted 0
    CHECK(h.reapStale() == 2);
    CHECK(out.size() == 1 && out[0].effects.size() == 2);
    CHECK(!out.empty() && out[0].cause.aux == 36);  // outermost cause
    CHECK(h.depth() == 0 && !h.active());
    CHECK(h.stats().staleScopes == 1);
    // subsequent captures work again
    h.begin(qcause(0x200, 5));
    h.record(stage(0x200, 5));
    h.end();
    CHECK(out.size() == 2);
    CHECK(h.reapStale() == 0);  // nothing open: no-op
}

TEST(consecutive_identical_state_effects_collapse_but_quantities_do_not) {
    CaptureHub h;
    h.setEnabled(true);
    std::vector<EffectTxn> out;
    h.setSink([&](EffectTxn&& t) { out.push_back(std::move(t)); });
    FormRef r("falloutnv.esm", 0x101C9E);
    h.begin(qcause(1, 1));
    h.record(EnableRef{r});
    h.record(EnableRef{r});   // second primitive of the same Enable command
    h.record(DisableRef{r});  // a real state change afterwards is kept
    h.record(DisableRef{r});
    h.record(AddItem{r, 1, Recipient::dflt()});
    h.record(AddItem{r, 1, Recipient::dflt()});  // two real pickups: never collapsed
    h.end();
    CHECK(out.size() == 1 && out[0].effects.size() == 4);
    CHECK(h.stats().collapsed == 2);
    // a new scope starts clean: the same effect in the next txn is recorded again
    h.begin(qcause(1, 2));
    h.record(DisableRef{r});
    h.end();
    CHECK(out.size() == 2 && out[1].effects.size() == 1);
}

TEST(failed_quest_txn_carries_fail_only_not_complete) {
    CaptureHub h;
    h.setEnabled(true);
    std::vector<EffectTxn> out;
    h.setSink([&](EffectTxn&& t) { out.push_back(std::move(t)); });
    FormRef q("falloutnv.esm", 0x15D912), other("falloutnv.esm", 0x104EAE);
    h.begin(qcause(0x15D912, 255));
    h.record(StopQuest{q});
    h.record(CompleteQuest{q});  // QuestStageItem::RunResult: SetCompleted(1) ...
    h.record(FailQuest{q});      // ... then SetFailed(1)
    h.record(CompleteQuest{other});  // a different quest that merely completed keeps its CompleteQuest
    h.end();
    CHECK(out.size() == 1 && out[0].effects.size() == 3);
    int fails = 0, completes = 0;
    for (auto& e : out[0].effects) {
        fails += std::holds_alternative<FailQuest>(e);
        if (auto* c = std::get_if<CompleteQuest>(&e)) { ++completes; CHECK(c->quest == other); }
    }
    CHECK(fails == 1 && completes == 1);
}

TEST(inventory_delta_reports_resolved_items_not_the_list) {
    // VCG02 stage 40: the script adds LEVELED LIST 0x96C06; the player actually gained 0x7EA24 (+1) - that is what must be captured.
    Inventory before{{0xF, 100}, {0x1111, 1}};
    Inventory after{{0xF, 100}, {0x1111, 1}, {0x7EA24, 1}};
    auto g = inventoryGained(before, after);
    CHECK(g.size() == 1 && g[0].first == 0x7EA24 && g[0].second == 1);
    CHECK(!isResolvingItemType(0x28));                       // plain item: record the command's own item
    CHECK(isResolvingItemType(0x34) && isResolvingItemType(0x55));  // leveled list, form list
    // removal sets (RemoveAllTypedItems): exact removed counts
    Inventory after2{{0xF, 90}, {0x1111, 0}};
    auto l = inventoryLost(before, after2);
    CHECK(l.size() == 2);
    CHECK(inventoryGained(before, before).empty() && inventoryLost(before, before).empty());
    // stacked count increases are reported as the delta, not the total
    Inventory more{{0xF, 130}, {0x1111, 1}};
    auto g2 = inventoryGained(before, more);
    CHECK(g2.size() == 1 && g2[0].first == 0xF && g2[0].second == 30);
}

TEST(weak_script_scope_captures_ambient_mutations_with_script_cause) {
    CaptureHub h;
    h.setEnabled(true);
    std::vector<EffectTxn> out;
    h.setSink([&](EffectTxn&& t) { out.push_back(std::move(t)); });
    h.setWeakCauseResolver([](uint32_t id) {
        Cause c;
        c.kind = CauseKind::Script;
        c.source = FormRef("falloutnv.esm", id);
        return c;
    });
    FormRef q("falloutnv.esm", 0x104EAE);
    // quest GameMode block / object script: no explicit scope open -> implicit scope instead of droppedNoScope
    CHECK(h.beginWeak(0x1234));
    CHECK(h.active() && h.depth() == 1);
    CHECK(h.record(SetObjective{q, 70, 2}));
    h.endWeak();
    CHECK(out.size() == 1 && out[0].cause.kind == CauseKind::Script && out[0].cause.source.localId == 0x1234);
    CHECK(h.stats().droppedNoScope == 0 && h.stats().weakTxns == 1);
    // an empty script run costs nothing: no txn, not even an "empty scope"
    CHECK(h.beginWeak(0x1));
    h.endWeak();
    CHECK(out.size() == 1 && h.stats().emptyScopes == 0);
    // never opens inside another scope
    h.begin(qcause(1, 1));
    CHECK(!h.beginWeak(0x2));
    h.end();
}

TEST(muted_hub_captures_nothing_for_guest_local_scripts) {
    CaptureHub h;
    h.setEnabled(true);
    std::vector<EffectTxn> out;
    h.setSink([&](EffectTxn&& t) { out.push_back(std::move(t)); });
    FormRef q("falloutnv.esm", 0x104EAE);
    h.pushMute();
    CHECK(!h.enabled() && !h.beginWeak(0x1));
    h.begin(qcause(1, 1));          // a SetStage inside the local script
    CHECK(!h.active() && !h.record(SetObjective{q, 70, 2}));
    h.end();
    h.popMute();
    CHECK(out.empty() && h.enabled());
    h.begin(qcause(1, 1));
    CHECK(h.record(SetObjective{q, 70, 2}));
    h.end();
    CHECK(out.size() == 1);
}

TEST(explicit_scope_inside_weak_scope_keeps_its_own_cause) {
    CaptureHub h;
    h.setEnabled(true);
    std::vector<EffectTxn> out;
    h.setSink([&](EffectTxn&& t) { out.push_back(std::move(t)); });
    h.setWeakCauseResolver([](uint32_t id) {
        Cause c;
        c.kind = CauseKind::Script;
        c.source = FormRef("falloutnv.esm", id);
        return c;
    });
    FormRef r("falloutnv.esm", 0x777);
    // object script (elevator): player.MoveTo, then SetStage of a rule-bearing quest, then more
    CHECK(h.beginWeak(0xABC));
    h.record(MoveRef{FormRef("falloutnv.esm", 0x14), r, 0, 0, 0});
    h.begin(qcause(0x15D912, 10));
    h.record(stage(0x15D912, 10));
    h.end();
    h.record(EnableRef{r});
    h.endWeak();
    CHECK(out.size() == 3);
    CHECK(out.size() == 3 && out[0].cause.kind == CauseKind::Script && std::holds_alternative<MoveRef>(out[0].effects[0]));
    CHECK(out.size() == 3 && out[1].cause.kind == CauseKind::QuestResult && std::holds_alternative<SetStage>(out[1].effects[0]));
    CHECK(out.size() == 3 && out[2].cause.kind == CauseKind::Script && std::holds_alternative<EnableRef>(out[2].effects[0]));
    CHECK(h.depth() == 0);
}

TEST(acting_context_is_a_stack_defaulting_to_the_host_player) {
    ActingContext a;
    CHECK(a.current() == 0x14);
    a.push(0xFF001234);
    CHECK(a.current() == 0xFF001234);
    a.push(0);  // unknown ghost -> falls back to the host player
    CHECK(a.current() == 0x14);
    a.pop();
    CHECK(a.current() == 0xFF001234);
    a.pop();
    a.pop();  // unbalanced pop is harmless
    CHECK(a.current() == 0x14 && a.depth() == 0);
}

TEST(guest_local_run_allow_list_keeps_selected_quest_scripts_running) {
    SuppressPolicy p;
    p.setGuest(true);
    p.setGuestLocalRun({0x500, 0x600});
    CHECK(!p.skipScript(kScriptTypeQuest, 0x500));  // allow-listed (achievement / rumble / DLC setup script)
    CHECK(p.skipScript(kScriptTypeQuest, 0x700));   // any other quest script is still skipped
    CHECK(p.skipScript(kScriptTypeQuest, 0));       // unknown script id: skipped
    CHECK(p.stats().questScripts == 2);
    p.setGuestLocalRun({});
    CHECK(p.skipScript(kScriptTypeQuest, 0x500));   // list replaced
}

TEST(guest_local_run_script_may_mutate_on_guest_but_others_may_not) {
    SuppressPolicy p;
    p.setGuest(true);
    p.setGuestLocalRun({0x500});
    p.enterScript();                        // ordinary quest-driven/object script on a guest
    CHECK(p.blockMutation());
    p.leaveScript();
    CHECK(p.isGuestLocalRun(0x500) && !p.isGuestLocalRun(0x501) && !p.isGuestLocalRun(0));
    p.enterLocalScript();                   // allow-listed DLC setup script
    CHECK(!p.blockMutation());
    p.enterScript();                        // a nested ordinary script inherits the exemption (called from the local one)
    CHECK(!p.blockMutation());
    p.leaveScript();
    p.leaveLocalScript();
    p.enterScript();
    CHECK(p.blockMutation());               // exemption ended with the local script
    p.leaveScript();
    CHECK(p.localScriptDepth() == 0 && p.scriptDepth() == 0);
}

TEST(guest_local_run_data_file_parses) {
    GuestLocalRunList l;
    std::string err;
    const char* ok = R"({"scripts":[{"quest":"NVDLC02MQ00","script":"HonestHearts.esm:008A32"},
        {"quest":"Bad1","script":"nocolon"},{"quest":"Bad2","script":"X.esm:zz"},{"quest":"Bad3"},
        {"quest":"VDLCPackQuest","script":"FalloutNV.esm:17912E"}]})";
    CHECK(parseGuestLocalRun(ok, &l, &err));
    CHECK(l.scripts.size() == 2 && l.skipped == 3);
    CHECK(l.scripts[0].plugin == "honesthearts.esm" && l.scripts[0].localId == 0x8A32 && l.quests[0] == "NVDLC02MQ00");
    CHECK(l.scripts[1].localId == 0x17912E);
    CHECK(!parseGuestLocalRun("{\"nope\":1}", &l, &err) && !err.empty());
    CHECK(!parseGuestLocalRun("not json", &l, &err));
}

TEST(guest_local_run_shipped_list_covers_required_setup_quests) {
    std::string path = std::string(NVH_DATA_DIR) + "/guest_local_run.json";
    FILE* f = fopen(path.c_str(), "rb");
    CHECK(f != nullptr);
    if (!f) return;
    std::string text;
    char buf[4096];
    size_t n;
    while ((n = fread(buf, 1, sizeof buf, f)) > 0) text.append(buf, n);
    fclose(f);
    GuestLocalRunList l;
    std::string err;
    CHECK(parseGuestLocalRun(text, &l, &err));
    CHECK(l.skipped == 0);
    auto has = [&](const char* q) { for (auto& x : l.quests) if (x == q) return true; return false; };
    for (const char* q : {"NVDLC01MQ00", "NVDLC02MQ00", "NVDLC03MQ00", "NVDLC04MQ00", "NVDLC05MQ00", "VDLCPackQuest", "AchievementQuest",
                          "ControlRumble", "AutoDocTimerQuest", "ImplantTimerQuest"})
        CHECK(has(q));
    for (auto& r : l.scripts) CHECK(r.localId != 0 && !r.plugin.empty());
}

TEST(guest_local_run_two_game_diff_quests_are_per_machine) {
    std::string path = std::string(NVH_DATA_DIR) + "/guest_local_run.json";
    FILE* f = fopen(path.c_str(), "rb");
    CHECK(f != nullptr);
    if (!f) return;
    std::string text;
    char buf[4096];
    size_t n;
    while ((n = fread(buf, 1, sizeof buf, f)) > 0) text.append(buf, n);
    fclose(f);
    GuestLocalRunList l;
    std::string err;
    CHECK(parseGuestLocalRun(text, &l, &err));
    auto idx = [&](const char* q) { for (size_t i = 0; i < l.quests.size(); ++i) if (l.quests[i] == q) return (int)i; return -1; };
    // per-machine: documented with a reason
    for (const char* q : {"VStreetLighting", "NVDLC02RainQuest", "NVDLC02RadioMessage", "VDLCPackQuest"}) {
        int i = idx(q);
        CHECK(i >= 0);
        if (i >= 0) CHECK(!l.reasons[i].empty());
    }
    CHECK(idx("VCG04") >= 0);
    // NOT per-machine: encounter spawners (a guest copy would spawn a second set of creatures) and story quests
    for (const char* q : {"VERNovac01", "VERStalkersSouth01", "VCG02", "VMS01", "VMS21"}) CHECK(idx(q) < 0);
}

TEST(per_machine_quest_vars_are_not_shared) {
    SuppressPolicy p;
    p.setGuestLocalRun({0x500, 0x600});
    CHECK(!p.sharesQuestVars(0x500) && !p.sharesQuestVars(0x600));  // GuestLocalRun quest: vars stay local
    CHECK(p.sharesQuestVars(0x501));                                // story quest: shared as before
    CHECK(p.sharesQuestVars(0));                                    // quest without a script
}

TEST(acting_slot_maps_ghosts_to_party_slots) {
    ActingContext a;
    CHECK(a.slot() == 0);                       // host player
    a.registerSlot(0xFF001234, 2);
    a.push(0xFF001234);
    CHECK(a.slot() == 2);
    a.push(0xFF00DEAD);                         // unregistered ghost -> host slot
    CHECK(a.slot() == 0);
    a.pop();
    a.registerSlot(0xFF001234, 3);              // re-register replaces
    CHECK(a.slot() == 3);
    a.pop();
    CHECK(a.slot() == 0);
}

TEST(companion_apply_is_for_the_owner_machine_only) {
    CHECK(companionForMe(1, 1) && !companionForMe(0, 1) && !companionForMe(1, 0) && companionForMe(0, 0));
    CHECK(setTeammateCmd(true) == "SetPlayerTeammate 1" && setTeammateCmd(false) == "SetPlayerTeammate 0");
}

TEST(fast_travel_locks_enable_travel_only_when_last_scope_releases) {
    FastTravelLocks l;
    FormRef a("falloutnv.esm", 1), b("falloutnv.esm", 2);
    CHECK(l.travelEnabled());
    CHECK(l.set(a, true) && !l.travelEnabled());   // first lock: state changes
    CHECK(!l.set(b, true));                        // second lock: no engine change
    CHECK(!l.set(a, false) && !l.travelEnabled()); // b still holds
    CHECK(l.set(b, false) && l.travelEnabled());   // last release re-enables
    CHECK(!l.set(b, false));                       // double release is a no-op
    CHECK(enableFastTravelCmd(true) == "EnableFastTravel 1" && enableFastTravelCmd(false) == "EnableFastTravel 0");
}

TEST(current_source_names_the_outermost_explicit_scope_or_weak_script) {
    CaptureHub h;
    h.setEnabled(true);
    h.setSink([](EffectTxn&&) {});
    FormRef q;
    CHECK(!h.currentSource(&q));                   // no scope
    {
        ScopedCapture sc(h, qcause(0x104C, 30));
        CHECK(h.currentSource(&q) && q.localId == 0x104C);
        ScopedCapture inner(h, qcause(0x2000, 5));
        CHECK(h.currentSource(&q) && q.localId == 0x104C);  // outermost wins
    }
    h.setWeakCauseResolver([](uint32_t id) { Cause c; c.kind = CauseKind::Script; c.source = FormRef("falloutnv.esm", id); return c; });
    CHECK(h.beginWeak(0x777));
    CHECK(h.currentSource(&q) && q.localId == 0x777);
    h.endWeak();
    CHECK(!h.currentSource(&q));
}

TEST(party_condition_swap_plan_only_touches_the_player_operand) {
    CHECK(planSwap(CondFunc::GetDistance, 0x1234, 0x14).param1 && !planSwap(CondFunc::GetDistance, 0x1234, 0x14).thisObj);
    CHECK(planSwap(CondFunc::GetDistance, 0x14, 0x1234).thisObj && !planSwap(CondFunc::GetDistance, 0x14, 0x1234).param1);
    CHECK(!planSwap(CondFunc::GetDistance, 0x1234, 0x5678).any());
    CHECK(planSwap(CondFunc::GetInCell, 0x14, 0x14).thisObj && !planSwap(CondFunc::GetInCell, 0x14, 0x14).param1);  // param1 is a cell, never inspected
    CHECK(!planSwap(CondFunc::IsInCombat, 0x1234, 0x14).any());
    CHECK(planSwap(CondFunc::GetDetected, 0x1234, 0x14).param1);
    CHECK(std::string(condFuncName(CondFunc::GetInSameCell)) == "GetInSameCell" && !param1IsReference(CondFunc::GetInCell));
}

TEST(party_condition_runs_once_per_member_and_reduces) {
    std::vector<PartyMember> party = {{0, 0x14}, {1, 0xFF000001}, {2, 0xFF000002}};
    int calls = 0;
    auto evalAs = [&](const PartyMember& m, double* v) { ++calls; *v = m.slot == 0 ? 900.0 : (m.slot == 1 ? 120.0 : 450.0); return true; };
    std::string seenFunc;
    uint8_t seenInit = 99;
    std::vector<CondValue> seen;
    auto reduce = [&](const std::string& f, const std::vector<CondValue>& v, uint8_t init) {
        seenFunc = f; seenInit = init; seen = v;
        double m = v[0].value;
        for (auto& x : v) if (x.value < m) m = x.value;  // min-over-party
        return m;
    };
    double out = -1;
    CHECK(evaluateOverParty(CondFunc::GetDistance, party, evalAs, reduce, 2, &out));
    CHECK(calls == 3 && out == 120.0 && seenFunc == "GetDistance" && seenInit == 2 && seen.size() == 3 && seen[1].slot == 1);
    // a ghost that is not loaded is skipped; the rest still reduce
    auto evalSkip = [&](const PartyMember& m, double* v) { if (m.slot == 1) return false; *v = m.slot ? 5.0 : 9.0; return true; };
    CHECK(evaluateOverParty(CondFunc::GetDistance, party, evalSkip, reduce, 0, &out) && out == 5.0 && seen.size() == 2);
    // fewer than two evaluated members: caller falls back to the engine result
    auto evalOne = [&](const PartyMember& m, double* v) { *v = 1; return m.slot == 0; };
    CHECK(!evaluateOverParty(CondFunc::GetDistance, party, evalOne, reduce, 0, &out));
    CHECK(!evaluateOverParty(CondFunc::GetDistance, {}, evalAs, reduce, 0, &out));
    // no reducer: initiator-less default = first value
    CHECK(evaluateOverParty(CondFunc::GetInCell, party, evalAs, nullptr, 0, &out) && out == 900.0);
}

namespace {
const char* kReducerRules = R"({"schema":1,"quests":{
 "Collect":{"tier":"T3","questline":"x","curated":false,"rules":[
   {"kind":"PartyAwareCond","params":{"funcs":{"GetItemCount":"sum-over-party","GetAV":"max-over-party","GetDistance":"min-over-party","GetInCell":"entry:all-in,exit:any-in","IsInCombat":"any-party-member"}},"origin":"derived"}]},
 "Plain":{"tier":"T3","questline":"x","curated":false,"rules":[
   {"kind":"PartyAwareCond","params":{"funcs":{"GetActorValue":"min-over-party"}},"origin":"derived"}]}
}})";
bool reducerResolve(const std::string& edid, fnvcoop::FormRef& out) {
    out = fnvcoop::FormRef("falloutnv.esm", edid == "Collect" ? 0x100u : edid == "Plain" ? 0x101u : 0x1FFu);
    return true;
}
// Runs one party condition the way the engine detour does: evaluateOverParty with the rules reducer behind it.
double overParty(fnvcoop::RulesEngine& eng, const fnvcoop::FormRef& quest, CondFunc f, const std::vector<double>& perSlot, uint8_t initiator,
                 fnvcoop::CondPhase phase = fnvcoop::CondPhase::Entry) {
    ConditionReducer red = makeRulesReducer(eng);
    std::vector<PartyMember> party;
    for (size_t i = 0; i < perSlot.size(); ++i) party.push_back({static_cast<uint8_t>(i), i == 0 ? kHostPlayerRef : 0xFF000000u + static_cast<uint32_t>(i)});
    double out = -12345;
    bool done = evaluateOverParty(
        f, party, [&](const PartyMember& m, double* v) { *v = perSlot[m.slot]; return true; },
        [&](const std::string& fn, const std::vector<CondValue>& vals, uint8_t init) { return red(quest, fn, vals, init, phase); }, initiator, &out);
    return done ? out : perSlot.front();
}
}  // namespace

TEST(party_conditions_reduce_through_the_rules_engine) {
    fnvcoop::InMemoryNetwork net(1);
    fnvcoop::HostSession host(*net.endpoint(0));
    fnvcoop::RulesDb db;
    std::string err;
    CHECK(db.load(kReducerRules, &err));
    fnvcoop::RulesEngine eng(host, db, reducerResolve);
    fnvcoop::FormRef collect, plain, none;
    reducerResolve("Collect", collect);
    reducerResolve("Plain", plain);
    reducerResolve("Other", none);
    // sum-over-party: two totems spread over the party reach a count of 2 (any-party-member would say 1)
    CHECK(overParty(eng, collect, CondFunc::GetItemCount, {1, 1, 0}, 0) == 2.0);
    // max-over-party, the rule spells the function GetAV and the engine evaluator is GetActorValue
    CHECK(overParty(eng, collect, CondFunc::GetActorValue, {60, 101, 80}, 0) == 101.0);
    CHECK(ruleFuncName(eng, collect, "GetActorValue") == "GetAV");
    CHECK(ruleFuncName(eng, plain, "GetActorValue") == "GetActorValue");
    CHECK(overParty(eng, plain, CondFunc::GetActorValue, {60, 101, 80}, 0) == 60.0);   // min-over-party under its own spelling
    // min-over-party: the closest member decides
    CHECK(overParty(eng, collect, CondFunc::GetDistance, {900, 40, 300}, 0) == 40.0);
    // any-party-member
    CHECK(overParty(eng, collect, CondFunc::IsInCombat, {0, 0, 1}, 0) == 1.0);
    // entry:all-in,exit:any-in needs the phase the caller declared
    CHECK(overParty(eng, collect, CondFunc::GetInCell, {1, 0}, 0, fnvcoop::CondPhase::Entry) == 0.0);
    CHECK(overParty(eng, collect, CondFunc::GetInCell, {1, 0}, 0, fnvcoop::CondPhase::Exit) == 1.0);
    // no rule for the quest / the function: the initiator's own value
    CHECK(overParty(eng, none, CondFunc::GetItemCount, {3, 4}, 1) == 4.0);
    CHECK(overParty(eng, plain, CondFunc::GetItemCount, {3, 4}, 0) == 3.0);
    // a one-member party never reaches the reducer (engine result is used)
    CHECK(overParty(eng, collect, CondFunc::GetItemCount, {5}, 0) == 5.0);
}

TEST(every_hooked_condition_function_has_a_rule_in_rules_json) {
    fnvcoop::RulesDb db;
    std::string err;
    CHECK(db.loadFile(NVH_RULES_JSON, &err));
    auto lower = [](std::string s) { for (char& c : s) c = static_cast<char>(std::tolower(static_cast<unsigned char>(c))); return s; };
    for (int i = 0; i < static_cast<int>(CondFunc::Count); ++i) {
        std::string name = lower(condFuncName(static_cast<CondFunc>(i)));
        int uses = 0;
        for (const auto& kv : db.quests())
            if (const fnvcoop::Rule* r = kv.second.find("PartyAwareCond"))
                for (const auto& f : r->funcs) {
                    std::string k = lower(f.first);
                    if (k == name || (name == "getactorvalue" && k == "getav")) ++uses;
                }
        if (uses == 0) std::printf("    no PartyAwareCond rule names %s\n", name.c_str());
        CHECK(uses > 0);   // a hooked evaluator nobody can configure is dead weight
    }
}


// ---------------------------------------------------------------- P2: nine more party-aware condition functions (G23), GetHasNote (G10), IsKiller (G04)
TEST(party_condition_plan_for_the_nine_new_functions) {
    // identity group: the player is the calling reference (GetInWorldspace, IsInInterior) ...
    CHECK(planSwap(CondFunc::GetInWorldspace, 0x14, 0).thisObj && !planSwap(CondFunc::GetInWorldspace, 0x1234, 0).any());
    CHECK(planSwap(CondFunc::IsInInterior, 0x14, 0).thisObj && !planSwap(CondFunc::IsInInterior, 0x1234, 0).any());
    // ... IsKiller's calling reference is the dead actor; only the claimed killer (param1) is the player
    CHECK(planSwap(CondFunc::IsKiller, 0x1234, 0x14).param1 && !planSwap(CondFunc::IsKiller, 0x1234, 0x14).thisObj);
    CHECK(!planSwap(CondFunc::IsKiller, 0x14, 0x1234).any());   // the player as the dead actor is not a kill attribution question
    CHECK(param1IsReference(CondFunc::IsKiller) && !param1IsReference(CondFunc::GetEquipped));
    // state group: player as calling reference
    CHECK(planSwap(CondFunc::HasPerk, 0x14, 0).thisObj && !planSwap(CondFunc::HasPerk, 0x1234, 0).any());
    CHECK(planSwap(CondFunc::GetEquipped, 0x14, 0).thisObj && planSwap(CondFunc::IsSneaking, 0x14, 0).thisObj);
    // evaluators that ignore the operand (read the global player) or carry an NPC: party-aware whatever the operands are
    CHECK(planSwap(CondFunc::GetHasNote, 0, 0).always && planSwap(CondFunc::GetHasNote, 0x1234, 0).any());
    CHECK(planSwap(CondFunc::IsPlayerInRegion, 0, 0).always);
    CHECK(planSwap(CondFunc::GetPlayerTeammate, 0x5555, 0).always && !planSwap(CondFunc::GetPlayerTeammate, 0x5555, 0).thisObj);
    CHECK(memberView(CondFunc::GetDistance) == MemberView::Ghost && memberView(CondFunc::IsKiller) == MemberView::Ghost);
    CHECK(memberView(CondFunc::GetInWorldspace) == MemberView::Ghost && memberView(CondFunc::IsInInterior) == MemberView::Ghost);
    for (CondFunc f : {CondFunc::GetEquipped, CondFunc::HasPerk, CondFunc::IsSneaking, CondFunc::GetHasNote, CondFunc::IsPlayerInRegion})
        CHECK(memberView(f) == MemberView::State);
    CHECK(memberView(CondFunc::GetPlayerTeammate) == MemberView::Owner);
    CHECK(std::string(condFuncName(CondFunc::IsKiller)) == "IsKiller" && std::string(condFuncName(CondFunc::GetHasNote)) == "GetHasNote" &&
          std::string(condFuncName(CondFunc::IsPlayerInRegion)) == "IsPlayerInRegion" && std::string(condFuncName(CondFunc::HasPerk)) == "HasPerk");
    CHECK(static_cast<int>(CondFunc::Count) == 23);   // the 17 of waves 0-1a plus the six of wave 1b (party_condition_plan_for_the_wave1b_functions)
}

namespace {
struct FakeCond {
    std::map<uint32_t, double> ghost;                 // ghost ref -> value the evaluator returns for it
    double local = 0;
    std::map<std::pair<int, uint8_t>, double> state;  // (func, slot) -> value
    int ghostCalls = 0, stateCalls = 0;
    CondOps ops() {
        CondOps o;
        o.evalLocal = [this](double* v) { *v = local; return true; };
        o.evalGhost = [this](uint32_t r, double* v) { ++ghostCalls; auto it = ghost.find(r); if (it == ghost.end()) return false; *v = it->second; return true; };
        o.state = [this](const StateQuery& q, double* v) {
            ++stateCalls;
            auto it = state.find({static_cast<int>(q.f), q.slot});
            if (it == state.end()) return false;
            *v = it->second;
            return true;
        };
        return o;
    }
};
std::vector<PartyMember> trio() { return {{0, kHostPlayerRef}, {1, 0xFF000001}, {2, 0xFF000002}}; }
}  // namespace

TEST(party_member_value_ghost_view_state_view_and_owner_view) {
    StateQuery base;
    double v = -1;
    {   // ghost view: spatial / identity functions are answered by the member's ghost reference
        FakeCond c; c.local = 0; c.ghost[0xFF000001] = 1; c.ghost[0xFF000002] = 0;
        CondOps o = c.ops();
        CHECK(evalMember(CondFunc::IsInInterior, trio()[1], base, o, &v) && v == 1.0 && c.ghostCalls == 1 && c.stateCalls == 0);
        CHECK(evalMember(CondFunc::IsInInterior, trio()[0], base, o, &v) && v == 0.0 && c.ghostCalls == 1);   // the local player never goes through a ghost
        CHECK(evalMember(CondFunc::IsKiller, trio()[1], base, o, &v) && v == 1.0);
        CHECK(!evalMember(CondFunc::GetInWorldspace, {3, 0xFF000009}, base, o, &v));   // ghost not loaded: member skipped
    }
    {   // state view: a ghost has no perks / notes / sneak state, so only the provider answers for remote members
        FakeCond c; c.local = 1; c.ghost[0xFF000001] = 1; c.state[{static_cast<int>(CondFunc::HasPerk), 1}] = 0; c.state[{static_cast<int>(CondFunc::GetHasNote), 2}] = 1;
        CondOps o = c.ops();
        CHECK(evalMember(CondFunc::HasPerk, trio()[0], base, o, &v) && v == 1.0 && c.stateCalls == 0);        // local: the engine's own answer
        CHECK(evalMember(CondFunc::HasPerk, trio()[1], base, o, &v) && v == 0.0 && c.ghostCalls == 0);        // remote: provider says no, the ghost's 1 is ignored
        CHECK(evalMember(CondFunc::GetHasNote, trio()[2], base, o, &v) && v == 1.0);
        CHECK(!evalMember(CondFunc::IsSneaking, trio()[1], base, o, &v));                                    // provider does not know: skipped
        CondOps none = o; none.state = nullptr;
        CHECK(!evalMember(CondFunc::HasPerk, trio()[1], base, none, &v));                                    // no provider at all: skipped, never guessed
        CHECK(evalMember(CondFunc::HasPerk, trio()[0], base, none, &v) && v == 1.0);
    }
    {   // owner view: the engine flag says "somebody's teammate", the provider says whose
        FakeCond c; c.local = 1; c.state[{static_cast<int>(CondFunc::GetPlayerTeammate), 0}] = 0; c.state[{static_cast<int>(CondFunc::GetPlayerTeammate), 1}] = 1;
        CondOps o = c.ops();
        CHECK(evalMember(CondFunc::GetPlayerTeammate, trio()[0], base, o, &v) && v == 0.0);
        CHECK(evalMember(CondFunc::GetPlayerTeammate, trio()[1], base, o, &v) && v == 1.0);
        CHECK(evalMember(CondFunc::GetPlayerTeammate, trio()[2], base, o, &v) && v == 1.0);   // no owner row for this slot: the engine flag stands
        c.local = 0;   // not a teammate at all: nobody owns it, whatever the provider table says
        CHECK(evalMember(CondFunc::GetPlayerTeammate, trio()[1], base, o, &v) && v == 0.0);
        c.local = 1;
        CondOps none = o; none.state = nullptr;
        CHECK(evalMember(CondFunc::GetPlayerTeammate, trio()[1], base, none, &v) && v == 1.0);   // no table: the engine flag (any member)
    }
}

// Guest pack facts: a guest's hand-in runs as a script on the host and must not take the item from the HOST's own pack.
TEST(a_guests_remove_item_skips_the_engine_and_is_recorded_for_the_rules) {
    GuestItems g;
    CHECK(!g.active() && g.slot() == 0);
    g.push(2);
    CHECK(g.active() && g.slot() == 2);
    g.push(0);                       // a nested host window: the host's own pack again
    CHECK(!g.active());
    g.pop();
    CHECK(g.active() && g.slot() == 2);
    g.pop();
    g.pop();                         // too many pops are harmless
    CHECK(!g.active());
    // the plan: the guest's removal on `player` is recorded and the engine call is skipped
    RemovePlan p = planPlayerRemoveItem(true, true, true, true);
    CHECK(!p.runEngine && p.record);
    p = planPlayerRemoveItem(true, false, true, true);               // the host's own hand-in: the engine removes it from the host
    CHECK(p.runEngine && p.record);
    p = planPlayerRemoveItem(false, true, true, true);               // some other reference (a container): not the player
    CHECK(p.runEngine && p.record);
    p = planPlayerRemoveItem(true, true, false, true);               // no capture scope: nothing is recorded, so the old behaviour (a guest scope never opens without one)
    CHECK(p.runEngine && !p.record);
    p = planPlayerRemoveItem(true, true, true, false);               // the arguments did not parse: the engine decides
    CHECK(p.runEngine && !p.record);
}

// Guest pack facts: a puppet carries nothing of the guest's pack, so GetItemCount and GetActorValue (InventoryWeight, the Jed gate) ask the state provider first
// (what the guest reported) and the ghost only when it does not know.
TEST(party_member_value_inventory_view_asks_the_reported_pack_first_and_the_puppet_when_unknown) {
    StateQuery base;
    double v = -1;
    CHECK(memberView(CondFunc::GetItemCount) == MemberView::StateOrGhost && memberView(CondFunc::GetActorValue) == MemberView::StateOrGhost);
    FakeCond c; c.local = 2; c.ghost[0xFF000001] = 0; c.ghost[0xFF000002] = 5;
    c.state[{static_cast<int>(CondFunc::GetItemCount), 1}] = 4;
    c.state[{static_cast<int>(CondFunc::GetActorValue), 1}] = 80.5;
    CondOps o = c.ops();
    CHECK(evalMember(CondFunc::GetItemCount, trio()[0], base, o, &v) && v == 2.0 && c.stateCalls == 0 && c.ghostCalls == 0);   // the local player: the engine's own answer
    CHECK(evalMember(CondFunc::GetItemCount, trio()[1], base, o, &v) && v == 4.0 && c.ghostCalls == 0);                          // reported: not the puppet's empty pack
    CHECK(evalMember(CondFunc::GetItemCount, trio()[2], base, o, &v) && v == 5.0 && c.ghostCalls == 1);                          // no report from this one: its puppet answers as before
    CHECK(evalMember(CondFunc::GetActorValue, trio()[1], base, o, &v) && v == 80.5);
    CondOps none = o; none.state = nullptr;
    CHECK(evalMember(CondFunc::GetItemCount, trio()[1], base, none, &v) && v == 0.0);                                             // no provider: the puppet, never a skip
    CHECK(!evalMember(CondFunc::GetItemCount, {3, 0xFF000009}, base, o, &v));                                                     // neither: skipped
    // folded into the party reduction the heaviest pack decides (host 2, guest 80.5, a third whose puppet says 5)
    std::vector<CondValue> vals;
    for (const PartyMember& m : trio()) { double x = 0; if (evalMember(CondFunc::GetActorValue, m, base, o, &x)) vals.push_back({m.slot, x}); }
    CHECK(vals.size() == 3 && vals[0].value == 2.0 && vals[1].value == 80.5 && vals[2].value == 5.0);
}

namespace {
const char* kNewFuncRules = R"({"schema":1,"quests":{
 "Notes":{"tier":"T3","questline":"x","curated":false,"rules":[
   {"kind":"PartyAwareCond","params":{"funcs":{"GetHasNote":"any-party-member","HasPerk":"initiator","IsKiller":"any-party-member","GetPlayerTeammate":"any-party-member"}},"origin":"derived"}]}
}})";
bool newFuncResolve(const std::string&, fnvcoop::FormRef& out) { out = fnvcoop::FormRef("falloutnv.esm", 0x200u); return true; }
double runNew(fnvcoop::RulesEngine& eng, CondFunc f, FakeCond& c, uint8_t initiator) {
    ConditionReducer red = makeRulesReducer(eng);
    fnvcoop::FormRef q;
    newFuncResolve("Notes", q);
    StateQuery base;
    CondOps o = c.ops();
    double out = -12345;
    bool done = evaluateOverParty(
        f, trio(), [&](const PartyMember& m, double* v) { return evalMember(f, m, base, o, v); },
        [&](const std::string& fn, const std::vector<CondValue>& vals, uint8_t init) { return red(q, fn, vals, init, fnvcoop::CondPhase::Entry); }, initiator, &out);
    return done ? out : c.local;
}
}  // namespace

TEST(party_conditions_over_the_new_functions_follow_their_rules) {
    fnvcoop::InMemoryNetwork net(1);
    fnvcoop::HostSession host(*net.endpoint(0));
    fnvcoop::RulesDb db;
    std::string err;
    CHECK(db.load(kNewFuncRules, &err));
    fnvcoop::RulesEngine eng(host, db, newFuncResolve);
    const int note = static_cast<int>(CondFunc::GetHasNote), perk = static_cast<int>(CondFunc::HasPerk), mate = static_cast<int>(CondFunc::GetPlayerTeammate);
    {   // GetHasNote, any-party-member: slot 2 looted the Jessup note, the host's own journal does not have it yet
        FakeCond c; c.local = 0; c.state[{note, 1}] = 0; c.state[{note, 2}] = 1;
        CHECK(runNew(eng, CondFunc::GetHasNote, c, 0) == 1.0);
        c.state[{note, 2}] = 0;
        CHECK(runNew(eng, CondFunc::GetHasNote, c, 0) == 0.0);
    }
    {   // HasPerk, initiator: the guest who talks decides, not the host
        FakeCond c; c.local = 1; c.state[{perk, 1}] = 0; c.state[{perk, 2}] = 1;
        CHECK(runNew(eng, CondFunc::HasPerk, c, 0) == 1.0);
        CHECK(runNew(eng, CondFunc::HasPerk, c, 1) == 0.0);
        CHECK(runNew(eng, CondFunc::HasPerk, c, 2) == 1.0);
    }
    {   // IsKiller, any-party-member: a guest's ghost is the recorded killer; then nobody is the killer
        FakeCond c; c.local = 0; c.ghost[0xFF000001] = 0; c.ghost[0xFF000002] = 1;
        CHECK(runNew(eng, CondFunc::IsKiller, c, 0) == 1.0);
        c.ghost[0xFF000002] = 0;
        CHECK(runNew(eng, CondFunc::IsKiller, c, 0) == 0.0);
    }
    {   // GetPlayerTeammate, any-party-member: a companion owned by the second guest counts for the party
        FakeCond c; c.local = 1; c.state[{mate, 0}] = 0; c.state[{mate, 1}] = 0; c.state[{mate, 2}] = 1;
        CHECK(runNew(eng, CondFunc::GetPlayerTeammate, c, 0) == 1.0);
        c.state[{mate, 2}] = 0;
        CHECK(runNew(eng, CondFunc::GetPlayerTeammate, c, 0) == 0.0);   // flagged teammate but owned by nobody present
    }
}

// W3a item 4: a ref variable keeps its form id in the low dword of the slot, not as the number (a mutant stores the number: the ref reads back as garbage).
TEST(a_ref_variable_slot_holds_the_form_id_in_its_low_dword) {
    const double slot = refVarSlotFromNumber(16786245.0);   // 0x01002345
    uint64_t bits;
    std::memcpy(&bits, &slot, sizeof bits);
    CHECK(bits == 0x01002345ull);
    CHECK(refVarNumberFromSlot(slot) == 16786245.0);
    CHECK(refVarNumberFromSlot(refVarSlotFromNumber(0xFF000123u)) == 4278190371.0);   // a created reference (top byte 0xFF) survives
    CHECK(refVarNumberFromSlot(refVarSlotFromNumber(0)) == 0.0);
    double engine;                                                  // what the engine stores for a ref local holding 0x00104C1C
    const uint64_t eb = 0x00104C1Cull;
    std::memcpy(&engine, &eb, sizeof engine);
    CHECK(refVarNumberFromSlot(engine) == 1068060.0);
}

// W3a item 5: the host's own SetPlayerControls call is skipped only for a captured lock of a guest that the router calls guest-only.
TEST(the_host_skips_its_controls_call_only_for_a_guest_only_lock) {
    int asked = 0;
    auto router = [&](uint8_t slot, bool) { ++asked; return slot == 2; };
    CHECK(skipHostControlsCall(true, 2, true, router));      // guest 2, scope=initiator
    CHECK(!skipHostControlsCall(true, 1, true, router));     // the router says this one concerns everybody
    CHECK(!skipHostControlsCall(true, 0, true, router));     // the host is acting: its own lock
    CHECK(!skipHostControlsCall(false, 2, true, router));    // not captured (no rules apply): the engine call runs
    CHECK(!skipHostControlsCall(true, 2, true, nullptr));    // no router: unchanged behaviour
    CHECK(asked == 2);
}

TEST(the_capture_hub_names_the_cause_of_its_outermost_scope) {
    nvhooks::CaptureHub hub;
    hub.setEnabled(true);
    fnvcoop::Cause out;
    CHECK(!hub.currentCause(&out));
    fnvcoop::Cause outer, inner;
    outer.kind = fnvcoop::CauseKind::QuestResult; outer.source = fnvcoop::FormRef("falloutnv.esm", 0x111); outer.aux = 30;
    inner.kind = fnvcoop::CauseKind::Script; inner.source = fnvcoop::FormRef("falloutnv.esm", 0x222);
    {
        nvhooks::ScopedCapture a(hub, outer);
        nvhooks::ScopedCapture b(hub, inner);
        CHECK(hub.currentCause(&out) && out.kind == fnvcoop::CauseKind::QuestResult && out.source.localId == 0x111 && out.aux == 30);
    }
    CHECK(!hub.currentCause(&out));
}

TEST(kill_attribution_writes_the_engine_killer_slot) {
    alignas(8) unsigned char actor[0x120] = {};
    unsigned char killerObj[16] = {};
    CHECK(storeKiller(actor, kFormTypeACHR, killerObj));
    void* back = nullptr;
    std::memcpy(&back, actor + 0xC0, sizeof back);   // the exact slot Actor::Kill writes and IsKiller reads
    CHECK(back == killerObj && loadKiller(actor, kFormTypeACHR) == killerObj);
    CHECK(storeKiller(actor, kFormTypeACRE, nullptr) && loadKiller(actor, kFormTypeACRE) == nullptr);   // clearing works
    CHECK(!storeKiller(actor, 0x31 /* a NOTE form */, killerObj) && loadKiller(actor, 0x31) == nullptr);   // never on a non-actor
    CHECK(!storeKiller(nullptr, kFormTypeACHR, killerObj));
    CHECK(killByCmd(0xFF001234u) == "KillActor FF001234");   // the console line that lets the engine store the killer in its own order
}


// ---------------------------------------------------------------- P2: G47 the GuestLocalRun rule IS the allow-list
namespace {
std::string readWholeFile(const std::string& path) {
    std::string text;
    FILE* f = fopen(path.c_str(), "rb");
    if (!f) return text;
    char buf[8192];
    size_t n;
    while ((n = fread(buf, 1, sizeof buf, f)) > 0) text.append(buf, n);
    fclose(f);
    return text;
}
}  // namespace

TEST(guest_local_run_list_is_read_from_the_rules_json_rule) {
    const char* rules = R"({"schema":1,"quests":{
      "AchievementQuest":{"tier":"T3","questline":"x","curated":false,"rules":[
        {"kind":"GuestLocalRun","params":{"scripts":["FalloutNV.esm:07691E"]},"origin":"questsweep"},
        {"kind":"RewardAll","params":{},"origin":"derived"}]},
      "NVDLC02MQ00":{"tier":"T3","questline":"x","curated":false,"rules":[
        {"kind":"GuestLocalRun","params":{"scripts":["HonestHearts.esm:008A32","nocolon","X.esm:zz"]},"origin":"questsweep"}]},
      "Other":{"tier":"T3","questline":"x","curated":false,"rules":[
        {"kind":"PartyWarp","params":{"scripts":["FalloutNV.esm:000999"]},"origin":"derived"}]},
      "NoRules":{"tier":"T3","questline":"x","curated":false}
    }})";
    GuestLocalRunList l;
    std::string err;
    CHECK(parseGuestLocalRunRules(rules, &l, &err));
    CHECK(l.scripts.size() == 2 && l.skipped == 2);   // only GuestLocalRun rules count; two malformed script entries are skipped and counted
    auto has = [&](const char* plugin, uint32_t id, const char* quest) {
        for (size_t i = 0; i < l.scripts.size(); ++i)
            if (l.scripts[i].plugin == plugin && l.scripts[i].localId == id && l.quests[i] == quest) return true;
        return false;
    };
    CHECK(has("falloutnv.esm", 0x7691E, "AchievementQuest") && has("honesthearts.esm", 0x8A32, "NVDLC02MQ00"));
    CHECK(!parseGuestLocalRunRules("{\"nope\":1}", &l, &err) && !err.empty());
    CHECK(!parseGuestLocalRunRules("not json", &l, &err));
    GuestLocalRunList none;
    CHECK(parseGuestLocalRunRules(R"({"schema":1,"quests":{}})", &none, &err) && none.scripts.empty());
}

TEST(editing_the_guest_local_run_rule_changes_what_a_guest_runs) {
    // the whole chain: rules text -> list -> SuppressPolicy gate. No second list is consulted anywhere.
    auto gate = [](const std::string& rulesText, uint32_t scriptLocalId) {
        GuestLocalRunList l;
        std::string err;
        if (!parseGuestLocalRunRules(rulesText, &l, &err)) return std::string("parse-error");
        SuppressPolicy p;
        p.setGuest(true);
        std::unordered_set<uint32_t> ids;
        for (const auto& r : l.scripts) ids.insert(r.localId);   // the runtime resolver maps FormRef -> engine id; the local id stands in for it here
        p.setGuestLocalRun(ids);
        return std::string(p.skipScript(kScriptTypeQuest, scriptLocalId) ? "skipped" : "runs");
    };
    const std::string withRule = R"({"schema":1,"quests":{"Q":{"rules":[{"kind":"GuestLocalRun","params":{"scripts":["FalloutNV.esm:000500","FalloutNV.esm:000501"]}}]}}})";
    const std::string edited = R"({"schema":1,"quests":{"Q":{"rules":[{"kind":"GuestLocalRun","params":{"scripts":["FalloutNV.esm:000501"]}}]}}})";
    const std::string gone = R"({"schema":1,"quests":{"Q":{"rules":[]}}})";
    CHECK(gate(withRule, 0x500) == "runs" && gate(withRule, 0x501) == "runs" && gate(withRule, 0x502) == "skipped");
    CHECK(gate(edited, 0x500) == "skipped" && gate(edited, 0x501) == "runs");   // dropping a script from the rule gates it again
    CHECK(gate(gone, 0x501) == "skipped");
}

TEST(shipped_rules_json_guest_local_run_rules_match_the_legacy_list_file) {
    // The legacy data file is only a fallback for an install without nvcoop_rules.json; until it is deleted it must say what the rules say.
    GuestLocalRunList fromRules, fromFile;
    std::string err;
    const std::string rulesText = readWholeFile(NVH_RULES_JSON);
    CHECK(!rulesText.empty() && parseGuestLocalRunRules(rulesText, &fromRules, &err));
    CHECK(fromRules.skipped == 0 && fromRules.scripts.size() >= 30);
    auto hasQuest = [&](const char* q) { for (auto& x : fromRules.quests) if (x == q) return true; return false; };
    auto hasScript = [&](const char* plugin, uint32_t id) { for (auto& r : fromRules.scripts) if (r.plugin == plugin && r.localId == id) return true; return false; };
    for (const char* q : {"VDLCPackQuest", "AchievementQuest", "AutoDocTimerQuest", "ImplantTimerQuest"}) CHECK(hasQuest(q));
    // the per-DLC setup scripts and the rumble script ride on VDLCPackQuest's rule in rules.json
    CHECK(hasScript("deadmoney.esm", 0x62B1) && hasScript("honesthearts.esm", 0x8A32) && hasScript("oldworldblues.esm", 0x2FD0) &&
          hasScript("lonesomeroad.esm", 0x3604) && hasScript("falloutnv.esm", 0x70ECA));
    CHECK(parseGuestLocalRun(readWholeFile(std::string(NVH_DATA_DIR) + "/guest_local_run.json"), &fromFile, &err));
    auto key = [](const GuestLocalRunList& l) {
        std::set<std::string> s;
        for (const auto& r : l.scripts) { char b[24]; std::snprintf(b, sizeof b, ":%X", r.localId); s.insert(r.plugin + b); }
        return s;
    };
    CHECK(key(fromRules) == key(fromFile));
}


// ---------------------------------------------------------------- P2: G42 / G35 which player does a script operand stand for, G12 faction removal
TEST(player_operands_resolve_to_exactly_one_party_slot) {
    ActingContext c;
    c.registerSlot(0xFF000001, 1);
    c.registerSlot(0xFF000002, 2);
    // nobody acting: `player` is the host's own player, the engine call may run
    auto t = c.playerTargetOf(0x14);
    CHECK(t.player && t.slot == 0 && t.onHost);
    // a guest's ghost is that guest and never the host's player
    t = c.playerTargetOf(0xFF000001);
    CHECK(t.player && t.slot == 1 && !t.onHost);
    t = c.playerTargetOf(0xFF000002);
    CHECK(t.player && t.slot == 2 && !t.onHost);
    // an NPC, a container, a ghost nobody registered: not a player
    CHECK(!c.playerTargetOf(0x104C79).player && !c.playerTargetOf(0xFF0000AA).player && !c.playerTargetOf(0).player);
    // a script run for guest 2: its `player` operand is guest 2; the host's own player must not die or be stripped for it
    c.push(0xFF000002);
    t = c.playerTargetOf(0x14);
    CHECK(t.player && t.slot == 2 && !t.onHost);
    c.push(0xFF000001);
    CHECK(c.playerTargetOf(0x14).slot == 1);
    c.pop();
    CHECK(c.playerTargetOf(0x14).slot == 2);
    c.pop();
    t = c.playerTargetOf(0x14);
    CHECK(t.slot == 0 && t.onHost);
    // an acting ghost nobody registered falls back to slot 0 (the host decides), like actingSlot()
    c.push(0xFF0000AA);
    CHECK(c.playerTargetOf(0x14).slot == 0 && c.playerTargetOf(0x14).onHost);
}

TEST(remove_from_all_factions_names_every_faction_the_actor_was_in) {
    // base record: A rank 0, B rank 3, C rank 1; the script had added D (rank 2), removed B and re-ranked A before the clear
    const std::vector<FactionRankEntry> base = {{0xA, 0}, {0xB, 3}, {0xC, 1}};
    const std::vector<FactionRankEntry> run = {{0xB, -1}, {0xD, 2}, {0xA, 5}};
    std::vector<uint32_t> left = factionsLeftBy(base, run);
    CHECK((left == std::vector<uint32_t>{0xA, 0xC, 0xD}));   // B was already out; D only exists in the runtime list; A once, with its runtime rank
    CHECK(factionsLeftBy(base, {}) == (std::vector<uint32_t>{0xA, 0xB, 0xC}));
    CHECK(factionsLeftBy({}, {{0x7, 0}}) == (std::vector<uint32_t>{0x7}));
    CHECK(factionsLeftBy({}, {}).empty());
    CHECK(factionsLeftBy({{0, 4}}, {}).empty());   // a null faction pointer is never announced
    // a runtime entry with a negative rank removes a base faction, a later positive one brings it back
    CHECK(factionsLeftBy({{0xA, 0}}, {{0xA, -1}, {0xA, 2}}) == (std::vector<uint32_t>{0xA}));
    CHECK(factionsLeftBy({{0xA, 0}}, {{0xA, 2}, {0xA, -1}}).empty());
}


// ---------------------------------------------------------------- P2: P4 handover 1 and 2 (host-side puppet events, slot table reset)
TEST(host_wraps_only_combat_events_of_a_registered_puppet) {
    ActingContext c;
    c.registerSlot(0xFF000001, 1);
    const uint32_t kStartCombat = 0x8000;
    CHECK(c.hostPuppetEvent(0xFF000001, kStartCombat));                     // a host NPC starts combat with guest 1's puppet: run it as that guest
    CHECK(!c.hostPuppetEvent(0xFF000009, kStartCombat));                    // an unregistered ref is not a puppet
    CHECK(!c.hostPuppetEvent(0x14, kStartCombat));                          // the host's own player is already `player`
    CHECK(!c.hostPuppetEvent(0xFF000001, 0x10000000));                      // OnTrigger: the host raises it for the puppet through intents already
    CHECK(!c.hostPuppetEvent(0xFF000001, 0x20000000) && !c.hostPuppetEvent(0xFF000001, 0x40000000));   // enter / leave likewise
    CHECK(!c.hostPuppetEvent(0xFF000001, 0x1) && !c.hostPuppetEvent(0xFF000001, 0x10000));              // pickups and container opens as well
}

TEST(clearing_the_slot_table_lets_a_ninth_puppet_keep_its_slot) {
    ActingContext c;
    for (uint32_t i = 0; i < 8; ++i) c.registerSlot(0xFF000100 + i, static_cast<uint8_t>(1 + (i % 3)));
    c.registerSlot(0xFF000200, 2);                         // table full: the ninth ref is silently dropped
    c.push(0xFF000200);
    CHECK(c.slot() == 0 && !c.playerTargetOf(0xFF000200).player);
    c.pop();
    c.clearSlots();
    CHECK(!c.playerTargetOf(0xFF000100).player);           // forgotten
    c.registerSlot(0xFF000200, 2);
    c.push(0xFF000200);
    CHECK(c.slot() == 2 && c.playerTargetOf(0xFF000200).slot == 2);
    c.pop();
}


// ---------------------------------------------------------------- P2: P5 handover 2 (lock pre-image of a scripted Lock / UnLock)
TEST(scripted_lock_and_unlock_carry_the_lock_pre_image) {
    fnvcoop::ObjectState lock;   // Lock 50 on a door that was unlocked
    lock.mask = fnvcoop::kObjLock; lock.locked = true; lock.level = 50;
    applyLockPreImage(&lock, false, 0);
    CHECK(lock.hasBefore && !lock.beforeLocked && lock.beforeLevel == 0);
    fnvcoop::ObjectState unlock;   // UnLock on a door that was locked at level 75
    unlock.ref = fnvcoop::FormRef("falloutnv.esm", 0x1234); unlock.mask = fnvcoop::kObjLock; unlock.locked = false;
    applyLockPreImage(&unlock, true, 75);
    CHECK(unlock.hasBefore && unlock.beforeLocked && unlock.beforeLevel == 75);
    fnvcoop::ObjectState open;   // SetOpenState does not touch the lock bit: no pre-image
    open.mask = fnvcoop::kObjOpen; open.open = true;
    applyLockPreImage(&open, true, 25);
    CHECK(!open.hasBefore);
    // the pre-image survives the wire (JSON round trip used by the flight recorder)
    fnvcoop::Effect e = unlock;
    std::string err;
    fnvcoop::Effect back;
    CHECK(effectFromJson(effectToJson(e), &back, &err) && std::get<fnvcoop::ObjectState>(back) == unlock);
}


// ---------------------------------------------------------------- P2: P3 handover R1 (the initiator's player facts answer a guest's result script)
TEST(player_facts_answer_the_four_reads_by_kind_and_thing) {
    const std::vector<PlayerFact> facts = {
        {kFactActorValue, fnvcoop::FormRef("", 5), 70},                               // Speech-like code 5 = 70
        {kFactItemCount, fnvcoop::FormRef("falloutnv.esm", 0xF), 1250},               // caps
        {kFactItemCount, fnvcoop::FormRef("deadmoney.esm", 0xF), 3},                  // same local id, other plugin
        {kFactHasPerk, fnvcoop::FormRef("falloutnv.esm", 0x77), 1},
        {kFactEquipped, fnvcoop::FormRef("falloutnv.esm", 0x99), 0}};
    int32_t v = -1;
    CHECK(lookupFact(facts, kFactActorValue, fnvcoop::FormRef("anything.esm", 5), &v) && v == 70);   // an AV is matched by its code alone
    CHECK(!lookupFact(facts, kFactActorValue, fnvcoop::FormRef("", 6), &v));
    CHECK(lookupFact(facts, kFactItemCount, fnvcoop::FormRef("falloutnv.esm", 0xF), &v) && v == 1250);
    CHECK(lookupFact(facts, kFactItemCount, fnvcoop::FormRef("deadmoney.esm", 0xF), &v) && v == 3);   // the plugin matters for forms
    CHECK(!lookupFact(facts, kFactItemCount, fnvcoop::FormRef("honesthearts.esm", 0xF), &v));
    CHECK(lookupFact(facts, kFactHasPerk, fnvcoop::FormRef("falloutnv.esm", 0x77), &v) && v == 1);
    CHECK(lookupFact(facts, kFactEquipped, fnvcoop::FormRef("falloutnv.esm", 0x99), &v) && v == 0);
    CHECK(!lookupFact(facts, kFactEquipped, fnvcoop::FormRef("falloutnv.esm", 0x77), &v));   // the kind matters
    CHECK(!lookupFact({}, kFactItemCount, fnvcoop::FormRef("falloutnv.esm", 0xF), &v));
    // the condition evaluators of the same names map to the same kinds
    CHECK(factKindOf(CondFunc::GetActorValue) == kFactActorValue && factKindOf(CondFunc::GetItemCount) == kFactItemCount &&
          factKindOf(CondFunc::HasPerk) == kFactHasPerk && factKindOf(CondFunc::GetEquipped) == kFactEquipped);
    CHECK(factKindOf(CondFunc::GetDistance) == 0 && factKindOf(CondFunc::IsKiller) == 0);
}

TEST(muted_hub_ignores_quest_var_of_guest_local_quest_owner) {
    // Local-only effects: while a GuestLocalRun script runs nothing is captured, and its quest-var owner is flagged unshared.
    SuppressPolicy p;
    p.setGuestLocalRun({0x500});
    CaptureHub h;
    h.setEnabled(true);
    std::vector<EffectTxn> out;
    h.setSink([&](EffectTxn&& t) { out.push_back(std::move(t)); });
    FormRef q("falloutnv.esm", 0x104EAE);
    h.pushMute();
    h.begin(qcause(1, 1));
    CHECK(!h.recordScriptVar({q, 3, 1.0}) && !h.record(SetFactionRank{q, q, 1}) && !h.record(AddItem{q, 1, Recipient::all()}));
    h.end();
    h.popMute();
    CHECK(out.empty() && h.stats().droppedLocal >= 3);
    CHECK(!p.sharesQuestVars(0x500) && p.sharesQuestVars(0x501));
}

// story-fix2: the DLC intro quests (NVDLC02/03/04 MQ00 stage 10, the OWB radio broadcast) were started by the client's own GuestLocalRun setup script while the host had not started them
// yet; the copies disagreed and the digest "repaired" one of them (S06's clean-digest rule). A quest start / stage of those scripts is the host's, per-machine effects stay per machine.
TEST(guest_dlc_setup_script_runs_but_its_quest_calls_are_the_hosts) {
    SuppressPolicy p;
    p.setGuest(true);
    p.setGuestLocalRun({0x500, 0x600});
    p.setHostQuestState({0x500, 0x999});                 // 0x999 is not allow-listed: it can not be host-quest-state
    CHECK(p.isHostQuestState(0x500) && !p.isHostQuestState(0x600) && !p.isHostQuestState(0x999) && !p.isHostQuestState(0));
    p.enterLocalScript(0x500);                           // the DLC setup script
    uint32_t who = 0;
    CHECK(p.inHostQuestStateScript(&who) && who == 0x500);
    CHECK(!p.blockMutation());                           // form lists, the DLC faction, messages: per machine
    CHECK(p.blockQuestState());                          // StartQuest / SetStage / objective: the host's
    CHECK(p.stats().mutations == 1);
    p.enterScript();                                     // a nested ordinary script (a stage result it caused) is blocked as on any guest
    CHECK(p.blockQuestState());
    p.leaveScript();
    p.beginApply();                                      // the host's txn arriving is applied
    CHECK(!p.blockQuestState());
    p.endApply();
    p.leaveLocalScript(0x500);
    CHECK(!p.inHostQuestStateScript() && p.localScriptDepth() == 0);
    p.enterLocalScript(0x600);                           // rumble, radio, achievements: their quest calls stay per machine
    CHECK(!p.blockQuestState() && !p.inHostQuestStateScript());
    p.enterLocalScript(0x500);                           // nested: the innermost one decides
    CHECK(p.blockQuestState());
    p.leaveLocalScript(0x500);
    CHECK(!p.blockQuestState());
    p.leaveLocalScript(0x600);
    p.enterScript();
    CHECK(p.blockQuestState());                          // an ordinary script on a guest is blocked as before
    p.leaveScript();
    SuppressPolicy h;                                    // the host never blocks
    h.setGuestLocalRun({0x500});
    h.setHostQuestState({0x500});
    h.enterLocalScript(0x500);
    CHECK(!h.blockQuestState() && !h.blockMutation());
    h.leaveLocalScript(0x500);
}

TEST(host_dlc_setup_script_quest_calls_are_captured_though_its_effects_are_muted) {
    CaptureHub h;
    h.setEnabled(true);
    std::vector<EffectTxn> out;
    h.setSink([&](EffectTxn&& t) { out.push_back(std::move(t)); });
    h.setWeakCauseResolver([](uint32_t id) {
        Cause c;
        c.kind = CauseKind::Script;
        c.source = FormRef("oldworldblues.esm", id);
        return c;
    });
    FormRef quest("oldworldblues.esm", 0x2FCA), radio("oldworldblues.esm", 0x13653), faction("oldworldblues.esm", 0x16154);
    h.pushMute();                                        // the host runs the setup script muted (GuestLocalRun)
    CHECK(!h.record(SetFactionRank{faction, quest, 1}));   // its per-machine effects stay out
    const int saved = h.liftMute();                      // QuestStateLift: StartQuest of the radio broadcast
    CHECK(h.beginWeak(0x2FD0));
    CHECK(h.record(StartQuest{radio}));
    h.endWeak();
    h.restoreMute(saved);
    CHECK(h.muted() && !h.record(SetFactionRank{faction, quest, 2}));   // muted again for the rest of the script
    const int saved2 = h.liftMute();                     // SetStage: an explicit scope, as h_SetStage opens it
    CHECK(h.beginWeak(0x2FD0));
    h.begin(qcause(0x2FCA, 10));
    CHECK(h.record(stage(0x2FCA, 10)));
    h.end();
    h.endWeak();
    h.restoreMute(saved2);
    h.popMute();
    CHECK(out.size() == 2);
    CHECK(out.size() == 2 && out[0].cause.kind == CauseKind::Script && std::holds_alternative<StartQuest>(out[0].effects[0]));
    CHECK(out.size() == 2 && out[1].cause.kind == CauseKind::QuestResult && std::holds_alternative<SetStage>(out[1].effects[0]));
    CHECK(h.enabled() && h.depth() == 0);
}

TEST(host_quest_state_scripts_are_all_on_the_guest_local_run_allow_list) {
    GuestLocalRunList fromRules;
    std::string err;
    CHECK(parseGuestLocalRunRules(readWholeFile(NVH_RULES_JSON), &fromRules, &err));
    CHECK(hostQuestStateScripts().size() == 4);
    for (const fnvcoop::FormRef& r : hostQuestStateScripts()) {
        bool listed = false;
        for (const auto& x : fromRules.scripts) listed |= (x == r);
        if (!listed) std::printf("    %s:%X is host-quest-state but not on the allow-list\n", r.plugin.c_str(), r.localId);
        CHECK(listed);   // a host-quest-state script that does not run on a guest at all would be dead weight (and its quest never starts there)
    }
}


// ---------------------------------------------------------------- wave 1b (P1 requests to P2): GetLevel, the other condition functions, object-script quests, player.kill
namespace {
uint32_t w1bHash(const std::string& s) {
    uint32_t h = 2166136261u;
    for (char c : s) { h ^= static_cast<uint8_t>(c); h *= 16777619u; }
    return h;
}
bool w1bResolve(const std::string& edid, fnvcoop::FormRef& out) { out = fnvcoop::FormRef("falloutnv.esm", w1bHash(edid)); return true; }
fnvcoop::FormRef w1bQuest(const char* edid) { fnvcoop::FormRef f; w1bResolve(edid, f); return f; }
}  // namespace

TEST(party_condition_plan_for_the_wave1b_functions) {
    CHECK(static_cast<int>(CondFunc::Count) == 23);
    // GetLevel / GetIsSex / GetSitting: the player is the calling reference, but a ghost is a puppet NPC with no level, sex or seat of the guest: the provider answers
    for (CondFunc f : {CondFunc::GetLevel, CondFunc::GetIsSex, CondFunc::GetSitting}) {
        CHECK(planSwap(f, 0x14, 0).thisObj && !planSwap(f, 0x1234, 0).any());
        CHECK(memberView(f) == MemberView::State);
    }
    // GetPCIsSex / IsPlayerActionActive read the GLOBAL player: party-aware whatever the operands are
    for (CondFunc f : {CondFunc::GetPCIsSex, CondFunc::IsPlayerActionActive}) {
        CHECK(planSwap(f, 0, 0).always && planSwap(f, 0x1234, 0).any());
        CHECK(memberView(f) == MemberView::State);
    }
    // GetShouldAttack: thisObj is the NPC, param1 the target; only a player TARGET is swapped for the member's ghost, and a ghost answers (the NPC's own AI decides)
    CHECK(param1IsReference(CondFunc::GetShouldAttack) && planSwap(CondFunc::GetShouldAttack, 0x1234, 0x14).param1 && !planSwap(CondFunc::GetShouldAttack, 0x1234, 0x14).thisObj);
    CHECK(!planSwap(CondFunc::GetShouldAttack, 0x1234, 0x5555).any());
    CHECK(memberView(CondFunc::GetShouldAttack) == MemberView::Ghost);
    const char* names[] = {"GetLevel", "GetIsSex", "GetPCIsSex", "GetSitting", "GetShouldAttack", "IsPlayerActionActive"};
    for (int i = 0; i < 6; ++i) CHECK(std::string(condFuncName(static_cast<CondFunc>(17 + i))) == names[i]);
    // the decision for every function the P1 request names
    for (const char* h : {"GetLevel", "GetShouldAttack", "GetSitting", "GetIsSex", "GetPCIsSex", "IsPlayerActionActive"}) CHECK(condCoverage(h) == CondCoverage::Hooked);
    for (const char* h : {"GetIsReference", "GetMapMarkerVisible", "GetTalkedToPC"}) CHECK(condCoverage(h) == CondCoverage::NoPlayerOperand);
    CHECK(condCoverage("GetActionRef") == CondCoverage::ActingPlayerRuns && condCoverage("IsActionRef") == CondCoverage::ActingPlayerRuns);
    CHECK(condCoverage("getav") == CondCoverage::Hooked && condCoverage("GETLEVEL") == CondCoverage::Hooked);   // rules.json spelling and case
    CHECK(condCoverage("NoSuchFunction") == CondCoverage::Unknown);
}

TEST(every_function_a_partyawarecond_rule_names_has_a_decision) {
    fnvcoop::RulesDb db;
    std::string err;
    CHECK(db.loadFile(NVH_RULES_JSON, &err));
    int seen = 0;
    for (const auto& kv : db.quests())
        if (const fnvcoop::Rule* r = kv.second.find("PartyAwareCond"))
            for (const auto& f : r->funcs) {
                ++seen;
                if (condCoverage(f.first) == CondCoverage::Unknown) std::printf("    rules.json names %s (%s) and nothing decides it\n", f.first.c_str(), kv.first.c_str());
                CHECK(condCoverage(f.first) != CondCoverage::Unknown);
            }
    CHECK(seen > 300);   // the whole rule set was walked, not an empty file
}

TEST(party_level_reduces_to_the_host_or_to_the_strongest_member) {
    fnvcoop::InMemoryNetwork net(1);
    fnvcoop::HostSession host(*net.endpoint(0));
    fnvcoop::RulesDb db;
    std::string err;
    CHECK(db.loadFile(NVH_RULES_JSON, &err));
    fnvcoop::RulesEngine eng(host, db, w1bResolve);
    ConditionReducer red = makeRulesReducer(eng);
    const int lvl = static_cast<int>(CondFunc::GetLevel);
    FakeCond c; c.local = 12; c.state[{lvl, 1}] = 30; c.state[{lvl, 2}] = 5;
    CondOps o = c.ops();
    auto run = [&](const char* quest, uint8_t initiator) {
        double out = -1;
        StateQuery base;
        const bool done = evaluateOverParty(
            CondFunc::GetLevel, trio(), [&](const PartyMember& m, double* v) { return evalMember(CondFunc::GetLevel, m, base, o, v); },
            [&](const std::string& fn, const std::vector<CondValue>& vals, uint8_t init) { return red(w1bQuest(quest), fn, vals, init, fnvcoop::CondPhase::Entry); }, initiator, &out);
        return done ? out : -2.0;
    };
    CHECK(run("NVDLC01MQ01", 2) == 12.0 && run("NVDLC03MQ01", 1) == 12.0);   // XP tiers: the host's level, whoever talks
    CHECK(run("NVDLC04MQ03a", 0) == 30.0 && run("NVDLC04WeakPlayerScaling", 2) == 30.0);   // the Lonesome Road scaling follows the strongest member
    CHECK(c.ghostCalls == 0 && c.stateCalls > 0);   // never through a ghost
    CondOps none = o; none.state = nullptr;   // no provider: the guests' levels are unknown, the engine's own answer stands (a single value never reduces)
    double out = -1;
    StateQuery base;
    CHECK(!evaluateOverParty(CondFunc::GetLevel, trio(), [&](const PartyMember& m, double* v) { return evalMember(CondFunc::GetLevel, m, base, none, v); }, nullptr, 0, &out));
}

TEST(a_condition_run_by_an_object_script_finds_the_rules_of_its_quest) {
    fnvcoop::InMemoryNetwork net(1);
    fnvcoop::HostSession host(*net.endpoint(0));
    fnvcoop::RulesDb db;
    std::string err;
    CHECK(db.loadFile(NVH_RULES_JSON, &err));
    fnvcoop::RulesEngine eng(host, db, w1bResolve);
    const std::string causeJson = readWholeFile(NVH_CAUSE_QUESTS_JSON);
    CHECK(!causeJson.empty() && eng.loadCauseQuests(causeJson, &err));
    ConditionReducer red = makeRulesReducer(eng);
    const std::vector<CondValue> levels = {{0, 12}, {1, 30}, {2, 5}};
    // lonesomeroad.esm:009726 is a Marked Men gear script (object script) of NVDLC04MQ03a: GetLevel is max-over-party there
    const fnvcoop::FormRef gear("lonesomeroad.esm", 0x9726);
    CHECK(conditionQuest(eng, gear) == w1bQuest("NVDLC04MQ03a"));
    CHECK(red(gear, "GetLevel", levels, 0, fnvcoop::CondPhase::Entry) == 30.0);
    // the quest itself is left alone; an unmapped script keeps the initiator reduction (as before)
    CHECK(conditionQuest(eng, w1bQuest("NVDLC04MQ03a")) == w1bQuest("NVDLC04MQ03a"));
    const fnvcoop::FormRef stranger("falloutnv.esm", 0xABCDEF);
    CHECK(conditionQuest(eng, stranger) == stranger && red(stranger, "GetLevel", levels, 2, fnvcoop::CondPhase::Entry) == 5.0);
    // any other function of the mapped quest is reduced by its rule as well (GetActorValue is max-over-party in NVDLC04MQ03a)
    CHECK(red(gear, "GetActorValue", {{0, 40}, {1, 90}}, 0, fnvcoop::CondPhase::Entry) == 90.0);
}

TEST(player_kill_is_recorded_as_the_ref_of_the_player_it_names) {
    ActingContext ac;
    ac.registerSlot(0xFF000001, 1);
    ac.registerSlot(0xFF000002, 2);
    auto ref = [](uint8_t slot, fnvcoop::FormRef* out) { *out = fnvcoop::FormRef("falloutnv.esm", 0xF000u + slot); return true; };
    // the host's own player, nobody acting: slot 0, the engine kill runs here
    PlayerKillPlan p = planPlayerKill(ac.playerTargetOf(0x14), false, ref, true);
    CHECK(!p.legacy && p.record && p.runEngine && p.ref == fnvcoop::FormRef("falloutnv.esm", 0xF000));
    // a guest's ghost: the kill is recorded for THAT guest's ref and never runs on the host
    p = planPlayerKill(ac.playerTargetOf(0xFF000002), false, ref, true);
    CHECK(!p.legacy && p.record && !p.runEngine && p.ref == fnvcoop::FormRef("falloutnv.esm", 0xF002));
    // `player` inside a script that runs as guest 1 is guest 1
    ac.push(0xFF000001);
    p = planPlayerKill(ac.playerTargetOf(0x14), true, ref, true);
    ac.pop();
    CHECK(p.report && p.record && !p.runEngine && p.ref == fnvcoop::FormRef("falloutnv.esm", 0xF001));
    // not capturing (a guest machine): nothing recorded; the observer is still told
    p = planPlayerKill(ac.playerTargetOf(0xFF000002), true, ref, false);
    CHECK(p.report && !p.record && !p.legacy);
    // nothing wired: the old path (record the operand, run the engine kill), a non-player is never a player kill
    p = planPlayerKill(ac.playerTargetOf(0xFF000002), false, nullptr, true);
    CHECK(p.legacy && p.runEngine && !p.record);
    CHECK(planPlayerKill(ac.playerTargetOf(0x104C79), true, ref, true).legacy);

    // the whole contract with the rules engine: KillActor{<guest ref>} in a PerPlayerState quest drains only that player's health
    const char* rules = R"({"schema":1,"quests":{
      "Collar":{"tier":"T3","questline":"x","curated":false,"rules":[{"kind":"PerPlayerState","params":{"scope":"collar","vars":["fCollarTimer"]},"origin":"curated"}]}}})";
    fnvcoop::InMemoryNetwork net(1);
    fnvcoop::HostSession host(*net.endpoint(0));
    fnvcoop::RulesDb db;
    std::string err;
    CHECK(db.load(rules, &err));
    fnvcoop::RulesEngine::Options opt;
    opt.playerRef = [&](fnvcoop::PlayerId s, fnvcoop::FormRef& f) { return ref(static_cast<uint8_t>(s), &f); };
    fnvcoop::RulesEngine eng(host, db, w1bResolve, opt);
    p = planPlayerKill(ac.playerTargetOf(0xFF000002), false, ref, true);
    fnvcoop::Cause cause;
    cause.kind = fnvcoop::CauseKind::Script;
    cause.source = w1bQuest("Collar");
    host.commit(0, cause, {fnvcoop::KillActor{p.ref}});
    CHECK(host.state().ledger[2].av.count(fnvcoop::kAvHealth) == 1 && host.state().ledger[2].av.at(fnvcoop::kAvHealth) == -fnvcoop::kKillDrain);
    CHECK(host.state().ledger[0].av.count(fnvcoop::kAvHealth) == 0 && host.state().ledger[1].av.count(fnvcoop::kAvHealth) == 0);
    CHECK(eng.stats().killsRerouted == 1);
}

TEST(the_bridge_activation_passes_the_consoles_last_argument) {
    // activate.ref (MActivateRef) must call Activate(player, 0, 0, 1) like the console: the last argument 0 deletes a placed item (a Stimpak) and gives it to nobody
    const ActivateArgs a = consoleActivateArgs();
    CHECK(a.flagA == 0 && a.flagB == 0 && a.last == 1);
}

// ---- W3d: scripts that run once per slot, object variable copies, the condition scope of an INFO -------------------------------------------------------------------------
TEST(a_script_the_rules_run_per_slot_is_found_by_its_form_id) {
    PerSlotScripts t;
    CHECK(t.empty() && t.find(0x100) == nullptr);
    t.set({{0x100, {"binradius"}}, {0x200, {}}, {0, {"ignored"}}});   // a script with no form id is not a script
    CHECK(t.size() == 2);
    const PerSlotScript* s = t.find(0x100);
    CHECK(s && s->scriptId == 0x100 && s->vars.size() == 1 && s->vars[0] == "binradius");
    CHECK(t.find(0x200) && t.find(0x200)->vars.empty());
    CHECK(t.find(0x300) == nullptr);                                  // an ordinary script: the engine's own single run
    t.set({{0x300, {}}});
    CHECK(t.find(0x100) == nullptr && t.find(0x300) != nullptr);      // replaced, not merged
}

TEST(object_variable_copies_are_kept_per_instance_and_per_slot_and_the_host_values_come_back) {
    ObjVarCopies c;
    const std::vector<std::string> names = {"binradius", "missing"};
    double aIn = 0, bIn = 0;                                           // the bInRadius of two speakers (two script instances)
    const uint64_t A = objVarInstance(0x77, 0x1000), B = objVarInstance(0x77, 0x2000);   // the same script on two references
    c.seed(A, names, {&aIn, nullptr});
    c.seed(B, names, {&bIn, nullptr});                                 // pristine: 0 / 0
    aIn = 7;                                                           // the engine's own (host) run of speaker A set it
    ObjVarCopies::Bracket br;
    c.enter(A, 1, names, {&aIn, nullptr}, br);
    CHECK(aIn == 0);                                                   // slot 1 starts from the seed, not from the host's 7
    aIn = 1;                                                           // slot 1 is inside the radius of A
    c.leave(A, 1, names, {&aIn, nullptr}, br);
    CHECK(aIn == 7);                                                   // the host's value is back for the engine
    double got = -1;
    CHECK(c.copyOf(A, "binradius", 1, &got) && got == 1);
    c.enter(A, 2, names, {&aIn, nullptr}, br);                         // slot 2 on the same speaker: not slot 1's state
    CHECK(aIn == 0);
    c.leave(A, 2, names, {&aIn, nullptr}, br);
    CHECK(c.copyOf(A, "binradius", 2, &got) && got == 0 && aIn == 7);
    c.enter(A, 1, names, {&aIn, nullptr}, br);                         // slot 1 again: its own copy
    CHECK(aIn == 1);
    c.leave(A, 1, names, {&aIn, nullptr}, br);
    c.enter(B, 1, names, {&bIn, nullptr}, br);                         // the other speaker, same slot: not speaker A's copy
    CHECK(bIn == 0);
    c.leave(B, 1, names, {&bIn, nullptr}, br);
    CHECK(c.copies() == 3 && !c.copyOf(A, "missing", 1, nullptr));     // a variable the script lacks is never stored
    // the copies and the seeds can be exported and put back (they travel with the host's save, plugin/coop/objvar_sidecar.h); an instance is the script and its reference
    CHECK(A != B && (A >> 32) == 0x77 && (A & 0xFFFFFFFFu) == 0x1000);
    ObjVarCopies d;
    d.importAll(c.exportCopies(), c.exportSeeds());
    CHECK(d.copies() == 3 && d.copyOf(A, "binradius", 1, &got) && got == 1 && d.copyOf(A, "binradius", 2, &got) && got == 0 && d.copyOf(B, "binradius", 1, &got) && got == 0);
    CHECK(d.exportSeeds().size() == c.exportSeeds().size());
    d.importAll({}, {});
    CHECK(d.copies() == 0 && d.exportSeeds().empty());
}

TEST(an_infos_conditions_are_reduced_under_the_quest_of_that_info) {
    CHECK(addr::InfoConditions.target == 0x61E600 && addr::InfoConditions.retBytes == 16 && addr::InfoConditions.stolen == 6);   // TESTopicInfo::CheckConditions: thiscall, 4 stack args
    fnvcoop::InMemoryNetwork net(1);
    fnvcoop::HostSession host(*net.endpoint(0));
    fnvcoop::RulesDb db;
    std::string err;
    CHECK(db.loadFile(NVH_RULES_JSON, &err));
    fnvcoop::RulesEngine eng(host, db, w1bResolve);
    ConditionReducer red = makeRulesReducer(eng);
    const fnvcoop::FormRef jedQuest = w1bQuest("NVDLC02DialogueCaravanMaster");
    int asked = 0;
    InfoQuestScopes scopes;
    CHECK(!scopes.active());
    scopes.set([&](uint32_t id, fnvcoop::FormRef* q) {
        ++asked;
        if (id != 0x01009EB6u && id != 0x01011525u) return false;     // the two Jed weight-gate INFOs (HonestHearts.esm 009EB6 / 011525)
        *q = jedQuest;
        return true;
    });
    fnvcoop::FormRef q;
    CHECK(scopes.active() && scopes.questOf(0x01009EB6u, &q) && q == jedQuest);
    CHECK(!scopes.questOf(0x01000001u, &q) && !scopes.questOf(0, &q));
    CHECK(scopes.questOf(0x01009EB6u, &q) && !scopes.questOf(0x01000001u, &q) && asked == 2);   // answers (the negative one too) are remembered
    // the Jed gate: `GetActorValue InventoryWeight >= 76`. Under the INFO's quest the heaviest pack of the party decides; without the scope the initiator's pack does.
    const std::vector<CondValue> packs = {{0, 30}, {1, 120}};
    CHECK(red(q, "GetActorValue", packs, 0, fnvcoop::CondPhase::Entry) == 120.0);
    const fnvcoop::FormRef info("honesthearts.esm", 0x9EB6);
    CHECK(red(info, "GetActorValue", packs, 0, fnvcoop::CondPhase::Entry) == 30.0);
    scopes.set(nullptr);
    CHECK(!scopes.active() && !scopes.questOf(0x01009EB6u, &q) && scopes.cached() == 0);
}

int main() {
    run("install_patches_entry_and_builds_trampoline", install_patches_entry_and_builds_trampoline);
    run("wrong_prologue_refuses_and_writes_nothing", wrong_prologue_refuses_and_writes_nothing);
    run("unrelated_bytes_refuse", unrelated_bytes_refuse);
    run("foreign_detour_is_reported_not_overwritten", foreign_detour_is_reported_not_overwritten);
    run("bad_specs_are_rejected", bad_specs_are_rejected);
    run("refcount_install_release_restore", refcount_install_release_restore);
    run("different_detour_for_same_target_refused", different_detour_for_same_target_refused);
    run("tampered_code_not_restored", tampered_code_not_restored);
    run("alloc_or_write_failure_leaves_game_code_untouched", alloc_or_write_failure_leaves_game_code_untouched);
    run("registry_destructor_restores_everything", registry_destructor_restores_everything);
    run("reload_cycles_restore_every_byte_and_balance_live_count", reload_cycles_restore_every_byte_and_balance_live_count);
    run("reload_restore_failure_is_reported_and_never_hangs", reload_restore_failure_is_reported_and_never_hangs);
    run("reload_preflight_finds_chained_hooks_without_touching_anything", reload_preflight_finds_chained_hooks_without_touching_anything);
    run("reload_suspend_resume_keeps_trampolines_and_release_still_restores", reload_suspend_resume_keeps_trampolines_and_release_still_restores);
    run("trampoline_lookup_names_the_original_code_of_installed_hooks_only", trampoline_lookup_names_the_original_code_of_installed_hooks_only);
    run("reload_suspend_leaves_chained_hooks_alone_and_process_wide_helpers_see_every_registry", reload_suspend_leaves_chained_hooks_alone_and_process_wide_helpers_see_every_registry);
    run("reload_tampered_hook_is_counted_so_the_loader_can_refuse_to_unload", reload_tampered_hook_is_counted_so_the_loader_can_refuse_to_unload);
    run("chain_over_foreign_entry_jump_relocates_it_and_release_puts_it_back", chain_over_foreign_entry_jump_relocates_it_and_release_puts_it_back);
    run("xnvse_primed_before_install_keeps_the_hook_and_the_swap_alive", xnvse_primed_before_install_keeps_the_hook_and_the_swap_alive);
    run("chain_refuses_a_foreign_jump_over_a_different_function_or_into_ourselves", chain_refuses_a_foreign_jump_over_a_different_function_or_into_ourselves);
    run("extract_args_extra_parameters_land_in_spare_slots_not_on_the_stack", extract_args_extra_parameters_land_in_spare_slots_not_on_the_stack);
    run("engine_address_table_is_sane", engine_address_table_is_sane);
    run("engine_table_signature_roundtrip_on_fake_image", engine_table_signature_roundtrip_on_fake_image);
    run("guest_local_run_script_may_mutate_on_guest_but_others_may_not", guest_local_run_script_may_mutate_on_guest_but_others_may_not);
    run("guest_local_run_data_file_parses", guest_local_run_data_file_parses);
    run("guest_local_run_shipped_list_covers_required_setup_quests", guest_local_run_shipped_list_covers_required_setup_quests);
    run("acting_slot_maps_ghosts_to_party_slots", acting_slot_maps_ghosts_to_party_slots);
    run("companion_apply_is_for_the_owner_machine_only", companion_apply_is_for_the_owner_machine_only);
    run("fast_travel_locks_enable_travel_only_when_last_scope_releases", fast_travel_locks_enable_travel_only_when_last_scope_releases);
    run("current_source_names_the_outermost_explicit_scope_or_weak_script", current_source_names_the_outermost_explicit_scope_or_weak_script);
    run("party_condition_swap_plan_only_touches_the_player_operand", party_condition_swap_plan_only_touches_the_player_operand);
    run("party_condition_runs_once_per_member_and_reduces", party_condition_runs_once_per_member_and_reduces);
    run("party_conditions_reduce_through_the_rules_engine", party_conditions_reduce_through_the_rules_engine);
    run("every_hooked_condition_function_has_a_rule_in_rules_json", every_hooked_condition_function_has_a_rule_in_rules_json);
    run("party_condition_plan_for_the_nine_new_functions", party_condition_plan_for_the_nine_new_functions);
    run("party_member_value_ghost_view_state_view_and_owner_view", party_member_value_ghost_view_state_view_and_owner_view);
    run("party_member_value_inventory_view_asks_the_reported_pack_first_and_the_puppet_when_unknown", party_member_value_inventory_view_asks_the_reported_pack_first_and_the_puppet_when_unknown);
    run("a_guests_remove_item_skips_the_engine_and_is_recorded_for_the_rules", a_guests_remove_item_skips_the_engine_and_is_recorded_for_the_rules);
    run("party_conditions_over_the_new_functions_follow_their_rules", party_conditions_over_the_new_functions_follow_their_rules);
    run("the_host_skips_its_controls_call_only_for_a_guest_only_lock", the_host_skips_its_controls_call_only_for_a_guest_only_lock);
    run("the_capture_hub_names_the_cause_of_its_outermost_scope", the_capture_hub_names_the_cause_of_its_outermost_scope);
    run("a_ref_variable_slot_holds_the_form_id_in_its_low_dword", a_ref_variable_slot_holds_the_form_id_in_its_low_dword);
    run("kill_attribution_writes_the_engine_killer_slot", kill_attribution_writes_the_engine_killer_slot);
    run("guest_local_run_two_game_diff_quests_are_per_machine", guest_local_run_two_game_diff_quests_are_per_machine);
    run("muted_hub_captures_nothing_for_guest_local_scripts", muted_hub_captures_nothing_for_guest_local_scripts);
    run("muted_hub_ignores_quest_var_of_guest_local_quest_owner", muted_hub_ignores_quest_var_of_guest_local_quest_owner);
    run("guest_dlc_setup_script_runs_but_its_quest_calls_are_the_hosts", guest_dlc_setup_script_runs_but_its_quest_calls_are_the_hosts);
    run("host_dlc_setup_script_quest_calls_are_captured_though_its_effects_are_muted", host_dlc_setup_script_quest_calls_are_captured_though_its_effects_are_muted);
    run("host_quest_state_scripts_are_all_on_the_guest_local_run_allow_list", host_quest_state_scripts_are_all_on_the_guest_local_run_allow_list);
    run("per_machine_quest_vars_are_not_shared", per_machine_quest_vars_are_not_shared);
    run("ref_angle_json_and_capture", ref_angle_json_and_capture);
    run("player_operands_resolve_to_exactly_one_party_slot", player_operands_resolve_to_exactly_one_party_slot);
    run("remove_from_all_factions_names_every_faction_the_actor_was_in", remove_from_all_factions_names_every_faction_the_actor_was_in);
    run("host_wraps_only_combat_events_of_a_registered_puppet", host_wraps_only_combat_events_of_a_registered_puppet);
    run("clearing_the_slot_table_lets_a_ninth_puppet_keep_its_slot", clearing_the_slot_table_lets_a_ninth_puppet_keep_its_slot);
    run("scripted_lock_and_unlock_carry_the_lock_pre_image", scripted_lock_and_unlock_carry_the_lock_pre_image);
    run("player_facts_answer_the_four_reads_by_kind_and_thing", player_facts_answer_the_four_reads_by_kind_and_thing);
    run("guest_local_run_list_is_read_from_the_rules_json_rule", guest_local_run_list_is_read_from_the_rules_json_rule);
    run("editing_the_guest_local_run_rule_changes_what_a_guest_runs", editing_the_guest_local_run_rule_changes_what_a_guest_runs);
    run("shipped_rules_json_guest_local_run_rules_match_the_legacy_list_file", shipped_rules_json_guest_local_run_rules_match_the_legacy_list_file);
    run("no_scope_no_record", no_scope_no_record);
    run("scope_emits_one_txn_with_cause_and_order", scope_emits_one_txn_with_cause_and_order);
    run("nested_scopes_merge_into_outermost_txn", nested_scopes_merge_into_outermost_txn);
    run("echo_guard_blocks_capture_of_remote_apply", echo_guard_blocks_capture_of_remote_apply);
    run("disabled_hub_is_inert", disabled_hub_is_inert);
    run("unbalanced_end_is_ignored", unbalanced_end_is_ignored);
    run("guest_local_run_allow_list_keeps_selected_quest_scripts_running", guest_local_run_allow_list_keeps_selected_quest_scripts_running);
    run("acting_context_is_a_stack_defaulting_to_the_host_player", acting_context_is_a_stack_defaulting_to_the_host_player);
    run("weak_script_scope_captures_ambient_mutations_with_script_cause", weak_script_scope_captures_ambient_mutations_with_script_cause);
    run("explicit_scope_inside_weak_scope_keeps_its_own_cause", explicit_scope_inside_weak_scope_keeps_its_own_cause);
    run("inventory_delta_reports_resolved_items_not_the_list", inventory_delta_reports_resolved_items_not_the_list);
    run("failed_quest_txn_carries_fail_only_not_complete", failed_quest_txn_carries_fail_only_not_complete);
    run("consecutive_identical_state_effects_collapse_but_quantities_do_not", consecutive_identical_state_effects_collapse_but_quantities_do_not);
    run("stale_scope_is_reaped_at_frame_boundary", stale_scope_is_reaped_at_frame_boundary);
    run("stage_gate_defers_unless_applying_or_bypassed", stage_gate_defers_unless_applying_or_bypassed);
    run("engine_table_pins_stack_bytes_of_hooked_functions", engine_table_pins_stack_bytes_of_hooked_functions);
    run("apply_lines_pin_command_names_and_parameter_order", apply_lines_pin_command_names_and_parameter_order);
    run("change_tracker_reports_first_sight_and_changes_only", change_tracker_reports_first_sight_and_changes_only);
    run("change_tracker_priming_takes_the_present_world_as_known", change_tracker_priming_takes_the_present_world_as_known);
    run("guest_objvars_table_lists_the_quest_vars_that_object_scripts_write", guest_objvars_table_lists_the_quest_vars_that_object_scripts_write);
    run("var_rollback_puts_back_only_what_a_script_changed", var_rollback_puts_back_only_what_a_script_changed);
    run("guest_ganger_death_scripts_do_not_count_on_the_guest_the_host_value_wins", guest_ganger_death_scripts_do_not_count_on_the_guest_the_host_value_wins);
    run("suppress_host_never_blocks", suppress_host_never_blocks);
    run("suppress_guest_blocks_script_mutations_only_inside_scripts", suppress_guest_blocks_script_mutations_only_inside_scripts);
    run("suppress_guest_skips_quest_scripts_but_not_object_scripts", suppress_guest_skips_quest_scripts_but_not_object_scripts);
    run("suppress_guest_result_scripts_never_run_locally", suppress_guest_result_scripts_never_run_locally);
    run("guest_console_line_on_own_player_is_local", guest_console_line_on_own_player_is_local);
    run("local_scope_guest_character_line_passes_inside_and_is_dropped_outside", local_scope_guest_character_line_passes_inside_and_is_dropped_outside);
    run("local_scope_run_is_per_machine_so_the_host_never_captures_it", local_scope_run_is_per_machine_so_the_host_never_captures_it);
    run("local_scope_exempts_console_lines_only_not_esm_scripts", local_scope_exempts_console_lines_only_not_esm_scripts);
    run("local_scope_nests_and_unbalanced_leave_is_harmless", local_scope_nests_and_unbalanced_leave_is_harmless);
    run("local_scope_leaves_the_player_prefix_rule_and_the_other_exemptions_alone", local_scope_leaves_the_player_prefix_rule_and_the_other_exemptions_alone);
    run("suppress_lifted_while_applying_host_txn", suppress_lifted_while_applying_host_txn);
    run("effect_json_roundtrip_every_type", effect_json_roundtrip_every_type);
    run("script_vars_become_setscriptvar_effects", script_vars_become_setscriptvar_effects);
    run("multiple_sinks_all_see_every_txn", multiple_sinks_all_see_every_txn);
    run("scope_observers_run_on_outermost_only_and_can_record", scope_observers_run_on_outermost_only_and_can_record);
    run("party_condition_plan_for_the_wave1b_functions", party_condition_plan_for_the_wave1b_functions);
    run("every_function_a_partyawarecond_rule_names_has_a_decision", every_function_a_partyawarecond_rule_names_has_a_decision);
    run("party_level_reduces_to_the_host_or_to_the_strongest_member", party_level_reduces_to_the_host_or_to_the_strongest_member);
    run("a_condition_run_by_an_object_script_finds_the_rules_of_its_quest", a_condition_run_by_an_object_script_finds_the_rules_of_its_quest);
    run("player_kill_is_recorded_as_the_ref_of_the_player_it_names", player_kill_is_recorded_as_the_ref_of_the_player_it_names);
    run("the_bridge_activation_passes_the_consoles_last_argument", the_bridge_activation_passes_the_consoles_last_argument);
    run("a_script_the_rules_run_per_slot_is_found_by_its_form_id", a_script_the_rules_run_per_slot_is_found_by_its_form_id);
    run("object_variable_copies_are_kept_per_instance_and_per_slot_and_the_host_values_come_back", object_variable_copies_are_kept_per_instance_and_per_slot_and_the_host_values_come_back);
    run("an_infos_conditions_are_reduced_under_the_quest_of_that_info", an_infos_conditions_are_reduced_under_the_quest_of_that_info);
    std::printf("%d checks, %d failed\n", g_checks, g_fail);
    return g_fail ? 1 : 0;
}
