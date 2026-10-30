// AvatarNet: the network service for player avatars (FULL-SYNC-PLAN 3.1). A live-hub service on channel kChannelAvatars.
//
//   every machine : takes the local player's 20 Hz snapshots from the avatar layer (IAvatarSide: sampler + equipment watcher), publishes
//                   them as ActorSnapshots of netId playerNetId(self) with the synccore/live delta codec, and hands the other players'
//                   snapshots to the avatar layer, which interpolates them and drives one restrained puppet per remote slot.
//   client        : sends its stream to the host only; hears the host (which relays everybody else) and the host's Roster.
//   host          : accepts a client's stream only when the snapshot is about THAT client's own slot (no forged avatars), relays it to the
//                   other clients, sends the Roster (who exists) and relays Look messages (appearance + equipment).
//   look          : appearance and what the player wears travel reliably when they change, and again to every newcomer / returning link.
//
// Under [entities] (docs/entity-core-plan.md, live_switches.h) two parts of this move to the actor module's player entities and AvatarNet becomes a shell around the
// avatar layer: parts=unified stops sending Roster, Look, Facts, Pack and Resync (the roster is the set of player records, the rest are parts of those records that
// ActorSync replicates; PlayerPort, below, is how it reads the local player's and writes a puppet's), and stream=unified hands the local snapshots to ActorSync
// and takes remote ones from it. Every decoder keeps working in every mode. The modes are read when the session starts.
//
// Game-free: the engine side is behind IAvatarSide (real: RuntimeSide over plugin/avatar/engine AvatarRuntime; tests: the same runtime on a
// FakeGameBackend). Unit-tested with a loopback of 3 machines (tests/test_live_wiring.cpp).
#pragma once
#include <deque>
#include <functional>
#include <map>
#include <memory>
#include <set>
#include <string>
#include <vector>

#include "avatar_side.h"
#include "avatar_types.h"
#include "avatar_wire.h"
#include "player_port.h"
#include "stream_rate.h"
#include "live/live.h"
#include "live_hub.h"
#include "actors/state_parts.h"

namespace nvcoop::livewire {

// Which players exist, as the actor module's player records say (parts=unified: the roster). The host's records follow its joined clients; a client's follow the host's.
class IEntityDirectory {
public:
    virtual ~IEntityDirectory() = default;
    virtual std::vector<PlayerId> playerSlots() const = 0;
};

// Runtime form id <-> FormRef for the Equipment and WeaponState parts (they name items by FormRef, so they do not depend on the load order). The default maps a
// runtime id to FormRef("@rt", id): lossless and game-free, which is what the handshake's equal mod list makes correct; the live glue installs the load-order resolver.
struct FormCodec {
    std::function<fnvcoop::FormRef(nvcoop::avatar::FormId)> toRef;
    std::function<nvcoop::avatar::FormId(const fnvcoop::FormRef&)> fromRef;   // 0 = unknown here
};

struct AvatarNetOptions {
    bool partsUnified = false;   // [entities] parts=unified: no Roster / Look / Facts / Pack / Resync is sent; the player parts carry them (always decoded)
    bool streamUnified = false;  // [entities] stream=unified: the local snapshots go to ActorSync (PlayerPort::sample), remote ones come from it (PlayerPort::onSnapshot)
    uint32_t ackIntervalMs = 50;
    uint32_t lookCheckMs = 500;            // how often the local look is compared with the one last sent
    uint32_t factsCheckMs = 1000;          // how often the local player's facts (level, sit state, actions, regions) are read; a guest sends them to the host only when they changed
    uint32_t packCheckMs = 500;            // how often the local pack (weight + watched item counts) is read; a guest sends it to the host only when it changed, and at most once per packMinGapMs
    uint32_t packMinGapMs = 1000;
    uint32_t poseReportMs = 200;           // how often remote players' places go to the ownership service (when they changed)
    fnvcoop::live::SenderConfig sender;
    nvcoop::avatar::StreamRateConfig rate;  // 20 Hz near / 5 Hz far / 1 Hz elsewhere (stream_rate.h); rate.enabled = false sends everything at full rate
    uint32_t appearancePollMs = 500;       // how often the local look (race, sex, hair, body, face) is read from the game (it was 2000: with the 500 ms part poll a new face took up to 2.5 s to leave)
    uint32_t lookSettleMs = 1200;          // a look that changed is offered to the others once it has stayed the same this long: the race menu edits the base live, and every offered look
                                           // makes the other machines build the puppet again. The first look of a session and a look announced by noteLookEdited() go out at once.
    bool tickSide = true;                  // the hub's tick drives the avatar layer (false: something else ticks it)
};

struct AvatarNetStats {
    uint64_t sent = 0, received = 0, accepted = 0, rejected = 0, forged = 0, noRoster = 0, relayed = 0, acksSent = 0, acksReceived = 0;
    uint64_t looksSent = 0, looksApplied = 0, looksRejected = 0, rostersSent = 0, rostersApplied = 0, badDatagrams = 0;
    uint64_t localSnapshots = 0, resyncsAnswered = 0, idleTicks = 0;
    uint64_t rateKept = 0, rateReleased = 0;          // snapshots held back by the near / far rate and released when their turn came
    uint64_t appearanceUpdates = 0, puppetRespawnsForLook = 0;
    uint64_t factsSent = 0, factsApplied = 0, factsRejected = 0;
    uint64_t packsSent = 0, packsApplied = 0, packsRejected = 0;
};

class AvatarNet : public ILiveService, public IPlayerEndpoint {
public:
    AvatarNet(IAvatarSide& side, AvatarNetOptions opt = {}, std::function<uint32_t()> epochSeed = nullptr);
    ~AvatarNet() override;

