// Player-ref condition evaluation for co-op (game-free logic; the detours live in engine_hooks.cpp).
//
// Condition evaluators (the per-function "eval" handlers of the command table: bool(thisObj, param1, param2, double* result), cdecl)
// are called with the local player (ref 0x14) as an operand. For party-aware functions the detour re-evaluates the SAME condition once per
// party member with that operand swapped for the member's ghost reference, and hands the per-member values to a reducer (L4
// RulesEngine::reduceCondition: initiator / any-party-member / min-over-party / entry:all-in,exit:any-in).
//
// Meaning of "player" per function (internal lane notes round 9):
//   spatial   GetDistance, GetInCell, GetInSameCell : any / min over the party (rules.json decides the reduction)
//   awareness IsInCombat, GetDetected, GetHealthPercentage : per member; rules.json decides (default: initiator)
//   inventory GetItemCount, GetActorValue : per member (MemberView::StateOrGhost: what the guest reported, else its puppet); rules.json decides (sum-over-party for pooled collections, max-over-party for
//             "the strongest member" checks, default: initiator)
//   identity  GetInWorldspace, IsInInterior (thisObj = the player), IsKiller (param1 = the player): the ghost answers, like the spatial group
//   state     GetEquipped, HasPerk, IsSneaking, GetHasNote, IsPlayerInRegion: a ghost has no perks, notes, region list or equip state, so the
//             answer for every member but the local player comes from the StateProvider the session wires (unknown = that member is skipped)
//   owner     GetPlayerTeammate: the operand is an NPC; a member's value is "the NPC is a teammate AND that member owns it" (companion owner table)
//   wave 1b   GetLevel (also the `Player.GetLevel` command, which calls the same evaluator), GetIsSex, GetSitting: the player is thisObj, a ghost does not carry the
//             guest's level / sex / seat, so the StateProvider answers (state). GetPCIsSex and IsPlayerActionActive read the GLOBAL player (always + state).
//             GetShouldAttack: thisObj is the NPC, param1 the target (ghost swap of the target).
//   not hooked, on purpose (see condCoverage): GetActionRef / IsActionRef (script commands only, the activator is already the acting player: coop_module runs a guest's
//             activation as 0x14), GetIsReference (a reference identity test, every member is "the player"), GetMapMarkerVisible (a marker flag, no player operand),
//             GetTalkedToPC (the NPC's own flag).
#pragma once
#include <cstdint>
#include <functional>
#include <string>
#include <vector>

#include "fnvcoop/rules.h"

