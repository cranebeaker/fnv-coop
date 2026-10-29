// Live IEngineApi for FalloutNV.exe 1.4.0.525 (x86) built against xNVSE headers (L8 skeleton).
//
// STATUS: compiles; NOT loaded in a game yet. Each method is tagged:
//   [live]  implemented from xNVSE/JIP-documented addresses and layouts
//   [stub]  needs an L5-verified address/hook; reports itself through unsupported() so the first in-game run
//           lists exactly what is missing (never silently pretends to work)
// Subtitle strategy for spectators: v1 uses xNVSE's QueueUIMessage (HUD message queue, address 0x7052F0 in the
// xNVSE source) which is available today; the vanilla Subtitles tile (HUDMainMenu::subtitlesArr, JIP GameUI.h
// 0x198) is the better target once an add-subtitle routine is verified.
#include <windows.h>
#include <cstdio>
#include <cstring>
#include <string>

#include "nvse/GameAPI.h"
#include "nvse/GameData.h"
#include "nvse/GameForms.h"
#include "nvse/GameObjects.h"

#include "engine_xnvse.h"
#include "glue.h"
#include "voice_token.h"
#include "ui_addrs.h"   // pinned prologues of the engine functions called below
#include "ui_hooks.h"   // L11 hooks_ui: nvui::ops (real message boxes, vanilla menus, CloseAllMenus core, StartConversation)

