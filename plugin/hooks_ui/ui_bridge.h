// L11 hooks_ui: the production Bridge (game-free). Connects UiRouter to L8's DialogueNode (menus, point-of-no-return consent, forced
// dialogue, condition views) and to the slideshow host. L10 owns the DialogueNode; it calls
//     nvui::DialogueBridgeConfig c; c.node = rt.dialogue(); c.nowMs = ...; c.consent = loadConsentMessages(ifstream(consent_messages.txt)); ...
//     router = nvui::MakeRouter(nvui::makeDialogueBridge(c));  nvui::attachMenuResume(*c.node, *router);
// Everything here runs on the game main thread.
#pragma once
#include <cctype>
#include <functional>
#include <map>
#include <memory>
#include <string>
#include <vector>

#include "fnvcoop_dlg/adapter.h"
#include "fnvcoop_dlg/consent.h"
#include "slideshow.h"
#include "ui_core.h"

namespace nvui {

// The confirm and decline buttons of a box the rules engine owns (ConsentMode::Rules), read from the button TEXTS: the three Lonesome Road Temple boxes
// (NVDLC04Temple{Caesar,NCR,NukeAll}ConfirmMSG) are ["No.", "Yes."] in the game data, "No" FIRST. Wave 1b assumed first = Yes. When one button reads
// yes and another reads no (case and punctuation ignored) those two are used; any other layout keeps confirm = first, decline = last.
inline void rulesBoxButtons(const std::vector<std::string>& buttons, int32_t* confirm, int32_t* decline) {
    int32_t yes = -1, no = -1;
    for (size_t i = 0; i < buttons.size(); ++i) {
        std::string t;
        for (char ch : buttons[i]) if (std::isalnum(static_cast<unsigned char>(ch))) t += char(std::tolower(static_cast<unsigned char>(ch)));
        if (t == "yes" && yes < 0) yes = int32_t(i);
        if (t == "no" && no < 0) no = int32_t(i);
    }
    if (yes >= 0 && no >= 0 && !mut(29)) {   // MUTANT 29: the buttons' texts are ignored (first = confirm, last = decline, wrong for ["No.", "Yes."])
        *confirm = yes;
        *decline = no;
    }
}

struct DialogueBridgeConfig {
    fnvcoop_dlg::DialogueNode* node = nullptr;                          // host node (guests never raise anything)
    std::function<uint32_t()> nowMs;                                     // same clock the node's tick() gets
    std::map<FormRef, fnvcoop_dlg::ConsentMessage> consent;              // plugin/dialogue/data/consent_messages.txt
    std::function<PlayerId(uint32_t actorRefID)> slotOfActor;            // 0x14 -> host, guest ghost refs -> their slot
    std::function<bool(uint32_t questRefID)> watchesQuest;               // optional exact prefilter; default: any variable rule exists
    std::function<uint64_t()> lastCommittedSeq;                          // optional: story seq of the INFO's captured effects (ForcedSay ordering)
    SlideHost* slides = nullptr;                                         // optional: ending slideshow broadcaster
    // wave 1b (G08): message boxes whose vote the RULES ENGINE owns (ChoiceConsent.boxes; RulesEngine::isChoice). They are raised with ConsentMode::Rules: the origin picks, the
    // dialogue host's box gate asks the engine (fnvcoop_dlg::linkRules), the other players answer a two-button box, the engine's verdict resumes the script.
    std::function<bool(const FormRef& message)> isRulesChoice;
    // wave 1b (G24): per-player quest variables. varName = the name of a quest variable index (nvhooks::questVarName), viewPlayerVar = RulesEngine::viewPlayerVar
    // (the evaluating slot's copy, else the seed, else `real`), hasPlayerVars = RulesEngine::perPlayerVarNames(quest) is not empty.
    std::function<bool(const FormRef& quest, uint32_t idx, std::string* name)> varName;
    std::function<double(const FormRef& quest, const std::string& var, PlayerId slot, double real)> viewPlayerVar;
    std::function<bool(const FormRef& quest)> hasPlayerVars;
};

inline Bridge makeDialogueBridge(const DialogueBridgeConfig& c) {
    Bridge b;
    auto consentMap = std::make_shared<std::map<FormRef, fnvcoop_dlg::ConsentMessage>>(c.consent);
    fnvcoop_dlg::DialogueNode* node = c.node;
    auto now = c.nowMs;

    b.slotOfActor = c.slotOfActor;
    b.raiseMenu = [node, now](PlayerId origin, MenuKind kind, const std::string& text, std::vector<std::string> buttons, int32_t def,
                              const FormRef& subject) { return node->raiseMenu(origin, kind, text, std::move(buttons), def, subject, now()); };
    b.raiseModal = [node, now](PlayerId origin, const std::string& text, std::vector<std::string> buttons, int32_t def, const ConsentRule& rule) {
        fnvcoop_dlg::HostDialogue::ModalOptions o;
        o.partyConsent = true;
        o.confirmButton = rule.confirmButton;
        o.declineChoice = rule.declineChoice;
        if (rule.mode == 3 && o.declineChoice < 0) o.declineChoice = buttons.empty() ? 0 : int32_t(buttons.size()) - 1;   // the conventional "No / Cancel" is the last button
        if (rule.mode == 3) rulesBoxButtons(buttons, &o.confirmButton, &o.declineChoice);   // ... unless the texts say otherwise (the Temple boxes are "No." then "Yes.")
        o.timeoutMs = rule.timeoutMs;
        o.mode = static_cast<fnvcoop_dlg::HostDialogue::ConsentMode>(rule.mode);
        o.anyButton = rule.anyButton;
        o.rulesChoice = rule.choice;
        return node->raiseModal(origin, text, std::move(buttons), def, o, now());
    };
    auto isRulesChoice = c.isRulesChoice;
    b.consentFor = [consentMap, isRulesChoice](const FormRef& msg) -> std::optional<ConsentRule> {
        auto it = consentMap->find(msg);
        if (it == consentMap->end() && isRulesChoice && !mut(26) && isRulesChoice(msg)) {   // MUTANT 26: a box the rules engine owns is raised as a plain box (nobody is asked)
            ConsentRule r;
            r.confirmButton = 0;       // default when the button texts do not name yes and no (rulesBoxButtons); the Temple boxes are ["No.", "Yes."] (game data)
            r.declineChoice = -1;      // the last button
            r.mode = 3;
            r.choice = msg;
            return r;
        }
        if (it == consentMap->end() || it->second.isInfo || mut(15)) return std::nullopt;  // MUTANT 15: the consent table is ignored
        ConsentRule r;
        r.confirmButton = it->second.confirmButton;
        r.declineChoice = it->second.declineChoice;
        r.timeoutMs = it->second.timeoutMs;
        r.mode = it->second.mode;
        r.anyButton = it->second.anyButton;
        return r;
    };
    // fork INFOs (Joshua or Daniel, Elijah's offer): the dialogue host holds the pick until the party answered
    if (node && node->host() && !mut(25)) {   // MUTANT 25: the fork INFOs of the consent table are never handed to the dialogue host
        std::map<FormRef, fnvcoop_dlg::HostDialogue::ChoiceRule> rules;
        for (const auto& kv : c.consent) {
            if (!kv.second.isInfo) continue;
            fnvcoop_dlg::HostDialogue::ChoiceRule r;
            r.mode = static_cast<fnvcoop_dlg::HostDialogue::ConsentMode>(kv.second.mode);
            r.timeoutMs = kv.second.timeoutMs;
            rules[kv.first] = r;
        }
        if (!rules.empty()) node->host()->setChoiceRules(std::move(rules));
    }

    b.watchesQuest = c.watchesQuest ? c.watchesQuest : [node](uint32_t) { return node->companions().ruleCount() != 0; };
    b.viewVar = [node](const FormRef& quest, uint32_t idx, PlayerId slot, int32_t real) { return node->viewQuestVarFor(quest, idx, slot, real); };
    if (c.varName && c.viewPlayerVar && c.hasPlayerVars && !mut(27)) {   // MUTANT 27: the per-player quest variables of the rules engine are not connected
        auto varName = c.varName;
        auto viewPlayerVar = c.viewPlayerVar;
        b.hasPlayerVars = c.hasPlayerVars;
        b.viewPlayerVar = [varName, viewPlayerVar](const FormRef& quest, uint32_t idx, PlayerId slot, double real) {
            std::string name;
            return varName(quest, idx, &name) ? viewPlayerVar(quest, name, slot, real) : real;
        };
    }

    auto seq = c.lastCommittedSeq;
    b.forcedSay = [node, now, seq](const ForcedSayArgs& a) {
        node->forcedSay(a.npc, a.topic, a.info, a.lines, a.target, a.scene, seq ? seq() : 0, now());
    };
    b.forceConversation = [node, now](const FormRef& npc, const FormRef& topic, PlayerId target) { return node->forceConversation(npc, topic, target, now()); };

    if (c.slides && !mut(16)) {  // MUTANT 16: the slide host is not connected
        SlideHost* s = c.slides;
        b.slideSwap = [s, now](const FormRef& ref, const std::string& o, const std::string& n) { s->onSwap(ref, o, n, now()); };
        if (!mut(20)) b.nextStamp = [s]() { return s->clock().next(); };  // MUTANT 20: scene narration is not stamped (no order against slides)
    }
    return b;
}

// Menu answers (and timeouts) reach the script that is waiting for them.
inline void attachMenuResume(fnvcoop_dlg::DialogueNode& node, UiRouter& router) {
    node.setMenuResume([&router](uint32_t menu, int32_t choice, bool timedOut) { router.onMenuResume(menu, choice, timedOut); });
}

}  // namespace nvui
