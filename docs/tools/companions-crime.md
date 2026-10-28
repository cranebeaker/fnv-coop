# companions-crime: followers, crime and kill rewards (FULL-SYNC-PLAN 3.4, 3.8, 3.10)

Code: `synccore/social/` (game-free rules, wire), `plugin/companions/` (per-machine wiring `SocialSync`, mux runtime, the x86 module).
Wire: `docs/protocol.md` section 18 (first byte `0x5C`). Status 2026-10-21: **unit-tested game-free only. NOTHING here has run in a game.** The module compiles and links into
nvcoop.dll; it does nothing until `nvcoop.ini [companions] enabled=1` AND the live layer calls `glueFrame` (see "Wiring that belongs to other lanes").

## Run it (G34 set pieces added 2026-10-27: twins 884-898 and 900 in synccore/social, 933-943 in plugin/companions)
- `synccore\social\run_tests.cmd` : the rules (companions, crime, rewards, wire). Real run + 66 runtime mutation twins (`FNVCOOP_MUT=801..883`, nothing recompiled). Prints `ALL OK`.
- `plugin\companions\run_tests.cmd` : `SocialSync` on a cluster of 3 machines (scripted worlds, recording fake engines, an in-memory star network, a committer that plays the
  story session). Real run + 28 runtime twins (`901..932`). Prints `ALL OK`.
- `python -I plugin/companions/proof/social_proof.py --selftest` : the in-game proof checker, game-free, with twins.
- All three are in `tools/test_all.py`.

## What it does

