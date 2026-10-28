// Effects: primitive world mutations captured while a (host-run) script executes.
// One script run = one EffectTxn. Clients apply effects directly; they never re-run scripts.
#pragma once
#include <cstdint>
#include <string>
#include <variant>
#include <vector>
#include "fnvcoop/types.h"

namespace fnvcoop {

// Integer deltas on purpose: replicas must stay bit-identical (XP/caps/karma/rep change in whole units).
struct SetStage {
    FormRef quest;
    uint32_t stage = 0;
    bool operator==(const SetStage& o) const { return quest == o.quest && stage == o.stage; }
};
// flags: bit0 = displayed, bit1 = completed. Merge = OR.
struct SetObjective {
    FormRef quest;
    uint32_t objective = 0;
    uint8_t flags = 0;
    bool operator==(const SetObjective& o) const {
        return quest == o.quest && objective == o.objective && flags == o.flags;
    }
};
struct SetGlobal {
    FormRef global;
    float value = 0;
    bool operator==(const SetGlobal& o) const { return global == o.global && value == o.value; }
};
struct EnableRef {
    FormRef ref;
    bool operator==(const EnableRef& o) const { return ref == o.ref; }
};
struct DisableRef {
    FormRef ref;
    bool operator==(const DisableRef& o) const { return ref == o.ref; }
};
struct AddItem {
    FormRef item;
    int32_t count = 0;
    Recipient to;
    bool operator==(const AddItem& o) const { return item == o.item && count == o.count && to == o.to; }
};
struct RemoveItem {
    FormRef item;
    int32_t count = 0;
    Recipient to;
    bool operator==(const RemoveItem& o) const { return item == o.item && count == o.count && to == o.to; }
};
struct ModActorValue {  // e.g. XP, skills
    uint32_t av = 0;
    int32_t delta = 0;
    Recipient to;
    bool operator==(const ModActorValue& o) const { return av == o.av && delta == o.delta && to == o.to; }
};
struct ModReputation {
    FormRef faction;
    int32_t fame = 0;
    int32_t infamy = 0;
    Recipient to;
    bool operator==(const ModReputation& o) const {
        return faction == o.faction && fame == o.fame && infamy == o.infamy && to == o.to;
    }
};
struct ModKarma {
    int32_t delta = 0;
    Recipient to;
    bool operator==(const ModKarma& o) const { return delta == o.delta && to == o.to; }
};
struct StartQuest {
    FormRef quest;
    bool operator==(const StartQuest& o) const { return quest == o.quest; }
};
struct StopQuest {
    FormRef quest;
    bool operator==(const StopQuest& o) const { return quest == o.quest; }
};
struct MoveRef {
    FormRef ref;
    FormRef cell;
    float x = 0, y = 0, z = 0;
    bool partyWarp = false;  // all players are moved together (story-critical transitions)
    bool operator==(const MoveRef& o) const {
        return ref == o.ref && cell == o.cell && x == o.x && y == o.y && z == o.z && partyWarp == o.partyWarp;
    }
};
struct KillActor {
    FormRef actor;
    bool operator==(const KillActor& o) const { return actor == o.actor; }
};
struct ResurrectActor {
    FormRef actor;
    bool operator==(const ResurrectActor& o) const { return actor == o.actor; }
};

// ---- vocabulary round 2 (L3/L8/L9 requests) ----
// Quest variable or ref script variable (owner = quest or ref). Host order, last write wins.
struct SetScriptVar {
    FormRef owner;
    uint16_t index = 0;
    double value = 0;
    bool operator==(const SetScriptVar& o) const { return owner == o.owner && index == o.index && value == o.value; }
};
// Enable/disable player controls (bitmask of control ids), routed by `to` (default: the initiator/origin).
struct SetPlayerControls {
    uint32_t controls = 0;
    bool enable = true;
    Recipient to;
    bool operator==(const SetPlayerControls& o) const { return controls == o.controls && enable == o.enable && to == o.to; }
};
struct PartyTether {  // keep the party together in this cell/worldspace
    FormRef cell;
    bool on = true;
    bool operator==(const PartyTether& o) const { return cell == o.cell && on == o.on; }
};
struct SlideshowEnd {  // intro slideshow finished (sticky)
    bool operator==(const SlideshowEnd&) const { return true; }
};
struct SaveSuppress {  // block saving during a scripted sequence
    bool on = true;
    bool operator==(const SaveSuppress& o) const { return on == o.on; }
};
// Recruit (active) or dismiss a follower. kind: 0 humanoid, 1 critter (follower caps are per kind, see HostSession::Options).
struct CompanionOwner {
    FormRef npc;
    uint8_t owner = 0;
    bool active = true;
    uint8_t kind = 0;
    bool operator==(const CompanionOwner& o) const {
        return npc == o.npc && owner == o.owner && active == o.active && kind == o.kind;
    }
};
// Event (not state): everyone's sink sees it; Context::catchUp tells replays apart from live.
struct ForcedDialogue {
    FormRef npc, topic;
    uint8_t initiator = 0;
    bool operator==(const ForcedDialogue& o) const { return npc == o.npc && topic == o.topic && initiator == o.initiator; }
};
// The origin player reached a barrier; `required` = slot mask that must all arrive. When complete, the shadow emits
// BarrierRelease (the host script advances the shared stage in reaction, e.g. chargen).
struct BarrierArrive {
    FormRef barrier;
    uint8_t required = 0;
    bool operator==(const BarrierArrive& o) const { return barrier == o.barrier && required == o.required; }
};
struct BarrierRelease {
    FormRef barrier;
    bool operator==(const BarrierRelease& o) const { return barrier == o.barrier; }
};
struct SayOnceFlag {  // INFO "say once" already spoken (union)
    FormRef info;
    bool operator==(const SayOnceFlag& o) const { return info == o.info; }
};
struct CompleteQuest {
    FormRef quest;
    bool operator==(const CompleteQuest& o) const { return quest == o.quest; }
};
// Quest failed (TESQuest::SetFailed): flags failed|completed, sticky. A failed quest also counts as completed.
struct FailQuest {
    FormRef quest;
    bool operator==(const FailQuest& o) const { return quest == o.quest; }
};
struct CompleteAllObjectives {
    FormRef quest;
    bool operator==(const CompleteAllObjectives& o) const { return quest == o.quest; }
};
struct SetLock {
    FormRef ref;
    bool locked = false;
    int32_t level = 0;
    bool hasBefore = false, beforeLocked = false;   // pre-image: the lock read just before this change (restores a lock the loaded save never touched)
    int32_t beforeLevel = 0;
    bool operator==(const SetLock& o) const {
        return ref == o.ref && locked == o.locked && level == o.level && hasBefore == o.hasBefore &&
               (!hasBefore || (beforeLocked == o.beforeLocked && beforeLevel == o.beforeLevel));
    }
};
// Faction membership/rank of an actor; rank < 0 = removed. Host order.
struct SetFactionRank {
    FormRef actor, faction;
    int32_t rank = 0;
    bool operator==(const SetFactionRank& o) const { return actor == o.actor && faction == o.faction && rank == o.rank; }
};
// Per-player perk rank (0 = removed), routed like rewards.
struct SetPerk {
    FormRef perk;
    int32_t rank = 1;
    Recipient to;
    bool operator==(const SetPerk& o) const { return perk == o.perk && rank == o.rank && to == o.to; }
};
struct SwapTexture {  // visual state of a ref
    FormRef ref, texture;
    uint8_t slot = 0;
    bool operator==(const SwapTexture& o) const { return ref == o.ref && texture == o.texture && slot == o.slot; }
};

// Party-wide fast-travel lock, host authoritative. One lock per scope (usually the quest whose scene is running);
// the party may travel only when no scope is locked.
struct SetFastTravelLock {
    FormRef scope;
    bool on = true;
    bool operator==(const SetFastTravelLock& o) const { return scope == o.scope && on == o.on; }
};

// ---- vocabulary round 3 (game-wide census) ----
// Created refs (PlaceAtMe / PlaceLeveledActorAtMe): the HOST allocates a stable FormRef 0xFF-prefixed (HostSession::allocCreatedRef) and
// sends SpawnRef; each machine creates its own local object and maps host-ref -> local-ref. DespawnRef is a tombstone.
constexpr uint32_t kCreatedRefPrefix = 0xFF000000u;
inline const char* createdRefPlugin() { return "fnvcoop.created"; }
inline bool isCreatedRef(const FormRef& r) { return (r.localId >> 24) == 0xFF; }
inline FormRef createdRef(uint32_t id) { return FormRef(createdRefPlugin(), id); }
struct SpawnRef {
    uint32_t id = kCreatedRefPrefix;   // 0xFF-prefixed host-allocated id (keeps Effect small: no FormRef for the constant plugin)
    FormRef base, cell;
    float x = 0, y = 0, z = 0;
    bool leveled = false;   // PlaceLeveledActorAtMe
    FormRef ref() const { return createdRef(id); }
    bool operator==(const SpawnRef& o) const {
        return id == o.id && base == o.base && cell == o.cell && x == o.x && y == o.y && z == o.z && leveled == o.leveled;
    }
};
struct DespawnRef {
    uint32_t id = kCreatedRefPrefix;
    // forget: undo a spawn instead of recording a despawn (a rebase after a host load): the ref disappears and leaves NO tombstone, so the
    // host's rewound id counter can hand the same id out again. On a ref that is only a tombstone it just drops the tombstone.
    bool forget = false;
    FormRef ref() const { return createdRef(id); }
    bool operator==(const DespawnRef& o) const { return id == o.id && forget == o.forget; }
};
// Object state. mask bit0: set open/closed, bit1: set lock (locked + level), bit2: destroyed (sticky), bit3: activate-once done (sticky).
constexpr uint8_t kObjOpen = 1, kObjLock = 2, kObjDestroyed = 4, kObjActivated = 8;
struct ObjectState {
    FormRef ref;
    uint8_t mask = 0;
    bool open = false, locked = false;
    int32_t level = 0;
    bool destroyed = false, activatedOnce = false;
    // Pre-image of the lock (mask bit1): the lock state the capturing machine read just before this change. Lets a rebase restore a lock the
    // loaded save never changed. `open` needs none (closed is the default and an open door is the exception), destroyed / activated-once are
    // sticky in the engine and cannot be undone at all.
    bool hasBefore = false, beforeLocked = false;
    int32_t beforeLevel = 0;
    bool operator==(const ObjectState& o) const {
        return ref == o.ref && mask == o.mask && open == o.open && locked == o.locked && level == o.level && destroyed == o.destroyed &&
               activatedOnce == o.activatedOnce && hasBefore == o.hasBefore &&
               (!hasBefore || (beforeLocked == o.beforeLocked && beforeLevel == o.beforeLevel));
    }
};
// Script variable of a non-quest owner (object / NPC script). Host order, last write wins.
struct RefScriptVar {
    FormRef ref;
    uint16_t index = 0;
    double value = 0;
    bool hasBefore = false;   // pre-image: the value read just before this write (restores a var the loaded save never wrote)
    double before = 0;
    bool operator==(const RefScriptVar& o) const {
        return ref == o.ref && index == o.index && value == o.value && hasBefore == o.hasBefore && (!hasBefore || before == o.before);
    }
};
// Inventory of a non-player ref (container, NPC). Deltas relative to the baseline world; RemoveAllItems clears.
struct RefAddItem {
    FormRef ref, item;
    int32_t count = 0;
    bool operator==(const RefAddItem& o) const { return ref == o.ref && item == o.item && count == o.count; }
};
struct RefRemoveItem {
    FormRef ref, item;
    int32_t count = 0;
    bool operator==(const RefRemoveItem& o) const { return ref == o.ref && item == o.item && count == o.count; }
};
// Pre-image of a clear: `items` = the contents (absolute counts) the capturing machine read just before emptying the container. A rebase
// after a host load puts them back with `undo` (the container becomes un-cleared and `items` are re-added). Without a pre-image
// (hasBefore == false) the clear can never be undone.
struct RemoveAllItems {
    FormRef ref;
    bool undo = false;
    bool hasBefore = false;
    std::vector<std::pair<FormRef, int32_t>> items;
    bool operator==(const RemoveAllItems& o) const { return ref == o.ref && undo == o.undo && hasBefore == o.hasBefore && items == o.items; }
};
// Journal. Topics, notes and map markers are shared; the active (focused) quest is per player.
struct AddTopic {
    FormRef topic;
    bool operator==(const AddTopic& o) const { return topic == o.topic; }
};
struct AddNote {
    FormRef note;
    bool operator==(const AddNote& o) const { return note == o.note; }
};
struct RemoveNote {
    FormRef note;
    bool operator==(const RemoveNote& o) const { return note == o.note; }
};
constexpr uint8_t kMarkerVisible = 1, kMarkerTravel = 2;
struct SetMapMarker {
    FormRef marker;
    uint8_t flags = 0;   // kMarkerVisible | kMarkerTravel (fast-travel allowed)
    bool hasBefore = false;   // pre-image: the flags read just before this change
    uint8_t before = 0;
    bool operator==(const SetMapMarker& o) const {
        return marker == o.marker && flags == o.flags && hasBefore == o.hasBefore && (!hasBefore || before == o.before);
    }
};
struct ForceActiveQuest {
    FormRef quest;
    Recipient to;
    bool operator==(const ForceActiveQuest& o) const { return quest == o.quest && to == o.to; }
};
// Scripted save (autosave / systemsave / forcesave): EVENT. The host saves, clients write their sidecar (ClientSession::saveState).
constexpr uint8_t kSaveAuto = 0, kSaveSystem = 1, kSaveForce = 2;
struct RequestSave {
    uint8_t kind = kSaveAuto;
    bool operator==(const RequestSave& o) const { return kind == o.kind; }
};
// Weather and time are host-authoritative.
struct SetWeather {
    FormRef weather;
    bool force = false;   // ForceWeather (instant) vs SetWeather (transition)
    bool hasBefore = false;   // pre-image: the weather read just before this change
    FormRef before;
    bool beforeForce = false;
    bool operator==(const SetWeather& o) const {
        return weather == o.weather && force == o.force && hasBefore == o.hasBefore && (!hasBefore || (before == o.before && beforeForce == o.beforeForce));
    }
};
struct SetTimeScale {
    float scale = 0;      // SetGlobalTimeMultiplier; 0 = never set
    bool hasBefore = false;   // pre-image: the multiplier read just before this change
    float before = 0;
    bool operator==(const SetTimeScale& o) const { return scale == o.scale && hasBefore == o.hasBefore && (!hasBefore || before == o.before); }
};
struct AdvanceTime {      // sleep / wait: EVENT (everyone's clock jumps)
    uint32_t minutes = 0;
    bool operator==(const AdvanceTime& o) const { return minutes == o.minutes; }
};

// SetAngle: one rotation axis of a reference, in degrees (0 = X, 1 = Y, 2 = Z). Host order; the committed (host) value wins.
// `before` is the pre-image: the angle the capturing machine read just before this change. The shadow keeps the FIRST one it sees per
// (ref, axis) as that axis's original, so a rebase after a host load can restore a ref the loaded save never rotated. Setting an axis back
// to its original removes it from the state. hasBefore = false when the capturer could not read it (such an axis can never be reverted).
constexpr uint8_t kAxisX = 0, kAxisY = 1, kAxisZ = 2;
struct RefAngle {
    FormRef ref;
    uint8_t axis = kAxisZ;
    float degrees = 0;
    bool hasBefore = false;
    float before = 0;
    bool operator==(const RefAngle& o) const {
        return ref == o.ref && axis == o.axis && degrees == o.degrees && hasBefore == o.hasBefore && (!hasBefore || before == o.before);
    }
};

// RefCommand (story-capture): one script command that acts on a reference, or on nothing, and has no Effect type of its own. `op` is a RefOp
// (refop.h) and decides what the slots mean (plugin/hooks/ref_ops.h has the per-op layout). `ref` is the calling reference (empty plugin =
// none), `a` / `b` are form arguments, i / j / k / l integer arguments, f a float. Events are applied once by every machine that should see them
// (`to` narrows player-targeted ones; actor AI orders are applied only by the machine that owns the actor). State ops keep the last command per
// key in the ShadowState and reach late joiners with the baseline. Unknown ops are rejected by the decoder.
struct RefCommand {
    uint16_t op = 0;
    FormRef ref, a, b;
    int32_t i = 0, j = 0, k = 0, l = 0;
    float f = 0;
    std::string s;   // a text argument (PlayIdle's idle name): at most 255 characters on the wire
    Recipient to;
    // Second shape (refOpIsWide, refop.h): a command with more parameters than the slots above. UseWeapon has four forms and five integers, so a
    // wide op also carries the forms `c` / `d` and the integer `m` (forms a b c d, integers i j k l m). They travel only for wide ops: every other
    // op's bytes are unchanged, and the decoder rejects nothing for them.
    FormRef c, d;
    int32_t m = 0;
    bool operator==(const RefCommand& o) const {
        return op == o.op && ref == o.ref && a == o.a && b == o.b && i == o.i && j == o.j && k == o.k && l == o.l && f == o.f && s == o.s && to == o.to &&
               c == o.c && d == o.d && m == o.m;
    }
};

// Convenience: the actor-value id the hooks use for experience (RewardXP folds into ModActorValue).
constexpr uint32_t kAvExperience = 0x10000;
// ModActorValue av of Health (the engine ActorValue code 0x10) and the drain a rerouted player.kill uses (RulesEngine PerPlayerState): far past any pool, so the player dies.
constexpr uint32_t kAvHealth = 16;
constexpr int32_t kKillDrain = 100000;

using Effect = std::variant<SetStage, SetObjective, SetGlobal, EnableRef, DisableRef, AddItem, RemoveItem,
                            ModActorValue, ModReputation, ModKarma, StartQuest, StopQuest, MoveRef, KillActor,
                            ResurrectActor, SetScriptVar, SetPlayerControls, PartyTether, SlideshowEnd, SaveSuppress,
                            CompanionOwner, ForcedDialogue, BarrierArrive, BarrierRelease, SayOnceFlag, CompleteQuest,
                            CompleteAllObjectives, SetLock, SetFactionRank, SetPerk, SwapTexture, SetFastTravelLock, FailQuest, SpawnRef, DespawnRef, ObjectState, RefScriptVar, RefAddItem,
                            RefRemoveItem, RemoveAllItems, AddTopic, AddNote, RemoveNote, SetMapMarker, ForceActiveQuest, RequestSave,
                            SetWeather, SetTimeScale, AdvanceTime, RefAngle, RefCommand>;

const char* effectName(const Effect& e);

// Transaction flags
// Origin machine already applied these effects locally (client intent); the origin skips its sink.
constexpr uint8_t kTxnOriginApplied = 1;

struct EffectTxn {
    uint64_t seq = 0;  // assigned by the host StoryLog; 0 = not yet committed
    PlayerId origin = kHostPlayer;
    Cause cause;
    std::vector<Effect> effects;
    uint8_t flags = 0;
    // Client-intent txns: the origin's epoch (see ClientSession). The origin skips its sink only when this
    // matches its current epoch, i.e. its live world really did apply the action locally.
    uint32_t originEpoch = 0;
    uint32_t originIntent = 0;  // id of the client intent that produced this txn
    bool operator==(const EffectTxn& o) const {
        return seq == o.seq && origin == o.origin && cause == o.cause && effects == o.effects && flags == o.flags &&
               originEpoch == o.originEpoch && originIntent == o.originIntent;
    }
};

}  // namespace fnvcoop
