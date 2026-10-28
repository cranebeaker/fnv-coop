#include "ui_core.h"

#include <algorithm>

#include "voice_token.h"

#include "fnvcoop_mut.h"  // FNV_MUT(n): runtime mutation twins (env FNVCOOP_MUT=n); constant false in production builds

namespace nvui {

bool mut(int n) { return FNV_MUT(n); }

int32_t defaultChoiceFor(size_t n) { return n == 0 ? -1 : int32_t(n) - 1; }

// ------------------------------------------------------------------ origin
void OriginContext::pop(Kind k) {
    for (size_t i = scopes_.size(); i-- > 0;) {
        if (scopes_[i].kind == k) {
            scopes_.erase(scopes_.begin() + std::ptrdiff_t(i));
            return;
        }
    }
}

void OriginContext::noteActivation(uint32_t target, PlayerId slot, uint64_t nowMs) {
    if (target) activations_[target] = {slot, nowMs};
}
void OriginContext::noteScript(uint32_t key, PlayerId slot, uint64_t nowMs) {
    if (key) scripts_[key] = {slot, nowMs};
}

PlayerId OriginContext::resolve(uint32_t thisObj, uint32_t scriptKey, uint64_t nowMs) const {
    if (mut(1)) return local_;  // MUTANT 1: origin ignored, everything lands on this machine's player
    if (!scopes_.empty()) return scopes_.back().slot;
    if (thisObj) {
        auto it = activations_.find(thisObj);
        if (it != activations_.end() && nowMs - it->second.atMs <= ttl_.activationMs) return it->second.slot;
    }
    if (scriptKey) {
        auto it = scripts_.find(scriptKey);
        if (it != scripts_.end() && (mut(7) || nowMs - it->second.atMs <= ttl_.stickyMs)) return it->second.slot;  // MUTANT 7: sticky forever
    }
    return local_;
}

// ------------------------------------------------------------------ message boxes
void UiRouter::finishBox(const Pending& p, int32_t choice) {
    int32_t out = kNoButton;
    if (choice >= 0 && (p.buttons == 0 || choice < p.buttons)) out = p.buttons == 0 ? 0 : choice;
    if (mut(3) && out != kNoButton) out += 1;  // MUTANT 3: off-by-one button
    eng_.setMessageBoxResult(p.scriptRefID, out);
    ++stats_.resumes;
}

Route UiRouter::onMessageBox(const MessageBoxCall& c) {
    if (!host_ || !br_.raiseMenu) {
        ++stats_.boxesLocal;
        return Route::Local;
    }
    const uint64_t now = eng_.nowMs();
    PlayerId origin = origin_.resolve(c.thisObj, c.scriptRefID, now);
    std::optional<ConsentRule> consent;
    if (br_.consentFor && br_.raiseModal && !c.message.plugin.empty()) consent = br_.consentFor(c.message);
    // A box that asks the PARTY is raised through the dialogue host even when the host player is the one deciding: the host's own native box
    // would have skipped the vote (the consent rules only ever reached the box a guest answered). Personal decisions (mode 2) stay local.
    const bool partyBox = consent && consent->mode != 2 && !mut(23);   // MUTANT 23: a host-origin box never asks the party
    if ((origin == kHost || origin == kNoSlot) && !partyBox) {
        ++stats_.boxesLocal;
        return Route::Local;
    }
    if (origin == kNoSlot) origin = kHost;
    if (origin == kHost) ++stats_.hostConsent;
    FormRef subject;
    if (c.thisObj) eng_.refOf(c.thisObj, subject);
    const int32_t def = defaultChoiceFor(c.buttons.size());

    raising_ = true;
    early_.reset();
    uint32_t id = consent ? br_.raiseModal(origin, c.text, c.buttons, def, *consent)
                          : br_.raiseMenu(origin, MenuKind::MessageBox, c.text, c.buttons, def, subject);
    raising_ = false;
    if (id == 0) {  // no dialogue host behind the bridge: do not strand the script
        early_.reset();
        ++stats_.boxesLocal;
        return Route::Local;
    }
    origin_.noteScript(c.scriptRefID, origin, now);
    eng_.setMessageBoxResult(c.scriptRefID, kNoButton);  // box "open": GetButtonPressed answers -1 until the reply
    Pending p{c.scriptRefID, int32_t(c.buttons.size())};
    ++stats_.boxesRouted;
    if (early_ && early_->first == id) {
        if (!mut(6)) finishBox(p, early_->second);  // MUTANT 6: a synchronous resume is dropped and the script never continues
        early_.reset();
    } else {
        pending_[id] = p;
    // G07: the script has a `begin MenuMode 1001` block (it polls GetButtonPressed there). With no menu open on this machine that block
    // never runs (inferred from the code, not seen in game): either mirror the box here until the origin answers, or say so.
    if (origin != kHost && mmHasBlock_ && !c.script.plugin.empty() && mmHasBlock_(c.script)) {
        if (mmMode_ == MenuModeMode::Mirror && !mut(22)) {   // MUTANT 22: the mirror is never opened (the MenuMode block stays dead)
            mirrors_.insert(id);
            ++stats_.mirrorsOpened;
            eng_.openMirror(id, c.text, origin);
        } else if (mmWarned_.insert(c.script).second) {
            ++stats_.menuModeSkipped;
            eng_.log("box for script " + c.script.plugin + ":" + std::to_string(c.script.localId) +
                     " was raised to a remote player and the script has a MenuMode block: it will not run on this machine (menumode=off; needs in-game proof)");
        }
    }
    }
    return Route::Suppressed;
}

void UiRouter::onMenuResume(uint32_t menu, int32_t choice, bool) {
    auto it = pending_.find(menu);
    if (it == pending_.end()) {
        closeMirror(menu);
        if (raising_) early_ = std::make_pair(menu, choice);
        else ++stats_.dupResumes;
        return;
    }
    Pending p = it->second;
    if (!mut(2)) pending_.erase(it);  // MUTANT 2: resume is not one-shot
    finishBox(p, choice);
    closeMirror(menu);
}

void UiRouter::closeMirror(uint32_t menu) {
    if (mirrors_.erase(menu) && !mut(24)) {   // MUTANT 24: the mirror stays up after the answer (the host stays paused in a dead box)
        ++stats_.mirrorsClosed;
        eng_.closeMirror(menu);
    }
}

// ------------------------------------------------------------------ service menus
Route UiRouter::onServiceMenu(const ServiceMenuCall& c) {
    if (!host_ || !br_.raiseMenu) {
        ++stats_.serviceLocal;
        return Route::Local;
    }
    const PlayerId origin = origin_.resolve(c.thisObj, c.scriptKey, eng_.nowMs());
    if (origin == kHost || origin == kNoSlot) {
        ++stats_.serviceLocal;
        return Route::Local;
    }
    FormRef subject = c.subject;
    if (subject.plugin.empty() && c.thisObj) eng_.refOf(c.thisObj, subject);
    br_.raiseMenu(origin, c.kind, std::string(), {}, -1, subject);
    ++stats_.serviceRouted;
    return Route::Suppressed;
}

// ------------------------------------------------------------------ conditions
double UiRouter::viewQuestVar(uint32_t questRefID, uint32_t idx, double real) {
    const bool companions = br_.viewVar && br_.watchesQuest && br_.watchesQuest(questRefID);
    bool playerVars = false;   // G24: the quest has per-player variables of the rules engine (asked once per quest)
    if (br_.viewPlayerVar && br_.hasPlayerVars && !mut(28)) {   // MUTANT 28: the per-player variable view is never applied
        auto c = playerVarQuests_.find(questRefID);
        if (c == playerVarQuests_.end()) {
            FormRef q;
            c = playerVarQuests_.emplace(questRefID, eng_.refOf(questRefID, q) && br_.hasPlayerVars(q)).first;
        }
        playerVars = c->second;
    }
    if (!companions && !playerVars) return real;
    FormRef q;
    if (!eng_.refOf(questRefID, q)) return real;
    const PlayerId slot = mut(4) ? kHost : origin_.resolve(0, 0, eng_.nowMs());  // MUTANT 4: always the host's view
    double v = real;
    if (companions) v = double(br_.viewVar(q, idx, slot, int32_t(real)));
    if (playerVars) v = br_.viewPlayerVar(q, idx, slot, v);
    ++stats_.condViews;
    if (v != real) ++stats_.condChanged;
    return v;
}

// ------------------------------------------------------------------ forced dialogue
UiRouter::TargetInfo UiRouter::classifyTarget(uint32_t refID, uint32_t) const {
    TargetInfo t;
    if (!refID) return t;
    PlayerId s = br_.slotOfActor ? br_.slotOfActor(refID) : (refID == kPlayerRefID ? kHost : kNoSlot);
    if (s == kNoSlot && refID == kPlayerRefID) s = kHost;
    if (s == kNoSlot) return t;
    t.player = true;
    if (refID == kPlayerRefID) {
        // "the player" inside a script = the player the script runs for; with no such player it is the party
        if (origin_.scoped()) t.slot = origin_.resolve(0, 0, 0);
        else t.party = true;
    } else {
        t.slot = s;
    }
    return t;
}

ForcedOutcome UiRouter::onForced(const ForcedCall& c) {
    ForcedOutcome o;
    if (!host_) return o;
    if (c.speaker == kPlayerRefID) {  // the player speaking is not an NPC line
        ++stats_.forcedIgnored;
        return o;
    }
    TargetInfo t;
    if (c.kind != ForcedCall::Kind::Say) t = classifyTarget(c.target, c.speaker);
    const bool aimed = c.kind == ForcedCall::Kind::Say || t.player;
    if (!aimed && !mut(5)) {  // MUTANT 5: NPC-to-NPC chatter is captured too
        ++stats_.forcedIgnored;
        return o;
    }
    o.target = t.party ? kNoSlot : t.slot;
    FormRef npc;
    eng_.refOf(c.speaker, npc);
    if (c.kind == ForcedCall::Kind::StartConversation) {
        if (!br_.forceConversation) return o;
        const PlayerId actor = br_.forceConversation(npc, c.topic, o.target);
        o.captured = true;
        o.target = actor;
        o.vetoNative = !mut(8) && actor != kHost && actor != kNoSlot;  // MUTANT 8: the host's DialogMenu opens for a remote actor
        if (o.vetoNative) ++stats_.vetoes;
        ++stats_.forced;
        return o;
    }
    if (!br_.forcedSay) return o;
    ForcedSayArgs a;
    a.npc = npc;
    a.topic = c.topic;
    a.info = c.info;
    a.lines = c.lines;
    // Receivers play the line with `<npc>.Say <topic>`: name the topic + INFO in each line's voice token (the same token a conversation uses).
    if (!mut(21) && !a.topic.plugin.empty() && !a.info.plugin.empty())   // MUTANT 21: forced lines carry no voice token (subtitle only, no voice)
        for (size_t i = 0; i < a.lines.size(); ++i)
            if (a.lines[i].voice.empty()) a.lines[i].voice = makeVoiceToken(a.topic, a.info, uint32_t(i + 1));
    a.target = o.target;
    a.scene = c.scene || t.party || (c.kind == ForcedCall::Kind::Say && !origin_.scoped());
    if (a.scene && br_.nextStamp)  // party scene (narrator, cutscene): everybody gets these, so they can be ordered against the slides
        for (auto& ln : a.lines) ln.voice = withOrder(ln.voice, br_.nextStamp());
    br_.forcedSay(a);
    o.captured = true;
    ++stats_.forced;
    return o;
}

// ------------------------------------------------------------------ slideshow
void UiRouter::onSwapTexture(uint32_t refID, const std::string& oldTex, const std::string& newTex) {
    if (!host_) return;
    Slide s;
    s.seq = ++slideSeq_;
    eng_.refOf(refID, s.ref);
    s.oldTex = oldTex;
    s.newTex = newTex;
    if (mut(9)) slides_.insert(slides_.begin(), s);  // MUTANT 9: slides recorded out of order
    else slides_.push_back(s);
    ++stats_.slides;
    if (br_.slideSwap) br_.slideSwap(s.ref, oldTex, newTex);
}

}  // namespace nvui
