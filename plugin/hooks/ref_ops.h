// Story-capture: the engine-facing table of the generic RefCommand effect (docs/tools/story-capture.md). Game-free and unit-tested
// (tests/test_story.cpp); the detours that use it are in engine_hooks.cpp, the apply in apply_remote.cpp.
//
// One RefOpSpec per script command: which engine handler to hook, how ExtractArgs returns its arguments, where each one lands in the
// RefCommand, and who may perform it on the receiving side. The wire / merge side of the ops (event or state, key slots, default recipient) is
// in synccore/include/fnvcoop/refop.h; test_story.cpp checks that both tables describe the same ops.
//
// Argument letters (one per ENGINE parameter, in engine order; the engine's own ParamInfo type ids are pinned in data/story_cmd_params.json,
// generated from a reference exe by tools/gen_story_params.py and checked by the unit tests):
//   o  a form (ObjectRef / Actor / Spell / Quest / Cell / ...), returned as TESForm*      -> RefCommand.a .b (.c .d on a wide op)
//   i  an integer                                                                          -> RefCommand.i .j .k .l (.m on a wide op)
//   f  a float                                                                              -> RefCommand.f
//   s  a text (a char buffer of kStoryTextMax bytes is handed to ExtractArgs)                                         -> RefCommand.s
//   c  a critical stage (index into g_criticalStageNames, printed as its name)                                            -> an int slot
//   g  an animation group id (index into the engine's g_animGroupInfos, printed as its name: story_animgroups.h)  -> an int slot
//   v  an actor value (skill) code                                                          -> an int slot; printed as the skill's name
//   x  an axis ('X' 'Y' 'Z')                                                                 -> an int slot holding 0 1 2
#pragma once
#include <cstdint>
#include <functional>
#include <string>

#include "fnvcoop/effect.h"
#include "fnvcoop/refop.h"
#include "hook_patch.h"

namespace nvhooks {

constexpr int kStoryTextMax = 256;   // ExtractArgs copies a text parameter into a buffer this big (xNVSE passes char[256] the same way)
constexpr int kMaxStoryArgs = 9;   // ExtractArgs is handed this many output pointers; no hooked command has more parameters (UseWeapon has nine)

enum RefOpFlags : uint8_t {
    kRoAiOrder = 1,       // an order to the actor's AI: only the machine that runs that actor's AI performs it (the others show its snapshots)
    kRoNeedsRef = 2,      // the engine command needs a calling reference (its `needsParent` flag): captured only with one, applied with one
    kRoPlayerRef = 4,     // with kRoNeedsRef: a command with no reference given in the script runs on the player (0x14)
    kRoSharedHook = 8,    // the handler is also hooked by engine_hooks.cpp itself (L5 round 7): its detour calls the story capture, no second detour
    kRoActivate = 16,     // Activate: performed only on object types where a scripted activation is safe (not containers, terminals, actors)
};

struct RefOpSpec {
    uint16_t op;                 // fnvcoop::RefOp
    const char* fn;              // the census / coverage function key (lower case script name)
    const HookSpec* hook;        // the engine handler (story_addrs.h, or engine_addrs.h for kRoSharedHook)
    const char* sig;             // engine parameters, in order (see above)
    const char* slots;           // destination slot per parameter: a b (forms)  i j k l (ints)  f (float); wide ops (refOpIsWide) also c d (forms) and m (int)
    uint8_t minArgs;             // required parameters (the rest are optional in the engine)
    uint8_t flags;               // RefOpFlags
};

extern const RefOpSpec kStorySpecs[];
extern const size_t kStorySpecCount;

const RefOpSpec* storySpecFor(uint16_t op);
const RefOpSpec* storySpecForFn(const std::string& lowerFn);

// ---- capture ----------------------------------------------------------------------------------------------------------------------
// ExtractArgs outputs, decoded per the signature letters by the engine glue. `form` is the runtime refID of the returned form (0 = none).
struct ArgValue {
    uint32_t form = 0;
    int32_t i = 0;
    float f = 0;
    std::string s;   // text parameters
};
using RefToForm = std::function<bool(uint32_t refID, fnvcoop::FormRef* out)>;
// callingRef: refID of the command's calling reference (0 = none). False (with *why) when the command should not be captured: missing
// calling reference for a command that needs one, a form that has no stable name, a bad axis.
bool buildRefCommand(const RefOpSpec& s, uint32_t callingRef, const ArgValue* args, const RefToForm& toForm, fnvcoop::RefCommand* out,
                     std::string* why = nullptr);

// ---- apply ------------------------------------------------------------------------------------------------------------------------
using FormToRef = std::function<uint32_t(const fnvcoop::FormRef&)>;   // 0 = not loaded here
struct LineResult {
    bool ok = false;
    bool formMissing = false;    // a referenced form is not loaded on this machine
    std::string line;            // console line
    uint32_t callingRef = 0;     // run the line with this reference as the console's calling reference (0 = none)
    std::string why;
};
// The console line (and calling reference) that performs `c` on this machine. Does not decide WHETHER to perform it (see opPerformedHere).
LineResult buildRefCommandLine(const fnvcoop::RefCommand& c, const FormToRef& fromForm);
// Who performs the command: false = leave it to the machine that runs the actor's AI. `remoteDriven` answers "this machine does NOT run the AI
// of that reference" (actor ownership); unset or an untracked actor means perform it.
bool opPerformedHere(const RefOpSpec& s, uint32_t callingRef, const std::function<bool(uint32_t refID)>& remoteDriven);
// True for an op that runs on the player (0x14) when the script names no calling reference (kRoPlayerRef). The identity layer (identity.h) reads it so an
// implicit "player.X" is translated like an explicit one.
bool refOpDefaultsToPlayer(uint16_t op);
// Activate is performed only on object types where it cannot open a menu or start a conversation on a machine that did not ask for it.
bool activateTypeSafe(unsigned formTypeId);

// ForceActiveQuest <quest>: the quest the player's journal focuses on (an existing Effect type, ForceActiveQuest).
std::string forceActiveQuestCmd(uint32_t questRefID);

// The skill name the console accepts for an actor-value code of SetPlayerTagSkill (eActorVal_Barter .. eActorVal_Unarmed); nullptr otherwise.
const char* skillNameOf(int32_t av);

}  // namespace nvhooks
