# Head-track target: whom an actor looks at

Research status, 2026-10-04. `FalloutNV.exe` 1.4.0.525 addresses. This is
the half of head tracking that picks the target; turning the head toward it
is [OPENING_LOOK_IK.md](OPENING_LOOK_IK.md). Implemented in
`world::head_track` and the viewer's `ai` (see "Implementation").

Provenance: traced and written by a contributor on
`playcon/claude/head-track-target` (9defa47) and
`playcon/claude/facegen-eyes` (ae5b593, ddc0d59); ported on 2026-10-06
(`claude/m1-lookik-reconcile`) after re-checking 008a3100, 008a3ed0,
008a4810, 008a46c0, 00663510, 0064be40, 0064bf90 and 0064c410 in Ghidra
and their constants and setting names. One conclusion of that branch was
wrong and is corrected below ("Eyes"): LookIK's eye chain (mode 0) does
run for people, so the eyes aim at the target as well as dart.

## The target stack (`HighProcess +0x3f8`)

A high process keeps six target slots at `+0x3f8` (one reference each)
with a flag byte each at `+0x410`. The names come from `009016a0`, which
prints the current slot:

| slot | name | set by (virtual) | cleared by (virtual) |
| --- | --- | --- | --- |
| 0 | DEFAULT | `+0x624` (`00900ec0`) | `+0x640` (`009010f0`) |
| 1 | ACTION | `+0x628` (`00900f00`) | `+0x644` (`00901120`) |
| 2 | SCRIPT | `+0x62c` (`00900f50`) | `+0x648` (`00901190`) |
| 3 | COMBAT | `+0x630` (`00900fa0`) | `+0x64c` (`00901200`) |
| 4 | DIALOG | `+0x634` (`00901050`) | `+0x650` (`00901230`) |
| 5 | (unnamed) | `+0x638` (`009010a0`) | `+0x654` (`009012a0`) |

`+0x63c` (`00900ff0`) sets any slot by index.

* Setting slot 1–5 stores the target and sets the flag when the target is
  not null (clears it when null). Setting slot 0 stores the target and
  always sets the flag.
* The current target (`+0x678`, `009015f0`) is the target of the highest
  flagged slot, which can be null. `+0x67c` (`00901660`) gives that slot.
  `009014d0` caches it at `+0x41c` (read by `+0x68c`, `008d9830`).
* Clearing slot 1, 2, 4 or 5 with "demote" set (`00901120` and the like):
  the slot is emptied, its target becomes slot 0's target (slot 0's flag is
  left as it was) and the hold timer `+0x418` is set to
  `fAIHoldDefaultHeadTrackTimer` (10). Without "demote" only the slot is
  emptied. Slot 3 has no demote form.
* `+0x660` (`00901310`) empties every slot; `+0x664` (`00901390`) empties
  every slot holding a given reference.
* The actor may choose its own target (`+0x668`, `00901400`) only when no
  slot 1–5 is flagged **and** the hold timer is below 0.

### Who sets which slot

* `SayTo` (`005c9100`, its script command handler):
  unless told not to, the listener's SCRIPT slot is emptied and its ACTION
  slot set to the speaker (`005c9281`–`005c92bb`); then the speaker's
  SCRIPT slot is emptied and its ACTION slot set to the listener
  (`005c92c9`–`005c9303`). The line itself runs as a "Say To" dialogue
  package (process virtual `+0x2a4`).
* The "Say To" dialogue package (`008dbe30`, `HighProcess` virtual 169):
  when it ends, both actors' ACTION slots are cleared with demote
  (`008dbe30` lines at `+0x644(1)`).
* Conversations (`MobileObject` virtual 159 `00935480` and 160 `00934fd0`,
  which start the dialogue package): each side's ACTION slot is cleared
  with demote and its DIALOG slot set to the other. Virtual 158
  (`00933890`) sets the DIALOG slot to the player. Ending one
  (`MobileObject`/`Actor` virtual 162, `00933d20`, `008b1070`; and
  `00762160`) clears DIALOG with demote.
