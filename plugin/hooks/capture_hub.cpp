#include "capture_hub.h"

namespace nvhooks {

#ifdef NVHOOKS_MUTANT
constexpr bool kMutEmitPerScope = true;   // every nested scope emits its own txn (breaks "one script run = one txn")
constexpr bool kMutRecordAmbient = true;  // record mutations outside any scope
#else
constexpr bool kMutEmitPerScope = false;
constexpr bool kMutRecordAmbient = false;
#endif

// A failed quest also counts as completed (L4 contract): QuestStageItem::RunResult calls SetCompleted(1) AND SetFailed(1) for fail
// stages, so the txn would carry CompleteQuest + FailQuest. Keep only the FailQuest.
static void foldFailedIntoFail(std::vector<fnvcoop::Effect>& fx) {
#ifndef NVHOOKS_MUTANT
    std::vector<fnvcoop::FormRef> failed;
    for (const auto& e : fx)
        if (auto* f = std::get_if<fnvcoop::FailQuest>(&e)) failed.push_back(f->quest);
    if (failed.empty()) return;
    std::vector<fnvcoop::Effect> out;
    for (auto& e : fx) {
        auto* c = std::get_if<fnvcoop::CompleteQuest>(&e);
        bool redundant = false;
        if (c)
            for (const auto& q : failed)
                if (q == c->quest) redundant = true;
        if (!redundant) out.push_back(std::move(e));
    }
    fx.swap(out);
#else
    (void)fx;  // mutant: failed quests are reported as completed AND failed
#endif
}

void CaptureHub::emit(const fnvcoop::Cause& cause) {
    std::vector<fnvcoop::Effect> fx = cap_.take();
    foldFailedIntoFail(fx);
    if (fx.empty()) return;
    fnvcoop::EffectTxn t;
    t.cause = cause;
    t.effects = std::move(fx);
    ++stats_.txns;
    for (size_t i = 0; i < sinks_.size(); ++i) {
        if (!sinks_[i]) continue;
        if (i + 1 == sinks_.size()) sinks_[i](std::move(t));
        else sinks_[i](fnvcoop::EffectTxn(t));
    }
}

bool CaptureHub::beginWeak(uint32_t scriptRefID) {
    if (!on() || !frames_.empty()) return false;
#ifdef NVHOOKS_MUTANT
    return false;  // mutant: ambient script mutations are dropped again (droppedNoScope)
#else
    Frame f;
    f.weak = true;
    f.lazyId = scriptRefID;
    frames_.push_back(std::move(f));
    last_.reset();
    return true;
#endif
}

void CaptureHub::endWeak() {
    if (frames_.empty()) return;
    while (frames_.size() > 1) end();  // a strong scope was left open inside the script: close it first
    if (frames_.back().weak) end();
}

void CaptureHub::begin(const fnvcoop::Cause& cause) {
    if (!on()) return;
    bool hasStrong = false;
    for (const Frame& f : frames_) hasStrong |= !f.weak;
    if (!hasStrong && frames_.size() == 1 && frames_[0].weak && !cap_.empty() && weakResolver_) {
        ++stats_.weakFlushes;  // effects the script made BEFORE the explicit scope: their own txn, cause = the script
        emit(weakResolver_(frames_[0].lazyId));
    }
    if (!hasStrong) {
        last_.reset();
        if (onBegin_) onBegin_();
    }
    Frame f;
    f.cause = cause;
    frames_.push_back(std::move(f));
}

void CaptureHub::end() {
    if (!enabled_ || frames_.empty()) return;  // not on(): a scope opened before a mute still closes
    if (frames_.back().weak) {  // weak frames only ever sit at the bottom
        if (weakResolver_ && !cap_.empty()) {
            ++stats_.weakTxns;
            emit(weakResolver_(frames_.back().lazyId));
        } else {
            cap_.take();
        }
        frames_.pop_back();
        return;
    }
    size_t strong = 0;
    const Frame* first = nullptr;
    for (const Frame& f : frames_)
        if (!f.weak) {
            if (!first) first = &f;
            ++strong;
        }
    bool outermost = strong == 1;
    if (outermost && onEnd_) onEnd_();
    if (outermost || kMutEmitPerScope) {
        size_t before = stats_.txns;
        emit(first->cause);
        if (stats_.txns == before) ++stats_.emptyScopes;
    }
    frames_.pop_back();
}

bool CaptureHub::currentSource(fnvcoop::FormRef* out) const {
    if (!on() || frames_.empty()) return false;
    const Frame* pick = &frames_.front();
    for (const Frame& f : frames_)
        if (!f.weak) { pick = &f; break; }  // outermost explicit scope wins over the implicit script scope below it
    fnvcoop::Cause c = pick->weak ? (weakResolver_ ? weakResolver_(pick->lazyId) : fnvcoop::Cause{}) : pick->cause;
    if (c.source.plugin.empty() && c.source.localId == 0) return false;
    if (out) *out = c.source;
    return true;
}

bool CaptureHub::currentCause(fnvcoop::Cause* out) const {
    if (!on() || frames_.empty()) return false;
    const Frame* pick = &frames_.front();
    for (const Frame& f : frames_)
        if (!f.weak) { pick = &f; break; }
    const fnvcoop::Cause c = pick->weak ? (weakResolver_ ? weakResolver_(pick->lazyId) : fnvcoop::Cause{}) : pick->cause;
    if (out) *out = c;
    return true;
}

size_t CaptureHub::reapStale() {
#ifdef NVHOOKS_MUTANT
    return 0;  // never recovers: every later capture disappears into the stuck scope
#else
    size_t n = frames_.size();
    if (n) ++stats_.staleScopes;
    while (!frames_.empty()) end();
    return n;
#endif
}

static bool idempotentState(const fnvcoop::Effect& e) {
    return std::holds_alternative<fnvcoop::EnableRef>(e) || std::holds_alternative<fnvcoop::DisableRef>(e) ||
           std::holds_alternative<fnvcoop::StartQuest>(e) || std::holds_alternative<fnvcoop::StopQuest>(e) ||
           std::holds_alternative<fnvcoop::CompleteQuest>(e);
}

bool CaptureHub::record(const fnvcoop::Effect& e) {
    if (muted_ > 0 && enabled_) { ++stats_.droppedLocal; return false; }
    if (!enabled_) return false;
    if (frames_.empty() && !kMutRecordAmbient) {
        ++stats_.droppedNoScope;
        return false;
    }
    if (cap_.suppressed()) {
        ++stats_.droppedEcho;
        cap_.record(e);  // bumps EffectCapture's own suppressed counter
        return false;
    }
#ifndef NVHOOKS_MUTANT
    // One engine command can reach two primitives for the same state change (Enable: form-flag clear + extra-flags clear):
    // consecutive identical state effects are collapsed. Quantities (AddItem ...) are never collapsed.
    if (idempotentState(e) && last_ && *last_ == e) {
        ++stats_.collapsed;
        return true;
    }
#endif
    cap_.record(e);
    last_ = e;
    ++stats_.recorded;
    return true;
}

}  // namespace nvhooks

namespace nvhooks {
bool CaptureHub::recordScriptVar(const ScriptVarChange& c) {
    if (c.varId > 0xFFFFu) return false;  // does not fit SetScriptVar::index (reported to L4 if it ever happens)
    return record(fnvcoop::SetScriptVar{c.owner, static_cast<uint16_t>(c.varId), c.value});
}
}  // namespace nvhooks
