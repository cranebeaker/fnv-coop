// Apply a remote Effect to the local world WITHOUT running result scripts.
//
//   Effect            | Engine call(s) used (addresses: docs/engine-addresses.md)                       | Conf
//   ------------------+----------------------------------------------------------------------------------+-----
//   SetStage          | ScriptGate + TESQuest::SetStage 0x60D510. Runs the engine-native part of the     | V
//                     | stage (auto SetRunning, currentStage, isDone, journal text, quest completed/failed
//                     | flags, HUD notice, MarkModified) but Script::Execute 0x5AC1E0 is gated off, so the
//                     | stage's result script never runs; its effects arrive as separate captured effects.
//   SetObjective      | TESQuest::GetObjective 0x60C8E0 -> BGSQuestObjective::SetState 0x5EC5D0(flags)    | V
//                     | (flags: bit0 displayed, bit1 completed -- same encoding as Effect::SetObjective)
//   SetGlobal         | TESGlobal float at +0x24 (form type 6; getter 0x526AC0 is a plain field read)      | M
//   EnableRef/Disable | TESObjectREFR::SetDisabled 0x5650D0(bool)                                         | M
//   AddItem/RemoveItem| TESObjectREFR::AddItem / RemoveItem virtuals (xNVSE GameObjects.h) on the player  | S
//   ModActorValue     | Actor vtable +0x3A8 (ModActorValue: avCode, float delta, 0). XP pseudo-AV 0x10000   | M
//                     | uses Player vtable +0x488 (RewardXP(int)), exactly as Cmd_RewardXP does           |
//   ModReputation     | read TESReputation +0x44/+0x48, TESReputation::Set 0x615580(type, newValue)       | V
//                     | (type 1 = fame/+0x44, else infamy/+0x48; Set also MarkModified's the form)
//   ModKarma          | Player::RewardKarma 0x94FD30(int)                                                  | M
//   StartQuest/Stop   | TESQuest::SetRunning 0x60C9C0(true/false)                                          | V
//   RefAngle          | console `SetAngle <X|Y|Z> <degrees>` run ON <ref> via setExecLine | M
//   MoveRef           | cell == marker REFR: console `MoveTo <marker> x y z` run ON <ref> via setExecLine (x/y/z = offsets) | M
//   KillActor         | console `KillActor` run ON <actor>                                      | M
//   ResurrectActor    | console `ResurrectActor 0` run ON <actor>                               | M
//   RefCommand        | the engine's own console command of the op, run ON the calling reference (ref_ops.h); AI orders only on the machine that
//                     | runs the actor's AI (setRemoteDrivenQuery); Activate only on activators / doors                                     | M
//   ForceActiveQuest  | console `ForceActiveQuest <quest>`                                                                                  | M
//
// Every apply runs under an EchoGuard (so the hooks do not re-capture it) and a ScriptGate.
#pragma once
#include <cstdint>
#include <functional>
#include <memory>
#include <string>

#include "apply_lines.h"
#include "capture_hub.h"
#include "engine_hooks.h"
#include "fnvcoop/effect.h"
#include "identity.h"
#include "move_queue.h"

