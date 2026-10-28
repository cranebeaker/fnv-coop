// Engine glue (L8): translates between the game's dialogue lifecycle and fnvcoop_dlg::DialogueNode.
//
// IEngineApi is everything the glue needs from the running game. plugin/dialogue/engine/engine_xnvse.cpp implements
// it against xNVSE headers (x86 only); engine_test.cpp implements it with a mock, so all the logic here is tested
// natively. Methods the live implementation cannot do yet (addresses unverified by L5) are stubs that report
// themselves via IEngineApi::unsupported() so the first live run tells us exactly what is missing.
//
// Engine -> glue events come from pollMenu(), called once per frame: it reads the DialogMenu (state, choice ListBox, spoken INFO) from
// memory and derives "topics built", "INFO chosen", "line finished" and "menu closed" from the state transitions, so the engine glue
// needs no detour inside the DialogMenu code (dialog_layout.h has the verified layout). The talk veto (onTalkRequested) still comes
// from the activate observer.
#pragma once
#include <cstdio>
#include <string>
#include <vector>
#include "fnvcoop_dlg/adapter.h"
#include "fnvcoop_dlg/facts.h"
#include "fnvcoop_dlg/mutations.h"
#include "fnvcoop_dlg/sayonce.h"
#include "info_reader.h"

namespace fndlg {

using fnvcoop_dlg::CheckKind;
using fnvcoop_dlg::DenyReason;
using fnvcoop_dlg::Event;
using fnvcoop_dlg::FormRef;
using fnvcoop_dlg::PlayerId;

struct IEngineApi {
    virtual ~IEngineApi() = default;
    virtual const IMemory& memory() = 0;
    virtual Addr32 dialogMenu() = 0;                                        // *(Addr32*)0x11D9510, 0 if closed
    virtual bool formRefOf(Addr32 form, FormRef& out) = 0;                  // runtime form -> (plugin, local id)
    virtual bool topicOf(Addr32 info, FormRef& topic, std::string& topicEdid) = 0;
    virtual bool infoRefOf(Addr32 info, FormRef& out) = 0;
    virtual std::string voicePath(Addr32 info, uint32_t responseNumber, Addr32 speakerRef) = 0;  // "" if unknown
    virtual std::vector<fnvcoop_dlg::Fact> collectFacts(Addr32 info) = 0;   // player facts the scripts/conditions read
    virtual CheckKind classifyCheck(Addr32 info, const InfoFlags& f) = 0;

    // --- initiator side
    virtual void resumeTalk(const FormRef& npc) = 0;                        // continue the deferred vanilla talk
    virtual void talkDenied(DenyReason why, PlayerId owner) = 0;            // "X is busy talking to <owner>"
    virtual void rebuildTopicList() = 0;                                    // after End result: build next list
    virtual void forceCloseDialog() = 0;                                    // close DialogMenu, no local end scripts
    virtual void setControls(uint8_t mask, bool locked) = 0;

    // --- spectator side
    virtual void banner(const FormRef& npc, PlayerId initiator) = 0;
    virtual void showTopics(const std::vector<fnvcoop_dlg::TopicEntry>& t) = 0;
    virtual void showSubtitle(const FormRef& npc, const std::string& text, float seconds) = 0;
    virtual void playVoice(const FormRef& npc, const std::string& path) = 0;
    virtual void hideOverlay() = 0;

    // --- forced dialogue (scripted SayTo / StartConversation)
    virtual bool npcAudible(const FormRef& npc) = 0;                         // within hearing/line-of-sight of this player
    virtual void startForcedConversation(const FormRef& npc, const FormRef& topic) = 0;  // native StartConversation on THIS machine

    // --- companions: remove a command topic from the real DialogMenu for a non-owner
    virtual void hideTopic(Addr32 info) = 0;

    // --- menus raised by host-run scripts: opened on the origin player only; everyone else gets a notice
    virtual void openMenu(const fnvcoop_dlg::MenuRequest& rq) = 0;          // ShowMessageBox / barter / wheel / terminal on THIS machine
    virtual void closeMenu(uint32_t menu) = 0;                              // forced close (timeout)
    virtual void menuNotice(const std::string& text, bool closed) = 0;      // read-only notice / its dismissal

