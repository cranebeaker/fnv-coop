// Transport adapter (L8): runs HostDialogue / ClientDialogue over fnvcoop::ITransport (L4), so dialogue shares the
// same peers/channel as EffectTxns. Dialogue packets start with kMagic (0xD1); L4 envelopes start with 0xFC, so the
// receiver demultiplexes on the first byte and no change to fnvcoop::Message is required. (If L4 later adds
// `DlgMsg{session, payload}` the payload is exactly encode()'s output; only `wrap`/`unwrap` below change.)
//
// Channel requirement: reliable + ordered per peer pair (ENet reliable channel). Duplicates are tolerated
// (idempotent host/clients); loss and reordering are not (a lost Pick would stall until the lease expires).
#pragma once
#include <cstdio>
#include <functional>
#include <string>
#include <memory>
#include <set>
#include "fnvcoop/transport.h"
#include "fnvcoop_dlg/client.h"
#include "fnvcoop_dlg/host.h"

namespace fnvcoop_dlg {

using fnvcoop::Bytes;
using fnvcoop::PeerId;

// Diagnostics: one line per conversation start step (a guest's request, the host's grant or refusal, the guest's menu event). Unset = silent.
// plugin/intents' module sets it to nvcoop::Log; the node never depends on it.
inline std::function<void(const std::string&)>& traceSink() {
    static std::function<void(const std::string&)> f;
    return f;
}
inline std::string traceRef(const fnvcoop::FormRef& r) {
    char b[24];
    std::snprintf(b, sizeof b, "%X", static_cast<unsigned>(r.localId));
    return r.plugin + ":" + b;
}
inline void trace(const std::string& s) {
    if (traceSink()) traceSink()(s);
}

class DialogueNode {
public:
    struct Config {
        PlayerId self = 0;
        bool isHost = false;
        PeerId hostPeer = 0;                              // client: where to send
        std::function<PeerId(PlayerId)> peerOfSlot;       // host: slot -> transport peer
        std::function<PlayerId(PeerId)> slotOfPeer;       // host: transport peer -> slot
        HostDialogue::Config host;
        ClientDialogue::Config client;
        CompanionRules companionRules;                    // per-owner caps (default 1 humanoid + 1 critter each, party total 4)
        std::set<FormRef> ownerOnlyTopics;                // companion command topics (loadOwnerOnlyTopics)
        std::function<uint64_t()> appliedSeq;             // story link: highest EffectTxn seq applied locally (see ClientDialogue::setAppliedProvider)
    };
    using Sink = std::function<void(const Event&)>;       // engine glue: OpenMenu, ShowLines, CloseMenu, ...

    DialogueNode(fnvcoop::ITransport* t, Config cfg, ResultRunner run, Sink sink)
        : t_(t), cfg_(std::move(cfg)), client_(cfg_.self, cfg_.client), companions_(cfg_.companionRules), sink_(std::move(sink)) {
        if (cfg_.appliedSeq) client_.setAppliedProvider(cfg_.appliedSeq);
        if (cfg_.isHost) {
            host_.reset(new HostDialogue(std::move(run), cfg_.host));
            host_->setCompanions(&companions_, cfg_.ownerOnlyTopics);
            host_->playerJoined(cfg_.self);
        }
    }

    static bool isDialoguePacket(const Bytes& b) { return !b.empty() && b[0] == kMagic; }

    // Feed every received packet; returns true if it was a dialogue packet (consumed).
    bool onPacket(PeerId from, const Bytes& b, uint32_t now) {
        if (!isDialoguePacket(b)) return false;
        Msg m;
        if (decode(b, m) != DecodeStatus::Ok) { ++badPackets_; return true; }
        if (auto* sm = std::get_if<SayOnceMark>(&m)) {   // not a conversation message: a guest's engine marked an INFO said (say-once, D11)
            if (cfg_.isHost) {
                const PlayerId slot = cfg_.slotOfPeer ? cfg_.slotOfPeer(from) : fnvcoop::kNoPlayer;
                if (slot < 8 && slot != cfg_.self && sayOnceSink_) sayOnceSink_(slot, sm->info);
                else ++sayOnceRejected_;
            }
            return true;
        }
        if (cfg_.isHost && isClientToHost(m)) {
            if (auto* sr = std::get_if<StartReq>(&m))
                trace("dialogue: host got StartReq req " + std::to_string(sr->reqId) + " npc " + traceRef(sr->npc) + " from peer " + std::to_string(from) + " = slot " + std::to_string(int(cfg_.slotOfPeer(from))));
            fanOut(host_->onMessage(cfg_.slotOfPeer(from), m, now), now);
        }
        else if (!cfg_.isHost || !isClientToHost(m)) deliverLocal(m, now);
        return true;
    }

