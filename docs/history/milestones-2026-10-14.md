# nv-rs milestones

Updated 2026-10-06. Priority tracker and session handoff.

## Baseline

README and previous research notes report readers, rendering, streaming,
walking, actors, AI, combat, dialogue, quests, inventory, menus, audio,
V.A.T.S. and custom saves. Many corresponding source modules exist.
The first foundations batch built and tested both workspaces and ran the
opening to Doc's instruction to use the vigor tester. Missing cutscene and
menu behavior prevents calling that a faithful opening. Evidence remains in
ENGINE_REFERENCE.md and %USERPROFILE%\nv-re\findings.
The clean public source history was prepared on 2026-10-03. Earlier private
history is retained separately; do not publish its build outputs.

## Ordered gates

| Milestone | Completion gate | Status |
| --- | --- | --- |
| M1: Opening and persistent world | Retail opening movie and scripted wakeup, animations/movement, Doc Mitchell's creation sequence, exit into Goodsprings, talk/interact, save, restart and reload. Player, NPC, quest, inventory and reference state survives. Compare with the original game. | Active; initial live run and package-action reader complete; choreography and full route pending. |
| M2: Core gameplay loop | Sunny's tutorial and a representative Goodsprings quest branch through their own scripts: movement, weapons/reloads, damage/death, AI, dialogue, loot, trade and progression. Save/reload at intermediate stages. Verify melee and V.A.T.S.; track other weapon classes explicitly. | Partial implementation reported; acceptance pending. Crafting (`ShowRecipeMenu`, [CRAFTING.md](CRAFTING.md)): rules and the game's menu done; tutorial help and item card remain. |
| M3: Base-game systems and campaign | Coverage matrix for quests, actor/creature types, weapon classes, effects, factions/crime, companions, travel and menus. Representative routes and ultimately campaign completion, with evidence and regression tests for blockers. | Inventory and acceptance routes needed. Terminals ([HACKING.md](HACKING.md), [TERMINALS.md](TERMINALS.md)): who gets in, hacking and the terminal's own menu done; rendered terminals (the PC default: both menus on the screen of `TerminalInterface01.NIF` with its camera, lights, screen effect, fade, the pointer through the screen and the power button) done, not yet compared with the running game. Menus' own fades ([MENU_FADES.md](MENU_FADES.md)): every game menu fades in and out as the interface runs it (`menufade`, the fade list, input only once shown), the terminal's 0.75 s on leaving; not compared with the running game. Item scripts ([ITEM_SCRIPTS.md](ITEM_SCRIPTS.md)): per-item scripts with `OnAdd`, `OnEquip`, `OnUnequip`, `GameMode`, `RemoveMe`, `DropMe`, `OnDrop` and `Drop` into the world, and the Pip-Boy's Drop (the pad's X: its refusals and "How many?"). Repairing ([REPAIR.md](REPAIR.md)): merchants' repair services (`ShowRepairMenu`) and the Pip-Boy's repair screen done, NPC skills now with their offsets; the player's armour wears from hits and its DT/DR follows its condition. Companions ([COMPANIONS.md](COMPANIONS.md)): trading things with them (`OpenTeammateContainer`) and their wheel of orders (`CompanionWheelMenu`, with the UI's `radial` tiles) done; since 2026-10-07 also the engine's teammate rules: coming along through doors, `MoveTo` and fast travel, Nerve, essential outside Hardcore with time down paused while the player fights, ammunition use, no fall damage, sneaking with the player; their armour picked after trading, the wheel's stick and subtitles, the Back Up hook; since 2026-10-07 (later) also the weapon they pick to hold after trading (`GetBestWeapon` by the traced damage per second, `00645380`, with the player's attack animations as the game reads them; the combat style from the wheel's Ranged/Melee switch; creatures such as ED-E from their `LNAM` weapon list) and the Pip-Boy ITEMS card's DPS and effects text (`00406620`); the Back Up package and `InitDefaultWorn`'s other callers remain. Caravan ([CARAVAN.md](CARAVAN.md)): the cards, the player's collection (`AddCardToPlayer`) and the rules (placing, values, jacks and jokers, drawing, the end) the opponent's AI (`ProcessAI`, with the exe's own sort), the bet and results, the menu's state machine (`world::caravan::menu`), its tiles (`ui::menus::caravan`) and the 3D table (`cellview::caravan`, the viewer's `caravan_table`) done: playable in the viewer from `ShowCaravanMenu`; its tutorials shown once each; the table keeps the file's 16:9 frustum on any screen, the deck screen's mouse drag follows the game's (`fMouseHeldTime`) and the pad's left stick works; the payment's inventory order remains. Tutorial messages ([TUTORIALS.md](TUTORIALS.md)): the once-only tutorial manager (`world::tutorial`, saved) and its help box (`ui::menus::tutorial`), hooked for Caravan, crafting, hacking, terminals, lockpicking (with the controls' key names in text), V.A.T.S., the Pip-Boy's ITEMS tabs and reputation titles; the start menu's Help (the help manual's pages) and `ShowTutorialMenu`. The HUD's corner messages show the game's own picture for each message ([HUD_MESSAGES.md](HUD_MESSAGES.md): scripts' messages, the casinos', locks and keys ("Unlocked with ..."), refusals, condition, reputation, karma, hardcore needs); the HUD's quest text shows a discovered place, quests added, completed or failed (the name a letter at a time) and objective lines with their boxes, as the game's `QuestUpdateManager` and HUD do. Help text written in HTML (`HelpHealingLimbs`) is laid out by the game's HTML layout ([HTML_TEXT.md](HTML_TEXT.md)). Casinos ([CASINO.md](CASINO.md)): the record, the player's winnings, levels and comps quest, refusals and the anti-cheat lock, settling chips, and the slot machines' rules and states (`world::casino`) done; the slot machines, blackjack and roulette playable in the viewer from their tables' and machines' scripts (`world::casino::{slots, blackjack, roulette}`, `ui::menus`, `cellview`, `game_menus`, `casino_scene`; checked live at Vikki & Vance: spins, a deal, a split, a stand, a roulette spin and its result); the roulette cursor by the pad remains. Weapon mods ([WEAPON_MODS.md](WEAPON_MODS.md)): slots, fitting and their effects on damage, clip, spread, weight, attack speed, projectiles, V.A.T.S. to-hit, condition and worth, the Pip-Boy's mod screen, the modded models (the player's first-person objects, `WNAM`/`WNM{n}`) split beams (projectiles, cone, the shown damage) and silencers (`world::noise`: attacks' noise, `VNAM`/`NAM5` sound levels for `fActorAlertSoundTimer`, heard by detection) done; the remaining effects remain. |
| M4: Stability and performance | Recorded routes and extended play without crashes or lost state. Measure frame times, memory, loading and streaming stalls on target hardware. Publish traces/settings and agreed budgets; remove measured stalls without changing behavior. | Not measured here; instrument earlier when it helps M1/M2. Chazm's PR #12 (2026-10-07, `claude/contrib-chazm-perf`, [CONTRIB_CHAZM.md](CONTRIB_CHAZM.md)): mean frame time, three interleaved runs before/after, Goodsprings 19.8 → 16.7 ms, Back in the Saddle 41.7 → 23.9 ms, Ghost Town Gunfight 24.9 → 20.1 ms, Doc's walk 14.0 → 10.8 ms; Doc's house screenshots pixel-identical, outdoor ones within run-to-run noise. |
| M5: DLC and mods | Official DLC progression and reproducible plugin/archive/loose-file, script and content-extension cases; document interfaces and exclusions. | Load-order infrastructure exists; broad compatibility unverified. Mods and plugins (M8, [MODS.md](MODS.md), 2026-10-07): the game's plugin order (master flag, dates, masters moved, `.nam`, masters made active), form IDs (engine forms 1–0x7FF kept), archive order and priority (`OpenArchive`'s list: mod archives first, the first loaded mod archive winning), `<plugin>*.bsa`, loose files with `bInvalidateOlderFiles` and `ArchiveInvalidation.txt`, traced and done with generated test setups (`mod-cases`); `nvinspect … coverage nvse` lists the script extender functions plugins call (nv-rs runs none); not compared with the original game yet. DLC track started by the maintainer alongside M1 (2026-10-05): Dead Money research pass in [DEAD_MONEY.md](DEAD_MONEY.md); next action: run its data pass. |
| M6: VR | Shared simulation with action inputs, independent aim and multiple views. Headset-tested tracking, controllers, menus, combat, comfort and frame budget. | Architecture documented; headset validation pending. |

Preserve save correctness, mod semantics and VR boundaries throughout; order
does not postpone foundational fixes. Set performance budgets from measured
hardware/display requirements. Original save compatibility and native binary
mod compatibility need separate researched scope; custom saves and plugin
reading do not establish them.

## Active work: M1

The user requested a stop and handoff to Claude on October 11. Automatic
translation and the recurring Codex check are paused; no new tasks start.
[HANDOFF.md](HANDOFF.md) now records the full session results, exact checkout
states, evidence and resumption order. Water's release passed and its clean
slot commit `22c7549` is ready but uncollected. #147 remains a draft: the published green
head omits the locally committed callback-return correction, whose normal
checks remain unrun. The final taskqueue/save-file and inventory-source
drafts are preserved with their pending checks. Build 40 is unchanged.
Next action for the next lead: inspect the handoff, preserve all local work,
then validate the return correction and ready water batch before publishing
a corrected #147 head with fresh core/viewer checks.

#146 merged `05ee5a0` after both
current-head checks passed. Taskqueue collection `c766db2` passes 14,469
engine tests, its doctest, strict lint and formatting; forty ordinary bodies
are implemented, with thirty-six compiler-only partials retained explicitly.
A fresh worker continues its remaining thirty-one ordinary bodies. Water
now owns the sole heavy compiler, followed by a03's save-file batch and the
root inventory-source workspace checks. The latter passes 38 package tests
and private eighteen-item source/composition probes, but remains outside
gameplay. Build 40 is current, without a confirmed next-play date. Next
action: publish this validated taskqueue batch, collect/restart each ready
slot and finish inventory-source validation. [HANDOFF.md](HANDOFF.md)
records exact states.

#145 merged `fb4b846` after both
current-head checks passed. Root's saved-local batch passes all focused
checks and 16,273 workspace tests including doctests; strict lint is running,
with release/debug builds and final load/save/three-leg-chain probes pending.
Taskqueue, water and save-file drafts retain their documented validation
state and wait for the single heavy compiler lane. Private inventory probes
preserve eighteen original multi-stack item payloads plus generated count,
health and Worn cases; source-bound ownership and world integration remain
open. Build 40 is current, with no confirmed next-play date. Next action:
finish and publish the saved-local batch, then collect each validated slot
and immediately restart its worker. [HANDOFF.md](HANDOFF.md) tracks ownership.

#143 merged after both CI checks.
HUD's final six normal bodies passed validation and collection `34ee119`:
14,410 combined engine tests, doctest, strict lint and formatting. Seventeen
compiler-only partials remain explicit. A fresh worker immediately started
save/load file and change-map translation. Water and taskqueue now validate
in two compiler lanes. Root #144 publishes three saved-item helpers after
16,238 workspace tests, strict lint, formatting and release/debug builds.
Both final release probes preserve selected original scripted item payloads
byte-for-byte and run the expected BEGIN handlers; one uses actual declared
variable defaults from the installed official plugins in saved order.
Ordinary script bodies, historical overrides and whole-save/world integration
remain open. Root's next bounded correction has 102 native cases and
thirteen drafted Rust regressions; compilation is queued. Taskqueue's
14,445 engine tests pass; lint was interrupted for memory pressure and
must rerun before collection. Water's three incorrect test expectations
were corrected after 41/44 passed and await a rerun. Root now validates
with one Cargo job while all three slots are held for memory. Rolling #145
was rebased onto merged #144 and awaits both fresh CI checks. Build 40 remains
current with no next-play date confirmed. Next action: validate the queued
batches, ship #145 after both checks, and continue the saved-local path.
[HANDOFF.md](HANDOFF.md) tracks active ownership and exact restart steps.
Script-loading dependency #135 is published at `86d19a0`: 15,876 workspace
tests including doctests, strict lint, formatting and release/debug builds
pass. It adds five helpers and ten regressions, with ten matching native
cases; interpreter/script-form integration remains open. The effect batch
now validates, followed by the ready HUD and movement drafts. #134/#135
await both CI checks. Next action: collect those validated slots and continue
the real script-loading path. Build 40 remains current without a new ETA.

#132 and #133 merged after both CI checks; rolling #134 now publishes
the next actor dialogue/HUD collection with fresh CI. Collection passes
14,126 combined engine tests, strict lint and formatting. A fresh movement
worker immediately continues; root's five script dependencies pass focused
tests and now run full checks. Six native cases establish the next parser's
signed fields and reference-prefix behavior, but it is still untranslated.
Build 40 remains current. Next action: publish the validated script helpers,
then collect the queued effect/HUD batches and continue script loading.

HUD collection `0eb6b3c` passes 14,100 combined engine tests, doctest,
strict lint and formatting; its fresh worker has drafted the remaining
item prompts, details and layout, now under review. Movement validates its
next twenty routines. Root's five script-loading dependencies have ten
passing native cases and ten prepared Rust regressions, with Rust checks
queued after movement collection. Both #132 and #133 pass core CI and
await viewer CI. Build 40 remains current; no next-play-build date is
confirmed. Next action: collect movement, validate script dependencies,
then validate the effect and HUD batches.