    virtual void trace(const std::string&) {}                               // one log line (silent in the tests)
    virtual void unsupported(const char* what) = 0;                         // live stubs report here

    // --- additions (defaults keep older mocks compiling)
    virtual void notice(const std::string&) {}                              // one-line HUD message for the local player
    virtual void markSaid(const FormRef&) {}                                // engine MarkSaid on this machine's copy of the INFO
    virtual void clearSaid(const FormRef&) {}                               // undo a said-mark (per-player lines on the host)
    virtual void frame(uint32_t /*nowMs*/) {}                               // once per poll: timers inside the engine layer (narration gate)
    virtual void commitSayOnce(const FormRef&) {}                           // host: record the SayOnceFlag effect for an INFO this machine's engine has marked said
    virtual int saidState(const FormRef&) { return -1; }                    // the INFO's "said once" byte on THIS machine: 0 / 1, -1 = unknown or not loaded
};

struct GlueConfig {
    bool broadcastHostTalk = true;   // the host player's own conversations are shown to the others (forced-say path)
    uint32_t grantTimeoutMs = 6000;  // a granted conversation whose DialogMenu never opens on this machine is given back to the host after this long
    bool trace = true;               // one log line per pick / line end / offer / menu state change (a conversation has a handful; none while no menu is open)
};

class EngineGlue {
public:
    EngineGlue(IEngineApi& api, ResponseLayout layout = {}) : api_(api), layout_(layout) {}
    void attach(fnvcoop_dlg::DialogueNode* n) {
        node_ = n;
        if (n) n->setSayOnceSink([this](PlayerId from, const FormRef& info) { onGuestSayOnce(from, info); });
    }
    fnvcoop_dlg::DialogueNode::Sink sink() { return [this](const Event& e) { onEvent(e); }; }

    void setLayout(ResponseLayout l) { layout_ = l; }
    const ResponseLayout& layout() const { return layout_; }
    void setConfig(const GlueConfig& c) { cfg_ = c; }
    void setSayOnce(fnvcoop_dlg::SayOncePolicy* p) { sayOnce_ = p; }

    struct Stats {
        uint64_t polls = 0, unreadable = 0, offers = 0, picks = 0, pickRefused = 0, lineDones = 0, leaves = 0, hostBroadcasts = 0,
                 sayMarked = 0, sayCleared = 0, commandRefused = 0, pickLatched = 0,
                 grantTimeouts = 0,   // grants whose menu never opened here, given back (story fix 2026-10-28)
                 engineMarks = 0, sayReported = 0, sayReports = 0, sayCommitted = 0;   // engine marks seen / sent to the host / received by the host / SayOnceFlag effects recorded
    };
    const Stats& stats() const { return stats_; }