    // Where a player is (cell + position of its newest snapshot; the LOCAL player's own slot too, from its published snapshots): the ownership and
    // spawn services want every player's place. present=false when the player is gone.
    using PoseSink = std::function<void(PlayerId slot, bool present, fnvcoop::live::CellId cell, const fnvcoop::live::Vec3& pos)>;
    void setPoseSink(PoseSink s) { poseSink_ = std::move(s); }
    void setLog(std::function<void(const std::string&)> log) { log_ = std::move(log); }
    // Race / sex of the local player (for the template NPC of its puppets on the other machines). Optional; without it the default template.
    // Without it the look is read through IAvatarSide::readLocalAppearance (the game).
    void setLocalAppearanceReader(std::function<bool(nvcoop::avatar::Appearance&)> f) { localAppearance_ = std::move(f); }
    // parts=unified: where the roster comes from, how items are named in the parts, and the port ActorSync routes player calls to.
    void setEntityDirectory(const IEntityDirectory* d) { entities_ = d; }
    void setFormCodec(FormCodec c) { formCodec_ = std::move(c); }
    PlayerPort& playerPort() { return port_; }
    bool partsUnified() const { return opt_.partsUnified; }
    bool streamUnified() const { return opt_.streamUnified; }

    // ---- IPlayerEndpoint (PlayerPort) ----
    PlayerId playerSelf() const override { return self_; }
    bool playerSessionUp() const override { return inSession_; }
    bool readLocalPart(nvcoop::actors::PartKind kind, Bytes& out) override;
    bool writeRemotePart(PlayerId slot, nvcoop::actors::PartKind kind, const Bytes& value) override;
    bool takeLocalSample(fnvcoop::live::ActorSnapshot& out) override;
    void remoteSnapshot(PlayerId slot, const fnvcoop::live::ActorSnapshot& s, uint32_t arrivalMs) override;

    // ---- ILiveService ----
    const char* name() const override { return "avatars"; }
    std::vector<uint8_t> channels() const override { return {kChannelAvatars}; }
    void sessionBegin(const SessionInfo& info, ILiveLink& link) override;
    void sessionEnd() override;
    void playerJoined(PlayerId slot) override;
    void playerLeft(PlayerId slot) override;
    void hostLinkUp() override;
    void hostLinkDown() override;
    void receive(PlayerId from, const uint8_t* data, size_t size) override;
    void tick(uint64_t nowMs) override;
    void idleTick(uint64_t nowMs) override;
    void loadBegin() override;
    void loadEnd() override;
    void gameExit() override;