Rolling #131 merged `7cda9bf` after both CI checks; #133 publishes the next
movement/effect batch with fresh CI. The effect collection passed 14,095
combined engine tests, strict lint and formatting, and its fresh worker
immediately continued. Root's script-loading follow-on has five reviewed
dependencies and six matching native constructor cases, with Rust checks
queued behind HUD and movement. Next action: collect those slots, validate
the script dependencies, then continue native action-list loading.

Persistence dependency #132 is published at `6513c26`; all 15,781 workspace
tests, strict lint, formatting and release/debug builds pass. The normal
engine registry preserves 10,201 original count/health-only item payloads
from eight saves byte-for-byte, including eighteen multi-stack entries;
three full-record decode failures remain excluded. Whole-save and playable
inventory acceptance are still pending. Build 40 remains current without
a confirmed next-build date. Next action: validate script-local construction
and supply script forms and mappings for the broader inventory probe.

Movement collection `757af70` passes 14,051 combined engine tests and
strict checks; a fresh worker starts its remaining normal bodies. Reader
#130 merged after both CI checks; rolling #131 has fresh CI after rebasing.
The draft inventory probe preserves 10,201 original count/condition item
payloads from eight saves byte-for-byte, including eighteen multi-stack
entries. Three records that fail full decoding are explicitly excluded.
Final engine validation runs now; no whole-save round trip or playable
consumer is established. Next action: finish these checks and publish the
three dependency helpers, then investigate script-extra loading.

HUD collection `5c12df2` passes combined engine tests, strict lint and
formatting, including its audio correction. A fresh worker continues the
next large HUD function. The accumulated movement/HUD/explosion batch can
now be published; movement save/load validation runs next. Reader #130
core CI passes and viewer CI is pending. Build 40 remains current: the
inventory payload work still has no playable consumer or release date.
Next action: publish the rolling batch and validate the form-map helpers.

Explosion collection `5380adf` passes 13,990 combined engine tests and
strict lint/formatting; a fresh worker starts effect items. HUD now
validates its audio correction before the next rolling publication.
The separately compiled form-map draft enables a byte-identical two-stack
payload load/write probe; formal engine validation remains pending.
Next action: collect HUD and publish the accumulated rolling batch.

Rolling #129 merged `1389dab` after both CI checks. Later movement/HUD
collections remain local pending a reviewed HUD audio-call correction.
Reader #130 passes 15,771 workspace tests, strict lint, formatting and
release/debug builds; its two-stack buffered read consumes all 41 bytes.
Full ItemChange loading reaches missing form-map allocator `0043a010`
before loading. That allocator and its refill are drafted, uncompiled.
Explosion now validates; movement and the next HUD batch follow. Build 40
remains current. Next action: collect these slots and validate form-map
allocation to unblock the full payload load/write probe.

HUD collection `74cd13b` passes 13,970 combined engine tests, doctest,
strict lint and formatting. A fresh worker immediately continues its next
normal routines. Inventory writer #128 merged `14c77da` after both CI
checks; the reader is rebased and validating, with full ItemChange payload
round-trip probes next. Movement and explosion drafts await their compiler
turns. Rolling #129 awaits viewer CI. Build 40 remains current; inventory
stack preservation has no playable consumer or next-build date yet.
Next action: finish reader checks and exercise the two-stack round trip.

Rolling #127 passed both CI checks and merged `72003ab`; #129 publishes
explosion effect processing at `12f6153` with fresh CI. Movement continuation
collected as `c757035`: 13,925 combined engine tests, doctest, strict lint
and formatting pass. Six larger bodies now have normal paths implemented;
four compiler exception-cleanup partials remain explicit. A fresh worker
immediately continues at `00931880`. HUD now validates; the inventory reader
and byte-identical payload round-trip probe follow. Build 40 remains current.
Next action: collect HUD, then validate the inventory reader.

Rolling #126 merged `97e7572` after both CI checks; #127 publishes the
collected movement helpers and animation loader with fresh CI. Inventory
follow-on #128 (`4b68ea0`) passes 15,691 core tests, strict lint, formatting and
release/debug builds. Its real-registry probe preserves two separate
count/condition stacks in a 41-byte payload. Native loading and a world
consumer remain unfinished. Build 40 remains current.
Save-buffer #125 merged `68387b4` after both CI checks. Eleven reader
helpers are peer-reviewed with ten matching native vectors, uncompiled.
Next action: validate loading and the two-stack native reader probe;
Explosion effects collected as `b9871e9`: 13,885 combined engine tests,
strict lint and formatting pass; compiler SEH cleanup stays partial.
A fresh worker immediately started the next explosion body. Movement now
validates, then HUD and the inventory reader.

Animation loader collected as `6c569b7`: 13,879 combined engine tests,
strict lint and formatting pass. Three compiler-cleanup partials remain
explicit; a fresh worker immediately started HUDMainMenu. Root now validates
the inventory payload dependencies and two-stack probe. Build 40 remains
current, without a new viewer consumer. Next action: finish this bounded
writer check, then collect the reviewed explosion effects batch.

MobileObject collected as `8f92810`: 34 complete functions and 13,864
combined engine tests, strict lint and formatting pass. A fresh worker
immediately tackles its larger remaining bodies. Animation loader now
validates; root's two-stack inventory probe follows. Rolling #126 and
save-buffer #125 await CI. Build 40 remains current; these translations
have no viewer consumer yet, so no next-build date is established.
Next action: collect the loader and validate the inventory payload probe.

Rolling #124 passed both checks and merged `c6ac7eb`. Rolling #126 now
publishes animation parsing and Init3D; save-buffer #125 is rebased with
fresh CI. MobileObject is validating; the animation loader is queued.
Root inventory writing has 16 helpers/15 tests prepared and a two-stack
probe ready, uncompiled. Next action: collect the queued slot batches and
validate that probe; Build 40 remains current.

Init3D collected as `407caf3`: 13,824 combined engine tests, strict lint
and formatting pass; SEH cleanup remains explicit. Fresh a04 immediately
starts the next explosion routine while a01 validates MobileObject.
Root's inventory form-ID/map follow-on is prepared with seven native
vectors and static review, but has not compiled. Next action: collect
MobileObject and the animation loader, then validate the inventory probe.

Save-buffer #125 is open at `7c58130`: all 15,641 core tests, strict lint,
formatting and release/debug builds pass. The real-registry count/health
writer produces a payload that the FOS decoder reads back correctly; a
full save round trip and viewer integration remain open. a04 Init3D now
validates; a01 and the reviewed animation loader wait for the compiler.
Build 40 remains current. Next action: collect these batches and trace
per-stack state ownership before adding a world consumer.

Reference #122 and rolling #123 passed both checks and merged as `6bea9a1`
and `8b5eebe`. Loaded-area bounds are published in rolling #124 with fresh
CI. Animation collection `6dbf98f` passes 13,820 combined engine tests,
strict lint and formatting; 37 functions are complete, two retain SEH
partials and `005f3a20` remains open. A fresh worker started that routine.
Root now validates the buffered count/health writer and payload-decoder
probe. Build 40 remains the play version. Next action: finish that bounded
probe while the other slots continue, then collect the prepared Init3D batch.

Loaded-area bounds collected as `86886d9`: 29 complete functions and three
explicit exception-cleanup partials; 13,790 combined engine tests, strict
lint and formatting pass. A fresh MobileObject worker started immediately.
Animation-group validation now has the compiler. Rolling #123 is open at
`f963f09`; #122 has passed core CI and awaits viewer CI. Root save-buffer
writing/growth/insertion code and tests are prepared, with six native
constructor cases, but uncompiled. Build 40 remains published; no new
viewer behavior or next-build date is established. Next action: finish
animation validation and the save-buffer probe; ship each passing PR.

Rolling #121 merged as `eda98da`. Explosion Initialize collection
`e4de812` passes 13,756 combined engine tests, strict lint and formatting;
exception cleanup remains partial. Its fresh worker starts `009ad670`.
Reference #122 is rebased with fresh CI; the follow-on has six native
save-buffer constructor cases, but its Rust code is not compiled yet.
Next action: publish multibound/Initialize and collect loaded-area bounds.

Reference adapter #122 is open at `0653079` after 15,587 core tests,
strict workspace lint, formatting, release/debug builds and the real
registry probe pass. The handle keeps reference lists isolated and aliases
the same object when copied. No viewer consumer or save round trip exists.
Next action: collect the prepared Explosion batch and trace buffered
extra-data saving while #121/#122 await their current-head CI gates.

Multibound collection `20c6123` passes 13,751 combined engine tests,
strict lint and formatting. Three exception-cleanup partials remain
explicit; eight generated cases also match the native executable. Its
fresh slot now translates TESAnimGroup. Rolling #120 merged as `3a92d0a`;
#121 has fresh CI. Next action: finish reference-adapter workspace checks
and the real-registry probe, then validate the prepared Explosion batch.

Navigation collection `a902f84` passes 13,724 combined engine tests, lint
and formatting, including the immediate-vtable correction. Its three
exception-cleanup partials remain explicit; the fresh slot advances to
loaded-area bounds. Multibound validation is running, with eight generated
native cases available as regression expectations. Next action: collect
multibound, then validate the reference adapter and bootstrap probe.

Explosion collection `d781c3b` passes 13,722 combined engine tests, lint
and formatting. Both added normal paths retain exception-cleanup partials;
the fresh agent owns the five untouched bodies. Navigation now validates.
Multibound review corrections are prepared but uncompiled. Rolling #120
passes core CI and awaits viewer CI. Next action: collect navigation and
finish the reference adapter after the queued multibound checks.

Reference bootstrap now passes through the real translated registry:
construction, embedded list access, count/health changes and count removal
preserving health. Dependencies pass 13,716 engine tests, lint, formatting
and debug build. An existing-reference handle and isolation test are
prepared but uncompiled. Explosion's focused suite passes 98 tests;
full validation is active. Next action: collect Explosion and validate the
reference adapter after the queued slot checks.

Resumed after the usage reset. Rolling #119 passed both checks and merged
as `951c0c7`; the checked pathfinding follow-on is being published next.
The allocator test failure was an incorrect backing-size expectation:
the platform heap rounds to eight bytes. The revised test checks both
requested and backing sizes; validation is running. Next action: finish
the reference bootstrap probe and resume serialized slot validation.

At the saved 98% usage stop, pathfinding collection `ded5afd` passes
13,717 combined engine tests, lint and formatting. Its 17 remaining
exception-cleanup partials stay recorded; a03 advanced to b0060 and
saved research without source edits. a01/a04 have uncompiled follow-ons
preserved in their worktrees. Rolling #119 core passes, viewer pending.
Next action: ship #119 after both checks pass, then resume the recorded
dependency and slot validation queue in HANDOFF.md.

Health adapter #118 passed both checks and merged as `a0ee61d` on
2026-10-10 at 22:32 local time. Rolling #119 is rebased onto that main
and includes the checked navigation continuation; fresh CI gates it.
Next action: finish pathfinding collection and validate allocator startup.

Navigation collection `d24c517` passes 13,698 combined engine tests,
lint and formatting. Two additional normal paths retain explicit
exception-cleanup partials; the last edge-repair body remains unregistered.
A fresh worker researches it while pathfinding validates. Rolling #119
and health #118 have passed core CI and await viewer CI.
Next action: collect pathfinding and validate the lead allocator helpers.

Explosion's owner/weapon callback-reload correction and nine further
functions pass 13,695 combined engine tests, lint and formatting.
Collection `a2fe87a` removes the publishing hold on the next rolling
batch. A fresh worker owns its 18 remaining entries (eleven explicit
exception-cleanup partials and seven untranslated). Navigation validates
next. These translations do not yet establish newly playable behavior.
Next action: publish the checked batch and collect the next finished slot.

Count adapter #117 passed both checks and merged as `cbca26e` on
2026-10-10 local time. Health #118 is rebased onto main with unchanged
implementation and fresh CI. Lead form dependencies pass 124 focused and
13,466 engine tests, lint and formatting; the reference probe reaches the
uninitialized allocator. Its traced startup helpers are prepared. Next
action: validate those dependencies and the pending explosion corrections.

Rolling #116 passed both checks and merged as `c23f744` on 2026-10-10
local time. Pathfinding's reviewed corrections and 35 further functions
pass 13,680 combined engine tests, lint and formatting; a fresh agent
owns its next task. The next rolling batch remains local until explosion's
owner-reload correction passes. Lead reference-constructor helpers and
form callback-reload regressions are in focused validation. Next action:
collect the checked explosion correction and open the next rolling PR.

Continuous update, 2026-10-10: navigation's relocation fix and three more
functions pass 13,638 combined engine tests, lint and formatting. A fresh
agent owns its three remaining geometry functions. Pathfinding validates
its review corrections next; explosion's owner reload fix is prepared
while its fresh agent continues the queued functions.
Count #117 and health #118 pass local checks and await CI; the health
probe checks 20 health and 14 count payloads with the save unchanged.
Reference-constructor dependency work is prepared but uncompiled.
Next action: collect the review fixes and ship remote #116 after CI.

Phase 2 list adapter #115 passed both checks and merged as `94165e8` on
2026-10-10 local time. Count follow-on #117 now targets main with fresh CI;
health adaptation is in local validation. Pathfinding collection adds 35
complete functions and five explicit exception-cleanup partials; 13,599
combined engine tests, lint and formatting pass. A fresh agent is on its
next task. Next action: finish health validation and ship remote #116
after its viewer check passes; the local navigation relocation fix still
gates publishing the subsequent batch.

