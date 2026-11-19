// Counts what nvcoop.dll itself holds on the heap, so a memory climb in the game can be pinned on this mod or ruled out.
// The process heap is shared by every NVSE plugin and NV:MP's client (each static CRT allocates from it), and the real out-of-memory
// sessions (2026-11-16, 2026-11-17) only showed that heap growing. The global operator new / delete of this dll are replaced by thin
// counting wrappers over malloc / free (layout unchanged: the block is a plain malloc block, _msize gives its size), and ENet's
// allocations are counted the same way through enet_initialize_with_callbacks (see enet_transport.cpp: alloc_count.h hooks).
#include "common/alloc_count.h"
#include <malloc.h>
#include <algorithm>
#include <atomic>
#include <cstdlib>
#include <new>

namespace {
// Constant-initialized: valid before any static constructor of this dll runs (those allocate too).
std::atomic<long long> g_live{0}, g_peak{0}, g_enetLive{0};

void noteAlloc(void* p) {
    if (!p) return;
    const long long now = g_live.fetch_add(static_cast<long long>(_msize(p)), std::memory_order_relaxed) + static_cast<long long>(_msize(p));
    long long peak = g_peak.load(std::memory_order_relaxed);
    while (now > peak && !g_peak.compare_exchange_weak(peak, now, std::memory_order_relaxed)) {}
}
void noteFree(void* p) {
    if (p) g_live.fetch_sub(static_cast<long long>(_msize(p)), std::memory_order_relaxed);
}
void* countedNew(size_t n) {
    for (;;) {
        if (void* p = std::malloc(n ? n : 1)) { noteAlloc(p); return p; }
        std::new_handler h = std::get_new_handler();
        if (!h) throw std::bad_alloc();
        h();
    }
}
}  // namespace

namespace nvcoop {
AllocStats allocStats() {
    return {static_cast<uint64_t>((std::max)(0ll, g_live.load())), static_cast<uint64_t>((std::max)(0ll, g_peak.load())),
            static_cast<uint64_t>((std::max)(0ll, g_enetLive.load()))};
}
void* enetCountedMalloc(size_t n) {
    void* p = std::malloc(n ? n : 1);
    if (p) g_enetLive.fetch_add(static_cast<long long>(_msize(p)), std::memory_order_relaxed);
    return p;
}
void enetCountedFree(void* p) {
    if (!p) return;
    g_enetLive.fetch_sub(static_cast<long long>(_msize(p)), std::memory_order_relaxed);
    std::free(p);
}
}  // namespace nvcoop

void* operator new(size_t n) { return countedNew(n); }
void* operator new[](size_t n) { return countedNew(n); }
void* operator new(size_t n, const std::nothrow_t&) noexcept { try { return countedNew(n); } catch (...) { return nullptr; } }
void* operator new[](size_t n, const std::nothrow_t&) noexcept { try { return countedNew(n); } catch (...) { return nullptr; } }
void operator delete(void* p) noexcept { noteFree(p); std::free(p); }
void operator delete[](void* p) noexcept { noteFree(p); std::free(p); }
void operator delete(void* p, size_t) noexcept { noteFree(p); std::free(p); }
void operator delete[](void* p, size_t) noexcept { noteFree(p); std::free(p); }
void operator delete(void* p, const std::nothrow_t&) noexcept { noteFree(p); std::free(p); }
void operator delete[](void* p, const std::nothrow_t&) noexcept { noteFree(p); std::free(p); }