### Companions (3.10)
| Rule | Where |
|---|---|
| A companion belongs to ONE recruiter wherever it goes. A companion that dies frees its recruiter's follower slot (the host releases it). The host learns recruit / dismiss from the captured `CompanionOwner` effect, keeps the `CompanionBook`, and sends the table (`MRoster`) to everybody. | `companions.h` `CompanionBook`, `social_sync.cpp` `onEffect` |
| The follower exception: every `CompanionOwner` effect the host learns of (and every release the module itself commits) is fed to `plugin/actors` through `actors::engine::onEffect` (`feedActors`), so the recruiter's machine owns the NPC. | `Config::feedActors` |
| Follower caps per player and kind (default 1 humanoid + 1 critter, 4 for the party). The session already drops a recruit beyond them at commit time; the host's book does not learn what was dropped, and the stray engine flag comes off. | `CompanionBook::check`, `onEffect` |
| Only my companions may be flagged as teammates in MY engine (the engine's follower AI follows the local player). A flag for somebody else's NPC, or for an NPC the roster does not know, is taken off (`SetPlayerTeammate 0`); my own, loaded but unflagged, is flagged. Nothing is touched before the host's picture arrived; the same NPC is not flipped twice inside 3 s. | `reconcileTeammates` |
| The host's own followers from before the session (a save with Boone in the party) are adopted once, at the first host tick. | `seedHostCompanions` |
| Commands are owner-only. The owner's machine refuses a stranger's command at once; the host refuses a forged one (`MCmdNack`), once per id, rate limited. A command that changes state (wait, follow, stance, range) carries a revision; an older revision never overwrites a newer one. A dismiss command releases the companion through the story session. | `CommandJudge`, `issueCommand`, `hCmd` |
| The companion wheel opens only for the owner: `allowActivation(who, npc)` is false for somebody else's companion (host setting `foreign=allow` lifts it). | `gateActivation` |
| Companion inventory per owner: the owner's machine polls its companion's inventory changes (engine `ExtraContainerChanges` countDelta, i.e. changes against the base container, so UI moves, scripts and the NPC's own pickups all count) and sends signed deltas. The host accepts a delta from the owner only, once per txn, within range; other machines apply it to their copy once; the owner ignores the echo; a joiner gets the table and derives what to change. | `CompanionInventory`, `sampleInventories` |
| Companions in combat: a hit inside the party (player to companion, companion to companion, companion to player) is neutralised unless friendly fire is on; enemies and companions still hurt each other. A party member is never a VATS target. A companion's kill credits its recruiter (XP, karma, crime). | `companionHitPolicy`, `vatsTargetAllowed`, `responsiblePlayer` |

### Companion set pieces (campaign gap G34, owner decision D2: every player has their own companions, ED-E belongs to the whole party)
Nothing here has run in a game. Switches in `[companions]`: `partyowned=1` `swapguard=1` `escort=1` `escortgrace=4000` `escortcooldown=12000` `escortfar=3500`. Data: `plugin/companions/data/companion_sets.txt`
(embedded in the DLL as `companion_sets_default.h`; a file `nvcoop_companion_sets.txt` next to the DLL replaces it); regenerate both with `python -I plugin/companions/proof/gen_sets.py`.
| Piece | Rule | Where |
|---|---|---|
| Party companion (ED-E) | A recruit that the rules engine names owner=party (`HostSession::isPartyCompanion`, installed by the `RulesEngine`, read through the runtime's host) or the data file's `party` lines marks the book entry `party`. It uses no follower slot or party-total slot, any player may command it (host judge), open its wheel and change its inventory, any player may dismiss it, and the `party` flag travels in the roster (`Companion` flags byte). The recruiter slot stays on the wire: that machine simulates ED-E and writes his engine state; a guest's order for him goes through the host (the host applies it when it simulates him). The host re-asks the oracle every second, so the order of the capture sink and the rules engine's filter does not matter. | `companions.cpp` `wantsParty` `refreshParty`, `social_sync.cpp` `refreshParty` |
| Recruiter leaves | Ordinary companions are released as before; party companions are handed to the host (`handParty`) and the hand-over is committed (`party-companion-handed`) so the session, the dialogue registry and a joiner's baseline name the same owner. | `social_sync.cpp` `handPartyCompanions` |
| Single slot groups | `group` lines (Dead Money: Dean, Christine, Dog; Honest Hearts: Follows-Chalk, Waking Cloud): a player gets one member at a time. A recruit that would give a player a second member is turned back and its stray flag taken off (repair: a dismiss is committed). | `slots.cpp` `vetCompanionTxn`, `SocialSync::onTxn` |
| Swap guard | The Waking Cloud hire INFO fires "the hired Follows-Chalk" off the shared flag. When the dismissed companion belongs to ANOTHER player than the recruiter in the same transaction, the dismissal is not learnt and a re-recruit for the true owner is committed 250 ms later (`companion-repair`). A player's own swap is the vanilla behaviour and passes. What cannot be undone from outside: the fire script's side effects (teleport home, gear moved to the firing player). The real prevention is the per-player view of `bFollowsChalkHired` in the hire INFO (dialogue registry `viewVar`, see docs/handover/P7-requests.md). | same |
| Hired sets, Fort entry / exit, Dam start | The book is the per-owner hired set. The scripts that move or disable every hired companion by name (vCCmovetothefortSCRIPT, YesManDeadSCRIPT and 215 more) read the shared hired flags, which hold the party aggregate: they move every player's companions. `glueViewSlot(script, acting)` says which view a script gets: `kViewParty` (0xFE, use the shared value) for the readers listed as `view-party`, the acting player for everything else (hire and fire scripts write the flags). | `slots.cpp` `viewSlotFor` |
| Dead Money `rCurrentFollower` | `glueCurrentFollower(slot, "deadmoney", out)` answers with the companion that player has in the group (the shared ref variable keeps the last hire of anybody). | `slots.cpp` `currentMember` |
| Escort | The recruiter's machine watches its own player: a companion it simulates that is loaded, alive, not waiting (the book's mode or the companion script's `Waiting`) and more than 3500 units from its player for 4 s is moved to the player (`MoveTo player`), at most once per 12 s. Covers PartyWarp, the Fort and Dam moves and any scripted `player.MoveTo` for companions the script did not move. | `escort.cpp`, `SocialSync::pollEscort` |
| Host companions at the start | A save with Boone in the party: the adoption is committed (`seed-companions`) so the story session, the dialogue registry (Boone's "hired" flag for the host) and joiners know it. | `SocialSync::seedHostCompanions` |
| Roster listener | `glueSetRosterListener` receives the whole roster (owner, kind, party) whenever it changed on this machine: the session wiring feeds the dialogue registry from it. | `Config::onRoster` |
Known limits: a non-owner's Summon command for a party companion is judged and ignored (nothing moves him to the commander); gear moved by a fire script to the wrong player is not returned; the
Arcade enslave INFO (164B1C) removes Arcade's `Hired` flag and the follower counters through the acting player's view, which only works once the view is wired.

### Crime, alarms, bounties, hostility (3.4)
New Vegas has **no crime gold** (read from the engine's command table: the crime state is per-faction flags `SetPCFactionAttack`, `SetPCFactionMurder`, `SetPCEnemyOfFaction`, plus
`SetEnemy` / `SetAlly`, `SendAssaultAlarm`, `StartCombat`). So the "bounty" here is a co-op WANTED SCORE per faction, shared by the whole party (the party is one Courier): crimes add
points (ours: assault 25, murder 100, theft 15, pickpocket 30, trespass 5, break-in 20, vandalism 10), points switch the engine's flags (attack at once, murder on a witnessed murder,
enemy at 100 points or on a witnessed murder), a pardon clears it. The numbers are `CrimeRules` defaults, not read from the game.
- **Host-authoritative.** Anybody reports (`MCrime`); the host decides once (`CrimeJudge`); everybody applies the host's result once (revisioned `MBounty`, id-deduplicated
  `MHostility`, id-deduplicated `MAlarm`). A client's own copy of those messages is ignored.
- **Detection by any player counts.** Each NPC's owner reports what its NPCs see (`MDetect`, re-sent every 2 s while it lasts, "lost" when it ends). A crime is witnessed by an NPC of the
  faction that cares, in the cell, seen within 6 s, whoever's machine reported the sighting. A report about an NPC the sender does not own is rejected. A surviving victim of an assault
  or pickpocket is a witness of it. Unwitnessed crimes do nothing.
- **Stealing / trespass by a guest through the host.** `onGuestInteraction(slot, pickup, target)` reads the target's ownership (`ExtraOwnership` of the reference, else of its cell), the
  lock, and the faction that cares, and turns the guest's intent into theft / break-in / trespass charged to the GUEST (`classifyInteraction`). The local player's own trespass is polled
  (`IsTrespassing`, once per cell per 30 s).
- **A burst is one crime** (an automatic weapon is one assault inside 3 s). Hurting something already hostile, the party's own faction, or a faction nobody knows is no crime.
- **Alarms**: one order per (faction, cell) while one is in force; every machine alerts the NPCs it simulates (`StartCombat <criminal>` on them), so no NPC reacts twice.
- **Reputation**: infamy per witnessed crime (murder 2, pickpocket 1) is a committed `ModReputation` to the shared mask; the host applies its own part directly.

### Kill XP and karma (3.8)
A kill is reported by whoever knows it (`reportKill`) and settled ONCE on the host by death id (`RewardLedger`), by the host's `SocialSettings`:
- XP: **killing blow** (a companion's kill credits its recruiter; NPC / environment kills pay nobody) or **party share** (`xpmode=1`), among the players present (`xpscope`: everybody
  connected, same cell, or within `xpradius`); the split reuses combat-core's `attributeKillXp`.
- Karma: by the victim's karma class through our own table (placeholders, see below); killing a hostile never costs karma; shared by SharePolicy (`sharekarma`).
- Delivery: committed effects with explicit masks (`ModActorValue` experience, `ModKarma`), so the story channel makes them exactly-once, replays them to a rejoiner and survives a lossy
  link; the host's own part is applied directly. **A slot whose engine already paid natively** (`nativeMask`: the killer's own machine killed a victim it simulates) is not paid again; a
  client can claim that for its own slot only.
- Settings travel (`MSettings`, revisioned): the host's choices are the only ones that count on every machine.

## Engine facts used (all from the exe's own tables or xNVSE headers; nothing invented)
| Piece | Source |
|---|---|
| `SetPlayerTeammate <int>` (on the NPC), `SetPCFactionAttack` / `SetPCFactionMurder` / `SetPCEnemyOfFaction <faction> <int>`, `ClearFactionPlayerEnemyFlag <faction>`, `StartCombat <actor>`, `RewardXP <int>`, `RewardKarma <int>` | parameter lists read from the command table in `<reference exe>` (`plugin/hooks/tools/re/command_table.json`) |
| Condition evaluators `GetDetected` 0x59D6E0 (calling ref = observer, param = the actor) and `IsTrespassing` 0x5A1900, signature cdecl `bool(thisObj, p1, p2, double*)`, prologue `55 8B EC` checked before every call | the 10th field of the same table; the call shape is `plugin/hooks_world/world_query.cpp`'s |
| Teammate list: `PlayerCharacter::teammates` (tList at +0x5FC); actor flag +0x18D | xNVSE `GameObjects.h`, `plugin/hooks/engine_hooks.cpp` `h_Teammate` |
| Companion stance = the actor value `Aggression` 0 / 1 / 2 (`<ref>.SetAV Aggression n`); wait / follow = the companion's own script variable `Waiting` (0 / 1) | the game's own companion scripts (`out/questdata/scripts.json.gz`: 47 sites set `Waiting`, 31 set `Aggression`) |
| Ownership: `ExtraOwnership` (kExtraData_Ownership) of a reference or its cell; lock: `ExtraLock` flags bit 0; inventory: `ExtraContainerChanges` entries `countDelta`; faction list: `TESActorBase::baseData.factionList` | xNVSE headers, read under SEH |

## TODO-RE and honest limits
- **Crime faction of an NPC**: the NPC's own "crime faction" field is not located; the FIRST faction in its faction list stands in (`FirstFactionId`).
- **Karma and reputation amounts for a kill / crime**: the engine's own numbers (GMSTs `fKarmaMod*`, docs/02-engine-re.md) are not read; `KarmaTable` and `CrimeRules::infamy` are ours.
- **Companion "range" (close / medium / far)** has no engine mapping; it is shared state only (counted as `stateUnmapped`). Stance and wait / follow are mapped as above, unverified in game.
- **Who calls the lane inputs**: the wheel's activation, the kill report, the witnessed-attack report and the VATS target filter need one call each from lanes that this task did not
  edit (list below). Until then only the dev bridge (`social.dev`) and the host-side intents drive them.
- **Detection of a puppet**: `GetDetected` on a puppet (a restrained NPC copy of a remote player) treats it as an ordinary actor: sneak and chameleon of the REAL player are not
  considered for a player on another machine. TODO-RE if that matters in play tests.
- The numbers (windows, cooldowns, points) are defaults, not tuned in a game.
- Overlap, by design: plugin/dialogue's `CompanionRegistry` (L8) rewrites the shared follower quest variables per owner for dialogue conditions; this module is the network and
  engine side. Both are fed by the same `CompanionOwner` facts. plugin/combat has its own basic crime-to-hostility path (`[combat] crime=1`) and kill XP; with both on, SetEnemy is applied
  twice (idempotent) and kill XP must be paid by ONE of them: `glueXpHandled()` tells combat to stand down.

## Wiring that belongs to other lanes (each one short call)
| Lane | Call |
|---|---|
| dll-wiring: frame (`plugin/coop/coop_module.cpp`) | **done**: `glueFrame(g_rt)` once per frame after `livewire::glueFrame`, `glueFrame(nullptr)` at shutdown. This is the ONE call site. |
| dll-wiring: `[live] social=1` (`plugin/coop/live/live_glue.cpp`) | **done**, default off, needs `actors=1`: a `SocialService` (the live status shows it) and, with `avatars=1`, `setPlayerRefMap` over the avatar layer's puppets. It does not tick the module (that would be a second `glueFrame` per frame); `[companions] enabled=1` is what starts the module |
| guest actions (`plugin/intents`, `plugin/coop/coop_module.cpp`) | **done**: before a guest's Activate / DialogueStart / ActivateRef runs `glueAllowActivation(slot, target)` (false = `Result::Denied`, nothing runs); after a guest's Activate / PickUp / ContainerOpen ran `glueGuestInteraction(slot, pickup, target)`. `HostDeps::allowActivation` / `interacted` carry it (game-free, tested); the legacy v1 executor in coop_module.cpp calls the glue directly when the intents module is off |
| combat (`plugin/combat`) | **done** through `Config::social` (`SocialHooks`, filled in combat_module.cpp): death path `glueReportKill` from the declarer only and combat's own `awardXp` stands down while `glueXpHandled()` (a native kill sets `nativeMask`); attacks (engine hit gate, forwarded hits, blast planning) and the killing blow `glueReportCrime` from the machine that simulates the attacker (burst throttled to one per 3 s per criminal, kind and faction; a companion's attack names its recruiter); hit gate and blast planner: `glueHitPolicy(...) == Neutralize` means `GateAction::Neutralize` / no planned hit |
| vats (`plugin/vats`) | before offering a target: `glueVatsTargetAllowed(targetRef)` |
| avatars | `nvcoop::social::setPlayerRefMap(slotOfRef, refOfSlot)` (default: the NV:MP ghost map, like combat) |