Navigation collection, 2026-10-10: 36 further functions pass 13,577
combined engine tests, lint and formatting. PC review found a buffer
relocation case in `0069b030` that still needs a correction and regression;
this batch remains local until fixed. A fresh navigation agent is working
on that correction and the six remaining queue entries. Pathfinding now
has the serialized compiler window. Next action: collect checked work and
ship the existing remote #116 when both checks pass.

Review corrections collected, 2026-10-10: a04 adds 24 further functions
and fixes the reviewed point-copy/accessor ABIs and x87 NaN stores;
13,541 combined engine tests, lint and formatting pass. The ledger now
keeps 12 documented AI partials in the task queue, with five tool tests
passing. Fresh a04 research continues while a01 validates in the single
compile window. Next action: ship updated #116 after both CI checks pass.

Rolling PR #114 passed both CI checks and merged as `89e1a0b` on
2026-10-10 local time. #116 now holds the subsequent navigation/explosion
batch, with review corrections required before merge. #115 is rebased
onto new main with unchanged adapter source and fresh CI. Next action:
collect corrected slots and finish the count-payload follow-on checks.

Explosion collection, 2026-10-10: 24 functions added, 16 skipped;
13,511 combined engine tests plus formatting/lint pass. Native review
found a return-value/NaN copy correction and accessor-description issues;
the next a04 task owns those fixes and missing branch tests before this
batch merges. Other slots continue. Next action: collect the corrections
and ship rolling PR #114 when its viewer check passes (it excludes this
new explosion batch).

Continuous batch update, 2026-10-10: build 40 from merged #113 passed
Doc Mitchell (63 s), Back in the Saddle (410 s) and Ghost Town Gunfight
(339 s) scripted acceptance routes. Navigation continuation adds 36
complete translations with four explicit skips; 13,487 combined engine
tests, lint and formatting pass. Phase 2 list adapter PR #115 passed local
core checks and awaits CI. Next action: continue the prepared translation
slots and validate the count-payload dependency (see ENGINE_STATE.md).

Engine-port takeover, 2026-10-10 evening: rolling translation PR #109
passed both CI checks and merged as `b995218`; these translations are
unit-tested engine code, not new playable behavior. Interrupted a01/a04
work resumed; Phase 1 PR 7 remains unfinished in p2. Current owners and
resume instructions: [HANDOFF.md](HANDOFF.md). Next action: finish and
verify the interface/render stage while collecting checked translations.

First resumed translation batch: a01's 40 navigation spatial helpers
collected as `74f7fb1`; formatting, lint and 13,281 engine tests pass.
Five generated point-product cases (`00698620`) also match native
`nv-call` output. These functions are not yet wired into play. Evidence
and queued slots: [HANDOFF.md](HANDOFF.md). Next action: finish the
interface/render acceptance and collect the remaining checked slots.

2026-10-10, Phase 1 PR 7: interface/render call and gate model, explicit
depth-2 wiring inventory, and post-swap audio/process-list scheduling
implemented. Both workspaces passed tests (core 15,108; viewer release
182), fmt, clippy with warnings denied and release builds; all three
acceptance routes passed. Evidence and open work:
[FRAME_SKELETON.md](FRAME_SKELETON.md), "PR 7 result". Original-game
comparison remains pending. Next action: merge after CI and review,
publish the play build, then begin Phase 2 (HANDOFF.md). Buffered
player input remains an interface follow-up before moving menu idle.
Interface/render PR #113 now passes local core/viewer tests, lint, release
builds and all three acceptance routes. CI and merge remain pending;
play build remains 39. Exact menu-input placement and renderer work remain
open, and original-game comparison is not done. Evidence and owners:
[HANDOFF.md](HANDOFF.md). Next action: merge after both required checks,
then publish play build 40 from merged main.

Navigation-render batch collected as `6855902`: 36 preserved translations
now covered by 31 focused tests, including nonempty triangle geometry and
512 native style vectors. Combined lead checks passed formatting, lint
and 13,312 engine tests. These functions remain outside live gameplay.
Next action remains interface/render CI and play build publication.

Exterior-cell loader batch collected as `d68664d`: 38 translations and
43 focused tests; combined formatting, lint and 13,355 engine tests pass.
Rolling #112 merged as `d854fbe`; #114 holds later batches. Interface/render
#113 is rebased onto that main and rerunning CI. Phase 2 list-adapter
preparation is isolated and not yet validated or used by the viewer.
Next action: merge #113 after both checks and publish play build 40.

Havok-world batch collected as `195ffaa`: 40 complete functions and
42 focused tests; combined formatting, lint and 13,397 engine tests pass.
The isolated Phase 2 list probe now passes with installed executable data
and the real registry. No world consumer or save round trip is established.
Next action remains interface/render CI and play build 40 publication.

Phase 1 PR 7 merged as `f917219` after both CI checks passed. Build 40
is building from main. AI task-manager batch collected as `136fa2a`:
28 complete and 12 partial functions (five behavior gaps, seven cleanup
gaps), with 13,438 combined engine tests passing before the PR 7 rebase.
The list adapter passed its initial full checks and is rebased for another
combined check. Next action: verify and publish play build 40.

Phase 2 health payload, 2026-10-10: native vectors exposed and now verify
the constructor's signaling-NaN correction. The read-only reference-save
probe passes all 20 health and 14 count payloads, with the source hash
unchanged. All 15,315 core tests, formatting, lint and release build pass.
No world consumer or save writer is added. Next action: merge the count
dependency, then gate the health follow-on on CI; see ENGINE_STATE.md.

Phase 2 count dependency, 2026-10-10: list adapter #115 is merged.
The missing base destructor now matches a generated native sentinel case.
Count creation, signed updates and removal pass through the real registry.
A read-only Doc Mitchell save probe checked 14 count payloads across 3,319
extra lists, with its source hash unchanged. Local core checks and release
validation pass, including the unchanged count source in the health
follow-on. This is count-only adaptation, not a save round trip. Next
action: merge after CI; details in [ENGINE_STATE.md](ENGINE_STATE.md).

2026-10-10, first Phase 2 list boundary: an explicit engine-owned list
handle delegates to translated PC functions, with four missing helpers
closed. Generated integration tests and an installed-executable probe
pass construction, insertion, cached lookup and detach. Local core checks
passed 15,154 tests, lint, formatting and release build after rebasing
onto merged PR #113. No world consumer
or save round trip is established. Evidence, ownership and the next count
destructor dependency: [ENGINE_STATE.md](ENGINE_STATE.md). Next action:
merge after CI, then close that dependency for a count adapter.

Build 40 published from merged main `f917219` at 19:47 local. Release
build and the fresh Doc Mitchell route passed (63 s); startup loaded
417 objects and 406 meshes in 3.8 s. App executable hash matches the
built source binary, and WHATS-NEW uses LF. The two longer routes are
rerunning. Next action: finish those checks and the Phase 2 adapter PR.

Next session: start with [HANDOFF.md](HANDOFF.md) and [SYSTEMS.md](SYSTEMS.md);
contributor rules: [CONTRIBUTING.md](../CONTRIBUTING.md).

Overnight 2026-10-06 → 07 (integration branch, play builds 12 onward):
radio unification and the 2 key, death into ragdoll, Chazm's PRs #11 and
#12 ([CONTRIB_CHAZM.md](CONTRIB_CHAZM.md)), and maintainer's-list fixes
B3, B4, B5, B6, B10, B11, B12, B13, B16. Each was traced in FalloutNV.exe, checked live
in the release viewer (what was and wasn't is in each topic page) and
passed all three acceptance routes after merging. None has been compared
side by side with the original game. The twelve major systems are open
as GitHub issues #13–#24. A shared read-only Ghidra server now serves
agents' queries ([RESEARCH_WORKFLOW.md](RESEARCH_WORKFLOW.md)).
Playtest of build 23 (2026-10-07): the Vigor Tester's screen was
invisible since build 11 (B22, fixed: one system sets the HUD camera's
blending for every 3D menu; checked live by capturing the window, against
the build-11 code). New: B23, the hands and weapon around menus.
B31 (branch `claude/b31-menu-blur`): the world behind menus captured once
and held with its traced modifier and blur (`0086f450`, `00718ab0`,
`00ba4270`); live: the Vigor Tester's room blurred, the Pip-Boy and a
message box not ([MENU_FADES.md](MENU_FADES.md)).
B33 (branch `claude/b33-greetings-chatter`): greetings' four remaining conditions and the held look (`008eeec0`, `008bc3d0`); live 9 → 5 greetings on a 240 s Goodsprings walk ([DIALOGUE.md](DIALOGUE.md)).
B23/B27 (branch `claude/b23-b27-first-person`): menus no longer hide the hands and weapon, only the dialogue menu (also under its barter menu) and the Pip-Boy arm while up or going down do (`00870bd0`, `011d9514`); the player's body tint made from the race's `.egt` and `FGTS` as the game does (`006149b0`, `0065b410`); live: pistol behind a message box, Pip-Boy put-away, tinted hand ([PLAYER_ACTIONS.md](PLAYER_ACTIONS.md), [FACEGEN_CONTROLS.md](FACEGEN_CONTROLS.md)).

B26/B30 (branch `claude/b26-b30-scripts`): Ringo's first-meeting line after the gunfight is the acceptance route skipping the meeting (say-once, `0061e600`; met first, he says "I owe you a huge favor" live); `GetAV XP` and other record-less actor values give the base form's 0 (`005f0fb0`), VCG04's revise box live at 0 XP.
**Next action:** after Chazm's PR is merged and tasks reassigned, B14
(opening), B23, then B1/B2 (physics), split into sub-PRs. The opening's
Vigor Tester step has no acceptance route yet (the doc route starts at
stage 110).
B1 PRs 1-4 (`claude/b1-havok-step`, 2026-10-07): the game's own Havok world constants, step driver, single-body integrator and sleeping translated (`physics::havok`; the invented sleep rules gone; contacts still this solver's until PR 7). Verified live: the VCG02 bottles stay on the rail, a shot one comes to rest and stays. Evidence: [PHYSICS.md](PHYSICS.md). **Next action:** B1 PR 5 (simulation islands).
B1 PR 10 (`claude/b1-character-proxy`, 2026-10-08): every walker moved by the game's own controller (Bethesda's states, steps, walls and slopes around Havok's proxy and simplex solver, `physics::controller`/`proxy`/`simplex`); acceptance passes; B25's waist-deep walking not seen live. Next: pushing bodies (`applySurfaceInteractions`) and swimming.

B1 PR 5 (`claude/b1-p5-islands`): Havok's simulation islands translated (`physics::islands`: swept broadphase boxes `00d1a330`, pairs merge islands `00cc0f40`/`00cb4c60` so a moving body wakes a sleeper when their boxes meet, parted pairs split at the next step `00cb6060`, sleep and wake per island through the dirty list `00cb55d0`). Evidence: [PHYSICS.md](PHYSICS.md) "Simulation islands". **Next action:** B1 PR 6 (contact manager).
B1 PR 6 (`claude/b1-p6-contacts`): Havok's contact manager translated (`physics::manifold`: points per agent kept within 0.1 Havok units, properties `00cfd800`, pairing `00d92df0`, removal `00cfd200`; one "contact point added" event per new point `00cfcf80` → `00d01850`, so resting bodies stop re-sounding). Evidence: [PHYSICS.md](PHYSICS.md) "Contact points". **Next action:** B1 PR 7 (contact solver).
B1 PR 7 (`claude/b1-p7-solver`): Havok's contact solver translated (`physics::solver`: accumulators `00d29830`, new-point callbacks `00d92900`, contact and friction Jacobians `00d72190`, `hkSolveConstraints` `00d8d030` with 4 substeps and the integrated-velocity sums, export `00def570`, apply `00d29bf0`); this solver's XPBD contacts deleted. Contact resting velocity FLT_MAX turns off Havok's immediate bounce. Evidence: [PHYSICS.md](PHYSICS.md) "The contact solver". **Next action:** B1 PR 8 (ragdoll constraints).

Overnight batches, 2026-10-06 (local session; integration branch
`claude/overnight-integration`, not merged into `main`; each batch also has
its own pushed `claude/m*-*` branch for review as a PR). Play copy builds 1-6
were published from the integration branch. Everything below is implemented
and unit-tested; **none of it has been compared side by side with the
original game**.
- M1: look-IK (`claude/m1-look-ik`), Pip-Boy mouse (`claude/m1-pipboy`),
  player movement (`claude/m1-movement`), Doc's door farewell via TCFU
  follow-ups (`claude/m1-dialogue`), NPC dialogue facing/zoom and topic
  list (`claude/m1-dialogue-npc`), player furniture sitting
  (`claude/m1-sitting`).
- M2: grid-wide exterior reference scripts (`claude/m2-cell-scripts`),
  package types and actions (`claude/m2-packages`), quest targets on the
  compass (`claude/m2-quest-targets`), dynamite/explosions
  (`claude/m2-explosives`), third-person camera and player body
  (`claude/m2-third-person`), NPC weapon choice/reloads/GetShouldAttack/
  OnStartCombat (`claude/m2-npc-combat`), Back in the Saddle and Ghost Town
  Gunfight route fixes (`claude/m2-vcg02-route`, `claude/m2-vms16-route`).
- B5 (`claude/b5-weapon-effects`, 2026-10-07): firing sounds (2D for the
  player, 3D and distant for people, placed and attenuated as the game's
  audio does), muzzle flashes on the player's (both views) and people's
  guns, melee swing-miss sounds; traced, unit-tested, seen live; not
  compared with the original ([WEAPON_EFFECTS.md](WEAPON_EFFECTS.md)).
