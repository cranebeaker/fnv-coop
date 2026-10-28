// Host-side conversation state machine (L8). Game-free, header-only. One instance per host process.
//
// The host is the lock manager and the only place result scripts run. It never evaluates conditions: the initiator
// resolves topic/INFO selection (it owns the player facts), the host validates ordering/ownership, runs the
// begin/end result scripts through the injected ResultRunner (-> one EffectTxn each) and fans out to everyone.
#pragma once
#include <functional>
#include <map>
#include <set>
#include <vector>
#include "fnvcoop_dlg/companions.h"
#include "fnvcoop_dlg/messages.h"
#include "fnvcoop_dlg/mutations.h"

namespace fnvcoop_dlg {

struct Out {
    uint8_t to = 0;  // bit i = slot i
    Msg msg;
};

struct ResultRequest {
    ConvId conv = 0;
    uint32_t step = 0;
    Phase phase = Phase::Begin;
    PlayerId initiator = 0;  // script "player" context: Recipient::origin() effects go to this slot
    FormRef npc, info;
    const std::vector<Fact>* facts = nullptr;  // initiator-observed player facts (may be empty)
};
// Runs the INFO's begin/end result script on the host under effect capture; returns the committed txn seq
// (0 = no effects). Called exactly once per (conv, step, phase).
using ResultRunner = std::function<uint64_t(const ResultRequest&)>;

class HostDialogue {
public:
    struct Config {
        uint32_t leaseMs = 300000;  // initiator silent this long -> Timeout (heartbeats refresh it)
        size_t maxConversations = 8;
        uint32_t messageMenuMs = 120000;    // message boxes / terminals: take the default choice after this
        uint32_t serviceMenuMs = 600000;    // barter, repair, caravan, wheel, hacking...
    };

    // No `Config cfg = {}` default argument: a nested type with member initializers is not complete there for GCC and clang.
    explicit HostDialogue(ResultRunner run) : HostDialogue(std::move(run), Config()) {}
    explicit HostDialogue(ResultRunner run, Config cfg) : run_(std::move(run)), cfg_(cfg) {}

    // ---- companions: owner-only command topics + owner takeover (see companions.h). Not owned; may be null.
    void setCompanions(CompanionRegistry* reg, std::set<FormRef> ownerOnlyTopics) { comp_ = reg; ownerOnly_ = std::move(ownerOnlyTopics); }
    const std::vector<FormRef>& lastReleased() const { return released_; }

    // ---- menus raised by host-run scripts. The script is suspended by the engine glue until `resume` fires
    // (exactly once per menu: reply, timeout, or the origin leaving).
    using MenuResume = std::function<void(uint32_t menu, int32_t choice, bool timedOut)>;
    void setMenuResume(MenuResume r) { menuResume_ = std::move(r); }

    // ---- ModalChoice options for raiseMenu (act2 rule kind ModalChoice: decider=origin, consent=party|none, timeout_s)
    // D3: how an irreversible choice is decided. Unanimous = a point of no return: every other player must answer "Ready" (one "Not yet"
    // or the timeout cancels). Vote = a story fork: the party votes, the origin counts as agreeing, the majority wins and the HOST breaks
    // a tie. Actor = a personal action (Caesar's surgery): the origin alone decides, nobody is asked.
    // Rules = 3: the rules engine (ChoiceConsent) owns the vote; the host only carries the question and the answers (see "votes owned by the rules engine" below).
    enum class ConsentMode : uint8_t { Unanimous = 0, Vote = 1, Actor = 2, Rules = 3 };
    struct ModalOptions {
        bool partyConsent = false;   // after the origin picks `confirmButton`, every other connected player must answer "Ready"
        int32_t confirmButton = 0;   // the "point of no return" button
        int32_t declineChoice = -1;  // what the script gets when anybody declines or the consent window times out
        uint32_t timeoutMs = 0;      // 0 = Config defaults
        ConsentMode mode = ConsentMode::Unanimous;
        bool anyButton = false;      // every button except declineChoice is the irreversible one (VMS42: two ways to blow up the caves)
        FormRef rulesChoice;         // ConsentMode::Rules: the message box the rules engine's ChoiceConsent rule names (RulesEngine::onChoiceRequested)
    };
    // Fork INFOs (Joshua or Daniel, Elijah's offer): the pick is held before any script runs, the party is asked, then it proceeds or
    // the conversation closes with CloseReason::ConsentDenied.
    struct ChoiceRule { ConsentMode mode = ConsentMode::Vote; uint32_t timeoutMs = 0; };
    void setChoiceRules(std::map<FormRef, ChoiceRule> r) { choiceRules_ = std::move(r); }
    size_t choiceRuleCount() const { return choiceRules_.size(); }
    // The rules engine took over an INFO's vote (RulesEngine::isChoice): the old hold at the pick must not ask the party a second time.
    void dropChoiceRule(const FormRef& info) {
        if (!mutations().rulesOwnerKeepsOldHold) choiceRules_.erase(info);
    }
    std::vector<FormRef> choiceRuleInfos() const {
        std::vector<FormRef> v;
        for (const auto& kv : choiceRules_) v.push_back(kv.first);
        return v;
    }