## nvcoop.ini `[companions]` (default OFF)
`enabled=0` `companions=1` `crime=1` `rewards=1` `foreign=deny|allow` `xpmode=0|1` (0 killing blow, 1 party share) `xpscope=0|1|2` (all connected, same cell, near) `xpradius=6000`
`karma=1` (kills move karma) `sharekarma=1` `sharerep=1` `friendlyfire=0` `humanoid=1` `critter=1` `party=4` `feedactors=1` `dev=1` (adds `social.dev`). The host's values are sent to
every machine; a client's own values for these are ignored.
Bridge: `social.status` (counters, digests), `social.dev {op}` with ops `crime {kind, faction, victimNet, cell, dead, native}`, `kill {id, victimNet, xp, karma, hostile, killer, recruiter,
native}`, `forgive {faction}`, `cmd {kind, npc, arg}`, `interact {slot, pickup, target}`, `settings {xpmode, scope, friendly, foreign, crime}`.

## Mutation twins
G34 (2026-10-27): rules 884 party companion uses a follower slot, 885 only the recruiter commands him, 886 wheel for the recruiter only, 887 inventory for the recruiter only, 888 leaver takes him along,
889 hand-over not done, 899 another player's current follower, 890 party flag out of the digest, 891 only the recruiter dismisses him, 892 swap guard off, 893 single slot off, 894 own swap reverted, 895 own swap refused, 896 waiting companion
escorted, 897 no grace, 898 no cooldown, 900 movers read the acting player. Wiring 933 host companions not committed, 934 transaction judgement skipped, 935 repair never committed, 936 oracle ignored,
937 party companion released with the leaver, 938 hand-over not committed, 939 escort off, 940 every machine escorts, 941 commander writes his own copy, 942 host ignores a guest's order for its party
companion, 943 roster listener not called.
Rules (`synccore/social`, runtime `FNV_MUT`): 801-829 companions (caps, take-over, dismiss, leaver, revisions, commands, wheel, inventory, teammates, party), 831-854 crime (judge, witness
window, burst, murder, alarms, pardon, decay, bounty revisions, guest interactions, detections), 861-870 rewards (once, native, karma, scope, effects, reputation sharing), 880-883 wire.
Wiring (`plugin/companions`): 901 client believes another client, 902 author claim believed, 903 detection for another machine's NPC, 904 kill settled per report, 905 host paid twice,
906 others never paid, 907 alarm applied twice, 909 dropped recruit learnt, 910 leaver's companions kept, 911 unflag before the host's picture, 912 no cooldown, 913 inventory never
reported, 914 owner applies the echo, 915 mirror applies a delta twice, 916 host obeys a stranger, 917 no refusal sent, 919 trespass every poll, 920 sightings not refreshed, 921 digest
mismatch not healed, 922 no join state, 923 actor layer not fed, 924 settings not sent, 925 wanted table not applied to the joiner's engine, 926 guest's theft charged to the host, 927
foreign wheel setting ignored, 928 companion treated as anybody, 931 a client claims another slot was paid, 932 a dead companion keeps its slot. Unused: 847 908 918 929 930.