    // ---- queries (glue, tests, bridge) ----
    nvcoop::avatar::RefId puppetRef(PlayerId slot) const { return side_.refOf(slot); }   // 0 = none (the ghost-ref map of the acting-player design)
    bool slotOfPuppetRef(nvcoop::avatar::RefId ref, PlayerId& slot) const { return ref && side_.slotOf(ref, slot); }
    const std::set<PlayerId>& roster() const { return roster_; }
    const AvatarNetStats& stats() const { return stats_; }
    const nvcoop::avatar::StreamRatePlanner& ratePlanner() const { return rate_; }
    // Tier the stream of `subject` currently has for `observer` (diagnostics, tests).
    nvcoop::avatar::StreamTier tierOf(PlayerId observer, PlayerId subject) { return rate_.tierOf(observer, subject); }
    bool hasLook(PlayerId slot) const { return looks_.count(slot) != 0; }
    // The character creation of this machine's player ended a step (the race menu closed, the last screen was left): read the look from the game on the next poll and
    // offer it without the settle time. Without it a finished face waited for the poll and the settle (up to ~2 s) and a menu closed in between could hide the change.
    void noteLookEdited();
    // W3a item 3 (host): what is known about a remote player for the party conditions. Sex and worn items come from its Look, sneaking from its newest snapshot, the rest from
    // its Facts message. Each is false until the player sent it ("unknown": the member is skipped).
    const nvcoop::avatar::PlayerFacts* factsOf(PlayerId slot) const { auto it = facts_.find(slot); return it == facts_.end() ? nullptr : &it->second; }
    // Guest pack facts (host): the weight and the watched item counts a guest sent in its Pack message. false = unknown (no Pack yet, or the item is not on the list both machines share):
    // the caller falls back to whatever it did before (the puppet's own pack).
    const nvcoop::avatar::PlayerPack* packOf(PlayerId slot) const { auto it = packs_.find(slot); return it == packs_.end() ? nullptr : &it->second; }
    bool packCount(PlayerId slot, nvcoop::avatar::FormId item, int64_t* out) const;
    bool packWeight(PlayerId slot, double* out) const;
    // The watched item forms (runtime form ids, resolved by the module from nvcoop_pack_watch.json once the game data is loaded). Asked again at every session start and load.
    using PackWatchProvider = std::function<std::vector<nvcoop::avatar::FormId>()>;
    void setPackWatch(PackWatchProvider p) { packWatchFn_ = std::move(p); watchLoaded_ = false; watchTriedAt_ = 0; }
    const std::vector<nvcoop::avatar::FormId>& packWatch() const { return watch_; }
    bool sneakingOf(PlayerId slot, bool* out) const { auto it = lastSnap_.find(slot); if (it == lastSnap_.end()) return false; *out = it->second.anim.sneak; return true; }
    const AvLook* lookOf(PlayerId slot) const { auto it = looks_.find(slot); return it == looks_.end() ? nullptr : &it->second; }
    bool inSession() const { return inSession_; }
    PlayerId self() const { return self_; }
    std::string statusJson() const;

private:
    void log(const std::string& s) const { if (log_) log_(s); }
    // The avatar layer's clock (what the snapshot ticks are stamped with); the hub's own when the side has none.
    uint32_t clock() const { const uint32_t c = side_.clockMs(); return c ? c : static_cast<uint32_t>(nowMs_); }
    void noteQueued(PlayerId to, PlayerId subject, uint32_t tick) { if (subject == self_) queuedTicks_[to].push_back(tick); }
    void addPuppet(PlayerId slot);
    void removePuppet(PlayerId slot);
    void syncRosterFromEntities();
    void absorbLocalSamples();
    bool localAppearanceNow(nvcoop::avatar::Appearance& out);
    bool localEquipmentNow(std::vector<nvcoop::avatar::EquipEntry>& out);
    void applyRemoteAppearance(PlayerId slot, const nvcoop::avatar::Appearance& app, bool oldHadLook, const nvcoop::avatar::Appearance& oldApp);
    void rebuildRemoteEquipment(PlayerId slot);
    fnvcoop::FormRef toRef(nvcoop::avatar::FormId id) const;
    nvcoop::avatar::FormId fromRef(const fnvcoop::FormRef& r) const;
    void applyRoster(const std::vector<uint8_t>& slots);
    void sendTo(PlayerId peer, bool reliable, Bytes data);
    void broadcastReliable(const Bytes& data, PlayerId except);
    void sendRoster(PlayerId to /* kNoPlayer = everyone */);
    void sendLook(const AvLook& l, PlayerId to /* kNoPlayer = everyone except `skip` */, PlayerId skip);
    void flushStreams(uint64_t nowMs);
    void sendAcks(uint64_t nowMs);
    void reportPoses(uint64_t nowMs);
    void checkLocalLook(uint64_t nowMs);
    void checkLocalFacts(uint64_t nowMs);
    void onFacts(PlayerId from, const uint8_t* data, size_t size);
    void checkLocalPack(uint64_t nowMs);
    void onPack(PlayerId from, const uint8_t* data, size_t size);
    void ensureWatch() const;
    void onSnapshotDatagram(PlayerId from, const uint8_t* body, size_t size);
    void onLook(PlayerId from, const uint8_t* data, size_t size);
    void queueTo(PlayerId peer, PlayerId subject, const fnvcoop::live::ActorSnapshot& snap, uint64_t nowMs);
    void releasePending(uint64_t nowMs);
    std::vector<PlayerId> uplinkObservers() const;
    std::vector<PlayerId> peers() const;   // who we exchange streams with: host = joined clients, client = the host

