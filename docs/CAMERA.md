# Player camera and third-person body

Topic file for the player's first/third-person camera, the view key, the
wheel zoom, vanity mode, the temporary views (furniture, Pip-Boy,
dialogue) and the player's third-person body. Read from `FalloutNV.exe`
1.4.0.525 (unpacked image) with Ghidra; names marked (Xbox PDB) come from
`Fallout_Release_MemDebug.pdb` (ADR-0002), matched by behaviour and the
`PlayerCharacter` layout. Branch `claude/m2-third-person`, 2026-10-06.

Status words: **implemented** (code exists), **tested** (regression
test), **compared** (checked against the running original). Nothing here
has been compared with the original game.

## Engine structure

FNV has no camera-state classes (Skyrim's `PlayerCamera`/`ThirdPersonState`
do not exist here). The state is `PlayerCharacter` fields and globals in
`.data`; `PlayerCharacter::UpdateCamera` runs a first-person or a
third-person branch. PC field offsets are the Xbox ones less 0x10 from
+0x648 on (confirmed by the code that reads them).

| PC field / global | Xbox PDB name | Meaning |
| --- | --- | --- |
| +0x64a | `b3rdPerson` | camera is (or is leaving) third person |
| +0x64b | `bActually3rdPerson` | third-person body shown (`00951a10`) |
| +0x64c | `bWant3rdPerson` | wanted view |
| +0x64d/+0x64e | `bTemp3rdPerson`, `bTemp3rdPersonSwitchBack` | furniture/vanity temporary view |
| +0x64f/+0x650 | `bTemp1stPerson`, `bTemp1stPersonSwitchBack` | Pip-Boy/iron sights temporary view |
| +0x670/+0x678 | `fWorldFOV`, `f3rdPersonFOV` | FOVs; +0x678 = `fOverShoulderFOV` (`00939197`) |
| +0x680 | `ucControlsDisabled` | bit 0x10 = POV (`0095f590` clears +0x64c) |
| +0x698 | `fEyeHeight` | pivot height |
| +0x21c | `pCameraCaster` | camera collision caster |
| +0xd58 | `kCamera3rdPersonShoulderOffset` | shoulder offset, world |
| `011e0b5c` | – | zoom (wanted distance) |
| `011e0768` / `011e07dc` | – | camera distance / switch closing rate |
| `011e0808` | – | smoothed camera target |
| `011e07b8` / `011e07b9` | – | view key held / vanity mode |
| `011e07c1/c2/c3/c4/c8` | – | key down this press / blocked / snap / idle time / hold time |
| `011e0b60` / `011e0b58` | – | orbit yaw / pitch offsets while the key is held |
| `011e08fc/08f4/0bb8/07bc` | – | vanity heading / pitch / phase / saved zoom |

## Functions (`world::player_camera`)

| PC | What | Status |
| --- | --- | --- |
| `0094ae40` | `PlayerCharacter::UpdateCamera` (Xbox PDB): third-person branch: R = Rz(heading + yaw offset)·Rx(pitch + pitch offset); pivot = root + fEyeHeight × scale; desired = pivot + R·((PosX·b, −zoom, PosZ·b)), b = 1 − zoom / fOverShoulderStartBlendDist; looks at pivot + R·(PosX, 1000 − zoom, PosZ); FOV f3rdPersonFOV. Vanity: Rz(heading + vanity heading)·Rx(vanity pitch), pivot + 100 × scale, looks at a point beside the pivot (OffsetPoint/Zooming × (1 − fOverShoulderRotMult)) | implemented, tested |
| `0094a0c0` | `UpdateChaseCamera` (Xbox PDB): distance from the chase pivot; switches move at fChase3rdPersonZUnitsPerSecond × dt × (MinMult + (zoom − WheelMin)/(WheelMax − WheelMin) × MaxMult); caster hit pulls in at once to hit − fCameraCasterSize/2 (blocked if > 2 short); otherwise eases out at the same speed, ≥ WheelMin unless blocked, ≤ fChaseCameraMax (WheelMax while the key is held); snap distance fZoom3rdPersonSnapDist ends a switch to first person / jumps a switch to third; vanity smoothing × fChase3rdPersonVanityXYMult | implemented, tested |
| `00950110` | `SetFirstPerson` (Xbox PDB); dead → zoom = fVanityModeWheelDeadMin (vfunc +0x22c = `IsDead`, Xbox slot +0x228) | implemented, tested |
| `00950290` | end of a switch: closing rate ≤ 30 (double `0101db88`) → b3rdPerson = wanted; first-person body shown | implemented, tested |
| `00950340` / `009503d0` | `ForceTemp3rdPerson` / `UpdateTemp3rdPerson` (Xbox PDB); forced zoom ≥ fVanityModeForceDefault | implemented, tested (furniture uses it) |
| `00950460` / `00950530` | `ForceTemp1stPerson` / `UpdateTemp1stPerson` (Xbox PDB); called by the Pip-Boy opening (`0070ee80`) | implemented, tested; iron-sights part not |
| `009500a0` | `StopVanityMode` (Xbox PDB) | implemented, tested |
| `00953060` | `FocusOnActor` (its error string): view key released, SetFirstPerson(1), switch end, temp-3rd update | implemented, tested |
| `00942be8`..`00942e05` | view key = control 13: press → third person + held; release within fVanityModeDelay with unchanged zoom → back to first if third was shown | implemented, tested |
| `00945a17`..`00945c62` | wheel (÷120 per notch): in −zoom×InMult, out +zoom×OutMult; < WheelMin → first person; first person + wheel back → third | implemented, tested |
| `00945ce4`..`00945d92` | mouse while the key is held: yaw += dx·π/180·fVanityModeXMult·dt, pitch += dy·π/180·YMult·dt; normal look skipped | implemented, tested |
| `00943483`..`0094362b` | vanity mode after fVanityModeAutoDelay idle (`bDisableAutoVanityMode` respected): AutoXSpeed turn, sin(phase) × AutoYDegrees tilt; any input stops it | implemented, tested; the trig function `00eca060` read as sine (not confirmed) |

