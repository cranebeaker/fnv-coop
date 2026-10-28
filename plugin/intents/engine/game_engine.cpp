// IIntentEngine over xNVSE (see game_engine.h). Style follows plugin/hooks_world/world_hooks.cpp: plain structs read under SEH, no C++ objects
// inside __try, console lines for what the engine only exposes as script commands.
#include "intents/engine/game_engine.h"

#include <Windows.h>

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
constexpr uint8_t kNoteForm = 0x31;   // BGSNote (the xNVSE "inv object" table in isItemType lists it as NOTE)
constexpr uint8_t kActi = 0x15, kTalkActi = 0x16, kTerm = 0x17, kCont = 0x1B, kDoor = 0x1C, kFurn = 0x27;
// The form flags that mean "not in the world any more": kFlags_Deleted (a taken reference, xNVSE GameObjects.h) and the GECK "initially
// disabled" bit 0x800 which the Disable command sets on the reference. TODO-RE: confirm 0x800 with ref.info in a game (it is not in xNVSE's header);
// a wrong value only costs the Taken verdict for items the host's own player picked up first, which the claim table still catches for guests.
constexpr uint32_t kGoneFlags = 0x20u | 0x800u;

struct Raw {
    bool ok = false;
    uint8_t refType = 0, baseType = 0;
    uint32_t flags = 0, baseId = 0, cell = 0, life = 0, scriptId = 0;
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
        if (out->refType == kRefr && !isItemType(out->baseType)) {   // the script of a scripted object: its own instance, else the one on the base form (retry_list.h is keyed by it)
            Script* sc = nullptr;
            if (ScriptEventList* el = r->GetEventList()) sc = el->m_script;
            if (!sc && b) if (TESScriptableForm* sf = DYNAMIC_CAST(b, TESForm, TESScriptableForm)) sc = sf->script;
            if (sc) out->scriptId = sc->refID;
        }
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

// The reference's own script instance (ExtraScript: script + event list), run now. Script::Execute @0x5AC1E0 is the call the engine's update makes for a scripted
// reference (internal lane notes "implicit weak scopes"); the run consumes the marks of the event list.
bool runRefScriptSEH(void* ref) {
    __try {
        TESObjectREFR* r = static_cast<TESObjectREFR*>(ref);
        auto* xs = static_cast<ExtraScript*>(r->extraDataList.GetByType(kExtraData_Script));
        if (!xs || !xs->script || !xs->eventList) return false;
        CALL_MEMBER_FN(xs->script, Execute)(r, xs->eventList, nullptr, false);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

// Is a mark of `mask` for the form `object` waiting in the reference's script event list?
bool eventPendingSEH(void* ref, void* object, uint32_t mask) {
    __try {
        TESObjectREFR* r = static_cast<TESObjectREFR*>(ref);
        auto* xs = static_cast<ExtraScript*>(r->extraDataList.GetByType(kExtraData_Script));
        if (!xs || !xs->eventList || !xs->eventList->m_eventList) return false;
        int budget = 256;
        for (auto it = xs->eventList->m_eventList->Begin(); !it.End() && budget-- > 0; ++it)
            if (it.Get() && (!object || it.Get()->object == object) && (it.Get()->eventMask & mask)) return true;
        return false;
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

// ---- quests --------------------------------------------------------------------------------------------------------------------------

constexpr uint8_t kQuestForm = 0x47;   // kFormType_TESQuest (xNVSE GameForms.h, TESIdleForm is 0x48)

bool questStageSEH(void* form, int32_t* stage) {
    __try {
        TESForm* f = static_cast<TESForm*>(form);
        if (!f || f->typeID != kQuestForm) return false;
        *stage = static_cast<TESQuest*>(form)->currentStage;
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

// ---- container contents (gear stashes) -------------------------------------------------------------------------------------------------------------
// ExtraContainerChanges entries: item refID -> countDelta relative to the base form's own inventory. The shared gear containers have an EMPTY base inventory
// (nvcoop_intents.json lists only those), so the deltas are the whole contents.

struct RawItem { uint32_t id; int32_t n; };
constexpr size_t kMaxStoredLines = 512;

bool readContainerSEH(void* ref, RawItem* out, size_t* n) {
    *n = 0;
    __try {
        TESObjectREFR* r = static_cast<TESObjectREFR*>(ref);
        auto* xc = static_cast<ExtraContainerChanges*>(r->extraDataList.GetByType(kExtraData_ContainerChanges));
        ExtraContainerChanges::EntryDataList* list = xc ? xc->GetEntryDataList() : nullptr;
        if (!list) return true;   // never touched: empty
        for (auto it = list->Begin(); !it.End(); ++it) {
            if (!it.Get() || !it.Get()->type) continue;
            if (it.Get()->countDelta == 0) continue;
            if (*n >= kMaxStoredLines) return false;   // more lines than the ledger keeps: refuse rather than silently drop gear
            out[(*n)++] = {it.Get()->type->refID, static_cast<int32_t>(it.Get()->countDelta)};
        }
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

// Mutes the capture hub for a scope: moving gear between the ledger and a container is not a change of the world.
struct HubMute {
    nvhooks::CaptureHub* hub;
    HubMute() : hub(nvhooks::hub()) { if (hub) hub->pushMute(); }
    ~HubMute() { if (hub) hub->popMute(); }
};

// ---- the acting scope ----------------------------------------------------------------------------------------------------------------

class GameScope : public IActingScope {
public:
    GameScope(nvhooks::CaptureHub& hub, uint32_t actor, PlayerId slot, const Cause& c) : hub_(hub), acting_(actor, static_cast<int>(slot)), cap_(hub, c) {}
    bool record(const fnvcoop::Effect& e) override { return hub_.record(e); }
    uint64_t recorded() const override { return hub_.stats().recorded; }

private:
    nvhooks::CaptureHub& hub_;
    nvhooks::ActingPlayerScope acting_;   // destroyed after cap_: the txn is emitted while the guest is still `player`
    nvhooks::ScopedCapture cap_;
};

class GameEngine : public IIntentEngine {
    std::vector<RawItem> hostMark_;
    bool hostMarkValid_ = false;
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
        out.resident = true;
        out.loaded = r.cell != 0;   // a reference in no cell is not in the world
        out.disabled = (r.flags & kGoneFlags) != 0;
        out.dead = (r.refType == kAchr || r.refType == kAcre) && (r.life == 1 || r.life == 2);
        out.inCombat = false;       // TODO-RE: no verified combat flag; DialogueStart checks death only
        out.hasPos = true;
        out.pos = {r.x, r.y, r.z};
        out.cell = r.cell;
        if (r.scriptId) nvhooks::resolver().toRef(r.scriptId, &out.script);
        if (out.cls == TargetClass::Item) {
            nvhooks::resolver().toRef(r.baseId, &out.base);
            out.count = r.count;
            out.note = r.baseType == kNoteForm;
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
        return std::unique_ptr<IActingScope>(new GameScope(*hub, actor, slot, cause));
    }

    bool activate(RefId actor, const FormRef& target) override {
        uint32_t id = nvhooks::resolver().fromRef(target);
        if (!id || !actor) return false;
        char line[64];
        std::snprintf(line, sizeof line, "Activate %08X 1", actor);
        return nvhooks::execConsoleLineOn(line, id);
    }

    bool runScript(const FormRef& target) override {
        uint32_t id = nvhooks::resolver().fromRef(target);
        void* t = id ? nvhooks::resolver().lookup(id) : nullptr;
        return t && runRefScriptSEH(t);
    }

    bool eventPending(RefId object, const FormRef& target, uint32_t mask) override {
        uint32_t id = nvhooks::resolver().fromRef(target);
        void* t = id ? nvhooks::resolver().lookup(id) : nullptr;
        void* o = object ? nvhooks::resolver().lookup(object) : nullptr;
        return t && (o || !object) && eventPendingSEH(t, o, mask);
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
        nvhooks::ActingPlayerScope acting(actor, static_cast<int>(slot));   // a guest with no puppet is acted for by the host player (0x14): the slot says whose window this is
        return infoRunSEH(infoForm, onEnd ? 1u : 0u, npcRef);
    }

    bool disableRef(const FormRef& target) override {
        uint32_t id = nvhooks::resolver().fromRef(target);
        return id && nvhooks::execConsoleLineOn("Disable", id);   // the engine's own hook records the DisableRef effect into the open scope
    }

    bool questStage(const FormRef& quest, int32_t* stage) override {
        uint32_t id = nvhooks::resolver().fromRef(quest);
        void* f = id ? nvhooks::resolver().lookup(id) : nullptr;
        return f && stage && questStageSEH(f, stage);
    }

    StageOutcome setStage(const FormRef& quest, uint32_t stage) override {
        uint32_t id = nvhooks::resolver().fromRef(quest);
        if (!id || stage == 0 || stage > 255) return StageOutcome::Failed;
        // The console command reaches the engine's SetStage through our own detour: the stage gate of the rules engine is asked (it may defer), the
        // stage is recorded on entry into the open acting scope, and its result scripts run captured. NOT under a StageGateBypass: the host keeps the say.
        const uint64_t held = nvhooks::stageGateDeferred();
        char line[64];
        std::snprintf(line, sizeof line, "setstage %08X %u", id, stage);
        const bool ran = nvhooks::execConsoleLine(line);
        if (nvhooks::stageGateDeferred() != held) return StageOutcome::Deferred;
        int32_t now = 0;
        void* f = nvhooks::resolver().lookup(id);
        return ran && f && questStageSEH(f, &now) && now >= static_cast<int32_t>(stage) ? StageOutcome::Applied : StageOutcome::Failed;
    }

    bool addNote(const FormRef& note) override {
        uint32_t id = nvhooks::resolver().fromRef(note);
        if (!id) return false;
        char line[64];
        std::snprintf(line, sizeof line, "AddNote %08X", id);   // the same line the remote applier uses; the engine's AddNote hook records the effect into the open scope
        return nvhooks::execConsoleLine(line);
    }

    bool readItems(const FormRef& container, std::vector<StoredItem>* out) override {
        uint32_t id = nvhooks::resolver().fromRef(container);
        void* ref = id ? nvhooks::resolver().lookup(id) : nullptr;
        if (!ref || !out) return false;
        RawItem raw[kMaxStoredLines];
        size_t n = 0;
        if (!readContainerSEH(ref, raw, &n)) return false;
        out->clear();
        for (size_t i = 0; i < n; ++i) {
            FormRef item;
            if (raw[i].n <= 0) continue;
            if (!nvhooks::resolver().toRef(raw[i].id, &item)) return false;   // a runtime-created item: cannot be named in the ledger, so do not touch the container
            out->push_back({item, raw[i].n});
        }
        return true;
    }

    bool replaceItems(const FormRef& container, const std::vector<StoredItem>& items) override {
        uint32_t id = nvhooks::resolver().fromRef(container);
        if (!id || !nvhooks::resolver().lookup(id)) return false;
        HubMute mute;
        if (!nvhooks::execConsoleLineOn("RemoveAllItems", id)) return false;
        char line[64];
        bool ok = true;
        for (const StoredItem& i : items) {
            uint32_t item = nvhooks::resolver().fromRef(i.item);
            if (!item || i.count <= 0) { ok = false; continue; }
            std::snprintf(line, sizeof line, "AddItem %08X %d 1", item, i.count);
            ok = nvhooks::execConsoleLineOn(line, id) && ok;
        }
        return ok;
    }

    // The host's own player stands in for a guest: what it gains meanwhile is the guest's (see IIntentEngine::markHostInventory).
    bool markHostInventory() override {
        void* ref = nvhooks::resolver().lookup(0x14);
        hostMarkValid_ = false;
        if (!ref) return false;
        RawItem raw[kMaxStoredLines];
        size_t n = 0;
        if (!readContainerSEH(ref, raw, &n)) return false;
        hostMark_.assign(raw, raw + n);
        hostMarkValid_ = true;
        return true;
    }
    int revertHostInventory() override {
        void* ref = nvhooks::resolver().lookup(0x14);
        if (!hostMarkValid_ || !ref) return 0;
        hostMarkValid_ = false;
        RawItem raw[kMaxStoredLines];
        size_t n = 0;
        if (!readContainerSEH(ref, raw, &n)) return 0;
        HubMute mute;
        int taken = 0;
        for (size_t i = 0; i < n; ++i) {
            int32_t was = 0;
            for (const RawItem& m : hostMark_) if (m.id == raw[i].id) { was = m.n; break; }
            if (raw[i].n <= was) continue;
            char line[64];
            std::snprintf(line, sizeof line, "RemoveItem %08X %d 1", raw[i].id, raw[i].n - was);
            if (nvhooks::execConsoleLineOn(line, 0x14)) taken += raw[i].n - was;
        }
        return taken;
    }

    void closeMenus() override { if (pinsOk()) voidCallSEH(addr::CloseAllMenusCore.addr); }
    void notify(const std::string& text) override { if (pinsOk()) queueMessageSEH(text.c_str()); }
    bool openContainer(const FormRef& container) override {
        uint32_t id = nvhooks::resolver().fromRef(container);
        return id && nvhooks::execConsoleLineOn("Activate 14 1", id);   // the console line runs inside a script: the guest observers stay out of it
    }
};

bool bytesMatch(unsigned addr, const unsigned char* want) {
    unsigned char got[16] = {};
    __try {
        std::memcpy(got, reinterpret_cast<const void*>(addr), 16);
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
    return std::memcmp(got, want, 16) == 0;
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
            bool m = bytesMatch(c->addr, c->bytes);
            if (!m) nvcoop::Log("intents: pin MISMATCH for %s at %08X (a different FalloutNV.exe?): that engine path stays off", c->name, c->addr);
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