    // ---- client API (also valid on the host: its own client side loops back without the network)
    void requestStart(const FormRef& npc, uint32_t now) {
        const auto role = client_.role();
        auto evs = client_.requestStart(npc);
        trace("dialogue: requestStart npc " + traceRef(npc) + " role was " + std::to_string(int(role)) + (evs.empty() ? " (nothing sent)" : " (sent)"));
        route(std::move(evs), now);
    }
    void offer(std::vector<TopicEntry> t, uint32_t now) { route(client_.offer(std::move(t)), now); }
    void pick(const FormRef& topic, const FormRef& info, int32_t choice, bool goodbye, std::vector<Line> lines,
              std::vector<Fact> facts, uint32_t now) {
        route(client_.pick(topic, info, choice, goodbye, std::move(lines), std::move(facts)), now);
    }
    void lineDone(uint32_t now) { route(client_.lineDone(now), now); }
    void leave(uint32_t now) { route(client_.leave(), now); }
    // The story session (re)joined: drop any stale conversation role (emits CloseMenu to the sink if there was one).
    void resync(uint32_t now) { route(client_.sessionReset(), now); }
    void tick(uint32_t now) {
        route(client_.tick(now), now);
        if (host_) fanOut(host_->tick(now), now);
    }

    // ---- say-once marks the engine made on a guest (D11). Guest: report to the host. Host: the sink gets (slot, info) for every report from a joined slot.
    using SayOnceSink = std::function<void(PlayerId from, const FormRef& info)>;
    void setSayOnceSink(SayOnceSink s) { sayOnceSink_ = std::move(s); }
    bool reportSayOnce(const FormRef& info) {
        if (cfg_.isHost || !t_ || info.plugin.empty()) return false;
        t_->send(cfg_.hostPeer, encode(SayOnceMark{info}));
        ++sayOnceSent_;
        return true;
    }
    uint32_t sayOnceSent() const { return sayOnceSent_; }
    uint32_t sayOnceRejected() const { return sayOnceRejected_; }

    // ---- host API
    void playerJoined(PlayerId p, uint32_t now = 0) { if (host_) fanOut(host_->playerJoined(p), now); }
    void playerLeft(PlayerId p, uint32_t now) { if (host_) fanOut(host_->playerLeft(p, now), now); }
    void npcDied(const FormRef& n, uint32_t now) { if (host_) fanOut(host_->npcDied(n, now), now); }
    void npcCombat(const FormRef& n, bool on, uint32_t now) { if (host_) fanOut(host_->npcCombat(n, on, now), now); }
    void controlLock(ConvId c, uint8_t mask, bool locked, uint32_t now) { if (host_) fanOut(host_->controlLock(c, mask, locked), now); }

    // ---- companions: per-owner view of the shared follower variables (every machine keeps a registry)
    CompanionRegistry& companions() { return companions_; }
    const std::set<FormRef>& ownerOnlyTopics() const { return cfg_.ownerOnlyTopics; }
    // The node is built by the runtime before the data files are known: the glue module fills these in afterwards (idempotent).
    void setOwnerOnlyTopics(std::set<FormRef> s) {
        cfg_.ownerOnlyTopics = std::move(s);
        if (host_) host_->setCompanions(&companions_, cfg_.ownerOnlyTopics);
    }
    void setCompanionRules(const CompanionRules& r) { cfg_.companionRules = r; companions_.setRules(r); }
    PlayerId self() const { return cfg_.self; }
    // Engine hook for GetQuestVariable evaluated for dialogue conditions / result-script reads: the value as seen by
    // `slot` (default: this machine's player). Quests without a rule are returned untouched.
    int32_t viewQuestVar(const FormRef& quest, uint32_t idx, int32_t real) const { return companions_.viewVar(quest, idx, cfg_.self, real); }
    int32_t viewQuestVarFor(const FormRef& quest, uint32_t idx, PlayerId slot, int32_t real) const { return companions_.viewVar(quest, idx, slot, real); }
    // CompanionOwner effect applied locally (ownership is replicated state)
    void onCompanionOwner(const FormRef& npc, PlayerId owner) { companions_.setOwner(npc, owner); }

    // ---- menus (host raises, origin answers)
    void setMenuResume(HostDialogue::MenuResume r) { if (host_) host_->setMenuResume(std::move(r)); }
    uint32_t raiseMenu(PlayerId origin, MenuKind kind, const std::string& text, std::vector<std::string> buttons,
                       int32_t defaultChoice, const FormRef& subject, uint32_t now) {
        uint32_t id = 0;
        if (host_) fanOut(host_->raiseMenu(origin, kind, text, std::move(buttons), defaultChoice, subject, now, &id), now);
        return id;
    }
    void menuReply(uint32_t menu, int32_t choice, uint32_t now) { route(client_.menuReply(menu, choice), now); }
    // ModalChoice variant (decider = origin, optional party consent for points of no return)
    uint32_t raiseModal(PlayerId origin, const std::string& text, std::vector<std::string> buttons, int32_t defaultChoice,
                        const HostDialogue::ModalOptions& opt, uint32_t now) {
        uint32_t id = 0;
        if (host_) fanOut(host_->raiseMenu(origin, MenuKind::MessageBox, text, std::move(buttons), defaultChoice, FormRef(), now, &id, opt), now);
        return id;
    }

