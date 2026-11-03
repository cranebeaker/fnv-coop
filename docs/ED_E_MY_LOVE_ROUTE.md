# ED-E My Love: Primm start

Quest `vDialogueEDE` (`001572E8`), quest script `vDialogueEDESCRIPT`
(`00157F1F`), disabled-robot script `EDEScriptDisabled` (`001572E0`). This
route is **PARTIAL** and stays opt-in (`pwsh -File scripts/acceptance.ps1
-Routes ede -Data <Data>`). It covers ED-E's Primm start, first log, and the
natural Ignacio trigger and second-log playback. The route stops after log two;
later quest progression and completion are not verified. It has not been
compared side by side with Fallout: New Vegas. The PowerShell wrapper has not
been run on Fedora because `pwsh` is unavailable; the equivalent release-viewer
route was run directly.

## Acceptance route

The route starts in `PrimmNashResidence` beside `NVCompanionEDEDisabledREF`
(`001572E6`). It adds 100 Repair to the test player, activates the disabled
robot with `--use`, and answers the repair message buttons. The record's own
`EDEScriptDisabled` handles repair: its `OnActivate` presents `EDERepair00`,
then the repair path presents `EDERepair01`; the GameMode script starts
`vDialogueEDE` when the player chooses “Run the repair.” It moves/enables
`EDE1Ref` and runs its greeting. The route selects the offered recruitment
INFO `001579C3` (“I think we should travel together.”), whose result script
sets `VNPCFollowers.bEDEHired`.

The harness moves the player to `OldLadyGibsonRef` and starts her conversation.
Her greeting INFO `000841E7` has the `bEDEHired` condition and its result script
sets `vDialogueEDE` stage 10. The stage script sees `iLogsPlayed == 0`, starts
`EDEDialoguePackage02`, and displays objective 5. That package plays log one,
INFO `00157F17`; its result sets `iLogsPlayed` to 1.

The route then advances six 23-hour periods through the in-game wait menu
(138 hours intended; 23 Right inputs per wait), moves the player to Ignacio at
HELIOS One, then starts his conversation. It selects the scientific, keep-the-peace, and “what more is
there” responses before choosing ARCHIMEDES INFO `000E35FF`. That INFO's result
sets `VMS03.bDiscussedWeaponWithIgnacio` and calls `SetStage vDialogueEDE 10`.
The quest emits “Two EDE logs heard,” then ED-E naturally starts log two,
INFO `00158829`; its result sets `iLogsPlayed` to 2. The route selects the
offered `Log Playback::End` option after playback. No quest/follower variable,
quest stage, objective, or `GameDaysPassed` value is set directly.

The wait-menu keys use the same player-ready clock as the timed T presses,
so the time spent loading does not shift them relative to the menu opening.
Explicit test inputs are the Repair skill boost, simulated E presses, dialogue
choices, the six in-game wait-menu advances, `MoveTo` for Gibson and Ignacio,
and `StartConversation` for both. These inputs arrange the test and select
real dialogue; none forces quest state. Repair uses button answers `1,1,1`:
the ESM's activation blocks show the initial repair message twice, followed by
the “Run the repair” choice. Both log sessions are closed through their offered
`Log Playback::End` option.

## Validation status

On 2026-10-31, commit `9d60396b` passed the `ede` PowerShell acceptance
route on Fedora in a native Wayland session using the installed game data.
All expected lines were present, including both log results; the log recorded
six 23-hour waits. The viewer exited 0 with no panic after 507 seconds. This
was a development-profile build with reduced optimization for the
`engine-fallout` and `engine-havok` crates to fit the machine's memory, not a
release build. No side-by-side comparison with the original was made.

On 2026-10-26, the current PR #87 release viewer ran this route from a new game
in Primm. The log recorded the repair/recruitment, Gibson INFO `000841E7`,
the stage-10 result, and log one. After six in-game waits, Ignacio INFO
`000E35FF` set the VMS03 flag and returned to the ED-E quest script. The viewer
emitted “Two EDE logs heard,” played log two INFO `00158829`, and recorded
`iLogsPlayed` as 2. The route selected the offered `Log Playback::End`, exited
with code 0, and produced no panic. The final screenshot showed the player back
in the world with ED-E. This run predates the rebase and the shared-clock
change, and was not compared side by side with Fallout: New Vegas. The
PowerShell wrapper was not run on Fedora for that run; the equivalent viewer
argument route was run directly.

## Remaining quest coverage

- Later natural progression is not covered: radio/message branch, holotape
  pickup, Hidden Valley discovery, Followers handover, return-to-Primm
  `MenuMode`, completion package, and quest completion.
- The quest script (`00157F1F`) awards 100 XP after completion and before
  stage 100, guarded by `bCompleteOnce`; this route has not reached that
  condition, verified the award, or visually checked the XP meter. A previous
  forced-state run logged `XP +100`, but it does not prove natural completion.
- No side-by-side comparison with the original game has been made.
- A type-1 exterior second location still needs the game's worldspace and
  radius test; only the interior-cell shortcut is implemented (`00676390`).

## Implementation provenance retained for later route work

**Later Pip-Boy `MenuMode` step (not implemented in this route).** In
FalloutNV.exe 1.4.0.525, `MenuModeFunction` (`005c4240`) evaluates the
condition through `GetMenuModeConditionFunction` (`0059c380`). The quest
script is considered each `TES::RunScripts` pass (`00455490`), but
`TESQuest::Run` (`0060d4d0`) calls `Script::Run` (`005ac1e0`), which reduces
the quest script's delay and returns before running it while that delay stays
positive. After a run, it sets the next delay from the owning quest
(`00792760`) or the `fQuestScriptDelayTime` setting. Therefore a quest's
`MenuMode` block is checked when its script delay expires, not every frame.
This route stops after log two and does not exercise the later Pip-Boy return
step.

**Map discovery.** The game's proximity loop at `00779070` checks the player's
3D distance against a marker radius, then sets visible and travel bits through
`0044de40` and `0044de80`. `ShowMap` at `005c8620` can set visibility alone or
both bits; `GetMapMarkerVisible` at `005daac0` reads the 0/1/2 state through
`005a51e0`. The Hidden Valley marker (`000CDA74`) has `XRDS = 10,000`. In the
viewer, `world::map::discover` records a discovered marker, which the map
state exposes as visible and travelable. The partial ED-E route does not
exercise this path.
