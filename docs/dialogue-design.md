# Dialogue co-op design (L8) v1

Owner decision (MASTER-PLAN): the player who **initiates** a conversation makes the choices; everyone else sees the
lines and the result; karma/reputation are shared; rewards go to all. Code: `plugin/dialogue/` (protocol, host and
client state machines, tests). Data: `out/questdata/infos.jsonl.gz` (28,933 INFOs). Regenerate the numbers with
`python -I plugin/dialogue/tools/info_stats.py` (full table: `plugin/dialogue/tools/info_stats.md`).

## 1. What the data says (all 28,933 INFOs)

| Pattern | INFOs | % | Consequence |
|---|---:|---:|---|
| have conditions | 27,850 | 96.3 | selection is evaluated once, by one machine (the initiator) |
| have a begin or end result script | 7,771 | 26.9 | host-run + effect capture (3,418 begin, 5,438 end) |
| script mutates the world (setstage, objectives, items, XP, ...) | 4,658 | 16.1 | captured as EffectTxn |
| script only sets quest/NPC script variables | 3,113 | 10.8 | **needs a `SetScriptVar` effect** (section 8) |
| script sets any `X.var` | 4,554 | 15.7 | same |
| script references `player` | 1,995 | 6.9 | player-context rule (3.2) |
| conditions reference the player (run-on target, GetPC*, GetIsReference) | 3,225 | 11.1 | initiator-evaluated facts (3.1) |
| skill/perk gate field set (Speech AV 643, other skills/perks 856) | 1,499 | 5.2 | per-initiator check |
| skill/perk/speech check in any form (incl. GetActorValue/HasPerk on player) | 1,714 | 5.9 | per-initiator check |
| speech check (Speech AV field 643, or GetActorValue 43 condition) | 661 | 2.3 | per-initiator check |
| `speech_challenge` flag (hard) | 3 | 0.0 | same path |
| reputation condition (shared) | 1,182 | 4.1 | identical everywhere |
| `GetQuestCompleted` (shared) | 526 | 1.8 | identical everywhere |
| inventory conditions (GetItemCount/GetEquipped on player) | 1,125 | 3.9 | initiator-evaluated |
| `GetScriptVariable` conditions (NPC script vars) | 594 | 2.1 | replicated via SetScriptVar (3.3) |
| control locks (Disable/EnablePlayerControls, EnableFastTravel) | 136 | 0.5 | lock lease (section 6) |
| service menu (ShowBarter/Caravan/Repair/Recipe menu) | 148 | 0.5 | service sub-session (7.2) |
| ending slides (EndGame, SwapTexture OnScreenSlide) | 315 | 1.1 | party-wide ending (7.3) |
| companion related (SetPlayerTeammate, vNPCFollowers, ...) | 200 | 0.7 | recruitment (7.1) |
| goodbye flag (ends conversation) | 9,831 | 34.0 | host closes after end script |
| player-choice links (`link_to`) | 8,118 | 28.1 | topic graph is walked by the initiator |
| NPC follow-ups | 1,760 | 6.1 | additional `Lines` in the same step |
| INFO types: topic 21,867, conversation 2,604, combat 2,300, detection 1,349, radio 769, misc 39, service 5 | | | only `topic`/`service` open a menu session; the rest are barks (7.4) |

Top script identifiers (INFOs using them): SetStage 1086, SetObjectiveCompleted 1016, SetObjectiveDisplayed 856,
RewardXP 833, RemoveItem 772, AddItem 744, EVP 471, AddReputation 311, SayTo 283, StartCombat 278, SetEnemy 176,
StartQuest 172, ShowMessage 167, AddScriptPackage 159, RewardKarma 129, SetAV 128, AddToFaction 106,
RemoveFromFaction 102, SetPlayerTeammate 87. Condition entries: 69,332 over 74 distinct functions; top: GetIsID 23,629,
GetIsVoiceType 17,322, GetQuestVariable 9,276, GetInFaction 3,465, GetGlobalValue 2,807, GetActorValue 1,679,
GetReputationThreshold 1,610, GetObjectiveCompleted 1,159, GetItemCount 1,061, GetStage 1,009. Condition run-on:
subject 64,407; **target (the player) 3,513**; reference 1,343; combat_target 69.