- B6 (`claude/b6-impacts`, 2026-10-07): impacts: world decals clipped onto
  struck `NiTriStrips` pieces and the land (lifetime, fade, limits), blood
  spatter decals, impact effect models with their own controllers, impact
  sounds placed and attenuated; traced, unit-tested; decals seen live,
  effect models placed but too faint to see in screenshots
  ([WEAPON_EFFECTS.md](WEAPON_EFFECTS.md)).
- M11 (2026-10-07, `claude/factions-crime`): reputation clamps, notices and
  the title box, karma for owned terminals and notes, assault/murder
  making the victim's factions enemies, hacking alarms, and the faction
  armour disguises' engine side (form-list `GetEquipped`, area pulses)
  traced and tested ([FACTIONS_CRIME.md](FACTIONS_CRIME.md)).
- Acceptance evidence ([GOODSPRINGS_ROUTE.md](GOODSPRINGS_ROUTE.md)): with
  dialogue choices replayed by `--run` lines, Ghost Town Gunfight reaches
  stage 100 (XP +50) on the integration build; Back in the Saddle completes
  with several steps forced by console lines.
- Blockers: starting VCG02 by talking to Sunny (greeting order traced and
  matching; needs an original-game check), long outdoor NPC paths (agent
  on `claude/m2-long-paths`), NPC aim far too accurate (agent on
  `claude/m2-npc-aim`), eyes not moving (FaceGen eye update untraced),
  explosion visuals, Pip-Boy local-map quest markers (world map done on
  `claude/m2-pipboy-complete`).
**Next action:** review and merge the `claude/m*-*` PRs into `main`, then
play the Goodsprings route in the play copy and file F12 reports.

Chazm's pull request (slaterain/nv-rs#11), 2026-10-06 (merged on
`claude/contrib-chazm`, not yet in the integration branch): the Bink intro
movie (bit-exact decoder, played on `--new-game`), crafting, terminal
hacking and the terminal menu, item scripts, Repair, weapon mods, attack
noise, companions (trading, the wheel), Caravan, casino rules and slot
machines. Where the Dead Money merge already had a version (crafting,
caravan cards, companion and terminal functions, casino menus) one
implementation is kept, Chazm's traced one with the other's traced details
folded in; the Pip-Boy's Drop is one path (mouse and pad X); his mouse
capture is replaced by ours. Unit tests pass; not compared with the
original game. Triage and what was rejected:
[CONTRIB_CHAZM.md](CONTRIB_CHAZM.md). **Next action:** the lead merges
`claude/contrib-chazm` into the integration branch after reviewing the
acceptance results.

NPC gear redraw, 2026-10-07 (`claude/npc-gear-redraw`): people
are redrawn when what they wear or hold changes (`EquipItem`,
`UnequipItem`, armour or a weapon taken away, a weapon swapped), only the
parts that changed rebuilt, on a thread (`BipedAnim::LoadBipedParts`
keeps unchanged slots): [NPC_GEAR.md](NPC_GEAR.md). Unit-tested and
checked live on Doc Mitchell; not compared with the original game.
**Next action:** let `world::outfit` re-pick through the companions'
`InitDefaultWorn`/`GetBestArmor` port (both in this branch now), so
re-picking uses the whole inventory.

Follow-ups for Chazm's areas, 2026-10-07 (branch
`claude/repair-mod-followups`, from the integration branch): the Pip-Boy's
Repair and Mod screens take the mouse (their `DoEnter` / `DoLeave` /
`DoClick`, the scroll knob turning with their lists, the brackets' exact
place; [REPAIR.md](REPAIR.md), [WEAPON_MODS.md](WEAPON_MODS.md)),
`RemoveMe` on a worn item takes the worn one off first
([ITEM_SCRIPTS.md](ITEM_SCRIPTS.md)), and the crafting menu's item card
([CRAFTING.md](CRAFTING.md)); the merchants' repair menu already took
the mouse (checked again). Unit-tested, seen in the viewer, acceptance
routes pass (doc, vcg02, vms16); not compared with the original game. **Next action:** keep an item's
condition and mods per instance (what it needs: [REPAIR.md](REPAIR.md),
"Not done").

Look-IK batch, 2026-10-06 (local session, branch `claude/m1-look-ik`).
Corrections: ADR-0004 (restructure) was rejected on 2026-10-06 and the
layout stays; `codex/m1-reload-update-order` was dropped unmerged; Codex is
no longer used. The Xbox 360 prototype's PDB confirmed the
`bhkRagdollController` (Xbox PDB) field names and the look-IK function pairs
in the PC disassembly. Native head and eye tracking is now ported in
`world::look_ik` and runs on placed people in the viewer. Its helpers and
the direction solve are checked against FalloutNV.exe with `nv-call` (the
first runs of the oracle tools against the real executable). Root clippy
and the look-IK tests pass. Doc's tracking has not been compared with the
original game. Not yet done: the target choice of 008a3100 (the player is
the only supported target), eye meshes (FaceGen eye update), and
`nv-probe` recordings in a private game copy. Evidence:
[OPENING_LOOK_IK.md](OPENING_LOOK_IK.md). Parallel batches (each on its own
`claude/m1-*` branch) cover dialogue exit and fidelity, couch sitting,
Pip-Boy mouse input and player movement. **Next action:** record Doc's
look-IK in a private game copy with `nv-probe` and compare.

Look-IK reconcile, 2026-10-06 (`claude/m1-lookik-reconcile`, unmerged):
the contributor's head-track target choice, actor look anchors, FaceGen
eye darting and eye limits (`playcon/claude/*`, 9defa47, 5dfb5d9,
ae5b593, ddc0d59) are ported onto the native controller after re-checking
them in Ghidra; where the two ports disagreed the executable settled it
(the eye chain runs for people, so eyes aim at the target; the tracking
distance is between positions). Doc, Sunny and Goodsprings NPCs track
the player and each other in the viewer; 0064c410 and the eye-range
helpers are nv-call regression vectors. Not compared with the original.
**Next action:** record Doc's look in a private game copy with `nv-probe`.

The published baseline includes the opening package look lock, Doc's queued
chair exit, native Info HUD and tester bounds correction, SPECIAL interface,
and same-cell trigger reset on F9. Live evidence reaches the tester through
stage55/60 and opens SPECIAL; it is not full-route acceptance.
Historical checks and play hashes remain in [OPENING.md](OPENING.md).

Persistence batch: failed custom save writes preserve the previous save, and
failed F9 destinations preserve the running world. Generated regressions
pass; 913 core and 84 viewer tests, both clippy/format/release checks and an
isolated installed-data forced-save smoke passed on
`codex/m1-opening-overnight`. Evidence and process handoff:
[PERSISTENCE.md](PERSISTENCE.md). PR #5 merged as `637b28d` after both GitHub
checks; live failed/valid F9 also passed. Verified build installed in play/app.

Next bounded batch implements native race/sex hair/eye reconciliation as
core face-editor groundwork, with six regressions and eight official-data
cases. 919 core / 84 viewer tests, both workspaces' checks/releases and the
installed-data opening smoke pass on `codex/m1-native-appearance`. The face
menu still auto-accepts.
Merged as `843926d` through PR #6 after both GitHub checks passed; verified
appearance build installed in play/app. No new face UI is exposed.

The bounded gamedb evaluation silently missed the central look-lock function
in a four-export probe. Full indexing is deferred; source snapshots, hashes,
DB and inventory remain private. Shared query workflow and limitations:
[RESEARCH_WORKFLOW.md](RESEARCH_WORKFLOW.md).

Movement batch (`claude/m1-movement`, unmerged): the player's speed now
follows `00647d10` in `world::locomotion` (weapon away × 1.1, armour and
drawn-weapon penalties, run perk on running only); over-encumbered blocks
running and jumping; a jump drops the run-up and air steering closes 0.3
of the gap a frame. Tested, not compared. Evidence and gaps (slopes,
per-direction animation speeds, camera): [MOVEMENT.md](MOVEMENT.md).

Player furniture batch, 2026-10-06 (`claude/m1-sitting`): E on furniture
now runs the game's own activation and sit procedure for the player
(`TESFurniture::Activate` `005095b0`, approach `00904f50`, temporary third
person `00950340`/`009503d0`, activate-key gate, seated pitch limit,
first-person seated loop with its Camera1st track); E on nothing while
seated gets up. Unit and generated-plugin regressions pass; no live couch
run or original comparison yet; no third-person camera exists for the
entry/exit. Evidence and gaps: [FURNITURE.md](FURNITURE.md). **Next:** live
`--stage VCG01 27` couch run through Doc's questionnaire.

Third-person batch, 2026-10-06 (`claude/m2-third-person`, unmerged): the
player camera's first/third person, view key (F; walk/fly moved to `),
wheel zoom, vanity mode, temporary views (furniture, Pip-Boy, dialogue)
and the chase camera with wall collision are translated from
`0094ae40`/`0094a0c0`/`00950110`… into `world::player_camera`; the
player's third-person body is built from the record and game state
(`world::actor::player_look`) and animated through the NPC path
(idle, directional walk/run, weapon, sitting). Tested, not compared.
Evidence and gaps (sneak/jump groups, body fade, pivot node term):
[CAMERA.md](CAMERA.md). **Next:** compare F/wheel/wall behaviour and the
couch entry framing in the original.

Movie batch, 2026-10-06 (Claude, branch `claude/m1-bink-intro`): new
`crates/bink` decodes Bink 1 video with tables read from the install's
`binkw32.dll`; `nvinspect <movie.bik> info|frames` and the 32-bit
`research/nv-oracle` tool `nv-bink` (the game's own library as oracle)
compare them frame by frame. All 8,692 intro frames match in Y, U and V;
982 core tests pass. The shipped `VCG00` plays `FNVIntro.bik` and jumps to
stage 90, so the burial stages (and `TriggerScreenSplatter`) never run.
Audio (`bink::AudioDecoder`, `nvinspect <movie.bik> audio`) matches the
DLL's `BinkGetTrackData` output in length and within one step on all
27.8 million samples. Colour conversion matches the DLL's 32-bit output on
all frames. `PlayBink`'s four flags (interruptible, mute, pause music,
letterbox) and the game's tiles, filtering and placement were read from the
exe; the viewer now plays the intro on `--new-game` (`viewer/src/movie.rs`)
with the game's clock stopped and input withheld, and the game continues
into Doc's wake-up afterwards (full 290 s run). The command tables were
confirmed at 40-byte entries. Evidence: [MOVIES.md](MOVIES.md). **Next
action:** crafting (`ShowRecipeMenu`, M2's campfire tutorial).

Outstanding M1 gates:
- Exact opening camera transition replay and Doc/player assistance timing.
- Doc head/eye tracking: ported, unit-tested and seen in the viewer,
  with whom actors look at and the FaceGen eyes (reconciled with a
  contributor's branches, 2026-10-06, `claude/m1-lookik-reconcile`); the
  original-game comparison remains
  ([OPENING_LOOK_IK.md](OPENING_LOOK_IK.md),
  [HEAD_TRACK_TARGET.md](HEAD_TRACK_TARGET.md)).
- Original face editor instead of auto-accept
  ([FACE_CREATION.md](FACE_CREATION.md)).
- Opening movie playback ([MOVIES.md](MOVIES.md)): done in the viewer
  with the game's presentation; remaining differences (filtering in linear
  light, the handler's sound fade) are listed there.
- In-progress animation save restoration, full character-creation route,
  exit to Goodsprings and save/restart/reload acceptance.
- SPECIAL's remaining visual/input fidelity and progression comparison
  ([VIGOR.md](VIGOR.md)); general idle callbacks remain partial.

Camera persistence now passes 926 core/85 viewer tests and both workspaces'
checks/releases, 20 installed-clip continuation cases, live F5/F9 and
fresh-process camera restoration. NPC animations and dialogue continuation
are still missing. No camera build published yet.

Camera PR #7 (`5b3ec14`) exposed a pre-existing parallel fixture collision;
unique fixture directories and a deterministic regression fix it. Its amended
CI is running (amended core passed). Active branch
`codex/m1-reload-dialogue-cleanup` isolates old dialogue/voice state from
successful reloads; failed loads preserve playback. Root926/viewer89 tests,
both workspaces' checks/releases and live success/rejection cases pass.
It does not restore the saved conversation. Native RaceSexMenu
navigation is traced; field callbacks and original comparison remain open.
See [FACE_CREATION.md](FACE_CREATION.md). Camera PR #7 and dialogue PR #8
have since merged; check their build publication against the Codex branch. See
[PERSISTENCE.md](PERSISTENCE.md) for owners and unfinished checks. Gaze cap
constructors are now identified; runtime configuration and original comparison
remain unverified. Do not repeat the established
package-look lock or tester activation fixes.

Pip-Boy input batch (`claude/m1-pipboy`): the mouse now drives the
Pip-Boy (screen picking, rows, tabs, model buttons, world-map markers,
wheel, drags) through the interface, F1-F3 and Tab release work, and the
arm's field-of-view scale is fixed. Traced and unit-tested, not compared
with the original game; gaps in [PIPBOY.md](PIPBOY.md). **Next action:**
live comparison of the mouse paths in the original game.

Pip-Boy live batch, 2026-10-06 (`claude/m2-pipboy-live`): driving the
release viewer with injected Windows input showed no Pip-Boy click ever
completed (the buttons were cleared every frame, so no release was seen);
fixed by reading the button events. Added, traced: the fast-travel
question, the player's own map marker (right button, set/move/remove
questions, saved), world-map zoom (wheel, Page Up/Down), the world map's
picture border for markers, ITEMS Drop (right button, quest-item refusal,
"how many?"), the keyring, ITEMS' button states, STATS healing mode (a
Stimpak aimed at a limb, `0082b970`), boxes over the Pip-Boy taking the
input. Each verified live in the viewer; none compared with the original
game. Gaps (Repair/Mod menus, hot keys, local map, radio, the light):
[PIPBOY.md](PIPBOY.md). **Next action:** compare these paths in the
original game, then the Repair menu.