    IAvatarSide& side_;
    AvatarNetOptions opt_;
    std::function<uint32_t()> epochSeed_;
    std::function<void(const std::string&)> log_;
    std::function<bool(nvcoop::avatar::Appearance&)> localAppearance_;
    PoseSink poseSink_;

    // session state
    ILiveLink* link_ = nullptr;
    bool inSession_ = false, isHost_ = false, hostUp_ = false;
    PlayerId self_ = kNoPlayer;
    uint32_t epoch_ = 1;
    uint64_t nowMs_ = 0;
    std::set<PlayerId> roster_;                          // remote slots that have a player (puppet or about to)
    std::set<PlayerId> joined_;                          // host: joined clients
    std::map<PlayerId, AvLook> looks_;
    std::map<PlayerId, nvcoop::avatar::PlayerFacts> facts_;   // host: the Facts message of each client
    bool factsSentValid_ = false;
    nvcoop::avatar::PlayerFacts lastFactsSent_;
    uint64_t lastFactsCheck_ = 0;
    std::map<PlayerId, nvcoop::avatar::PlayerPack> packs_;   // host: the Pack message of each client
    bool packSentValid_ = false;
    nvcoop::avatar::PlayerPack lastPackSent_;
    uint64_t lastPackCheck_ = 0, lastPackSend_ = 0;
    PackWatchProvider packWatchFn_;
    mutable bool watchLoaded_ = false;
    mutable uint64_t watchTriedAt_ = 0;
    mutable std::vector<nvcoop::avatar::FormId> watch_;       // sorted, de-duplicated
    mutable uint32_t watchId_ = 0;
    std::map<PlayerId, fnvcoop::live::ActorSnapshot> lastSnap_;   // newest accepted snapshot per remote slot (place for the ownership service)
    std::map<PlayerId, fnvcoop::live::ActorSnapshot> reportedSnap_;
    mutable nvcoop::avatar::StreamRatePlanner rate_;   // tierOf() keeps the hysteresis state, so reading a tier in a const query is fine
    std::map<PlayerId, fnvcoop::live::ActorSnapshot> early_;   // the newest snapshot of a player the roster has not named yet (kept for his puppet, see onSnapshotDatagram)
    // parts=unified / stream=unified
    PlayerPort port_{*this};
    const IEntityDirectory* entities_ = nullptr;
    FormCodec formCodec_;
    struct RemoteEquip { std::vector<fnvcoop::FormRef> forms; bool haveForms = false; nvcoop::actors::WeaponState weapon; bool haveWeapon = false; };
    std::map<PlayerId, RemoteEquip> remoteEquip_;              // the Equipment and WeaponState parts of each remote player, combined into the corrector's list
    std::deque<fnvcoop::live::ActorSnapshot> samples_;         // stream=unified: local snapshots waiting for ActorSync
    std::map<PlayerId, std::vector<uint32_t>> queuedTicks_;   // ticks of the local snapshots waiting in the sender for the next flush (their age is the Send stage)
    std::map<PlayerId, uint64_t> rx_;                  // accepted snapshots per remote slot (statusJson: the in-game proof turns the difference into a rate)
    std::map<PlayerId, nvcoop::avatar::Appearance> applied_;   // the look each puppet was last built / updated with
    nvcoop::avatar::Appearance localApp_;                       // the cached local look
    bool haveLocalApp_ = false;
    uint64_t lastAppRead_ = 0;
    bool forceAppRead_ = false;                                 // noteLookEdited(): read the game at the next chance
    uint64_t settleNowUntil_ = 0;                               // ... and offer a changed look without the settle time until then
    nvcoop::avatar::Appearance lookCand_, lookOffered_;         // the newest look read, and the one the Look part offers (readLocalPart)
    bool haveLookCand_ = false, haveLookOffered_ = false;
    uint64_t lookCandSince_ = 0;
    fnvcoop::live::SnapshotSender sender_;
    fnvcoop::live::SnapshotReceiver recv_;
    std::vector<fnvcoop::live::ActorSnapshot> localOut_;   // snapshots the side produced during this tick
    bool haveEquip_ = false;
    std::vector<nvcoop::avatar::EquipEntry> equip_;        // the local player's worn items as last reported by the side
    bool lookSentValid_ = false;
    AvLook lastLookSent_;
    uint64_t lastAck_ = 0, lastLookCheck_ = 0, lastPoseReport_ = 0;
    fnvcoop::live::ActorSnapshot lastLocal_, reportedLocal_;   // the local player's newest published place, and the one the pose sink last heard
    bool haveLocal_ = false, reportedLocalValid_ = false;
    AvatarNetStats stats_;
};

}  // namespace nvcoop::livewire