Every condition function falls into one of four classes (3.1), every script mutation maps to an existing or proposed
Effect (section 8), and every UI-side effect (menus, slides, control locks, companions) has a rule below. Open
items are listed in section 9.

## 2. Roles and who runs what

| Concern | Runs on | Why |
|---|---|---|
| Real `DialogMenu`, voice playback, topic list, picking | **initiator's** game | it is their UI; the vanilla code path stays untouched |
| INFO selection (`GetTopicInfo`, random INFOs, say-once) | initiator | needs the initiator's player facts; random resolved once |
| Begin / end result scripts | **host only**, under effect capture | single authority, no duplicated rewards (MASTER-PLAN) |
| NPC lock, ordering, lease, close | host (`HostDialogue`) | one arbiter, no races |
| Subtitle + voice for spectators | each spectator, driven by `Lines` | spectators never evaluate conditions |

The host machine can itself be the initiator: same state machine, loopback transport (test `happy_host_initiator`).

## 3. Condition evaluation

### 3.1 Four classes of condition
1. **NPC/subject state** (GetIsID, GetIsVoiceType, GetDead, GetInFaction on subject, ...): the NPC is host-owned and
   replicated; evaluated by the initiator on its puppet copy.
2. **Shared story state** (GetQuestVariable, GetStage, objectives, GetGlobalValue, GetQuestCompleted, reputation,
   karma): replicated by the sync core, so every machine agrees; evaluated by the initiator.
3. **Player-private facts** (run-on target: GetActorValue incl. Speech/skills 1,448, GetItemCount 814, GetIsSex,
   HasPerk, GetEquipped, GetHasNote, GetIsRace, GetLevel): **evaluated against the initiator, on the initiator's
   machine**. This is the speech/barter/perk policy: the initiator's Speech 55 decides whether a `[Speech 50]` option
   exists, never the host's or a spectator's. A companion's own stats are subject-class and unaffected.
4. **NPC/quest script variables** (`GetScriptVariable`, `GetQuestVariable`): see 3.3.

### 3.2 Result scripts and "the player"
The result script runs on the host, but `player.` / `GetPlayer` inside it must mean the initiator for **reads**, and
`Recipient::origin()` for **effects**:
- *Effects*: player-targeted effects are captured with `Recipient::origin()`; the host policy upgrades them (rewards,
  XP, caps: all players; karma/rep: shared; quest items: all players; companions: recruiter). `ResultRequest.initiator`
  tells L5 who "origin" is.
- *Reads*: `Pick` carries `facts[]` (actor values, item counts, perks, equipped) that the initiator observed for exactly
  the forms the INFO's conditions/scripts reference (statically known from SCRO refs and condition params; max
  `kMaxFacts = 128`). While the script runs, L5 answers `GetActorValue/GetItemCount/HasPerk/GetEquipped` on the
  player ref from these facts and falls back to the host player only for forms not in the bundle.

### 3.3 Script variables
NPC and quest script variables are not replicated by the engine. INFO conditions can only see values changed by
(a) result scripts, (b) quest scripts and (c) the NPC's own object script. (a) and (b) run on the host and are
replicated through `SetScriptVar` effects (section 8), so the initiator's puppet copy matches the host. (c) is the
residual risk (non-captured NPC script writes); the L9 differential suite should flag any divergence.

## 4. Conversation session lifecycle