Settings with data overrides (FalloutNV.esm): fVanityModeWheelMax 250,
fChase3rdPersonZUnitsPerSecond 800, fOverShoulderStartBlendDist 40000,
fVanityModeAutoDelay 120; others are exe defaults (see the module).

## Viewer (`viewer/src/player_camera.rs`, `player_body.rs`)

- F is the view key (the game's Toggle POV); the viewer's walk/fly toggle
  moved to ` (backquote). Wheel zoom; holding F orbits with the mouse.
- The camera entity stays the eye during the frame (aim and activation
  unchanged, `docs/VR.md` rule 2); `place_view` (PostUpdate, before
  transform propagation) moves it to the third-person place and
  `restore_eye` (PreUpdate) puts the eye back. Collision: the cell's
  `physics::Collider` ray (the game uses a sphere caster: approximation).
- FOV: written only when the wanted value changes (third person 55,
  else the world's 75), not while V.A.T.S. is on.
- Furniture (`sitting.rs`) now begins/ends the camera's temporary third
  person; `IsPC1stPerson` is taken as "first person wanted".
- Body: `world::actor::player_look` (record traits; state sex, worn
  armour, weapon) built with the NPC path, `ActorRig` animated by
  `animate_actors`: idle, directional walk/run (`mt{backward,left,right,
  fast*}.kf` beside the walk; rate from the Forward group, `00895110`),
  weapon aim/attacks, furniture entry/exit/seated loop. Shown iff
  `bActually3rdPerson`; the first-person view hides then.

Tests: `world` `player_camera::tests` (9), `world/tests/player_look.rs`;
viewer `player_camera::tests`, `player_body::tests`,
`actors::tests::backing_up_plays_its_own_group_at_the_forward_groups_rate`.

## Gaps (labelled in code, not substituted)

- Not compared with the original game (distances, speeds, look point,
  collision feel, vanity mode).
- Pivot height term `011e07d0.z − 011e07d4.z` (first-person nodes) taken
  as 0; `fEyeHeight` is the viewer's 120; player scale 1.
- Body fade when the camera is inside the player (`006214d0`,
  `011a3b38`), HUD mode changes (`00771700`), kill-camera mode, iron
  sights forcing first person, unknown globals `011f21d0/1`, `011e0780`,
  `011e0788`, `011a3b31`, process vfunc +0x610.
- The game's camera update while in menu mode (Pip-Boy) isn't traced; the
  viewer keeps updating.
- Body: sneak and jump groups, weapon-kind movement groups (back/left/
  right play nothing with the weapon out), turning in place, blinking and
  lip sync, death ragdoll.
- Billboards and the sky dome follow the eye, not the third-person camera.

Next action: compare in the original game: tap/hold F, wheel steps,
camera distances against a wall, and Doc's couch entry/exit framing.
