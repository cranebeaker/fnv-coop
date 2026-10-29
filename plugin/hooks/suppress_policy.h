// Guest suppression policy (game-free, unit-tested). The engine detours ask this object what to do.
//
// Rules (coordinator decision, internal lane notes round 4):
//  1. Quest-owned scripts (Script::info.type == 1: every block type, GameMode/MenuMode/OnTrigger...) never run on a
//     guest. The host replicates their quest vars / globals.
//  2. INFO result scripts and stage result scripts never run on a guest.
//  3. Any OTHER script (object / NPC / magic effect) still runs on a guest, but a shared-state mutator it calls is
//     dropped (blockMutation()).
//     Exception to "still runs": an inline `set Quest.var to ...` is not a mutator call (the interpreter stores it, nothing to detour), so a
//     guest brackets the object scripts that write shared quest variables and puts the variables back (guest_objvars.h; the host's value
//     arrives through the state replicator).
//  Everything is lifted while a host EffectTxn is being applied (applying > 0).
#pragma once
#include <cstdint>
#include <functional>
#include <unordered_set>
#include <vector>

namespace nvhooks {

struct Suppressed {
    uint64_t mutations = 0;      // blocked primitive / command calls made by a running local script
    uint64_t resultScripts = 0;  // blocked TESTopicInfo::RunResultScript calls
    uint64_t questScripts = 0;   // skipped quest-owned Script::Execute calls
};

constexpr uint16_t kScriptTypeObject = 0;
constexpr uint16_t kScriptTypeQuest = 1;
constexpr uint16_t kScriptTypeMagic = 0x100;

class SuppressPolicy {
public:
    void setGuest(bool g) { guest_ = g; }
    void setQuestGate(bool on) { questGate_ = on; }
    bool guest() const { return guest_; }
    bool questGate() const { return questGate_; }

    void beginApply() { ++applying_; }
    void endApply() { if (applying_ > 0) --applying_; }
    bool applying() const { return applying_ > 0; }

