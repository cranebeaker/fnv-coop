// The identity layer for the lane-local world effects (WorldEffect). Same idea and the same helpers as plugin/hooks/identity.h (read its header
// comment): a world effect that names the SENDER's player is mapped to that player's puppet or dropped, by this table. Game-free.
//
// The 2026-10-29 bug lived here: the host's VCG01 gave the waking host player an AI package (SetScriptPackage 001055C4 on 00000014), the guest applied it
// to its own player and replayed the host's wake-up in a loop. The first fix was a filter in the guest's world-effect lambda (coop_module.cpp) that missed
// the baseline path; the translation is now in WorldApplier::applyWorld, which both paths go through.
#pragma once
#include <cstring>
#include <variant>

#include "identity.h"
#include "world_effects.h"

namespace nvworld {

inline constexpr nvhooks::IdentityRow kWorldRows[] = {
    // the sender's own client state: an actor value is its parts channel, essential / restrained / ignore-crime flags are how ITS game treats its player (a restrained
    // or essential guest character is the bug), the AI package is played by the sender's own character (a stream); none of them describes the puppet
    {"SetActorValueRef", "actor", nvhooks::PlayerRef::Drop},
    {"SetEssential", "actor", nvhooks::PlayerRef::Drop},
    {"SetRestrained", "actor", nvhooks::PlayerRef::Drop},
    {"SetScriptPackage", "actor", nvhooks::PlayerRef::Drop},
    {"SetIgnoreCrime", "actor", nvhooks::PlayerRef::Drop},
    // lock / open / destroyed state of the player object: nothing a door or container does
    {"SetObjectState", "ref", nvhooks::PlayerRef::Drop},
    // ownership of a reference is by the PLAYER: "owned by the player" (the command's own default) is whoever plays on a machine, so the owner stays the local player.
    // The owned thing is a world object; if it were the player itself the effect has no meaning
    {"SetOwner", "ref", nvhooks::PlayerRef::Drop},
    {"SetOwner", "owner", nvhooks::PlayerRef::Keep},
    {"SetCellOwner", "owner", nvhooks::PlayerRef::Keep},
};
inline constexpr const char* kWorldNoPlayerRef[] = {
    "SetRelation",            // faction against faction
    "SetFactionPlayerEnemy",  // a faction's "the player is its enemy" flag: the flag is about the player role, not one player
    "SetCellPublic",          // a cell flag
    "Present",                // image spaces, music and sounds are about the machine's screen and speakers
};

inline nvhooks::PlayerRef worldAct(const char* effect, const char* field) {
    return nvhooks::rowAct(kWorldRows, sizeof kWorldRows / sizeof kWorldRows[0], effect, field);
}

// `e` is rewritten in place. sender: the slot whose machine produced the effect (always the host for the world channel).
inline nvhooks::Verdict translateWorldIncoming(WorldEffect& e, fnvcoop::PlayerId sender, const nvhooks::IdentityView& v) {
    using nvhooks::Verdict;
    if (!v.on || FNV_MUT(67)) return Verdict::Unchanged;   // the same switch as the story effects: no translation at all
    Verdict verdict = Verdict::Unchanged;
    bool keep = true;
    auto fix = [&](fnvcoop::FormRef& r, const char* effect, const char* field) {
        nvhooks::PlayerRef act = worldAct(effect, field);
        if (FNV_MUT(79) && act == nvhooks::PlayerRef::Keep) act = nvhooks::PlayerRef::Puppet;   // twin 79: an owner that is the sender's player becomes its puppet (the owned thing is stolen from the local player)
        return nvhooks::translateRef(r, act, sender, v, &verdict);
    };
    std::visit(
        [&](auto& x) {
            using T = std::decay_t<decltype(x)>;
            if constexpr (std::is_same_v<T, SetActorValueRef>) keep = fix(x.actor, "SetActorValueRef", "actor");
            else if constexpr (std::is_same_v<T, SetEssential>) keep = fix(x.actor, "SetEssential", "actor");
            else if constexpr (std::is_same_v<T, SetRestrained>) keep = fix(x.actor, "SetRestrained", "actor");
            else if constexpr (std::is_same_v<T, SetScriptPackage>) keep = fix(x.actor, "SetScriptPackage", "actor");
            else if constexpr (std::is_same_v<T, SetIgnoreCrime>) keep = fix(x.actor, "SetIgnoreCrime", "actor");
            else if constexpr (std::is_same_v<T, SetObjectState>) keep = fix(x.ref, "SetObjectState", "ref");
            else if constexpr (std::is_same_v<T, SetOwner>) keep = fix(x.ref, "SetOwner", "ref") && fix(x.owner, "SetOwner", "owner");
            else if constexpr (std::is_same_v<T, SetCellOwner>) keep = fix(x.owner, "SetCellOwner", "owner");
        },
        e);
    if (!keep) { ++nvhooks::identityStats().dropped; return Verdict::Dropped; }
    return verdict;
}

}  // namespace nvworld