    // ---- engine -> glue (L5 hooks / the frame poll call these on the main thread)
    // TESObjectREFR::Activate toward an NPC resulted in "talk". Returns true if vanilla must be VETOED/deferred.
    bool onTalkRequested(const FormRef& npc, uint32_t now) {
        auto& c = node_->client();
        if (c.role() == fnvcoop_dlg::ClientDialogue::Role::Initiator || c.role() == fnvcoop_dlg::ClientDialogue::Role::Requesting)
            return false;  // already in/awaiting a conversation: let vanilla handle repeated activation (menu re-entry)
        pendingNpc_ = npc;
        node_->requestStart(npc, now);
        return true;
    }
    // DialogMenu finished building its topic/choice list.
    void onTopicsBuilt(uint32_t now) {
        MenuView mv;
        if (!api_.dialogMenu() || !readMenu(api_.memory(), api_.dialogMenu(), mv)) return;
        offerFrom(mv, now);
    }
    // The engine marked an INFO said on THIS machine (TESTopicInfo::MarkSaid, the detour in plugin/hooks_ui reports the 0 -> 1 changes that are not our own).
    // Shared lines: a guest tells the host, the host records the SayOnceFlag effect (its own byte is already set). Per-player lines stay local (D11).
    void onEngineMark(const FormRef& info) {
        if (!sayOnce_ || !node_) return;
        const bool host = node_->host() != nullptr;
        const auto r = sayOnce_->onEngineMark(info, host);
        ++stats_.engineMarks;
        trace("dialogue: say-once engine mark " + refText(info) + (host ? " on the host: " : " on a guest: ") +
              (r.kind == fnvcoop_dlg::SayOncePolicy::Route::CommitOnHost ? "commit" : r.kind == fnvcoop_dlg::SayOncePolicy::Route::ReportToHost ? "report to the host" : "kept local"));
        if (r.kind == fnvcoop_dlg::SayOncePolicy::Route::CommitOnHost) { api_.commitSayOnce(info); ++stats_.sayCommitted; }
        else if (r.kind == fnvcoop_dlg::SayOncePolicy::Route::ReportToHost && node_->reportSayOnce(info)) ++stats_.sayReported;
    }
    bool onEngineMark(Addr32 infoAddr) {
        FormRef info;
        if (!infoAddr || !api_.infoRefOf(infoAddr, info)) return false;
        onEngineMark(info);
        return true;
    }
    // Host: a guest's engine marked `info` said and told us.
    void onGuestSayOnce(PlayerId from, const FormRef& info) {
        if (!sayOnce_ || !node_ || !node_->host()) return;
        const auto r = sayOnce_->onGuestReport(info);
        if (r.kind != fnvcoop_dlg::SayOncePolicy::Route::CommitOnHost) {
            trace("dialogue: say-once report " + refText(info) + " from slot " + std::to_string(int(from)) + ": kept with that player");
            return;
        }
        const bool had = api_.saidState(info) == 1;
        trace("dialogue: say-once report " + refText(info) + " from slot " + std::to_string(int(from)) + (had ? ": already set on the host" : ": mark and commit"));
        if (had) return;   // the host's own byte is set (it ran the line, or the shared world already holds it): nothing to commit
        if (!fnvcoop_dlg::mutations().sayOnceMarkHostNoMark) api_.markSaid(info);
        api_.commitSayOnce(info);
        ++stats_.sayCommitted;
        ++stats_.sayReports;
    }

    // The initiator selected an INFO; the engine is about to speak it.
    bool onInfoChosen(Addr32 info, uint32_t now) {
        FormRef topic, infoRef;
        std::string edid;
        InfoFlags f;
        std::vector<fnvcoop_dlg::Line> lines;
        MenuView mv;
        const bool haveMenu = api_.dialogMenu() && readMenu(api_.memory(), api_.dialogMenu(), mv);
        if (!resolveTopic(info, haveMenu ? &mv : nullptr, topic) || !api_.infoRefOf(info, infoRef) || !readInfoFlags(api_.memory(), info, f))
            return false;
        // Companion commands are owner-only: the host rejects the pick anyway, say why instead of silently doing nothing.
        if (!fnvcoop_dlg::mutations().nonOwnerPicksCommand && node_->companions().isCompanion(pendingNpc_) &&
            node_->companions().ownerOf(pendingNpc_) != node_->self() && node_->ownerOnlyTopics().count(topic)) {
            ++stats_.commandRefused;
            api_.notice("Only the player who recruited this companion can give orders.");
            return false;
        }
        if (!readLines(api_.memory(), info, layout_, lines)) { api_.unsupported("response chain unreadable (cache miss and no loader)"); return false; }
        if (haveMenu) {
            for (uint32_t i = 0; i < lines.size(); ++i) lines[i].voice = api_.voicePath(info, i + 1, mv.partnerRef);
        }
        if (sayOnce_) sayOnce_->noteOwnSaid(infoRef);
        lastPickInfo_ = infoRef;
        ++stats_.picks;
        const uint32_t stepBefore = node_->client().step();
        const size_t nLines = lines.size();
        node_->pick(topic, infoRef, -1, f.goodbye, std::move(lines), api_.collectFacts(info), now);
        const bool sent = node_->client().step() != stepBefore;
        if (!sent) ++stats_.pickRefused;
        { char b[160]; std::snprintf(b, sizeof b, "pick %s info %X step %u goodbye %d lines %u", sent ? "sent" : "NOT sent (the last line's result is not in yet)", unsigned(infoRef.localId), unsigned(node_->client().step()), f.goodbye ? 1 : 0, unsigned(nLines)); trace(b); }
        return true;
    }
    void onLineFinished(uint32_t now) {
        { char b[64]; std::snprintf(b, sizeof b, "line done step %u", unsigned(node_->client().step())); trace(b); }
        node_->lineDone(now);
    }
    void onMenuClosedByPlayer(uint32_t now) { node_->leave(now); }
    void onFrame(uint32_t now) { node_->tick(now); pollMenu(now); }
    // The origin player answered (button index) or closed (-1) a menu opened by openMenu().
    void onMenuAnswered(uint32_t menu, int32_t choice, uint32_t now) { node_->menuReply(menu, choice, now); }

