// AvatarRuntime: everything the player-avatar layer does per frame, game-free, on top of IGameBackend.
//
//   local player  --ReadLocalFrame--> LocalSampler (20 Hz) --> snapshot sink (the co-op layer sends it)
//                 --EquipWatcher---->                      --> equipment sink (reliable channel)
//   remote player <-- OnRemoteSnapshot(slot, snap, arrivalMs) --> live::InterpBuffer --> PuppetManager --> GamePuppetEngine --> game
//
// Plus the dev "loopback": a puppet of the LOCAL player driven by its own snapshots through the same interpolation buffer, shifted by
// a fixed offset. One game instance is enough to prove the whole puppet chain with the T7 comparison (tools/avatar/proof_puppet.py
// --loopback): the real player and its puppet are both in the same probe.
//
// The co-op layer (plugin/coop, owned by the dll-wiring agent) needs four calls: SetLocalIdentity, SetSnapshotSink / SetEquipSink,
// AddRemote / RemoveRemote and OnRemoteSnapshot / OnRemoteEquipment. docs/tools/avatar.md "Wiring" lists them.
#pragma once
#include <cstdint>
#include <deque>
#include <functional>
#include <map>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "../puppet_manager.h"
#include "avatar_tuning.h"
#include "game_backend.h"
#include "local_sampler.h"
#include "puppet_game_engine.h"

namespace nvcoop::avatar::engine {

struct RuntimeConfig {
    Config puppet;                          // localSlot, maxPuppets, templates, defaultTemplate, thresholds
    EngineOptions engine;                   // taggedBases is extended with every template base
    LocalSamplerConfig sampler = DefaultLagTuning().sampler;   // 30 Hz, the shipped velocity filter (avatar_tuning.h)
    live::InterpConfig interp = DefaultLagTuning().interp;     // low-latency playback
    uint32_t equipPollMs = 400;
    Slot loopbackSlot = 1;                  // the slot the loopback puppet uses (must differ from puppet.localSlot)
    Vec3 loopbackOffset{160.0f, 0.0f, 0.0f};   // world-space shift of the loopback puppet
    uint32_t saveHoldMs = 10000;            // puppets stay away this long after SaveBegin if nobody says SaveEnd (a save that never reported back)
};

enum class GameEvent : uint8_t { SaveBegin, SaveEnd, LoadBegin, LoadEnd, NewGame, ExitToMenu };

class AvatarRuntime {
public:
    AvatarRuntime(IGameBackend& be, RuntimeConfig cfg);
    AvatarRuntime(const AvatarRuntime&) = delete;
    AvatarRuntime& operator=(const AvatarRuntime&) = delete;

    using SnapshotSink = std::function<void(const live::ActorSnapshot&)>;
    using EquipSink = std::function<void(const std::vector<EquipEntry>&)>;
    void SetSnapshotSink(SnapshotSink s) { snapSink_ = std::move(s); }
    void SetEquipSink(EquipSink s) { equipSink_ = std::move(s); }
    void SetLog(std::function<void(const std::string&)> log);
    void SetLocalIdentity(live::NetId netId, live::OwnerEpoch epoch);

    // This machine's own slot as the session assigned it (the ini default 0 is only right for the host). Call before AddRemote.
    void SetLocalSlot(Slot slot);

    // ---- remote players ---------------------------------------------------------------------------------------------
    bool AddRemote(Slot slot, const Appearance& a);
    void RemoveRemote(Slot slot);
    // A changed look of a known remote player: applied to the live puppet (a new puppet only when race or sex pick another template).
    // False when the slot is unknown.
    bool SetRemoteAppearance(Slot slot, const Appearance& a);
    // The local player's own look (race, sex, hair, eyes, body, face data) as the backend reads it. False when unreadable.
    bool ReadLocalAppearance(Appearance& out);
    // W3a item 3: level, sit state, active actions and regions of the local player (evaluators + the region list). False when nothing could be read.
    bool ReadLocalFacts(PlayerFacts& out);
    // Guest pack facts: the local player's InventoryWeight and the counts of the watched item forms (only those above zero, sorted by form). False when nothing could be read
    // (the player is not readable, or every evaluator is unavailable).
    bool ReadLocalPack(const std::vector<FormId>& watch, PlayerPack& out);
    // The worn list the equipment watcher last published (the stable one the equipment sink got). For a consumer that was not listening at that moment: a session
    // that began after it, or a player entity's Equipment part (docs/entity-core-plan.md S5), which is read, not pushed. False until the watcher published once.
    bool ReadLocalEquipment(std::vector<EquipEntry>& out);
    void OnRemoteSnapshot(Slot slot, const live::ActorSnapshot& s, uint32_t arrivalMs);
    void OnRemoteEquipment(Slot slot, const std::vector<EquipEntry>& wanted);

    // ---- dev loopback -----------------------------------------------------------------------------------------------
    bool SetLoopback(bool on);
    bool loopback() const { return loopback_; }
    void SetLoopbackOffset(const Vec3& o) { cfg_.loopbackOffset = o; }

    // ---- per frame / events -----------------------------------------------------------------------------------------
    void Tick(uint32_t nowMs);
    void OnEvent(GameEvent e);