    // ---- votes owned by the rules engine (campaign wave 1b: ReadyCheck and ChoiceConsent, G27 / G08) ----------------------------------------------------------
    // RulesEngine decides who must agree and how the vote is counted; this class only carries the question to the other connected players and their answers back.
    //   * An INFO's END script is held when the info gate says Hold (RulesEngine::onInfoResult == Deferred): the script does not run and the conversation waits at that
    //     line. A ReadyCheck INFO (House, the Yes Man topics) and a ChoiceConsent INFO (Daniel, Elijah) both come through here.
    //   * A message box whose irreversible button the origin pressed (ModalOptions::mode == Rules + rulesChoice) is held the same way (box gate).
    // The other players get a two-button box (0 Agree = yes, 1 Not yet = no, MenuRequest::consent) and every answer goes to the vote sink: the rules engine counts a yes as
    // ScenePass of the ref and a no as ScenePass of choiceDeclineId(ref). rulesResolved(ref, accepted) is the engine's verdict (onReadyResolved / onChoiceResolved): an
    // accepted INFO runs its END script and continues, a declined one is dropped and the conversation closes with ConsentDenied (the origin is told); an accepted box
    // resumes its script with the origin's button, a declined one with the decline choice. Without gates nothing changes.
    enum class Gate : uint8_t { Run, Hold };
    using RulesGate = std::function<Gate(const FormRef& ref, PlayerId origin)>;
    using VoteSink = std::function<void(PlayerId slot, const FormRef& ref, bool yes)>;
    void setRulesGates(RulesGate info, RulesGate box) { infoGate_ = std::move(info); boxGate_ = std::move(box); }
    void setVoteSink(VoteSink s) { voteSink_ = std::move(s); }
    // The verdict for a held vote. Unknown / already released refs are ignored (a duplicate, or the origin left meanwhile).
    std::vector<Out> rulesResolved(const FormRef& ref, bool accepted, uint32_t now) {
        std::vector<Out> o;
        for (auto it = menus_.begin(); it != menus_.end(); ++it)
            if (it->second.rulesWait && it->second.rulesRef == ref) { rulesDone(it, accepted, now, o); break; }
        return o;
    }
    size_t heldByRules() const {
        size_t n = 0;
        for (const auto& kv : menus_) if (kv.second.rulesWait) ++n;
        return n;
    }

    // ---- forced dialogue: scripted SayTo / StartConversation. Called by the host hook after the script command ran.
    using ActorChooser = std::function<PlayerId(const FormRef& npc, uint8_t connectedMask)>;
    void setActorChooser(ActorChooser c) { chooser_ = std::move(c); }
    // One-way scripted line (SayTo player / Say). `target` = the player it is aimed at (kNoPlayer = party), `scene` = cutscene
    // (everybody must see it). `lines` are already resolved against the target's facts; `txnSeq` = the capture txn of the INFO's
    // result scripts. Delivered to every connected player; receivers decide audibility unless `scene`.
    std::vector<Out> forcedSay(const FormRef& npc, const FormRef& topic, const FormRef& info, std::vector<Line> lines,
                               PlayerId target, bool scene, uint64_t txnSeq) {
        std::vector<Out> o;
        ForcedLines f; f.id = ++nextForced_; f.kind = ForcedKind::Say; f.npc = npc; f.topic = topic; f.info = info;
        f.target = target; f.scene = scene; f.txnSeq = txnSeq; f.lines = std::move(lines);
        if (connected_) o.push_back({connected_, std::move(f)});
        return o;
    }
    // StartConversation: the NPC walks up and opens the real DialogMenu for the acting player (`target`, or the chooser's pick for
    // a party scene). A scripted conversation displaces whatever the NPC or the actor were doing. *actorOut = chosen actor.
    std::vector<Out> forceConversation(const FormRef& npc, const FormRef& topic, PlayerId target, uint32_t now,
                                       PlayerId* actorOut = nullptr) {
        std::vector<Out> o;
        if (actorOut) *actorOut = fnvcoop::kNoPlayer;
        if (dead_.count(npc)) return o;
        PlayerId actor = (target < 8 && (connected_ & bit(target))) ? target : fnvcoop::kNoPlayer;
        if (actor == fnvcoop::kNoPlayer) {
            if (chooser_) actor = chooser_(npc, connected_);
            if (actor >= 8 || !(connected_ & bit(actor))) {
                actor = fnvcoop::kNoPlayer;
                for (PlayerId p = 0; p < 8; ++p) if (connected_ & bit(p)) { actor = p; break; }
            }
        }
        if (mutations().forcedAlwaysHost && (connected_ & bit(fnvcoop::kHostPlayer))) actor = fnvcoop::kHostPlayer;
        if (actor == fnvcoop::kNoPlayer) return o;
        if (mutations().forcedNoPreempt) {
            if (byNpc(npc) || !byInitiator(actor).empty()) return o;
        } else {
            std::vector<ConvId> displaced;
            if (auto* c = byNpc(npc)) displaced.push_back(c->id);
            for (auto* c : byInitiator(actor)) displaced.push_back(c->id);
            for (ConvId id : displaced) if (auto* c = find(id)) closeConv(*c, CloseReason::Preempted, now, o);
        }
        Conv c;
        c.id = ++nextConv_; c.npc = npc; c.initiator = actor; c.reqId = 0; c.lastActive = now;
        npcLock_[npc] = c.id;
        ForcedStart fs; fs.id = ++nextForced_; fs.conv = c.id; fs.npc = npc; fs.topic = topic;
        o.push_back({connected_, Opened{c.id, 0, npc, actor}});
        o.push_back({mutations().forcedStartToAll ? connected_ : bit(actor), fs});
        convs_[c.id] = std::move(c);
        if (actorOut) *actorOut = actor;
        return o;
    }
    // `origin` = the player whose context the script runs in (ResultRequest.initiator / the activator), NOT the host
    // player by default. Returns the menu id via *menuOut (0 = resumed immediately because the origin is gone).
    std::vector<Out> raiseMenu(PlayerId origin, MenuKind kind, const std::string& text, std::vector<std::string> buttons,
                               int32_t defaultChoice, const FormRef& subject, uint32_t now, uint32_t* menuOut = nullptr) {
        return raiseMenu(origin, kind, text, std::move(buttons), defaultChoice, subject, now, menuOut, ModalOptions());
    }
    std::vector<Out> raiseMenu(PlayerId origin, MenuKind kind, const std::string& text, std::vector<std::string> buttons,
                               int32_t defaultChoice, const FormRef& subject, uint32_t now, uint32_t* menuOut,
                               const ModalOptions& opt) {
        std::vector<Out> o;
        uint32_t id = ++nextMenu_;
        if (menuOut) *menuOut = id;
        if (!(connected_ & bit(origin))) {  // origin vanished before we could ask: take the default, no UI anywhere
            if (!mutations().noMenuDefault && menuResume_) menuResume_(id, defaultChoice, true);
            return o;
        }
        PendingMenu pm;
        pm.id = id; pm.origin = origin; pm.kind = kind; pm.buttons = buttons; pm.defaultChoice = defaultChoice;
        bool service = kind != MenuKind::MessageBox && kind != MenuKind::Terminal;
        pm.deadline = now + (opt.timeoutMs ? opt.timeoutMs : (service ? cfg_.serviceMenuMs : cfg_.messageMenuMs));
        pm.opt = opt; pm.text = text;
        PlayerId target = mutations().menuOnHost ? fnvcoop::kHostPlayer : origin;
        MenuRequest rq; rq.menu = id; rq.kind = kind; rq.origin = origin; rq.text = text; rq.buttons = std::move(buttons);
        rq.defaultChoice = defaultChoice; rq.subject = subject;
        uint8_t reqTo = mutations().menuToSpectators ? connected_ : bit(target);
        o.push_back({reqTo, rq});
        uint8_t spect = uint8_t(connected_ & ~bit(origin));
        if (spect) o.push_back({spect, MenuNotice{id, kind, origin, text.substr(0, 160)}});
        for (auto* c : byInitiator(origin)) c->lastActive = now;   // a menu is activity: keep the conversation alive
        menus_[id] = std::move(pm);
        return o;
    }
    size_t pendingMenus() const { return menus_.size(); }

