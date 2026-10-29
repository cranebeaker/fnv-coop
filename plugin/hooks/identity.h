// The identity layer: ONE translation applied to every effect that arrives from another player's machine (sync-model.md, "Entities").
//
// Why. Every machine's capture names its own player the same way: PlayerRef, FalloutNV.esm 000014 ("P0"). The host's scripts act on the
// host's player, so a host effect that names P0 means THE HOST'S player, and on a guest 0x14 is the GUEST'S own character. Applied as
// written it hits the wrong person (2026-10-29: Doc's scene gave the waking host player an AI package, the guest applied it to itself and
// replayed the host's wake-up in a loop). Every incoming effect therefore goes through translateIncoming() before it reaches the engine:
// a reference to the SENDER's player is
//   Keep    left as written: the effect is explicitly addressed to this machine's own player (the rules put this player in `to`), or the
//           reference only means "the player" (an owner), so it resolves to the local player;
//   Puppet  replaced by the placeholder puppetRef(sender), which IdentityResolver turns into that player's puppet on this machine: the
//           character as something the world reacts to (a combat target, a MoveTo anchor, an activator, its faction);
//   Drop    the effect is not applied here: it is state of the sender's own client (its AI packages, restrained / essential flags, actor
//           values, visibility, position), which reaches this machine as the sender's stream / parts / events instead.
// A reference that names THIS machine's player the way the host does (the slot's wire reference, IdentityView::selfWire) is rewritten to
// P0, so an effect addressed to the local player by reference (a party warp) lands on the local player.
//
// The decision is a small explicit table keyed by effect type and field (kStoryRows below; kWorldRows in hooks_world/world_identity.h),
// with the reason for each row. Game-free; the unit tests are in hooks_world/tests/identity_tests.h (twins 67 to 80).
#pragma once
#include <cstdint>
#include <cstring>
#include <functional>
#include <utility>
#include <variant>

#include "engine_hooks.h"   // FormResolver (declaration only)
#include "fnvcoop/effect.h"
#include "fnvcoop/refop.h"
#include "fnvcoop/types.h"
#include "fnvcoop_mut.h"

