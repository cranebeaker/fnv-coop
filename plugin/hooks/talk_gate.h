// Who gets a conversation on a guest. The host's story makes NPCs start conversations (ForceGreet packages after an EVP, StartConversation in
// a script): the host replicates the EVP, so each copy's NPC walked up to ITS OWN player and the conversation opened on every machine at once
// (Amata at the vault door, Dad in CG03). Only the host's choices count (a guest never runs INFO result scripts), so a guest got a conversation
// that changed nothing and had to be clicked through. On a guest a conversation now opens only when the local player asked for it: they
// activated that NPC a moment ago. Every conversation, player- or NPC-started, goes through one engine call (0x705070, the pending dialogue
// request run from the main loop), which is where the plugin asks allow(). Game-free so the test runs it without the engine.
#pragma once
#include <cstdint>

#include "fnvcoop_mut.h"

namespace nvhooks {

class TalkGate {
public:
    static constexpr uint64_t kOwnActivationMs = 2000;   // activation -> its OnActivate block: the same or the next frame
    static constexpr uint64_t kAskWindowMs = 5000;   // activation -> DialogMenu: a frame or two, longer when the NPC first turns around

    void setEnabled(bool on) { enabled_ = on; }
    bool enabled() const { return enabled_; }
    // The local player activated `refID` (outside scripts, not while applying a host txn).
    void noteActivate(uint32_t refID, uint64_t nowMs) { asked_ = activated_ = refID; askedAt_ = activatedAt_ = nowMs; }
    // An OnActivate block hands the activation back to the engine with `Activate` (Lucas Simms: talk only goes on from there). On a guest that
    // command runs inside a script, where shared mutations are blocked: true = it is this player's own activation of `refID`, let it run.
    bool ownActivation(uint32_t refID, uint64_t nowMs) const {
        return refID && refID == activated_ && nowMs - activatedAt_ <= kOwnActivationMs && !FNV_MUT(16);   // twin 16: the block swallows it again
    }
    // The plugin itself opens a conversation with `refID` on this machine (the 0xD1 dialogue lane: a spectator view of the host's talk, or the
    // player's own talk resumed after the activate observer deferred it): let it through like an activation.
    void expect(uint32_t refID, uint64_t nowMs) { noteActivate(refID, nowMs); }
    // A conversation with `speaker` is about to open. False = the NPC started it: it belongs to the host's player.
    bool allow(uint32_t speaker, uint64_t nowMs) {
        if (!enabled_) { current_ = speaker; return true; }
        const bool asked = speaker && speaker == asked_ && nowMs - askedAt_ <= kAskWindowMs;
        if (asked || FNV_MUT(15)) {   // twin 15: every conversation opens again (the NPC-started one too)
            asked_ = 0;               // one activation opens one conversation
            lastBlocked_ = 0;
            current_ = speaker;
            ++opened_;
            return true;
        }
        if (speaker != lastBlocked_) ++blocked_;   // a refused request comes back every frame until the NPC gives up: count it once
        lastBlocked_ = speaker;
        return false;
    }
    void reset() { asked_ = 0; askedAt_ = 0; lastBlocked_ = 0; current_ = 0; activated_ = 0; activatedAt_ = 0; }
    // The speaker of the last conversation that opened here (this player's own talk): its INFO result scripts are this player's.
    uint32_t current() const { return current_; }
    uint64_t opened() const { return opened_; }
    uint64_t blocked() const { return blocked_; }

private:
    bool enabled_ = false;
    uint32_t asked_ = 0;
    uint64_t askedAt_ = 0;
    uint32_t lastBlocked_ = 0;
    uint32_t current_ = 0;
    uint32_t activated_ = 0;
    uint64_t activatedAt_ = 0;
    uint64_t opened_ = 0, blocked_ = 0;
};

}  // namespace nvhooks
