// Ref-typed quest script variables (W3a item 4). The engine keeps a `ref` local in the same 8-byte slot as a float (ScriptLocal::data, a double), but the
// value is the FORM ID in the low dword of those 8 bytes, not the number: storing 0x01002345 as a double 16786245.0 would leave a garbage ref. Callers of
// nvhooks::questVarValue / setQuestVarValue use the plain number (the form id) for a ref variable; these two functions convert.
#pragma once
#include <cstdint>
#include <cstring>

namespace nvhooks {

// The slot content for form id `formId` (`formIdAsNumber` is the id as a double, which is exact for every 32 bit id).
inline double refVarSlotFromNumber(double formIdAsNumber) {
#ifdef NVHOOKS_MUTANT
    return formIdAsNumber;   // mutant: the number is stored as a double (the ref reads back as a denormal)
#else
    const uint64_t bits = static_cast<uint32_t>(static_cast<int64_t>(formIdAsNumber));
    double d;
    std::memcpy(&d, &bits, sizeof d);
    return d;
#endif
}
// The form id (as a number) a ref variable slot holds.
inline double refVarNumberFromSlot(double slot) {
#ifdef NVHOOKS_MUTANT
    return slot;
#else
    uint64_t bits;
    std::memcpy(&bits, &slot, sizeof bits);
    return static_cast<double>(static_cast<uint32_t>(bits));
#endif
}

}  // namespace nvhooks