namespace nvhooks {

enum class CondFunc : uint8_t {
    GetDistance, GetInCell, GetInSameCell, IsInCombat, GetDetected, GetHealthPercentage, GetItemCount, GetActorValue,
    GetInWorldspace, IsKiller, GetEquipped, GetPlayerTeammate, GetHasNote, HasPerk, IsSneaking, IsPlayerInRegion, IsInInterior,
    GetLevel, GetIsSex, GetPCIsSex, GetSitting, GetShouldAttack, IsPlayerActionActive, Count };
const char* condFuncName(CondFunc f);  // the name rules.json uses (lower-cased by the rules engine)

constexpr uint32_t kHostPlayerRef = 0x14;

struct PartyMember {
    uint8_t slot = 0;
    uint32_t refID = 0;  // 0x14 for the local player, the ghost reference of every other present member
};
struct CondValue {
    uint8_t slot = 0;
    double value = 0;
};

// Which operands of the call are "the player" and get swapped. param1 is only inspected for functions whose param1 is a reference
// (GetDistance, GetInSameCell, GetDetected); `param1RefID` must be 0 when param1 is not a form.
struct SwapPlan {
    bool thisObj = false;
    bool param1 = false;
    bool always = false;  // the engine evaluator ignores its operands and reads the global player (GetHasNote, IsPlayerInRegion), or the operand is an NPC (GetPlayerTeammate)
    bool any() const { return thisObj || param1 || always; }
};
SwapPlan planSwap(CondFunc f, uint32_t thisObjRefID, uint32_t param1RefID);
bool param1IsReference(CondFunc f);

// The condition functions rules.json can name under PartyAwareCond, and what the plugin does about each. `Hooked` = a CondFunc exists (evaluator detour or command
// handler); the others are decisions, each with its reason in party_cond.cpp. A name that is none of them is a rule nobody can run (a test walks rules.json).
enum class CondCoverage : uint8_t { Hooked, NoPlayerOperand, ActingPlayerRuns, Unknown };
CondCoverage condCoverage(const std::string& funcName);

// How the value of a member OTHER than the local player is obtained.
enum class MemberView : uint8_t {
    Ghost,  // run the engine evaluator with the player operand replaced by the member's ghost reference
    State,  // ask the StateProvider (a ghost cannot answer); no provider or "unknown" = the member is skipped
    Owner,  // engine flag of the NPC AND the StateProvider's "this member owns it" (unknown owner = the engine flag alone)
    StateOrGhost,  // guest pack facts: GetItemCount and GetActorValue ask the StateProvider first (a guest's weight / watched item counts), and the ghost when it does not know
};
MemberView memberView(CondFunc f);
// The player-fact kind (player_facts.h) a condition function reads: 1 actor value, 2 item count, 3 has perk, 4 equipped; 0 = not a fact function.
uint8_t factKindOf(CondFunc f);

// One question to the state provider. thisRefID: runtime id of the calling reference (the NPC for GetPlayerTeammate, else the player 0x14).
// formID: runtime id of param1 when it is a form (note, perk, item, region, worldspace), else 0. arg: param2 as an integer (HasPerk: the
// "alternate" flag).
struct StateQuery {
    CondFunc f = CondFunc::Count;
    uint8_t slot = 0;
    uint32_t thisRefID = 0;
    uint32_t formID = 0;
    int32_t arg = 0;
    int32_t arg1 = 0;   // param1 when it is a plain integer, not a form (GetIsSex: 0 male / 1 female; IsPlayerActionActive: the action id); 0 otherwise
};
using StateProvider = std::function<bool(const StateQuery&, double* value)>;

// What evalMember may call. All three are closures over the original evaluator call (operands already captured).
struct CondOps {
    std::function<bool(double*)> evalLocal;                       // the original evaluator, operands untouched (the local player)
    std::function<bool(uint32_t ghostRefID, double*)> evalGhost;  // operands swapped for that reference; false = not loaded
    StateProvider state;                                          // may be empty
};
// One member's value under the function's MemberView. False = the member cannot be evaluated and is skipped.
bool evalMember(CondFunc f, const PartyMember& m, const StateQuery& base, const CondOps& ops, double* value);

// Evaluates the condition once per member. `evalAs(member, &value)` runs the original evaluator with the player operand(s) replaced by
// that member; false = member could not be evaluated (ghost not loaded) and is skipped. Returns false when fewer than two members were
// evaluated (caller then uses the plain engine result). Otherwise *reduced is the reducer's answer.
using MemberEval = std::function<bool(const PartyMember&, double* value)>;
using Reducer = std::function<double(const std::string& func, const std::vector<CondValue>&, uint8_t initiator)>;
// The reducer the evaluators call: RulesEngine::reduceCondition behind a thin adapter (rules_reducer.h). `phase` is Entry unless the caller
// opened a ConditionQuestScope with CondPhase::Exit (a trigger that is already occupied asks "is anybody still inside").
using ConditionReducer = std::function<double(const fnvcoop::FormRef& quest, const std::string& func, const std::vector<CondValue>&,
                                              uint8_t initiator, fnvcoop::CondPhase phase)>;

bool evaluateOverParty(CondFunc f, const std::vector<PartyMember>& members, const MemberEval& evalAs, const Reducer& reduce, uint8_t initiator,
                       double* reduced, std::vector<CondValue>* valuesOut = nullptr);

}  // namespace nvhooks
