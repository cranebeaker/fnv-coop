// Say-once ("already said") state across players (L8, owner decision D11). Game-free, header-only.
//
// The engine marks an INFO as said on the machine that speaks it (TESTopicInfo::MarkSaid 0x61F220, also called by
// RunResultScript on the host for a guest's pick). With initiator-picks that left the other machines out of step: player B could still
// choose a Speech-success line player A already used, and the host player never saw it as said.
// Policy: say-once state is SHARED, so every machine that watches a line (spectators and the host) marks the INFO said, EXCEPT the
// per-player lines in data/sayonce_per_player.txt (Speech-XP and trainer lines), which every player may use once for themselves.
// For a per-player line the host also undoes the mark its own RunResultScript made for the guest's pick, unless its own player
// had already said it.
#pragma once
#include <istream>
#include <set>
#include <sstream>
#include <string>
#include "fnvcoop/types.h"
#include "fnvcoop_dlg/mutations.h"

namespace fnvcoop_dlg {

struct SayOnceAction {
    enum Kind : uint8_t { None, Mark, Clear } kind = None;
};

class SayOncePolicy {
public:
    void setPerPlayer(std::set<fnvcoop::FormRef> s) { perPlayer_ = std::move(s); }
    size_t perPlayerCount() const { return perPlayer_.size(); }
    bool perPlayer(const fnvcoop::FormRef& info) const { return perPlayer_.count(info) != 0; }

    // The local player started a line themselves (their engine marked it): remembered so the host never undoes a mark it owns.
    void noteOwnSaid(const fnvcoop::FormRef& info) { own_.insert(info); }
    bool ownSaid(const fnvcoop::FormRef& info) const { return own_.count(info) != 0; }
    void reset() { own_.clear(); }

    // A line of someone else's conversation was received (spectator / host side). `isHost`: this machine ran the result script.
    SayOnceAction onSpectated(const fnvcoop::FormRef& info, bool isHost) const {
        SayOnceAction a;
        const bool per = perPlayer(info) && !mutations().sayOnceAllShared;
        if (mutations().sayOnceNeverShared) return a;                       // MUTANT: nothing is shared (the old behaviour)
        if (!per) { a.kind = SayOnceAction::Mark; return a; }
        if (isHost && !ownSaid(info) && !mutations().sayOnceKeepHostMark) a.kind = SayOnceAction::Clear;
        return a;
    }

    // ---- the engine's own marks (TESTopicInfo::MarkSaid 0x61F220 detour, plugin/hooks_ui), seen on THIS machine
    // Spoken lines reach the other machines through the dialogue protocol (onSpectated). The marks nobody reports are the ones the engine makes by
    // itself: a scripted `SayTo` in a quest stage result runs on every machine that applies the stage, and only the copy whose NPC is loaded and
    // near speaks (and marks) it (story S06: SayOnceFlag 10A1E6 set on the guest, absent on the host, never repaired because the digest only copies
    // the host's flags to guests). Shared lines: a guest reports the mark to the host, the host commits it (marks its own copy and records the
    // SayOnceFlag effect), so every copy agrees. Per-player lines (Speech-XP, trainer; D11) stay with the player who used them: never reported.
    struct Route {
        enum Kind : uint8_t { Ignore, ReportToHost, CommitOnHost } kind = Ignore;
    };
    // The detour only reports a byte that went 0 -> 1 in the call.
    Route onEngineMark(const fnvcoop::FormRef& info, bool isHost) const {
        Route r;
        if (info.plugin.empty()) return r;
        if (perPlayer(info) && !mutations().sayOnceMarkTrainerShared) return r;
        if (isHost) { r.kind = mutations().sayOnceMarkHostNoCommit ? Route::Ignore : Route::CommitOnHost; return r; }
        if (mutations().sayOnceMarkNotReported) return r;
        r.kind = Route::ReportToHost;
        return r;
    }
    // The host got a guest's report. CommitOnHost = mark the host's copy and record the SayOnceFlag effect.
    Route onGuestReport(const fnvcoop::FormRef& info) const {
        Route r;
        if (info.plugin.empty() || mutations().sayOnceMarkHostDrops || mutations().sayOnceMarkHostNoCommit) return r;
        if (perPlayer(info) && !mutations().sayOnceMarkTrainerShared) return r;
        r.kind = Route::CommitOnHost;
        return r;
    }

private:
    std::set<fnvcoop::FormRef> perPlayer_, own_;
};

// "speech|trainer <plugin>:<HEXID>" lines (plugin/dialogue/tools/gen_player_data.py).
inline std::set<fnvcoop::FormRef> loadSayOncePerPlayer(std::istream& in) {
    std::set<fnvcoop::FormRef> s;
    std::string line;
    while (std::getline(in, line)) {
        if (line.empty() || line[0] == '#') continue;
        std::istringstream ls(line);
        std::string kind, ref;
        if (!(ls >> kind >> ref) || (kind != "speech" && kind != "trainer")) continue;
        size_t c = ref.rfind(':');
        if (c == std::string::npos) continue;
        std::string plugin = ref.substr(0, c);
        for (auto& ch : plugin) ch = char(tolower(static_cast<unsigned char>(ch)));
        s.insert(fnvcoop::FormRef(plugin, uint32_t(std::stoul(ref.substr(c + 1), nullptr, 16))));
    }
    return s;
}

}  // namespace fnvcoop_dlg
