# Contributor branches from `playcon` (Dead Money and others)

Integrated 2026-10-06 on `claude/contrib-playcon` (from
`claude/overnight-integration` at `2a7c094`). The contributor's branches
were all based on the old public main `647af94`; each kept branch was
merged with a real `git merge` in stack order, so their commits and
authorship stay in the history. Their handoff is
[HANDOFF_DEAD_MONEY.md](HANDOFF_DEAD_MONEY.md); its rule "no decompiled
code in this repository" is replaced here by ours (ADR-0003: marked
translations with addresses are allowed; raw exports are not committed).

## Untraced behaviour switch

Untraced rules that would change what the base game does are behind
`world::guesses` (off by default; the viewer turns it on with
`NV_GUESSES=1`). The Ghost Town Gunfight and opening routes run with it
off. Dead Money test runs that need Dog to follow (Act 2) set it.

## Triage

"Ours" names the topic file or code we already had. [G] = untraced.

| Branch | What it adds | What we already had | Traced? | Result |
| --- | --- | --- | --- | --- |
| `dlc-dead-money` | DLC research pass (`DEAD_MONEY.md`), `scripts/dlc-data-pass.ps1` | nothing | data-only | merged (`.gitignore` both kept) |
| `dm-radio` | radio script functions (`PipboyRadio`, `SetNPCRadio`, ...) as state only | names listed only; our radio/start menu/local map agent works on `claude/m2-startmenu-radio-map` | addresses given | merged; since `claude/m2-radio-unify` the handlers in `world::more_functions::radio` act on the Pip-Boy's own radio (`world::radio`), one state |
| `dm-los-anim` | `IsAnimPlaying` on objects, `GetLineOfSight` with a viewer `Sight` | Runner without sight | traced (`0059c990`, `0088b880`) | merged; `Runner` keeps our `references_changed` and their `sight` |
| `dm-shaders` | `PlayMagicShaderVisuals` / `Stop...` | no | traced | merged |
| `dm-terminal-back` | `ForceTerminalBack` | no | traced | merged |
| `dm-caravan-cards` | caravan card functions | no | traced | merged |
| `dm-companions-actors` | `OpenTeammateContainer`, `PushActorAway`, `SetDisposition`, `GetCauseofDeath` | no | traced | merged |
| `dm-dispel` | `DispelAllSpells` | no | traced | merged |
| `dm-traps` | `SetVATSTarget`, `FireWeapon` | no | traced | merged |
| `dm-conditions` | `GetVATSValue`, `IsFacingUp` (spine node) | animation picker, look-IK in `actors.rs` | traced | merged; `actors.rs` keeps ours and adds `spine_up` |
| `dm-menus` | script side of recipe and casino menus | no | traced | merged |
| `dm-entry` | `--character FILE` test characters, bunker entry | no | n/a (test tool) | merged |
| `dm-intro` | slideshow, Villa start, `SayToDone`, `SwapTextureOnRef`, Ogg decoded in the viewer, persistent refs outdoors | persistent refs per square (`interactive_in_square`), grid-wide cell scripts (`world::ref_scripts`) | mostly; `SayToDone` dispatch point see below | merged; their `interactive_references_outdoors`, per-cell refresh and seat code dropped for ours |
| `script-cell-grid-lag` | waits for the grid after a scripted worldspace move | our scheduler attaches only loaded squares of the current grid | n/a | recorded as merged with no content (`-s ours`): nothing to guard |
| `dm-voice` | narrator voice, talking activators speak, `--choose`, dense-world load radius, own Ogg voices, Pip-Boy voice notes, references enabled after load | traced Pip-Boy note audio (`00796fd0` 0x18, PIPBOY.md), traced `bring_in_enabled` (`005c43d0`) | partly | merged; Pip-Boy notes and `bring_in_enabled` kept ours; their voice-type fallback guess replaced by the traced rule (`00616fa0`: an `INFO`'s `ANAM` speaker gives the voice type) |
| `dm-act2` | essential knock-down, teammates follow, player sits, shared texture/material caches | player sitting (FURNITURE.md) | knock-down now traced; follow [G] | merged; essential rule translated from `0089d900` / `00888b50` / `008a0960` (10 s, full restore; replaces 12 s and a quarter); follow (200) and teammates coming along gated; player sitting dropped (ours); caches dropped (path-keyed textures collide with FaceGen/hair tints, shared materials would carry relighting and effects across squares) |
| `teammate-wait` | a guard package is the wait order | no | [G] | merged, gated with the follow rule |
| `dm-verify` | finished one-shot groups stop counting; LOS actor test | no | checked in the original | merged; actor `IsAnimPlaying` (1 standing, 0 down) gated |
| `dm-crafting` | recipes, recipe menu, `nvinspect craft`, `--open-menu recipes:` | no | data-only rules; menu order, skill rule, click [G] | merged; replaced on 2026-10-06 by Chazm's traced crafting ([CONTRIB_CHAZM.md](CONTRIB_CHAZM.md)): `--open-menu recipes:` kept, `nvinspect craft` replaced by `recipes`, no longer gated |
| `dm-casino-character` | Act 2 and casino test characters, Dog escort notes | no | n/a | merged |
| `viewer-use-flag` | `--use REF` | no | n/a (test tool) | merged; their trigger body heights dropped (our scheduler) |
| `dialogue-info-links` | a topic says lines its `INFC` list names | no | `00618aa0`; choice [G] | merged; greetings unchanged for Sunny, Doc, Trudy, Ringo, Easy Pete, Joe Cobb, Victor, Chet, Cheyenne; Ringo's first-talk choices gain the two sibling topics of line `0015EBEA` |
| `dm-coverage` | `DEAD_MONEY_COVERAGE.md` | no | docs | merged |
| `dm-handoff` | `HANDOFF_DEAD_MONEY.md` | no | docs | merged (decompiled-code rule superseded, see top) |
| `package-end-action` | travel package End action | ours (PACKAGES.md) | | not merged: duplicate |
| `brought-in-talkers` | people brought in join the talker list | ours | | not merged: duplicate |
| `lookik-*`, `head-track-target`, `facegen-eyes` | look IK, head tracking, eye darting | reconciled earlier (HEAD_TRACK_TARGET.md, OPENING_LOOK_IK.md) | | not merged again |
| `focused-edison-s9k4g2` | early `world::look_ik`, furniture clock audit note | `world::look_ik` and later docs | | not merged: superseded |
| `race-sex-appearance` | `fit_to_race` (`007b1ca0`), player race/hair/eyes kept and saved, RACE_SEX_MENU.md | `appearance::reconcile_parts` (same trace, FACE_CREATION.md) | traced | partial: `RACE_SEX_MENU.md` cherry-picked (`20c24ad`); `fit_to_race` dropped as duplicate; the saved race/hair/eyes fields not taken (no menu sets them yet) |
| `facegen-controls` | `SI.CTL` reader, slider/age/gender maths (FACEGEN_CONTROLS.md) | no | traced | cherry-picked (`edcf8b3`) |
| `research-tools` | `tools/re` scripts and a research handoff | `research/ghidra` toolkit | n/a | not merged: duplicate tooling, stale handoff, rule conflicts with ADR-0003 |
| `script-transpiler` | `crates/scriptgen` and shared interpreter primitives | no | parity-tested | merged (dev tool; generated code stays outside the repo) |

## Guesses: traced, gated, remaining

Traced and translated during the merge:

- Essential actors (`0089d900` `Actor::Kill`, `00888b50`, `008a0960`,
  `bEssentialTakeNoDamage` `00fa8770`): down for `fEssentialDeathTime`
  (exe default 10 s), health and conditions fully restored on going down
  and on getting up; one unconscious or restrained is set back to
  `fEssentialHealthPercentReGain` (0.3) of base health instead.
- Voice type of a line (`00616fa0`): the `INFO`'s own speaker (`ANAM`)
  first; a talking activator's `VNAM` (`009185e0`).
- `SayToDone`: the Xbox prototype names the call site
  `TESObjectREFR::SayToCallBack`; the PC address isn't pinned yet.

Gated behind `world::guesses` (`NV_GUESSES=1`), off on base-game routes:

- teammates with nothing to do follow the player at 200 units, a guard
  package being the wait order (`world::ai::current_package`);
- (teammates coming along when the player changes place,
  `viewer::scripts::bring_teammates`, was replaced on 2026-10-07 by the
  traced `world::companions::come_along`, `00973de0`, always on);
- `IsAnimPlaying` on a person (1 standing, 0 down).

(A script's `ShowRecipeMenu` was gated here until Chazm's traced crafting
replaced the Dead Money version on 2026-10-06; it now always opens the
menu. The other overlapping script functions, `OpenTeammateContainer`,
`ForceTerminalBack`, `AddCardToPlayer`, `GetContainer`, `RemoveMe` and the
casino `Show...MenuParams`, are now carried out once, by Chazm's code, with
this branch's traced details folded in: see [CONTRIB_CHAZM.md](CONTRIB_CHAZM.md).)

Remaining, labelled, not gated:

- a downed essential takes no more harm until the flag goes (whether
  `Actor::Kill` is re-entered in life state 6 isn't read; either way they
  get up with full health);
- `INFC` lines picked like the topic's own;
- a talking activator a script activates starts a conversation;
- Dead Money's Villa world loads radius 1 (a graphics-memory
  accommodation for a laptop GPU, not original behaviour).