    // A player joined (or rejoined) the session. Returns the catch-up for them: for every open conversation Opened, then the latest
    // Lines (step > 0) and, while the initiator is choosing, the latest Offer. Without it a late joiner never learns a conversation
    // is running and ignores its Lines and Closed.
    std::vector<Out> playerJoined(PlayerId p) {
        std::vector<Out> o;
        connected_ |= bit(p);
        if (mutations().noJoinCatchUp) return o;
        for (auto& kv : convs_) {
            const Conv& c = kv.second;
            if (c.initiator == p) continue;
            o.push_back({bit(p), Opened{c.id, c.reqId, c.npc, c.initiator}});
            if (c.hasLines) o.push_back({bit(p), c.lastLines});
            if (c.st == St::AwaitPick && c.hasOffer) o.push_back({bit(p), c.lastOffer});
        }
        return o;
    }
    // Initiator dropped -> close (flushing a begun line's end script); spectators just stop receiving.
    std::vector<Out> playerLeft(PlayerId p, uint32_t now) {
        std::vector<Out> o;
        connected_ &= ~bit(p);
        for (auto* c : byInitiator(p)) closeConv(*c, CloseReason::Disconnect, now, o);
        expireMenus(now, o, p);
        released_.clear();
        if (comp_) released_ = comp_->releaseOwner(p);
        return o;
    }

    std::vector<Out> onMessage(PlayerId from, const Msg& m, uint32_t now) {
        std::vector<Out> o;
        std::visit([&](const auto& x) { handle(from, x, now, o); }, m);
        return o;
    }

    std::vector<Out> npcDied(const FormRef& npc, uint32_t now) {
        std::vector<Out> o;
        dead_.insert(npc);
        if (auto* c = byNpc(npc)) closeConv(*c, CloseReason::NpcDead, now, o);
        return o;
    }
    std::vector<Out> npcCombat(const FormRef& npc, bool inCombat, uint32_t now) {
        std::vector<Out> o;
        if (inCombat) {
            combat_.insert(npc);
            if (auto* c = byNpc(npc)) closeConv(*c, CloseReason::Combat, now, o);
        } else {
            combat_.erase(npc);
        }
        return o;
    }
    // The initiator (or the NPC) entered combat: the engine closes the DialogMenu itself.
    std::vector<Out> initiatorCombat(PlayerId p, uint32_t now) {
        std::vector<Out> o;
        for (auto* c : byInitiator(p)) closeConv(*c, CloseReason::Combat, now, o);
        return o;
    }
    // INFO script called Disable/EnablePlayerControls (captured as a control effect by L5).
    std::vector<Out> controlLock(ConvId conv, uint8_t mask, bool locked) {
        std::vector<Out> o;
        auto it = convs_.find(conv);
        if (it == convs_.end()) return o;
        it->second.locks = locked ? uint8_t(it->second.locks | mask) : uint8_t(it->second.locks & ~mask);
        o.push_back({bit(it->second.initiator), Control{conv, mask, locked}});
        return o;
    }
    std::vector<Out> shutdown(uint32_t now) {
        std::vector<Out> o;
        std::vector<ConvId> ids;
        for (auto& kv : convs_) ids.push_back(kv.first);
        for (auto id : ids) closeConv(convs_.at(id), CloseReason::HostShutdown, now, o);
        return o;
    }
    std::vector<Out> tick(uint32_t now) {
        std::vector<Out> o;
        std::vector<ConvId> ids;
        for (auto& kv : convs_) if (now - kv.second.lastActive >= cfg_.leaseMs) ids.push_back(kv.first);
        for (auto id : ids) closeConv(convs_.at(id), CloseReason::Timeout, now, o);
        expireMenus(now, o, fnvcoop::kNoPlayer);
        return o;
    }