    // ---- ghost-ref map ----------------------------------------------------------------------------------------------
    RefId RefOf(Slot slot) const { return pm_.RefOf(slot); }
    std::optional<Slot> SlotOf(RefId ref) const { return pm_.SlotOf(ref); }
    // A pose held over the puppet's planned group (a downed friend lies there); kNoGroup = back to normal driving. See PuppetManager::SetPoseOverride.
    void SetPoseOverride(Slot slot, int baseGroup) { pm_.SetPoseOverride(slot, baseGroup); }

    // ---- introspection ----------------------------------------------------------------------------------------------
    PuppetManager& manager() { return pm_; }
    GamePuppetEngine& engine() { return eng_; }
    LocalSampler& sampler() { return sampler_; }
    const RuntimeConfig& config() const { return cfg_; }
    std::string StatusJson();   // avatar.status
    std::string LocalJson();    // avatar.local: the last local sample and what the engine said
    std::string StagesJson() const;   // avatar.status "stages"
    uint64_t remoteSnapshots() const { return remoteSnaps_; }
    uint64_t droppedRemote() const { return droppedRemote_; }
    uint64_t posesFed() const { return posesFed_; }

    // Stage timers (avatar.status "stages"): where the time between the sender's sample and the shown puppet goes. The network layer reports its
    // stages through NoteStage; the runtime measures the rest itself.
    void NoteStage(NetStage st, double ms);
    struct Stages {
        live::Gauge send, rx;      // reported by the network layer (NetStage)
        live::Gauge netPhase;      // the network layer's frame minus the avatar layer's last Tick: > 0 = it runs after it in the same frame, as designed
        live::Gauge netGap;        // time between two network-layer frames
        live::Gauge frameGap;      // time between two Tick calls (the avatar layer's frame)
        live::Gauge sampleUs;      // reading the local player + building the snapshot + the sink (sender)
        live::Gauge driveUs;       // sampling the buffers + SetPose
        live::Gauge applyUs;       // PuppetManager::Tick: the engine calls (SetPos ...)
        live::Gauge localDrift3D;  // the local player: distance between his reference position (what is sent) and his 3D node (what is drawn), game units
        live::Gauge poseToSetPosMs; // from the start of Tick (where the pose is computed) to the end of the engine calls, in ticks that moved a puppet
    };
    const Stages& stages() const { return stages_; }
    // avatar.trace: the last `n` samples of the local player as published (tick, position, velocity) and, per remote, of every frame's pose fed to the puppet
    // (frame time, shown position, and where the engine said the puppet stood at the start of that frame). Both are stamped on the same clock, so on one PC a
    // walker's path and his puppet's can be laid over each other without the bridge's read times (docs/tools/avatar.md "stage timers").
    std::string TraceJson(size_t n) const;
    const live::InterpDiag* InterpOf(Slot slot) const {
        auto it = remotes_.find(slot);
        return it == remotes_.end() ? nullptr : &it->second->buf.diag();
    }

private:
    struct Remote {
        live::InterpBuffer buf;
        explicit Remote(const live::InterpConfig& c) : buf(c) {}
        bool everSampled = false;
    };

    void PublishLocal(uint32_t nowMs);
    void LogLocalAction(const live::ActorSnapshot& s);
    void DriveRemotes(uint32_t nowMs);
    void Log(const std::string& s) const { if (log_) log_(s); }

    IGameBackend& be_;
    RuntimeConfig cfg_;
    GamePuppetEngine eng_;
    PuppetManager pm_;
    LocalSampler sampler_;
    EquipWatcher equip_;
    std::map<Slot, std::unique_ptr<Remote>> remotes_;
    SnapshotSink snapSink_;
    EquipSink equipSink_;
    std::function<void(const std::string&)> log_;
    bool loopback_ = false;
    bool loading_ = false;
    live::NetId netId_ = 0;
    uint64_t remoteSnaps_ = 0, droppedRemote_ = 0, posesFed_ = 0;
    uint32_t lastTick_ = 0;
    bool saveHeld_ = false;
    uint32_t saveHeldAt_ = 0;
    live::ActorSnapshot lastSnap_;
    bool haveSnap_ = false;
    Stages stages_;
    struct LocalPt { uint32_t t; float x, y, z, vx, vy, vz; };
    struct RemotePt { uint32_t t; float x, y, z, ax, ay, az; uint8_t state; float delay, want, jitter, newestAge; };
    static constexpr size_t kTraceLocal = 900, kTraceRemote = 1800;
    std::deque<LocalPt> traceLocal_;
    std::map<Slot, std::deque<RemotePt>> traceRemote_;
    uint64_t lastTickUs_ = 0;
    bool haveTickUs_ = false;
    uint32_t lastSetPosCount_ = 0;
    // log lines of the local player's attacks (PublishLocal): events already logged, the process action and animation sets last logged
    uint64_t loggedActionEvents_ = 0;
    int loggedProcAction_ = -2, loggedSetState_ = -1, actionLogs_ = 0;
    uint32_t lastNetFrame_ = 0;
    bool haveNetFrame_ = false;
};

}  // namespace nvcoop::avatar::engine