    // ---- frame poll: derive the engine events from the DialogMenu
    void pollMenu(uint32_t now) {
        if (!node_) return;
        ++stats_.polls;
        api_.frame(now);
        const Addr32 menu = api_.dialogMenu();
        const auto role = node_->client().role();
        const bool initiator = role == fnvcoop_dlg::ClientDialogue::Role::Initiator;
        // The host granted the conversation but the menu never opened here (the deferred activation did not start it): without this the NPC stays locked on the host
        // for good and this machine stays the initiator, so every later talk is refused as "a conversation is already open".
        if (initiator) {
            const fnvcoop_dlg::ConvId cv = node_->client().conv();
            if (menu) menuSeenConv_ = cv;
            else if (menuSeenConv_ != cv && !fnvcoop_dlg::mutations().grantNoWatchdog) {
                if (grantConv_ != cv) { grantConv_ = cv; grantAt_ = now; }
                else if (now - grantAt_ >= cfg_.grantTimeoutMs) {
                    ++stats_.grantTimeouts;
                    grantAt_ = now;
                    node_->leave(now);
                    api_.unsupported("granted conversation: the DialogMenu never opened, the conversation is given back to the host");
                }
            }
        }
        if (!menu) {                                           // the menu is gone
            if (poll_.open) {
                trace(std::string("menu closed after state ") + std::to_string(poll_.lastState) + " spoken " + idOf(poll_.lastSpoken) + " role " + std::to_string(int(role)));
                const bool afterGoodbye = poll_.lastPickGoodbye && poll_.lineFinished;
                if (initiator && !(afterGoodbye && !fnvcoop_dlg::mutations().pollLeaveAfterGoodbye)) { ++stats_.leaves; onMenuClosedByPlayer(now); }
            }
            poll_ = Poll();
            return;
        }
        MenuView mv;
        if (!readMenu(api_.memory(), menu, mv)) { ++stats_.unreadable; return; }
        using namespace dialog_menu;
        if (!poll_.open) {
            poll_ = Poll();
            poll_.open = true;
            FormRef p;
            if (mv.partnerRef && api_.formRefOf(mv.partnerRef, p)) { poll_.partner = p; if (!initiator) pendingNpc_ = p; }
        }
        if (cfg_.trace && (mv.state != poll_.lastState || mv.spokenInfo != poll_.lastSpoken)) {
            poll_.lastState = mv.state;
            poll_.lastSpoken = mv.spokenInfo;
            trace(std::string("menu state ") + std::to_string(mv.state) + " spoken " + idOf(mv.spokenInfo) + " role " + std::to_string(int(role)) + " rows " + std::to_string(mv.items.size()));
        }
        const bool hostNative = !initiator && role != fnvcoop_dlg::ClientDialogue::Role::Requesting && node_->host() != nullptr &&
                                cfg_.broadcastHostTalk && !fnvcoop_dlg::mutations().hostTalkSilent;
        // 1. the engine built a (new) choice list and waits for a pick
        if (mv.state == kStateIdle) {
            poll_.pickLatch = 0;                               // the menu is waiting for a pick again: the next spoken INFO is a new choice, even the same one
            const uint64_t sig = signature(mv);
            if (!poll_.offered || sig != poll_.listSig) {
                poll_.offered = true;
                poll_.listSig = sig;
                if (initiator) offerFrom(mv, now);
            }
        }
        // 2. a pick: the spoken INFO changed while a line is queued / playing
        const bool speaking = mv.state == kStatePlaying || mv.state == kStateQueued;
        if (speaking && mv.spokenInfo) {
            const bool fresh = !poll_.speaking || mv.spokenInfo != poll_.spoken;
            // The engine can show the same INFO as "spoken" again after its line (a response advance, a click on the row still speaking, a re-read follow-up):
            // that is the line going on, not a second choice. Only the menu returning to idle (a list to pick from) makes the same INFO a new pick.
            const bool again = mv.spokenInfo == poll_.pickLatch && !fnvcoop_dlg::mutations().noPickLatch && !fnvcoop_dlg::mutations().pollPickEveryFrame;
            if (again && fresh) {
                poll_.speaking = true;
                poll_.spoken = mv.spokenInfo;
                ++stats_.pickLatched;
                trace("same INFO " + idOf(mv.spokenInfo) + " spoken again without a list in between: not a new pick");
            } else if (fresh || fnvcoop_dlg::mutations().pollPickEveryFrame) {
                poll_.speaking = true;
                poll_.spoken = mv.spokenInfo;
                poll_.lineFinished = false;
                if (initiator) {
                    if (!fnvcoop_dlg::mutations().pollNoPick) {
                        InfoFlags f;
                        poll_.sentPick = onInfoChosen(mv.spokenInfo, now);
                        if (poll_.sentPick) poll_.pickLatch = mv.spokenInfo;
                        poll_.lastPickGoodbye = readInfoFlags(api_.memory(), mv.spokenInfo, f) && f.goodbye;
                    }
                } else if (hostNative) {
                    broadcastNative(mv, now);
                }
            }
        }
        // 3. the line ended (or the menu started to close): the end script of that INFO is due
        if (poll_.speaking && !speaking) {
            poll_.speaking = false;
            poll_.lineFinished = true;
            poll_.offered = false;                              // the engine builds a fresh list next
            if (initiator && poll_.sentPick && !fnvcoop_dlg::mutations().pollNoLineDone) { ++stats_.lineDones; onLineFinished(now); }
            poll_.sentPick = false;
        }
    }

