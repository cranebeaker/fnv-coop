# fnv-coop wire protocol v6

Status: matches `synccore/` (kWireVersion = 6, kProtocolVersion = 6, host sidecar version 4, client sidecar version 3; v6 added the RefCommand effect (tag 49, section 21) and its section at the end of the ShadowState encoding; v5 added `Welcome.rewards` (reward ids), IntentKind 6/7 (ReportNear / ReportFar) and the exact-state host objective merge; v3 added FailQuest (tag 32); v4 added effect tags 33-48 (census round 3), the CompanionOwner `active`/`kind` fields and the round-3 sections at the end of the ShadowState encoding). Source of truth is the code:
`synccore/include/fnvcoop/{wire.h,protocol.h,effect.h,transport_util.h}`, `synccore/src/{wire.cpp,protocol.cpp,shadow.cpp}`.
Round-trip, fuzz and golden tests live in `synccore/tests/` (run `synccore\run_tests.bat`).

This document is for people writing another lane, a tool, or a compatible implementation. It describes bytes on the wire and the
rules the two session state machines (`HostSession`, `ClientSession`) follow. It does not describe game hooks.

## 1. Layers

```
game hooks (L5)  ->  effects  ->  HostSession / ClientSession (synccore)  ->  ITransport  ->  UDP (ENet)
```

* A **transport** (`ITransport`) moves opaque byte datagrams between peers. Reliable ordered by default; ENet channel 0 is
  reliable-ordered, channel 1 is unreliable (only used when a caller asks for it explicitly). Datagrams are limited to
  `kMaxMessageBytes` = 8 MiB; real traffic stays far below the MTU-sized fragments ENet handles itself.
* The **session layer** is reliable on top of any lossy transport: it retransmits, de-duplicates and re-orders on its own, so
  the protocol also works over an unreliable channel (the simulator uses 0.5% loss, 0.2% duplication, corruption, reordering).
* Topology is a star: one host, up to `maxPlayers` (<= 8, `kMaxSlots`) clients. Slot 0 is the host player (`kHostPlayer`).

## 2. Channel registry (first byte)

`MuxTransport` (`transport_util.h`) dispatches on the first byte of every datagram so several protocols share one connection:

| First byte | Channel | Owner |
|---|---|---|
| `0xFC` | story / session envelope (everything in this document) | L4 synccore |
| `0xD1` | dialogue adapter | L8 `plugin/dialogue` |
| `0xE5` | ending slideshow (SwapTexture slides host -> guests, End carries the slide total) | L11 `plugin/hooks_ui/slideshow.h` |
| `0xAD` | player avatars: live snapshots (kind 1/2), Roster 0x40, Look 0x41 (v1, still decoded), Resync 0x42, **Look v2 0x43** (`plugin/coop/live/avatar_wire.h`, "Avatar channel 0xAD" below) | avatars `plugin/coop/live/avatar_*`, dll-wiring `live_*` |
| `0xA5` | spawn-site arbitration: client rolls (MRoll), verdicts, site records, site ownership, table digest / resync; always reliable (`synccore/spawn/spawn_wire.h`, docs/tools/spawn-arb.md) | spawn-arb `synccore/spawn` + `plugin/spawn` |
| `0xCB` | combat: hits, hit NACKs, vitals, death declarations, projectile / explosion declarations, crime reports, hostility effects (section 14) | combat-hooks `plugin/combat/combat_wire.h` |
| `0x7E` | world time: clock beacons, wait / sleep / fast-travel requests and verdicts, combat status (section 15) | world-time `plugin/world_time/wt_wire.h` |
| `0xDA` | area desync digests: per-area digests, targeted item diff, authoritative heal pushes (section 17) | area-desync `synccore/areadigest/ad_wire.h` |
| `0x5C` | social: companion roster / commands / state / inventory, crime reports, detections, wanted table, alarms, hostility, kill reports, social settings, digests (section 18) | companions-crime `synccore/social/wire.h`, `plugin/companions` |
| `0x8D` | guest intents: Intent (guest -> host) and Verdict (host -> guest) for activations, pickups, triggers, container locks, terminal entries (section 19) | guest-intents `plugin/intents/intent_wire.h` |
| `0xB7` | real-time VATS: host settings, a player's VATS mode, queued VATS actions, the party slow-mo grant (section 20) | vats `plugin/vats/vats_wire.h` |
| `0xB1` | loot: container / corpse open, take, put, close, register; dropped items, pickup claims, physics clutter (section 22) | loot `synccore/loot/loot_wire.h`, `plugin/loot` |

New channels must pick an unused first byte that is not 0xFC and is not 0xA1 (see 3.2), and be registered here and in
`transport_util.h`. Datagrams with an unknown first byte are dropped by the mux. `AppData` (type 8) is an alternative for small
payloads that should ride the story channel; it is best effort.

## 3. Envelope

### 3.1 Normal messages

```
offset 0  0xFC          magic0
       1  0x4E          magic1 ('N')
       2  kWireVersion  (2)
       3  type          message type id, 1..17 (section 5)
       4  [string table][body]
 end-4    crc32 LE      CRC-32 (IEEE, reflected, init/final 0xFFFFFFFF) over bytes 0 .. end-5
```

Minimum size is 9 bytes. Decode failures, in check order: `TooShort`, `Malformed` (> 8 MiB), `BadMagic`, `BadVersion`, `BadCrc`,
`Malformed` (body invalid / unknown type), `Trailing` (bytes left over after the body). A receiver silently drops anything that
does not decode `Ok`; the sender's retransmission covers the loss.

### 3.2 Handshake messages (Hello = 1, Reject = 3)

Identical layout, but byte 2 is the version-independent value `kHandshakeEnvelope` = 0xA1 instead of `kWireVersion`. This lets two
incompatible builds still exchange Hello/Reject and explain why they cannot play together. For these two types:

* the body must never change in an incompatible way: only append new fields at the end;
* decoders must tolerate trailing bytes (a newer build may append fields), so `Trailing` is not an error for them;
* the first fields (`session`, and `reason` for Reject) stay in place forever.

### 3.3 Primitive encodings (class `Archive`, one symmetric reader/writer)

| Name | Encoding |
|---|---|
| `u8/u16/u32/u64` | fixed width, little endian |
| `varu`, `varu32` | LEB128 unsigned varint (max 10 bytes for 64 bit; 32-bit variants reject values > 2^32-1) |
| `vari`, `vari32` | zigzag then LEB128 |
| `f32` | IEEE-754 binary32 LE bits; **non-finite values are an encode and decode error** |
| `f64` | binary64 LE bits; same finite rule |
| `boolean` | one byte, 0 or 1 (anything else is malformed) |
| `str` | varint length, raw bytes (no terminator); cap 255 unless stated |
| `bytes` | varint length, raw bytes; cap stated per field |
| `count` | varint element count; rejected if above the field's cap **or above the bytes remaining** (every element is at least 1 byte) |

After any failure the archive is poisoned: reads return zeros, writes are no-ops, and the message fails as a whole. There is no
partial decode.

### 3.4 String table

`[string table][body]`: the table is `varu n` followed by `n` strings (`str`). Inside the body, a `FormRef`'s plugin name is a varint
index into this table. Writers intern each distinct plugin name once per message (max 4096 entries, names <= 255 bytes); readers
reject out-of-range indexes. This is why `FormRef`s are cheap: `Plugin.esm` is sent once per message, then 1 byte each.

## 4. Shared structures

All integers below are the varint/fixed forms of 3.3 in the order listed.

* **FormRef**: `pluginRef` (string table index), `varu32 localId`. Never a raw load-order FormID. Plugin names should be
  lower-cased (`FormRef::make`); comparison is byte-exact.
* **Recipient**: `u8 mode` (0 Default, 1 Origin, 2 All, 3 Mask), then `u8 mask` only when mode == 3 (bit i = slot i).
* **Cause**: `u8 kind` (0 Unknown, 1 QuestResult, 2 DialogueResult, 3 Console, 4 Loot, 5 Combat, 6 Script, 7 Intent; >= 8 rejected),
  `FormRef source`, `vari32 aux`.
* **SharePolicy**: `u8 bits` (bit0 journal, 1 globals, 2 kills, 3 karma, 4 reputation, 5 questRewardsToAll, 6 clientLoad=Block;
  bit7 must be 0), `u8 maxPlayers` (1..8).
* **EffectTxn**: `varu seq`, `u8 origin` (< 8), `u8 flags` (bit0 = origin already applied locally), `varu32 originEpoch`,
  `varu32 originIntent`, `Cause`, `count effects (<= 1024)`, then each `Effect`.
* **Effect**: `u8 tag` then the fields of that alternative. The tag is the index in the `Effect` variant (section 6). An unknown
  tag fails the whole message, so **adding or reordering effects is an incompatible change** (section 8).
* **ShadowState** (Welcome snapshot): `varu seq`, then the sections stages, objectives, globals, enabled, kills, questRunning, ...
  each as `count (<= kMaxEntries)` + entries (see `io(Archive&, ShadowState&)` in `shadow.cpp`; the order there is normative).

## 5. Messages

Type id = (index in `Message` variant) + 1. Direction: C = client to host, H = host to client. `session` is a per-connect id chosen
by the client in Hello and echoed in every later message so stale datagrams from an old connection are ignored.