```
initiator            host (HostDialogue)                         spectators
 requestStart(npc) -> StartReq{reqId,npc}
                      lock(npc) taken? -> StartDenied{Busy|AlreadyTalking|NpcDead|NpcInCombat|TooMany}
                      else -> Opened{conv,npc,initiator} -----------------------> banner
 OpenMenu (real DialogMenu)
 Offer{topics} ------> relay ---------------------------------------------------> read-only list
 Pick{step,topic,info,lines,facts}
                      validate(initiator, state, step == n+1)
                      run BEGIN script -> txn(seq b)
                      Lines{step,info,lines} ----------------------------------> subtitle + voice
                      Result{step,Begin,b} -> all
 (plays line locally)
 LineDone{step} ----> run END script -> txn(seq e)
                      Result{step,End,e} -> all      (initiator gate opens)
 TopicsReady (build next topic list)
 ... repeat; goodbye INFO: after END -> Closed{Goodbye} -> all, unlock(npc)
```
Rules: strictly increasing `step`; duplicate `Pick`/`LineDone` are idempotent (the txn is never re-run; the Result is
re-sent); non-initiator messages are rejected and counted; the host validates everything and trusts no spectator. The
initiator never receives its own `Lines`. `Opened` is broadcast so spectators know who is talking. A retransmitted
`StartReq` (same reqId) re-sends `Opened`. Messages travel as the opaque payload of an L4 envelope (proposal in
`internal lane notes`); the codec is CRC-protected and fuzz-tested.

### 4.1 NPC lock and the double-start race
The host handles `StartReq` strictly sequentially. The first to arrive takes the NPC lock; the second gets
`StartDenied{Busy, owner}`. The client **does not open the menu** until `OpenMenu` (an activation hook defers the
vanilla talk). Cost: one RTT before the menu opens, which is cheaper and safer than optimistic open + rollback. A
player can be in only one conversation (`AlreadyTalking`). Different NPCs may talk in parallel with different
initiators. Lock lifetime equals conversation lifetime and is released on every close path.

### 4.2 Spectators
Spectators get: a banner, a read-only topic list, `Lines` (text + voice file + emotion; played on the NPC puppet via
say/subtitle, subtitle-only fallback if the voice file cannot be resolved), and `Closed`. They are **not** locked: they
can walk away (overlay hides on distance or `leave()`), fight, or loot. They cannot pick, cannot start a parallel
conversation with the same NPC, and never receive control locks. Stale, duplicate or out-of-order `Lines` (step <= last)
are dropped. Story-critical conversations (L3 risk classifier) also require the party in the same area at start
(MASTER-PLAN "party spread"); that gate belongs to the quest-gate layer, not here.

### 4.3 Initiator is a client: round trip and latency budget
One exchange: `Pick` (client to host) -> begin script -> `Result(Begin)` back; `LineDone` -> end script ->
`Result(End)` back. The initiator UI **never blocks on Begin**: text and voice are local and take seconds to play,
which hides the RTT. The only blocking point is **End**: the next topic list is built after `Result(End)` so
`add_topics`, quest-var changes and say-once state are visible. Budget: `LineDone -> TopicsReady <= RTT + script
time`; target RTT p95 150 ms, script time < 20 ms. **Gate timeout 1500 ms**, then `TopicsReady{timedOut}` and the list
is built from local state (rebuilt when the late Result arrives). Heartbeat 15 s; host lease 300 s (initiator silent
-> `Closed{Timeout}`). Test `topic_gate_latency` asserts the gate holds and opens within RTT+slack at 200 ms RTT;
`gate_timeout` covers a lost uplink.

## 5. Edge cases (each has a test in `plugin/dialogue/tests/main.cpp`)

| Case | Rule | Test |
|---|---|---|
| Second player starts talking to the same NPC | `Denied(Busy, owner)`, no menu | double_start_order_a/b |
| Same player, second NPC | `Denied(AlreadyTalking)` | same_player_two_npcs |
| Spectator sends Pick/LineDone/EndReq | rejected, no script runs | non_initiator_pick |
| Duplicate / re-delivered Pick | begin script runs once | duplicate_pick |
| Skipped step | rejected | out_of_order_step |
| Initiator disconnects mid-line | begin already ran, so the **end script is flushed** (never half-applied); spectators get `Closed(Disconnect)`; locks released; NPC free at once | disconnect_mid_line |
| NPC dies | `Closed(NpcDead)`; later starts `Denied(NpcDead)` | npc_dies |
| Combat starts (NPC or initiator) | engine closes the menu; host sends `Closed(Combat)`; start denied while NPC in combat | combat_mid_dialogue |
| Initiator idle/AFK | lease expires -> `Closed(Timeout)` | lease_timeout |
| Esc/walk-away by initiator | `EndReq`; end script flushed if a line was running | lock_release |
| Lost `StartReq` ack | retransmit re-sends `Opened`, no second conversation | retransmit_start |
| Host shuts down | `Closed(HostShutdown)` to all | API `shutdown` |
| Late joiner mid-conversation | sees nothing until the next `Opened`; story state arrives via log replay | by design |