Pip-Boy completion batch, 2026-10-06 (`claude/m2-pipboy-complete`): hot
keys (wheel, assignment, use with the Pip-Boy away), notes by kind (text,
image, audio with its countdown), world-map quest target markers, the
aimed limb's blink, knobs and rad needle turning, the Pip-Boy light
lighting the place, dropped items resting on the floor and taken back
with E. Verified live in the viewer (except the blink and needle, tested
only); none compared with the original game. Repair/Mod belong to another
contributor's branch. Still missing: the start menu on Escape, the radio,
the local map. **Next action:** the start menu (`007cb7d0`, `007cc6e0`).

Start menu, radio and local map batch, 2026-10-06
(`claude/m2-startmenu-radio-map`): Escape opens the pause menu
(`StartMenu` 1013) with its background, settings pages, the viewer's
saves (save / load / confirmations) and Quit; DATA › Radio (stations in
range, tuning, songs and programmes, playing with the Pip-Boy away); DATA
› Local Map (pictures from above, fog of war, doors, quest markers, the
arrow, zoom) indoors and outdoors; hot key controls read through
`controls.rs`. Verified live in the viewer only; none compared with the
game. Details and gaps: [START_MENU.md](START_MENU.md),
[PIPBOY.md](PIPBOY.md). **Next action:** compare the local map and
pause menu with the game; Radio New Vegas's news live.

Radio and 2 key batch, 2026-10-06 (`claude/m2-radio-unify`): one radio
state. The Dead Money contributor's radio script functions now act on the
Pip-Boy's radio (`world::radio`): `PipboyRadio` / `PipBoyRadioOff` tune
and stop what DATA › Radio shows and plays, `StartRadioConversation`
replaces a station's programme, `SetNPCRadio` makes a person a receiver
playing the station's lines (mono songs) near the player (`00835810`,
`00834260`); old `scriptradio` saves load. The 2 key traced
(`0077da60`, `00781ba0`): it swaps ammunition, and as hot key slot 1
(which nothing can be put on) does nothing, tap or hold. Seen live in the
viewer (script tune / off, Sunny's radio, the 2 key's swap); tested; not
compared with the game. Gaps (falloff, lip-synced receivers, radio
templates): [PIPBOY.md](PIPBOY.md). **Next action:** compare a receiver
(Dead Money's Starlet at the fountain) in the original game.

Dialogue batch, 2026-10-06 (`claude/m1-dialogue`): Doc's farewell at the
front door could not be left because line follow-ups (`TCFU`) were not
read. Follow-ups, Goodbye states, random runs and Intelligence classes are
now traced from `00762ff0`/`0061af30`/`0061a7d0` and implemented, with
generated regressions; a real-data walk of the chain reaches the Goodbye
line, which un-destroys the house door. Not yet compared with the original
game. Evidence and gaps: [DIALOGUE.md](DIALOGUE.md). **Next action:** play
the route through the door with installed data.

M2 blocker batch, 2026-10-06 (`claude/m2-cell-scripts`): object scripts
now run for every attached cell (the interior, or the 5×5 grid with the
game's re-centring margin, including persistent objects), disabled ones
too; `OnLoad` fires once per cell attach or enable; triggers follow the
traced one-event-per-step occupancy; the pass stops after `Activate`/
`MoveTo`-type commands. Traced addresses, tests and gaps:
[SCRIPTS_RUNTIME.md](SCRIPTS_RUNTIME.md). Generated-world tests only; not
compared with the original. **Next action:** play Goodsprings to VCG02
stage 20 and compare trigger/`OnLoad` timing with the original.

M2 blocker batch `claude/m2-explosives` (Ghost Town Gunfight dynamite):
thrown weapons, grenade flight and explosion damage traced and implemented
with generated regressions; not compared with the original game. Evidence,
gaps and next action: [EXPLOSIVES.md](EXPLOSIVES.md).

M2 blocker batch (branch `claude/m2-quest-targets`): quest targets (`QSTA`),
the active quest, the door the compass follows and the blinking compass
quest icons, traced and translated; generated-data tests and an
installed-data check pass; not compared with the game; Pip-Boy map quest
markers and the navmesh-level path search remain. See
[QUEST_TARGETS.md](QUEST_TARGETS.md).

Dialogue NPC batch, 2026-10-06 (`claude/m1-dialogue-npc`, on
`claude/m1-dialogue`): the menu's zoom in/out (`00762950`), the player's
view focused on the speaker's head (`00953060`), the speaker's in-menu
turn (`008a5580`, replacing the inferred rule) and the main topic list
as the player's topics (`0083ec30`/`0083ed50`/`00619030`, dropping the
opening-follow-ups guess) are traced, implemented and unit-tested; not
compared with the original. **Next action:** record a Doc Mitchell and a
Sunny Smiles conversation in the original and compare zoom, turn and list.

B13 barter over dialogue, 2026-10-07 (`claude/b13-barter-menu`): service
menus (barter, recipes) now fade the dialogue menu out and back in as the
game does (`00763ff0`/`007640a0`, menu fades `00a1d910`/`00a1db20`);
unit-tested and verified live with Chet; details and gaps in
[DIALOGUE.md](DIALOGUE.md).

B14 opening, 2026-10-07 (`claude/b14-opening`): traced, not fixed. Doc's
chair is reached by the ordinary travel/sit procedure (no start-seated path
in the game); the viewer's path goes through the chair, so he's blocked.
Needs the game's navmesh obstacle system first; details in
[OPENING.md](OPENING.md) (B14).

B16 local map, 2026-10-07 (`claude/b16-local-map`): the Pip-Boy local map's
scale, zoom limits and steps re-traced and found as implemented (see
[PIPBOY.md](PIPBOY.md)); fixed the map running off to its limits after any
drag (the drag's movement was added on every refresh, not once), a drag
flung off when the pointer left the Pip-Boy's screen, the map not centred
again when DATA / the tab is shown or the pictures finish, and added the
pad's sticks (zoom, pan); unit-tested, verified live with the mouse; not
compared with the original.

## M2 blocker batch: gunfight packages

`claude/m2-packages` (2026-10-06): flee, guard, procedure lists, package
type data (`PKW3`, `PKPT`, `PLD2`...) and begin/end/change action dispatch
traced and implemented for Ghost Town Gunfight (`VMS16`). Generated
regressions pass; not compared with the original. Finding: the gunfight's
flee packages have no target or place, so the settlers just stop and
stand; CF/AM guard packages send settlers back to their editor location
after stage 70's `MoveTo`. Evidence and gaps: [PACKAGES.md](PACKAGES.md).
**Next action:** compare both `bTrudyHelp` branches in the original game.

## M2 blocker batch: NPC combat

`claude/m2-npc-combat` (2026-10-06): people now choose their weapon the
way the combat controller rates it (`009993c0`, DPS `00645380`/`00646060`,
weapon kinds `00522c80`, planner costs `011a4280`), switch when out of
reach or ammunition, empty and reload clips, and only use rounds up for
"NPCs use ammo" weapons (dynamite) or teammates (`008a8dd0`);
`GetShouldAttack` (`0059ed30`), `OnStartCombat` (`00980830`/`009887b0`)
and `SetUnconscious` ending the fight (`005d0760`) are in. Generated
regressions pass; not compared with the original. The planner's search
beyond its action costs is inferred. Evidence and gaps:
[NPC_COMBAT.md](NPC_COMBAT.md). **Next action:** record a powder-ganger
fight in the original game and compare weapon switches and dynamite
throws.

M2 route batch (`claude/m2-vcg02-route`): Back in the Saddle (`VCG02`)
driven in the viewer with installed data and completed in one run, with
the quest start, bottle hits, two package end actions, the walk to the
first well, the kills and the reward request simulated by console lines
(new `--run-at`/`--say` testing aids). Fixed: people walking into the
place joining trigger/talk lists, drawing references enabled after load
(bottles, geckos), `KillActor`'s `OnDeath`, shots at disabled objects.
Blockers: Sunny's first greeting (traced order says "Everything all
right?", which can't start the quest), no long-distance paths, package end
actions. Not compared with the original. Evidence:
[GOODSPRINGS_ROUTE.md](GOODSPRINGS_ROUTE.md). **Next action:** record
Sunny's first greeting in the original game.

## M2 blocker batch: NPC aim and next target

`claude/m2-npc-aim` (2026-10-06): people's shots now fly as the game aims
them (`00523150` NPC branch: aim point `009a8460`, cone = weapon min
spread + gun wobble × `fNPCMaxGunWobbleAngle`, iron sights past 512 ×
sight usage `008f74c0`) as rays to the first body, object or wall, with
the line-of-fire hold (`009a6e90`); after a kill or give-up fighters take
the best of their remaining targets (`CombatGroup::GetBestTarget`,
`00986c60`). Gangers' leveled guns are now resolved (they had chosen
fists). Gunfight re-run: both `bTrudyHelp` branches reach stage 100; the
varmint-rifle ganger hits the player 1–3 times in 9 shots at 3000–2000
units (was 10/10). Generated regressions pass; not compared with the
original. Evidence and gaps: [NPC_COMBAT.md](NPC_COMBAT.md). **Next
action:** record ganger hit rates at range in the original game.

NPC hits batch (`claude/m2-npc-hits`, 2026-10-06): people's shots passed
through the Powder Gangers because the gangers, enabled by script after
their squares loaded, dropped out of the place's people at the next square
change (shots only test those bodies). Fixed; Ghost Town Gunfight passed 3
of 3 acceptance runs (people-at-people shots under 400 units: 0 of 95 hit
before, 27 of 57 after), `doc` and `vcg02` pass. Line of fire traced
(`009a6e90` ray pick, `bTargetBlocked` → planner `ACQUIRE_LINE_OF_SIGHT`,
not done). Not compared with the original. Evidence:
[NPC_COMBAT.md](NPC_COMBAT.md). **Next action:** trace the combat
planner's blocked-target actions.

M2 blocker batch (`claude/m2-long-paths`, 2026-10-06): long outdoor
walks. The game's high-level route over the navmesh info map (`NAVI`,
`NavMeshInfoSearch` (Xbox PDB) `006b8c50`/`006b8490`), the detailed path
only over attached cells (`006c9fc0`) and the virtual handler's node walk
out of sight (`009ea8a0`) are traced and implemented; the viewer's
navmesh is now the attached cells' and people beyond them walk out of
sight. Sunny's VCG02 walks (both wells, sneak, wells 2 and 3) complete by
themselves with the player following or ahead; one run completed VCG02.
Generated regressions pass; not compared with the original. Evidence:
[PATHING.md](PATHING.md), [GOODSPRINGS_ROUTE.md](GOODSPRINGS_ROUTE.md).
**Next action:** record where Sunny stops in the original when the player
stays behind the saloon.

NPC animation batch (`claude/m2-npc-anims`, 2026-10-06): Doc's chair
"jump" was the entry easing out over 0.5 s after the heading had turned
half a turn; the game swaps without a blend there (`cSkipNextBlend`,
`004974a0`, set by `009213e0`/`00921e80`), now carried out. People in the
dialogue menu were frozen because the dialogue menu's own screen counted
as a menu for animation; the speaker now animates and plays the game's
talking idles (`008a5580`/`008a20d0`/`008dab40`, `SNAM` speaker idles,
`MenuMode`/`IsTalking`/`GetDialogueEmotion`). Verified live with screen
captures (chair before/after, Sunny and seated Doc talking); not compared
with the original. Evidence and gaps (lines outside the menu, listener
idles, holstering): [ANIMATION.md](ANIMATION.md). **Next action:** record
Doc sitting down and a Sunny conversation in the original and compare.

M2 batch `claude/m2-player-actions` (2026-10-06): the crosshair's Info
panel from `00579280`/`00775a00` (Sit, Sleep, Take, Open, Talk, Search,
Pickpocket/Steal, "Door to Goodsprings", lock and Empty lines, weight and
value), pickups' "added" message and sound, the Sneak toggle with the
lower eye and the sneak meter, iron sights (Aim control, FOV, `IS`
animations, true iron sights), mouse look with the right button free,
Always Run and Auto Move. Seen live in the viewer (couch sit/stand,
stimpak pickup, Sunny talk/pickpocket, [HIDDEN], varmint rifle sights);
not compared with the original. Evidence and gaps (scopes, sway,
hotkeys): [PLAYER_ACTIONS.md](PLAYER_ACTIONS.md). **Next action:**
compare those screens in the original game.

M2 blocker batch (`claude/m2-npc-nav`, 2026-10-06): NPC routes and
collision. People were set along their path points with no collision, on
a stand-in search and funnel. Now the game's navmesh search costs
(`006a6fa0`), its `PathSmootherPOVSearch` (`006ad770`: corner circles of
1.2 × the request radius, side clearance, tangents, retries keeping off
narrow edges), the straight-line test with side lines (`006cd1f0`), the
stuck test (`009e4cf0`) and the player's character controller for every
person (`009ddc00` → `00930c70`) are implemented. Live: Doc walks to his
door and talks, Sunny's VCG02 runs to stage 45 by herself, settler 04
goes through the saloon door. Generated regressions pass; not compared
with the original. Gaps: light clutter blocks people (Easy Pete stuck at
his eating marker), door triangle registration, off-navmesh ends.
Evidence: [PATHING.md](PATHING.md). **Next action:** record Doc's walk
and Easy Pete's approach in the original.

