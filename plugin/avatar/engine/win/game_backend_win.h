// The one place that touches the running game for the puppet layer (x86 only, compiled into nvcoop.dll against the xNVSE headers).
// Every raw read / call is guarded with SEH and checked against the pins in ../engine_pins.h; a pin that does not match the live
// process disables the one function it guards (the caller then sees "unknown"), it never calls an address that was not verified.
#pragma once
#include <cstdint>
#include <string>

#include "../game_backend.h"

struct NVSEConsoleInterface;

namespace nvcoop::avatar::engine {

class WinBackend : public IGameBackend {
public:
    explicit WinBackend(NVSEConsoleInterface* console);

    // NVSE messages that change the world (PreLoadGame, PostLoadGame, NewGame, ExitToMainMenu): bumps WorldEpoch.
    void NoteWorldChanged() { ++epochBase_; }

    // {"pins":[{"name":..,"addr":..,"ok":true}],"consoleOk":..,"nonCombatLines":..}: shown in avatar.status
    std::string PinsJson() const;
    bool PinOk(const char* name) const;
    bool Ready() const { return console_ != nullptr && lookupOk_; }

    uint32_t WorldEpoch() override;
    bool MenuOpen() override;
    bool ReadCell(CellId cell, CellInfo& out) override;
    RefId AnchorRef(CellId cell, Vec3& pos) override;
    void ListActors(CellId cell, std::vector<ActorEntry>& out) override;
    void LoadedCells(std::vector<CellId>& out) override;
    bool ReadRef(RefId ref, RefInfo& out) override;
    bool ReadAnim(RefId ref, AnimRead& out) override;
    bool ReadEquipped(RefId ref, std::vector<EquippedItem>& out) override;
    bool ReadAppearance(RefId ref, Appearance& out) override;
    FormId CloneNpcBase(FormId base) override;
    bool WriteNpcLook(FormId base, FormId tmpl, const Appearance& a, bool* faceWritten) override;
    int RunLine(RefId ctx, const std::string& line) override;
    bool Eval(Cond fn, RefId ref, FormId param, double& out) override;
    bool ReadPlayerRegions(std::vector<FormId>& out, size_t max) override;
    uint64_t NowUs() override;

private:
    NVSEConsoleInterface* console_;
    bool pinOk_[16] = {};
    bool lookupOk_ = false;
    uint32_t epochBase_ = 1;
    uint32_t lastPlayerPtr_ = 0;
    uint32_t playerSwaps_ = 0;
    uint64_t qpcFreq_ = 1;
};

}  // namespace nvcoop::avatar::engine