* `Look` (`005c9790`) sets the SCRIPT slot; `StopLook` (`005c98e0`) clears
  it with demote.
* Combat (`008a03a0`) sets and clears COMBAT.

## Choosing a target (`008a3100`, every actor update)

In order, for actor A with process P (`dt` the update's time):

1. Not the player; has a process.
2. If P's `+0x28c` reports one and A's cached distance to the player
   (`+0x5fc`, `008a3b50`) is below 0 or above
   `fAIMaxHeadTrackDistanceFromPC` (2000), nothing more.
3. A's update timer `+0x74` −= dt; P's hold timer `+0x418` −= dt.
4. If the current target is the player and A's detection entry for the
   player (P virtual `+0x504`, `008f6650`, the list at `+0x25c`) has a
   level (`+8`) below 1, and three process conditions hold (`+0x2d8`,
   `+0x30c`, a combat state), every slot is emptied.
5. If more actors than `iUpdateActorsPerFrame` ([HeadTracking], 1) have
   re-chosen this frame (`011df674`, reset each frame in `0086e650`;
   `008a33b6`–`008a33c0` skips when counter > limit, so two get through),
   nothing more.
6. A needs a 3D. A current target that is an actor without 3D empties
   every slot.
7. A's new-target timer `+0x158` −= dt.
8. When the update timer is ≤ 0 (`008a3531`): the frame's counter + 1; the
   update timer := random in [`fUpdateDelaySecondsMin`,
   `fUpdateDelaySecondsMax`] ([HeadTracking], 1 and 1.5; `00476b70`). If
   A may choose (above): the new-target timer is raised to at least 0; the
   chooser (`008a3ed0`) runs; if its pick differs from the current target,
   slot 0 := the pick (`+0x624`), the hold timer := 10 and the new-target
   timer := random in [`fUpdateDelayNewTargetSecondsMin`,
   `fUpdateDelayNewTargetSecondsMax`] (6 and 10).
9. Then the target's point goes to the look controller, or the look eases
   out (OPENING_LOOK_IK.md "On and off"). The look also eases out when A
   is not alive (`+0x108` life state not 0 or 5),
   `bDisableHeadTracking:HeadTracking` is set (`00408d60`), or an
   animation condition on the head node holds (untraced).

Body turning toward the target (`iActorTurnDegree`, `iActorKeepTurnDegree`
× 0.8) and the smile distance (`fAIMaxSmileDistance`) are read here too;
the body turn is the viewer's `ai::look_frame`.

### The chooser (`008a3ed0`)

* If A runs a package of type 6 or 9 (`009344a0`, `0041ca90`) with a target
  (`00671d10`, `0044ddc0`): that target, when within 500 (the player) or
  100 (others) of A. Otherwise:
* Every actor in the high-process list, then the player, that passes the
  filter `008a4810(candidate, 1)`; of those the one with the highest score
  `008a46c0` above 0. None: null.

### The filter (`008a4810`)

A candidate C passes when all of:

* C is not A, is an actor (virtual `+0x100`), not disabled (`00440da0`) and
  has 3D (virtual `+0x1d0`);
* the distance `005723b0` ≤ `fAIMaxHeadTrackDistance` (400; both branches
  read the same setting, `008a48ae`/`008a48bf`);
* the distance ≥ 100, or C is the player, or C is P's cached current
  target (`+0x68c`);
* the angle between A's heading and the direction to C, wrapped to ±π,
  is at most 120° (`0102b3c8`), or 180° when C is the current target;
* A or C is the player, or one of them has no `+0x214` value
  (`Character +0x1ac`);
* A detects C: the detection level from `008a0d10` is at least 1; and not
  (C is the player, the current target, and `008a0d10`'s last output is
  clear); and not (`004938e0` holds and its other output is clear).

### The score (`008a46c0`)

score = (1000 − distance) / (1000 − `fAIBestHeadTrackDistance` (500)),
then × 0.5 when A has no line of sight to C (`0088b880`), × (A's new-target
timer + 1) when C is the current target, × 0.5 when C is dying, dead or
unconscious (`Actor` virtual 139 with 0, `008844f0`: life state 1, 2 or 6).
A non-actor scores 0.

## Eyes (FaceGen, traced 2026-10-04)

The eyes aim at the target through LookIK's eye chain and dart around
that aim (corrected 2026-10-06; the contributor branch had the eyes dart
around straight ahead only, because it took LookIK's mode 0 never to run;
see OPENING_LOOK_IK.md, "Reconciled implementation"). While the actor has
a head-track target:

* The FaceGen node's update (`00663510`) runs the eye update `0064be40(dt)`
  after the face's keys when: the node animates (`+0xd6`), belongs to an
  actor (`+0xe8`) whose process has a current head-track target
  (`+0x678`), `bDisableHeadTracking` is clear, and three more conditions
  (`006639f0`, `+0xd8`, `004f0140`) hold. The eyes are otherwise left as
  they were.
* `0064be40` (skipped when `BSFaceGenAnimationData +0x18e` is set): the
  strongest expression (`0064bda0`: the highest weight in (0, 1], −1 for
  none) picks how the eyes dart (`0064bf90`), then the eyes turn from
  (`+0x140`, `+0x144`) toward the target (`fDesiredEyeHeading`/`Pitch`,
  `+0x150`/`+0x154`, Xbox PDB: written every update by the actor update
  `00888970` through `00888a20`, the look controller's eye heading and
  pitch `+0x19c`/`+0x1a0` while its look is on, else 0) plus the dart offset (`+0x180`,
  `+0x184`), at most `fTrackSpeed` (2) × dt radians each way, and
  `0064c410` applies them.
* `0064bf90`: the timer `+0x17c` runs down by dt; at 0 the strongest
  expression + 1 indexes the byte table `0064c3fc` into the jump table
  `0064c3e4` (expressions above 12 skip it):

  | expressions | offsets | timer and offset |
  | --- | --- | --- |
  | none, Anger, MoodCocky, MoodAngry | Angry | 30%: 2–3 s, 0; else 0.5–1.5 s, random |
  | Happy, Surprise | Happy | 30%: 3–4 s, 0; else 0.5–1.5 s, random |
  | Sad, MoodDrugged, MoodSad | Sad | 2–3 s; 30%: 0, else random |
  | Fear, MoodAfraid | Fear | 0.5–1.5 s; pitch 0; 50%: heading 0, else random |
  | MoodNeutral | Neutral | 30%: 3–4 s, 0; else 0.5–1.5 s, random |
  | MoodAnnoyed, MoodPleasant | — | nothing changes |

  "random" is heading in [`fEyeHeadingMinOffsetEmotion…`,
  `…Max…`] and pitch in [`fEyePitchMin…`, `…Max…`] (radians; `00476b70` is
  uniform, `004dff00(p)` is true with chance p).
* `0064c410` (when the global `011d59e0`, set by the FaceGen manager, is
  on): heading kept within ±`fTrackEyeXY` (28°) and pitch within
  ±`fTrackEyeZ` (20°), each range kept within 0–90° (`00649f00`,
  `00649f70`); then LookLeft (modifier 9) or LookRight (10) = |heading| ÷
  range, LookDown (8) or LookUp (11) = |pitch| ÷ range, written straight
  into the modifier weights (`0064a4d0`).

Implemented as `world::face::FaceAnimation::track_eyes` and
`world::face::eye_morphs` (0064c410, checked bit for bit with nv-call),
called by the viewer's `faces::animate_faces` with the look controller's
`desired_eyes` for actors whose `Walker::looking_at` is someone.
Expressions aren't driven in the viewer, so every face uses the Angry row
(no expression). The three unnamed gate conditions aren't modelled.

`fTrackXY`, `fTrackMinZ`/`fTrackMaxZ`, the fudges and
`fTrackJustAcquiredDuration` are read only by their static initialisers, so
they have no effect in this build. When a head is attached (`00607420`),
LookIK's eye chain (mode 0) gets `fTrackEyeXY` (28) as its cone
(`00607810(0, ..)`), `fTrackEyeZ` (20) is stored at controller `+0x1a8`
(`00607830`; no reader found) and the dead zones `fTrackDeadZoneXY` (20)
and `fTrackDeadZoneZ` (10), each at most its range (`00649fe0`,
`0064a070`), become the eye bone's `fDeadHeading`/`fDeadPitch`
(`00c748d0(0, ..)`): beyond them the head turns too. Settings registered
by 00f80fc0, 00f80ff0, 00f81020, 00f81050 and 00f80f00 (`fTrackSpeed`);
the emotion offsets by 00f81140..00f814d0 (defaults in
`EyeSettings::DEFAULT`, re-read 2026-10-06).

## Settings

| setting | default | object |
| --- | --- | --- |
| `fAIHoldDefaultHeadTrackTimer` | 10 | `011cdbb8` |
| `fAIMaxHeadTrackDistance` | 400 | `011cd938` |
| `fAIBestHeadTrackDistance` | 500 | `011cd114` |
| `fAIMaxHeadTrackDistanceFromPC` | 2000 | `011cd2f4` |
| `fAIInteriorHeadTrackMult` | 0.5 | `011cd920` (read by `008a2ed0`, not by the above) |
| `bDisableHeadTracking:HeadTracking` | 0 | `011df6d0` |
| `fUpdateDelaySecondsMin/Max:HeadTracking` | 1, 1.5 | `011df810`, `011df828` |
| `fUpdateDelayNewTargetSecondsMin/Max:HeadTracking` | 6, 10 | `011df700`, `011df78c` |
| `iUpdateActorsPerFrame:HeadTracking` | 1 | `011df848` |

Game settings can be overridden by `GMST` records; the INI ones by the INI.

## Implementation (2026-10-04)

`world::head_track` holds the stack (`HeadTrack`), the settings, the
filter (`accepts`) and the score (`score`) on facts the caller supplies
(`Candidate`), the chooser (`choose`) and the update (`HeadTrack::update`).
The viewer (`viewer/src/ai.rs`) keeps one per walker:

* `SayTo` from a script to the player: while the line plays, the
  speaker's ACTION slot is the player; when it ends, it is cleared with
  demote. The dialogue menu with the player uses the DIALOG slot the same
  way.
* The viewer's existing "Say To" package step, greetings (ACTION) and
  conversations between people (DIALOG) set their slot while the line or
  conversation lasts and clear it with demote after. Before, they set a
  look with a fixed 10 s timeout.
* The chooser runs on the timers, frame budget and the 2000 limit above,
  with candidates from the viewer's own list of people: distance and angle
  from their positions, detection from the viewer's detection run (an
  actor "notices" someone), line of sight by the viewer's ray cast.

Inferences, labelled in the code: the viewer's "noticed" stands for
detection level ≥ 1; the conditions in the filter's last bullet and
step 4's process conditions are not modelled; the package-target branch of
the chooser is not modelled (no package of type 6 or 9 in the viewer's
packages is tracked); "Say To" ends when the line ends. Also not modelled:
008a3100's early branch for a process package with flag 0x100000 (it eases
the look out) and its return when an actor with FaceGen data is farther
from the player than an untraced setting (the test after `008adcb0` in
`008a3100`).

## Still open

* What `004938e0`, `+0x2d8`, `+0x30c` and the head-node animation
  condition are.
* Which systems set slot 5.
* Comparison with the original game: whom Doc and Goodsprings residents
  look at, and when they look away.
