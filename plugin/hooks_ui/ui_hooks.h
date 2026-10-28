// L11 hooks_ui: x86 glue (xNVSE headers, FalloutNV.exe 1.4.0.525). Installs prologue-checked detours through
// nvhooks::HookRegistry (L5's patcher; own instance, own memory ops) and feeds nvui::UiRouter.
// Inert until Init() is called. Everything runs on the game main thread.
#pragma once
#include <functional>
#include <string>
#include <vector>

#include "hook_patch.h"
#include "slideshow.h"
#include "ui_core.h"

namespace nvui {

struct HookReport {
    std::string name;
    nvhooks::PatchStatus status;
};
struct InitReport {
    std::vector<HookReport> hooks;
    int ok = 0, failed = 0;
};

// host=true: route boxes/menus/forced dialogue of REMOTE origins away from this machine. host=false (guest): only the condition
// view hook is installed (conditions on the guest are evaluated for the guest's own slot).
// The router with the game IUiEngine behind it (owned here; one per process).
UiRouter* MakeRouter(Bridge b);
InitReport Init(UiRouter& router, bool host);
void Shutdown();
int auditTamperedHooks();  // hot reload preflight: our hooks that another patcher chained over (0 = safe to unload)
UiRouter* router();

// Reads the response chain of an INFO (L8's fndlg::readLines behind a lambda; wired by L10). Without it ForcedSay carries no text.
using InfoReader = std::function<std::vector<fnvcoop_dlg::Line>(uint32_t infoAddr)>;
void setInfoReader(InfoReader f);

struct Counters {
    uint64_t seenShowMessage = 0, seenBoxes = 0, seenService = 0, seenCond = 0, seenForced = 0, seenSwap = 0, seenActivate = 0;
    uint64_t seenEngineBoxes = 0;   // ShowMessageBox outside ShowMessage (the engine's own boxes; script 0 in recentBoxes)
    uint64_t seenMarkSaid = 0;      // TESTopicInfo::MarkSaid calls that turned the said byte on (reported to the sink)
};
const Counters& counters();

// Say-once (owner decision D11): every 0 -> 1 change of an INFO's said byte that TESTopicInfo::MarkSaid makes (the argument is the TESTopicInfo*) is
// handed to the sink, on guests and on the host. Calls made while a SaidMute is alive are not reported (our own marks: the dialogue glue marking a
// line it already knows about, the host marking its copy for a guest's report). The sink is plugin/dialogue's EngineGlue::onEngineMark.
using SaidSink = std::function<void(uint32_t infoAddr)>;
void setSaidSink(SaidSink f);
struct SaidMute {
    SaidMute();
    ~SaidMute();
    SaidMute(const SaidMute&) = delete;
    SaidMute& operator=(const SaidMute&) = delete;
};

// The last kRecentBoxes script message boxes (ShowMessage) this machine raised, oldest first: which MESG, which script, what text.
// A diagnostic for tests (a box keeps the game in menu mode and swallows the bots' input), shown by ui.status.
struct RecentBox {
    uint64_t seq = 0;
    uint32_t scriptRefID = 0, thisObj = 0;
    FormRef message;
    std::string text;
};
constexpr size_t kRecentBoxes = 16;
std::vector<RecentBox> recentBoxes();

// ---- origin-player side: what L8's EngineGlue stubs (openMenu / closeMenu / forceCloseDialog / resumeTalk) call -----------------
namespace ops {
using Answer = std::function<void(uint32_t menu, int32_t choice)>;
void setAnswerSink(Answer a);  // -> EngineGlue::onMenuAnswered
// MenuRequest on THIS machine: MessageBox opens a real message box (queued one at a time); service menus run the vanilla command
// on the subject; anything else is reported through log and answered with -1.
bool openMenu(const fnvcoop_dlg::MenuRequest& rq);
void closeMenu(uint32_t menu);          // timeout: forces the box shut without answering
void pump();                            // per frame: starts the next queued box
bool openDialogMenu(uint32_t speakerRefID, const FormRef& topic);   // forced conversation: `<npc>.StartConversation player <topic>`
// G07 mirror: a read-only copy of a box that was raised to a remote player (one dummy button that re-shows the box when pressed) keeps
// the engine in message-menu mode, so the script's `begin MenuMode 1001` block runs here. closeMirror when the origin answered.
bool openMirror(uint32_t menu, const std::string& text, uint8_t origin);
void closeMirror(uint32_t menu);
bool forceCloseDialog();                // CloseAllMenusCore while the DialogMenu is open; never runs the END script locally
bool dialogOpen();
// Ending slideshow playback on a client: the SwapTexture the host script ran (`SwapTexture <ref> "old" "new"` through the console executor).
bool swapTexture(const FormRef& ref, const std::string& oldTex, const std::string& newTex);
size_t queuedBoxes();
// Bot/test: answer the message box we are showing as if the player clicked `button` (closes it, then runs the answer sink). false if none.
bool answerShowing(int button, uint32_t* menuId);
// L8 engine glue stubs (unproven in-game until internal lane notes says so):
uint32_t topicOfInfo(uint32_t infoAddr);                    // TESTopicInfo* -> TESTopic* learned from GetTopicInfo (0 = never resolved)
bool sayTopic(uint32_t npcRefID, const FormRef& topic);     // `<npc>.Say <topic>`: plays the NPC's line for that topic (voice + lip sync)
bool resumeTalk(uint32_t npcRefID);                         // deferred vanilla talk -> `<npc>.StartConversation player GREETING`
void setControlsLocked(bool locked);                        // player controls (all-or-nothing, L5's SetPlayerControlsLocked)
bool nearPlayer(uint32_t refID, float maxDist);
// Host: record the SayOnceFlag effect for an INFO this machine's engine has marked said (inside the open capture scope when there is one, else its own).
// false when this machine does not capture (guests) or the hub is off.
bool recordSayOnce(const FormRef& info);             // same cell (or both exterior) and within maxDist game units of the local player
}  // namespace ops

// The one ShowSequencer of this process: slides (SlideClient) and scene narration (L8 glue, through NarrationGate) are ordered against
// each other by the host's stamps; without a shared sequencer a stamped narration line could not be played at all.
ShowSequencer& showSequencer();

// Ending slideshow: the client side is the same on every machine; the host broadcasts through `send` (set once at session start).
SlideClient& slideClient();
SlideHost& slideHost(SlideHost::Send send = nullptr);

// Test hooks (RPC ui.* in ui_module.cpp)
bool installedByName(const std::string& n);

}  // namespace nvui