    // ---- node -> engine
    void onEvent(const Event& e) {
        using K = Event::Kind;
        switch (e.kind) {
            case K::OpenMenu: api_.resumeTalk(pendingNpc_); break;
            case K::Denied: api_.talkDenied(e.deny, e.owner); break;
            case K::SpectatorOpened: {
                const auto& o = std::get<fnvcoop_dlg::Opened>(e.msg);
                npcOfSpectated_ = o.npc;
                api_.banner(o.npc, o.initiator);
                break;
            }
            case K::SpectatorOffer: api_.showTopics(std::get<fnvcoop_dlg::Offer>(e.msg).topics); break;
            case K::ShowLines: {
                const auto& l = std::get<fnvcoop_dlg::Lines>(e.msg);
                api_.trace(std::string("dialogue: lines of ") + l.info.plugin + " reach this " + (node_->host() ? "host" : "machine"));
                for (const auto& ln : l.lines) {
                    api_.showSubtitle(npcOfSpectated_, ln.text, subtitleSeconds(ln.text));
                    if (!ln.voice.empty()) api_.playVoice(npcOfSpectated_, ln.voice);
                }
                applySayOnce(l.info);
                break;
            }
            case K::TopicsReady: api_.rebuildTopicList(); break;
            case K::ControlLock: {
                const auto& c = std::get<fnvcoop_dlg::Control>(e.msg);
                api_.setControls(c.lockMask, c.locked);
                break;
            }
            case K::CloseMenu:
                // Initiator: the engine closes the menu itself after a goodbye line; every other reason is forced.
                if (e.wasInitiator && e.reason != fnvcoop_dlg::CloseReason::Goodbye) api_.forceCloseDialog();
                if (e.wasInitiator && e.reason == fnvcoop_dlg::CloseReason::ConsentDenied && !fnvcoop_dlg::mutations().denyKeepsSaid) {
                    // the party refused the fork: the engine already spoke and marked the line, give the player their choice back
                    api_.notice("The party did not agree to that choice.");
                    if (!lastPickInfo_.plugin.empty()) api_.clearSaid(lastPickInfo_);
                }
                if (!e.wasInitiator) api_.hideOverlay();
                break;
            case K::MenuShow: api_.openMenu(std::get<fnvcoop_dlg::MenuRequest>(e.msg)); break;
            case K::MenuNoticeShow: {
                const auto& n = std::get<fnvcoop_dlg::MenuNotice>(e.msg);
                api_.menuNotice("Player " + std::to_string(int(n.origin)) + ": " + n.text, false);
                break;
            }
            case K::MenuEnd: {
                const auto& c = std::get<fnvcoop_dlg::MenuClosed>(e.msg);
                if (e.timedOut) api_.closeMenu(c.menu);
                api_.menuNotice(c.label, true);
                break;
            }
            case K::ForcedSayShow: {
                const auto& f = std::get<fnvcoop_dlg::ForcedLines>(e.msg);
                // The host's engine ran the script (or the host player's own conversation) natively: it has already played the line.
                if (node_->host() && !fnvcoop_dlg::mutations().forcedHostReplays) break;
                // The target and cutscene viewers always get it; everybody else only if they could hear the NPC.
                const bool mine = f.target == node_->self() || f.scene;
                if (!mine && !api_.npcAudible(f.npc)) break;
                for (const auto& ln : f.lines) {
                    api_.showSubtitle(f.npc, ln.text, subtitleSeconds(ln.text));
                    if (!ln.voice.empty()) api_.playVoice(f.npc, ln.voice);
                }
                // Same shared say-once rule as a normal conversation, for the machines that did not run the INFO themselves (guests).
                if (!node_->host()) applySayOnce(f.info);
                break;
            }
            case K::ForcedOpen: {
                const auto& fs = std::get<fnvcoop_dlg::ForcedStart>(e.msg);
                pendingNpc_ = fs.npc;                                         // companion/topic filtering needs the NPC
                api_.startForcedConversation(fs.npc, fs.topic);
                break;
            }
            case K::Send: break;
        }
    }