## 6. Dialogue control locks (generic rule)
`DisablePlayerControls` / `EnablePlayerControls` / `EnableFastTravel` appear in 136 INFOs. L5 captures them as a
control effect scoped to the conversation and the host sends `Control{conv, mask, locked}` to the **initiator only**
(spectators are never frozen by someone else's dialogue). The host tracks the mask as a **lease**: any bits still set
when the conversation closes (goodbye, disconnect, timeout, death) are released with `Control{locked=false}` before
`Closed`, so a dropped initiator can never leave a stuck player (test `control_lock_release`). A lock raised outside
a conversation belongs to the quest-script owner (L4/L5).

## 7. Special conversations
### 7.1 Companion recruitment (200 INFOs)
Recruit INFOs call `SetPlayerTeammate`, add the NPC to TeammateFaction/FollowerFaction, and bump `vNPCFollowers`
quest vars. The initiator is the **recruiter and owner** (MASTER-PLAN default). `SetPlayerTeammate` is captured as a
`CompanionOwner{npc, owner}` effect (L4). SUPERSEDED by section 11: the follower cap is per recruiting player (1 humanoid + 1
critter each, party total default 4) and the shared VNPCFollowers conditions are rewritten per owner; command topics are
owner-only.

### 7.2 Service menus (barter/repair/caravan/recipe: 148 INFOs + 5 `service` INFOs)
After the script calls `ShowBarterMenu`, the conversation enters a **service sub-session** owned by the initiator: the
NPC and its vendor container stay locked, the menu is local to the initiator, and each completed trade is submitted as
a host-validated inventory transaction (AddItem/RemoveItem to the initiator, transfer on the vendor container, caps as
`Caps001`). Vendor stock is host state, visible to all after commit. The Barter check uses the initiator's skill.
Dialogue only holds the lock and ends the sub-session on menu exit; trade messages belong to the inventory sync
(L4/L6). Design only in v1: not in the state machine.

### 7.3 Ending slides (315 INFOs)
`EndGame` and `SwapTexture OnScreenSlideRef` run in the host script; the slideshow is shown to **all players**
(`Recipient::all`); variants keyed on `GetPCIsSex` use the initiator. Credits/return-to-menu is party-wide. Ending
conversations are story-critical, so all players must be in range when the final INFO is picked.

### 7.4 Barks and non-menu INFOs
`combat` (2,300), `detection` (1,349), `radio` (769) and `misc` (39) are not menu sessions: barks are replicated by the
NPC puppets (Say events, L6) and need no lock or initiator. `conversation` INFOs (2,604, NPC-to-NPC) run on the host
with the NPCs; spectators see them as ordinary NPC speech. Only `topic`/`service` INFOs open a session.

## 8. Effects the dialogue layer needs from the sync core (proposals to L4)
- `SetScriptVar{owner(quest/ref), varName, value}` (whitelisted per quest/NPC): **4,554 INFOs** set `X.var` in result
  scripts; without it `GetQuestVariable`/`GetScriptVariable` conditions diverge (3,113 of those do nothing else).
- `SetFollower{actor, owner}`, `SetControl{mask, enable}` (conversation-scoped), `TransferItem{from, to, item, count}`,
  `SetTopicKnown{topic}` (AddTopic, 473 INFOs; default shared, per the TES3MP `shareTopics` flag).
- `SayTo`/`EVP`/`AddScriptPackage`/`StartCombat` are NPC-local AI side effects handled by NPC sync (L6), not effects.
- Dialogue wire payload: `DlgMsg{ uint32 session; bytes payload }` in `fnvcoop::Message`.

## 10. Transport adapter, engine glue and scenario specs (round 2)

- **Adapter** (`include/fnvcoop_dlg/adapter.h`): `DialogueNode` runs `HostDialogue`/`ClientDialogue` over L4's
  `ITransport`. Dialogue packets start with byte `0xD1`, L4 envelopes with `0xFC`, so the receiver demultiplexes on the
  first byte and `fnvcoop::Message` needs no change (a future `DlgMsg` wrapper would carry exactly `encode()`'s bytes).
  Requires a reliable ordered channel per peer; duplicates are tolerated. The host's own client side loops back
  in-process. Tests run it over `InMemoryNetwork` with latency and 60% duplication.
- **Engine glue** (`engine/`): `dialog_layout.h` (DialogMenu / TESTopicInfo / TESTopicInfoResponse as byte offsets),
  `info_reader.*` (menu view, flags, response chain, response-offset probe, voice path), `glue.h` (`EngineGlue` over the
  abstract `IEngineApi`: hook callbacks in, `Event`s out), `engine_xnvse.cpp` (the live `IEngineApi`, x86, xNVSE
  headers). Everything except the live API is unit-tested on a synthetic memory image and a mock engine.
  The TESTopicInfo response-list head is not named in the xNVSE headers: `probeResponseOffset` finds it structurally
  and cross-checks the first line against the ESM text from L3, so the first live run confirms or refutes the layout.
- **Spectator output** (v1): HUD message queue (`QueueUIMessage`, xNVSE) for banner and subtitle; direct writes to
  `HUDMainMenu::subtitlesArr` (JIP GameUI.h 0x198, `SubtitleData{text[0x104], startTime, displayTime, pos, priority}`)
  are the better target once an add-subtitle routine is verified. Voice playback needs a Say-like API (stub).
- **Scenario specs** (`plugin/dialogue/scenarios/`, generated by `tools/gen_m1_dialogue.py`, which aborts if any
  FormID, prompt, speaker or script text disagrees with the ESM data): Doc Mitchell word test with a parallel-start
  race and an initiator drop, Sunny accept/decline, Trudy Speech 25 with the initiator/host skill split (pass and
  fail-because-host-is-high), Cobb "Ringo's dead" with shared reputation, Ringo turning hostile (combat mid-dialogue).
  Actions: `setup`, `talk`, `parallel`, `pick`, `pick_info`, `pick_nowait`, `leave`, `event`, `assert`; each step has an
  `expect` block (`spectator_lines`, `script_runs`, `script_var`, `stage`, `objective_completed`, `xp_each_player`,
  `reputation_shared`, `denied`, `closed`, `lock_released`, `converged`). The L2 runner maps `talk/pick/leave` onto
  nvbridge calls that drive the glue's `onTalkRequested/onInfoChosen/onMenuClosedByPlayer`, and `event` onto host-side
  triggers.

## 11. Companions and host-run menus (round 3, owner decisions 2026-10-16)

### 11.1 Owner-only companion commands (exception to initiator-picks)
Data: `tools/gen_companion_topics.py` classifies the player-prompted topics of the three follower quests (VNPCFollowers,
NVDLC01Followers, NVDLC02FollowersQuest) into `data/companion_topics.txt`: 22 `ownerOnly` (wait, stay close, follow
distance, trade equipment, tactics, combat style, "go back to the Lucky 38", "part ways", ...) and 1 `recruit`
("I think we should travel together", any player may take an unowned companion; recognised by `SetPlayerTeammate 1`).
Barks and `PLAYER ...` event topics are NPC speech, not topics.
- Host: for a conversation with a registered companion, `Offer` is relayed to spectators **without** command topics; a
  `Pick` of a command topic is accepted only from the companion's owner (else rejected, no script runs); the owner's command
  `Lines` are **not** shown to spectators ("only the recruiting player sees them"); results still run once on the host.
- Glue: `EngineGlue::onTopicsBuilt` hides the command topics from a non-owner's real DialogMenu (`hideTopic`).
- Takeover: if a non-owner is merely chatting with the companion, the owner's `StartReq` closes that conversation
  (`Closed{Preempted}`) and takes the NPC; a non-owner still gets `Busy` against the owner. The old chatter becomes a
  spectator of the owner's conversation.
- Dismiss: only the owner (`dismiss`); when an owner leaves the session the registry releases their companions.

### 11.2 Per-owner rewrite of the shared follower variables
VNPCFollowers keeps one set of vars for a one-player game: `bHumanoidInParty` (47), `bCritterInParty` (48),
`bPlayerHasFollower` (18), `nCurrentFollowers` (19) and Hired/Fired/Available per companion (e.g. Boone 6/7/8). The
indices are checked against the quest script by `gen_companion_topics.py --check`. Used by 87 recruit/dismiss INFOs and
every follower-gated condition.
- Every machine owns a `CompanionRegistry` (ownership comes from the replicated `CompanionOwner` effect). Rules:
  per-owner cap (1 humanoid + 1 critter each), party total (default 4), ownership exclusive.
- **Read path** (adapter): `DialogueNode::viewQuestVar(quest, idx, real)` / `viewQuestVarFor(..., slot, ...)` return the
  value as seen by the evaluating player: humanoid/critter-in-party = "this player's slot is full or the party total is
  reached" (so the recruit topic disappears exactly when this player cannot recruit), has-follower and current-followers
  = this player's count, `Hired(c)` = "c is mine", `Available(c)` = 0 when c is hired by someone else. Quests/indices
  without a rule are returned unchanged. The engine hook for `GetQuestVariable` (conditions on the initiator, and
  result-script reads on the host with `ResultRequest.initiator`) calls it.
- **Write path**: recruit/dismiss INFOs still `set VNPCFollowers.bHumanoidInParty to 1/0`; the shared variable is
  defined as the party AGGREGATE (any owner has a humanoid, total followers) so host-side quest scripts (waiting-
  companion auto-fire, achievements) stay valid. `reconcile()` returns the `SetScriptVar` corrections after
  `CompanionOwner` effects were applied; the registry, not the script's literal write, is the truth.
- Host validation: `recruit()` returns Ok / AlreadyYours / OwnedByOther / OwnerCap* / PartyFull / Unknown.

### 11.3 Menus raised by host-run scripts
Scripts run on the host, so `ShowMessage` with buttons, `ShowBarterMenu`, repair/recipe/caravan menus, the companion wheel
and terminals would pop up on the host player. Routing (all inside the 0xD1 channel, message types 13-16; proposed to
L4 who owns the transport, no change to `fnvcoop::Message`):

| Message | Direction | Content |
|---|---|---|
| `MenuRequest` | host -> origin only | menu id, kind, origin, text, buttons (<=10), default choice, subject (NPC/terminal) |
| `MenuReply` | origin -> host | menu id, choice (button index, -1 = cancel/closed) |
| `MenuNotice` | host -> everyone else | menu id, kind, origin, text (<=160): read-only banner, no buttons |
| `MenuClosed` | host -> everyone | menu id, choice, timedOut, label of the chosen button |

- **Origin** is the player whose context the script runs in: `ResultRequest.initiator` for dialogue scripts, the activator
  for terminals/containers; never "the host player" by default. L5 supplies it from the capture context.
- The engine glue suspends the script (ShowMessageBox pending state) and calls `HostDialogue::raiseMenu`; the resume
  callback fires **exactly once** per menu: on the origin's reply, on the default choice after a timeout (message boxes
  120 s, service menus 600 s), or on the default if the origin disconnects or is already gone (then no UI anywhere).
- The host validates the reply: sender == origin, choice in [-1, buttons-1] (service menus without buttons: -1/0),
  duplicates ignored. A raised menu counts as activity for the origin's conversation lease.
- Origin = host player is the ordinary local case (loopback). Spectators never receive the interactive request.
- Client events: `MenuShow` (origin opens the real menu and answers with `menuReply`), `MenuNoticeShow`, `MenuEnd`
  (origin force-closes its UI if `timedOut`). Glue API: `openMenu`, `closeMenu`, `menuNotice`.
- Service menus (barter etc.) reuse the same pair: the "reply" is the menu being closed; item transfers travel as effects.

## 12. Forced dialogue: SayTo / StartConversation / Say (round 4)

About 1,600 calls game-wide (42 in act2) run on the host from quest, object and INFO scripts and aim a line or a whole
conversation at "the player". Two different things hide behind the names:

| Script call | Meaning | Interactive | Our message |
|---|---|---|---|
| `NPC.SayTo player topic` / `Say topic` | the NPC speaks one scripted INFO, no choices | no | `ForcedLines` (host -> all) |
| `NPC.StartConversation player topic` | the NPC opens the real DialogMenu on that topic for the player | yes | `Opened` (all) + `ForcedStart` (actor only) |

**Who is "the player"?** (1) the acting player: `ResultRequest.initiator` inside an INFO, the activator / trigger entrant /
hit target for object scripts (L5 supplies the slot from the capture context); (2) no acting player (quest timers, scene
stages): a **party scene**, `target = kNoPlayer`: SayTo is delivered with `scene` set so every player sees it; StartConversation needs
exactly one actor, chosen by `HostDialogue::setActorChooser` (production: the nearest connected player to the NPC, tie = lowest
slot; default: lowest connected slot). Cutscene-class quests (L3 tier T4 / rule `SetPlayerControls scope=all`) mark `scene` explicitly.

**SayTo (one-way).** The host script runs natively, so the host engine selects the INFO (conditions evaluated with the target's
facts, same substitution as 3.2), runs its begin/end result scripts under capture and speaks it on the host NPC. The hook then calls
`forcedSay(npc, topic, info, lines, target, scene, txnSeq)`; every machine receives the resolved lines. Receivers only play them:
the target and scene viewers always; others only if they could hear the NPC (`IEngineApi::npcAudible`, distance/line of sight).
No condition is evaluated on receivers, so a spectator never sees a different line than the target. Duplicate deliveries are dropped
by id; `txnSeq` ties the line to the capture txn of its scripts; late joiners do not replay it (event, `Context::catchUp`).

**StartConversation (interactive).** `forceConversation(npc, topic, target)` takes the NPC lock, closes whatever the NPC or the actor
was doing (`Closed{Preempted}`: a scripted conversation is story and wins over idle chatting), broadcasts `Opened`, and sends
`ForcedStart` to the actor only. The actor's glue calls the native `StartConversation` on that machine (the host vetoes the script
command's own menu unless the host player is the actor); from then on it is an ordinary initiator conversation (4.x): the actor
picks, everybody else spectates, result scripts run on the host. If the target left, the actor falls back through the chooser; if
nobody is connected or the NPC is dead the call is a no-op.

**Capture.** L5 records `ForcedDialogue{npc, topic, initiator}` (L4 effect, an event) for the log/replay and calls the two APIs above.
`initiator` carries the resolved actor slot (0xFF = party scene SayTo). Proposed additive fields for L4: `kind` (say/conversation)
and `scene`; the dialogue layer does not depend on them.

**ModalChoice alignment (act2 rule kind).** `decider: initiator` = the menu origin (section 11.3). `consent: party` =
`HostDialogue::ModalOptions{partyConsent, confirmButton, declineChoice, timeoutMs}`: when the origin picks the confirm button,
every other connected player gets a consent prompt (`MenuRequest.consent`, buttons "Ready"/"Not yet"); all ready -> the script
resumes with the confirm choice, any "Not yet" or the window running out -> `declineChoice` (timeout flagged), a player who leaves
stops counting, non-required players and out-of-range values are rejected. The origin choosing any other button resumes at once.
`timeout_s` maps to `timeoutMs`. Rule kind `GuestEventRelay` (guest-originated script events) is not a dialogue concern.

## 9. Open questions
1. Per-player vs party follower cap (7.1).
2. Whether say-once ("already said") state is shared: decided D11 (shared, except Speech-XP and trainer lines). The engine's own marks (scripted SayTo of a forced stage chain) reach the other copies through the `TESTopicInfo::MarkSaid` detour and the `SayOnceMark` message (docs/tools/story.md, "The fix").
3. Residual non-captured NPC object-script writes (3.3).
4. Exact `DialogMenu` close/force-close API (hook request in `internal lane notes`).