M2 blocker batch (`claude/m2-npc-nav-2`, 2026-10-06): NPC navigation
gaps. Closed doors marked on the navmesh as the obstacle manager does
(`006997e0`), locations resolved by the game's height window (`00696a50`)
and ends off the navmesh joined by its ray-cast way (`006caa40`,
`006cac90`, `006e6e40`), travel/follow/long-way searches on a path-manager
worker (`006eb9d0`), combat/sitting walks with the actor's radius and no
straight-line fallback, people pushing clutter through the physics
batch's movers. Live: the gunfight replay completes ("Defeat the Powder
Gangers", XP +50, ~100 s; 7 dead), Doc walks to the door and talks, Easy
Pete reaches his place in the saloon, Sunny walks the long way. Not
compared with the original. Gaps: door-detection box, `PATHPICK` layers,
the move into the high process (a viewer bridge puts people back from
offstage onto the navmesh), push mass limit on the physics side. Evidence:
[PATHING.md](PATHING.md). **Next action:** record where an actor coming
into the high process stands (Sunny after `MoveTo SunnySpawnMarker`).

M2 batch (`claude/m2-physics`, 2026-10-06): clutter physics. Rigid body
values read from the models; free bodies simulated at the game's Havok
step clock (`00c66760`) against the cell's collision, each other and the
player; shot pushes (`009c2e80`), explosion pushes (`009b0920`) and the
gameplay impulse scaling (`0062b520`) translated; moved objects kept in
the state and saves. The VCG02 bottles are plain Havok clutter (no `DEST`):
verified live, a varmint rifle shot tips one off the fence onto the ground
behind it and F5/F9 keeps it there. Solver internals are this solver's,
labelled; not compared with the original. Evidence and gaps:
[PHYSICS.md](PHYSICS.md). **Next action:** record a bottle shot off the
fence in the original game and compare.

M2 batch `claude/m2-player-actions-2` (2026-10-06): scopes (the weapon's
`MOD3` overlay framed as `0076bfe0`/`0077ee50`/`00709d50` set it up, HUD
mode 0x17, hidden arms, `ScopeWobble.nif` sway on the view), the gun's
first-person sway (`00962de0`, `WeaponWobbles`), blocking (`BlockIdle`,
the threshold bonus of `009b5a30` in the hit cone of `009a6ae0`, the
block hit and counter timer), power attacks (`00948310`: hold delay,
directions, sneak attacks, perk customs, × `fDamagePowerAttackBonus`),
the Ammo Swap control with the HUD's `AmmoTypeLabel`, and death
(`fPlayerDeathReloadTime`, then the most recent save). Seen live in the
viewer (scope, block, forward power attack, swap to hollow points,
death → reload, [DANGER], third-person sneak pose); not compared with the
original. `WG`/`VAL` re-checked: invisible by data and code. Evidence
and gaps: [PLAYER_ACTIONS.md](PLAYER_ACTIONS.md). **Next action:**
compare the scope, block/power attack and death timing in the original.

NPC animation batch 2 (`claude/m2-npc-anims-2`, 2026-10-06):
`Actor::PickAnimations` (`00895110`) is now translated whole for
standing, walking, running, sneaking, turning and the weapon: every
`.kf` of the actor's folders indexed by group id (weapon kind, movement
kind) with the game's lookup and fallbacks (`00495740`); drawing and
putting away (`Equip`/`Unequip`, the weapon in hand at `Attach`); the aim
over a drawn weapon; NPC sneak and weapon-out rules (`00888b50`,
`008eeec0`); the strafe/back choice facing a target (`009e2aa0`); idles
in their record's section (upper-body greets, movement-section hit
reactions); the say's speaker/listener requests outside the menu
(`008a20d0`, GREET, conversations); hit reactions (`0089a760`); the
player's third-person body on the same path with `cSkipNextBlend`.
Verified live in the gunfight and with injected keys in third person;
not compared with the original. Evidence and gaps:
[ANIMATION.md](ANIMATION.md). **Next action:** record a ganger drawing
and a settler greeting in the original and compare.

Third-person weapon fix (`claude/m2-third-person-weapon`, 2026-10-06):
the player's gun floated in front of a fists-up body. Fixed as traced:
another weapon in hand restarts the weapon section and puts the weapon
where its drawn state has it (`004ab750` → `ForceWeaponDrawnSheathed`
`009231d0` → `ReparentWeapon` `00923960`: the kind's `Equip` plays), so
the pistol's or rifle's aim replaces the fists' guard; the `Weapon` bone
hangs under the `prn:` node of the group that put it there (`005f3a20`;
holsters: pistol hip, rifle back, heavy weapons hand); the third-person
reload plays; a gun is put away after R is held `fPlayerWeaponReloadTimer`
(0.5 s, `009466d0`). `--key-at` drives keys for pictures. Verified live
in third person (pistol and varmint rifle drawn and holstered, fists,
attack 0x0420, reload 0x04bc); not compared with the original. Details:
[ANIMATION.md](ANIMATION.md). **Next action:** compare the drawn and
holstered poses with the original game.

M2 batch (`claude/m2-physics-2`, 2026-10-06): physics completion. Traced
and translated: the collision filter's layer table (`00c828f0`, shots
cast on the projectile layer), contact friction/restitution (`00cfd800`),
the land body's friction, the Z-key grab and Havok's mouse spring
(`0095f6c0`…`00961280`, `00cbb1e0`), impact sounds by material
(`00837550`, `00839e00`), physics damage (`006238b0`, `0062be90`),
saved velocities (`00563220`/`00563380`), the walkers' move limit
(`fMoveLimitMass`). Fixed: moved bodies hit and picked where they are; no
prompt on destroyed references (the swinging-door pick took clutter for
doors). Verified live: shot/second shot, no prompt, Z grab and carry,
dynamite moving the bottles, F5/F9 of flying bodies, walking into a
tumbleweed, contact sounds logged. Not compared with the original;
Havok's solver, deactivation and constraints aren't reproduced. Evidence
and gaps: [PHYSICS.md](PHYSICS.md). **Next action:** record a bottle shot
and a Z-grab carry in the original game and compare.

## Contributor merge: Playcon Dead Money (`claude/contrib-playcon`, 2026-10-06)

The outside contributor's branches (remote `playcon`, based on old main
`647af94`) were merged in stack order with their history kept; triage
table, conflicts and guesses in [CONTRIB_PLAYCON.md](CONTRIB_PLAYCON.md).
Taken: the Dead Money stack (`dlc-dead-money` … `dm-casino-character`:
radio script state, LOS/IsAnimPlaying, shaders, terminal back, caravan
cards, companion/actor functions, dispel, traps, VATS conditions, recipe
and casino menus' script side, `--character`, intro slideshow and
`SayToDone`, talking activators, `--choose`, crafting, test characters),
`teammate-wait`, `viewer-use-flag`, `dialogue-info-links` (`INFC`),
`dm-coverage`, `dm-handoff`, `script-transpiler` (`scriptgen`), and from
the face branches `RACE_SEX_MENU.md` and the `SI.CTL` FaceGen controls.
Dropped as duplicates of our traced work: player sitting, Pip-Boy voice
notes, `bring_in_enabled`, per-cell script refresh and its grid-lag fix,
`package-end-action`, `brought-in-talkers`, the look-IK/eyes branches,
`research-tools`, `focused-edison`, `fit_to_race`; dropped as unsafe: the
shared texture/material caches. Their radio module stays isolated from
the Pip-Boy (the radio agent's branch owns that); `sounds::play` no longer
takes Bevy's Ogg assets (Ogg is decoded in the viewer), which that branch
will need to follow. Guesses traced here: essential knock-down
(`0089d900`, 10 s, full restore) and a line's voice type (`00616fa0`,
`INFO` `ANAM`). Still untraced and behind `world::guesses`
(`NV_GUESSES=1`, off on base-game routes): teammate follow at 200 and the
wait rule, teammates coming along, a person's `IsAnimPlaying`, scripts
opening the recipe menu. Checks: root 1255 passed / 1 failed (the
worktree-only `repo_hygiene` `.git` pointer), viewer 133 passed, both
clippy and fmt clean, release viewer built. Live: the Ghost Town Gunfight
replay reaches `XP +50`; `--new-game` in Doc Mitchell's house runs Doc's
`VCG01Intro` SayTo lines to stage 10 and the name entry, as the
integration build does; Dead Money's Villa arrival
(`DLC01StartMarker --official --character characters\dead-money-villa.txt`)
reaches Elijah's hologram and his dialogue menu. `INFC`: greetings of nine
Goodsprings NPCs unchanged. **Next action:** trace `RecipeMenu` (`007274b0`)
and the follower rules so the Dead Money routes run without guesses.

