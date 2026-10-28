#include "suppress_policy.h"

#include "fnvcoop/authority.h"

namespace nvhooks {

#ifdef NVHOOKS_MUTANT
constexpr bool kMutIgnoreApplying = true;   // suppression not lifted while applying a host txn
constexpr bool kMutQuestGateOff = true;     // quest scripts run on guests
constexpr bool kMutIgnoreDepth = true;      // block even outside scripts (breaks loads / console)
#else
constexpr bool kMutIgnoreApplying = false;
constexpr bool kMutQuestGateOff = false;
constexpr bool kMutIgnoreDepth = false;
#endif

bool SuppressPolicy::skipScript(uint16_t scriptType, uint32_t scriptRefID) {
#ifndef NVHOOKS_MUTANT
    if (scriptRefID && localRun_.count(scriptRefID)) return false;  // GuestLocalRun: this quest script runs on every machine
#endif
    if (kMutQuestGateOff) return false;
    if (!guest_ || !questGate_ || scriptType != kScriptTypeQuest) return false;
    if (fnvcoop::questScriptsMayRun(fnvcoop::Role::Guest)) return false;  // L4 authority model (test-only override)
    if (applying_ > 0 && !kMutIgnoreApplying) return false;
    ++stats_.questScripts;
    return true;
}

bool SuppressPolicy::sharesQuestVars(uint32_t questScriptRefID) const {
#ifdef NVHOOKS_MUTANT
    return true;  // mutant: per-machine quest vars leak onto the wire
#else
    return !isGuestLocalRun(questScriptRefID);
#endif
}

bool SuppressPolicy::blockMutation() {
    if (!guest_) return false;
    if (applying_ > 0 && !kMutIgnoreApplying) return false;
    if (depth_ <= 0 && !kMutIgnoreDepth) return false;
#ifndef NVHOOKS_MUTANT
    if (localDepth_ > 0) return false;  // inside an allow-listed per-machine quest script
#endif
    ++stats_.mutations;
    return true;
}

bool SuppressPolicy::blockQuestState() {
    if (!guest_) return false;
    if (applying_ > 0 && !kMutIgnoreApplying) return false;
    if (depth_ <= 0 && !kMutIgnoreDepth) return false;
#ifndef NVHOOKS_MUTANT
    // inside an allow-listed per-machine script the quest calls pass too, except for the DLC setup scripts whose quest state belongs to the host
    if (localDepth_ > 0 && !inHostQuestStateScript()) return false;
#else
    if (localDepth_ > 0) return false;   // mutant: the host-quest-state exemption is gone, a guest's DLC setup script starts the quests itself
#endif
    ++stats_.mutations;
    return true;
}

bool SuppressPolicy::blockResultScript() {
    if (!guest_) return false;
    if (applying_ > 0 && !kMutIgnoreApplying) return false;
    ++stats_.resultScripts;
    return true;
}

#ifdef NVHOOKS_MUTANT
constexpr bool kMutGateIgnoresBypass = true;  // deferred stage can never be re-issued (livelock)
#else
constexpr bool kMutGateIgnoresBypass = false;
#endif

bool SuppressPolicy::isLocalPlayerConsoleLine(const char* text) {
#ifdef NVHOOKS_MUTANT
    (void)text;
    return false;  // mutant: a guest's own-player console lines are dropped again (a scenario cannot move the guest)
#else
    if (!text) return false;
    const char* p = text;
    while (*p == ' ' || *p == '\t') ++p;
    static const char kPrefix[] = "player.";
    for (size_t i = 0; i + 1 < sizeof(kPrefix); ++i) {
        char c = p[i];
        if (c >= 'A' && c <= 'Z') c = static_cast<char>(c - 'A' + 'a');
        if (c != kPrefix[i]) return false;
    }
    // one line only: a multi-line console script could hide a shared-state mutation after the player line
    for (const char* q = p; *q; ++q) {
        if (*q == '\n' || *q == '\r') {
            for (const char* r = q; *r; ++r)
                if (*r != '\n' && *r != '\r' && *r != ' ' && *r != '\t') return false;
            break;
        }
    }
    return true;
#endif
}

bool StageGate::allow(uint32_t quest, uint32_t stage, bool applying) {
    if (!fn_) return true;
    if (applying) return true;
    if (bypass_ > 0 && !kMutGateIgnoresBypass) return true;
    if (fn_(quest, stage)) return true;
    ++deferred_;
    return false;
}

}  // namespace nvhooks
