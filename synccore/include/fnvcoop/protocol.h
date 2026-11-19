// Session protocol messages and the envelope: [0xFC 0x4E ver type][string table][body][crc32 LE].
#pragma once
#include <variant>
#include "fnvcoop/baseline.h"
#include "fnvcoop/policy.h"
#include "fnvcoop/shadow.h"
#include "fnvcoop/transport.h"
#include "fnvcoop/wire.h"

namespace fnvcoop {

// What a build must agree on to play together. Zero / empty fields mean "not set" and are not checked by the host.
struct HandshakeInfo {
    uint16_t protocol = kProtocolVersion;
    uint32_t buildNumber = 0;          // monotonically increasing fnv-coop build (newer > older), shown in messages
    uint64_t buildHash = 0;            // hash of the nvcoop binary / source revision
    uint64_t rulesHash = 0;            // RulesDb::textHash() of rules.json
    uint64_t volatileHash = 0;         // VolatileKeys::contentHash()
    std::string buildName;             // human readable, e.g. "fnv-coop 0.7.3"
    std::vector<std::string> plugins;  // load order (plugin file names), for the "mod list differs" message
};
// Reject reasons
enum RejectReason : uint8_t { kRejectFull = 1, kRejectContent = 2, kRejectNoBaseline = 3, kRejectProtocol = 4, kRejectBuild = 5,
                              kRejectRules = 6, kRejectVolatile = 7, kRejectModList = 8 };

// Client -> host. `token` is a stable per-install identity (maps to a slot); `session` is unique per connect.
struct Hello {
    uint32_t session = 0; uint64_t token = 0; uint64_t lastSeq = 0;
    uint8_t flags = 0;            // bit0: wantBaseline (my world comes from my own save; send me the shared world)
    uint64_t contentHash = 0;     // plugin load-order hash (must equal the host's when wantBaseline)
    uint32_t lastEpoch = 0;       // world epoch my state belongs to (0 = none)
    HandshakeInfo hs;             // build / rules / volatile / mod list identity
    // Appended after hs (absent from older builds = 0): hash of the client's shared shadow at lastSeq. The host resumes the cursor only if
    // its own state at lastSeq hashes the same; epoch numbers alone do not identify a timeline across host runs (a prepared-save load).
    uint64_t cursorCheck = 0;
};
constexpr uint8_t kHelloWantBaseline = 1;
// Host -> client. If hasSnapshot, the client replaces its state with `snapshot` (applying the diff to its world).
struct Welcome {
    uint32_t session = 0; uint8_t slot = 0; uint64_t headSeq = 0; SharePolicy policy;
    bool hasSnapshot = false; ShadowState snapshot;
    // Host's record of this client's committed intents (for the intent epoch it last used).
    uint32_t intentEpoch = 0, lastIntent = 0;
    uint32_t baselineId = 0;   // != 0: a world-baseline transfer follows for this join
    uint8_t baselineAbs = 0;   // bit0: karma synced by baseline, bit1: reputation synced by baseline
    uint32_t worldEpoch = 0;   // the host world epoch this session belongs to
    // Every reward (items / actor values) this slot received up to the snapshot, with its id. A baseline joiner applies only the ones it
    // has not delivered yet (its save sidecar remembers the ids); everyone marks them delivered once the snapshot is in.
    std::vector<RewardGrant> rewards;
};
struct Reject { uint32_t session = 0; uint8_t reason = 0; std::string text; };   // text: ready to show to the player
// worldEpoch 0 = unspecified (accepted); otherwise txns/acks from another epoch are stale and dropped.
struct TxnBatch { uint32_t session = 0; std::vector<EffectTxn> txns; uint32_t worldEpoch = 0; };
struct Ack { uint32_t session = 0; uint64_t upTo = 0; uint32_t worldEpoch = 0; };
// Host -> client: the host world was replaced (loaded an older save, or you hold a different epoch). Re-dump your world and
// rejoin with a baseline under a new session.
struct Rebase { uint32_t session = 0; uint32_t epoch = 0; };
// AppliedEffects: the client already applied `effects` locally (pickup etc.); the host records them.
// Others are REQUESTS: the guest did not execute anything; the host runs the real activation/trigger/pickup
// (HostSession::onAction) and commits whatever effects result.
enum class IntentKind : uint8_t { AppliedEffects = 0, ActivateRef = 1, TriggerEnter = 2, PickUp = 3,
                             ScenePass = 4,    // the player finished their per-player step of quest `ref` (chargen etc.)
                             ReportCell = 5,   // the player is now in cell `ref` (party tether / presence)
                             ReportNear = 6,   // the player is now within the tether radius of anchor ref `ref`
                             ReportFar = 7,    // the player left the tether radius of anchor ref `ref`
                             ReportKill = 8,   // the player's own attack killed actor `ref` on their machine: the host kills it too
                             DialogueResult = 9,   // the player's own conversation reached INFO `ref`'s result script (cause.source = the speaker,
                                                   // cause.aux = 0 begin / 1 end): the host runs it for them (a guest never runs one)
                             Count_ };
// Client-reported local action (at-most-once, in order, durable across reconnects).
struct Intent { uint32_t session = 0; uint32_t epoch = 0; uint32_t id = 0; Cause cause; std::vector<Effect> effects;
               IntentKind kind = IntentKind::AppliedEffects; FormRef ref; };
struct IntentAck { uint32_t session = 0; uint32_t epoch = 0; uint32_t id = 0; };

// Opaque application payload sharing the session channel (e.g. dialogue protocol messages from plugin/dialogue).
// Best effort: delivered at most once per datagram, no ordering/retransmit guarantee beyond the transport's.
constexpr size_t kMaxAppPayload = 64 * 1024;
struct AppData { uint32_t session = 0; uint16_t channel = 0; uint8_t from = 0; std::vector<uint8_t> payload; };

// ---- world baseline transfer (chunked, acked, resumable by chunk index) ----
struct BaselineSummaryChunk {    // client -> host
    uint32_t session = 0, id = 0, index = 0, total = 1;
    std::vector<std::pair<uint64_t, uint32_t>> entries;
    bool hasAbs = false; int64_t karma = 0; std::vector<RepAbs> rep;   // only in chunk 0
};
struct BaselineDiffChunk {       // host -> client
    uint32_t session = 0, id = 0, index = 0, total = 1;
    std::vector<Effect> effects;
    std::vector<uint64_t> revertKeys;
    uint32_t kept = 0;   // entries the client already had (identical), for status reporting
};
struct BaselineAck {             // phase 0: host acks summary chunks (next = first missing); phase 1: client acks diff chunks
    uint32_t session = 0, id = 0; uint8_t phase = 0; uint32_t next = 0;
};
constexpr size_t kMaxRewardsPerWelcome = 1u << 16;
constexpr size_t kBaselineSummaryPerChunk = 256, kBaselineEffectsPerChunk = 128, kBaselineRevertsPerChunk = 512;

// ---- runtime desync detection / self-heal ----
// Client -> host: hash per shard of the client's REAL world (engine dump) at its applied seq.
struct SyncDigest { uint32_t session = 0, epoch = 0; uint64_t seq = 0; uint32_t digestId = 0; std::vector<uint32_t> shards; };
// Host -> client: send me the entries of these shards (bit i = shard i, 128 bits).
struct ShardRequest { uint32_t session = 0, digestId = 0; uint64_t maskLo = 0, maskHi = 0; };
// Client -> host: the digested world's entries (as effects) for the requested shards.
struct ShardEntries { uint32_t session = 0, digestId = 0; bool truncated = false; std::vector<Effect> entries; };
// Host -> client: force-set repair. Applied with baseline (force) semantics; keys touched since the digest are skipped.
struct SyncRepair { uint32_t session = 0, epoch = 0, digestId = 0; std::vector<Effect> effects; std::vector<uint64_t> revertKeys; };
// Host -> client: please send a digest as soon as you are quiescent (after a barrier / scene).
struct DigestRequest { uint32_t session = 0, epoch = 0; };
constexpr size_t kMaxShardEntries = 1000;

using Message = std::variant<Hello, Welcome, Reject, TxnBatch, Ack, Intent, IntentAck, AppData, BaselineSummaryChunk,
                             BaselineDiffChunk, BaselineAck, Rebase, SyncDigest, ShardRequest, ShardEntries, SyncRepair, DigestRequest>;

bool isHandshakeType(uint8_t type);   // Hello / Reject: version-independent envelope, trailing fields tolerated

enum class DecodeStatus { Ok, TooShort, BadMagic, BadVersion, BadCrc, Malformed, Trailing };
const char* toString(DecodeStatus s);

Bytes encodeMessage(const Message& m);  // empty on failure (e.g. non-finite float, oversize)
// verifyCrc=false is for fuzzing the parser behind the checksum.
DecodeStatus decodeMessage(const uint8_t* data, size_t size, Message& out, bool verifyCrc = true);
inline DecodeStatus decodeMessage(const Bytes& b, Message& out) { return decodeMessage(b.data(), b.size(), out); }

}  // namespace fnvcoop