    // ---- introspection (tests, nvbridge `dialogue` state)
    bool npcBusy(const FormRef& npc) const { return npcLock_.count(npc) != 0; }
    size_t open() const { return convs_.size(); }
    bool controlsLocked(ConvId c) const { auto it = convs_.find(c); return it != convs_.end() && it->second.locks; }
    PlayerId initiatorOf(ConvId c) const { auto it = convs_.find(c); return it == convs_.end() ? fnvcoop::kNoPlayer : it->second.initiator; }
    uint32_t scriptRuns() const { return runs_; }
    uint32_t rejected() const { return rejected_; }
    uint32_t repeatPicks() const { return repeatPicks_; }   // picks answered without running (the same INFO again, no list offered in between)

private:
    enum class St : uint8_t { AwaitPick, Playing, Consenting, HoldingEnd };
    struct PendingMenu {
        uint32_t id = 0; PlayerId origin = 0; MenuKind kind = MenuKind::MessageBox;
        std::vector<std::string> buttons; int32_t defaultChoice = -1; uint32_t deadline = 0;
        ModalOptions opt; std::string text; bool consenting = false; uint8_t need = 0, yes = 0, no = 0;
        int32_t picked = -1;                       // the origin's button, resumed on approval
        ConvId heldConv = 0;                       // != 0: a held fork INFO pick, not a script menu (nothing to resume)
        // wave 1b: a vote the rules engine owns (rulesWait). endConv != 0: it holds an INFO's END script (the data below is what the script needs), else a message box.
        bool rulesWait = false;
        FormRef rulesRef;
        ConvId endConv = 0; uint32_t endStep = 0; FormRef endNpc; std::vector<Fact> endFacts; bool endGoodbye = false;
    };
    struct Conv {
        ConvId id = 0; FormRef npc; PlayerId initiator = 0; uint32_t reqId = 0;
        St st = St::AwaitPick; uint32_t step = 0; bool goodbye = false; FormRef info;
        std::vector<Fact> facts; uint64_t beginTxn = 0, endTxn = 0; bool endDone = true;
        uint8_t locks = 0; uint32_t lastActive = 0;
        Lines lastLines; bool hasLines = false; Offer lastOffer; bool hasOffer = false;   // catch-up for late joiners
        Pick held; bool lineDoneEarly = false;                                           // a fork pick waiting for the party's answer
        bool gateAsked = false, endHeld = false;                                         // wave 1b: the rules engine was asked about the END script / said Hold
        bool offeredSince = false;   // the initiator offered a topic list since its last pick (a pick of the same INFO as the last one is only real after one)
        bool silent = false;         // this step is a repeat of the previous pick (no list in between): answered, never run, never shown again
    };
    static uint8_t bit(PlayerId p) { return p < 8 ? uint8_t(1u << p) : uint8_t(0); }

    Conv* byNpc(const FormRef& n) {
        auto it = npcLock_.find(n);
        return it == npcLock_.end() ? nullptr : find(it->second);
    }
    std::vector<Conv*> byInitiator(PlayerId p) {
        std::vector<Conv*> v;
        for (auto& kv : convs_) if (kv.second.initiator == p) v.push_back(&kv.second);
        return v;
    }
    Conv* find(ConvId id) { auto it = convs_.find(id); return it == convs_.end() ? nullptr : &it->second; }
    bool isInitiator(PlayerId from, const Conv& c) const { return mutations().anyonePicks || from == c.initiator; }
    uint8_t others(const Conv& c) const { return uint8_t(connected_ & ~bit(c.initiator)); }

    uint64_t runScript(Conv& c, Phase ph) {
        ++runs_;
        ResultRequest rq{c.id, c.step, ph, c.initiator, c.npc, c.info, &c.facts};
        return run_ ? run_(rq) : 0;
    }
    void runEnd(Conv& c, std::vector<Out>& o) {
        if (c.endDone || c.endHeld) return;
        if (c.silent) {   // the repeat of a pick: the first one's END script ran (or runs) once; this step only answers
            c.endDone = true;
            o.push_back({connected_, Result{c.id, c.step, Phase::End, 0}});
            return;
        }
        if (infoGate_ && !c.gateAsked && !mutations().noInfoGate) {   // G27: a ReadyCheck / ChoiceConsent INFO asks the party before its END script
            c.gateAsked = true;
            if (infoGate_(c.info, c.initiator) == Gate::Hold) { holdEnd(c, o); return; }
        }
        c.endTxn = runScript(c, Phase::End);
        c.endDone = true;
        o.push_back({connected_, Result{c.id, c.step, Phase::End, c.endTxn}});
    }
    void closeConv(Conv& c, CloseReason r, uint32_t, std::vector<Out>& o) {
        // Finish what started: a begun line's end script still runs, so quest state never half-applies.
        if (!mutations().dropEndOnClose) runEnd(c, o);
        if (c.locks && !mutations().skipControlRelease) {
            o.push_back({bit(c.initiator), Control{c.id, c.locks, false}});
            c.locks = 0;
        }
        o.push_back({connected_, Closed{c.id, r}});
        ConvId id = c.id;
        if (!mutations().leakLockOnClose) npcLock_.erase(c.npc);
        convs_.erase(id);
    }

