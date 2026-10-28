// A spectator machine plays a guest's line with the console command `<npc>.Say <topic>` (nvui::ops::sayTopic -> nvhooks::execConsoleLineOn). The engine's Say
// does more than talk: it runs the result scripts (Begin, End) of the INFO it picks, by itself, as one more host-side run.
//
// Story run S06 (2026-10-28, the second PC): the guest picked "Sure, I'll come with you" (INFO 10A1EB, `Player.AddItem Ammo556mm 60`). The host's log shows the
// engine running that Begin script three times, 30 -> 90 -> 150 -> 210 rounds: once by the INFO result runner (the guest's window, committed as seq 175, origin 1) and
// twice natively while the host played the two response lines of the INFO (seq 176 and 177, origin 0). Every one of them committed a full-mask AddItem, so both copies
// gained 180 (the other run 240: one more line). The runner is the single authority for a guest's result scripts; the engine's own run for the voice is a copy.
//
// The gate: an `execConsoleLineOn("Say ...")` arms a window for that NPC; an INFO result script the engine starts for that NPC (or with no speaker) inside the window,
// outside every acting window (the runner opens one), does not run. Game-free so that the host-side flow can be driven in the co-op tests.
#pragma once

#include <cstdint>

#include "fnvcoop_mut.h"

namespace nvhooks {

// True for a console line that makes an NPC speak: "Say <topic> ...", leading blanks and any letter case.
inline bool isSayLine(const char* line) {
    if (!line) return false;
    while (*line == ' ' || *line == '\t') ++line;
    const char w[] = "say";
    for (int i = 0; i < 3; ++i) {
        char c = line[i];
        if (c >= 'A' && c <= 'Z') c = static_cast<char>(c - 'A' + 'a');
        if (c != w[i]) return false;
    }
    return line[3] == ' ' || line[3] == '\t';
}

class SpectatorSayGate {
public:
    static constexpr uint64_t kWindowMs = 12000;   // a response line plays for a few seconds; its End script runs when the audio ends
    static constexpr int kSlots = 8;
    static constexpr uint32_t kPlayerRef = 0x14;

    // A Say was issued on `npcRefID` (0 = unknown: every speaker is covered).
    void arm(uint32_t npcRefID, uint64_t nowMs) {
        ++armed_;
        for (E& e : e_)
            if (e.until && e.npc == npcRefID) { e.until = nowMs + kWindowMs; return; }
        for (E& e : e_)
            if (!e.until || e.until < nowMs) { e.npc = npcRefID; e.until = nowMs + kWindowMs; return; }
        e_[next_] = E{npcRefID, nowMs + kWindowMs};   // all eight in use and live: replace the oldest slot in turn
        next_ = (next_ + 1) % kSlots;
    }

    // True = the engine's own run of an INFO result script (speaker `actorRefID`) must be skipped. `insideRunner`: an acting window is open, i.e. the call comes from the
    // INFO result runner of the dialogue host, the one run that counts.
    bool blocks(uint32_t actorRefID, bool insideRunner, uint64_t nowMs) {
        if (FNV_MUT(97)) return false;   // MUTANT 97: the engine's own run for a spectator voice goes ahead (the reward lands once per response line, again)
        if (insideRunner) return false;
        for (const E& e : e_) {
            if (!e.until || e.until < nowMs) continue;
            if (actorRefID == 0 || actorRefID == kPlayerRef || e.npc == 0 || e.npc == actorRefID) {
                ++blocked_;
                return true;
            }
        }
        return false;
    }

    uint64_t armedCount() const { return armed_; }
    uint64_t blockedCount() const { return blocked_; }
    void clear() { for (E& e : e_) e = E{}; next_ = 0; }

private:
    struct E {
        uint32_t npc = 0;
        uint64_t until = 0;
    };
    E e_[kSlots];
    int next_ = 0;
    uint64_t armed_ = 0, blocked_ = 0;
};

}  // namespace nvhooks