    // ---- votes owned by the rules engine (ReadyCheck / ChoiceConsent, rules_link.h wires a RulesEngine in one call)
    void setRulesGates(HostDialogue::RulesGate info, HostDialogue::RulesGate box) { if (host_) host_->setRulesGates(std::move(info), std::move(box)); }
    void setVoteSink(HostDialogue::VoteSink s) { if (host_) host_->setVoteSink(std::move(s)); }
    void rulesResolved(const FormRef& ref, bool accepted, uint32_t now) { if (host_) fanOut(host_->rulesResolved(ref, accepted, now), now); }

    // ---- forced dialogue (host hook for SayTo / StartConversation; the ForcedDialogue effect is the capture record)
    void setActorChooser(HostDialogue::ActorChooser c) { if (host_) host_->setActorChooser(std::move(c)); }
    void forcedSay(const FormRef& npc, const FormRef& topic, const FormRef& info, std::vector<Line> lines, PlayerId target,
                   bool scene, uint64_t txnSeq, uint32_t now) {
        if (host_) fanOut(host_->forcedSay(npc, topic, info, std::move(lines), target, scene, txnSeq), now);
    }
    PlayerId forceConversation(const FormRef& npc, const FormRef& topic, PlayerId target, uint32_t now) {
        PlayerId actor = fnvcoop::kNoPlayer;
        if (host_) fanOut(host_->forceConversation(npc, topic, target, now, &actor), now);
        return actor;
    }

    ClientDialogue& client() { return client_; }
    HostDialogue* host() { return host_.get(); }
    uint32_t badPackets() const { return badPackets_; }

private:
    static bool isClientToHost(const Msg& m) {
        return std::holds_alternative<StartReq>(m) || std::holds_alternative<Offer>(m) ||
               std::holds_alternative<Pick>(m) || std::holds_alternative<LineDone>(m) ||
               std::holds_alternative<EndReq>(m) || std::holds_alternative<Heartbeat>(m) ||
               std::holds_alternative<MenuReply>(m);
    }
    // Offer is both directions: client->host carries the initiator's list, host->client the relayed copy. The
    // host tells them apart by who sent it (only clients send to the host), so a host-side client never sees one
    // from the network.
    void deliverLocal(const Msg& m, uint32_t now) { route(client_.onMessage(m, now), now); }

    void route(std::vector<Event> evs, uint32_t now) {
        for (auto& e : evs) {
            if (e.kind != Event::Send) {
                if (e.kind == Event::OpenMenu || e.kind == Event::Denied || e.kind == Event::CloseMenu)
                    trace("dialogue: event " + std::to_string(int(e.kind)) + (e.kind == Event::Denied ? " deny " + std::to_string(int(e.deny)) : std::string()) + (sink_ ? "" : " (no sink)"));
                if (sink_) sink_(e);
                continue;
            }
            if (cfg_.isHost) fanOut(host_->onMessage(cfg_.self, e.msg, now), now);
            else t_->send(cfg_.hostPeer, encode(e.msg));
        }
    }
    void fanOut(const std::vector<Out>& outs, uint32_t now) {
        for (auto& o : outs) {
            if (auto* op = std::get_if<Opened>(&o.msg)) trace("dialogue: host grants conv " + std::to_string(op->conv) + " req " + std::to_string(op->reqId) + " npc " + traceRef(op->npc) + " to initiator slot " + std::to_string(int(op->initiator)));
            else if (auto* dn = std::get_if<StartDenied>(&o.msg)) trace("dialogue: host DENIES req " + std::to_string(dn->reqId) + " reason " + std::to_string(int(dn->reason)) + " owner " + std::to_string(int(dn->owner)));
            for (PlayerId p = 0; p < 8; ++p) {
                if (!((o.to >> p) & 1)) continue;
                if (p == cfg_.self) deliverLocal(o.msg, now);
                else t_->send(cfg_.peerOfSlot(p), encode(o.msg));
            }
        }
    }

    fnvcoop::ITransport* t_;
    Config cfg_;
    ClientDialogue client_;
    CompanionRegistry companions_;
    std::unique_ptr<HostDialogue> host_;
    Sink sink_;
    uint32_t badPackets_ = 0, sayOnceSent_ = 0, sayOnceRejected_ = 0;
    SayOnceSink sayOnceSink_;
};

}  // namespace fnvcoop_dlg
