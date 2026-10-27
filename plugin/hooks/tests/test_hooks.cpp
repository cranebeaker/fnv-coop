// Unit tests for the hook patcher (fake memory) and the capture hub. Exit code != 0 on any failure.
// Built twice: against the real sources, and against NVHOOKS_MUTANT sources (the latter MUST fail).
#include <cstdarg>
#include <cstdio>
#include <cstring>
#include <string>
#include <map>
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
#include "../guest_local_run.h"
#include "../guest_objvars.h"
#include "../party_cond.h"
#include "../rules_reducer.h"
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
    // Another plugin's module: code at [kForeign, kForeign + 0x10000) when foreignOn.
    static constexpr Addr kForeign = 0x60000000;
    bool foreignOn = false;
    bool foreignCode(Addr a) override { return foreignOn && a >= kForeign && a < kForeign + 0x10000; }
    // Re-point the E8 at `off` of the function at `target` to `to` (what ShowOff / Stewie's Tweaks do under TTW).
    void retarget(Addr target, unsigned off, Addr to) {
        uint32_t rel = static_cast<uint32_t>(to - (target + off + 5));
        std::memcpy(&image[target - kImage + off + 1], &rel, 4);
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

TEST(call_retargeted_to_another_plugin_past_the_stolen_bytes_is_accepted) {
    FakeMemory m;
    m.foreignOn = true;
    HookSpec s = makeSpec();
    m.place(s);
    m.retarget(s.target, 12, FakeMemory::kForeign + 0x100);   // E8 at index 12, its operand runs past the 16 bytes
    auto theirs = m.bytes(s.target + 12, 5);
    HookRegistry reg(m);
    Addr tramp = 0;
    CHECK(reg.acquire(s, 0x30000000, &tramp) == PatchStatus::Ok);
    CHECK(m.bytes(s.target + 12, 5) == theirs);                // their call is left as they set it
    CHECK(std::memcmp(m.bytes(tramp, 6).data(), s.expected, 6) == 0);
}

TEST(call_retargeted_inside_the_image_still_refuses) {
    FakeMemory m;
    m.foreignOn = true;
    HookSpec s = makeSpec();
    m.place(s);
    m.retarget(s.target, 12, FakeMemory::kImage + 0x800);     // another exe build: the call stays in the image
    auto before = m.image;
    HookRegistry reg(m);
    CHECK(reg.acquire(s, 0x30000000, nullptr) == PatchStatus::PrologueMismatch);
    CHECK(m.image == before);
}

TEST(call_retargeted_to_another_plugin_refuses_without_image_check) {
    FakeMemory m;   // foreignOn stays false: the default MemoryOps::foreignCode, every byte must match
    HookSpec s = makeSpec();
    m.place(s);
    m.retarget(s.target, 12, FakeMemory::kForeign + 0x100);
    HookRegistry reg(m);
    CHECK(reg.acquire(s, 0x30000000, nullptr) == PatchStatus::PrologueMismatch);
    CHECK(m.writes == 0);
}

TEST(stolen_byte_difference_refuses_even_with_a_foreign_call) {
    FakeMemory m;
    m.foreignOn = true;
    HookSpec s = makeSpec();
    m.place(s);
    m.retarget(s.target, 12, FakeMemory::kForeign + 0x100);
    m.image[s.target - FakeMemory::kImage + 3] ^= 0x01;       // a byte we would overwrite differs
    HookRegistry reg(m);
    CHECK(reg.acquire(s, 0x30000000, nullptr) == PatchStatus::PrologueMismatch);
    CHECK(m.writes == 0);
}

TEST(non_call_byte_past_the_stolen_bytes_still_refuses) {
    FakeMemory m;
    m.foreignOn = true;
    HookSpec s = makeSpec();
    m.place(s);
    m.image[s.target - FakeMemory::kImage + 8] ^= 0x01;       // part of `mov ecx,[ebp-8]`, not a call operand
    HookRegistry reg(m);
    CHECK(reg.acquire(s, 0x30000000, nullptr) == PatchStatus::PrologueMismatch);
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

int main() {
    run("install_patches_entry_and_builds_trampoline", install_patches_entry_and_builds_trampoline);
    run("wrong_prologue_refuses_and_writes_nothing", wrong_prologue_refuses_and_writes_nothing);
    run("call_retargeted_to_another_plugin_past_the_stolen_bytes_is_accepted",
        call_retargeted_to_another_plugin_past_the_stolen_bytes_is_accepted);
    run("call_retargeted_inside_the_image_still_refuses", call_retargeted_inside_the_image_still_refuses);
    run("call_retargeted_to_another_plugin_refuses_without_image_check",
        call_retargeted_to_another_plugin_refuses_without_image_check);
    run("stolen_byte_difference_refuses_even_with_a_foreign_call", stolen_byte_difference_refuses_even_with_a_foreign_call);
    run("non_call_byte_past_the_stolen_bytes_still_refuses", non_call_byte_past_the_stolen_bytes_still_refuses);
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
    run("suppress_lifted_while_applying_host_txn", suppress_lifted_while_applying_host_txn);
    run("effect_json_roundtrip_every_type", effect_json_roundtrip_every_type);
    run("script_vars_become_setscriptvar_effects", script_vars_become_setscriptvar_effects);
    run("multiple_sinks_all_see_every_txn", multiple_sinks_all_see_every_txn);
    run("scope_observers_run_on_outermost_only_and_can_record", scope_observers_run_on_outermost_only_and_can_record);
    std::printf("%d checks, %d failed\n", g_checks, g_fail);
    return g_fail ? 1 : 0;
}
