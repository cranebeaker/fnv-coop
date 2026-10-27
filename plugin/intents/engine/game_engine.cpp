// IIntentEngine over xNVSE (see game_engine.h). Style follows plugin/hooks_world/world_hooks.cpp: plain structs read under SEH, no C++ objects
// inside __try, console lines for what the engine only exposes as script commands.
#include "intents/engine/game_engine.h"

#include <Windows.h>

#include <cstdint>
#include <cstdio>
#include <cstring>

#include "GameAPI.h"
#include "GameData.h"
#include "GameExtraData.h"
#include "GameForms.h"
#include "GameObjects.h"
#include "GameScript.h"
#include "common/nvcoop_api.h"
#include "intents/engine/intents_addrs.h"
#include "engine_addrs.h"
#include "engine_hooks.h"

namespace nvcoop::intents::game {

namespace {

// ---- raw reads (SEH) -----------------------------------------------------------------------------------------------------------------

constexpr uint8_t kRefr = 0x3A, kAchr = 0x3B, kAcre = 0x3C;
constexpr uint8_t kActi = 0x15, kTalkActi = 0x16, kTerm = 0x17, kCont = 0x1B, kDoor = 0x1C, kFurn = 0x27;
// The form flags that mean "not in the world any more": kFlags_Deleted (a taken reference, xNVSE GameObjects.h) and the GECK "initially
// disabled" bit 0x800 which the Disable command sets on the reference. TODO-RE: confirm 0x800 with ref.info in a game (it is not in xNVSE's header);
// a wrong value only costs the Taken verdict for items the host's own player picked up first, which the claim table still catches for guests.
constexpr uint32_t kGoneFlags = 0x20u | 0x800u;

struct Raw {
    bool ok = false;
    uint8_t refType = 0, baseType = 0;
    uint32_t flags = 0, baseId = 0, cell = 0, life = 0;
    float x = 0, y = 0, z = 0;
    int count = 1;
};

bool isItemType(uint8_t t) {
    switch (t) {
        case 0x18: case 0x19: case 0x1A: case 0x1D: case 0x1F: case 0x28: case 0x29: case 0x2E: case 0x2F: case 0x31: case 0x67: case 0x6C: case 0x73: case 0x74:
            return true;   // ARMO BOOK CLOT INGR MISC WEAP AMMO KEYM ALCH NOTE IMOD CHIP CARD CARM (the xNVSE "inv object" form types)
        default:
            return false;
    }
}

bool readRawSEH(void* form, Raw* out) {
    __try {
        TESForm* f = static_cast<TESForm*>(form);
        out->refType = f->typeID;
        if (out->refType != kRefr && out->refType != kAchr && out->refType != kAcre) return true;   // ok stays false: not a reference
        TESObjectREFR* r = static_cast<TESObjectREFR*>(form);
        out->flags = r->flags;
        out->x = r->posX; out->y = r->posY; out->z = r->posZ;
        out->cell = r->parentCell ? r->parentCell->refID : 0;
        TESForm* b = r->baseForm;
        if (b) { out->baseType = b->typeID; out->baseId = b->refID; }
        if (out->refType != kRefr) out->life = *reinterpret_cast<uint32_t*>(reinterpret_cast<char*>(form) + 0x108);   // Actor life state: 0 alive, 1 dying, 2 dead
        if (isItemType(out->baseType)) {
            BSExtraData* xd = r->extraDataList.GetByType(kExtraData_Count);
            if (xd) { int c = static_cast<ExtraCount*>(xd)->count; if (c > 0) out->count = c; }
        }
        out->ok = true;
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

TargetClass classify(const Raw& r) {
    if (r.refType == kAchr) return TargetClass::Npc;
    if (r.refType == kAcre) return TargetClass::Creature;
    if (isItemType(r.baseType)) return TargetClass::Item;
    switch (r.baseType) {
        case kDoor: return TargetClass::Door;
        case kActi: case kTalkActi: return TargetClass::Activator;
        case kTerm: return TargetClass::Terminal;
        case kCont: return TargetClass::Container;
        case kFurn: return TargetClass::Furniture;
        case 0x20: case 0x21: case 0x22: return TargetClass::Static;   // STAT, SCOL, MSTT
        default: return TargetClass::Unknown;
    }
}

bool raiseSEH(void* object, void* xlist, uint32_t mask, bool* ret) {
    __try {
        *ret = reinterpret_cast<bool(__cdecl*)(void*, void*, uint32_t)>(addr::MarkScriptEvent.addr)(object, xlist, mask);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

bool voidCallSEH(unsigned addr) {
    __try {
        reinterpret_cast<void(__cdecl*)()>(addr)();
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

bool queueMessageSEH(const char* text) {
    __try {
        reinterpret_cast<bool(__cdecl*)(const char*, uint32_t, const char*, const char*, float, bool)>(addr::QueueUIMessage.addr)(text, 0, nullptr, nullptr, 2.0f, false);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

bool topMenuSEH(uint32_t* id) {
    __try {
        void* mgr = *reinterpret_cast<void**>(addr::InterfaceManagerPtr);
        if (!mgr) { *id = 0; return true; }
        *id = reinterpret_cast<uint32_t(__thiscall*)(void*)>(addr::TopMenuId.addr)(mgr);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

bool infoRunSEH(void* info, uint32_t onEnd, void* npc) {
    __try {
        reinterpret_cast<void(__thiscall*)(void*, uint32_t, void*)>(nvhooks::addr::TopicInfoRunResult.target)(info, onEnd, npc);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

// ---- terminals -----------------------------------------------------------------------------------------------------------------------
// BGSTerminal::menuEntries is a tList<TerminalEntry> at +0x98 (xNVSE GameForms.h); an entry that opens a sub menu names the BGSTerminal in +0x70.

BGSTerminal* terminalOfRef(void* ref) {
    TESObjectREFR* r = static_cast<TESObjectREFR*>(ref);
    TESForm* b = r ? r->baseForm : nullptr;
    return b && b->typeID == kTerm ? static_cast<BGSTerminal*>(b) : nullptr;
}

void* entryAtPathSEH(void* terminalRef, const uint8_t* path, size_t n) {
    __try {
        BGSTerminal* t = terminalOfRef(terminalRef);
        for (size_t i = 0; t && i < n; ++i) {
            ListNode<TerminalEntry>* node = &t->menuEntries.m_listHead;
            for (uint8_t k = 0; node && k < path[i]; ++k) node = node->next;
            TerminalEntry* e = node ? node->data : nullptr;
            if (!e) return nullptr;
            if (i + 1 == n) return e;
            t = e->subMenu;
        }
        return nullptr;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return nullptr;
    }
}

bool runEntrySEH(void* entry, void* terminalRef) {
    __try {
        reinterpret_cast<void(__thiscall*)(void*, void*)>(addr::TerminalEntryRun.target)(entry, terminalRef);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

// Depth-first search for `entry` below terminal `t`; writes the index path. Bounded: depth 8, 512 nodes.
bool findPathSEH(BGSTerminal* t, void* entry, uint8_t* path, int depth, int* budget) {
    __try {
        if (!t || depth >= 8) return false;
        uint8_t idx = 0;
        for (ListNode<TerminalEntry>* node = &t->menuEntries.m_listHead; node && node->data; node = node->next, ++idx) {
            if (--*budget < 0) return false;
            path[depth] = idx;
            if (node->data == entry) { path[depth + 1] = 0xFF; return true; }
            if (node->data->subMenu && findPathSEH(node->data->subMenu, entry, path, depth + 1, budget)) return true;
        }
        return false;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

// ---- the acting scope ----------------------------------------------------------------------------------------------------------------

class GameScope : public IActingScope {
public:
    GameScope(nvhooks::CaptureHub& hub, uint32_t actor, const Cause& c) : hub_(hub), acting_(actor), cap_(hub, c) {}
    bool record(const fnvcoop::Effect& e) override { return hub_.record(e); }

private:
    nvhooks::CaptureHub& hub_;
    nvhooks::ActingPlayerScope acting_;   // destroyed after cap_: the txn is emitted while the guest is still `player`
    nvhooks::ScopedCapture cap_;
};

class GameEngine : public IIntentEngine {
public:
    bool describe(const FormRef& target, TargetInfo& out) override {
        out = TargetInfo();
        uint32_t id = nvhooks::resolver().fromRef(target);
        if (!id) return false;
        out.ref = id;
        void* f = nvhooks::resolver().lookup(id);
        if (!f) return true;   // the plugin is loaded, the reference is not in memory
        Raw r;
        if (!readRawSEH(f, &r) || !r.ok) return true;
        out.cls = classify(r);
        out.loaded = r.cell != 0;   // a reference in no cell is not in the world
        out.disabled = (r.flags & kGoneFlags) != 0;
        out.dead = (r.refType == kAchr || r.refType == kAcre) && (r.life == 1 || r.life == 2);
        out.inCombat = false;       // TODO-RE: no verified combat flag; DialogueStart checks death only
        out.hasPos = true;
        out.pos = {r.x, r.y, r.z};
        out.cell = r.cell;
        if (out.cls == TargetClass::Item) {
            nvhooks::resolver().toRef(r.baseId, &out.base);
            out.count = r.count;
        }
        return true;
    }

    bool position(RefId ref, Vec3& pos, uint32_t* cell) override {
        void* f = nvhooks::resolver().lookup(ref);
        if (!f) return false;
        Raw r;
        if (!readRawSEH(f, &r) || !r.ok) return false;
        pos = {r.x, r.y, r.z};
        if (cell) *cell = r.cell;
        return true;
    }

    std::unique_ptr<IActingScope> beginActing(RefId actor, PlayerId slot, const Cause& cause) override {
        nvhooks::CaptureHub* hub = nvhooks::hub();
        if (!hub || !actor) return nullptr;
        // The host's own player (the stand-in) is NOT registered: the slot table maps a ref to a slot for good, and 0x14 must stay slot 0.
        // TODO: while a stand-in acts, nvhooks::actingSlot() therefore reads 0 (CompanionOwner.owner of a companion a guest recruits that way).
        if (actor != 0x14) nvhooks::registerPlayerSlot(actor, static_cast<uint8_t>(slot));
        return std::unique_ptr<IActingScope>(new GameScope(*hub, actor, cause));
    }

    bool activate(RefId actor, const FormRef& target) override {
        uint32_t id = nvhooks::resolver().fromRef(target);
        if (!id || !actor) return false;
        char line[64];
        std::snprintf(line, sizeof line, "Activate %08X 1", actor);
        return nvhooks::execConsoleLineOn(line, id);
    }

    bool raiseEvent(RefId actor, const FormRef& target, uint32_t mask) override {
        if (!pinsOk()) return false;
        void* obj = nvhooks::resolver().lookup(actor);
        uint32_t id = nvhooks::resolver().fromRef(target);
        TESObjectREFR* t = static_cast<TESObjectREFR*>(id ? nvhooks::resolver().lookup(id) : nullptr);
        if (!obj || !t) return false;
        bool ret = false;
        return raiseSEH(obj, &t->extraDataList, mask, &ret);
    }

    bool runTerminalEntry(RefId, const FormRef& terminal, const std::vector<uint8_t>& path, bool* supported) override {
        uint32_t id = nvhooks::resolver().fromRef(terminal);
        void* ref = id ? nvhooks::resolver().lookup(id) : nullptr;
        if (supported) *supported = true;
        if (!ref || path.empty()) return false;
        void* entry = entryAtPathSEH(ref, path.data(), path.size());
        if (!entry) return false;
        return runEntrySEH(entry, ref);   // the acting scope around us is what makes `player` the guest inside the result script
    }

    bool runInfoResult(RefId actor, PlayerId slot, const FormRef& info, const FormRef& npc, bool onEnd, bool* supported) override {
        // L5 hooks this function (TESTopicInfo::RunResultScript, pinned in plugin/hooks/engine_addrs.h) and captures it under a DialogueResult scope on the
        // host. Only call it when that hook is installed: without it nothing would capture the effects and we would run unverified code for nothing.
        if (supported) *supported = true;
        if (!nvhooks::trampolineOfAny(nvhooks::addr::TopicInfoRunResult.target)) { if (supported) *supported = false; return false; }
        uint32_t infoId = nvhooks::resolver().fromRef(info), npcId = nvhooks::resolver().fromRef(npc);
        void* infoForm = infoId ? nvhooks::resolver().lookup(infoId) : nullptr;
        void* npcRef = npcId ? nvhooks::resolver().lookup(npcId) : nullptr;
        if (!infoForm || !actor) return false;
        if (actor != 0x14) nvhooks::registerPlayerSlot(actor, static_cast<uint8_t>(slot));
        nvhooks::ActingPlayerScope acting(actor);
        return infoRunSEH(infoForm, onEnd ? 1u : 0u, npcRef);
    }

    bool disableRef(const FormRef& target) override {
        uint32_t id = nvhooks::resolver().fromRef(target);
        return id && nvhooks::execConsoleLineOn("Disable", id);   // the engine's own hook records the DisableRef effect into the open scope
    }

    void closeMenus() override { if (pinsOk()) voidCallSEH(addr::CloseAllMenusCore.addr); }
    void notify(const std::string& text) override { if (pinsOk()) queueMessageSEH(text.c_str()); }
    bool openContainer(const FormRef& container) override {
        uint32_t id = nvhooks::resolver().fromRef(container);
        return id && nvhooks::execConsoleLineOn("Activate 14 1", id);   // the console line runs inside a script: the guest observers stay out of it
    }
};

// Executable memory outside FalloutNV.exe's image: another xNVSE plugin's detour or trampoline.
bool foreignExec(uintptr_t a) {
    auto base = reinterpret_cast<uintptr_t>(GetModuleHandleA(nullptr));
    auto nt = reinterpret_cast<const IMAGE_NT_HEADERS*>(base + reinterpret_cast<const IMAGE_DOS_HEADER*>(base)->e_lfanew);
    if (a >= base && a < base + nt->OptionalHeader.SizeOfImage) return false;
    MEMORY_BASIC_INFORMATION mbi{};
    if (!VirtualQuery(reinterpret_cast<const void*>(a), &mbi, sizeof mbi) || mbi.State != MEM_COMMIT) return false;
    return (mbi.Protect & (PAGE_EXECUTE | PAGE_EXECUTE_READ | PAGE_EXECUTE_READWRITE | PAGE_EXECUTE_WRITECOPY)) != 0;
}

// We only CALL these functions, so another plugin's 5-byte JMP over the entry is fine: the game's own callers go through
// it too (found under Tale of Two Wastelands: E9 <detour> at MarkScriptEvent). Every byte after the jump must still match
// and the jump must leave the exe image; anything else means another exe build.
bool bytesMatch(unsigned addr, const unsigned char* want, unsigned char* got) {
    __try {
        std::memcpy(got, reinterpret_cast<const void*>(addr), 16);
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
    if (std::memcmp(got, want, 16) == 0) return true;
    if (got[0] != 0xE9 || std::memcmp(got + 5, want + 5, 11) != 0) return false;
    int32_t rel;
    std::memcpy(&rel, got + 1, 4);
    return foreignExec(static_cast<uintptr_t>(addr) + 5 + rel);
}

}  // namespace

IIntentEngine& engine() {
    static GameEngine e;
    return e;
}

bool pinsOk() {
    static int state = 0;   // 0 unknown, 1 ok, 2 mismatch
    if (state == 0) {
        bool ok = true;
        for (const addr::CallSpec* c : addr::kAllCalls) {
            unsigned char got[16] = {};
            bool m = bytesMatch(c->addr, c->bytes, got);
            if (m && got[0] == 0xE9 && c->bytes[0] != 0xE9)
                nvcoop::Log("intents: %s at %08X is detoured by another plugin; calling through its jump", c->name, c->addr);
            if (!m) {
                char hex[16 * 3 + 1] = {};
                for (int i = 0; i < 16; ++i) std::snprintf(hex + i * 3, 4, "%02X ", got[i]);
                nvcoop::Log("intents: pin MISMATCH for %s at %08X (a different FalloutNV.exe?): that engine path stays off; found %s",
                            c->name, c->addr, hex);
            }
            ok = ok && m;
        }
        state = ok ? 1 : 2;
    }
    return state == 1;
}

bool containerMenuOpen() {
    if (!pinsOk()) return false;
    uint32_t id = 0;
    return topMenuSEH(&id) && id == addr::kContainerMenu;
}

bool terminalPathOf(uint32_t terminalRefId, void* entry, std::vector<uint8_t>* path) {
    void* ref = nvhooks::resolver().lookup(terminalRefId);
    BGSTerminal* t = ref ? terminalOfRef(ref) : nullptr;
    if (!t || !entry) return false;
    uint8_t buf[10] = {};
    int budget = 512;
    if (!findPathSEH(t, entry, buf, 0, &budget)) return false;
    path->clear();
    for (int i = 0; i < 8 && buf[i] != 0xFF; ++i) {
        path->push_back(buf[i]);
        if (buf[i + 1] == 0xFF) break;
    }
    return !path->empty();
}

uint32_t refIdOf(void* form) {
    __try {
        return form ? static_cast<TESForm*>(form)->refID : 0;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return 0;
    }
}

}  // namespace nvcoop::intents::game