namespace fndlg {
namespace {

// ReadProcessMemory on ourselves: a bad pointer returns false instead of faulting the game.
struct ProcessMemory : IMemory {
    bool read(Addr32 a, void* out, size_t n) const override {
        SIZE_T got = 0;
        return a >= 0x10000 && ReadProcessMemory(GetCurrentProcess(), reinterpret_cast<LPCVOID>(uintptr_t(a)), out, n, &got) && got == n;
    }
};

std::string PluginName(UInt32 formId) {
    DataHandler* dh = DataHandler::Get();
    UInt32 idx = formId >> 24;
    if (dh && idx < dh->modList.loadedModCount && dh->modList.loadedMods[idx]) {
        std::string s = dh->modList.loadedMods[idx]->name;
        for (auto& c : s) c = char(tolower(static_cast<unsigned char>(c)));
        return s;
    }
    return std::string();
}

// (plugin file, local id) -> runtime refID through the load order; 0 if the plugin is not loaded
uint32_t RuntimeRefId(const FormRef& r) {
    DataHandler* dh = DataHandler::Get();
    if (!dh) return 0;
    for (UInt32 i = 0; i < dh->modList.loadedModCount && i < 0xFF; ++i) {
        ModInfo* mi = dh->modList.loadedMods[i];
        if (!mi) continue;
        std::string n = mi->name;
        for (auto& c : n) c = char(tolower(static_cast<unsigned char>(c)));
        if (n == r.plugin) return (uint32_t(i) << 24) | (r.localId & 0x00FFFFFF);
    }
    return 0;
}

// ---- engine calls (each in its own function: SEH cannot share a frame with C++ objects). A fault answers false instead of crashing.
const fnvcoop_dlg::FactTable* g_facts = nullptr;
constexpr uint32_t kPlayerRef = 0x14;
constexpr uint32_t kActorValueOwnerOffset = 0xA4;   // Actor+0xA4 = ActorValueOwner (docs/engine-addresses.md: vtbl[3] = GetActorValue 0x66EF50)

uint32_t LookupForm(uint32_t refID) {
    __try {
        return reinterpret_cast<uint32_t>(reinterpret_cast<void*(__cdecl*)(uint32_t)>(nvui::addr::LookupFormByID.addr)(refID));
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return 0;
    }
}
bool CallLoadResponses(uint32_t info) {   // TESTopicInfo::GetResponses(info, 0): the chain lands in the engine cache cell
    __try {
        reinterpret_cast<void*(__thiscall*)(void*, void*)>(nvui::addr::DialogGetResponses.addr)(reinterpret_cast<void*>(uintptr_t(info)), nullptr);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool CallMarkSaidRaw(uint32_t info) {
    __try {
        reinterpret_cast<void(__thiscall*)(void*)>(nvui::addr::InfoMarkSaid.target)(reinterpret_cast<void*>(uintptr_t(info)));
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool CallMarkSaid(uint32_t info) {
    nvui::SaidMute mute;   // our own mark (the address is the hooked entry): not an engine mark to report
    return CallMarkSaidRaw(info);
}
bool WriteByte(uint32_t addr, uint8_t v) {
    __try {
        *reinterpret_cast<volatile uint8_t*>(uintptr_t(addr)) = v;
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool CallCondition(uint32_t fn, uint32_t thisObj, uint32_t p1, double* out) {   // cdecl (thisObj, p1, p2, double*)
    __try {
        *out = 0;
        return reinterpret_cast<bool(__cdecl*)(void*, void*, void*, double*)>(uintptr_t(fn))(
            reinterpret_cast<void*>(uintptr_t(thisObj)), reinterpret_cast<void*>(uintptr_t(p1)), nullptr, out);
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool ReadActorValue(uint32_t player, uint32_t code, float* out) {   // the player is always loaded: the direct read is exact
    __try {
        *out = reinterpret_cast<float(__thiscall*)(void*, uint32_t)>(0x66EF50)(reinterpret_cast<void*>(uintptr_t(player + kActorValueOwnerOffset)), code);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
bool ReadItemCount(uint32_t player, uint32_t item, int* out) {
    __try {
        void* data = reinterpret_cast<void*(__cdecl*)(void*)>(nvui::addr::ContainerChangesOf.addr)(reinterpret_cast<void*>(uintptr_t(player)));
        if (!data) return false;
        *out = reinterpret_cast<int(__thiscall*)(void*, void*)>(nvui::addr::ContainerChangesCount.addr)(data, reinterpret_cast<void*>(uintptr_t(item)));
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
// The game's own subtitle line at the bottom of the screen (HUDMainMenu's subtitle list, JIP GameUI.h 0x198). The engine adds a line with
// 0x705210, cdecl (const char* text, BSSoundHandle sound, NiPoint3 pos, TESObjectREFR* speaker, bool top), the call its Say / dialogue code
// makes (11 call sites); it hands the line to HUDMainMenu (singleton 0x11D96C0) 0x774FD0, which copies 0x104 characters and takes the display
// time from the sound (none here: from the text length). It adds nothing while the player's General Subtitles option is off unless the speaker
// has the "always show subtitles" flag (Actor+0x140 bit 30): the option byte (Setting bGeneralSubtitles:GamePlay 0x11D8928, value at +4) is
// set for the call and put back. Only the add reads it (its other readers are the settings menus), so a line once added stays on screen.
constexpr uint32_t kQueueSubtitle = 0x00705210;
constexpr uint8_t kQueueSubtitlePrologue[11] = {0x55, 0x8B, 0xEC, 0x6A, 0xFF, 0x68, 0x90, 0x79, 0xF0, 0x00, 0x64};
constexpr uint32_t kGeneralSubtitlesValue = 0x011D892C;
struct SoundHandle { uint32_t id = 0xFFFFFFFF; uint8_t b4 = 0, pad[3] = {}; uint32_t state = 0; };   // BSSoundHandle, no sound
struct Point3 { float x = 0, y = 0, z = 0; };
using QueueSubtitleFn = void(__cdecl*)(const char*, SoundHandle, Point3, void*, bool);
bool CallQueueSubtitle(const char* text, uint32_t speaker, Point3 pos) {
    static const bool pinOk = std::memcmp(reinterpret_cast<const void*>(uintptr_t(kQueueSubtitle)), kQueueSubtitlePrologue, sizeof kQueueSubtitlePrologue) == 0;
    if (!pinOk) return false;
    volatile uint8_t* option = reinterpret_cast<volatile uint8_t*>(uintptr_t(kGeneralSubtitlesValue));
    const uint8_t was = *option;
    __try {
        *option = 1;
        reinterpret_cast<QueueSubtitleFn>(uintptr_t(kQueueSubtitle))(text, SoundHandle{}, pos, reinterpret_cast<void*>(uintptr_t(speaker)), false);
        *option = was;
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        *option = was;
        return false;
    }
}
bool RefPos(uint32_t ref, Point3* out) {   // TESObjectREFR posX/Y/Z
    __try {
        const TESObjectREFR* r = reinterpret_cast<const TESObjectREFR*>(uintptr_t(ref));
        *out = Point3{r->posX, r->posY, r->posZ};
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
// A spectator's line at the bottom like any line the NPC says. False: not shown there (the caller falls back to the HUD message queue).
bool ShowBottomSubtitle(const FormRef& npc, const std::string& text) {
    const uint32_t speaker = LookupForm(RuntimeRefId(npc));
    const uint32_t who = speaker ? speaker : LookupForm(kPlayerRef);
    Point3 pos;
    if (!who || !RefPos(who, &pos)) return false;
    return CallQueueSubtitle(text.substr(0, 0x103).c_str(), speaker, pos);
}

uint32_t PeekU32(uint32_t a) {
    __try {
        return *reinterpret_cast<volatile uint32_t*>(uintptr_t(a));
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return 0;
    }
}
// One fact for the local player. Actor values and item counts use the engine primitives (the condition functions for these two are
// party-reduced by plugin/hooks once a session runs, which would answer for the party instead of this player); perks and equipment use the
// condition functions, which no hook touches.
bool EvalFact(const fnvcoop_dlg::FactSpec& s, int32_t* value) {
    const uint32_t player = LookupForm(kPlayerRef);
    if (!player) return false;
    using fnvcoop_dlg::FactKind;
    if (s.kind == FactKind::ActorValue) {
        float f = 0;
        if (!ReadActorValue(player, s.what.localId, &f)) return false;
        *value = int32_t(f);
        return true;
    }
    const uint32_t form = LookupForm(RuntimeRefId(s.what));
    if (!form) return false;
    if (s.kind == FactKind::ItemCount) {
        int n = 0;
        if (!ReadItemCount(player, form, &n)) return false;
        *value = n;
        return true;
    }
    double d = 0;
    const uint32_t fn = s.kind == FactKind::HasPerk ? nvui::addr::CondHasPerk.addr : nvui::addr::CondGetEquipped.addr;
    if (!CallCondition(fn, player, form, &d)) return false;
    *value = d != 0 ? 1 : 0;
    return true;
}

}  // namespace

void SetFactTable(const fnvcoop_dlg::FactTable* t) { g_facts = t; }

// The response loader (see info_reader.h): makes `info` current in the engine's response cache. It must not run while the engine is
// mid-line on ANOTHER INFO (state playing / queued): the cache would be evicted under nodes the engine still walks.
bool LoadResponsesSafely(uint32_t info) {
    const uint32_t menu = PeekU32(dialog_menu::kSingletonPtr);
    if (menu) {
        const uint32_t state = PeekU32(menu + dialog_menu::kState), spoken = PeekU32(dialog_menu::kSpokenInfoGlobal);
        if ((state == dialog_menu::kStatePlaying || state == dialog_menu::kStateQueued) && spoken && spoken != info) return false;
    }
    return CallLoadResponses(info);
}

class XnvseEngine;
XnvseEngine* g_engine = nullptr;
void ReportUnsupported(const char* what);

// Spectator output goes through the narration gate: scene lines carry the host's show stamp ("<token>#o<ord>.<after>") and must be
// played in order with the slides; unstamped lines play at once. Subtitle = the game's own subtitle line at the bottom (the HUD message queue
// if that is refused), voice = `<npc>.Say <topic>`.
nvui::NarrationGate& Gate() {
    static nvui::NarrationGate g(nvui::showSequencer(), nvui::NarrationGate::Sink{
        [](const FormRef& npc, const std::string& text, float seconds) {
            if (ShowBottomSubtitle(npc, text)) return;
            const std::string t = text.substr(0, 0x1FF);
            QueueUIMessage(t.c_str(), 2, nullptr, nullptr, seconds, false);
        },
        [](const FormRef& npc, const std::string& token) {
            nvui::VoiceToken v;
            if (!nvui::parseVoiceToken(token, v)) {
                ReportUnsupported("playVoice: not a voice token");
                return;
            }
            if (!nvui::ops::sayTopic(RuntimeRefId(npc), v.topic)) ReportUnsupported("playVoice: npc or topic not loaded");
        }});
    return g;
}

class XnvseEngine : public IEngineApi {
public:
    using Report = void (*)(const char*);
    explicit XnvseEngine(Report r = nullptr) : report_(r) { g_engine = this; }

    // [live] process memory, DialogMenu singleton (*(DialogMenu**)0x11D9510)
    const IMemory& memory() override { return mem_; }
    Addr32 dialogMenu() override {
        Addr32 p = 0;
        return readU32(mem_, dialog_menu::kSingletonPtr, p) ? p : 0;
    }

    // [live] runtime FormID -> (plugin file, plugin-local id); dynamic (0xFF) forms have no stable name
    bool formRefOf(Addr32 form, FormRef& out) override {
        uint32_t id = 0;
        if (!form || !readU32(mem_, form + 0x0C, id)) return false;  // TESForm::refID at 0x0C
        std::string plugin = PluginName(id);
        if (plugin.empty()) return false;
        out = FormRef(plugin, id & 0x00FFFFFF);
        return true;
    }
    bool infoRefOf(Addr32 info, FormRef& out) override { return formRefOf(info, out); }

    // [live via L11, in-game proof pending] TESTopicInfo -> parent TESTopic. Vanilla has no back pointer, so L11's GetTopicInfo hook
    // remembers the topic of every INFO the engine resolves (topic lists are built through it). No editor ids at runtime: edid stays empty.
    bool topicOf(Addr32 info, FormRef& topic, std::string& edid) override {
        edid.clear();
        const uint32_t t = nvui::ops::topicOfInfo(info);
        if (!t || !formRefOf(t, topic)) {
            unsupported("topicOf: INFO never resolved through GetTopicInfo");
            return false;
        }
        return true;
    }

    // [live via L11] the "voice file" is a token naming topic + INFO (voice_token.h); playVoice turns it into `<npc>.Say <topic>`.
    std::string voicePath(Addr32 info, uint32_t responseNumber, Addr32) override {
        FormRef topic, inf;
        std::string edid;
        if (!topicOf(info, topic, edid) || !formRefOf(info, inf)) return std::string();
        return nvui::makeVoiceToken(topic, inf, responseNumber);
    }

    // [live, in-game proof pending] the facts an INFO reads about the player (data/info_facts.txt, generated from the ESMs): actor values
    // through the ActorValueOwner (0x66EF50), item counts through ExtraContainerChanges (0x4BF220 / 0x4C8F30), perks and equipment through
    // the condition functions. Without the table (file missing) the pick carries no facts and the host answers as itself.
    std::vector<fnvcoop_dlg::Fact> collectFacts(Addr32 info) override {
        if (!g_facts || g_facts->size() == 0) return {};
        FormRef ref;
        if (!formRefOf(info, ref)) return {};
        return g_facts->collect(ref, [](const fnvcoop_dlg::FactSpec& s, int32_t& v) { return EvalFact(s, &v); });
    }

    // [live] speech/skill classification from the INFO's own fields
    CheckKind classifyCheck(Addr32, const InfoFlags& f) override {
        if (f.speechChallenge) return CheckKind::Speech;
        if (f.actorValueOrPerk) return CheckKind::Skill;
        return CheckKind::None;
    }

    // [live via L11, in-game proof pending] the vanilla Activate was suppressed by the activate observer; the grant opens the DialogMenu on
    // GREETING with `<npc>.StartConversation player GREETING` (same mechanism as startForcedConversation, covered by proof_ui.py, not run yet).
    void resumeTalk(const FormRef& npc) override {
        if (!nvui::ops::resumeTalk(RuntimeRefId(npc))) unsupported("resumeTalk: npc ref not loaded");
    }

    // [live] HUD message through xNVSE QueueUIMessage
    void talkDenied(DenyReason why, PlayerId owner) override {
        char buf[128];
        std::snprintf(buf, sizeof buf, why == DenyReason::Busy ? "Busy talking to player %d." : "Cannot talk right now (%d).",
                      why == DenyReason::Busy ? int(owner) : int(why));
        QueueUIMessage(buf, 0, nullptr, nullptr, 2.0f, false);
    }

    // [needs in-game proof] the engine builds the next list itself at the end of the line, from the initiator's local state; the
    // end-script gate only decides when the glue reports TopicsReady. Re-running the engine's list builder from here is not verified
    // (0x7640A0 and the 0x762FF0 follow-up path are only mapped statically), so a late Result does not rebuild: reported once.
    void rebuildTopicList() override {
        static bool reported = false;
        if (!reported) {
            reported = true;
            unsupported("rebuildTopicList: the engine list is built from local state; late end-script effects reach the NEXT list (needs in-game proof)");
        }
    }
    // [stub] close the DialogMenu without local end scripts. Candidates: the CloseAllMenus routine near 0x715177
    // (JIP GameUI.h comment) or Menu::HandleClick on the goodbye tile; both unverified.
    void forceCloseDialog() override { if (!nvui::ops::forceCloseDialog()) unsupported("forceCloseDialog: no DialogMenu open"); }   // [live via L11] CloseAllMenus core, END script never runs locally
    // [live via L5/L11] all-or-nothing: the mask is not honoured, any lock request locks every player control (nvhooks::SetPlayerControlsLocked)
    void setControls(uint8_t, bool locked) override { nvui::ops::setControlsLocked(locked); }

    // [live] spectator banner/topics/subtitle through the HUD message queue
    void banner(const FormRef&, PlayerId initiator) override {
        char buf[96];
        std::snprintf(buf, sizeof buf, "Player %d is talking...", int(initiator));
        QueueUIMessage(buf, 2, nullptr, nullptr, 2.0f, false);
    }
    void showTopics(const std::vector<fnvcoop_dlg::TopicEntry>&) override {}  // v1: no topic overlay
    void showSubtitle(const FormRef& npc, const std::string& text, float seconds) override { Gate().onSubtitle(npc, text, seconds); }
    // [live via L11, in-game proof pending] token from voicePath() -> `<npc>.Say <topic>`; the engine picks the INFO by the local conditions
    void playVoice(const FormRef& npc, const std::string& token) override { Gate().onVoice(npc, token, GetTickCount64()); }
    void frame(uint32_t) override { Gate().tick(GetTickCount64()); }   // releases a stashed voice-less subtitle and gap-timed show items
    void hideOverlay() override {}

    // [live via L11, in-game proof pending] same cell (or both exterior) and within 2500 game units (about 36 m) of the local player
    bool npcAudible(const FormRef& npc) override {
        const uint32_t r = RuntimeRefId(npc);
        return r && nvui::ops::nearPlayer(r, 2500.0f);
    }
    // [stub] native StartConversation (NVSE Cmd_StartConversation) with the player as target, executed on THIS machine only; the
    // script command that triggered it on the host is vetoed there unless the host player is the actor
    void startForcedConversation(const FormRef& npc, const FormRef& topic) override {   // [live via L11] <npc>.StartConversation player <topic>
        uint32_t ref = RuntimeRefId(npc);
        if (!ref || !nvui::ops::openDialogMenu(ref, topic)) unsupported("startForcedConversation: npc ref not loaded");
    }

    // [by design not done] unlinking a row from the ListBox would desynchronise the tile index (a click uses tile trait 0xFAC as the list
    // index, docs/re/dialogmenu-menus-vats-inventory.md), so the row stays visible; the host rejects the pick and the glue explains why
    // (EngineGlue::onInfoChosen -> notice). Reported once.
    void hideTopic(Addr32) override {
        static bool reported = false;
        if (!reported) {
            reported = true;
            unsupported("hideTopic: owner-only command rows stay visible for non-owners; picking one is refused with a notice");
        }
    }
    void notice(const std::string& t) override { QueueUIMessage(t.substr(0, 0x1FF).c_str(), 2, nullptr, nullptr, 3.0f, false); }
    // [live, in-game proof pending] the engine's own MarkSaid (sets [info+0x22] and the save-modified flag unless flags1 & 0x10)
    void markSaid(const FormRef& info) override {
        const uint32_t f = LookupForm(RuntimeRefId(info));
        if (!f || !CallMarkSaid(f)) unsupported("markSaid: INFO not loaded");
    }
    void commitSayOnce(const FormRef& info) override { nvui::ops::recordSayOnce(info); }   // host: the SayOnceFlag effect for the clients
    void clearSaid(const FormRef& info) override {
        const uint32_t f = LookupForm(RuntimeRefId(info));
        if (!f || !WriteByte(f + topic_info::kSaidOnce, 0)) unsupported("clearSaid: INFO not loaded");
    }
    // [stub] the host's ShowMessageBox/ShowBarterMenu/... interception (L5) forwards the request here; opening it
    // locally means calling the same engine function the script command would have called, on THIS machine
    void openMenu(const fnvcoop_dlg::MenuRequest& rq) override { if (!nvui::ops::openMenu(rq)) unsupported("openMenu: kind not supported by hooks_ui"); }   // [live via L11]
    void closeMenu(uint32_t menu) override { nvui::ops::closeMenu(menu); }                                                                      // [live via L11]
    // [live] read-only notice through the HUD message queue
    void menuNotice(const std::string& text, bool closed) override {
        if (text.empty()) return;
        std::string t = (closed ? "Chose: " : "") + text.substr(0, 0x1F0);
        QueueUIMessage(t.c_str(), 2, nullptr, nullptr, 2.0f, false);
    }

    void trace(const std::string& s) override { if (report_) report_(s.c_str()); }
    void unsupported(const char* what) override {
        if (report_) report_(what);
        ++unsupportedCalls_;
    }
    unsigned unsupportedCalls() const { return unsupportedCalls_; }
    int saidState(const FormRef& info) override {
        const uint32_t f = LookupForm(RuntimeRefId(info));
        uint8_t b = 0;
        return f && readU8(mem_, f + topic_info::kSaidOnce, b) ? (b ? 1 : 0) : -1;
    }

private:
    ProcessMemory mem_;
    Report report_;
    unsigned unsupportedCalls_ = 0;
};

// Factory for the nvcoop module code (L1's NVCOOP_MODULE) to instantiate.
void ReportUnsupported(const char* what) {
    if (g_engine) g_engine->unsupported(what);
}

IEngineApi* CreateXnvseEngine(void (*report)(const char*)) {
    setResponseLoader([](Addr32 info) { return LoadResponsesSafely(info); });   // makes a default-constructed ResponseLayout work in the live game
    return new XnvseEngine(report);
}

}  // namespace fndlg
