// Effect capture: a scoped "capture context" that is active while a quest-stage / topic-result / other host-run
// script executes. Engine mutation hooks call CaptureHub::record(); outside a scope (or while an EchoGuard from a
// remote apply is held) nothing is recorded.
//
// Game-free (links only the synccore headers); the engine glue lives in engine_hooks.cpp.
//
// Nesting: scopes nest (a topic result script calls SetStage, whose stage items run more result scripts). Only the
// OUTERMOST scope emits an EffectTxn, with the outermost Cause, so "one result-script run = one txn" holds and the
// effect order is the order in which the engine performed the mutations. SetStage is recorded on ENTRY (before the
// nested items run) so replicas replay: stage first, then the stage's own consequences.
#pragma once
#include <cstdint>
#include <functional>
#include <optional>
#include <vector>

#include "fnvcoop/capture.h"
#include "fnvcoop/effect.h"

namespace nvhooks {

// Script-variable write helper (quest or ref script var): becomes Effect SetScriptVar{owner, index, value}.
struct ScriptVarChange {
    fnvcoop::FormRef owner;
    uint32_t varId = 0;  // ScriptLocal::id; must fit uint16 (SetScriptVar::index)
    double value = 0;
};

class CaptureHub {
public:
    using Sink = std::function<void(fnvcoop::EffectTxn&&)>;

    struct Stats {
        uint64_t recorded = 0;
        uint64_t droppedNoScope = 0;  // mutation outside any capture scope (ambient gameplay)
        uint64_t droppedEcho = 0;     // suppressed by an EchoGuard (remote apply)
        uint64_t txns = 0;
        uint64_t emptyScopes = 0;     // outermost scope closed with nothing recorded
        uint64_t collapsed = 0;       // consecutive identical Enable/Disable/Start/Stop/CompleteQuest effects merged
        uint64_t weakTxns = 0;        // txns emitted by implicit script scopes
        uint64_t weakFlushes = 0;     // weak effects flushed because an explicit scope opened inside the script
        uint64_t droppedLocal = 0;    // effects dropped because a GuestLocalRun script (per-machine) produced them
        uint64_t staleScopes = 0;     // scopes force-closed by reapStale() (their closing end() never ran)
    };

    // Sinks are called, in registration order, for every outermost txn. setSink() replaces slot 0 only.
    void setSink(Sink s) { if (sinks_.empty()) sinks_.push_back(std::move(s)); else sinks_[0] = std::move(s); }
    void addSink(Sink s) { sinks_.push_back(std::move(s)); }
    // Outermost-scope observers. onOutermostBegin runs before the first frame is pushed; onOutermostEnd runs while
    // the frame is still open (record() works) and before the txn is emitted. Used for state that has no mutation
    // hook (TESGlobal values: the script interpreter writes them inline), captured by diffing around the scope.
    void setScopeObservers(std::function<void()> onBegin, std::function<void()> onEnd) {
        onBegin_ = std::move(onBegin);
        onEnd_ = std::move(onEnd);
    }
    // Same gating as record(): in scope, enabled, not echo-suppressed.
    bool recordScriptVar(const ScriptVarChange& c);  // == record(SetScriptVar{...})
    // Only the host captures. Clients keep the hub disabled (hooks then cost one branch).
    void setEnabled(bool on) { enabled_ = on; }
    bool enabled() const { return on(); }
    // Local-only mute: while > 0 nothing is captured (no scopes, no records, no weak frames). Held around GuestLocalRun scripts, whose
    // effects are per-machine by definition and must never reach the wire.
    void pushMute() { ++muted_; }
    void popMute() { if (muted_ > 0) --muted_; }
    bool muted() const { return muted_ > 0; }
    // Lift the mute for a moment (host): a GuestLocalRun script whose quest state is the host's still runs muted, but its StartQuest / SetStage / objective calls are shared story state
    // and must be captured. liftMute() returns what to hand back to restoreMute().
    int liftMute() {
#ifdef NVHOOKS_MUTANT
        return muted_;   // mutant: the mute stays (nothing is lifted): the host's quest calls of the DLC setup scripts never reach the guest
#else
        const int m = muted_;
        muted_ = 0;
        return m;
#endif
    }
    void restoreMute(int m) { muted_ = m; }

    // Implicit "weak" scope around a script execution that no explicit scope covers (quest GameMode blocks, object / effect scripts
    // running ambiently): opened only when NO frame is open, cause = Script / the script form, resolved lazily at emit time. Cheap: no
    // observers, no Cause construction, no txn when nothing was recorded. An explicit scope opening inside it (SetStage, INFO result)
    // first FLUSHES the weak effects so far as their own txn, so quest-stage txns keep the quest as their cause.
    using CauseResolver = std::function<fnvcoop::Cause(uint32_t scriptRefID)>;
    void setWeakCauseResolver(CauseResolver r) { weakResolver_ = std::move(r); }
    bool beginWeak(uint32_t scriptRefID);  // true if a weak frame was opened (then call endWeak)
    void endWeak();
    void begin(const fnvcoop::Cause& cause);
    void end();  // must pair with begin(); unbalanced end() is ignored
    // Call once per main-loop frame, OUTSIDE any script: no capture scope can legitimately span a frame boundary. A scope that is
    // still open lost its closing end() (an engine exception/longjmp skipped our destructor); it is closed here (its txn is emitted)
    // so later captures are not swallowed by a stuck scope. Returns the number of frames reaped.
    size_t reapStale();
    // The source form (quest / script) of the outermost open scope: the natural "scope" for effects that need one (fast-travel lock).
    // False when no scope is open or the cause has no source.
    bool currentSource(fnvcoop::FormRef* out) const;
    // The cause of the outermost open scope (the implicit script scope resolved the way it will be at emit time). False when no scope is open.
    bool currentCause(fnvcoop::Cause* out) const;
    bool active() const { return on() && !frames_.empty(); }
    size_t depth() const { return frames_.size(); }

    // Returns true if the effect was captured.
    bool record(const fnvcoop::Effect& e);

    // Hold an EchoGuard on this while applying a remote txn so the applied mutations are not re-captured.
    fnvcoop::EffectCapture& echo() { return cap_; }

    const Stats& stats() const { return stats_; }
    void resetStats() { stats_ = {}; }

private:
    struct Frame {
        fnvcoop::Cause cause;
        bool weak = false;      // implicit script scope (see beginWeak)
        uint32_t lazyId = 0;    // weak frames: script refID, turned into a Cause only if something was recorded
    };
    void emit(const fnvcoop::Cause& cause);
    bool on() const { return enabled_ && muted_ == 0; }
    bool enabled_ = false;
    int muted_ = 0;
    std::vector<Sink> sinks_;
    std::optional<fnvcoop::Effect> last_;
    std::function<void()> onBegin_, onEnd_;
    std::vector<Frame> frames_;
    CauseResolver weakResolver_;
    fnvcoop::EffectCapture cap_;
    Stats stats_;
};

class ScopedCapture {
public:
    ScopedCapture(CaptureHub& h, const fnvcoop::Cause& c) : h_(h) { h_.begin(c); }
    ~ScopedCapture() { h_.end(); }
    ScopedCapture(const ScopedCapture&) = delete;
    ScopedCapture& operator=(const ScopedCapture&) = delete;

private:
    CaptureHub& h_;
};

}  // namespace nvhooks