    // Script::Execute bracket. skipScript() is evaluated BEFORE enterScript().
    bool skipScript(uint16_t scriptType) { return skipScript(scriptType, 0); }
    // `scriptRefID` lets the GuestLocalRun allow-list (per-machine quest scripts: achievements, rumble, DLC setup, fades) keep specific
    // quest scripts running on guests.
    bool skipScript(uint16_t scriptType, uint32_t scriptRefID);
    void setGuestLocalRun(std::unordered_set<uint32_t> ids) { localRun_ = std::move(ids); }
    // Quest script variables of a GuestLocalRun quest are per-machine state: never captured, polled or sent (and never shared divergence).
    bool sharesQuestVars(uint32_t questScriptRefID) const;
    size_t guestLocalRunSize() const { return localRun_.size(); }
    bool isGuestLocalRun(uint32_t scriptRefID) const { return scriptRefID && localRun_.count(scriptRefID); }
    void enterScript() { ++depth_; }
    void leaveScript() { if (depth_ > 0) --depth_; }
    // A GuestLocalRun script is per-machine by definition: while one runs, its mutations (SetStage, StartQuest, AddItem, ...) are NOT
    // blocked on a guest (the host drops the same script's txns, so each machine applies them itself).
    void enterLocalScript() { ++depth_; ++localDepth_; }
    void leaveLocalScript() { if (depth_ > 0) --depth_; if (localDepth_ > 0) --localDepth_; }
    int localScriptDepth() const { return localDepth_; }
    // A GuestLocalRun script whose QUEST STATE stays host-authoritative (the DLC setup scripts' start of their intro quest and its stage 10, hostQuestStateScripts()): it runs on a
    // guest for its per-machine effects, but blockQuestState() still drops the quest start / stage / objective it calls, and on the host those calls are captured and replicated
    // (the hub's mute is lifted for them, engine_hooks.cpp QuestStateLift). Subset of the allow-list: only ids that are also allow-listed count.
    void setHostQuestState(std::unordered_set<uint32_t> ids) { hostState_ = std::move(ids); }
    bool isHostQuestState(uint32_t scriptRefID) const { return scriptRefID && isGuestLocalRun(scriptRefID) && hostState_.count(scriptRefID); }
    void enterLocalScript(uint32_t scriptRefID) { enterLocalScript(); hostStack_.push_back(isHostQuestState(scriptRefID) ? scriptRefID : 0); }
    void leaveLocalScript(uint32_t) { leaveLocalScript(); if (!hostStack_.empty()) hostStack_.pop_back(); }
    // The innermost running allow-listed script is a host-quest-state one (its quest calls are the host's story state), and which script it is.
    bool inHostQuestStateScript(uint32_t* scriptRefID = nullptr) const {
        if (hostStack_.empty() || !hostStack_.back()) return false;
        if (scriptRefID) *scriptRefID = hostStack_.back();
        return true;
    }
    // A console or bridge line that acts on the local player ("player.MoveTo ...", "player.SetAV Health 5000") is per-machine by
    // nature (each machine owns its own player), so on a guest it runs like a GuestLocalRun script instead of being dropped. Only
    // a single line that starts with "player." qualifies; anything else typed on a guest console (SetStage, Enable, ...) stays blocked.
    static bool isLocalPlayerConsoleLine(const char* text);
    // Explicit local-player scope (nvhooks::LocalPlayerScope): the console lines run while it is open act on THIS machine's own player and world view
    // only. A line run inside it is a per-machine run like a GuestLocalRun script: not blocked on a guest, muted from the capture hub on a host, so it is
    // never captured or sent. It covers console / bridge lines (temporary scripts) only: an ESM script that happens to run inside the scope (an event
    // a line raised) keeps the rules of any other script. Nests; the counter is plain because the scope is main-thread only (engine_hooks.cpp).
    void enterLocalPlayerScope() { ++scopeDepth_; }
    void leaveLocalPlayerScope() { if (scopeDepth_ > 0) --scopeDepth_; }
    bool inLocalPlayerScope() const {
#ifdef NVHOOKS_MUTANT
        return false;   // mutant: the scope exempts nothing (the guest's character creation is dropped again)
#else
        return scopeDepth_ > 0;
#endif
    }
    // A console / bridge line runs as a temporary script: refID 0, or 0xFF...... (ESM scripts are never console lines).
    static bool isConsoleLineScriptId(uint32_t scriptRefID) { return scriptRefID == 0 || (scriptRefID >> 24) == 0xFF; }
    // The per-machine decision of Script::Execute: a GuestLocalRun script, a console line inside a LocalPlayerScope, or (guest only) a typed single
    // "player." line. `consoleText` yields the line's source text and is only called for a console line outside a scope on a guest.
    template <class TextFn>
    bool runsLocal(uint32_t scriptRefID, TextFn&& consoleText) const {
        if (isGuestLocalRun(scriptRefID)) return true;
        if (!isConsoleLineScriptId(scriptRefID)) return false;
        if (inLocalPlayerScope()) return true;
        return guest_ && isLocalPlayerConsoleLine(consoleText());
    }
    int scriptDepth() const { return depth_; }

    bool blockMutation();      // shared-state mutator called from a running script
    // Quest start / stop / complete, stage and objective calls: like blockMutation(), except that the exemption of a GuestLocalRun script does NOT cover them when the script is a
    // host-quest-state one (a guest's own DLC setup script must not start a quest or set its stage: the host does, and the txn reaches the guest).
    bool blockQuestState();
    bool blockResultScript();  // INFO / stage result script entry