namespace nvhooks {

enum class ApplyStatus { Ok, Unsupported, FormMissing, NotForMe, EngineRejected };
const char* applyStatusName(ApplyStatus s);

class RemoteApplier {
public:
    RemoteApplier(CaptureHub& hub, FormResolver& res);
    ~RemoteApplier();
    RemoteApplier(const RemoteApplier&) = delete;
    RemoteApplier& operator=(const RemoteApplier&) = delete;
    // Applies one effect that ARRIVED from the host (caller decides recipient filtering via applyTxn or by itself). On a guest (identity.h configured) a
    // reference to the sender's player is first mapped to its puppet here or the effect is dropped (reported Ok, counted in identityStats()); on the host
    // the layer is off and this is applyNow.
    ApplyStatus apply(const fnvcoop::Effect& e);
    // The same for an effect produced by another player's machine (EffectTxn::origin): "the player" in it is THAT player.
    ApplyStatus applyFrom(fnvcoop::PlayerId sender, const fnvcoop::Effect& e);
    // An effect this machine made up about its OWN player (a respawn move, a rule applied for the host's own player): no translation, 0x14 is the local player.
    ApplyStatus applyLocal(const fnvcoop::Effect& e) { return applyNow(e); }
    // Applies all effects in order. localSlot filters player-targeted effects by Recipient. Returns count applied.
    size_t applyTxn(const fnvcoop::EffectTxn& t, fnvcoop::PlayerId localSlot);
    std::string lastError() const { return err_; }
    // This machine's party slot (applyTxn sets it from its argument); decides who applies CompanionOwner. A guest with the identity layer configured uses its own slot from there.
    void setLocalSlot(fnvcoop::PlayerId s) { localSlot_ = s; }
    // Runs a console command with a calling reference (kill / resurrect / MoveTo). Injected by the plugin (NVSE console interface);
    // returns false on failure. Without it those effects stay Unsupported.
    void setExecLine(std::function<bool(const std::string&, uint32_t)> f) { exec_ = std::move(f); }
    // Baseline/join apply (L4 ctx.baseline): FORCE-SET semantics, no result scripts, no HUD notices where avoidable.
    //   SetStage     : stage may go DOWN (currentStage byte written directly; the engine SetStage only ever raises it)
    //   SetObjective : status byte forced to exactly `flags` (clear+set)
    //   SetGlobal / SetScriptVar : overwrite (same as apply)
    //   everything else: same as apply() (deltas stay deltas; ResurrectActor/KillActor/MoveRef remain Unsupported)
    ApplyStatus applyForce(const fnvcoop::Effect& e);
    // TEST ONLY (mutation twin): apply without closing the ScriptGate, so result scripts run again on the replica.
    void setUseGate(bool on) { useGate_ = on; }
    // Actor ownership: true = this machine does NOT run the AI of that reference (its pose comes from snapshots). RefCommand AI orders
    // (StartCombat, EvaluatePackage, ...) are then left to the machine that does. Unset = every order is performed here.
    void setRemoteDrivenQuery(std::function<bool(uint32_t refID)> f) { remoteDriven_ = std::move(f); }
    // The same for every applier of the process that has no query of its own (coop_module.cpp creates its applier privately).
    static void setDefaultRemoteDrivenQuery(std::function<bool(uint32_t refID)> f);
    // Diagnostics: one line per SetStage this machine applies (the quest, the stage, what the engine said, the stage's done flag before / after). Unset = silent.
    static void setTrace(std::function<void(const std::string&)> f);
    // Moves that wait for their destination cell to load (a MoveRef to a cell with no loaded reference to anchor on). tickPending() retries them;
    // tickAll() does that for every applier of the process, at most once a second (hooks_module.cpp calls it every frame).
    size_t pendingMoves() const { return moves_.size(); }
    size_t tickPending();
    static size_t tickAll();

private:
    ApplyStatus applyNow(const fnvcoop::Effect& e);
    bool forMe(const fnvcoop::Recipient& r, fnvcoop::PlayerId origin, fnvcoop::PlayerId me) const;
    ApplyStatus applyMove(const fnvcoop::MoveRef& x, const ExecFn& exec, bool park, std::unique_ptr<ScriptGate>* gate);
    ApplyStatus applyRefCommand(const fnvcoop::RefCommand& x, const ExecFn& exec, std::unique_ptr<ScriptGate>* gate);
    CaptureHub& hub_;
    IdentityResolver idres_;   // res_ with the identity placeholders (identity.h) resolved
    FormResolver& res_;
    std::string err_;
    bool useGate_ = true;
    fnvcoop::PlayerId localSlot_ = 0;
    FastTravelLocks travelLocks_;
    std::function<bool(const std::string&, uint32_t)> exec_;  // (console command, calling reference refID)
    std::function<bool(uint32_t)> remoteDriven_;
    MoveQueue moves_;
};

// Diagnostic line through the trace set by RemoteApplier::setTrace (a no-op when unset). Also used by the capture side (engine_hooks.cpp).
void traceLine(const std::string& line);
bool traceEnabled();
// The player's count of one item as the engine stores it (ExtraContainerChanges); -1 when unreadable. Diagnostics only.
int playerItemCountForTrace(uint32_t itemRefId);
// Guest: while `veto` answers true, a SetPlayerControls effect that disables controls is not applied (the host's scene locks its own player; a
// guest that has made its character walks around meanwhile). Empty = always applied.
void setControlsLockVeto(std::function<bool()> veto);
// Every player control on again (the engine's own SetControlsDisabled, all bits). False when there is no player.
bool enableAllPlayerControls();

}  // namespace nvhooks
