// RuntimeSide: IAvatarSide over avatar::engine::AvatarRuntime (plugin/avatar/engine). Header only; included by the glue and the tests, which
// link nvcoop_avatar_engine (or its _rt twin library).
#pragma once
#include "avatar_side.h"
#include "avatar_runtime.h"

namespace nvcoop::livewire {

class RuntimeSide : public IAvatarSide {
public:
    explicit RuntimeSide(nvcoop::avatar::engine::AvatarRuntime& rt) : rt_(rt) {}
    void setSnapshotSink(SnapshotSink s) override { rt_.SetSnapshotSink(std::move(s)); }
    void setEquipSink(EquipSink s) override { rt_.SetEquipSink(std::move(s)); }
    void setLocalIdentity(fnvcoop::live::NetId n, fnvcoop::live::OwnerEpoch e) override { rt_.SetLocalIdentity(n, e); }
    void setLocalSlot(nvcoop::avatar::Slot s) override { rt_.SetLocalSlot(s); }
    bool addRemote(nvcoop::avatar::Slot s, const nvcoop::avatar::Appearance& a) override { return rt_.AddRemote(s, a); }
    void removeRemote(nvcoop::avatar::Slot s) override { rt_.RemoveRemote(s); }
    bool setRemoteAppearance(nvcoop::avatar::Slot s, const nvcoop::avatar::Appearance& a) override { return rt_.SetRemoteAppearance(s, a); }
    bool readLocalAppearance(nvcoop::avatar::Appearance& out) override { return rt_.ReadLocalAppearance(out); }
    bool readLocalFacts(nvcoop::avatar::PlayerFacts& out) override { return rt_.ReadLocalFacts(out); }
    bool readLocalPack(const std::vector<nvcoop::avatar::FormId>& watch, nvcoop::avatar::PlayerPack& out) override { return rt_.ReadLocalPack(watch, out); }
    bool readLocalEquipment(std::vector<nvcoop::avatar::EquipEntry>& out) override { return rt_.ReadLocalEquipment(out); }
    void onRemoteSnapshot(nvcoop::avatar::Slot s, const fnvcoop::live::ActorSnapshot& snap, uint32_t arrivalMs) override { rt_.OnRemoteSnapshot(s, snap, arrivalMs); }
    void onRemoteEquipment(nvcoop::avatar::Slot s, const std::vector<nvcoop::avatar::EquipEntry>& w) override { rt_.OnRemoteEquipment(s, w); }
    void noteStage(nvcoop::avatar::NetStage st, double ms) override { rt_.NoteStage(st, ms); }
    void tick(uint32_t nowMs) override { rt_.Tick(nowMs); }
    void onEvent(Event e) override {
        using GE = nvcoop::avatar::engine::GameEvent;
        switch (e) {
            case Event::LoadBegin: rt_.OnEvent(GE::LoadBegin); break;
            case Event::LoadEnd: rt_.OnEvent(GE::LoadEnd); break;
            case Event::ExitToMenu: rt_.OnEvent(GE::ExitToMenu); break;
            case Event::SaveBegin: rt_.OnEvent(GE::SaveBegin); break;
            case Event::SaveEnd: rt_.OnEvent(GE::SaveEnd); break;
        }
    }
    bool purgePending() const override { return const_cast<nvcoop::avatar::engine::AvatarRuntime&>(rt_).manager().PurgePending(); }
    nvcoop::avatar::RefId refOf(nvcoop::avatar::Slot s) const override { return rt_.RefOf(s); }
    bool slotOf(nvcoop::avatar::RefId ref, nvcoop::avatar::Slot& slot) const override {
        auto s = rt_.SlotOf(ref);
        if (!s) return false;
        slot = *s;
        return true;
    }
    std::string statusJson() override { return rt_.StatusJson(); }
    nvcoop::avatar::engine::AvatarRuntime& runtime() { return rt_; }

private:
    nvcoop::avatar::engine::AvatarRuntime& rt_;
};

}  // namespace nvcoop::livewire
