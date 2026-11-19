// Effect capture buffer + echo guard.
// The engine hooks call EffectCapture::record() for every primitive mutation they observe. While a remote
// txn is being applied the session holds an EchoGuard, so those mutations are NOT recorded and can never be
// re-sent (applying a remote txn must not re-emit it).
#pragma once
#include <cstddef>
#include <vector>
#include "fnvcoop/effect.h"
#include "fnvcoop/mutations.h"

namespace fnvcoop {

class EffectCapture {
public:
    // Returns true if the effect was accepted (hooks may then let the engine mutation proceed).
    // false = blocked: either we are applying a remote/ghost event (EchoGuard), or this is a guest machine and the
    // mutation did not come from a genuine local player action (LocalAction). Guests never author shared state
    // from scripts: only the host's EffectTxn does.
    bool record(const Effect& e) {
        if (suppressed()) { ++suppressed_; return false; }
        if (guest_ && localDepth_ == 0 && !mutations().noGuestSuppression) { ++guestBlocked_; return false; }
        buf_.push_back(e);
        return true;
    }
    bool suppressed() const { return depth_ > 0 && !mutations().noEchoGuard; }
    void setGuest(bool g) { guest_ = g; }  // ClientSession sets this; the host leaves it false
    bool guest() const { return guest_; }
    size_t guestBlockedCount() const { return guestBlocked_; }
    bool empty() const { return buf_.empty(); }
    std::vector<Effect> take() { std::vector<Effect> r; r.swap(buf_); return r; }
    // Take back the most recent recording of `e` (the engine refused the mutation after the hook recorded it on entry). False if
    // nothing equal is buffered (already emitted, or never recorded).
    bool retractLast(const Effect& e) {
        for (size_t i = buf_.size(); i-- > 0;)
            if (buf_[i] == e) { buf_.erase(buf_.begin() + static_cast<std::ptrdiff_t>(i)); return true; }
        return false;
    }
    size_t suppressedCount() const { return suppressed_; }

private:
    friend class EchoGuard;
    friend class LocalAction;
    std::vector<Effect> buf_;
    int depth_ = 0, localDepth_ = 0;
    bool guest_ = false;
    size_t suppressed_ = 0, guestBlocked_ = 0;
};

class EchoGuard {
public:
    explicit EchoGuard(EffectCapture* c) : c_(c) { if (c_) ++c_->depth_; }
    ~EchoGuard() { if (c_) --c_->depth_; }
    EchoGuard(const EchoGuard&) = delete;
    EchoGuard& operator=(const EchoGuard&) = delete;
private:
    EffectCapture* c_;
};

// Marks a genuine, locally authoritative player action (pickup, own-weapon kill, ...) on a guest machine.
// Wrap the engine mutation hooks of such an action in this scope, then submit the captured effects as an intent.
class LocalAction {
public:
    explicit LocalAction(EffectCapture* c) : c_(c) { if (c_) ++c_->localDepth_; }
    ~LocalAction() { if (c_) --c_->localDepth_; }
    LocalAction(const LocalAction&) = delete;
    LocalAction& operator=(const LocalAction&) = delete;
private:
    EffectCapture* c_;
};

// Alias with the name hooks should use for "currently applying a remote / ghost event": ghost-death kills,
// remote txn application, anything the engine does on this machine only because another machine did it.
using RemoteEventScope = EchoGuard;

}  // namespace fnvcoop
