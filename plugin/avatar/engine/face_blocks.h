// The FaceGen morph coefficients of an NPC base, as they travel in the Look message (Appearance::faceCoeffs). Game-free.
//
// TESNPC::faceGenData[3] holds three float arrays: FGGS (geometry, symmetric, 50), FGGA (geometry, asymmetric, 30) and FGTS (texture, symmetric, 50:
// the skin tone and the face tint). The sender lays the readable blocks one after another, every readable block with all of its numbers, ZEROS
// INCLUDED: a face whose asymmetric block is all zero (a default face, an untouched slider set) is real data. The receiver has the same three blocks in
// the copy it writes into and accepts the vector only when its length is exactly the sum of the counts of the blocks it can read itself, so a block
// that one side could not read can never shift the numbers of the next block into the wrong array (the first version dropped all-zero blocks on the
// sender and wrote by its own block sizes on the receiver: the skin tone landed in the geometry).
#pragma once
#include <cmath>
#include <cstddef>
#include <cstdint>
#include <vector>

#include "fnvcoop_mut.h"   // twin 960: the old sender that leaves out an all-zero block

namespace nvcoop::avatar::engine {

constexpr int kFaceBlockCount = 3;
constexpr uint32_t kFaceBlockMaxCount = 64;

struct FaceBlockRead {
    bool readable = false;       // the block exists, has a sane size and every float is finite
    std::vector<float> values;
};

// The numbers are quantised to 1/16 and clamped to what an int8 holds. Returns the number of blocks that went into `out` (which is cleared first).
inline int PackFaceBlocks(const FaceBlockRead (&blocks)[kFaceBlockCount], std::vector<int8_t>& out) {
    out.clear();
    int used = 0;
    for (int b = 0; b < kFaceBlockCount; ++b) {
        const FaceBlockRead& blk = blocks[b];
        if (!blk.readable || blk.values.empty() || blk.values.size() > kFaceBlockMaxCount) continue;
        bool anyNonZero = false;
        std::vector<int8_t> q;
        q.reserve(blk.values.size());
        for (float f : blk.values) {
            long v = std::lround(f * 16.0f);
            v = v < -127 ? -127 : (v > 127 ? 127 : v);
            if (v != 0) anyNonZero = true;
            q.push_back(static_cast<int8_t>(v));
        }
        if (FNV_MUT(960) && !anyNonZero) continue;   // MUTANT 960: an all-zero block is left out (the receiver's blocks no longer line up)
        out.insert(out.end(), q.begin(), q.end());
        ++used;
    }
    return used;
}

// Where one block of the receiving copy lives. count 0 = this machine cannot read that block. dst null = it can, but must not be written (the copy
// shares the array with the template, which is the local player's own face).
struct FaceSlot {
    uint32_t count = 0;
    float* dst = nullptr;
};

// The expected length of a face vector for these slots.
inline size_t FaceVectorLength(const FaceSlot (&slots)[kFaceBlockCount]) {
    size_t n = 0;
    for (const FaceSlot& s : slots) n += s.count;
    return n;
}

// Writes the coefficients into the writable blocks. False (nothing written) when the vector does not have exactly the length of the blocks this
// machine can read. *wrote tells whether any block was written.
inline bool UnpackFaceBlocks(const int8_t* coeffs, size_t n, const FaceSlot (&slots)[kFaceBlockCount], bool* wrote) {
    if (wrote) *wrote = false;
    if (!coeffs || n == 0 || n != FaceVectorLength(slots)) return false;
    size_t k = 0;
    for (const FaceSlot& s : slots) {
        if (!s.count) continue;
        if (s.dst) {
            for (uint32_t i = 0; i < s.count; ++i) s.dst[i] = coeffs[k + i] / 16.0f;
            if (wrote) *wrote = true;
        }
        k += s.count;
    }
    return true;
}

}  // namespace nvcoop::avatar::engine