M2 batch (`claude/m2-physics-3`, 2026-10-06): playtest physics fixes.
Traced and fixed: a loading place's bodies go into Havok asleep
(`00c674d0`, `00c6b0a0`; Doc Mitchell's clutter, the rail bottle and the
pickets no longer move on load), statics' bodies are fixed (`005768b0`),
the wind listener rolls `WIND`-flagged bodies (tumbleweeds; `00c74570`,
`00c74550`, the NIF body flags read by `00c8ea30`), dog ragdolls keep
their `bhkRigidBodyT` torso (all 20 joints), one ragdoll's bodies meet by
the part table (`00c84740`, `00624070`), unskinned biped pieces hang from
their slot's bone and FaceGen-flagged ones are head parts (`004ac1e0`,
`01188be8`; Easy Pete's hat). Verified live with screenshots and logs; a
console-killed Cheyenne still stands in her death pose (the death
animation through the ragdoll isn't traced). Evidence and gaps:
[PHYSICS.md](PHYSICS.md). **Next action:** trace how the game drives a
dying creature's ragdoll with its death animation.

M2 batch (`claude/m2-death-ragdoll`, 2026-10-07): death → ragdoll traced
(`0089d900`): ragdolls play no `Death` animation, every body goes dynamic
from its pose with the death nudge (`fDeathForceForceMin`), a killing
hit's push only after a hit; the dead go limp with the AI held still
(Cheyenne and Easy Pete seen falling live). Details: [PHYSICS.md](PHYSICS.md).
**Next action:** trace which `00c65b00` branch the living biped takes
(the bodies' velocity handed to the ragdoll).

B12 (`claude/b12-doc-dialogue`): Doc's door conversation no longer restarts forever: a dialogue package that has talked is finished (`005fa330` saves it at DONE, `008b1070`/`00913250` restore it, `0090a1a0` keeps the same package), the menu opens an update after `InitiateDialogue`, AI holds still under message boxes, and an own-delay quest's first run is traced (`005ac1e0`). Verified live: hardcore box, no new conversation, VCG01 completes, Doc sandboxes; not compared with the original ([DIALOGUE.md](DIALOGUE.md)). **Next action:** watch the farewell in the original game.

B29 (`claude/b29-revise-loop`, 2026-10-07): the revise prompt (`VCG04`, out past Goodsprings, not Doc's door) kept coming back: `GetButtonPressed` was one value any script could take, and the activator `MoveTo player` moved didn't run where it was moved, so answers fired later back in Goodsprings. Now a box's button is its shower's only (`005b4630`/`005b4940`/`005b4a80`) and moved references run in the cell under them (`005ccb20` → `00573800` → `00548230`); `StartQuest` resets nothing (`005c71c0`). Verified live at the border (rebuild, asked once more, Finished ends it); not compared with the original ([OPENING.md](OPENING.md)). **Next action:** the face and SPECIAL chargen menus the prompt opens.

B32 (`claude/b32-dlc-rerun`, 2026-10-07): DLC start messages "again on a later cell change" were each acceptance route starting a new game; script variables already persist across cell changes as in the game (`00455490`, `ExtraScript` via `00565870`). Verified live (three `MoveTo` cell changes, each DLC message once) and by `ref_scripts::once_only_guards_survive_cell_changes` ([SCRIPTS_RUNTIME.md](SCRIPTS_RUNTIME.md)). **Next action:** none.

B3 (`claude/b3-crosshair-pick`, 2026-10-07): the crosshair pick is the
game's view caster (`0070bc20` → `00631d60`: layer-40 sphere cast, exact
ray/NiPick per hit, fuzzy fallback for statics), one pick for talk, doors,
objects, HUD and Grab; live in Doc's house and the Prospector Saloon
([PLAYER_ACTIONS.md](PLAYER_ACTIONS.md)). **Next action:** compare the
pick's fuzzy cases with the original game.

B10 (`claude/b10-greetings`): greetings traced (`008eeec0`, `008bc3d0`): one GREET line to the player at a time, a 30 s player-wide `fHelloCooldownTime` after any greeting (`008bc520`/`008bc560`, player update `00944179`), the greeter's 20 s counted from the line's end, the package's hello/chatter flags; activating someone whose greeting is a one-response Goodbye line only says it (`005fa330`), and saying stops the speech in progress (`00934250`). Verified live: settler says a line without the menu, Easy Pete opens it, greetings 30+ s apart; not compared with the original ([DIALOGUE.md](DIALOGUE.md)). **Next action:** record a Goodsprings walk in the original and count greetings.

B11 (`claude/b11-voice-skip`): lines after a skip were silent because the voice file name was wrong for short quest names with long topics (Doc's psych test, his Pip-Boy line), not because of the skip: the name rule is now translated from `006172c0` (a quest under 11 bytes keeps whole, the topic gets the rest of 25). Verified live: every response of Doc's psych test and farewell plays its voice with every line skipped; not compared with the original ([DIALOGUE.md](DIALOGUE.md)). **Next action:** check all voice file names against the voice archives.

B4 (`claude/b4-npc-ground`, 2026-10-07): people back from a walk out of sight (Ringo in the gunfight, 64-75 under the land) had no controller and walked under the ground for good; `MobileObject::Move`'s two rules are now followed (`0092f260`: outdoors, feet more than 30 under the land are put on it, the player's too; further than `fCharControllerWarpDistSqr` (2449.5) from the camera people walk on the navmesh's height without their controller). Verified live (gunfight, Back in the Saddle, the player started under the land); not compared with the original ([PHYSICS.md](PHYSICS.md)). **Next action:** a ragdoll seen sunk to the waist after the gunfight (B1's solver).

## M4: PR #12 review and streaming hitches (`claude/perf-streaming`, 2026-10-07)

Merge notes for PR #12 (performance), stacked on `claude/perf-latest`;
details and method in [PERFORMANCE.md](PERFORMANCE.md). The maintainer's
three review points:
- **Shared textures:** tinted faces and hair are keyed by what they were
  made from (`"<base> + FaceGen <tint>"`, `"<base> + layer <file>"`, per-NPC
  tint files), so they don't mix. The cache's lifetime was wrong instead
  (textures freed while held, re-uploads across frames); fixed in
  `ba8e864`.
- **Screenshots** of Doc's house, faces (Doc, Trudy, Sunny, Chet, Pete) and
  Goodsprings before/after: differences within the noise of two runs of
  the same build (idle poses, grass, tumbleweeds).
- **Acceptance-route frame rates** (median fps before → after, busy
  machine, re-measure on a quiet one): doc 252–264 → 474–504, vcg02 132 →
  273, vms16 113 → 240; all routes pass in both.

In this branch: `ba8e864` (PR #12's texture lifetime fixed). Follow-ups: `d0bbcc0` (player body/view relit, not rebuilt),
`1fe77bf` (outdoor collider gathered without re-measuring), `5e7f8eb`
(materials kept when their lights don't change): flying out of
Goodsprings, each 2 s window's longest frame 130–230 ms before
`d0bbcc0`, 45–115 ms after it (median 75 ms), median 36–39 ms after
`5e7f8eb`. Not fixable here: a minimized window held to 60 fps (bevy_winit
redraw pacing). **Next action:** re-measure the route table on a quiet
machine.

Stutters in the merged build (2026-10-07, `claude/overnight-agents`),
measured as each frame's own work on the main and render threads
(`--fps`): a folder sound (every gunshot, most impacts) looked through
every archived path for its folder each time it played (~10 ms, twice a
shot), now listed once per folder (warmed on a thread at startup) with
decoded sounds kept; the path smoother's triangle-under-a-point looked at
every triangle of the mesh (10k outdoors) for each line it walked (a
sandbox choice's path up to ~50 ms), now through the grid index with the
same answer (tested against the full scan); the squares' navmeshes joined
on crossing into another square decode their `NAVM` records once (30–60
ms to under 7). Walking through Goodsprings, the longest main-thread frame
went from 25–68 ms to 7–14 ms; a pistol shot from ~20 ms of work to under
1 ms after the first. Test aids: `--background` (behind other windows,
without the focus, the mouse left alone) and work times in `--fps`.

M4 weapon class batch `claude/energy-weapons` (2026-10-07): energy
weapons. Traced and implemented: critical effects (laser disintegration,
plasma goo: cast on a killing critical, kept on the dead, ash/goo piles
standing for the corpse, critical stages, the Disintegrations statistic),
the weapons' resist type (Energy Resistance) after the armour floor,
automatic weapons' critical chance ÷ fire rate, beams striking at once
and plasma bolts flying at their speed (player and NPCs). Generated-data
tests; live check in Doc Mitchell's house; not compared with the
original. Evidence, gaps and next action: [ENERGY_WEAPONS.md](ENERGY_WEAPONS.md).

M4 weapon class batch `claude/launchers-mines` (2026-10-07): launchers,
explosive projectiles, thrown weapons and mines. Traced and implemented:
missiles falling under their record's gravity (the projectile's character
controller), the ammunition's own projectile (HE/HV missiles), explosions'
object effects (EMP, fire) and knockdown rules, the AI holding explosives
that would catch its own side, mines (proximity, owners, Light Step, the
fuse and blink, disarming with XP, taking the mine, placed `PGRE` mines
saved), and E on ash piles; worn armour's DR read in hundredths.
Generated-data tests; live checks (grenade rifle and rocket in
Goodsprings, the mined corridor in `NorthVegasHouseTools`); not compared
with the original. Evidence, gaps and next action:
[EXPLOSIVES.md](EXPLOSIVES.md).

M4 weapon class batch `claude/melee-unarmed` (2026-10-07): melee and
unarmed. Inventory against the exe, then traced and implemented: reach
of hand-to-hand weapons, what a swing hits (`FindMeleeTarget`: combat
target, else nearest the hit cone's middle; the player's cone by the
attack, dead targets ×2), fists' power attack bonus after the armour,
unarmed uppercut/cross outside V.A.T.S. by the Unarmed skill and their
effects (stagger and disarm, ×2.5 limb damage), the stagger rules,
Super Slam's knockdown chance, V.A.T.S. specials' names (`VANM`),
thresholds and spells (Mauler's knockdown). Generated-data tests; live
check in Doc Mitchell's house; not compared with the original. Evidence,
gaps (fatigue damage, the stagger/knockdown animations, NPC specials)
and next action: [MELEE_UNARMED.md](MELEE_UNARMED.md).

M4/M3 batch `claude/fatigue-blockers` (2026-10-07): fatigue and
knock-outs, and two quest blockers. Traced and implemented: full fatigue
(record + derived, auto-calculated people), fists' and the bean bag's
fatigue damage through the armour, `fMinimumFatigue`, regeneration with
the all-back-at-0 rule, the knock states (fatigue below 0, paralysis,
essential down; `GetKnockedState` now 1, not 2), `GetFatiguePercentage`;
`ForceFlee` (the engine's flee package out of a fight, nothing in one)
and `GetGroupMemberCount`/`GetGroupTargetCount` (combat groups: alone,
joined by helpers, merged, the player's). Unblocks by function: 9 quests
calling `ForceFlee`, 9 asking `GetGroupMemberCount`. Generated-data
tests; live checks in the Prospector Saloon; not compared with the
original. Evidence, gaps and next action: [FATIGUE.md](FATIGUE.md).

B14 research `claude/b14-navmesh-obstacles` (2026-10-07): the navmesh
obstacle manager registers only forms flagged 0x02000000 (no furniture),
so it isn't why the game's Doc gets round his chair; not ported, cause
open: [OPENING.md](OPENING.md) B14, [PATHING.md](PATHING.md).

B14 fixed `claude/b14-doc-seated` (2026-10-07): after a cell load, the game seats loaded actors in their package's furniture (`00972d30` → `0088d2f0`), so Doc is seated from the first frame. Also, people sitting down or seated no longer block others (`00920d00`/`00c711d0`). Verified live (Doc, Easy Pete): [OPENING.md](OPENING.md) B14.

## Original `.fos` saves (M7 research, `claude/fos-saves`, 2026-10-07)

Format and scope written down from the exe's writer and reader and checked
byte-for-byte against nine real saves: [FOS_SAVES.md](FOS_SAVES.md). Header,
plugins, location table, all global data tables, change form records,
form id and worldspace arrays decoded; quests, globals, misc stats, cells,
topics, actor bases, factions, classes and challenges decode to their exact
lengths. Read-only reader `crates/fos` and `nvinspect fos <SAVE> [PLUGIN]`,
run on all nine saves.

Part 2 (`claude/fos-import`): references' extra data (`00426a30`, the
saved-under table `01183d30`), inventories (`004d4090`), mobile objects,
actors, all four AI process levels, movers and pathing, packages made in
game (the combat controller and its procedures), the player
(`009590f0`: perks, active quest, hot keys, notes), projectiles, the
small base form types, the weather and the radio decoded: every change
form in all nine saves (36,322) decodes to its exact length, none skipped.
`world::fos_import` builds a `GameState` from a save (globals and game
time, quests with stages, objectives and variables, said topics, the
player's place, name, S.P.E.C.I.A.L., experience, perks, inventory and hot
keys, references moved, disabled, locked, found map markers, containers,
dead actors and their values, factions, challenges, reputations, local
map fog, weather, radio); plugin indices are matched by name. Gaps listed
in FOS_SAVES.md. `nvinspect fos-import` runs it on all nine saves with no
failures. The viewer starts from a save with `--load-fos <SAVE>`
(checked live on two of the saves: the player where the save says, the
time and quest stages printed). **Next action:** compare a loaded save
against the original game running the same save.

## Engine port Phase 0: map and ledger (`claude/phase0-ledger`, 2026-10-09)

Done ([ENGINE_PORT_PLAN.md](ENGINE_PORT_PLAN.md) "Phase 0 result",
[LEDGER.md](LEDGER.md), [research/engine-map](../research/engine-map/README.md)).

- Evidence: 66,259 functions (3,271 created on a private Ghidra copy at
  vtable slot targets analysis had missed), 17,604 with Xbox PDB names
  (hold-out error about 1.7% for the call-graph tier), a subsystem for every
  function (2,353 explicitly unplaced). Game code 40,523 functions, 8.6%
  translated or traced. Ledger regenerates with one command; map with
  `scripts/engine-map.ps1`.
- Forecast: a 12-function trial cost about 19k Sonnet tokens per function;
  the 37,047 open game functions project to about 700M tokens before
  wiring. A week covers one or two whole systems, not the game.
- Files (owner: this branch): `research/engine-map/*`,
  `research/ghidra/NvEngineMap.java`, `research/ghidra/NvCreateFunctions.java`,
  `research/ghidra/README.md` (two sections), `scripts/ledger/*`,
  `scripts/engine-map.ps1`, `docs/LEDGER.md` (generated),
  `docs/FRAME_SKELETON.md`, `docs/ENGINE_PORT_PLAN.md`.
- Private state: named Ghidra copy `%USERPROFILE%
v-reghidra-phase0`
  (plus `-a`, `-b`, `-c` copies used by the trial agents, deletable);
  intermediate files in `%USERPROFILE%
v-reworkphase0`. The shared
  server (8089) was not running and was not touched.
- Unfinished checks: PROTOTYPE_SYMBOLS.md's manual 100-row spot-check per
  tier; `NvCreateFunctions.java` has no fixture test yet.
- Next action: Phase 1 PR 1 (frame map) from
  [FRAME_SKELETON.md](FRAME_SKELETON.md); restart the shared server on the
  named copy when convenient.

### Translation push (`claude/engine-crate`, from 2026-10-09 night)

The maintainer set a one-week fast track (about 1.9 billion tokens).

- Architecture: `crates/engine` on a model of the game's memory
  ([ADR-0006](adr/0006-engine-crate-memory-model.md), for maintainer review;
  [ENGINE_CRATE.md](ENGINE_CRATE.md)). Merged: #72 (Phase 0), #73 (crate).
- 16 Sonnet agents in parallel, each in its own worktree under
  `%USERPROFILE%/nv-re/work/agents/aNN`, one unit file (or unit part) each,
  40 functions per session; the lead collects, checks (fmt, clippy -D
  warnings, all engine tests), commits and re-tasks (private scripts in
  `%USERPROFILE%/nv-re/work/phase0/bin`: `cycle.sh`, `task.sh`, `split.sh`).
  Decompiles come from a private read-only Ghidra server on the named copy
  (127.0.0.1:8090, `binstart-srv.ps1`).
- Measured: about 6.7k to 11k Sonnet tokens per translated function,
  15 to 30 minutes per 40-function session.
- Files: every unit file under `crates/engine/src/units` has one owner at a
  time (the slot's task file says which). `units/platform.rs`,
  `units/crt.rs`, `types.rs` and the engine core are lead-owned.
- Next action: keep the slots cycling; merge a rolling PR whenever CI
  passes; then Phase 1 wiring from FRAME_SKELETON.md.

### Phase 1 PR 1: frame map (`claude/phase1-frame-map`, 2026-10-09)

- Evidence: `research/engine-map/frame.tsv` (the `Main::OnIdle` tree to
  depth 3, 6,692 call sites) from the new `frame` step; LEDGER.md "Frame"
  counts it by status (depth 1: 22 translated, 37 traced, 5 platform,
  79 open). FRAME_SKELETON.md: 38 more depth-1 functions paired with
  evidence, the Havok step traced to `TES::UpdateCellAnimations`
  (`00453550`) on the AI linear task threads, and the actor updates to
  the thread functions `008c7bd0`/`008c7da0`/`008c7f50`.
- Files: `research/engine-map/src/bin/frame.rs`, `src/names.rs` (shared
  with `match.rs`), `frame.tsv`, README; `scripts/engine-map.ps1`;
  `scripts/ledger/src/main.rs`; `docs/LEDGER.md`; `docs/FRAME_SKELETON.md`.
- Open: the threads = 1 path (`008ca070`) and whether the INI can lower
  `iNumHWThreads`; six `?` rows and six position-only leads.
- Next action: PR 2 (`world::frame`, the control flow of `0086e650`).

### Phase 1 PR 2: `world::frame` (`claude/phase1-world-frame`, 2026-10-09)

- Evidence: `crates/world/src/frame.rs`, translated from `0086e650`
  (disassembly and decompiler agree): the 143 calls in the exe's order
  (`STEPS`, checked against `frame.tsv` by a test), each with its gate
  (36 gate kinds, one test per gate naming its branch addresses) and its
  wiring. Gates: Tab+Alt held ends the frame; menu mode (V.A.T.S.'s menu,
  sleep/wait, dialogue, the Pip-Boy, the pause menu) and the free camera's
  frozen world (`TFC 1`, `Main` +7) stop the world block, the process
  lists and the AI work; the fader lets the lists and AI run in menu mode,
  the console stops them; V.A.T.S. playback skips the scene graph's field
  of view; the loading block; sleeping redraws the menu background;
  threads = 1 vs > 1. FRAME_SKELETON.md "PR 2 result".
- Wiring: 1 step is an existing system (`Main::OnIdle_HandleMenuBackground`
  → `world::menu_background`); 5 partly exist (timer, faders, calendar,
  tree wind, grid move); 137 open. Nothing in the viewer changed.
- Files: `crates/world/src/frame.rs`, `crates/world/src/lib.rs`,
  `docs/FRAME_SKELETON.md`, `docs/LEDGER.md` (generated).
- Next action: PR 3 (order the viewer's systems by `FrameStep` sets).

### Phase 1 PR 3: the viewer's order from the frame (`claude/phase1-bevy-order`, 2026-10-09)

- Evidence: `viewer/src/frame_order.rs`: `world::frame`'s 8 stages and 143
  steps as Bevy system sets in `Update`, in the exe's order, each run under
  its gate from `world::frame` (`stage_reached`, `step_runs`, new) on one
  `FrameState` filled once per frame (`begin_frame`). 121 `Update` systems
  placed: Loading 5 and Interface 20 ahead of the stages (the menus take
  Bevy's input before the player; explained there), stage 1: 1, stage 2:
  20, stage 3: 4 (steps 32 and 44), stage 4: 13 (step 77), stage 6: 39,
  AfterFrame 19. Removed the one chain over `main`'s groups and 21
  `.before`/`.after`s; the rest stay, each with its reason. The four
  inputs without a source (fader 1, the frozen world, the interface mode,
  the thread count) fixed at the PC case (thread count: the processor
  count, at least 2). FRAME_SKELETON.md "PR 3 result".
- Tests: set order equals `world::frame::stages()`, step sets in call order;
  menu mode stops a gated step set (`Calendar::Update`) but not its stage;
  Tab+Alt stops stages 2-8. Acceptance: doc, vcg02, vms16 pass (on `main`
  before the change vcg02 missed "XP +50" in one run; doc and vms16 passed).
- Behaviour: the menu background and screen effects now run before the
  scripts (a script's effect plays a frame later); previously unordered
  systems have fixed places; no system is gated differently yet.
- Files: `viewer/src/frame_order.rs` (new), `viewer/src/main.rs`,
  `viewer/src/vats.rs` and 21 plugin files, `crates/world/src/frame.rs`,
  `docs/FRAME_SKELETON.md`, `docs/LEDGER.md` (generated).
- Next action: PR 4 (the player stage: `PlayerCharacter::Update`'s order,
  with menu mode tested by the player so the interface can move into its
  stage).

### Phase 1 PR 4: the player stage (`claude/phase1-player-stage`, 2026-10-09)

- Evidence: `crates/world/src/frame/player.rs` (`world::frame::player`):
  `Main::OnIdle_UpdatePlayer` (`0086f940`) as its 30 calls in order (27
  direct, equal to `frame.tsv`'s depth-2 rows, and three through the
  player's vtable: the 3D twice, `PlayerCharacter::Update` at slot +0x2f8,
  `0108ad34`) with 15 gates, and `PlayerCharacter::Update` (`0093e860`) as
  58 sub-steps (the calls that do the player's work, at their call sites)
  with 16 gates: V.A.T.S. ended, a forced activation, the fade, knocked
  down or paralysed, the controlled branch (AI-controlled or dead) and its
  time-out, the free branch, dialogue, V.A.T.S. ending, a muzzle flash;
  each gate with its branch addresses, steps whose own block tests more
  (controls, timers, the weapon) list those branches. In menu mode the
  player's update doesn't run (only `ForceGrenadeHold`) unless the Pip-Boy
  is opening; the fly camera (`Main` +6) replaces it.
  FRAME_SKELETON.md "PR 4 result".
- Viewer: `frame_order::PlayerSet` (calls and sub-steps as sets under
  their gates on `ThisPlayer`). Mapped: `fly_camera` (UpdateFlyCamera),
  `player_attack` (the attack, `00948310`), `walk` (the move, `009ea570`);
  inside the update's gated call: `scope_sway`, `player_furniture`; at
  their sub-step but ungated (they work in menus too): `look_around`
  (UpdateHeadingAndLooking), `player_idle::animate` (the own view's
  animation update); ungated around the step, with reasons: `view_input`,
  `give_start_weapon`, the body, and ten systems after it. All 30 calls and
  58 sub-steps stay open (none is a world system).
- Tests: `world::frame::player` 16 (order against `frame.tsv`, a test per
  gate); engine (new dev-dependency on `world`): the translations of
  `0086f940` (16 input combinations, exact call list) and `0093e860` (13,
  every reached step called in order) follow the model; viewer: player
  sets in the exe's order, menu mode and the fly camera stop their sets,
  the fixed inputs.
- Behaviour: in menu mode (menus, Pip-Boy, dialogue, V.A.T.S.'s menu,
  message boxes) walking, attacks, the scope's sway and the furniture stop
  (the player no longer falls while a menu is up; `player_attack`'s HUD
  line and timers wait); the attack runs before the move; flying, the move
  follows the mouse a frame later.
- Files: `crates/world/src/frame/player.rs` (new),
  `crates/world/src/frame.rs`, `crates/engine/Cargo.toml`,
  `crates/engine/src/units/fallout_misc/main.rs` and
  `fallout_ai/playercharacter.rs` (tests only), `viewer/src/frame_order.rs`,
  `viewer/src/main.rs`, `docs/FRAME_SKELETON.md`, `docs/LEDGER.md`
  (generated).
- Next action: PR 5 (the world and time stage).

### Phase 1 PR 5: the world and time stage (`claude/phase1-world-stage`, 2026-10-09)

- Evidence: `crates/world/src/frame/world_time.rs`
  (`world::frame::world_time`): the 12 functions stage 4 calls, as their
  work calls in call-site order with gates from their own branches (all from
  the disassembly): `TES::TestAllCells` (25 sub-steps), `Calendar::Update`
  (7), `TES::RunAnimations` (3), `ProcessLists::RunActorScripts` (1),
  `UpdateRadiationList` (9), `ChangeProcessLevelTempList` (17),
  `UpdateFollowerTempList` (2), `GarbageCollector::Update` (18) and
  `ClearTempEffects` (4), `BSTreeManager::Update` (7, untranslated),
  `Main::OnIdle_UpdateCurrentGridCell` (6) and `TES::UpdateCurrentGridCell`
  (22: the first load, queueing inside the centre cell, the detach and
  attach across a border, the terrain). The grid update stops in menu mode,
  while a new game's loading menu is up (`[011d8907]`) and with the world
  frozen; the trees' clock and wind stop in menu mode. Correction: the
  stage's slot +0x104 call is `BSSceneGraph::SetViewDistanceBasedOnFrameRate`
  (`00c52590`), not the sky's update (`Sky::Update` is in stage 6).
  FRAME_SKELETON.md "PR 5 result".
- Viewer: `frame_order::WorldSet` (each sub-step a set under its gate on
  `ThisWorld`). Mapped: `trees::blow_wind` (new, split from `sway_trees`)
  under the wind update's gate; at their sub-steps but ungated, with
  reasons: `sway_trees` (camera axes, light, levels of detail),
  `run_scripts` at `RunActorScripts`' `RunScript` (with the game clock,
  `Calendar::Update`'s work), `stream_squares` at `GridCellArray::SetCenter`
  and the distant land and objects at `BGSTerrainManager::Update` (moved
  from `ViewerSet::Loading`). Doors, movies and the weather chain placed for
  order after the scripts. Open: the cell tests, `RunAnimations` (AI thread
  with threads > 1), the radiation, process-level and follower lists, the
  garbage collector, the rest of the tree manager and the grid.
- Tests: `world_time` 13 (order against `frame.tsv` to depth 3, a test per
  gate); engine: the translations of `0086fbe0`, `00867a40`, `00455640`,
  `00452580`, `004556d0`, `00978550`, `009777a0`, `0096eb40`, `0096e9b0`,
  `00868850`, `00868d10` checked against the model under their gates;
  viewer: world sets in the exe's order, menu mode stops the gated sets, the
  fixed inputs.
- Behaviour: trees hold still in menu mode; the squares and distant land
  stream after the player's move in the same frame; weather before trees.
- Files: `crates/world/src/frame/world_time.rs` (new),
  `crates/world/src/frame.rs`, engine tests in
  `fallout_misc/{main,calendar,garbagecollector}.rs`,
  `fallout_shared/tes.rs`, `fallout_ai/processlists.rs`,
  `viewer/src/frame_order.rs`, `viewer/src/main.rs`, `viewer/src/trees.rs`,
  `docs/FRAME_SKELETON.md`, `docs/LEDGER.md` (generated).
- Next action: PR 6 (the AI task stage: `Sky::Update` and the weather chain
  move there with `TES::UpdateCellAnimations`).

### Phase 1 PR 6: the AI task stage (`claude/phase1-ai-stage`, 2026-10-10)

- Evidence: `crates/world/src/frame/ai_stage.rs`
  (`world::frame::ai_stage`), all from the disassembly: the AI linear task
  thread functions as their work calls and handshakes (the combined
  `008c7bd0`, made when `iNumHWThreads` ≤ 2, `008c7307`; the pair
  `008c7da0`/`008c7f50` otherwise, so on most PCs; events `008c79e0`/
  `008c7a70`, the render wait `008c80d0`), `schedule()` putting them in one
  order the exe can run (each thread's order and every wait kept); and the
  work calls with gates of `Main::OnIdle_UpdateAnimationsAndEffects`
  (`0086fc60`: nothing in menu mode unless in dialogue or fader 1, nor with
  the world frozen), `TES::UpdateCellAnimations` (`00453550`: wind, the
  Havok step between `LockHavokUpdateMT` 1/0, `Sky::Update` only with one
  thread), `TES::UpdateCellMainThread` (`004537c0`: `Sky::Update`),
  `GridCellArray::UpdateManagedNodes` (`004ba9a0`), the two
  `bhkWorld::Update` sites (`00554780`, `00551890`),
  `ProcessLists::RunActorUpdates` (`0096c7c0`: slot +0x2f8 per processed
  actor) and `Actor::Update` (`00888b50`, 19 sub-steps). Corrections: with
  AI threads the sky is the main thread's (`004537c0`), and the offstage
  moves are stage 7's (`Main::PostSwapProcess` → `0086f890` → `0096d810`).
  FRAME_SKELETON.md "PR 6 result".
- Viewer: `frame_order::AiSet` (thread calls inside `StartThreads`' and
  `WaitForThreads`' steps, sub-steps nested, gates on `ThisAi`). Under
  gates: `move_pieces` (`TES::RunAnimations`), the projectiles
  (`resolve_shots`, `fly_bolts`, `fly_thrown`, the movement pass), the
  Havok step (`clutter::simulate`, contact sounds), the weather's step
  (`Sky::Update`). At their calls outside the gates, with reasons: the HUD
  and Pip-Boy (interface idle), the posing, idles, head tracking, clothes
  and faces (animation pass), `move_actors` (movement). `move_offstage`
  moved to stage 7. Doors and movies stay in stage 4.
- Tests: `ai_stage` 14; engine: `0086fc60` (15 states), `00453550` (12),
  `004537c0`, `Actor::Update` (5) follow the model; viewer: AI sets in the
  exe's order for 2 and 8 threads, menu mode stops the gated sets,
  dialogue keeps the sky. Acceptance: doc, vcg02 and vms16 pass (57 s,
  399 s, 339 s; no panic).
- Behaviour: loose objects, bolts, thrown things, people's shots and
  models' own animations hold still in menu mode (and the dialogue menu);
  the weather's step waits in menus other than dialogue; people are posed
  before they move (a gait change shows a frame later); offstage people
  move after the render work.
- Files: `crates/world/src/frame/ai_stage.rs` (new),
  `crates/world/src/frame.rs`, engine tests in `fallout_misc/main.rs`,
  `fallout_shared/tes.rs`, `fallout_ai/actor_p2.rs`,
  `viewer/src/frame_order.rs`, `viewer/src/main.rs` and the plugins
  `bolts`, `clutter`, `companions`, `explosives`, `hiteffects`, `hud`,
  `impact_fx`, `local_map`, `pipboy`, `weapon_fx`; `docs/FRAME_SKELETON.md`,
  `docs/LEDGER.md` (generated).
- Next action: PR 7 (the interface and render stage: `Interface::Idle`,
  `LastMinuteUpdate`, `Main::Swap`'s pre/post work, and
  `Main::OnIdle_UpdateProcessLists`' calls).

## Deferred

Cosmetic material/lighting discrepancies, isolated facial polish, sun glare,
distant water and other rendering leftovers stay in the research archives.
Promote only for a milestone blocker or explicit user priority. Small gameplay
fixes needed by the active route count as blockers; never invent a substitute.

## Handoff

After each batch replace the active-work section with the latest outcome,
evidence links, blockers and one next action. Keep this file short; detailed
logs belong in topic references. Completion requires a reproducible acceptance
run, not a count of parsed records or implemented functions.