    const Suppressed& stats() const { return stats_; }
    void resetStats() { stats_ = {}; }

private:
    bool guest_ = false;
    bool questGate_ = true;
    int applying_ = 0;
    int depth_ = 0;
    int localDepth_ = 0;
    int scopeDepth_ = 0;   // open LocalPlayerScopes
    Suppressed stats_;
    std::unordered_set<uint32_t> localRun_;
    std::unordered_set<uint32_t> hostState_;
    std::vector<uint32_t> hostStack_;   // one entry per running allow-listed script: its id when host-quest-state, else 0
};

// Host-side deferral of engine SetStage calls the rules engine wants to hold back (e.g. the VCG01 chargen barrier).
// allow() is asked at the top of the SetStage detour. Calls made by our own applier (applying) or by the owner of the
// gate re-issuing a deferred stage (StageGateBypass alive) always pass.
class StageGate {
public:
    using Fn = std::function<bool(uint32_t quest, uint32_t stage)>;  // quest = engine refID
    void set(Fn f) { fn_ = std::move(f); }
    void beginBypass() { ++bypass_; }
    void endBypass() { if (bypass_ > 0) --bypass_; }
    bool allow(uint32_t quest, uint32_t stage, bool applying);
    uint64_t deferred() const { return deferred_; }

private:
    Fn fn_;
    int bypass_ = 0;
    uint64_t deferred_ = 0;
};

// "Acting player": while the host runs a block FOR a guest, which reference stands for `player` in the host-side helpers that consult it
// (activator selection for Activate, player-ref condition evaluators, inventory/perk facts). Stack-like; default = the host's player 0x14.
// NOT a global ThePlayer swap: script `player` operands are cached form pointers and the engine assumes a PlayerCharacter there.
class ActingContext {
public:
    static constexpr uint32_t kHostPlayer = 0x14;
    uint32_t current() const { return depth_ ? stack_[depth_ - 1] : kHostPlayer; }
#ifdef NVHOOKS_MUTANT
    void push(uint32_t refID, int forcedSlot = -1) { if (depth_ < 8) { forced_[depth_] = forcedSlot; stack_[depth_++] = refID; } }  // mutant: a null ghost id becomes ref 0
#else
    // forcedSlot >= 0: the party slot the window is for, whatever `refID` is (a guest with no puppet is acted for by the host's own player 0x14, which no ghost table can name)
    void push(uint32_t refID, int forcedSlot = -1) { if (depth_ < 8) { forced_[depth_] = forcedSlot; stack_[depth_++] = refID ? refID : kHostPlayer; } }
#endif
    void pop() { if (depth_) --depth_; }
    int depth() const { return depth_; }
    // Party slot of the acting player: 0 for the host's own player, else the slot registered for that guest's ghost (unknown ghost -> 0).
    void registerSlot(uint32_t ghostRefID, uint8_t slot) {
        for (int i = 0; i < nSlots_; ++i) if (ghost_[i] == ghostRefID) { slot_[i] = slot; return; }
        if (nSlots_ < 8) { ghost_[nSlots_] = ghostRefID; slot_[nSlots_++] = slot; }
    }
    void clearSlots() { nSlots_ = 0; }
    uint8_t slot() const {
        if (depth_ && forced_[depth_ - 1] >= 0) return static_cast<uint8_t>(forced_[depth_ - 1]);
        uint32_t c = current();
        for (int i = 0; i < nSlots_; ++i) if (ghost_[i] == c) return slot_[i];
        return 0;
    }
    // Which co-op player a script operand stands for (G35 / G42: `player.KillActor`, `player.RemoveAllItems`, a guest's ghost as the operand).
    // The engine's `player` (0x14) is the acting player's stand-in: the host's own player while nobody acts (slot 0), else the acting guest. A
    // registered ghost is that guest. `onHost` = the engine call really concerns the host's own player, so the original may run; for a guest it
    // must not (it would hit the host's player instead).
    struct PlayerTarget {
        bool player = false;
        uint8_t slot = 0;
        bool onHost = false;
    };
    // Host side of the guests' events (P4 handover 1): the engine raises `MarkBaseExtraListScriptEvent(object, target, mask)` with the PUPPET / ghost of a
    // guest as the object when a host-owned NPC does something to it (starts combat with it). `player` in the block that runs is the host's player, so
    // the event must run as that guest. Only the combat mask is wrapped: OnTrigger / OnTriggerEnter / OnTriggerLeave, pickups and container events for a
    // puppet already reach the host as intents (TriggerStay / TriggerEnter / pickup), wrapping them too would run every block twice.
    static constexpr uint32_t kEventOnStartCombat = 0x8000;
    bool hostPuppetEvent(uint32_t objectRefID, uint32_t mask) const {
        if (objectRefID == kHostPlayer) return false;
        bool known = false;
        for (int i = 0; i < nSlots_; ++i) if (ghost_[i] == objectRefID) known = true;
#ifdef NVHOOKS_MUTANT
        return known;   // mutant: every event of a puppet is wrapped (trigger blocks would run twice)
#else
        return known && mask == kEventOnStartCombat;
#endif
    }
    PlayerTarget playerTargetOf(uint32_t refID) const {
        if (refID == kHostPlayer) {
            const uint8_t s = slot();
#ifdef NVHOOKS_MUTANT
            return {true, s, true};   // mutant: a kill / strip meant for a guest runs on the host's own player
#else
            return {true, s, s == 0};
#endif
        }
        for (int i = 0; i < nSlots_; ++i)
            if (ghost_[i] == refID) return {true, slot_[i], slot_[i] == 0};
        return {};
    }

private:
    uint32_t ghost_[8] = {};
    uint8_t slot_[8] = {};
    int nSlots_ = 0;
    uint32_t stack_[8] = {};
    int forced_[8] = {-1, -1, -1, -1, -1, -1, -1, -1};
    int depth_ = 0;
};

}  // namespace nvhooks
