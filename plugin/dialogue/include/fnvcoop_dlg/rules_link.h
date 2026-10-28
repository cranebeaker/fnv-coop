// Connects the rules engine's ReadyCheck / ChoiceConsent votes to the dialogue host (campaign wave 1b, G08 / G27). Header only, game-free.
//
//   fnvcoop_dlg::linkRules(*rt.dialogue(), *rules, [] { return nowMs(); });     // once per session, after the RulesEngine is built
//
// What it wires:
//   * an INFO's END script asks RulesEngine::onInfoResult first (ReadyCheck INFOs: House, the Yes Man topics; ChoiceConsent INFOs: Daniel, Elijah). Deferred holds the
//     script; onReadyResolved / onChoiceResolved release it (accepted: it runs, declined: it is dropped and the conversation closes with ConsentDenied).
//   * a message box the origin confirmed asks RulesEngine::onChoiceRequested (ChoiceConsent.boxes: the three Temple boxes) when the router raised it with
//     ConsentMode::Rules (hooks_ui: DialogueBridgeConfig::isRulesChoice).
//   * the other players answer a two-button box on their own machine; the dialogue host turns each answer into the rules engine's vote: Agree = ScenePass of the ref
//     (what ClientSession::submitAction(ScenePass, ref) would send), Not yet = ScenePass of choiceDeclineId(ref). A ready check has no "no": the timeout cancels it.
//   * an INFO the engine owns is taken out of the old consent table (HostDialogue::choiceRules_), so the party is asked once.
// Callbacks already set on the engine (onReadyResolved / onChoiceResolved) keep running first.
#pragma once
#include <functional>
#include <string>

#include "fnvcoop/rules.h"
#include "fnvcoop_dlg/adapter.h"

namespace fnvcoop_dlg {

// `Node` is a DialogueNode (or anything with setRulesGates / setVoteSink / rulesResolved / host(), which is what the tests use).
template <class Node>
inline void linkRules(Node& node, fnvcoop::RulesEngine& eng, std::function<uint32_t()> nowMs) {
    using Gate = HostDialogue::Gate;
    node.setRulesGates(
        [&eng](const FormRef& info, PlayerId who) { return eng.onInfoResult(info, who) == fnvcoop::StageDecision::Deferred ? Gate::Hold : Gate::Run; },
        [&eng](const FormRef& box, PlayerId who) {
            return eng.isChoice(box) && eng.onChoiceRequested(box, who) == fnvcoop::StageDecision::Deferred ? Gate::Hold : Gate::Run;
        });
    node.setVoteSink([&eng](PlayerId slot, const FormRef& ref, bool yes) {
        if (yes) { eng.handleAction(slot, fnvcoop::IntentKind::ScenePass, ref); return; }
        if (eng.isChoice(ref)) eng.handleAction(slot, fnvcoop::IntentKind::ScenePass, fnvcoop::RulesEngine::choiceDeclineId(ref));
    });
    auto prevReady = eng.onReadyResolved;
    eng.onReadyResolved = [&node, nowMs, prevReady](const FormRef& quest, const FormRef& info, bool proceed) {
        if (prevReady) prevReady(quest, info, proceed);
        node.rulesResolved(info, proceed, nowMs ? nowMs() : 0);
    };
    auto prevChoice = eng.onChoiceResolved;
    eng.onChoiceResolved = [&node, nowMs, prevChoice](const FormRef& quest, const FormRef& choice, bool accepted) {
        if (prevChoice) prevChoice(quest, choice, accepted);
        node.rulesResolved(choice, accepted, nowMs ? nowMs() : 0);
    };
    if (HostDialogue* h = node.host())
        for (const FormRef& info : h->choiceRuleInfos())
            if (eng.isChoice(info)) h->dropChoiceRule(info);
}

// The rules-engine side of plugin/hooks_ui's DialogueBridgeConfig (the bridge itself does not link the engine):
//   auto v = fnvcoop_dlg::rulesViews(*rules);
//   bc.isRulesChoice = v.isRulesChoice;  bc.hasPlayerVars = v.hasPlayerVars;  bc.viewPlayerVar = v.viewPlayerVar;  bc.varName = nvhooks::questVarName;
struct RulesViews {
    std::function<bool(const FormRef& message)> isRulesChoice;                                              // a ChoiceConsent box (RulesEngine::isChoice)
    std::function<bool(const FormRef& quest)> hasPlayerVars;                                                // the quest has PerPlayerVars / PerPlayerState variables
    std::function<double(const FormRef& quest, const std::string& var, PlayerId slot, double real)> viewPlayerVar;   // what a condition for `slot` reads
};
inline RulesViews rulesViews(fnvcoop::RulesEngine& eng) {
    RulesViews v;
    v.isRulesChoice = [&eng](const FormRef& m) { return eng.isChoice(m); };
    v.hasPlayerVars = [&eng](const FormRef& q) { return !eng.perPlayerVarNames(q).empty(); };
    v.viewPlayerVar = [&eng](const FormRef& q, const std::string& var, PlayerId slot, double real) { return eng.viewPlayerVar(q, var, slot, real); };
    return v;
}

}  // namespace fnvcoop_dlg
