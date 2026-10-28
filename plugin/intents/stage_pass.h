// A guest's SetStage intent on a BARRIER stage is that player's pass (G26, wave 1b). Game-free.
//
// The rules engine holds a Barrier stage (chargen end VCG01 200, the ending) until every joined player passed. A guest's own game reaches the stage by itself (its
// chargen script ends); its SetStage is forwarded to the host as an intent. The host must not run it: it counts as the guest's pass, and the engine performs the stage
// itself on release (RulesEngine::applyStage). Anything that is not a pass stage is handled as before (ExclusiveStages policy first, then the real SetStage).
//
// The plugin wires it once per session next to the rules engine:
//   nvcoop::intents::setGuestStagePassHook([&eng](PlayerId s, const FormRef& q, uint32_t st) { return nvcoop::intents::stagePassOf(eng, s, q, st); });
#pragma once
#include <cstdint>
#include <functional>

#include "fnvcoop/rules.h"

namespace nvcoop::intents {

using fnvcoop::FormRef;
using fnvcoop::PlayerId;

enum class StagePass : uint8_t {
    NotPass,   // the stage is not a pass stage: unchanged behaviour
    Proceed,   // a pass stage, but nothing is held (the party is alone / everybody already passed): run the intent as usual
    Deferred,  // counted as the player's pass; the stage waits for the whole party and the engine performs it on release: do NOT run it
};
using GuestStagePassFn = std::function<StagePass(PlayerId slot, const FormRef& quest, uint32_t stage)>;

inline StagePass stagePassOf(fnvcoop::RulesEngine& eng, PlayerId slot, const FormRef& quest, uint32_t stage) {
    if (!eng.isPassStage(quest, stage)) return StagePass::NotPass;
    return eng.onGuestStageIntent(slot, quest, stage) == fnvcoop::StageDecision::Deferred ? StagePass::Deferred : StagePass::Proceed;
}

// The one hook of the process (set by the session code that owns the rules engine, cleared with nullptr when the engine goes away).
GuestStagePassFn& guestStagePassHook();
inline void setGuestStagePassHook(GuestStagePassFn f) { guestStagePassHook() = std::move(f); }

}  // namespace nvcoop::intents
