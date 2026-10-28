// L11 hooks_ui x86 glue. Addresses/conventions: internal lane notes (all verified in Ghidra on a reference exe, prologues pinned in
// tests/test_ui.cpp). Every command-handler detour is cdecl (COMMAND_ARGS: paramInfo, scriptData, thisObj, containingObj, script,
// eventList, double* result, UInt32* opcodeOffsetPtr); a static_assert ties each one to `ret` (retBytes == 0).
#include "ui_hooks.h"

#include <Windows.h>

#include <cstdio>
#include <cstring>
#include <deque>
#include <memory>
#include <unordered_map>

#include "common/nvcoop_api.h"
#include "activate_args.h"   // plugin/hooks: the engine Activate arguments of the console / bridge activation
#include "engine_hooks.h"
#include "extract_pad.h"   // plugin/hooks: every ExtractArgs call gets spare out slots (issue #33)
#include "ui_addrs.h"

using nvhooks::extractPadded;

namespace nvui {
namespace {

using nvhooks::PatchStatus;

// ------------------------------------------------------------------ memory
struct Win32Memory : nvhooks::MemoryOps {
    bool read(nvhooks::Addr a, void* out, size_t n) override {
        __try {
            std::memcpy(out, reinterpret_cast<const void*>(a), n);
            return true;
        } __except (EXCEPTION_EXECUTE_HANDLER) {
            return false;
        }
    }
    bool write(nvhooks::Addr a, const void* in, size_t n) override {
        DWORD old = 0;
        void* p = reinterpret_cast<void*>(a);
        if (!VirtualProtect(p, n, PAGE_EXECUTE_READWRITE, &old)) return false;
        std::memcpy(p, in, n);
        FlushInstructionCache(GetCurrentProcess(), p, n);
        DWORD tmp;
        VirtualProtect(p, n, old, &tmp);
        return true;
    }
    nvhooks::Addr allocExec(size_t n) override {
        return reinterpret_cast<nvhooks::Addr>(VirtualAlloc(nullptr, n, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE));
    }
    void freeExec(nvhooks::Addr a, size_t) override { VirtualFree(reinterpret_cast<void*>(a), 0, MEM_RELEASE); }
};

Win32Memory g_mem;
std::unique_ptr<nvhooks::HookRegistry> g_reg;
UiRouter* g_router = nullptr;
bool g_host = false;
Counters g_cnt;
std::deque<RecentBox> g_recentBoxes;
InfoReader g_infoReader;
std::vector<std::string> g_installed;

bool safeRead(uint32_t a, void* out, size_t n) {
    __try {
        std::memcpy(out, reinterpret_cast<const void*>(a), n);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
uint32_t rd32(uint32_t a) {
    uint32_t v = 0;
    safeRead(a, &v, 4);
    return v;
}
uint32_t refIdOf(const void* form) { return form ? rd32(reinterpret_cast<uint32_t>(form) + 0x0C) : 0; }
uint8_t typeOf(const void* form) {
    uint8_t t = 0;
    if (form) safeRead(reinterpret_cast<uint32_t>(form) + 0x04, &t, 1);
    return t;
}
size_t safeCopy(const char* p, char* buf, size_t cap) {
    size_t n = 0;
    __try {
        while (n + 1 < cap && p[n]) {
            buf[n] = p[n];
            ++n;
        }
    } __except (EXCEPTION_EXECUTE_HANDLER) {
    }
    return n;
}
std::string safeStr(const char* p) {
    if (!p) return std::string();
    static char buf[0x4000];
    const size_t n = safeCopy(p, buf, sizeof buf);
    return std::string(buf, n);
}

constexpr uint32_t kQuestFormType = 0x47;

using ExtractArgsFn = bool(__cdecl*)(void*, void*, uint32_t*, void*, void*, void*, void*, ...);
ExtractArgsFn extractArgs() { return reinterpret_cast<ExtractArgsFn>(addr::ExtractArgs.addr); }

bool toRef(uint32_t refID, FormRef& out) { return refID && nvhooks::resolver().toRef(refID, &out); }

// runtime refID -> in-memory form (the engine's global form table: LookupFormByID 0x4839C0, cdecl, verified by L5 in docs/engine-addresses.md)
uint32_t lookupForm(uint32_t refID) {
    using Fn = void*(__cdecl*)(uint32_t);
    __try {
        return reinterpret_cast<uint32_t>(reinterpret_cast<Fn>(0x4839C0)(refID));
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return 0;
    }
}

// ------------------------------------------------------------------ engine seam for the router
struct GameEngine : IUiEngine {
    uint64_t nowMs() override { return GetTickCount64(); }
    void setMessageBoxResult(uint32_t scriptRefID, int32_t button) override {
        // what ShowMessageBox_Callback (0x5B4A70) does: remember the button; GetButtonPressed (0x5B4A80) then returns it if the
        // stored script refID is the caller's, and clears both
        *reinterpret_cast<uint32_t*>(addr::MessageBoxScriptRefID) = scriptRefID;
        *reinterpret_cast<uint8_t*>(addr::MessageBoxButton) = uint8_t(button & 0xFF);
    }
    bool refOf(uint32_t refID, FormRef& out) override { return toRef(refID, out); }
    void log(const std::string& s) override { nvcoop::Log("[ui] %s", s.c_str()); }
    void openMirror(uint32_t menu, const std::string& text, uint8_t origin) override { ops::openMirror(menu, text, origin); }
    void closeMirror(uint32_t menu) override { ops::closeMirror(menu); }
};
GameEngine g_engine;

// ------------------------------------------------------------------ command handler detours (all cdecl COMMAND_ARGS)
#define CMD_ARGS void *paramInfo, void *scriptData, void *thisObj, void *contObj, void *script, void *eventList, double *result, uint32_t *opcodeOff
#define CMD_PASS paramInfo, scriptData, thisObj, contObj, script, eventList, result, opcodeOff
using Cmd = bool(__cdecl*)(CMD_ARGS);

// ---- ShowMessage + the box itself ----
Cmd o_ShowMessage;
int g_inShowMessage = 0;
void* g_showThisObj = nullptr;
FormRef g_showMessageForm;
FormRef g_showScript;   // the running script of the ShowMessage in flight (G07: does it have a MenuMode block?)

bool __cdecl h_ShowMessage(CMD_ARGS) {
    ++g_cnt.seenShowMessage;
    if (!g_host || !g_router) return o_ShowMessage(CMD_PASS);
    // identify the MESG form (consent rules); same extraction the vanilla handler does (message form + one more out slot)
    g_showMessageForm = FormRef();
    {
        void* msg = nullptr;
        uint32_t dummy = 0;
        uint32_t off = opcodeOff ? *opcodeOff : 0;
        // ShowMessage takes up to nine more variables and a duration: all of them land in the padding, never through stray stack words
        if (extractPadded(extractArgs(), paramInfo, scriptData, &off, thisObj, contObj, script, eventList, {&msg, &dummy})) toRef(refIdOf(msg), g_showMessageForm);
    }
    g_showThisObj = thisObj;
    g_showScript = FormRef();
    toRef(refIdOf(script), g_showScript);
    ++g_inShowMessage;
    struct Leave {
        ~Leave() { --g_inShowMessage; }
    } leave;
    return o_ShowMessage(CMD_PASS);
}

using Fn_ShowMessageBox = bool(__cdecl*)(const char*, uint32_t, uint32_t, void*, uint32_t, uint32_t, float, float, const char*, const char*,
                                          const char*, const char*, const char*, const char*, const char*, const char*, const char*,
                                          const char*, const char*);
Fn_ShowMessageBox o_ShowMessageBox;

// Variadic in the engine (buttons, NULL terminated). Reading b0..b9 is safe because we only dereference up to the first NULL; the
// caller pushed at least that many.
bool __cdecl h_ShowMessageBox(const char* msg, uint32_t u1, uint32_t u2, void* cb, uint32_t u4, uint32_t u5, float f6, float f7, const char* b0,
                              const char* b1, const char* b2, const char* b3, const char* b4, const char* b5, const char* b6,
                              const char* b7, const char* b8, const char* b9) {
    if (g_inShowMessage == 0) {     // the engine's own box (load notices, errors): not routed, but logged for the tests
        RecentBox rb;
        rb.seq = ++g_cnt.seenEngineBoxes, rb.text = safeStr(msg).substr(0, 160);
        g_recentBoxes.push_back(std::move(rb));
        if (g_recentBoxes.size() > kRecentBoxes) g_recentBoxes.pop_front();
    }
    if (g_inShowMessage > 0 && g_host && g_router) {
        ++g_cnt.seenBoxes;
        MessageBoxCall c;
        c.scriptRefID = rd32(addr::MessageBoxScriptRefID);
        c.thisObj = refIdOf(g_showThisObj);
        c.message = g_showMessageForm;
        c.script = g_showScript;
        c.text = safeStr(msg);
        const char* bs[10] = {b0, b1, b2, b3, b4, b5, b6, b7, b8, b9};
        for (int i = 0; i < 10 && bs[i]; ++i) c.buttons.push_back(safeStr(bs[i]));
        RecentBox rb;
        rb.seq = g_cnt.seenBoxes, rb.scriptRefID = c.scriptRefID, rb.thisObj = c.thisObj, rb.message = c.message, rb.text = c.text.substr(0, 160);
        g_recentBoxes.push_back(std::move(rb));
        if (g_recentBoxes.size() > kRecentBoxes) g_recentBoxes.pop_front();
        if (g_router->onMessageBox(c) == Route::Suppressed) return true;
    }
    return o_ShowMessageBox(msg, u1, u2, cb, u4, u5, f6, f7, b0, b1, b2, b3, b4, b5, b6, b7, b8, b9, nullptr);
}

// ---- service menus ----
#define SERVICE_HOOK(NAME, KIND)                                                                              \
    Cmd o_##NAME;                                                                                             \
    bool __cdecl h_##NAME(CMD_ARGS) {                                                                         \
        ++g_cnt.seenService;                                                                                  \
        if (g_host && g_router) {                                                                             \
            ServiceMenuCall c;                                                                                \
            c.kind = KIND;                                                                                    \
            c.thisObj = refIdOf(thisObj);                                                                     \
            c.scriptKey = refIdOf(script);                                                                    \
            if (g_router->onServiceMenu(c) == Route::Suppressed) return true;                                 \
        }                                                                                                     \
        return o_##NAME(CMD_PASS);                                                                            \
    }
SERVICE_HOOK(Barter, MenuKind::Barter)
SERVICE_HOOK(Repair, MenuKind::Repair)
SERVICE_HOOK(Recipe, MenuKind::Recipe)
SERVICE_HOOK(Caravan, MenuKind::Caravan)
SERVICE_HOOK(Slot, MenuKind::Other)
SERVICE_HOOK(BlackJack, MenuKind::Other)
SERVICE_HOOK(Roulette, MenuKind::Other)

// ---- GetQuestVariable / GetScriptVariable condition functions ----
using Fn_Cond = bool(__cdecl*)(void*, void*, uint32_t, double*);
Fn_Cond o_Cond;
bool __cdecl h_Cond(void* thisObj, void* form, uint32_t idx, double* result) {
    const bool ok = o_Cond(thisObj, form, idx, result);
    if (ok && form && g_router && result && typeOf(form) == kQuestFormType) {
        ++g_cnt.seenCond;
        *result = g_router->viewQuestVar(refIdOf(form), idx, *result);
    }
    return ok;
}

// ---- forced dialogue ----
int g_forcedDepth = 0;
uint32_t g_lastInfo = 0;
using Fn_GetTopicInfo = int(__cdecl*)(int, uint8_t*, void*, void*, char, int, int);
Fn_GetTopicInfo o_GetTopicInfo;
// info -> topic, learned from every resolution the engine does (the topic-list builder calls this for each offered topic). Vanilla
// TESTopicInfo has no back pointer, so this is how L8's topicOf works. Installed on guests too: the initiator builds the lists.
std::unordered_map<uint32_t, uint32_t> g_topicOfInfo;
constexpr size_t kMaxTopicCache = 400000;
int __cdecl h_GetTopicInfo(int topic, uint8_t* flag, void* speaker, void* target, char p5, int p6, int p7) {
    int info = o_GetTopicInfo(topic, flag, speaker, target, p5, p6, p7);
    if (info && topic && g_topicOfInfo.size() < kMaxTopicCache) g_topicOfInfo[uint32_t(info)] = uint32_t(topic);
    if (g_forcedDepth > 0 && info) g_lastInfo = uint32_t(info);
    return info;
}

void recordForcedEffect(const ForcedCall& c, const ForcedOutcome& o) {
    nvhooks::CaptureHub* hub = nvhooks::hub();
    if (!hub || !hub->active() || !o.captured || c.kind == ForcedCall::Kind::Say) return;
    fnvcoop::ForcedDialogue fd;
    toRef(c.speaker, fd.npc);
    fd.topic = c.topic;
    fd.initiator = (o.target < 8) ? o.target : 0;
    hub->record(fd);
}

Cmd o_Say, o_SayTo, o_StartConv;

bool __cdecl h_Say(CMD_ARGS) {
    ++g_cnt.seenForced;
    if (!g_host || !g_router) return o_Say(CMD_PASS);
    void* topic = nullptr;
    int32_t a = 0, b = 0, d = 0;
    void* base = nullptr;
    uint32_t off = opcodeOff ? *opcodeOff : 0;
    const bool got = extractPadded(extractArgs(), paramInfo, scriptData, &off, thisObj, contObj, script, eventList, {&topic, &a, &base, &b, &d});
    g_lastInfo = 0;
    ++g_forcedDepth;
    const bool r = o_Say(CMD_PASS);
    --g_forcedDepth;
    if (got && topic) {
        ForcedCall c;
        c.kind = ForcedCall::Kind::Say;
        c.speaker = refIdOf(thisObj);
        toRef(refIdOf(topic), c.topic);
        if (g_lastInfo) {
            toRef(refIdOf(reinterpret_cast<void*>(g_lastInfo)), c.info);
            if (g_infoReader) c.lines = g_infoReader(g_lastInfo);
        }
        recordForcedEffect(c, g_router->onForced(c));
    }
    return r;
}

bool __cdecl h_SayTo(CMD_ARGS) {
    ++g_cnt.seenForced;
    if (!g_host || !g_router) return o_SayTo(CMD_PASS);
    void* actor = nullptr;
    void* topic = nullptr;
    int32_t a = 0, b = 0;
    uint32_t off = opcodeOff ? *opcodeOff : 0;
    const bool got = extractPadded(extractArgs(), paramInfo, scriptData, &off, thisObj, contObj, script, eventList, {&actor, &topic, &a, &b});
    g_lastInfo = 0;
    ++g_forcedDepth;
    const bool r = o_SayTo(CMD_PASS);
    --g_forcedDepth;
    if (got && topic) {
        ForcedCall c;
        c.kind = ForcedCall::Kind::SayTo;
        c.speaker = refIdOf(thisObj);
        c.target = refIdOf(actor);
        toRef(refIdOf(topic), c.topic);
        if (g_lastInfo) {
            toRef(refIdOf(reinterpret_cast<void*>(g_lastInfo)), c.info);
            if (g_infoReader) c.lines = g_infoReader(g_lastInfo);
        }
        recordForcedEffect(c, g_router->onForced(c));
    }
    return r;
}

bool __cdecl h_StartConversation(CMD_ARGS) {
    ++g_cnt.seenForced;
    if (!g_host || !g_router) return o_StartConv(CMD_PASS);
    void* target = nullptr;
    void* topic = nullptr;
    void* r1 = nullptr;
    void* r2 = nullptr;
    int32_t a = 0, b = 0;
    uint32_t off = opcodeOff ? *opcodeOff : 0;
    if (extractPadded(extractArgs(), paramInfo, scriptData, &off, thisObj, contObj, script, eventList, {&target, &topic, &r1, &r2, &a, &b}) && topic) {
        ForcedCall c;
        c.kind = ForcedCall::Kind::StartConversation;
        c.speaker = refIdOf(thisObj);
        c.target = refIdOf(target);
        toRef(refIdOf(topic), c.topic);
        const ForcedOutcome o = g_router->onForced(c);
        recordForcedEffect(c, o);
        if (o.vetoNative) return true;  // the actor's own machine opens the DialogMenu; the host does not
    }
    return o_StartConv(CMD_PASS);
}

// ---- say-once marks: TESTopicInfo::MarkSaid (0x61F220, thiscall(info), plain ret). Observe only: the original always runs; the 0 -> 1 change of
// the said byte goes to the sink (plugin/dialogue routes it: a guest reports it to the host, the host commits it as SayOnceFlag). Routing lives in
// fnvcoop_dlg::SayOncePolicy (game-free, tested), not here.
using Fn_MarkSaid = void(__thiscall*)(void*);
Fn_MarkSaid o_MarkSaid;
SaidSink g_saidSink;
int g_saidMute = 0;
constexpr uint32_t kInfoSaidOffset = 0x22;
void __fastcall h_MarkSaid(void* info, void*) {
    const bool watch = info && g_saidSink && g_saidMute == 0;
    uint8_t before = 1, after = 0;
    if (watch && !safeRead(reinterpret_cast<uint32_t>(info) + kInfoSaidOffset, &before, 1)) before = 1;
    o_MarkSaid(info);
    if (!watch || before) return;
    if (!safeRead(reinterpret_cast<uint32_t>(info) + kInfoSaidOffset, &after, 1) || !after) return;   // the engine declined (flags1 & 0x10)
    ++g_cnt.seenMarkSaid;
    g_saidSink(reinterpret_cast<uint32_t>(info));
}

// ---- ending slideshow: SwapTexture ref, oldPath, newPath ----
Cmd o_Swap;
bool __cdecl h_SwapTexture(CMD_ARGS) {
    ++g_cnt.seenSwap;
    if (g_host && g_router) {
        void* ref = nullptr;
        char oldTex[512] = {}, newTex[512] = {};
        uint32_t off = opcodeOff ? *opcodeOff : 0;
        if (extractPadded(extractArgs(), paramInfo, scriptData, &off, thisObj, contObj, script, eventList, {&ref, oldTex, newTex}))
            g_router->onSwapTexture(refIdOf(ref), oldTex, newTex);
    }
    return o_Swap(CMD_PASS);
}

// ------------------------------------------------------------------ calling-convention pins
#define PIN_CDECL(spec) static_assert(addr::spec.retBytes == 0, #spec " is not a plain `ret` (cdecl) function")
PIN_CDECL(ShowMessage);
PIN_CDECL(ShowMessageBox);
PIN_CDECL(CondEvalVar);
PIN_CDECL(CmdSay);
PIN_CDECL(CmdSayTo);
PIN_CDECL(CmdStartConversation);
PIN_CDECL(CmdSwapTexture);
PIN_CDECL(InfoMarkSaid);
PIN_CDECL(GetTopicInfo);
PIN_CDECL(CmdShowBarterMenu);
PIN_CDECL(CmdShowRepairMenu);
PIN_CDECL(CmdShowRecipeMenu);
PIN_CDECL(CmdShowCaravanMenu);
PIN_CDECL(CmdShowSlotMachine);
PIN_CDECL(CmdShowBlackJack);
PIN_CDECL(CmdShowRoulette);

struct Entry {
    const nvhooks::HookSpec* spec;
    void* detour;
    void* original;  // address of the `o_*` pointer
    bool hostOnly;
};
#define E(spec, det, orig, hostOnly) {&addr::spec, reinterpret_cast<void*>(&det), reinterpret_cast<void*>(&orig), hostOnly}
const Entry kEntries[] = {
    E(CondEvalVar, h_Cond, o_Cond, false),
    E(ShowMessage, h_ShowMessage, o_ShowMessage, true),
    E(ShowMessageBox, h_ShowMessageBox, o_ShowMessageBox, true),
    E(CmdSay, h_Say, o_Say, true),
    E(CmdSayTo, h_SayTo, o_SayTo, true),
    E(CmdStartConversation, h_StartConversation, o_StartConv, true),
    E(CmdSwapTexture, h_SwapTexture, o_Swap, true),
    E(GetTopicInfo, h_GetTopicInfo, o_GetTopicInfo, false),
    E(CmdShowBarterMenu, h_Barter, o_Barter, true),
    E(CmdShowRepairMenu, h_Repair, o_Repair, true),
    E(CmdShowRecipeMenu, h_Recipe, o_Recipe, true),
    E(CmdShowCaravanMenu, h_Caravan, o_Caravan, true),
    E(CmdShowSlotMachine, h_Slot, o_Slot, true),
    E(CmdShowBlackJack, h_BlackJack, o_BlackJack, true),
    E(CmdShowRoulette, h_Roulette, o_Roulette, true),
    E(InfoMarkSaid, h_MarkSaid, o_MarkSaid, false),   // both roles: a guest reports its marks, the host commits its own
};

}  // namespace

// ------------------------------------------------------------------ public
UiRouter* MakeRouter(Bridge b) {
    static std::unique_ptr<UiRouter> r;
    r = std::make_unique<UiRouter>(g_engine, std::move(b));
    return r.get();
}

InitReport Init(UiRouter& router, bool host) {
    InitReport rep;
    g_router = &router;
    g_host = host;
    router.setHost(host);
    if (!g_reg) g_reg = std::make_unique<nvhooks::HookRegistry>(g_mem);
    for (const Entry& e : kEntries) {
        if (e.hostOnly && !host) continue;
        nvhooks::Addr tramp = 0;
        PatchStatus st = g_reg->acquire(*e.spec, reinterpret_cast<nvhooks::Addr>(e.detour), &tramp, reinterpret_cast<nvhooks::Addr*>(e.original));
        rep.hooks.push_back({e.spec->name, st});
        if (nvhooks::installed(st)) {
            *reinterpret_cast<nvhooks::Addr*>(e.original) = tramp;
            g_installed.push_back(e.spec->name);
            ++rep.ok;
        } else {
            ++rep.failed;
            nvcoop::Log("[ui] hook %s skipped: %s", e.spec->name, nvhooks::statusName(st));
        }
    }
    return rep;
}

int auditTamperedHooks() { return g_reg ? g_reg->auditTampered() : 0; }

void Shutdown() {
    g_reg.reset();
    g_installed.clear();
    g_router = nullptr;
}
UiRouter* router() { return g_router; }
void setInfoReader(InfoReader f) { g_infoReader = std::move(f); }
void setSaidSink(SaidSink f) { g_saidSink = std::move(f); }
SaidMute::SaidMute() { ++g_saidMute; }
SaidMute::~SaidMute() { if (g_saidMute > 0) --g_saidMute; }
bool ops::recordSayOnce(const FormRef& info) {
    nvhooks::CaptureHub* hub = nvhooks::hub();
    if (!hub || !hub->enabled() || info.plugin.empty()) return false;
    if (hub->active()) return hub->record(fnvcoop::SayOnceFlag{info});   // inside a script's capture scope: part of that txn
    fnvcoop::Cause c;
    c.kind = fnvcoop::CauseKind::Script;
    c.source = info;
    nvhooks::ScopedCapture scope(*hub, c);
    return hub->record(fnvcoop::SayOnceFlag{info});
}
const Counters& counters() { return g_cnt; }
std::vector<RecentBox> recentBoxes() { return {g_recentBoxes.begin(), g_recentBoxes.end()}; }
bool installedByName(const std::string& n) {
    for (auto& s : g_installed)
        if (s == n) return true;
    return false;
}

// ------------------------------------------------------------------ origin side ops
namespace ops {
namespace {
struct QueuedBox {
    uint32_t id;
    std::string text;
    std::vector<std::string> buttons;
};
std::deque<QueuedBox> g_queue;
uint32_t g_showing = 0;
Answer g_answer;
QueuedBox g_mirrorBox{};          // the read-only copy of a box a remote player is answering (G07)
uint32_t g_mirrorId = 0;
bool g_mirrorRepeat = false;

void __cdecl boxCallback() {
    const int b = reinterpret_cast<int(__cdecl*)()>(addr::MessageBoxLastButton.addr)();
    const uint32_t id = g_showing;
    g_showing = 0;
    if (id && id == g_mirrorId) {   // the host pressed the mirror's dummy button: not an answer, the box comes back next frame
        g_mirrorRepeat = true;
        return;
    }
    if (id && g_answer) g_answer(id, b);
}

bool showNow(const QueuedBox& q) {
    const char* b[10] = {};
    std::vector<std::string> btn = q.buttons;
    if (btn.empty()) btn.push_back("Ok");
    for (size_t i = 0; i < btn.size() && i < 10; ++i) b[i] = btn[i].c_str();
    Fn_ShowMessageBox fn = o_ShowMessageBox ? o_ShowMessageBox : reinterpret_cast<Fn_ShowMessageBox>(0x703E80);
    g_showing = q.id;
    // same arguments as the vanilla ShowMessage command (xNVSE Cmd_MessageBoxEx): unk5 = 0x17, own callback
    const bool ok = fn(q.text.c_str(), 0, 0, reinterpret_cast<void*>(&boxCallback), 0, 0x17, 0.f, 0.f, b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7],
                       b[8], b[9], nullptr);
    if (!ok) g_showing = 0;
    return ok;
}
}  // namespace

void setAnswerSink(Answer a) { g_answer = std::move(a); }

bool openMenu(const fnvcoop_dlg::MenuRequest& rq) {
    if (rq.kind == MenuKind::MessageBox || rq.kind == MenuKind::Terminal) {
        g_queue.push_back({rq.menu, rq.text, rq.buttons});
        pump();
        return true;
    }
    // service menus: run the vanilla command on the subject; the menu has no result, the "answer" is that it was shown
    const char* cmd = rq.kind == MenuKind::Barter ? "ShowBarterMenu" : rq.kind == MenuKind::Repair ? "ShowRepairMenu"
                    : rq.kind == MenuKind::Recipe ? "ShowRecipeMenu" : rq.kind == MenuKind::Caravan ? "ShowCaravanMenu" : nullptr;
    if (!cmd) {
        nvcoop::Log("[ui] openMenu: kind %d not supported", int(rq.kind));
        if (g_answer) g_answer(rq.menu, -1);
        return false;
    }
    const uint32_t ref = nvhooks::resolver().fromRef(rq.subject);
    const bool ok = ref && nvhooks::execConsoleLineOn(cmd, ref);
    if (g_answer) g_answer(rq.menu, ok ? 0 : -1);
    return ok;
}

bool openMirror(uint32_t menu, const std::string& text, uint8_t origin) {
    g_mirrorId = menu;
    g_mirrorRepeat = false;
    g_mirrorBox = QueuedBox{menu, "Player " + std::to_string(int(origin)) + " is deciding:\n" + text, {"Waiting..."}};
    g_queue.push_back(g_mirrorBox);
    pump();
    return true;
}

void closeMirror(uint32_t menu) {
    if (menu != g_mirrorId) return;
    g_mirrorId = 0;
    g_mirrorRepeat = false;
    for (auto it = g_queue.begin(); it != g_queue.end(); ++it)
        if (it->id == menu) {
            g_queue.erase(it);
            break;
        }
    if (g_showing == menu) {
        g_showing = 0;   // never answers: the origin already did
        reinterpret_cast<void(__cdecl*)()>(addr::CloseAllMenusCore.addr)();
    }
}

void pump() {
    if (g_mirrorRepeat && g_mirrorId && !g_showing) {   // re-show a mirror the host dismissed
        g_mirrorRepeat = false;
        g_queue.push_front(g_mirrorBox);
    }
    if (g_showing || g_queue.empty()) return;
    QueuedBox q = std::move(g_queue.front());
    g_queue.pop_front();
    if (!showNow(q) && g_answer) g_answer(q.id, -1);
}

void closeMenu(uint32_t menu) {
    for (auto it = g_queue.begin(); it != g_queue.end(); ++it)
        if (it->id == menu) {
            g_queue.erase(it);
            return;
        }
    if (g_showing == menu) {
        g_showing = 0;  // the host already took its default: do not answer
        reinterpret_cast<void(__cdecl*)()>(addr::CloseAllMenusCore.addr)();
    }
}

bool dialogOpen() { return rd32(addr::DialogMenuPtr) != 0; }

bool forceCloseDialog() {
    if (!dialogOpen()) return false;
    // the END result script of the open INFO must not run on this machine: it only runs when *0x11D9518 != 0
    *reinterpret_cast<uint32_t*>(addr::DialogMenuInfoSpoken) = 0;
    reinterpret_cast<void(__cdecl*)()>(addr::CloseAllMenusCore.addr)();
    return true;
}

bool openDialogMenu(uint32_t speakerRefID, const FormRef& topic) {
    const uint32_t t = nvhooks::resolver().fromRef(topic);
    if (!speakerRefID || !t) return false;
    char line[64];
    std::snprintf(line, sizeof line, "StartConversation player %08X", t);
    return nvhooks::execConsoleLineOn(line, speakerRefID);
}

bool swapTexture(const FormRef& ref, const std::string& oldTex, const std::string& newTex) {
    const uint32_t id = nvhooks::resolver().fromRef(ref);
    if (!id) return false;
    char line[1200];
    std::snprintf(line, sizeof line, "SwapTexture %08X \"%s\" \"%s\"", id, oldTex.c_str(), newTex.c_str());
    return nvhooks::execConsoleLine(line);
}

size_t queuedBoxes() { return g_queue.size() + (g_showing ? 1 : 0); }

bool answerShowing(int button, uint32_t* menuId) {
    if (!g_showing) return false;
    const uint32_t id = g_showing;
    g_showing = 0;
    reinterpret_cast<void(__cdecl*)()>(addr::CloseAllMenusCore.addr)();
    if (menuId) *menuId = id;
    if (g_answer) g_answer(id, button);
    return true;
}

uint32_t topicOfInfo(uint32_t infoAddr) {
    auto it = g_topicOfInfo.find(infoAddr);
    return it == g_topicOfInfo.end() ? 0 : it->second;
}

bool sayTopic(uint32_t npcRefID, const FormRef& topic) {
    const uint32_t t = nvhooks::resolver().fromRef(topic);
    if (!npcRefID || !t) return false;
    char line[64];
    std::snprintf(line, sizeof line, "Say %08X", t);
    return nvhooks::execConsoleLineOn(line, npcRefID);
}

namespace {
using ActivateFn = uint32_t(__thiscall*)(void*, void*, uint8_t, uint32_t, uint32_t);
bool activateAsPlayerSEH(uint32_t npcRefID) {
    __try {
        void* npc = reinterpret_cast<void*>(lookupForm(npcRefID));
        void* player = reinterpret_cast<void*>(lookupForm(0x14));
        if (!npc || !player) return false;
        const nvhooks::ActivateArgs a = nvhooks::consoleActivateArgs();
        reinterpret_cast<ActivateFn>(0x573170)(npc, player, a.flagA, a.flagB, a.last);   // TESObjectREFR::Activate, as the player's key press (and the bridge's activate.ref) calls it
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}
}  // namespace

bool resumeTalk(uint32_t npcRefID) {
    if (!npcRefID) return false;
    // The vanilla Activate was deferred by the activate observer until the host granted the conversation: do it now, the way it was asked (story fix 2026-10-28).
    // The grant makes the observer let this one through (the conversation role is no longer idle). `StartConversation player GREETING` is an AI order: the NPC
    // first has to come to the player, and an NPC 200+ units away never opened the menu (the first Goodsprings story run: S01, S06, S13, S16), which left the
    // granted conversation open on the host and the guest's role stuck.
    if (activateAsPlayerSEH(npcRefID)) return true;
    return nvhooks::execConsoleLineOn("StartConversation player GREETING", npcRefID);
}

void setControlsLocked(bool locked) { nvhooks::SetPlayerControlsLocked(locked); }

bool refPosition(uint32_t refID, uint32_t* cell, float xyz[3]) {
    // TESObjectREFR: pos at 0x30..0x38, parentCell at 0x40 (xNVSE GameObjects.h); TESObjectCELL::worldSpace at 0xC0 (null = interior)
    const uint32_t form = refID ? lookupForm(refID) : 0;
    if (!form) return false;
    uint8_t type = 0;
    if (!safeRead(form + 0x04, &type, 1) || type < 0x3A || type > 0x3C) return false;  // REFR / ACHR / ACRE
    float p[3] = {};
    uint32_t c = 0;
    if (!safeRead(form + 0x30, p, sizeof p) || !safeRead(form + 0x40, &c, 4)) return false;
    if (cell) *cell = c;
    if (xyz) std::memcpy(xyz, p, sizeof p);
    return true;
}
bool cellIsInterior(uint32_t cell) { return cell && rd32(cell + 0xC0) == 0; }

bool nearPlayer(uint32_t refID, float maxDist) {
    uint32_t cn = 0, cp = 0;
    float n[3], p[3];
    if (!refPosition(refID, &cn, n) || !refPosition(0x14, &cp, p)) return false;
    if (!cn || !cp) return false;
    if (cn != cp && (cellIsInterior(cn) || cellIsInterior(cp))) return false;
    const float dx = n[0] - p[0], dy = n[1] - p[1], dz = n[2] - p[2];
    return dx * dx + dy * dy + dz * dz <= maxDist * maxDist;
}
}  // namespace ops

// ------------------------------------------------------------------ ending slideshow (both roles)
ShowSequencer& showSequencer() {
    static ShowSequencer s;
    return s;
}
SlideClient& slideClient() {
    static SlideClient c(SlideClient::Callbacks{
        [](const FormRef& r, const std::string& o, const std::string& n) { return ops::swapTexture(r, o, n); },
        [](bool locked) { nvhooks::SetPlayerControlsLocked(locked); },
        [](const std::string& s) { nvcoop::Log("[ui] slideshow: %s", s.c_str()); }}, &showSequencer());
    return c;
}
SlideHost& slideHost(SlideHost::Send send) {
    static std::unique_ptr<SlideHost> h;
    if (!h) h = std::make_unique<SlideHost>(send ? std::move(send) : SlideHost::Send([](fnvcoop::PlayerId, std::vector<uint8_t>) {}));
    return *h;
}

}  // namespace nvui