## In-game proof (for the helper; the code-only agents did not run any of it)
1. Two or three copies in one session, `nvcoop.ini` `[live] actors=1 avatars=1 combat=1` (+ `social=1` once dll-wiring added it), `[companions] enabled=1 dev=1 xpmode=1`.
   `nvbridge social.status` on each: `running` true, `synced` true on the guests, digests equal.
2. `python -I plugin/companions/proof/social_proof.py snapshot --out out/social/before.json --instances 0,1,2`.
3. + 4. Companions and crime, scripted (no human): `python -I plugin/companions/proof/social_proof.py drive --faction FalloutNV.esm:0A46E7 --instances 0,1,2` (0A46E7 = NCRFactionNV).
   Without `--npc` it recruits Boone (`--boone`, default FalloutNV.esm:096BCE) for guest 1: on the host `enable`, `moveto player`, then bridge `exec.ref {ref:<boone>, cmd:"SetPlayerTeammate 1", scope:true,
   acting:{ghost:<guest 1's puppet from coop.status playerRefs.slots>, slot:1}}`, which the capture hook (`h_Teammate`) records as a `CompanionOwner` effect for slot 1. It asserts `companions` 1 on every
   instance, `social.dev {op:"cmd", kind:1, npc:<boone>}` on guest 1 (wait): `stateApplied` +1 there; the same command on guest 2 refused (`cmdRefused` +1, `stateApplied` unchanged); `teammateUnset` on the host is
   printed as info. Without `--victim-net` it places an NCR trooper (`--trooper-base`, default eldoradoNCRTrooper1 FalloutNV.esm:0E7E21) next to guest 1's puppet with `placeatme` run by the host, finds the
   new reference with `dev.refsOf` (before / after) and reads its net id with `coop.owner`, then drives the crime and kill and compares the engine flags (`GetPCFactionAttack` ...) on every instance; guards that
   the other machines simulate turn on guest 1. `--npc` / `--victim-net` keep the old manual drive. Game-free: `--selftest` runs the whole setup against a fake bridge with 7 broken-setup twins
   (recruit never recorded, recorded for the wrong slot, guest 2 allowed to command, trooper not created, no net id, no puppet, flags differ).
5. `snapshot --out out/social/after.json`, then `check out/social/before.json out/social/after.json --crimes 1 --kills 1 --alarms 1 --xp-grants 1`: every line must say PASS.
6. Look for: kill XP arriving once on the killer's screen, karma moving once on every screen, no HUD spam, guards of the wanted faction reacting once. If `engine_evalFaults` is not 0,
   the `GetDetected` / `IsTrespassing` calling convention is wrong: set `[companions] crime=0` and report it.