    static float subtitleSeconds(const std::string& text) {  // reading time floor 2 s, ~15 chars/s
        float s = float(text.size()) / 15.0f;
        return s < 2.0f ? 2.0f : s;
    }

private:
    struct Poll {
        bool open = false, offered = false, speaking = false, sentPick = false, lineFinished = false, lastPickGoodbye = false;
        uint64_t listSig = 0;
        Addr32 spoken = 0;
        FormRef partner;
        Addr32 pickLatch = 0;               // the INFO already reported as a pick; the same INFO is not a new pick until the menu waited for one again (state idle)
        uint32_t lastState = 0xFFFFFFFFu;   // trace only: the state / spoken INFO last logged
        Addr32 lastSpoken = 0;
    };

    static uint64_t signature(const MenuView& mv) {   // order-sensitive hash of the offered INFOs
        uint64_t h = 1469598103934665603ull;
        for (const MenuItem& it : mv.items) { h ^= it.info; h *= 1099511628211ull; }
        return h ^ mv.items.size();
    }
    // the TESTopic of an offered INFO: from the ListBox row, else through the engine API
    bool resolveTopic(Addr32 info, const MenuView* mv, FormRef& topic) {
        std::string edid;
        if (mv) {
            for (const MenuItem& it : mv->items)
                if (it.info == info && it.topic && api_.formRefOf(it.topic, topic)) return true;
        }
        return api_.topicOf(info, topic, edid);
    }
    void offerFrom(const MenuView& mv, uint32_t now) {
        std::vector<fnvcoop_dlg::TopicEntry> t;
        const bool companion = node_->companions().isCompanion(pendingNpc_);
        const PlayerId owner = node_->companions().ownerOf(pendingNpc_);
        for (const MenuItem& item : mv.items) {
            fnvcoop_dlg::TopicEntry e;
            InfoFlags f;
            if (!resolveTopic(item.info, &mv, e.topic)) continue;
            // Companion commands are owner-only: a non-owner never sees them (host also rejects the pick).
            if (companion && owner != node_->self() && node_->ownerOnlyTopics().count(e.topic)) { api_.hideTopic(item.info); continue; }
            if (readInfoFlags(api_.memory(), item.info, f)) {
                e.check = uint8_t(api_.classifyCheck(item.info, f));
                e.flags = uint8_t(f.saidOnce ? 1 : 0);
            }
            t.push_back(e);
        }
        ++stats_.offers;
        if (cfg_.trace) {
            std::string ids;
            for (const MenuItem& item : mv.items) { ids += ' '; ids += idOf(item.info); }
            trace("offer " + std::to_string(t.size()) + " of " + std::to_string(mv.items.size()) + " rows, infos:" + ids);
        }
        node_->offer(std::move(t), now);
    }
    // The host player talks natively (the engine ran everything): show the lines to the others like a scripted SayTo.
    void broadcastNative(const MenuView& mv, uint32_t now) {
        FormRef topic, info;
        std::vector<fnvcoop_dlg::Line> lines;
        if (!api_.infoRefOf(mv.spokenInfo, info) || poll_.partner.plugin.empty()) return;
        resolveTopic(mv.spokenInfo, &mv, topic);
        if (!readLines(api_.memory(), mv.spokenInfo, layout_, lines)) return;
        for (uint32_t i = 0; i < lines.size(); ++i) lines[i].voice = api_.voicePath(mv.spokenInfo, i + 1, mv.partnerRef);
        if (sayOnce_) sayOnce_->noteOwnSaid(info);
        ++stats_.hostBroadcasts;
        node_->forcedSay(poll_.partner, topic, info, std::move(lines), node_->self(), false, 0, now);
    }
    void applySayOnce(const FormRef& info) {
        if (!sayOnce_ || info.plugin.empty()) return;
        const auto a = sayOnce_->onSpectated(info, node_->host() != nullptr);
        api_.trace("dialogue: say-once " + std::string(a.kind == fnvcoop_dlg::SayOnceAction::Mark ? "mark" : a.kind == fnvcoop_dlg::SayOnceAction::Clear ? "clear" : "keep") + " " + info.plugin + ":" +
                   [&] { char b[16]; std::snprintf(b, sizeof b, "%X", unsigned(info.localId)); return std::string(b); }() + (node_->host() ? " on the host" : " on a guest"));
        if (a.kind == fnvcoop_dlg::SayOnceAction::Mark) { api_.markSaid(info); ++stats_.sayMarked; }
        else if (a.kind == fnvcoop_dlg::SayOnceAction::Clear) { api_.clearSaid(info); ++stats_.sayCleared; }
    }

    static std::string refText(const FormRef& r) {
        char b[16];
        std::snprintf(b, sizeof b, "%X", unsigned(r.localId));
        return r.plugin + ":" + b;
    }
    // the id of an engine INFO / row for the trace: the plugin-local id in hex, "?" when the engine form has no stable name
    std::string idOf(Addr32 form) {
        FormRef r;
        char b[24];
        if (!form || !api_.formRefOf(form, r)) return "-";
        std::snprintf(b, sizeof b, "%X", unsigned(r.localId));
        return b;
    }
    void trace(const std::string& s) { if (cfg_.trace) api_.trace(s); }

    IEngineApi& api_;
    ResponseLayout layout_;
    GlueConfig cfg_;
    fnvcoop_dlg::DialogueNode* node_ = nullptr;
    fnvcoop_dlg::SayOncePolicy* sayOnce_ = nullptr;
    FormRef pendingNpc_, npcOfSpectated_, lastPickInfo_;
    Poll poll_;
    Stats stats_;
    fnvcoop_dlg::ConvId menuSeenConv_ = 0, grantConv_ = 0;   // the conversation whose menu was seen / whose grant is being timed
    uint32_t grantAt_ = 0;
};

}  // namespace fndlg
