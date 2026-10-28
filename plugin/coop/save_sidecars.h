// Per-save sidecar files of the plugin's own state (campaign gap G35 / P4 handover 8). Game-free and header-only.
//
// State that belongs to ONE save game (the gear ledger of plugin/intents: whose gear sits in the Fort locker) must travel with that save: a ledger kept
// in one file next to the dll would survive loading an older save and hand out gear that the loaded world never took away. So every kind of
// sidecar is written next to the .fos as <save><ext> when the game saves, and restored from the file of the save that is loaded. A save without the file
// (made before the feature, or by a build without it) RESETS the state: restore("") is the "empty" state, never "keep what you have".
//
// The module owns one SaveSidecars, adds a Kind per state, and forwards the NVSE messages: SaveGame (path), PreLoadGame (path), PostLoadGame.
#pragma once
#include <functional>
#include <string>
#include <vector>

#include "fnvcoop_mut.h"

namespace nvcoop::coop {

class SaveSidecars {
public:
    struct Kind {
        std::string ext;                                    // ".fnvintents"
        std::function<std::string()> snapshot;              // current state as text; empty = nothing to store (the feature is off)
        std::function<void(const std::string& text)> restore;   // text of the loaded save's file; "" = that save has none: back to the empty state
    };
    struct Io {
        std::function<bool(const std::string& path, const std::string& text)> write;
        std::function<bool(const std::string& path, std::string* text)> read;     // false = no such file
        std::function<void(const std::string& line)> log;
    };

    void add(Kind k) { kinds_.push_back(std::move(k)); }
    size_t size() const { return kinds_.size(); }

    // NVSE SaveGame: the save at `savePath` was written.
    void onSave(const std::string& savePath, const Io& io) {
        for (const Kind& k : kinds_) {
            const std::string text = k.snapshot ? k.snapshot() : std::string();
            if (text.empty()) continue;
            if (FNV_MUT(16)) continue;   // MUTANT 16: nothing is written next to the save (the ledger would be shared by every save)
            const bool ok = io.write && io.write(savePath + k.ext, text);
            if (io.log) io.log((ok ? "sidecar written " : "sidecar NOT written ") + savePath + k.ext);
        }
    }
    // NVSE PreLoadGame announces the path; PostLoadGame (successful) carries none.
    void onPreLoad(const std::string& savePath) { loadPath_ = savePath; }
    void onPostLoad(const Io& io) {
        for (const Kind& k : kinds_) {
            std::string text;
            bool have = !loadPath_.empty() && io.read && io.read(loadPath_ + k.ext, &text);
            if (!have) text.clear();
            if (FNV_MUT(17) && !have) continue;   // MUTANT 17: a save without the file keeps whatever state the previous save left behind
            if (k.restore) k.restore(text);
            if (io.log) io.log(std::string(have ? "sidecar restored " : "no sidecar, state reset ") + loadPath_ + k.ext);
        }
    }

private:
    std::vector<Kind> kinds_;
    std::string loadPath_;
};

}  // namespace nvcoop::coop
