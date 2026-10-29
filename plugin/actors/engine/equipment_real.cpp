// The raw read behind EquipmentAccess (equipment.h): the worn entries of an actor reference. x86 only, compiled against the xNVSE headers.
//
// The walk is the one plugin/avatar uses for the player's puppets (ReadEquippedRaw), repeated here on purpose so the two lanes stay independent:
// the reference's extra data list (TESObjectREFR +0x44, its first node at +4 from there) holds an ExtraContainerChanges (type 0x15, data at +0x0C)
// whose objList is a tList of EntryData { extendData @0, countDelta @4, type @8 }; an entry is worn when one of its extra data lists holds
// ExtraWorn (0x16) or ExtraWornLeft (0x17). The form is at type+0x0C (refID), its type id at type+4. Layouts: xNVSE GameExtraData.h / GameForms.h
// for FalloutNV.exe 1.4.0.525. NEVER RUN IN A GAME YET. Every access is SEH guarded: a wrong offset yields "unreadable", never a crash.
#include <Windows.h>

#include <cstdint>
#include <cstring>
#include <vector>

#include "actors/engine/equipment_real.h"

#include "nvse/GameAPI.h"
#include "nvse/GameForms.h"
#include "nvse/GameObjects.h"

namespace nvcoop::actors::engine {

namespace {

constexpr uintptr_t kRefExtraDataList = 0x44;
constexpr uint8_t kExtraContainerChanges = 0x15, kExtraWorn = 0x16, kExtraWornLeft = 0x17;
constexpr int kMaxEntries = 64;

bool SafeRead(uintptr_t addr, void* out, size_t n) {
    __try {
        std::memcpy(out, reinterpret_cast<const void*>(addr), n);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

TESForm* LookupSEH(uint32_t id) {
    if (!id) return nullptr;
    __try {
        return LookupFormByID(id);
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return nullptr;
    }
}

// Plain data only inside the __try (no C++ objects with destructors). Returns the number of entries written, or -1 when the reference is unreadable.
int ReadWornRaw(uintptr_t ref, WornEntry* out, int max) {
    __try {
        uintptr_t x = 0;
        if (!SafeRead(ref + kRefExtraDataList + 4, &x, sizeof x)) return -1;
        uintptr_t ecc = 0;
        for (int guard = 0; x && guard < 64; ++guard) {
            uint8_t type = 0;
            if (!SafeRead(x + 4, &type, 1)) return -1;
            if (type == kExtraContainerChanges) { ecc = x; break; }
            if (!SafeRead(x + 8, &x, sizeof x)) return -1;
        }
        if (!ecc) return 0;   // no container changes: it wears nothing we know of
        uintptr_t data = 0;
        if (!SafeRead(ecc + 0x0C, &data, sizeof data) || !data) return 0;
        uintptr_t objList = 0;
        if (!SafeRead(data, &objList, sizeof objList) || !objList) return 0;
        int n = 0;
        uintptr_t node = objList;   // tList: the list object is its own first node { data, next }
        for (int guard = 0; node && guard < 1024; ++guard) {
            uintptr_t entry = 0, next = 0;
            if (!SafeRead(node, &entry, sizeof entry) || !SafeRead(node + 4, &next, sizeof next)) return n;
            if (entry) {
                uintptr_t ext = 0, type = 0;
                SafeRead(entry, &ext, sizeof ext);
                SafeRead(entry + 8, &type, sizeof type);
                bool worn = false;
                if (ext && type) {
                    uintptr_t en = ext;   // ExtendDataList: tList<ExtraDataList>
                    for (int g2 = 0; en && g2 < 64 && !worn; ++g2) {
                        uintptr_t edl = 0, enext = 0;
                        if (!SafeRead(en, &edl, sizeof edl) || !SafeRead(en + 4, &enext, sizeof enext)) break;
                        if (edl) {
                            uintptr_t b = 0;   // BaseExtraList: vtbl @ 0, m_data @ +4
                            SafeRead(edl + 4, &b, sizeof b);
                            for (int g3 = 0; b && g3 < 64; ++g3) {
                                uint8_t t = 0;
                                if (!SafeRead(b + 4, &t, 1)) break;
                                if (t == kExtraWorn || t == kExtraWornLeft) { worn = true; break; }
                                if (!SafeRead(b + 8, &b, sizeof b)) break;
                            }
                        }
                        en = enext;
                    }
                }
                if (worn && n < max) {
                    uint32_t form = 0;
                    uint8_t ftype = 0;
                    SafeRead(type + 0x0C, &form, sizeof form);
                    SafeRead(type + 4, &ftype, 1);
                    if (form) { out[n].form = form; out[n].formType = ftype; ++n; }
                }
            }
            node = next;
        }
        return n;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return -1;
    }
}

}  // namespace

bool readWornEntries(RefId ref, std::vector<WornEntry>& out) {
    out.clear();
    TESForm* f = LookupSEH(ref);
    if (!f || (f->typeID != kFormType_Character && f->typeID != kFormType_Creature)) return false;
    WornEntry buf[kMaxEntries];
    const int n = ReadWornRaw(reinterpret_cast<uintptr_t>(f), buf, kMaxEntries);
    if (n < 0) return false;
    out.assign(buf, buf + n);
    return true;
}

}  // namespace nvcoop::actors::engine