namespace nvhooks {

enum class PlayerRef : uint8_t { Keep, Puppet, Drop };
enum class Verdict : uint8_t { Unchanged, Rewritten, Dropped };

inline const char* playerRefName(PlayerRef a) { return a == PlayerRef::Keep ? "keep" : a == PlayerRef::Puppet ? "puppet" : "drop"; }

// ---- placeholders ---------------------------------------------------------------------------------------------------------------------
// The FormRef of "player <slot>'s puppet on this machine". It never goes on the wire: translateIncoming() writes it, IdentityResolver reads it.
inline const char* puppetPlugin() { return "fnvcoop.puppet"; }
inline fnvcoop::FormRef puppetRef(fnvcoop::PlayerId slot) { return fnvcoop::FormRef(puppetPlugin(), slot); }
inline bool isPuppetRef(const fnvcoop::FormRef& r, fnvcoop::PlayerId* slot = nullptr) {
    if (r.plugin != puppetPlugin() || r.localId >= fnvcoop::kMaxSlots) return false;
    if (slot) *slot = static_cast<fnvcoop::PlayerId>(r.localId);
    return true;
}
// How a capture names its own player: FalloutNV.esm 000014 (the lower-cased name FormRef::make / makeRef produce).
inline fnvcoop::FormRef playerP0() { return fnvcoop::FormRef("falloutnv.esm", 0x14); }

// ---- what this machine knows ---------------------------------------------------------------------------------------------------------
struct IdentityHooks {
    std::function<fnvcoop::PlayerId()> self;                                  // this machine's party slot (kNoPlayer while unknown)
    std::function<bool(fnvcoop::PlayerId, fnvcoop::FormRef*)> wireRefOf;      // how the session names a slot's player (Runtime::playerRefOf)
    std::function<uint32_t(fnvcoop::PlayerId)> puppetOf;                      // refID of a remote player's puppet on this machine (0 = none)
};
inline IdentityHooks& identityHooks() { static IdentityHooks h; return h; }
inline bool& identityOn() { static bool on = false; return on; }
// Turns the layer on (a guest). The host never configures it: its own player is 0x14 for every effect it applies.
inline void configureIdentity(IdentityHooks h) { identityHooks() = std::move(h); identityOn() = true; }
inline void resetIdentity() { identityHooks() = IdentityHooks(); identityOn() = false; }
inline bool identityActive() { return identityOn(); }

struct IdentityStats {
    uint64_t dropped = 0, toPuppet = 0, toSelf = 0;
};
inline IdentityStats& identityStats() { static IdentityStats s; return s; }

// The pure inputs of a translation (tests build these by hand).
struct IdentityView {
    bool on = false;
    fnvcoop::PlayerId self = fnvcoop::kNoPlayer;   // this machine's slot; kNoPlayer = a guest that has not been told yet
    fnvcoop::FormRef p0 = playerP0();
    fnvcoop::FormRef selfWire;                     // this player as the session's rules name it (empty = unknown)
    bool (*opDefaultsToPlayer)(uint16_t op) = nullptr;   // RefCommand ops that run on the player when the script names no reference (ref_ops.h)
};
inline IdentityView currentIdentity() {
    IdentityView v;
    if (!identityOn()) return v;
    v.on = true;
    const IdentityHooks& h = identityHooks();
    if (h.self) v.self = h.self();
    if (h.wireRefOf && v.self != fnvcoop::kNoPlayer) {
        fnvcoop::FormRef w;
        if (h.wireRefOf(v.self, &w) && w != v.p0) v.selfWire = w;
    }
    return v;
}

// ---- the table ------------------------------------------------------------------------------------------------------------------------
// One row per (effect type, reference field) that can name a player. Effects that carry no actor reference are listed in kStoryNoPlayerRef,
// so the unit test can prove every Effect alternative was looked at. The table is the only place the choice is made.
struct IdentityRow {
    const char* effect;
    const char* field;
    PlayerRef act;
};
inline constexpr IdentityRow kStoryRows[] = {
    // visibility of the sender's character belongs to its owner (the avatar layer shows or hides the puppet from the owner's stream); a hidden puppet would fight it
    {"EnableRef", "ref", PlayerRef::Drop},
    {"DisableRef", "ref", PlayerRef::Drop},
    // the mover: the sender's position is its stream (the puppet is driven from snapshots); moved here it would teleport the wrong player
    {"MoveRef", "ref", PlayerRef::Drop},
    // the anchor of `Actor.MoveTo <marker>` (cell carries a marker reference): "next to the player" is next to that player's puppet on this machine
    {"MoveRef", "cell", PlayerRef::Puppet},
    // death is an owner event (death event + health part), not a console kill run on whoever 0x14 is here; the rules turn a player kill into a health drain addressed to a slot
    {"KillActor", "actor", PlayerRef::Drop},
    {"ResurrectActor", "actor", PlayerRef::Drop},
    // a script variable of the player object belongs to the sender's own game (there is no shared player script)
    {"SetScriptVar", "owner", PlayerRef::Drop},
    // recruiting the player as a companion is meaningless; if one ever arrives it must not make this player somebody's follower
    {"CompanionOwner", "npc", PlayerRef::Drop},
    {"ForcedDialogue", "npc", PlayerRef::Drop},
    // lock / texture / object state / ref variables / inventory / angle of the player object: the sender's own client state (inventory and look are its parts, rotation its stream)
    {"SetLock", "ref", PlayerRef::Drop},
    {"SwapTexture", "ref", PlayerRef::Drop},
    {"ObjectState", "ref", PlayerRef::Drop},
    {"RefScriptVar", "ref", PlayerRef::Drop},
    {"RefAddItem", "ref", PlayerRef::Drop},
    {"RefRemoveItem", "ref", PlayerRef::Drop},
    {"RemoveAllItems", "ref", PlayerRef::Drop},
    {"RefAngle", "ref", PlayerRef::Drop},
    // faction membership of the character: the AI of the NPCs this machine runs reads the puppet's factions to decide who to attack
    {"SetFactionRank", "actor", PlayerRef::Puppet},
    // RefCommand: the calling reference is decided per op (refCmdSubject below); a form ARGUMENT that names the player is a target (combat target, activator, look-at, disposition target)
    {"RefCommand", "args", PlayerRef::Puppet},
};
inline constexpr const char* kStoryNoPlayerRef[] = {
    // quests, journal, globals, weather, time, saves, session state: no actor reference at all
    "SetStage", "SetObjective", "SetGlobal", "StartQuest", "StopQuest", "CompleteQuest", "CompleteAllObjectives", "FailQuest", "AddTopic", "AddNote",
    "RemoveNote", "SetMapMarker", "SetFastTravelLock", "SetWeather", "SetTimeScale", "AdvanceTime", "RequestSave", "PartyTether", "SlideshowEnd",
    "SaveSuppress", "BarrierArrive", "BarrierRelease", "SayOnceFlag", "SpawnRef", "DespawnRef",
    // addressed to a player by Recipient (`to`), never by reference: the session already narrowed them to this machine's own player, which is exactly what they mean
    "AddItem", "RemoveItem", "ModActorValue", "ModReputation", "ModKarma", "SetPerk", "SetPlayerControls", "ForceActiveQuest",
};

inline PlayerRef rowAct(const IdentityRow* rows, size_t n, const char* effect, const char* field) {
    for (size_t i = 0; i < n; ++i)
        if (std::strcmp(rows[i].effect, effect) == 0 && std::strcmp(rows[i].field, field) == 0) return rows[i].act;
    return PlayerRef::Drop;   // a field nobody wrote a row for is never applied to the wrong player
}
inline PlayerRef storyAct(const char* effect, const char* field) { return rowAct(kStoryRows, sizeof kStoryRows / sizeof kStoryRows[0], effect, field); }

// RefCommand whose calling reference is the sender's player. The rules route a command on the PLAYER (a spell, a conditioned item, a build call) to the players
// it is meant for, so a command that reaches this machine addressed to it (`to`) is data of THIS character and is kept. Everything else is the sender's body:
// its animation, AI and presentation (PlayIdle, PlayGroup, EvaluatePackage, StartCombat, Look, SetUnconscious, SetGhost, FireWeapon, UseWeapon, Rotate, ...).
inline bool refCmdIsCharacterData(uint16_t op) {
    using namespace fnvcoop;
    switch (op) {
        case kOpAddSpell: case kOpRemoveSpell: case kOpDispel: case kOpDispelAllSpells: case kOpCastImmediateOnSelf:         // spells and active effects
        case kOpEquipItem: case kOpUnequipItem: case kOpResetInventory: case kOpDropMe: case kOpDrop: case kOpRemoveMe:     // gear and inventory
        case kOpAddItemHealthPercent: case kOpModFactionRank: case kOpAddCardToPlayer:                                     // rewards
        case kOpSetPlayerTagSkill: case kOpAddSPECIALPoints: case kOpSetSPECIALPoints: case kOpSetTagSkills: case kOpAddTagSkills:   // character build
        case kOpSetLevel: case kOpResetXP: case kOpAgeRace: case kOpSexChange: case kOpSetPCCanUsePowerArmor: case kOpSetPCSleepHours:
        case kOpWakeUpPC: case kOpSetHardcore: case kOpSetInChargen: case kOpOpenTeammateContainer:
            return true;
        default:
            return false;
    }
}
inline bool recipientIncludes(const fnvcoop::Recipient& to, fnvcoop::PlayerId self) {
    if (to.mode == fnvcoop::Recipient::All) return true;
    if (to.mode != fnvcoop::Recipient::Mask) return false;
    return self == fnvcoop::kNoPlayer ? to.mask != 0 : to.includes(self);
}

// ---- the translation ------------------------------------------------------------------------------------------------------------------
enum class WhoIs : uint8_t { Nobody, Sender, Me };
inline WhoIs whoIs(const fnvcoop::FormRef& r, fnvcoop::PlayerId sender, const IdentityView& v) {
    if (!v.selfWire.plugin.empty() && r == v.selfWire && !FNV_MUT(73)) return WhoIs::Me;   // twin 73: the slot's own wire reference is not recognised
    if (r == v.p0) return sender == v.self && !FNV_MUT(74) ? WhoIs::Me : WhoIs::Sender;    // twin 74: a machine's own effects are translated as if a remote player sent them
    return WhoIs::Nobody;
}

// Applies one table decision to one reference. False = the whole effect must be dropped.
inline bool translateRef(fnvcoop::FormRef& r, PlayerRef act, fnvcoop::PlayerId sender, const IdentityView& v, Verdict* verdict) {
    switch (whoIs(r, sender, v)) {
        case WhoIs::Nobody: return true;
        case WhoIs::Me:
            if (r != v.p0) { r = v.p0; if (*verdict == Verdict::Unchanged) *verdict = Verdict::Rewritten; ++identityStats().toSelf; }
            return true;
        case WhoIs::Sender: break;
    }
    if (FNV_MUT(68) && act == PlayerRef::Puppet) act = PlayerRef::Drop;     // twin 68: a target that is the sender's player is dropped (the NPC never fights the puppet)
    if (FNV_MUT(69) && act == PlayerRef::Puppet) act = PlayerRef::Keep;     // twin 69: a target stays P0 (the NPC fights this machine's own player)
    if (FNV_MUT(70) && act == PlayerRef::Drop) act = PlayerRef::Puppet;     // twin 70: the sender's own client state lands on its puppet
    if (FNV_MUT(75) && act == PlayerRef::Drop) act = PlayerRef::Keep;       // twin 75: the sender's own client state lands on this machine's own player (the 2026-10-29 bug)
    switch (act) {
        case PlayerRef::Keep: return true;
        case PlayerRef::Puppet:
            r = puppetRef(sender);
            if (*verdict == Verdict::Unchanged) *verdict = Verdict::Rewritten;
            ++identityStats().toPuppet;
            return true;
        case PlayerRef::Drop: break;
    }
    return false;
}

inline Verdict translateRefCommand(fnvcoop::RefCommand& c, fnvcoop::PlayerId sender, const IdentityView& v) {
    Verdict verdict = Verdict::Unchanged;
    // `player.X` written without a reference runs on the player: make that explicit so the same decision covers it
    bool implicit = false;
    if (c.ref.plugin.empty() && v.opDefaultsToPlayer && v.opDefaultsToPlayer(c.op) && !FNV_MUT(76)) {   // twin 76: an implicit player stays implicit (it runs on this machine's own player)
        c.ref = v.p0;
        implicit = true;
    }
    if (whoIs(c.ref, sender, v) == WhoIs::Sender) {
        bool keep = refCmdIsCharacterData(c.op) && (recipientIncludes(c.to, v.self) || FNV_MUT(78));   // twin 78: a command that was not addressed to this player lands on it anyway
        if (FNV_MUT(71)) keep = true;    // twin 71: the sender's body commands (animations, AI orders) land on this machine's own player
        if (FNV_MUT(72)) keep = false;   // twin 72: a command addressed to this player never arrives (rewards, character build)
        if (!keep) return Verdict::Dropped;
        if (implicit) c.ref = fnvcoop::FormRef();   // back to "the player" (P0 resolves to this machine's player either way)
    } else {
        if (implicit) c.ref = fnvcoop::FormRef();   // it was already this machine's own player: nothing to make explicit
        else if (!translateRef(c.ref, PlayerRef::Keep, sender, v, &verdict)) return Verdict::Dropped;   // the slot's wire reference becomes P0
    }
    const PlayerRef args = storyAct("RefCommand", "args");
    fnvcoop::FormRef* forms[] = {&c.a, &c.b, &c.c, &c.d};
    for (fnvcoop::FormRef* f : forms)
        if (!translateRef(*f, args, sender, v, &verdict)) return Verdict::Dropped;
    return verdict;
}

// `e` is rewritten in place. `sender` is the slot whose machine produced the effect (EffectTxn::origin; the host for baseline state).
inline Verdict translateIncoming(fnvcoop::Effect& e, fnvcoop::PlayerId sender, const IdentityView& v) {
    using namespace fnvcoop;
    if (!v.on || FNV_MUT(67)) return Verdict::Unchanged;   // twin 67: no translation (every host effect lands on this machine's own player)
    Verdict verdict = Verdict::Unchanged;
    auto fix = [&](FormRef& r, const char* effect, const char* field) { return translateRef(r, storyAct(effect, field), sender, v, &verdict); };
    bool keep = true;
    std::visit(
        [&](auto& x) {
            using T = std::decay_t<decltype(x)>;
            if constexpr (std::is_same_v<T, EnableRef>) keep = fix(x.ref, "EnableRef", "ref");
            else if constexpr (std::is_same_v<T, DisableRef>) keep = fix(x.ref, "DisableRef", "ref");
            else if constexpr (std::is_same_v<T, MoveRef>) keep = fix(x.ref, "MoveRef", "ref") && fix(x.cell, "MoveRef", "cell");
            else if constexpr (std::is_same_v<T, KillActor>) keep = fix(x.actor, "KillActor", "actor");
            else if constexpr (std::is_same_v<T, ResurrectActor>) keep = fix(x.actor, "ResurrectActor", "actor");
            else if constexpr (std::is_same_v<T, SetScriptVar>) keep = fix(x.owner, "SetScriptVar", "owner");
            else if constexpr (std::is_same_v<T, CompanionOwner>) keep = fix(x.npc, "CompanionOwner", "npc");
            else if constexpr (std::is_same_v<T, ForcedDialogue>) keep = fix(x.npc, "ForcedDialogue", "npc");
            else if constexpr (std::is_same_v<T, SetLock>) keep = fix(x.ref, "SetLock", "ref");
            else if constexpr (std::is_same_v<T, SwapTexture>) keep = fix(x.ref, "SwapTexture", "ref");
            else if constexpr (std::is_same_v<T, ObjectState>) keep = fix(x.ref, "ObjectState", "ref");
            else if constexpr (std::is_same_v<T, RefScriptVar>) keep = fix(x.ref, "RefScriptVar", "ref");
            else if constexpr (std::is_same_v<T, RefAddItem>) keep = fix(x.ref, "RefAddItem", "ref");
            else if constexpr (std::is_same_v<T, RefRemoveItem>) keep = fix(x.ref, "RefRemoveItem", "ref");
            else if constexpr (std::is_same_v<T, RemoveAllItems>) keep = fix(x.ref, "RemoveAllItems", "ref");
            else if constexpr (std::is_same_v<T, RefAngle>) keep = fix(x.ref, "RefAngle", "ref");
            else if constexpr (std::is_same_v<T, SetFactionRank>) keep = fix(x.actor, "SetFactionRank", "actor");
            else if constexpr (std::is_same_v<T, RefCommand>) {
                const Verdict rc = translateRefCommand(x, sender, v);
                keep = rc != Verdict::Dropped;
                if (rc == Verdict::Rewritten) verdict = Verdict::Rewritten;
            }
        },
        e);
    if (!keep) { ++identityStats().dropped; return Verdict::Dropped; }
    return verdict;
}

// "Is this effect mine?" decisions (CompanionOwner names a party slot) need THIS machine's slot. RemoteApplier::localSlot_ was only ever set by applyTxn (tests), so in the
// game it stayed 0 on every guest: a guest took the HOST's recruit as its own companion and never took its own. The guest's slot comes from the identity hooks; the
// host (layer off) keeps the applier's own value.
inline fnvcoop::PlayerId localSlotOf(const IdentityView& v, fnvcoop::PlayerId fallback) {
    if (FNV_MUT(80)) return fallback;   // twin 80: the guest still believes it is slot 0
    return v.on && v.self != fnvcoop::kNoPlayer ? v.self : fallback;
}

// ---- FormResolver decorator -----------------------------------------------------------------------------------------------------------
// The placeholder puppetRef(slot) resolves to that player's puppet; everything else is the inner resolver's.
class IdentityResolver : public FormResolver {
public:
    explicit IdentityResolver(FormResolver& inner) : inner_(inner) {}
    bool toRef(uint32_t refID, fnvcoop::FormRef* out) override { return inner_.toRef(refID, out); }
    uint32_t fromRef(const fnvcoop::FormRef& r) override {
        fnvcoop::PlayerId slot = fnvcoop::kNoPlayer;
        if (isPuppetRef(r, &slot)) {
            if (FNV_MUT(77)) slot = fnvcoop::kHostPlayer;   // twin 77: every player's placeholder resolves to the host's puppet
            const IdentityHooks& h = identityHooks();
            return h.puppetOf ? h.puppetOf(slot) : 0u;
        }
        return inner_.fromRef(r);
    }
    void* lookup(uint32_t refID) override { return inner_.lookup(refID); }

private:
    FormResolver& inner_;
};

}  // namespace nvhooks