| Id | Name | Dir | Body (in order) |
|---|---|---|---|
| 1 | Hello | C | `varu32 session`, `u64 token` (stable per-install identity -> slot), `varu lastSeq`, `u8 flags` (bit0 wantBaseline), `u64 contentHash`, `varu32 lastEpoch`, HandshakeInfo, `u64 cursorCheck` (appended; absent from older builds = 0: hash of the client's shared shadow at `lastSeq`, sent when resuming, see section 11) |
| 2 | Welcome | H | `varu32 session`, `u8 slot` (< 8), `varu headSeq`, SharePolicy, `boolean hasSnapshot` [ShadowState], `varu32 intentEpoch`, `varu32 lastIntent`, `varu32 baselineId` (!= 0: baseline transfer follows), `u8 baselineAbs` (bit0 karma, bit1 reputation synced by baseline), `varu32 worldEpoch`, `count rewards` then per reward `varu id`, Effect (v5: every item / actor-value reward this slot received this epoch, see section 13) |
| 3 | Reject | H | `varu32 session`, `u8 reason`, `str text` (player-facing, <= 255) |
| 4 | TxnBatch | H | `varu32 session`, `varu32 worldEpoch` (0 = unspecified), `count txns (<= 256)`, EffectTxn each |
| 5 | Ack | C | `varu32 session`, `varu upTo` (highest contiguous seq applied), `varu32 worldEpoch` |
| 6 | Intent | C | `varu32 session`, `varu32 epoch`, `varu32 id`, `u8 kind`, Cause, `count effects (<= 1024)` + Effects, FormRef `ref` only if kind != AppliedEffects |
| 7 | IntentAck | H | `varu32 session`, `varu32 epoch`, `varu32 id` |
| 8 | AppData | both | `varu32 session`, `varu32 channel` (<= 0xFFFF), `u8 from` (slot), `bytes payload` (<= 64 KiB) |
| 9 | BaselineSummaryChunk | C | `varu32 session,id,index,total` (total >= 1, index < total), `count entries (<= 4096)` of (`u64 key`, `u32 hash`), `boolean hasAbs`, `vari karma`, `count rep (<= 4096)` of (FormRef faction, `vari fame`, `vari infamy`). Abs fields are only meaningful in chunk 0 but always present |
| 10 | BaselineDiffChunk | H | `varu32 session,id,index,total`, `count effects (<= 1024)` + Effects, `count revertKeys (<= 4096)` of `u64`, `varu32 kept` |
| 11 | BaselineAck | both | `varu32 session`, `varu32 id`, `u8 phase` (0 host acks summary chunks, 1 client acks diff chunks; > 1 rejected), `varu32 next` (first missing chunk index) |
| 12 | Rebase | H | `varu32 session`, `varu32 epoch` |
| 13 | SyncDigest | C | `varu32 session`, `varu32 epoch`, `varu seq`, `varu32 digestId`, `count shards` (must equal 128) of `u32` |
| 14 | ShardRequest | H | `varu32 session`, `varu32 digestId`, `u64 maskLo`, `u64 maskHi` (bit i = shard i) |
| 15 | ShardEntries | C | `varu32 session`, `varu32 digestId`, `boolean truncated`, `count entries (<= 1000)` + Effects |
| 16 | SyncRepair | H | `varu32 session`, `varu32 epoch`, `varu32 digestId`, `count effects (<= 1024)` + Effects, `count revertKeys (<= 4096)` of `u64` |
| 17 | DigestRequest | H | `varu32 session`, `varu32 epoch` |

Hello's HandshakeInfo: `varu32 protocol` (<= 0xFFFF), `varu32 buildNumber`, `u64 buildHash`, `u64 rulesHash`, `u64 volatileHash`,
`str buildName` (<= 64), `count plugins (<= 1024)` of `str` (<= 255). Zero/empty fields mean "not set" and are not checked.

IntentKind: 0 AppliedEffects (client already applied; host records), 1 ActivateRef, 2 TriggerEnter, 3 PickUp, 4 ScenePass,
5 ReportCell, 6 ReportNear, 7 ReportFar. Kinds 1..7 are **requests**: the guest did not execute anything, the host runs the real action and commits the
resulting effects. Intent `ref` is the target form (or quest for ScenePass, cell for ReportCell, tether anchor ref for ReportNear / ReportFar).

Reject reasons: 1 full, 2 content (plugin hash differs while wantBaseline), 3 no baseline, 4 protocol, 5 build, 6 rules,
7 volatile list, 8 mod list. `text` is always filled so a client that does not know a reason id can still show it.

Limits summary: `kMaxEffectsPerTxn` 1024, `kMaxTxnsPerBatch` 256, `kMaxStringLen` 255, `kMaxMessageBytes` 8 MiB, `kMaxAppPayload`
64 KiB, `kMaxShardEntries` 1000, `kSyncShards` 128, baseline chunk sizes `kBaselineSummaryPerChunk` 256 /
`kBaselineEffectsPerChunk` 128 / `kBaselineRevertsPerChunk` 512 (senders); table 4096 strings.
Receivers enforce the caps above, which are looser than the senders' chunk sizes, so chunk sizes can be tuned without a version bump.

## 6. Effect tags

Tag = variant index. Field list after the tag (types per 3.3, FormRef per section 4, `to` = Recipient):

<!-- BEGIN EFFECT TABLE (generated by synccore/tools/gen_effect_table.py from effect.h + wire.cpp; do not edit by hand) -->
| Tag | Effect | Fields |
|---|---|---|
| 0 | SetStage | quest, varu32 stage (merge = max) |
| 1 | SetObjective | quest, varu32 objective, u8 flags (flags: bit0 displayed, bit1 completed. A host-origin write is the exact flag state (a hide survives resync and late join); a guest-origin write only adds bits) |
| 2 | SetGlobal | global, f32 value |
| 3 | EnableRef | ref |
| 4 | DisableRef | ref |
| 5 | AddItem | item, vari32 count, to |
| 6 | RemoveItem | item, vari32 count, to |
| 7 | ModActorValue | varu32 av, vari32 delta, to (av 0x10000 = experience) |
| 8 | ModReputation | faction, vari32 fame, vari32 infamy, to |
| 9 | ModKarma | vari32 delta, to |
| 10 | StartQuest | quest |
| 11 | StopQuest | quest |
| 12 | MoveRef | ref, cell, f32 x, f32 y, f32 z, boolean partyWarp |
| 13 | KillActor | actor |
| 14 | ResurrectActor | actor |
| 15 | SetScriptVar | owner, varu32 index, f64 value (index <= 0xFFFF) |
| 16 | SetPlayerControls | varu32 controls, boolean enable, to |
| 17 | PartyTether | cell, boolean on |
| 18 | SlideshowEnd | (none) |
| 19 | SaveSuppress | boolean on |
| 20 | CompanionOwner | npc, u8 owner, boolean active, u8 kind (owner < 8; active false = dismiss; kind 0 humanoid, 1 critter) |
| 21 | ForcedDialogue | npc, topic, u8 initiator (event, not state; initiator < 8) |
| 22 | BarrierArrive | barrier, u8 required (required = slot mask) |
| 23 | BarrierRelease | barrier |
| 24 | SayOnceFlag | info |
| 25 | CompleteQuest | quest |
| 26 | CompleteAllObjectives | quest |
| 27 | SetLock | ref, boolean locked, vari32 level, boolean hasBefore, boolean beforeLocked, vari32 beforeLevel |
| 28 | SetFactionRank | actor, faction, vari32 rank (rank < 0 = removed) |
| 29 | SetPerk | perk, vari32 rank, to (rank 0 = removed) |
| 30 | SwapTexture | ref, texture, u8 slot |
| 31 | SetFastTravelLock | scope, boolean on |
| 32 | FailQuest | quest (sticky; implies completed (shadow state sets both questFailed and questCompleted)) |
| 33 | SpawnRef | varu32 id, base, cell, f32 x, f32 y, f32 z, boolean leveled (id is 0xFF-prefixed and host allocated (PlaceAtMe / PlaceLeveledActorAtMe)) |
| 34 | DespawnRef | varu32 id, boolean forget (id is 0xFF-prefixed; tombstone, a later SpawnRef of the same id is ignored) |
| 35 | ObjectState | ref, u8 mask, boolean open, boolean locked, vari32 level, boolean destroyed, boolean activatedOnce, boolean hasBefore, boolean beforeLocked, vari32 beforeLevel (mask <= 15: bit0 open, bit1 lock, bit2 destroyed, bit3 activate-once) |
| 36 | RefScriptVar | ref, varu32 index, f64 value, boolean hasBefore, f64 before (index <= 0xFFFF; object / NPC script variable) |
| 37 | RefAddItem | ref, item, vari32 count (inventory of a non-player ref) |
| 38 | RefRemoveItem | ref, item, vari32 count |
| 39 | RemoveAllItems | ref, boolean undo, boolean hasBefore, vari32 c, vari32 cc |
| 40 | AddTopic | topic (journal, shared) |
| 41 | AddNote | note (shared) |
| 42 | RemoveNote | note |
| 43 | SetMapMarker | marker, u8 flags, boolean hasBefore, u8 before (flags bit0 visible, bit1 fast travel allowed; <= 3) |
| 44 | ForceActiveQuest | quest, to (per player UI focus; Default = the initiator) |
| 45 | RequestSave | u8 kind (kind 0 autosave, 1 systemsave, 2 forcesave; EVENT) |
| 46 | SetWeather | weather, boolean force, boolean hasBefore, before, boolean beforeForce (force = ForceWeather) |
| 47 | SetTimeScale | f32 scale, boolean hasBefore, f32 before (SetGlobalTimeMultiplier, 0 = never set) |
| 48 | AdvanceTime | varu32 minutes (sleep / wait; EVENT) |
| 49 | RefAngle | ref, u8 axis, f32 degrees, boolean hasBefore, f32 before |
| 50 | RefCommand | varu32 op, ref, a, b, vari32 i, vari32 j, vari32 k, vari32 l, f32 f, str s, to, c, d, vari32 m |
<!-- END EFFECT TABLE -->

Integer deltas on purpose (XP, caps, karma, rep): replicas stay bit-identical. Floats appear only for globals, positions and
script variables, and must be finite. `ForcedDialogue` is an event (not state); replays carry `catchUp` so sinks can tell.

## 7. Session behaviour

### 7.1 Ordering and delivery

* The host assigns a dense, monotonically increasing `seq` to each committed EffectTxn (`StoryLog`). Clients apply strictly in
  seq order, buffer gaps, ignore duplicates, and send `Ack{upTo}` (cumulative).
* The host retransmits unacked txns with exponential backoff, independent of live commits (a live send never refreshes the
  retransmit timer; otherwise steady commits starve retransmits). A client that is far behind (> 32 txns) gets no live sends,
  only ack-clocked catch-up: each Ack that advances immediately triggers the next batch.
* Client **intents** are durable and exactly-once: numbered `id` within an `epoch`; the host executes each id once, in order,
  acknowledges with `IntentAck`, and the client resends unacked intents on a timer and after reconnect (`Welcome.lastIntent`
  tells it where the host stopped). The `epoch` changes when the client's local world is replaced (load/rejoin) so old ids are
  not confused with new ones.
* `worldEpoch` identifies a host world lineage. It changes when the host loads an older save. Txns/acks from another epoch are
  stale and dropped.

### 7.2 Join

1. Client sends Hello (new `session`). Host validates: protocol, build, rules hash, volatile hash, mod list, free slot, and
   plugin `contentHash` when wantBaseline. Failure -> Reject (reason + text), nothing else is sent.
2. Host sends Welcome. `hasSnapshot` = the client must replace its shadow with the snapshot (fresh join / far behind /
   log compacted past `lastSeq`); otherwise the client simply resumes from `lastSeq` and receives TxnBatch catch-up.
3. If `baselineId != 0` (the client's world came from its own save): summary phase, then diff phase (7.3).

### 7.3 World baseline transfer (join with wantBaseline)

* Phase 0: client sends `BaselineSummaryChunk` (index/total) listing `(key, valueHash)` for every tracked world entry; the host
  replies `BaselineAck{phase 0, next}`; the client resends from `next`.
* Phase 1: host computes the diff versus its authoritative state and streams `BaselineDiffChunk`s (effects to force-set plus
  `revertKeys` the client has but the host does not); the client applies them with force semantics and replies
  `BaselineAck{phase 1, next}`.
* Hashes canonicalize script-variable and global values to float32 then round to 1e-4 so benign float noise between machines
  does not produce spurious diffs. Both ends must use the same canonicalization (it is part of the protocol).
* Chunks are idempotent by index; a restarted client uses a new `id`.

### 7.4 Rebase

When the host loads an older save (or a client reconnects holding a different epoch) it sends `Rebase{epoch}`. The client
re-dumps its world and rejoins with a baseline under a new session. Sticky one-way flags (e.g. CompleteQuest) cannot be un-set
by a rewind; this is a documented limitation.

### 7.5 Desync detection and repair

The client periodically (and on `DigestRequest`, after barriers/scenes) sends `SyncDigest`: 128 shard hashes (8 entry classes x 16
key buckets) over its REAL engine world at an applied `seq`. The host compares against the same seq; for mismatching shards it
sends `ShardRequest` (128-bit mask), the client answers `ShardEntries` (<= 1000 per message, `truncated` set if it hit the cap),
and the host sends `SyncRepair`: force-set effects and revert keys, skipping keys touched since the digest. Repairs are only
applied if the `epoch` matches. A revert key the client has no inverse for is left as it is and counted
(`ClientSession::SyncStats::repairRevertsUnrevertable`, `coop.status` client `repairUnrevertable`).

Client-only quest running keys (`StartQuest` / `StopQuest` entries the host's state has no entry for, issue #25) are not sent as
revert keys: a quest's running flag has no inverse. The host settles them itself by committing its OWN running state for those quests as
a normal host txn (`HostSession::hostQuestRunning` reads the host engine; without it the completeness default, not running, is used).
Every client receives it, the host's key space is complete afterwards, and the digest converges. Counted in `HostStats::questRunningAdopted`
(`desyncs.digests.questRunningAdopted`) and in the repair counters.

Client-only quest state that is real save state (a non-zero `SetStage`, `CompleteQuest`, `FailQuest`) is the same shape: both copies loaded
it from the same save, but a host without a world baseline never recorded it, and neither stage merge (max) nor completion has an inverse.
The host adopts such a key only after its OWN engine confirms it (`HostSession::hostQuestState`: engine stage >= the client's stage, or
equal completed / failed flag), by committing the client's value as a normal host txn (`questStateAdopted`). When the engine disagrees the key
is a real divergence: it is not adopted, not resent as a revert, and stays in the desync report (`questStateDisagreed`, counted per round).
Without the hook (or when it fails) nothing is adopted and the old revert keys are sent.

Client-only objectives follow the same rules. `SetObjective` with flags 0 (not displayed, not completed: the objective analogue of stage 0)
is adopted outright, the host commits the same flags 0 (`objectiveZeroAdopted`). `SetObjective` with non-zero flags and
`CompleteAllObjectives` are adopted only after the host engine confirms them (`HostSession::hostObjectives` returns the status bits of every
objective of the quest: the objective's flags must equal the client's, or every objective of the quest must be completed;
`objectiveStateAdopted`). A disagreement is a real divergence: not adopted, still reported (`objectiveStateDisagreed`).

Client-only script vars (`SetScriptVar`, quest vars and the vars of watched refs) are the same shape. A baseline-less host primes its engine poll
with the values already there and drops them, so its shared world only holds vars that changed since the session started, while the client's
digest lists every var of every quest (the completeness rule). `SetScriptVar` has no inverse. A client-only var with value 0 is the untouched
default, the meaning of an absent key: the host commits the same 0 as a normal host txn (`scriptVarAdopted`), unless its engine
(`HostSession::hostScriptVars`) holds another value for it (`scriptVarDisagreed`: a real divergence, not adopted, not resent, still reported).
A non-zero value is adopted only when the host's own engine reads the same value; with no reader, a failing one, or an owner it cannot read,
nothing is adopted. In the 2026-10-26 two-PC benchmark these keys went out as 256 revert keys per digest round that the client skipped
(5.8 digest mismatches and 1180 repair effects per minute, idle included, the digest never ok).

The repair loop never sends a revert key for a client-only entry of a kind the client can never revert (`mayHaveRevert` in baseline.cpp lists
the kinds `revertEffectFor` handles): it is counted per round (`revertsSkippedNoInverse`, `desyncs.digests`) and stays in the report.
Twins: `scriptVarReverted`, `noInverseResent`.

Second two-PC benchmark (2026-10-27, after the script var fix: 5.9 mismatches and 744 repair effects per minute, digest never ok). What was left:

- **The host never loaded the volatile list.** Only a world dump loads `nvcoop_volatile_keys.txt` and resolves its variable names, and a baseline-less host never dumps. Its poll
  therefore committed per-machine vars (VNPCFollowers bFixedEDE / bWeaponCheck, the random-encounter quests' iSpawnMonsters, ...) that the client's dump leaves out: "differs: host has
  it, client (missing)", every round. The host now calls `snapshot::PrepareVolatileKeys` when its state poll starts (retried until every name resolves).
- **A baseline-less host committed "not running" for quests the client was running.** `hostQuestRunning` was only wired with a world baseline, so the completeness default won and the
  client applied `StopQuest` to ~190 running quests (their scripts stopped). The plugin now reads the running flag (`TESQuest::flags & 1`) per reported quest
  (`Runtime::setQuestRunningReader`). A quest the engine cannot read is left alone; only a host with no hook at all (game-free sessions) still commits the default.
- **Client-only SetGlobal** is settled like script vars: adopted where `hostGlobals` (`TESGlobal::data`) shows the same value, never sent as a revert key.
- **Settling is not repair.** The client's dump lists every quest, objective and var; the host's world only what changed. ~5000 default-valued keys (not running, stage 0, flags 0, var 0) and
  ~400 engine-confirmed ones were committed once. They are counted as `effectsSettled` / `settleRounds`, no longer in `effectsRepaired` (which is what the benchmark's `deff` reads).
- **Settling is fast.** The bad shards are asked round-robin (`askCursor`), up to `maxShardsPerRound` (32) and the reply cap, and a round that settled keys with bad shards left asks for the
  next digest at once (`settleFollowUps`) instead of in 10 s. The benchmark needed 25 rounds because the same stuck quest shards were asked first every time.
- **Known divergences are acknowledged once.** A client-only key the host engine contradicts (the 8 + 1 completed quests: `questStateDisagreed`), cannot read for `kUnreadRounds` rounds, or that has
  no inverse is listed in the report with `acked: true` and remembered per client. From then on the host sums the client's term for that key into its shard hash (`ShardAdjust`), so the digest
  agrees instead of mismatching every round, and nothing is sent. The same goes for a key repaired `kMaxRepairAttempts` (3) times with the same two values and still different (`repairGaveUp`):
  no key is resent for ever. A change of either side's value withdraws the acknowledgement (`ackDropped`) and the key is looked at afresh; `divergenceAcked` counts them.
  Twins: `ackIgnored`, `askLowestFirst`, `noSettleFollowUp`, `settleCountsAsRepair`, `repairForever`, `runningBlindStop`, `globalReverted`.

### 7.6 AppData

Opaque, best effort, for adapters that want to ride the story channel (dialogue uses its own mux channel instead). `from` is the
sender slot as claimed by the sender; the host overwrites it for relayed traffic.

## 8. Versioning rules

1. `kWireVersion` is in every normal envelope; a receiver drops other versions (`BadVersion`). Bump it on **any** incompatible
   byte-level change: new/reordered/removed Effect tags or Message ids, changed field order or types, changed caps that a
   conforming sender could exceed, changed CRC/string-table rules.
2. `kProtocolVersion` is in Hello's HandshakeInfo and is checked by the host (Reject 4 with a readable text). Bump it together with
   kWireVersion, and also when behaviour changes without bytes changing (merge rules, baseline canonicalization, retransmit
   semantics a peer relies on).
3. **Hello and Reject must stay decodable by every past and future build**: version byte 0xA1, fields only appended, receivers
   ignore trailing bytes, type ids 1 and 3 never reused.
4. To add an effect type without breaking old builds there is no negotiation: append at the end of the variant (new tag), bump both
   versions. Old builds then Reject the new build with a clear message instead of misparsing.
5. Hash inputs that gate play (rules.json text hash, volatile-key list content hash, plugin list) are compared at join; changing
   `rules.json` therefore forces everyone to the same copy by design (Reject 6/7/8).
6. Every change must come with: updated tests (round trip + fuzz corpus), a mutation twin where applicable, and an update to this
   file. Receivers must treat all input as hostile: bounds-check counts, reject non-finite floats, never trust `slot`/`from`.

## 9. Test and tooling pointers

* `synccore\run_tests.bat [x64]` builds and runs ~45 ctest entries (wire round trip, truncation/bit-flip fuzz, session scenarios
  under loss/dup/corruption, mutation twins, ENet loopback).
* `build\L4\fnvcoop_soak.exe --seconds 3600 --players 4 --seed N [--threaded]` runs the Goodsprings traffic-shape soak
  (see `internal lane notes`).
* Diagnostics: `HostSession::explainJoinEntry`, `ClientSession::explainSummaryEntry` name the world entry behind a baseline or
  digest mismatch.

## 10. Dialogue channel (first byte 0xD1)

Owner: L8 (`plugin/dialogue/include/fnvcoop_dlg/`, header-only); L4 supplies the transport, the mux (section 2) and the session
rules below. Dialogue packets ride the same connection as the story envelope and are demultiplexed on the first byte.

**Delivery requirement:** reliable and ordered per peer pair (ENet channel 0). Duplicates are tolerated (the host is idempotent per
`(conv, step)`); loss and reordering are not. This is why dialogue does not use the story session's own retransmit machinery.

### 10.1 Envelope

```
0xD1  ver(=1)  type  body  crc32 LE (over everything before it)
```

Different from the story envelope: no magic1, no string table (strings and FormRef plugin names are inline), the version byte is the
dialogue protocol version (`kVersion` in `messages.h`, currently 1), and a decoder rejects trailing bytes. Varints and zigzag are as in
3.3. Limits: string <= 1024 bytes, lines <= 32, topics <= 64, facts <= 128. `FormRef` = `str plugin`, `varu32 localId`.

### 10.2 Messages (type id = index + 1)

| Id | Name | Dir | Body |
|---|---|---|---|
| 1 | StartReq | C->H | `varu reqId`, FormRef npc |
| 2 | Offer | both | `varu conv`, `varu step`, `count topics` of (FormRef topic, `u8 check`, `u8 flags`). C->H: the initiator's list. H->C: the relayed copy for spectators |
| 3 | Pick | C->H | `varu conv`, `varu step`, FormRef topic, FormRef info, `vari choice`, `u8 goodbye`, `count lines` of (`str text`, `str voice`, `u8 emotion`, `u8 emotionValue`), `count facts` of (`u8 kind`, FormRef what, `vari value`) |
| 4 | LineDone | C->H | `varu conv`, `varu step` |
| 5 | EndReq | C->H | `varu conv`, `u8 reason` (CloseReason 1..8) |
| 6 | Heartbeat | C->H | `varu conv` |
| 7 | StartDenied | H->C | `varu reqId`, `u8 reason` (1 Busy, 2 AlreadyTalking, 3 NpcDead, 4 NpcInCombat, 5 Unknown, 6 TooMany), `u8 owner` slot or 0xFF |
| 8 | Opened | H->C | `varu conv`, `varu reqId`, FormRef npc, `u8 initiator` |
| 9 | Lines | H->C | `varu conv`, `varu step`, FormRef topic, FormRef info, `u8 goodbye`, `count lines` as in Pick |
| 10 | Result | H->C | `varu conv`, `varu step`, `u8 phase` (0 Begin, 1 End), `varu txnSeq` |
| 11 | Control | H->C | `varu conv`, `u8 lockMask`, `u8 locked` |
| 12 | Closed | H->C | `varu conv`, `u8 reason` (1 Goodbye, 2 InitiatorLeft, 3 Disconnect, 4 NpcDead, 5 Combat, 6 Timeout, 7 HostShutdown, 8 ProtocolError) |

`conv` ids are assigned by the host (never 0). `step` counts the initiator's picks within a conversation, from 1.

### 10.3 Rules

* **Initiator lock.** The host grants at most one conversation per NPC and per player. A second StartReq gets StartDenied (Busy with the
  holder's slot, or AlreadyTalking). A retransmitted StartReq (same `reqId`, same npc) re-sends Opened. Only the initiator's messages for
  a conv are accepted (anything else counts as `rejected`). The host never evaluates conditions: the initiator resolves topic/INFO
  selection and reports the `facts` it observed; the host validates ordering.
* **Line broadcast.** On Pick the host runs the Begin result script, sends `Lines` to every connected slot except the initiator, and
  `Result{Begin}` to everyone. `Offer` (the next topic list) is relayed to spectators the same way.
* **Choice and steps.** Pick must carry `step == current + 1` while the conversation awaits a pick; a duplicate Pick (same step) re-sends the
  Result and never re-runs the script. LineDone runs the End script exactly once, sends `Result{End}`, and closes the conversation with
  `Goodbye` when the Pick had the goodbye flag.
* **A pick runs once.** The host runs the Begin script once per `(conv, step)` and the End script once per `(conv, step)`; the plugin's runner (`IntentService::runDialogueResult`) adds a
  second line of defence keyed by `(session, conv, step, phase)` (a repeated call is answered with the first run's txn seq, counted as `service.dialogueResults.deduped`; mutation twin 441).
  A Pick for the SAME INFO as the previous step with no Offer from the initiator since the previous pick is a repeat, not a second choice (the engine showed the INFO it was
  speaking as "spoken" again): the host answers the step (`Result` Begin / End with txnSeq 0), shows nothing to spectators and runs nothing (`HostDialogue::repeatPicks`). The initiator's
  glue also latches a picked INFO until the DialogMenu is back at "pick a line" (`pickLatched`). Story run S06: one pick of "Sure, I'll come with you" (`Player.AddItem Ammo556mm 60`) paid 3 to 4 times, and not because the runner ran more than once: the host plays the line for the spectators with `<npc>.Say`, and the engine's Say runs the INFO's result scripts itself, once per response line, as host-origin work (own commit, full mask, the host's real AddItem). `SpectatorSayGate` (plugin/hooks/spectator_say.h) skips every INFO result script the engine starts for an NPC the plugin just made `Say` something, outside the runner's acting window (mutation twin 97).
* **Result-txn link.** `Result.txnSeq` is the story-log seq of the EffectTxn the result script committed (0 = none). That txn travels on the
  story channel, so it can arrive after the Result. The initiator must not build the next topic list (its conditions read quest state)
  until its story session has applied `max(Begin.txnSeq, End.txnSeq)`. `ClientDialogue::setAppliedProvider` /
  `DialogueNode::Config::appliedSeq` supply the applied seq; the gate falls back to `gateTimeoutMs` (default 1500 ms from LineDone) and the
  TopicsReady event then has `timedOut` set.
* **Initiator disconnect.** When the transport reports the initiator gone the plugin calls `DialogueNode::playerLeft(slot)` (alongside
  `HostSession::peerDisconnected`). The host closes every conversation of that slot with `Closed{Disconnect}`: a begun line's End script
  still runs once (quest state is never half-applied), the NPC lock is freed, the control-lock mask is released (`Control{locked=false}`
  to the initiator while it is connected) and all remaining slots, including the host's own client side, get `Closed` so their menus and
  overlays close. Silence for `leaseMs` (default 300 s; Heartbeat or any initiator message refreshes it) closes with `Timeout`; NPC
  death/combat close with `NpcDead`/`Combat`.
* **Late join.** `DialogueNode::playerJoined(slot)` sends the joiner `Opened` for each running conversation, then the latest `Lines` and,
  while the initiator is choosing, the latest `Offer`. Call it when the story session reports the slot joined.
* **Rejoin.** A returning initiator never received the Closed. When its story session (re)joins it must call `DialogueNode::resync()`, which
  clears the stale Initiator/Spectator role and emits `CloseMenu` (`wasInitiator` set if the real menu is open) so the engine glue
  closes the menu and re-enables player controls.

## 11. Rejoin and late-join rules (story channel)

Pinned by the integrated tests `synccore/tests/test_rejoin*.cpp`:

* A plain reconnect (same process, `ClientSession::connect` with a larger `session`) resumes from `lastSeq`; txns it missed are replayed
  once, never twice. If the host world epoch changed meanwhile (the host loaded a save) the host sends `Rebase` and the client re-baselines
  via its baseline provider; txns of the old epoch are fenced off by `worldEpoch` and never replayed into the new epoch.
* A resume from the client's own save sidecar (`onLocalSaveLoaded`) is only valid against the timeline the sidecar was saved in, and
  epoch numbers do not identify one across host runs (a fresh host loading a prepared save reaches the epoch number the client's newer
  sidecar was saved in, with another history). The resuming Hello carries `cursorCheck`, a hash of the client's shadow at `lastSeq`; the
  host resumes only if its own state at `lastSeq` hashes the same and `lastSeq` is not past its head (a cursor the log no longer holds:
  only a token this host welcomed in this epoch). Otherwise it answers `Rebase` with its current epoch (the client accepts a Rebase to its
  own epoch while not joined), the client re-baselines and adopts the host's sequence; per-player state stays what its save holds.
  Counted in `HostStats::resumesRefused`.
* A baseline join (restart with the client's own save, `Hello.flags` bit0) receives the shared world as a diff plus the per-slot ledger
  (items/XP earned so far) as live effects. A restarted client whose save already contains those rewards receives them again: per-player
  rewards are not part of the world diff. Known limitation; clients that can tell should treat ledger effects delivered with `fromSnapshot`
  as "grant if missing".
* Barrier: `BarrierArrive` merges `required` (the slots joined at that arrival) and `arrived`; release happens once. A player who joins
  while a barrier is open must pass it too, or leave: leavers (`peerDisconnected`) stop counting immediately and `barrierTimeoutMs` is the
  last resort. Late joiners receive the barrier state through the normal snapshot/baseline.

## 12. Guest-originated interactions and the acting-player context

Problem: script blocks that react to "the player" (`OnActivate` + `IsActionRef player`, `OnTrigger/OnTriggerEnter player`, `OnAdd player`,
`OnHit` by player, pickups, and every `GetDistance player` / `GetItemCount player` / `GetAV player` inside them) are bound to the host's
player ref (0x14). A guest's avatar is only a ghost on the host (NV:MP), so those blocks never fire for a guest and, when they do fire for the
host, they read the host's inventory and position. Quest and object scripts run only on the host (guests are gated), so the host must run
the block on the guest's behalf. One design for all kinds:

**Guest side (detect, never apply).**
* ActivateRef / PickUp: the activation observer (`nvhooks::setActivateObserver`, ini `forward_activate=1`) runs on the guest. The local UI half
  (container menu, door animation) proceeds; the script half is already neutered by the guest gate. The runtime sends
  `Intent{kind=ActivateRef|PickUp, ref=target}` (protocol 5 above; durable, in order, exactly once, refused while `syncing`).
* TriggerEnter: sent for trigger volumes the host cannot observe itself (the guest is in a cell the host has not loaded, or the ghost is not
  yet there). Hook point = the guest's event dispatch (L5, `MarkScriptEvent` region): when the engine would fire a player-filtered block of a
  trigger/volume for the local player, send `Intent{TriggerEnter, ref=trigger ref}` instead. Hit / OnAdd are covered by Activate/PickUp and by the
  host seeing the ghost; they get their own kinds only if a gap list shows blocks that neither path reaches.
* Nothing is applied locally from the intent; effects come back as a normal host txn (origin = the guest slot).

**Host side (run the block as the guest).**
1. `HostSession::onAction(slot, kind, ref)` calls the runtime's `ActionExecutor`. The runtime sets `actingSlot() = slot` and calls the
   *acting hook* `(slot, true)`; afterwards `(slot, false)`. Inside that window the engine layer must:
   * open a `CaptureHub` scope (`Cause{Intent, source=ref, aux=kind}`); the runtime forces the txn origin to `slot`, so player-targeted effects
     (`AddItem`, `ModActorValue`, karma, reputation) are routed by the normal recipient rules (origin / rewards-to-all per policy/rules.json);
   * resolve the actor of the interaction and every `player` reference read by the block to the guest: `IsActionRef player`, `GetDistance
     player`, `GetItemCount player`, `GetActorValue player`, `GetInSameCell player`, `player.<cmd>`.
2. The *acting player* is the guest's ghost refID (`Runtime` `ghostOf(slot)`, provided by L6). The engine layer (L5) implements the resolution
   at ONE choke point: while an acting scope is alive, the script engine's resolution of form 0x14 (the `player` SCRO/SCRV reference and the
   `Condition::Evaluate` "run on player" subject) returns the acting ghost; the event-filter comparison "actor == player" treats the ghost as the
   player. Outside an acting scope nothing changes (host gameplay is untouched).
3. The block is started by re-issuing the interaction as the guest: `<target>.Activate <ghostRef> 1` (console path through
   `nvhooks::execConsoleLineOn`, calling reference = target). Pickups are `Activate` of the item ref; trigger enters are dispatched through the
   same event path once L5 provides a safe entry (`TriggerEnter` is logged as unsupported until then).
4. Stand-in (what is implemented now): when no ghost refID is known, the executor runs the interaction as the host's own player (0x14) with the
   same capture scope and origin attribution. `IsActionRef player` passes and the stage/var effects are correct and shared; the limitation is
   that the host's player really receives the item and `GetDistance player` measures the host. It is fully replaced by step 2 once the ghost
   refID exists.

**Invariants / tests.** (a) the executor window is the only time `actingSlot()` is set (tested: `testActions`, acting hook on/off pair, seen
inside); (b) txn origin == guest slot and rewards reach the requester only for origin-routed causes (tested); (c) a rejected / unknown target
leaves no scope open and logs `coop.guestAction issued:false`; (d) the same interaction submitted twice is executed twice (intent ids are
exactly-once per submission, not per target - the script decides whether it is repeatable).

**Coordinator decisions recorded here.** Message-box transitions (Fort -> Dam) become a party barrier (`Barrier` rule + `PartyWarp`), then every
player is warped; shared player storage containers (`NVCCPlayerStorageREF`, companion containers) become per-player (host keeps one container
per slot, intent target resolved by slot).

**Status 2026-10-21:** the host side (puppet as acting player, stand-in policy, TriggerEnter through `MarkBaseExtraListScriptEvent`, per-slot container redirection) and the guest side
(all seven kinds) are implemented in `plugin/intents` on its own channel, section 19; the stand-in described under 4 is now limited to targets that open no menu.
**Open items (older text).** L5: acting-player resolution at the script engine choke point + the guest dispatch hook for `TriggerEnter`; L6: ghost refID per
slot (`Runtime::setGhostResolver`); L4: rules for per-player containers; L10: per-slot container redirection in the executor.

## 12. Census round 3 semantics (wire v4)

**Created refs.** `HostSession::allocCreatedRef()` hands out stable ids `0xFF000000 | n` (plugin name `fnvcoop.created` for the FormRef form,
`createdRef(id)`); the host commits `SpawnRef`, every machine creates its own local object and keeps a host-ref -> local-ref map, other effects
(MoveRef, EnableRef, ObjectState, RefAddItem ...) address it with the same FormRef. `DespawnRef` is a tombstone so a late or replayed SpawnRef
never resurrects the object. The counter lives in the shadow state, so ids are never reused after save/load.

**Followers.** `CompanionOwner{npc, owner, active, kind}` recruits (active) or dismisses. The host drops recruits beyond
`HostSession::Options::followers` (default: 1 humanoid + 1 critter per owner, 4 for the party; counted in `stats.followerCapDrops`);
dismisses always pass. Recruit perks: `SetPerk` with the default recipient goes to every player for quest-reward causes
(QuestResult / DialogueResult) and to the origin (the recruiter) for everything else. `SetFactionRank` is shared.

**Control locks.** `SetPlayerControls{controls, enable, to}` (scope one or all through `to`). A lock never outlives a connection: the host releases a
leaving slot on `peerDisconnected` and again when a known token sends a new Hello, and a client releases its own controls in
`ClientSession::disconnect()` (the sink receives an enable). PartyWarp stays `MoveRef.partyWarp` plus the rules engine expansion.

**Object state.** Open/closed, destroyed (sticky), activate-once (sticky) live in `objects`; lock + level reuse the `locks` map (the same state
as `SetLock`). Default (closed, intact) objects are not stored.

**Ref script vars and inventories.** `RefScriptVar` is host-order last-write-wins and baseline covered; ref inventories are deltas against the
baseline world (`RemoveAllItems` clears and is sticky until later adds) and reach joiners through the snapshot diff, like the player ledger.

**Journal.** Topics, notes and map markers are shared and gated by `SharePolicy::journal`; `ForceActiveQuest` is per player (ledger).
Achievements and challenges are per machine and have no wire representation.

**Scripted saves, weather, time.** `RequestSave` and `AdvanceTime` are events (delivered once live, flagged `catchUp` on replay, never in
snapshots): the host saves and every client writes its sidecar (`ClientSession::saveState`) on RequestSave. `SetWeather` and `SetTimeScale` are
host-authoritative state and covered by the baseline.

**GuestLocalRun (rules.json).** Quests (and the script EDIDs in the optional `scripts` list) with this rule run locally on every machine and are never
replicated: `RulesEngine` drops any txn whose cause is such a quest and exposes `guestLocalRun(quest)`, `guestLocalQuests()` and
`guestLocalScripts()` for L5 to feed `SuppressPolicy::skipScript` (guests must not suppress these scripts).

## 13. Round 8 semantics (wire v5)

**Objectives.** `SetObjective.flags` is the exact flag state when the txn origin is the host (a hide, displayed 1 -> 0, survives resync and late
join; snapshot diffs and baseline entries carry the full flags, not the added bits). A guest-origin write can only add bits.

**Reward ids.** A reward is an additive per-player item or actor-value effect. Its id is `(worldEpoch & 0xFFFF) << 48 | (txn seq & 0xFFFFFFFF) << 16 |
effect index`, computed identically by the host (at commit), the receiving client (when the txn is folded in, including its own looted items) and
the client sidecar, so nothing extra travels with live txns. The host keeps the per-slot list of `{id, effect}` of the current epoch (cleared with the
ledgers when the world is replaced) and puts it in `Welcome.rewards`. The client persists the set of delivered ids in its save sidecar (client
sidecar v2, stored before the shadow so it is readable even when the shadow is not resumable). A baseline rejoin applies only the rewards whose
id is missing; a snapshot join marks every listed id delivered after the diff. Loading a save without a sidecar delivers everything (nothing is
known). `Mutations::noRewardIds` is the twin.

**Rules vocabulary added.** PartyTether `anchor` + `radius` (clients report `ReportNear` / `ReportFar`; an empty `cells` list without an anchor is a
load error), per-stage `stages` lists on Barrier / PartyTether (ExclusiveStages carries its own outcome list), `ExclusiveStages` (first committed
outcome wins, later ones are dropped and counted), `ReadyCheck` (`ScenePass` with `ref` = the INFO; `RulesEngine::onReadyRequested`), wait / sleep
consent (`ScenePass` with `ref` = `RulesEngine::timeSkipId()`; `onTimeSkipRequested`), `Barrier.before = slideshow` (controls of everyone locked from the
barrier release until the committed `SlideshowEnd`, with a failsafe), `PerPlayerScene` (effects with a Default recipient go to the acting player unless an
explicit sharing rule of the quest applies; `run = every-machine` + `scope = local-machine` makes the quest guest-local), `PerPlayerVars` (host side
per-slot copies, `RulesEngine::saveState`), the `max-over-party` / `sum-over-party` reductions, and payment: a `RemoveItem` in a topic result
(`CauseKind::DialogueResult`) with a Default recipient is charged to the initiator while rewards in the same result still go to the party.

## 14. Combat channel (first byte 0xCB)

Code: `plugin/combat/combat_wire.{h,cpp}` (messages), `plugin/combat/combat_sync.cpp` (rules of use), `synccore/combat/combat.h` (the payload structs and
their `io()`). Rides the coop `MuxTransport` next to the story channel; everything is reliable-ordered (ENet channel 0). Status: unit-tested game-free
(`plugin\combat\run_tests.cmd`); never run in a game.

```
byte 0   0xCB                kChannelCombat
byte 1   0x01                kCombatModuleWire (bumped on any incompatible change here; synccore/combat bumps kCombatWireVersion for its structs)
byte 2.. Archive (string table + body):  u8 src, u8 dest, u8 hops, u8 kind-1, body
```

| kind (index+1) | message | direction | body |
|---|---|---|---|
| 1 | `MHit` | attacker's machine -> victim's owner (via the host) | `HitEvent` |
| 2 | `MHitNack` | victim's owner -> attacker | `hitId`, `HitReason`, `resync` |
| 3 | `MVitals` | victim's owner -> everyone | `who`, `epoch`, `seq`, `health`, `maxHealth`, `limbMask`, `dead`, `cause` (hit id) |
| 4 | `MDeath` | the declarer -> everyone | `DeathDecl` |
| 5 | `MProjectile` | shooter's owner -> everyone | `ProjectileSpawn` |
| 6 | `MExplosion` | source's owner -> everyone | `ExplosionEvent` |
| 7 | `MCrime` | anybody -> host | `CrimeReport` |
| 8 | `MHostility` | host -> everyone | `HostilityEffect` |

Rules (each has a mutation twin in `plugin/combat/tests`):
- `src` is the author. The host overwrites it with the transport peer of a client's datagram (`Stats::spoofed`), so a client speaks only for itself. A client
  accepts datagrams from the host only.
- **A hit is resolved once, by the owner of its victim.** The attacker's machine fills a `HitEvent` (damage as ITS engine computed it, limb, impact point,
  weapon, ammo, projectile / explosion id, `attackerEpoch` for an NPC attacker, `vats` flag) and sends it with `dest` = the owner it believes in. The host
  re-routes by its own table (`hops` counts relays, limit 3), never trusting `dest`. The receiver runs `HitValidator::judge` (owner? new id? alive? attacker is
  the sender / the NPC's owner at the current epoch? range? rate?). Accepted: `applyHit` (health, limb crippling, named / essential floor at 1), the engine takes
  the health off, `MVitals` goes to everyone. A client never forwards a hit it does not own the victim of: it NACKs.
- **NACK and re-send.** `HandoffInProgress`, `UnknownVictim`, `NotVictimOwner` (the id was not consumed) are re-sent with the SAME id; `FutureAttackerEpoch`,
  `AttackerUnknown` (consumed) with a FRESH id. At most 3 re-sends, 150 ms apart, forgotten after 4 s.
- **Vitals** carry `(epoch, seq)` per victim; a receiver ignores an older pair, accepts only the owner's (or, for a player, that player's) message, mirrors health
  to its copy (never below 1 while alive: a death arrives as `MDeath`) and takes the UNION of crippled limbs (nothing un-cripples a limb).
- **Death.** ActorSync / the actor port decides THAT an actor dies (owner; for a named NPC only the host's KillActor). `MDeath` adds the credit: killer, weapon,
  crippled limbs, once per actor (`DeathLedger`: idempotent by actor id and by declaration id). A named NPC is declared by the host (`DeathSource::StoryHost`)
  only; an owner's declaration for a named NPC is refused. Kill XP is paid once per death (`XpLedger`), to the killing blow or split (`xpmode`).
- **Projectiles / explosions** are declared by the shooter's / source's owner and are VISUAL ONLY on every other machine: the engine hit gate neutralises any hit
  made by a copy, a visual explosion (matched by position and base form for 2 s) or a sourceless projectile while a visual one flies. Explosion damage goes out as
  one `MHit` per victim to that victim's owner with `hitId = explosionHitId(explosion, victim)`, so a second planning pass cannot double it.
- **Crime / hostility** (off unless `[combat] crime=1`): `MCrime` -> host `HostilityBook` -> `MHostility` to everyone; a client's `MHostility` is ignored.

VATS: real-time VATS shots resolve through the same `MHit` (the `vats` flag is informational). Track B owns the VATS UI; see docs/tools/combat-hooks.md "VATS interface".

## 15. World-time channel (first byte 0x7E)

Own channel with its own version byte (`kWireVersion` in `plugin/world_time/wt_types.h`, currently 1): the story wire version and `kProtocolVersion` are
NOT touched, because nothing in the story envelope or in the effect table changed. Source of truth: `plugin/world_time/wt_wire.{h,cpp}` (round trip, every
single-bit flip, every truncation and the range checks have tests and twins), behaviour in `wt_session.cpp`, `wt_clock.cpp`, `wt_broker.cpp`. Off unless
`nvcoop.ini [worldtime] enabled=1` on every machine.

```
byte 0  0x7E            kChannelWorldTime
byte 1  kWireVersion
byte 2  MsgType
...     payload, little endian; strings are u8 length + bytes (at most 96); FormRef = string plugin + u32 id
last 4  crc32 (IEEE) of every byte before it
```

| Type | Message | Direction | Payload |
|---|---|---|---|
| 1 | `Beacon` | host -> clients | `u32 seq`, time (`i32 year`, `u8 month`, `u8 day`, `f64 hour`, `f64 daysPassed`), `f32 rate` (game minutes per real second, measured; 0 while the host clock was paused), `f32 timeScale`, `u32 skipEpoch`, `u8 flags` (bit0 jump, bit1 partyTravel) |
| 2 | `SkipRequest` | client -> host | `u32 reqId`, `u8 kind` (0 wait, 1 sleep), `u32 minutes` (1..100000) |
| 3 | `TravelRequest` | client -> host | `u32 reqId`, `FormRef marker` |
| 4 | `Verdict` | host -> the asker | `u32 reqId`, `u8 kind`, `u8 granted`, `u8 reason`, `u32 minutes`, `u8 party` |
| 5 | `TravelGo` | host -> clients | `u32 reqId`, `FormRef marker`, `u8 mover`, `u8 party` |
| 6 | `Hello` | client -> host | `u8 wireVersion`: "send me a beacon now" (repeated every 3 s until one arrived) |
| 7 | `Status` | client -> host | `u8 inCombat`, about once a second; the host ignores a report older than 5 s |

Rules:
- **The host's clock is the only clock.** A `Beacon` goes out every 5 s and at once after a jump (script `set GameHour`, a host load, a granted skip; the first
  beacon of a host process is a jump). A client extrapolates it (`rate`), and snaps to the extrapolated host time when its own clock is more than 2.5 game
  minutes away (soft, at most once a second) or when the beacon says `jump` (hard, at once). Reordered old beacons are ignored (a host restart is recognised by
  `jump` with a lower `seq`). Drift is "hour-of-day difference plus a whole number of days", so it does not depend on whether `GameDaysPassed` is continuous.
- **Wait / sleep** needs every joined player: the host asks through L8's `raiseModal` (asker confirms, everybody else answers Ready / Not yet, any no or a
  timeout declines), refuses while anybody is fighting, allows one question at a time, starts an 8 s cooldown after a no, caps a skip at 12 h and re-checks the
  party (a player who joined, a fight that started) when the consent arrives. A grant sets the host clock, records the story effect `AdvanceTime` (tag 48) and
  sends a jump beacon with `skipEpoch + 1`; a client applies the skip once whichever of effect and beacon arrives first.
- **Fast travel** is free by default (the engine's own may-travel check refuses combat). With `[worldtime] travel=party`, or while a `PartyTether` holds the
  party, a client asks first; after consent the host sends `TravelGo(party=1)` and EVERYBODY travels to the marker. The mode rides in every beacon.
- Requests from a slot that is not a joined client, beacons from a client, and duplicate request ids are ignored. A client drops a request that got no
  verdict after 60 s.
- Not on this channel: `RequestSave` (tag 45) is acknowledged and ignored by clients (only the host's save is the world); `ObjectState` (tag 35) is also
  produced by a poll of native door / lock / destroyed changes (`wt_objects.cpp`), so the story log, the baseline and rejoin carry them like script changes.

## 16. Avatar channel (first byte 0xAD)

Owner: avatars (`plugin/coop/live/avatar_wire.*`, `avatar_net.*`, `plugin/avatar/stream_rate.h`). Star topology: a client talks to the host only, the host
relays. Game-free codec, unit tested (`plugin\avatar\full\run_tests.cmd`). Nothing here has run in a game.

| Kind byte | Message | Direction | Delivery | Payload |
|---|---|---|---|---|
| 1 / 2 | SnapshotBatch / SnapshotAck | both | unreliable sequenced | the `synccore/live` datagram (CRC protected), netId `0xFE0000<slot>` |
| 0x40 | Roster | host -> clients | reliable | `u8 n`, `n` slots (every slot that has a player, the host included) |
| 0x41 | Look v1 | owner -> host -> others | reliable | still decoded: slot, race, sex, hair, eyes, faceHash, name, equipment (form + weapon flag) |
| 0x42 | Resync | client -> host | reliable | empty: "my link came back, send the roster and every look again" |
| 0x44 | Facts | client -> host only | reliable, on change | `u8 version (1)`, `u8 slot`, `u16 level`, `u8 sitting`, `u32 actions` (bit n = IsPlayerActionActive n), `u8 n` (at most 16), `n` x `u32 region` |
| **0x45** | **Pack** | client -> host only | reliable, on change, at most 1 per second | `u8 version (1)`, `u8 slot`, `u8 flags` (bit 0 weight follows, the rest reserved), `u32 watchId`, [`u32 weight` (InventoryWeight x 1000)], `u16 n` (at most 255), `n` x (`u32 form`, `i32 count`), forms ascending, counts above zero |
| **0x43** | **Look v2** | owner -> host -> others | reliable | `u8 version (2)`, `u8 slot`, `u8 flags`, `u32 race`, `u32 hair`, `u32 eyes`, `u32 faceHash`, [`u32 hairColor`], [`u16 height`, `u16 weight`], [`u8 n`, `n` face coefficients], `u8 nameLen`, name, `u8 n`, `n` x (`u32 form`, `u8 flags`, [`u8 mods`]) |

Look v2 flags: bit 0 female, bit 1 hair colour follows, bit 2 height and weight follow (thousandths, 0 = unknown), bit 3 face block follows (1..160 signed bytes,
the FaceGen coefficients in 1/16), bits 4 to 7 reserved (a decoder rejects them). Equipment flags: bit 0 weapon (the HELD weapon), bit 1 a modification byte follows
(1..7, bit n = mod n+1; only on a weapon), the other bits reserved. A decoder rejects a truncated message, trailing bytes, a slot of 8 or more, more than 64 items,
more than 48 name bytes. `encodeLook` always writes v2; the count of equipment entries is the last byte of a look without equipment.

Pack (guest pack facts): what a guest carries that the host cannot read off its puppet (the puppet's pack is empty): the InventoryWeight actor value and the counts of the WATCHED item
forms. The watch list is `nvcoop_pack_watch.json` (`plugin/coop/tools/gen_coop_data.py`), generated from the rules (`PooledItems` items) and the quest data (every `GetItemCount` of an INFO,
quest or stage condition, and `GetItemCount <item>` in the scripts of INFOs, quests and stages): 265 forms, not the whole inventory. Both machines load the same file; `watchId` is an
FNV-1a over the sorted runtime form ids of it, so the host reads a form that is absent from the message as "zero" only when its own list has the same id and holds the form; anything else is
unknown and the puppet's own answer stands. The guest reads its pack every 500 ms, sends when it changed, and at most once per second. Host: `AvatarNet::packCount / packWeight` ->
`gluePackCount / gluePackWeight` -> `bindSlotPack` (the party state provider: GetItemCount and GetActorValue InventoryWeight of a guest, `MemberView::StateOrGhost`) and the
`Runtime::setItemCountProbe` of the pooled hand-ins. A hand-in that runs for a guest (a dialogue result) does not remove from the host's pack (`nvhooks::GuestItemScope`): the removal is
recorded, the rules split it over the holders, each part reaches that guest's machine as a txn recipient, and the host's own share is taken by `Runtime::onHostCommitted`.

Rules:
- **Rates** (FULL-SYNC-PLAN section 2): the stream of player S reaches observer O at 20 Hz when O is in the same cell within 2800 units (1.25x hysteresis), at 5 Hz in
  the same cell beyond that or in an adjacent cell, at 1 Hz anywhere else (the ownership and spawn services still want every place). A snapshot that arrives before its
  turn is kept (latest wins) and released when the turn comes, so the final position of a player who stopped is never lost. A client decides what the host gets of it
  (the uplink) by the HIGHEST tier any other player has for it, because the host relays that stream onward.
- **No forged avatars**: the host accepts a snapshot or a look from a client only for that client's own slot; a roster only from the host.
- A look is sent when it changes (checked twice a second; the game is asked for the local appearance every 2 s) and once to every newcomer. A client applies a look to a
  puppet whose template (race, sex) is unchanged IN PLACE (hair, height, equipment, weapon mods); another race or sex needs another template NPC, which is a new puppet that
  keeps the equipment.
- Versioning: the avatar messages carry their own version (`kAvatarWireVersion = 2`, inside Look v2); an older build ignores kind 0x43 (it counts it as a bad datagram)
  and shows its puppets with the template look. `kProtocolVersion` is NOT bumped for this channel: it is not part of the Hello handshake, and the live layer's
  own version gate (`kLiveSchemaVersion`, `kActorWireVersion`) already refuses mixed builds of the snapshot schema.

## 17. Area digest channel (first byte 0xDA)

Code: `synccore/areadigest/ad_wire.{h,cpp}` (messages), `synccore/areadigest/areadigest.cpp` (rules of use). Rides the coop `MuxTransport` next to the other
channels; everything is reliable-ordered (ENet channel 0). Status: unit-tested game-free (`plugin/areadigest/run_tests.cmd`); never run in a game.
Design and counters: `docs/tools/area-desync.md`.

```
byte 0   0xDA                kChannelAreaDigest
byte 1.. Archive (string table + body):  u8 version (kAreaDigestWireVersion = 1), u8 src slot, u8 body index, body
```

| index | message | direction | body |
|---|---|---|---|
| 0 | `MDigest` | client -> host, every `intervalMs` (2 s) | `round`, loaded `cells`, per (cell, category) `{cell, cat, hash, count}`, optional story digest `{hash, count, seq, epoch}` |
| 1 | `MItemsReq` | host -> client (or host -> an actor's owner) | `caseId`, groups `{cell, cat}` and/or keys `{cat, key}` whose full items are wanted |
| 2 | `MItems` | client -> host | `caseId`, `truncated`, items `{key, cat, cell, alive, hp, pos, state, settledMs, version}` |
| 3 | `MHeal` | host -> the machine that is wrong | `caseId`, items `{authoritative item, forVersion, epoch}` |

Categories: 1 actors, 2 containers, 3 objects (0 = story is only the optional digest part of `MDigest`, never an item).

Rules (each has a mutation twin in `synccore/areadigest/tests`):
- `src` is the author claim; the receiver trusts the transport peer (`Counters::spoofed` counts a mismatch). A client accepts `MItemsReq` and `MHeal` from the host
  only, the host accepts `MDigest` and `MItems` from joined clients only (`wrongDirection`, `unknownPeer`).
- A digest is rate limited per client (`intervalMs / 2`). An item that changed less than `settleMs` ago hashes only its identity (cell, alive): a value still
  travelling never makes two digests differ. A group whose diff showed nothing to heal is not diffed again while both hashes stay the same (backoff 10 s, doubling, 40 s cap).
- The authority of an item is the owner of the actor (fetched from the owner with an `MItemsReq` by key when it is a third machine) and the host for everything
  else. An actor with no known owner, or with a handoff in flight, is never healed. Nothing is healed unless both sides were quiet for `settleMs`.
- `MHeal` carries the target's own item `version` and the owner `epoch`: the target drops the heal when the item changed since it reported it, when it knows a newer
  owner epoch, when it owns the actor itself, or when nothing differs any more. A duplicated `MHeal` therefore changes nothing the second time.
- Position: an actor more than 2 m off is moved smoothly, more than 10 m off or in another cell it is snapped. Health differences up to `hpTolerance` (5) and
  corpse positions are ignored. Death is never applied by a heal (it is handed to the actor module); a resurrect is refused.
- Containers and objects are `delegated` in the plugin: a mismatch asks the story layer's repair (`HostSession::requestDigest`) instead of pushing values.
- Versioning: the module carries its own version byte; `kProtocolVersion` / `kWireVersion` are NOT bumped for this channel (it is not part of the Hello
  handshake, like 0xAC, 0xCB, 0xA5, 0xAD). A build without the module drops the datagrams as unroutable; the host's exchange with such a client times out
  (`casesTimedOut`) and backs off.

## 18. Social channel (first byte 0x5C)

Code: `synccore/social/wire.{h,cpp}` (messages and their payload structs), `plugin/companions/social_sync.cpp` (rules of use). Rides the coop `MuxTransport` next to the other
channels; everything is reliable-ordered. Status: unit-tested game-free (`plugin\companions
un_tests.cmd`, `synccore\social
un_tests.cmd`); never run in a game.

```
byte 0   0x5C                kChannelSocial
byte 1   0x01                kSocialWireVersion
byte 2.. Archive (string table + body):  u8 src, u8 dest, u8 kind index, body
```

`src` is the author claim; the host replaces it with the transport peer's slot on a client's datagram (`Stats::spoofed` counts a mismatch) and a client accepts the host only.
`dest` = 255 means everyone except `src`. A client sends to the host only.

| kind (index+1) | message | direction | body |
|---|---|---|---|
| 1 | `MCrime` | anybody -> host | `CrimeEvent` (kind, criminal, victim, crime faction, cell, where, dead / hostile flags, `nativeMask`) |
| 2 | `MDetect` | an NPC's owner -> host | `Detection` (observer NPC net id, its crime faction, the player it sees, cell, seen / lost) |
| 3 | `MBounty` | host -> everyone | the wanted table or its changes: per faction `{bounty, attack, murder, enemy, rev}`; `full` replaces the table |
| 4 | `MAlarm` | host -> everyone | `AlarmOrder` (id, faction, cell, where, criminal, until) |
| 5 | `MHostility` | host -> everyone | combat's `HostilityEffect` (SetEnemy / SetAlly / StopCombat), same idempotency ids as section 14 |
| 6 | `MKill` | anybody -> host | `KillReport` (death id, victim facts, killer, recruiter, contributors, `nativeMask`, cell, where) |
| 7 | `MRoster` | host -> everyone | the companion table: `{npc, owner, kind, state{mode, stance, range, rev}, flags}`; `flags` bit 0 = party companion (owner=party, ED-E in Lonesome Road: `owner` stays the recruiter slot, any player may command it, it uses no follower slot); a value above 1 is a malformed entry |
| 8 | `MCmd` | the owner -> host | `CompanionCmd` (id, from, npc, kind, arg) |
| 9 | `MCmdNack` | host -> the issuer | command id and the refusal reason |
| 10 | `MState` | host -> everyone | one companion's `{mode, stance, range, rev}` |
| 11 | `MInv` | the owner -> host, host -> everyone else | one signed inventory delta `{txn, npc, item, delta}` |
| 12 | `MInvFull` | the owner -> host -> everyone, or host -> a joiner | a companion's whole table of changes |
| 13 | `MInvAsk` | anybody -> host | "send me this companion's table" |
| 14 | `MSettings` | host -> everyone | the host's settings: kill XP mode and scope, karma sharing, friendly fire, foreign-companion policy, follower caps, karma table, `rev` |
| 15 | `MDigest` | host -> everyone, every 5 s | companion-table digest, wanted-table digest, settings `rev` |
| 16 | `MResync` | client -> host | "send me everything" (join, a digest mismatch, a refused command) |

Rules (each has a mutation twin: `synccore/social/tests` 801..883, `plugin/companions/tests` 901..931):
- **Companions** belong to their recruiter. The host learns recruits and dismisses from the story effects (`CompanionOwner`), drops a recruit beyond the follower caps, and tells
  everyone (`MRoster`). Every machine keeps only ITS player's companions flagged as teammates in its engine (a flag for anybody else's, or for an NPC the roster does not
  know, is taken off after a cooldown; nothing is touched before the host's picture has arrived). The roster is also fed to plugin/actors (`engine::onEffect`), so the actor
  layer's follower exception makes the recruiter's machine the NPC's owner.
- **Commands** (wait, follow, stance, range, wheel, trade, equip, heal, talk, summon, dismiss) are owner-only: the owner's machine refuses a stranger's at once, the host
  refuses a forged one with `MCmdNack`, once per id, rate limited. State carries a revision; an older one never overwrites a newer one.
- **Inventory**: only the owner's machine reports its companion's inventory changes (a poll of the engine's changes against the base container, so UI moves, scripts and
  the NPC's own pickups all count). The host accepts a delta from the owner only, once per txn id, within range; every other machine applies it to its copy once; the owner
  ignores the echo. A joiner gets the host's table and derives what to change.
- **Crime**: a witnessed crime becomes one decision on the host (`CrimeJudge`): a wanted score per faction, the engine's faction flags (SetPCFactionAttack / SetPCFactionMurder /
  SetPCEnemyOfFaction), SetEnemy / SetAlly through `MHostility`, one alarm per faction and cell, infamy as a committed `ModReputation`. A witness is an NPC of the faction that
  cares, in the cell, seen within the window by ANY player: each NPC's owner reports its own NPCs' sightings (`MDetect`; a report about an NPC the sender does not own is
  rejected). A guest's theft / break-in / trespass is judged on the host from the guest's intent. A burst is one crime.
- **Kill rewards**: a kill is settled once on the host (by death id) by the host's setting (killing blow or party share, scope: everybody / same cell / near). XP, karma and
  reputation reach the other players as committed effects with explicit recipient masks (`ModActorValue` experience, `ModKarma`, `ModReputation`); the host's own part is applied
  directly. A slot whose own engine already paid natively (`nativeMask`, claimable for the sender's own slot only) is not paid again.
- Versioning: the module carries its own version byte; `kProtocolVersion` / `kWireVersion` are NOT bumped for this channel (it is not part of the Hello handshake, like 0xAC,
  0xCB, 0xA5, 0xAD, 0x7E, 0xDA). A build without the module drops the datagrams as unroutable.

## 19. Guest-intents channel (first byte 0x8D)

Code: `plugin/intents/intent_wire.{h,cpp}` (codec), `intent_host.cpp` (rules of use), `intent_client.cpp`, docs/tools/guest-intents.md. Rides the coop `MuxTransport` next to the
story channel; always reliable-ordered. Status: unit-tested game-free (`plugin\intents\run_tests.cmd`, including a real Runtime with a host and two guests over ENet loopback);
never run in a game. This is the full implementation of the design in section 12 (the older story-channel intents `ActivateRef` / `TriggerEnter` / `PickUp` still work and are
served by the same host pipeline).

```
byte 0   0x8D   kChannelIntents
byte 1   0x01   kIntentsWire (bumped on any incompatible change here)
byte 2   type   1 Intent, 2 Verdict
byte 3.. Archive (string table + body)
```

| type | message | direction | body |
|---|---|---|---|
| 1 | `Intent` | guest -> host | `varu32 session`, `varu32 id`, `u8 kind` (1..10), FormRef `target`, `boolean hasAux` [FormRef `aux`], `vari32 value`, `bytes path` (<= 8) |
| 2 | `Verdict` | host -> guest | `varu32 session`, `varu32 id`, `u8 result` (0..16), `u8 holder` (slot, 0xFF none), `boolean hasRedirect` [FormRef `redirect`] |

Kinds: 1 Activate, 2 TriggerEnter, 3 PickUp, 4 Hit, 5 DialogueStart, 6 TerminalUse (`path` = entry indices from the top menu down), 7 ContainerOpen, 8 ContainerClose, 9 TriggerLeave, 10 SetStage (`target` = the quest, `value` = the stage 1..255; a request: the guest applied nothing, the host runs the real `SetStage` only when the quest is behind that stage and the host's rules allow it), 11 TriggerStay (`target` = the trigger volume; a keep-alive of the guest's presence inside it, sent at most once per `stay_keepalive_ms`; the host raises `OnTrigger` for the puppet by itself every host frame until TriggerLeave or the lease `stay_lease_ms` runs out), 12 StartCombat (`target` = the actor that started combat with the guest's player; the host runs its `OnStartCombat` blocks once with the puppet as `player`).
Results: 0 Executed, 1 Duplicate, 2 Disabled, 3 NotJoined, 4 RateLimited, 5 BadTarget, 6 NotLoaded, 7 OutOfRange, 8 Busy (`holder`), 9 Taken, 10 Denied, 11 NoPuppet, 12 Unsupported, 13 Failed, 14 Stale, 15 NoAdvance (SetStage: the quest is already at or past the stage), 16 Deferred (SetStage: the host's barrier / tether holds it and releases it later).

Rules (each has a mutation twin in `plugin/intents/tests`, numbers 301..336 and 401..415):
- `session` is the guest's story session id; ids start at 1 in every session and only grow. The host runs each (slot, session, id) once; a repeat gets the SAME verdict again, an older id
  is `Duplicate`, id 0 is `Stale`. The slot is the transport peer's, never a field of the message. A datagram from a peer that is not a joined player is dropped.
- A guest never applies anything from an intent. The host runs the interaction under `Runtime::runAsGuest(slot)` with the guest's puppet as the acting player; every effect
  comes back as an ordinary txn with `origin = slot`. A pickup is a CLAIM: the first wins (`Taken` for the rest), the item ref is disabled and an `AddItem{base, count, Recipient::origin()}`
  is committed, so the item reaches the claimer only. A container has one user at a time (`Busy` + `holder`), leased (`lock_lease_ms`), released by ContainerClose, by the holder
  leaving, or by the lease. A shared storage container in `nvcoop_intents.json` is answered with `redirect` = the container of that player.
- SetStage: the guest's detour of the engine's `SetStage` drops a script / console call on a guest (the host owns quest progress) and hands it to the intents client instead of losing it
  (`nvhooks::setStageRequestObserver`, counters `stagesBlocked` / `stagesForwarded` in `hooks.scripts`). One intent per (quest, stage) per `coalesce_ms`. The host answers `NoAdvance` when the
  quest is already at or past the stage (the host's own copy of the script got there first; result scripts never run twice), `Denied` when the rules engine's ExclusiveStages already decided the
  fork the other way, `Deferred` when a barrier / tether holds it; otherwise it runs the real `setstage` as the guest and the txn (origin = guest) reaches every copy, the requester's too. Each refusal is
  a counter (`intents.status` service.detail.stage) and a host log line. Quest VARIABLES a guest writes are not part of this: an inline `set Quest.var` has no call to detour (guest_objvars.h puts the
  host's value back for known object scripts; the host's digest repair does it for everything else).
- PickUp of a NOTE (the item's base form is a note / holotape) grants the journal entry, not an inventory item: the host's journal gains the note (`AddNote`) and the shared `AddNote` effect travels in the txn with `origin` = the guest. The ref is disabled after the note was added.
- Gear stashes (docs/tools/guest-intents.md): around every guest interaction the host swaps the contents of the shared gear containers of `nvcoop_intents.json` (`stashes`) for that guest's stash and back; nothing of the swap is on the wire.
- Guards (host): rate limit per player (token bucket), reach from the puppet to the target, `Hit` refused on actors (the combat channel raises OnHit for them), a per-player trigger
  cooldown, a bounded claim table. Everything is switchable per kind in `nvcoop.ini [intents]`.
- Versioning: the module has its own version byte; `kProtocolVersion` / `kWireVersion` are NOT bumped (the channel is not part of the Hello handshake, like 0xAC, 0xCB, 0xA5, 0xAD,
  0x7E, 0xDA). A build without the module drops the datagrams as unroutable and the guest's intents time out (`pending_timeout_ms`), after which the local call has already gone on for
  everything except pickups and trigger events.

## 20. VATS channel (first byte 0xB7)

Code: `plugin/vats/vats_wire.{h,cpp}` (messages), `plugin/vats/vats_sync.cpp` (rules of use), docs/tools/vats.md. Rides the coop `MuxTransport`; everything is
reliable-ordered (ENet channel 0). Star topology: a client talks to the host only, the host relays. Status: unit-tested game-free (`plugin\vats\run_tests.cmd`);
never run in a game. This channel is self-versioned (byte 1) and does NOT change the story envelope, so `kProtocolVersion` is not bumped (the combat, avatar and
world-time channels did the same): a build without this module drops the unknown first byte at the mux.

```
byte 0   0xB7                kChannelVats
byte 1   0x01                kVatsModuleWire (bumped on any incompatible change here)
byte 2.. Archive (string table + body):  u8 src, u8 dest, u8 hops, u8 kind-1, body
```

| kind (index+1) | message | direction | body |
|---|---|---|---|
| 1 | `MConfig` | host -> everyone | `varu32 epoch`, `u8 flags` (1 realtime, 2 partySlowMo, 4 autoFire, 8 killCam, 16 guardFriendly, 32 perks), `f32 slowScale`, `varu32 slowMaxMs`, `varu32 slowCooldownMs`, `varu32 autoFireDelayMs` |
| 2 | `MPhase` | anyone -> everyone | `u8 player`, `u8 mode` (0 none, 1 target select, 2 limb select, 3 unknown, 4 playback), `u8 limb` (25..31 or 255), `u8 queued`, `varu32 target` (actor net id, 0 = none), `varu32 seq` |
| 3 | `MAction` | anyone -> everyone | `u8 shooter`, `varu32 victim` (net id), `u8 limb`, `u8 queued`, `varu32 seq` |
| 4 | `MSlowMo` | host -> everyone | `varu32 epoch`, `varu32 seq`, `bool active`, `f32 scale`, `varu32 remainingMs`, `u8 cause` |

Rules (each has a mutation twin in `plugin/vats/tests`):
- **The host's settings win.** A client adopts every `MConfig` it receives and falls back to its own nvcoop.ini only when the host link drops. The host rejects an
  `MConfig` from a client. Numbers are clamped on decode (`slowScale` 0.10 to 0.90, `slowMaxMs` 500 to 10000, `slowCooldownMs` 0 to 120000, `autoFireDelayMs` 0 to 3000).
- `src` is the author. The host overwrites it with the transport peer's slot (`Stats::spoofed`), and a client may only announce its own `MPhase` / `MAction`.
  A client accepts datagrams from the host only. Relays stop at `hops` = 3.
- **Party slow-mo is host-authoritative.** Only an ENTRY into VATS (mode 0 to non-zero) of any player can start a window, one window at a time, never longer than
  `slowMaxMs` (a second entrant does not extend it), ending early when the last VATS user leaves or disconnects, followed by a cooldown. The host repeats the
  current grant every second while it runs. A client expires a window by itself `remainingMs` + 750 ms after the last grant it heard (a lost host cannot leave the
  party slowed), ignores an older `(epoch, seq)` and clamps `scale` and `remainingMs`.
- The damage of a VATS shot is NOT on this channel: it is an ordinary `MHit` of the combat channel (section 14), resolved once by the victim's owner.

## 21. RefCommand: script commands on a reference (wire v6, story-capture)

Code: `synccore/include/fnvcoop/refop.h` (op ids and merge rules), `effect.h` (the struct), `wire.cpp` (`ioE`), `shadow.cpp` / `baseline.cpp` (state),
`plugin/hooks/ref_ops.{h,cpp}` (engine side). Docs: docs/tools/story-capture.md. Status: unit-tested game-free (`synccore` group `refcmd`, `plugin\hooks\run_story_tests.cmd`);
never run in a game. The hooks that produce it are OFF unless `nvcoop.ini [hooks] story_capture=1`.

One new effect, tag 49 (the table in section 6 is generated): `varu32 op, ref, a, b, vari32 i, j, k, l, f32 f, str s (at most 255 bytes), recipient`.
`ref` is the calling reference (empty plugin = none), `a` / `b` are form arguments, `s` is a text argument. What the slots mean is per op (`kStorySpecs`).

- **Second shape (wide ops).** `refOpIsWide(op)` (today only UseWeapon, op 78: four forms and five integers) adds `ref c, ref d, vari32 m` after the recipient. Only wide ops carry
  them, so every other op's bytes are unchanged. The decoder reads them by the op id alone.
- **Op ids are append-only** and part of the format. A decoder rejects op 0 and every id it does not know (`Malformed`); a txn that carries one is dropped whole.
- **Event or state.** `refop.h` marks every op. An EVENT (StartCombat, EvaluatePackage, Dispel, the chargen calls ...) is delivered once to its
  recipients and remembered nowhere. A STATE op (SetGhost, SetScale, AddSpell / RemoveSpell, AddFormToFormList ...) replaces the previous command with the same key
  `(group, ref, a, b)` (the slots the op keys on; AddSpell and RemoveSpell share a group, so the later of the two per spell wins). State commands are part of the
  ShadowState (sidecar), of the baseline (class 5, key hash segment 12 next to textures) and of the diff a late joiner gets. They carry no pre-image: a host load
  cannot take one back from a client (the same limit as the one-way quest effects).
- **Recipients.** The capture sets `to`: Origin for the character-build calls (SetLevel, SetSPECIALPoints ...), All for achievements and challenges and for every command on
  something other than the player (actor, container, list, quest), Default for a command on the player. `normalizeTxn` resolves Default like the reward of AddItem (everyone
  for a quest or dialogue result, the acting player otherwise), so a client performs a command only if its slot is in `to`.
- **Who performs it.** Every machine that is a recipient receives an event; actor AI orders (EvaluatePackage, StartCombat, StopCombat, ResetAI, Look, SetAlert, PlayGroup ...)
  are performed only by the machine that runs the actor's AI (`RemoteApplier::setDefaultRemoteDrivenQuery`, wired to the actor ownership layer); the others answer NotForMe and
  see the result through the actor snapshots. Activate is performed only on activators and doors.
- **Version.** `kWireVersion` and `kProtocolVersion` are 6, the host sidecar is version 4 and the client sidecar version 3. An older build is turned away at the handshake
  and an older sidecar is not resumed (the world is rebuilt from the host's baseline).


## 22. Loot channel (first byte 0xB1)

Code: `synccore/loot/loot.{h,cpp}` (rules), `synccore/loot/loot_wire.{h,cpp}` (messages), `plugin/loot/loot_sync.{h,cpp}` (the live-hub service), docs/tools/loot.md. Rides the coop
`MuxTransport` through the live hub (`livewire::ILiveService`); everything is reliable-ordered except `MClutter` (unreliable sequenced). Status: unit-tested game-free
(`plugin\loot
un_tests.cmd`); never run in a game. The channel is self-versioned and does NOT change the story envelope, so `kProtocolVersion` / `kWireVersion` are not bumped
(like 0xAC, 0xCB, 0xA5, 0xAD, 0x7E, 0xDA, 0xB7): a build without the module drops the datagrams as unroutable and a guest's requests time out and roll back.

```
byte 0   0xB1                kChannelLoot
byte 1.. Archive (string table + body):  u8 version (=2), u8 src, u8 body index, body
```

`src` is informational: the host uses the transport peer's slot and counts a mismatch (`Stats::spoofed`); a guest accepts datagrams from the host only.

| index | message | direction | body |
|---|---|---|---|
| 0 | `ReqRegister` | guest -> host | `varu32 txn`, ContainerRec (the sender resolved this container / corpse: the real items) |
| 1 | `ReqOpen` | guest -> host | `varu32 txn`, `varu32 cont`, `boolean hasAdopt` [ContainerRec: unregistered container, the opener's contents become the shared ones] |
| 2 | `ReqMove` | guest -> host | `varu32 txn`, `varu32 cont`, FormRef base, `vari32 count`, `boolean put` (take / put) |
| 3 | `ReqClose` | guest -> host | `varu32 txn`, `varu32 cont`, `u64 digest` of the closer's contents |
| 4 | `ReqHold` | guest -> host | `varu32 cont` (lock keep-alive) |
| 5 | `ReqDrop` | guest -> host | `varu32 txn`, FormRef base, `vari32 count`, `varu32 meta`, `varu32 cell`, pos, rot |
| 6 | `ReqClaim` | guest -> host | `varu32 txn`, `varu32 item`, `boolean hasAdopt` [WorldItem: a placed item the host has not heard of, then FormRef ref: the placed world ref itself, plugin:localId] |
| 7 | `ReqResync` | guest -> host | `varu32 cont`, `boolean all` |
| 8 | `ReqDigest` | guest -> host | `u64 digest`, `varu32 containers`, `varu32 items` |
| 9 | `ReqTouch` | guest -> host | `varu32 net`, pos, rot (asks for clutter ownership) |
| 10 | `Result` | host -> guest | `varu32 txn`, `u8 op`, `u8 status`, `varu32 id`, `varu32 rev`, `u8 other`, FormRef base, `vari32 count`, `varu32 newId`, `boolean hasRec` [ContainerRec] |
| 11 | `MContainer` | host -> guest | ContainerRec (resolve broadcast, resync answer, table, script change) |
| 12 | `MDelta` | host -> guest | `varu32 cont`, `varu32 rev`, FormRef base, `vari32 newCount` (absolute count of one base after the change) |
| 13 | `MLock` | host -> guest | `varu32 cont`, `varu32 lockSeq`, `u8 holder` (0xFF none) |
| 14 | `MItemNew` | host -> guest | WorldItem (a dropped item, or the table) |
| 15 | `MItemGone` | host -> guest | `varu32 item`, `u8 by`, `varu32 txn`, FormRef ref (a PLACED item's world ref; empty plugin for a dropped one) |
| 16 | `MContainerGone` | host -> guest | `varu32 cont` (the host forgot it: cell reset) |
| 17 / 18 | `MTableBegin` / `MTableEnd` | host -> guest | `varu32 seq` / `varu32 seq`, `varu32 containers`, `varu32 items` (ids not mentioned between the two are removed, only when the counts match) |
| 19 | `MClutter` | either, unreliable | `varu32 net`, `varu32 epoch`, `varu32 seq`, pos, rot, `boolean moving` (a guest sends it to the host, the host relays it) |
| 20 | `MClutterOwn` | host -> guest | `varu32 net`, `u8 owner`, `varu32 epoch`, pos, rot |

ContainerRec = `varu32 id`, `u8 kind` (0 container, 1 corpse), `varu32 cell`, pos, items (`count <= 512` of FormRef + `vari32 count`), `varu32 rev`, `u8 holder`, `varu32 lockSeq`.
WorldItem = `varu32 id`, FormRef base, `vari32 count`, `varu32 meta`, `varu32 cell`, pos, rot, `u8 dropper`, `varu32 dropTxn`. Dropped items get host-issued ids in
`[0xFFB00000, 0xFFBFFFFF]`; placed items keep the id every machine derives (`plugin/actors deriveNetId`).

Rules (each has a mutation twin: synccore/loot 101..137, plugin/loot 141..167):
- **Every request with a txn is idempotent.** The host keeps the last 256 answers per sender; a replay gets the cached answer (with a container record refreshed to the state NOW), a txn
  older than the window gets `Stale`. The ledger is cleared when the sender joins; the guest restarts its counter on `hostLinkUp`.
- **One lock per container**, leased (default 10 minutes without a request or `ReqHold`), released by `ReqClose`, by the holder leaving, or by the lease. Take / put from a non-holder: `NotHolder`.
- **Contents are resolved once.** `ReqRegister` and the adopt record of `ReqOpen` make the FIRST sender's items the shared ones; a later different registration gets `AlreadyResolved` with the
  shared record, which the sender puts into its engine.
- **A close compares digests.** `ReqClose.digest` against the host's contents: a mismatch is `DigestMismatch` plus the authoritative record; the lock is released either way.
- **Pickups are claims.** The first claim of an item wins, later ones get `AlreadyTaken` (+ the winner) or `Unknown`; a placed item stays taken (tombstone, saved with the host table). The winner's own repeat of a claim
  is `Ok` (never a loss: its item would be taken back), and a client sends one claim per item at a time. A placed item's net id is a hash of its ref that nothing maps back, so the claim and `MItemGone` carry
  the ref (plugin:localId): that is how the other copies find and remove it (wire version 2; twins 170..175). The join table also carries one `MItemGone` with the ref per placed tombstone (max 4096, between
  `MTableBegin` and `MTableEnd`, not counted in `MTableEnd`), so a client that joins after the pickup removes the item as well (twins 180..183, 176, 177).
- **Containers are mirrored by revision.** `MDelta` applies only as `rev + 1`; a gap asks `ReqResync`; the client digest (`ReqDigest`, every 2 s while quiet) triggers a table push on mismatch (at most once per 2 s per guest).
- **Clutter** has one owner per epoch. Only the owner's snapshots at the current epoch and a higher seq are relayed; a second mover takes over only from an owner that is not moving.
- **Coordination with section 19.** Both modules can handle a pickup or a container open. With `[loot] enabled=1` set `[intents] pickup=0` and `container=0`; the loot module logs a warning when they are not.