    void handle(PlayerId from, const StartReq& m, uint32_t now, std::vector<Out>& o) {
        auto mine = byInitiator(from);
        if (!mine.empty()) {
            Conv* c = mine.front();
            if (c->reqId == m.reqId && c->npc == m.npc)  // retransmit: resend the grant
                o.push_back({connected_, Opened{c->id, c->reqId, c->npc, c->initiator}});
            else
                o.push_back({bit(from), StartDenied{m.reqId, DenyReason::AlreadyTalking, from}});
            return;
        }
        if (dead_.count(m.npc)) { o.push_back({bit(from), StartDenied{m.reqId, DenyReason::NpcDead, fnvcoop::kNoPlayer}}); return; }
        if (combat_.count(m.npc)) { o.push_back({bit(from), StartDenied{m.reqId, DenyReason::NpcInCombat, fnvcoop::kNoPlayer}}); return; }
        if (!mutations().noNpcLock) {
            if (auto* other = byNpc(m.npc)) {
                // The owner of a companion may take it over from someone who is merely chatting with it.
                if (comp_ && !mutations().noPreempt && other->initiator != from && comp_->ownerOf(m.npc) == from) {
                    closeConv(*other, CloseReason::Preempted, now, o);
                } else {
                    o.push_back({bit(from), StartDenied{m.reqId, DenyReason::Busy, other->initiator}});
                    return;
                }
            }
        }
        if (convs_.size() >= cfg_.maxConversations) { o.push_back({bit(from), StartDenied{m.reqId, DenyReason::TooMany, fnvcoop::kNoPlayer}}); return; }
        Conv c;
        c.id = ++nextConv_; c.npc = m.npc; c.initiator = from; c.reqId = m.reqId; c.lastActive = now;
        npcLock_[m.npc] = c.id;
        o.push_back({connected_, Opened{c.id, c.reqId, c.npc, c.initiator}});
        convs_[c.id] = std::move(c);
    }
    void handle(PlayerId from, const Offer& m, uint32_t now, std::vector<Out>& o) {
        Conv* c = find(m.conv);
        if (!c || !isInitiator(from, *c)) { ++rejected_; return; }
        c->lastActive = now;
        if (c->st != St::AwaitPick) { ++rejected_; return; }
        Offer shown = m;
        if (comp_ && comp_->isCompanion(c->npc) && !mutations().spectatorsSeeCommands) {
            // command topics belong to the owner alone: nobody else is told they are on offer
            shown.topics.clear();
            for (auto& t : m.topics) if (!ownerOnly_.count(t.topic)) shown.topics.push_back(t);
        }
        c->lastOffer = shown; c->hasOffer = true;
        c->offeredSince = true;
        o.push_back({others(*c), shown});
    }
    void handle(PlayerId from, const Pick& m, uint32_t now, std::vector<Out>& o) {
        Conv* c = find(m.conv);
        if (!c || !isInitiator(from, *c)) { ++rejected_; return; }
        c->lastActive = now;
        if (m.step == c->step && c->step != 0) {  // duplicate delivery: idempotent
            if (c->st == St::Consenting) return;      // still waiting for the party: no Result yet
            if (mutations().replayDuplicatePick) c->beginTxn = runScript(*c, Phase::Begin);
            o.push_back({connected_, Result{c->id, c->step, Phase::Begin, c->beginTxn}});
            return;
        }
        if (c->st != St::AwaitPick || m.step != c->step + 1) { ++rejected_; return; }
        const bool command = comp_ && comp_->isCompanion(c->npc) && ownerOnly_.count(m.topic) != 0;
        if (command && comp_->ownerOf(c->npc) != c->initiator && !mutations().nonOwnerCommands) { ++rejected_; return; }
        // The same INFO again with no topic list offered since the last pick is not a new choice: the engine's menu has not been back at "pick a line"
        // (a line playing on, a follow-up chain re-read, a click on a row still speaking). Answer the step, run nothing, show nothing.
        if (c->step != 0 && m.info == c->info && !c->offeredSince && !mutations().repeatPickRuns) {
            ++repeatPicks_;
            c->step = m.step; c->st = St::Playing; c->goodbye = m.goodbye; c->silent = true; c->endDone = false; c->beginTxn = 0; c->endTxn = 0;
            o.push_back({connected_, Result{c->id, c->step, Phase::Begin, 0}});
            return;
        }
        auto rule = choiceRules_.find(m.info);
        if (rule != choiceRules_.end() && rule->second.mode != ConsentMode::Actor && others(*c) && !mutations().noChoiceConsent) {
            holdPick(*c, m, rule->second, now, o);        // irreversible fork: ask the party BEFORE any script runs
            return;
        }
        acceptPick(*c, m, command, o);
    }
    void acceptPick(Conv& cv, const Pick& m, bool command, std::vector<Out>& o) {
        Conv* c = &cv;
        c->step = m.step; c->st = St::Playing; c->info = m.info; c->goodbye = m.goodbye;
        c->facts = m.facts; c->endDone = false; c->offeredSince = false; c->silent = false;
        c->beginTxn = runScript(*c, Phase::Begin);
        Lines l; l.conv = c->id; l.step = m.step; l.topic = m.topic; l.info = m.info; l.goodbye = m.goodbye; l.lines = m.lines;
        const bool hide = command && !mutations().spectatorsSeeCommands;   // "only the recruiting player sees them"
        if (!hide) { c->lastLines = l; c->hasLines = true; } else { c->hasLines = false; }
        c->hasOffer = false;
        if (others(*c) && !hide) o.push_back({others(*c), l});
        o.push_back({connected_, Result{c->id, c->step, Phase::Begin, c->beginTxn}});
    }
    void holdPick(Conv& c, const Pick& m, const ChoiceRule& rule, uint32_t now, std::vector<Out>& o) {
        c.step = m.step; c.st = St::Consenting; c.info = m.info; c.goodbye = m.goodbye; c.held = m; c.lineDoneEarly = false; c.endDone = true;
        PendingMenu pm;
        pm.id = ++nextMenu_; pm.origin = c.initiator; pm.kind = MenuKind::MessageBox; pm.buttons = {"Agree", "Not yet"}; pm.defaultChoice = 1;
        pm.deadline = now + (rule.timeoutMs ? rule.timeoutMs : cfg_.messageMenuMs);
        pm.opt.partyConsent = true; pm.opt.mode = rule.mode; pm.opt.declineChoice = 1; pm.opt.confirmButton = 0;
        pm.text = m.lines.empty() ? std::string() : m.lines.front().text.substr(0, 100);
        pm.consenting = true; pm.need = others(c); pm.heldConv = c.id; pm.picked = 0;
        MenuRequest rq; rq.menu = pm.id; rq.kind = MenuKind::MessageBox; rq.origin = c.initiator; rq.consent = true;
        rq.text = "Player " + std::to_string(int(c.initiator)) + " is about to decide for the whole party: " + pm.text;
        rq.buttons = pm.buttons; rq.defaultChoice = 1;
        o.push_back({pm.need, rq});
        menus_[pm.id] = std::move(pm);
    }
    void handle(PlayerId from, const LineDone& m, uint32_t now, std::vector<Out>& o) {
        Conv* c = find(m.conv);
        if (!c || !isInitiator(from, *c)) { ++rejected_; return; }
        c->lastActive = now;
        if (m.step != c->step) { ++rejected_; return; }
        if (c->st == St::Consenting) { c->lineDoneEarly = true; return; }   // the initiator's own line ended before the party answered
        if (c->st == St::HoldingEnd) return;                                // the END script waits for the rules engine's vote: a resend changes nothing
        if (c->st == St::AwaitPick) {  // duplicate: resend the result, never rerun
            o.push_back({connected_, Result{c->id, c->step, Phase::End, c->endTxn}});
            return;
        }
        finishLine(*c, now, o);
    }
    // The END script waits for the party: ask everybody but the initiator (asking is agreeing). The conversation stays at this line (HoldingEnd) until rulesResolved.
    void holdEnd(Conv& c, std::vector<Out>& o) {
        c.endHeld = true;
        PendingMenu pm;
        pm.id = ++nextMenu_; pm.origin = c.initiator; pm.kind = MenuKind::MessageBox; pm.buttons = {"Agree", "Not yet"}; pm.defaultChoice = 1;
        pm.deadline = c.lastActive + cfg_.messageMenuMs + kRulesGraceMs;
        pm.consenting = true; pm.need = others(c); pm.rulesWait = true; pm.rulesRef = c.info;
        pm.endConv = c.id; pm.endStep = c.step; pm.endNpc = c.npc; pm.endFacts = c.facts; pm.endGoodbye = c.goodbye;
        pm.text = "an irreversible choice";
        MenuRequest rq; rq.menu = pm.id; rq.kind = MenuKind::MessageBox; rq.origin = c.initiator; rq.consent = true;
        rq.text = "Player " + std::to_string(int(c.initiator)) + " is about to decide for the whole party. Agree?";
        rq.buttons = pm.buttons; rq.defaultChoice = 1;
        if (pm.need) o.push_back({pm.need, rq});
        menus_[pm.id] = std::move(pm);
    }
    void finishLine(Conv& c, uint32_t now, std::vector<Out>& o) {
        runEnd(c, o);
        if (c.endHeld) { c.st = St::HoldingEnd; return; }   // released by rulesResolved
        c.st = St::AwaitPick;
        if (c.goodbye) closeConv(c, CloseReason::Goodbye, now, o);
    }
    void handle(PlayerId from, const EndReq& m, uint32_t now, std::vector<Out>& o) {
        Conv* c = find(m.conv);
        if (!c || !isInitiator(from, *c)) { ++rejected_; return; }
        closeConv(*c, m.reason, now, o);
    }
    void handle(PlayerId from, const Heartbeat& m, uint32_t now, std::vector<Out>&) {
        Conv* c = find(m.conv);
        if (c && isInitiator(from, *c)) c->lastActive = now;
    }
    void handle(PlayerId from, const MenuReply& m, uint32_t now, std::vector<Out>& o) {
        auto it = menus_.find(m.menu);
        if (it == menus_.end()) {                        // duplicate / late reply
            if (mutations().resumeMenuTwice && menuResume_) menuResume_(m.menu, m.choice, false);
            ++rejected_; return;
        }
        PendingMenu& pm = it->second;
        if (pm.consenting) {                             // phase 2: party consent / vote, answers from the other players
            if (!(pm.need & bit(from)) && !mutations().anyoneRepliesMenu) { ++rejected_; return; }
            if (m.choice != 0 && m.choice != 1) { ++rejected_; return; }
            if (pm.rulesWait) {                          // the rules engine counts: every answer goes to it, the menu stays open until its verdict
                if (voteSink_ && !mutations().voteAnswerDropped) voteSink_(from, pm.rulesRef, mutations().voteAnswerInverted ? m.choice != 0 : m.choice == 0);
                return;
            }
            if (m.choice == 1) { pm.no = uint8_t(pm.no | bit(from)); pm.yes = uint8_t(pm.yes & ~bit(from)); }
            else { pm.yes = uint8_t(pm.yes | bit(from)); pm.no = uint8_t(pm.no & ~bit(from)); }
            const Verdict v = verdict(pm, false);
            if (v != Verdict::Pending) finishMenu(it, v == Verdict::Approve ? pm.picked : pm.opt.declineChoice, false, o);
            return;
        }
        if (from != pm.origin && !mutations().anyoneRepliesMenu) { ++rejected_; return; }
        const int32_t maxChoice = int32_t(pm.buttons.empty() ? 1 : pm.buttons.size()) - 1;
        if ((m.choice < -1 || m.choice > maxChoice) && !mutations().anyMenuChoice) { ++rejected_; return; }
        const bool irreversible = m.choice == pm.opt.confirmButton || (pm.opt.anyButton && m.choice >= 0 && m.choice != pm.opt.declineChoice);
        const bool asks = pm.opt.mode != ConsentMode::Actor || mutations().actorAsksParty;   // Actor: a personal decision, nobody is asked
        if (pm.opt.partyConsent && irreversible && asks && !mutations().skipConsent) {
            uint8_t need = uint8_t(connected_ & ~bit(pm.origin));
            if (need && pm.opt.mode == ConsentMode::Rules) {   // the rules engine owns this vote (ChoiceConsent box)
                if (!boxGate_ || mutations().boxGateIgnored || boxGate_(pm.opt.rulesChoice, pm.origin) == Gate::Run) { finishMenu(it, m.choice, false, o); return; }
                pm.consenting = true; pm.need = need; pm.yes = 0; pm.no = 0; pm.picked = m.choice;
                pm.rulesWait = true; pm.rulesRef = pm.opt.rulesChoice;
                pm.deadline = now + (pm.opt.timeoutMs ? pm.opt.timeoutMs : cfg_.messageMenuMs) + kRulesGraceMs;
                MenuRequest rq; rq.menu = pm.id; rq.kind = pm.kind; rq.origin = pm.origin; rq.consent = true;
                rq.text = "Player " + std::to_string(int(pm.origin)) + " is about to decide for the whole party: " + pm.text.substr(0, 120);
                rq.buttons = {"Agree", "Not yet"}; rq.defaultChoice = 1;
                o.push_back({need, rq});
                return;
            }
            if (need) {                                  // point of no return / story fork: ask everybody else
                pm.consenting = true; pm.need = need; pm.yes = 0; pm.no = 0; pm.picked = m.choice;
                pm.deadline = now + (pm.opt.timeoutMs ? pm.opt.timeoutMs : cfg_.messageMenuMs);
                MenuRequest rq; rq.menu = pm.id; rq.kind = pm.kind; rq.origin = pm.origin; rq.consent = true;
                rq.text = "Player " + std::to_string(int(pm.origin)) + " wants to continue: " + pm.text.substr(0, 120);
                rq.buttons = {"Ready", "Not yet"}; rq.defaultChoice = 1;
                o.push_back({need, rq});
                return;
            }
        }
        finishMenu(it, m.choice, false, o);
    }
    template <class It> void finishMenu(It it, int32_t choice, bool timedOut, std::vector<Out>& o) {
        PendingMenu pm = it->second;
        menus_.erase(it);
        closeMenu(pm, choice, timedOut, o);
        if (pm.heldConv) resolveHeld(pm.heldConv, choice == 0, o);
    }
    void closeMenu(const PendingMenu& pm, int32_t choice, bool timedOut, std::vector<Out>& o) {
        std::string label;
        if (choice >= 0 && size_t(choice) < pm.buttons.size()) label = pm.buttons[size_t(choice)];
        o.push_back({connected_, MenuClosed{pm.id, choice, timedOut, label}});
        if (menuResume_ && !pm.heldConv && !pm.endConv) menuResume_(pm.id, choice, timedOut);
    }
    // The verdict for a vote the rules engine owns (or the failsafe when it never came): see rulesResolved.
    template <class It> void rulesDone(It it, bool accepted, uint32_t now, std::vector<Out>& o) {
        PendingMenu pm = it->second;
        if (!pm.endConv) { finishMenu(it, accepted ? pm.picked : pm.opt.declineChoice, false, o); return; }   // a box: the script gets the origin's button or the decline choice
        menus_.erase(it);
        closeMenu(pm, accepted ? 0 : 1, false, o);
        Conv* c = find(pm.endConv);
        const bool run = (accepted && !mutations().heldEndNeverReleased) || (!accepted && mutations().heldEndRunsOnDeny);
        uint64_t txn = 0;
        if (run) {
            ++runs_;
            ResultRequest rq{pm.endConv, pm.endStep, Phase::End, pm.origin, pm.endNpc, pm.rulesRef, &pm.endFacts};
            txn = run_ ? run_(rq) : 0;
        }
        if (!c || !c->endHeld) return;                                       // the conversation is gone (the initiator left): the script ran or was dropped, nothing to continue
        if (mutations().heldEndNeverReleased && accepted) return;           // (mutant) the line hangs
        c->endHeld = false; c->endDone = true; c->endTxn = txn;
        if (run) o.push_back({connected_, Result{c->id, c->step, Phase::End, txn}});
        if (!accepted) { closeConv(*c, CloseReason::ConsentDenied, now, o); return; }   // told: the conversation ends, no script ran
        c->st = St::AwaitPick;
        if (c->goodbye) closeConv(*c, CloseReason::Goodbye, now, o);
    }
    // The party answered for a held fork INFO: run the pick as if it had just arrived, or close the conversation without any script.
    void resolveHeld(ConvId id, bool approved, std::vector<Out>& o) {
        Conv* c = find(id);
        if (!c || c->st != St::Consenting) return;
        if (!approved) { closeConv(*c, CloseReason::ConsentDenied, 0, o); return; }
        const Pick p = c->held;
        acceptPick(*c, p, false, o);
        if (c->lineDoneEarly) { c->lineDoneEarly = false; finishLine(*c, 0, o); }
    }
    // D3 tally. Unanimous: every player still here must have said yes; one no ends it. Vote: the origin counts as yes; the majority of the
    // players present wins as soon as it is certain; with everybody answered (or the window over) a tie goes to the host.
    enum class Verdict : uint8_t { Pending, Approve, Decline };
    static int popcount8(uint8_t v) { int n = 0; for (; v; v = uint8_t(v & (v - 1))) ++n; return n; }
    Verdict verdict(const PendingMenu& pm, bool timedOut) const {
        const bool vote = pm.opt.mode == ConsentMode::Vote && !mutations().voteIsUnanimous;
        if (!vote) {
            if (pm.no & connected_) return Verdict::Decline;
            const uint8_t need = uint8_t(pm.need & connected_);
            if ((pm.yes & need) == need) return Verdict::Approve;
            return timedOut ? Verdict::Decline : Verdict::Pending;
        }
        const int total = popcount8(connected_);
        const uint8_t agreeMask = uint8_t((pm.yes | bit(pm.origin)) & connected_);
        const int agree = popcount8(agreeMask), disagree = popcount8(uint8_t(pm.no & connected_));
        if (agree * 2 > total) return Verdict::Approve;
        if (disagree * 2 > total) return Verdict::Decline;
        const bool allAnswered = uint8_t((agreeMask | pm.no) & connected_) == connected_;
        if (!allAnswered && !timedOut) return Verdict::Pending;
        if (timedOut && mutations().voteTimeoutApproves) return Verdict::Approve;
        if (agree > disagree) return Verdict::Approve;
        if (agree < disagree) return Verdict::Decline;
        if (mutations().voteNoHostTieBreak) return Verdict::Decline;
        return (agreeMask & bit(fnvcoop::kHostPlayer)) ? Verdict::Approve : Verdict::Decline;   // tie: the host decides (no answer from the host = no)
    }
    // `leaver` = a player that just disconnected (kNoPlayer = only time-based expiry)
    void expireMenus(uint32_t now, std::vector<Out>& o, PlayerId leaver) {
        for (auto it = menus_.begin(); it != menus_.end();) {
            PendingMenu& pm = it->second;
            const bool originGone = leaver != fnvcoop::kNoPlayer && pm.origin == leaver;
            if (pm.rulesWait && !originGone) {           // the rules engine owns the count (its own timeout resolves it); this is only the failsafe
                if (leaver != fnvcoop::kNoPlayer) pm.need = uint8_t(pm.need & ~bit(leaver));
                if (now >= pm.deadline && !mutations().noMenuDefault) { auto cur = it++; rulesDone(cur, false, now, o); continue; }
                ++it;
                continue;
            }
            if (pm.consenting && !originGone) {
                if (leaver != fnvcoop::kNoPlayer) {      // a consenter left: they no longer count
                    pm.need = uint8_t(pm.need & ~bit(leaver)); pm.yes = uint8_t(pm.yes & ~bit(leaver)); pm.no = uint8_t(pm.no & ~bit(leaver));
                }
                const bool late = now >= pm.deadline && !mutations().noMenuDefault;
                const Verdict v = verdict(pm, late);
                if (v != Verdict::Pending) {
                    const int32_t choice = v == Verdict::Approve ? pm.picked : pm.opt.declineChoice;
                    auto cur = it++;
                    finishMenu(cur, choice, late, o);
                    continue;
                }
                ++it;
                continue;
            }
            const int32_t fallback = pm.consenting ? pm.opt.declineChoice : pm.defaultChoice;
            if ((originGone || now >= pm.deadline) && !mutations().noMenuDefault) {
                closeMenu(pm, fallback, true, o);
                it = menus_.erase(it);
            } else if (originGone) { it = menus_.erase(it); } else ++it;
        }
    }

    // Host-to-client message types arriving from a client are protocol errors.
    template <class T> void handle(PlayerId, const T&, uint32_t, std::vector<Out>&) { ++rejected_; }

    std::map<uint32_t, PendingMenu> menus_;
    std::map<FormRef, ChoiceRule> choiceRules_;
    static constexpr uint32_t kRulesGraceMs = 30000;   // the failsafe waits this much longer than the rules engine's own timeout
    RulesGate infoGate_, boxGate_;
    VoteSink voteSink_;
    uint32_t nextMenu_ = 0, nextForced_ = 0;
    ActorChooser chooser_;
    MenuResume menuResume_;
    CompanionRegistry* comp_ = nullptr;
    std::set<FormRef> ownerOnly_;
    std::vector<FormRef> released_;

    ResultRunner run_;
    Config cfg_;
    uint8_t connected_ = 0;
    ConvId nextConv_ = 0;
    std::map<ConvId, Conv> convs_;
    std::map<FormRef, ConvId> npcLock_;
    std::set<FormRef> dead_, combat_;
    uint32_t runs_ = 0, rejected_ = 0, repeatPicks_ = 0;
};

}  // namespace fnvcoop_dlg
