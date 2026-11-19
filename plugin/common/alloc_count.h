// What nvcoop.dll itself holds on the heap (alloc_count.cpp). Only the game dll counts; test executables leave it out.
#pragma once
#include <cstddef>
#include <cstdint>

namespace nvcoop {
struct AllocStats {
    uint64_t live = 0;       // bytes this dll allocated with operator new and has not freed
    uint64_t peak = 0;       // the most `live` has been
    uint64_t enetLive = 0;   // bytes ENet holds (packets, peers), when ENet was started with the counting callbacks
};
AllocStats allocStats();
// ENetCallbacks for enet_initialize_with_callbacks (plain malloc / free underneath).
void* enetCountedMalloc(size_t n);
void enetCountedFree(void* p);
}  // namespace nvcoop
