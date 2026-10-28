#include "party_cond.h"

#include <cctype>

namespace nvhooks {

const char* condFuncName(CondFunc f) {
    switch (f) {
        case CondFunc::GetDistance: return "GetDistance";
        case CondFunc::GetInCell: return "GetInCell";
        case CondFunc::GetInSameCell: return "GetInSameCell";
        case CondFunc::IsInCombat: return "IsInCombat";
        case CondFunc::GetDetected: return "GetDetected";
        case CondFunc::GetHealthPercentage: return "GetHealthPercentage";
        case CondFunc::GetItemCount: return "GetItemCount";
        case CondFunc::GetActorValue: return "GetActorValue";
        case CondFunc::GetInWorldspace: return "GetInWorldspace";
        case CondFunc::IsKiller: return "IsKiller";
        case CondFunc::GetEquipped: return "GetEquipped";
        case CondFunc::GetPlayerTeammate: return "GetPlayerTeammate";
        case CondFunc::GetHasNote: return "GetHasNote";
        case CondFunc::HasPerk: return "HasPerk";
        case CondFunc::IsSneaking: return "IsSneaking";
        case CondFunc::IsPlayerInRegion: return "IsPlayerInRegion";
        case CondFunc::IsInInterior: return "IsInInterior";
        case CondFunc::GetLevel: return "GetLevel";
        case CondFunc::GetIsSex: return "GetIsSex";
        case CondFunc::GetPCIsSex: return "GetPCIsSex";
        case CondFunc::GetSitting: return "GetSitting";
        case CondFunc::GetShouldAttack: return "GetShouldAttack";
        case CondFunc::IsPlayerActionActive: return "IsPlayerActionActive";
        default: return "";
    }
}

bool param1IsReference(CondFunc f) {
#ifdef NVHOOKS_MUTANT
    if (f == CondFunc::GetShouldAttack) return false;   // mutant: the target of GetShouldAttack is never swapped for the member's ghost
#endif
    return f == CondFunc::GetDistance || f == CondFunc::GetInSameCell || f == CondFunc::GetDetected || f == CondFunc::IsKiller || f == CondFunc::GetShouldAttack;
}

CondCoverage condCoverage(const std::string& name) {
    std::string n;
    for (char c : name) n.push_back(static_cast<char>(std::tolower(static_cast<unsigned char>(c))));
    if (n == "getav") n = "getactorvalue";   // rules.json spells the actor-value function both ways
    for (int i = 0; i < static_cast<int>(CondFunc::Count); ++i) {
        std::string h;
        for (const char* c = condFuncName(static_cast<CondFunc>(i)); *c; ++c) h.push_back(static_cast<char>(std::tolower(static_cast<unsigned char>(*c))));
        if (h == n) return CondCoverage::Hooked;
    }
    // GetIsReference: a reference identity test (thisObj == param1); "the player" is the same reference for every member, so there is nothing to swap.
    // GetMapMarkerVisible: a flag on the map marker reference. GetTalkedToPC: a flag on the NPC. None of the three reads a player.
    if (n == "getisreference" || n == "getmapmarkervisible" || n == "gettalkedtopc") return CondCoverage::NoPlayerOperand;
    // GetActionRef / IsActionRef have no condition evaluator (their eval column is 0): script commands only. The activator they compare is the acting player of the
    // intent (coop_module runs a guest's activation as the host's 0x14, see plugin/intents ActorMode), so `IsActionRef player` already holds for the guest who acted.
#ifdef NVHOOKS_MUTANT
    if (n == "getactionref") return CondCoverage::Unknown;   // mutant: a function a rule names is forgotten
#endif
    if (n == "getactionref" || n == "isactionref") return CondCoverage::ActingPlayerRuns;
    // GetButtonPressed and EnterTrigger are not player-state reads: GetButtonPressed answers the script's own box (UiRouter), EnterTrigger is the trigger event.
    if (n == "getbuttonpressed" || n == "entertrigger") return CondCoverage::NoPlayerOperand;
    return CondCoverage::Unknown;
}

namespace {
// Functions whose evaluator never looks at the calling reference (GetHasNote and IsPlayerInRegion read the global player) or whose operand
// is an NPC (GetPlayerTeammate): the player is not an operand to swap, so a call is party-aware whenever there is a party.
bool alwaysPartyAware(CondFunc f) {
    return f == CondFunc::GetHasNote || f == CondFunc::IsPlayerInRegion || f == CondFunc::GetPlayerTeammate || f == CondFunc::GetPCIsSex ||
           f == CondFunc::IsPlayerActionActive;
}
}  // namespace

SwapPlan planSwap(CondFunc f, uint32_t thisObjRefID, uint32_t param1RefID) {
    SwapPlan p;
#ifdef NVHOOKS_MUTANT
    p.always = false;   // mutant: GetHasNote / IsPlayerInRegion / GetPlayerTeammate are never party-aware (the host's own answer wins)
#else
    p.always = alwaysPartyAware(f);
#endif
    // IsKiller's calling reference is the dead actor, never the player: only its param1 (the claimed killer) is a player operand.
    p.thisObj = f != CondFunc::IsKiller && !p.always && thisObjRefID == kHostPlayerRef;
    p.param1 = param1IsReference(f) && param1RefID == kHostPlayerRef;
    return p;
}

MemberView memberView(CondFunc f) {
#ifdef NVHOOKS_MUTANT
    if (f == CondFunc::GetLevel || f == CondFunc::GetItemCount || f == CondFunc::GetActorValue) return MemberView::Ghost;   // mutant: a guest's level / pack is read off its ghost (a puppet NPC with its own level and pack)
#endif
    switch (f) {
        case CondFunc::GetEquipped:
        case CondFunc::HasPerk:
        case CondFunc::IsSneaking:
        case CondFunc::GetHasNote:
        case CondFunc::IsPlayerInRegion:
        case CondFunc::GetLevel:   // a ghost is a puppet NPC: its level is not the guest's
        case CondFunc::GetIsSex:
        case CondFunc::GetPCIsSex:
        case CondFunc::GetSitting:
        case CondFunc::IsPlayerActionActive: return MemberView::State;
        case CondFunc::GetPlayerTeammate: return MemberView::Owner;
        case CondFunc::GetItemCount:   // guest pack facts: the puppet does not carry the guest's pack
        case CondFunc::GetActorValue: return MemberView::StateOrGhost;
        default: return MemberView::Ghost;
    }
}

uint8_t factKindOf(CondFunc f) {
    switch (f) {
        case CondFunc::GetActorValue: return 1;
        case CondFunc::GetItemCount: return 2;
        case CondFunc::HasPerk: return 3;
        case CondFunc::GetEquipped: return 4;
        default: return 0;
    }
}

bool evalMember(CondFunc f, const PartyMember& m, const StateQuery& base, const CondOps& ops, double* value) {
    const MemberView view = memberView(f);
    StateQuery q = base;
    q.f = f;
    q.slot = m.slot;
    if (view == MemberView::Owner) {
        // The engine flag says the NPC is somebody's teammate; the provider says whose. No flag = nobody's, whatever the table says.
        double engine = 0;
        if (!ops.evalLocal || !ops.evalLocal(&engine)) return false;
        if (engine == 0) { *value = 0; return true; }
        double owned = 0;
#ifdef NVHOOKS_MUTANT
        *value = engine;  // mutant: every member counts as the owner
#else
        *value = (ops.state && ops.state(q, &owned)) ? owned : engine;
#endif
        return true;
    }
    if (m.refID == kHostPlayerRef) return ops.evalLocal && ops.evalLocal(value);
#ifdef NVHOOKS_MUTANT
    if (view == MemberView::State) return ops.evalGhost && ops.evalGhost(m.refID, value);  // mutant: a ghost is asked for perks / notes / sneak state
#else
    if (view == MemberView::State) return ops.state && ops.state(q, value);
#endif
    if (view == MemberView::StateOrGhost && ops.state && ops.state(q, value)) return true;   // what the guest reported; unknown = its puppet answers as before
    return ops.evalGhost && ops.evalGhost(m.refID, value);
}

bool evaluateOverParty(CondFunc f, const std::vector<PartyMember>& members, const MemberEval& evalAs, const Reducer& reduce, uint8_t initiator,
                       double* reduced, std::vector<CondValue>* valuesOut) {
    std::vector<CondValue> vals;
    vals.reserve(members.size());
    for (const PartyMember& m : members) {
        double v = 0;
        if (evalAs(m, &v)) vals.push_back({m.slot, v});
    }
    if (vals.size() < 2) return false;
    if (valuesOut) *valuesOut = vals;
    *reduced = reduce ? reduce(condFuncName(f), vals, initiator) : vals.front().value;
    return true;
}

}  // namespace nvhooks
