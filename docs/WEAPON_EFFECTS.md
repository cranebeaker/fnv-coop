# Weapon fire effects and impacts (B5, B6)

Batch `claude/b5-weapon-effects`, 2026-10-07. Code: `crates/world/src/weapon_fx.rs`
(rules), `viewer/src/weapon_fx.rs` (sounds and flashes on screen), reports from
`viewer/src/combat.rs` (the player, `FireWeapon` objects) and
`viewer/src/fighting.rs` (people). Executable: `FalloutNV.exe` 1.4.0.525. Impacts
(decals, effect models, impact sounds) are B6: see [Impacts (B6)](#impacts-b6).

## What the game does (traced)

### Firing sound (`0083ac30`)

- Called by the weapon fire `00523150` with the weapon, the shooter, its fire node
  and "is the player". Guns only: thrown weapons (animation 10–13) take the other
  branch, and melee attacks don't go through `00523150`.
- Weapon sounds (`TESObjectWEAP`): `+0x21c` `SNAM` shoot 3D, `+0x220` `SNAM` shoot
  distant, `+0x224` `XNAM` shoot 2D, `+0x22c` `TNAM` (melee swing / dry fire),
  `+0x240`…`+0x248` the silenced set (`WMS1`, `WMS1`, `WMS2`), chosen when the
  player's weapon has mod effect 11 or 16 (`004bda70`; read for the player only).
- No 3D sound: nothing. The player with a 2D sound: the 2D sound alone (flags
  0x20181), first or third person. Everyone else: the 3D sound and the distant one
  (0x20182).
- Each starts only when the listener is within its largest distance (`SNDD` byte 1
  × 100) × a multiplier on each axis (`0082eca0`).
- Interior multipliers when the player's cell is an interior (flag 0x01) not
  behaving like an exterior (0x80; `00425fd0`, `00454b10`):
  `fWeaponInteriorNearVolumeMod` 1.0, `fWeaponInteriorFarVolumeMod` 0.1,
  `fWeaponInteriorNearAttenuationMod` 3.0, `fWeaponInteriorFarAttenuationMod` 0.75
  (exe defaults; the reads at `0083ad5b`…`0083ad88`). Outdoors all 1.
- Volumes: 3D near volume, distant far volume. Both buffers then get the **3D
  sound's** distances: smallest byte × 5, largest byte × 100 × near multiplier
  (disassembly `0083b4ba`…`0083b566`; the distant sound's bytes are read but unused,
  so when the 3D sound didn't start both are 0). A playing 3D buffer given a
  distance of 0 or less keeps 69.99 (smallest) and 1e9 (largest) (`00aefda0`), so
  that distant sound carries with no distance falloff at the far volume.
- Position: the fire node's (people), else the shooter's 3D; attached to it
  (`00ad8f20`), so it follows.
- 20 entries (0x24 bytes at `011dd9e8`: shooter, time, distance²). Scan: someone
  other than the player stops at their own entry; the oldest entry (ties to the
  later) and, for the player, the entry farthest from the listener beyond the new
  sound that isn't the player's. Oldest not playing (`00ad8ce0`) → it; else own
  entry; else (player) the farthest; else no sound.

### Loudness (the audio thread)

- `0082d400` sets a sound up from its record: static attenuation (`SNDD` i16 at 8,
  form `+0x4c`) by message 0x21, distances (byte 0 × 5, 14 when 0; byte 1 × 100,
  60 when 0), the curve (`SNDD` +12…+20, percent) through `00aeff60`:
  −2000·log10(p ÷ 100), rounded; 0 → 5000. The game sound's constructor
  (`00aeac20`) has 0, 600, 1400, 2600, 5000.
- `00aed990` every update: d = distance to the listener; if |max − min| ≤ 10 or
  d ≥ 1.1 max, attenuation 0 when d ≤ max else 10000; otherwise 10000 past max, 0
  within min, else linear along the curve at min, ¼, ½, ¾ of the way, max.
- `00aed660`: volume in DirectSound hundredths of a dB = trunc(2000·log10(volume))
  − static − distance (and two other fields not used here), clamped to
  [−10000, −1].
- DirectSound's own distance falloff is switched off (minimum distance 1e9 on the
  3D buffer, `00aec530`); it only pans.

### Muzzle flash

- Lit at each projectile's launch (`009bda10` → `009c2ff0`): `PROJ` flag 0x08 and a
  `NAM1` model; the shooter present, not dead (+0x22c), not knocked out (+0x230),
  for actors not in anim action 9 or 15–17 (`008a8870`).
- Actors keep one flash in their process (`+0x3d4`), made again when the projectile
  (model, light) changes (`009bb6d0`, `00902190`); other shooters a list
  (`00978740`).
- Flash object (`009bacb0`): shown, light on, fresh, rest (`+4`), time left (`+8`,
  the `PROJ` duration `DATA` 44). Lighting (`009bb690`) only when rest ≤ 0. Update
  (`009bb080`): light follows shown; rest −= dt; shown: left −= dt, at ≤ 0 hidden,
  rest = 0.1 s, left = duration. dt ÷ the time multiplier in V.A.T.S. or under
  Turbo.
- Placed at the fire node (actors vtable +0x20c; others `ProjectileNode` /
  `##ProjectileNode`, `00525700`); the player in first person at the first-person
  model's node (`009bb240`). Its light is the `PROJ` muzzle flash light (`DATA` 20,
  form `+0x74`, `009bb7f0`).

### Tracers

- Missile set-up `009b7cc0`: tracer = `rand % 100` < trunc(chance × 100)
  (`004fdf60`, chance `DATA` 24). Hitscan off in V.A.T.S. A hitscan non-tracer gets
  projectile flag 0x2, so no model is loaded (`009bda10`): ordinary bullets show
  nothing in flight. Tracers fade over the `PROJ` fade duration (`009becc0`).
- Vanilla: only `MinigunTracerProjectile` (0.3) and `AutoTracerProjectile` (0.5)
  have a chance; the 9mm, .357, varmint rifle and shotgun projectiles have 0.

### Melee swings

- `00899200` at the swing's hit: with no target found, the weapon's `TNAM` plays at
  the attacker (flags 0x102, its own distances), attached to it. A hit plays the
  impact sounds instead (`hiteffects`). The machete has no `TNAM`.

## Implemented

- `world::weapon_fx`: `WeaponSounds`, `SoundLevels` (distances, curve),
  `curve_millibels`, `distance_attenuation`, `millibels` / `amplitude`, `audible`,
  `FireSoundMods`, `plan_fire_sounds`, `FireSoundSlots`, `ProjectileEffects`
  (muzzle flash, tracer roll, model shown), `MuzzleFlash`, `shot_projectile`.
- Viewer `weapon_fx`: shots report (player's guns, people's guns, `FireWeapon`
  objects); the planned sounds play at the game's volume and are re-attenuated
  every frame from the camera's distance, following the shooter; the 20 entries are
  kept. Muzzle flashes: the `NAM1` model spawned once per shooter, placed every
  frame at the held weapon's `ProjectileNode` (first-person model in first person,
  the third-person body or the person otherwise), billboards facing the camera,
  shown and hidden by the game's clock. Melee swings that meet no one play `TNAM`.
- Changed: melee attacks no longer play the weapon's `SNAM` (the game doesn't); the
  player's 2D sound replaces the 3D one for the player.
- `NV_SHOT_ON_FLASH=player|npc`: the `--screenshot` waits for a flash on screen
  (test aid). The log prints each firing sound (`firing sound <EDID> (2D|3D|distant)
  from <shooter> at <d> units: <dB>`) and each flash lit (`muzzle flash <model> on
  <shooter> at <x,y,z>`).
- Tests: 9 in `world::weapon_fx` (generated records), 1 in the viewer's
  `weapon_fx` (loudness by distance).

## Seen live (release viewer, 2026-10-07; private outputs `nv-re\work\b5\live`)

- Goodsprings, `--weapon WeapNV9mmPistol`, the left button pressed every 0.4 s by
  `--key-at`: the flash shows at the pistol's muzzle in first person and, after
  `--key-at 3 f`, at the third-person body's pistol; the log has
  `WPN9mmFire2D (2D) … −10.1 dB` per shot (the sound's static attenuation).
- Ghost Town Gunfight (the acceptance route's commands): 108 flashes lit on the
  gangers (handgun, rifle flashes) at their weapons' nodes; the picture shows a
  revolver ganger's flash (its glow; the flash's planes point at the camera). The
  log has the gangers' 3D sounds fading with distance (.357 3D −10.6 dB at 102
  units, −28.6 dB at 1510) and distant sounds past the 3D range.
- Heard: not checked by ear (no audio capture in these runs).

## Not done

- Panning (DirectSound places 3D sounds left/right; Bevy plays them centred).
- The flash's light (the viewer's lights are fixed per surface at load), the flash
  model's animations (drawn as it stands) and particles (left out).
- Flashes for `FireWeapon` objects; Turbo / V.A.T.S. time scales on the flash.
- Projectiles in flight and tracers on screen (the viewer's shots are rays); the
  tracer's placement while it fades isn't traced.
- Thrown weapons still play their `SNAM` when thrown (pre-existing; `00523150`
  doesn't).
- The anim-action check (9, 15–17) on lighting a flash.
- Nothing compared with the original game.

# Impacts (B6)

Batch `claude/b6-impacts`, 2026-10-07. Code: `crates/world/src/decals.rs` (decal
rules), `crates/world/src/impacts.rs` (which impact), `crates/cellview/src/impacts.rs`
(effect models), `nif::Nif::own_controllers`, `viewer/src/hiteffects.rs` (what a hit
asks for), `viewer/src/impact_fx.rs` (drawing), `weapon_fx::play_at` (sounds). The
choice of impact (set by weapon, material by Havok material / `NAM4` / power armour,
blood, body part spatter set) was already traced: `world::impacts` and
`nv-re\findings\hiteffects.md`.

## What the game does (traced)

### A shot on the world (`009c20e0`)

- Impact = the projectile's weapon set at the struck sub-shape's material.
- **Decal**, when the impact has a texture set (`006733e0`): its size U(min width,
  max width) of the impact's own `DODT` (the record's `+0x54`: `004a40c0`,
  `004a40a0`, `00476b70`), chosen once per projectile and used for both sides; the
  `DecalCaster` query (`00622a70`: a shape of 32, or 256 for decals over 32 wide;
  layer 0x27) at the point; for each collision object found whose struck sub-shape
  has the impact's material, its scene graph (the land's geometry, or a reference's
  3D when its base takes decals, `004a1060`: not placeable water, people, creatures
  or projectiles) gets a decal (`004a3fe0` → `004a1a70`) with: the point, the hit
  normal, the size, the `DODT` depth (`008aff10`), a turn U(0, 1) (`004a4240`), a
  picture `trunc(U(0, 4))` (`00ec62c0`), the impact's angle threshold (`009a9350`),
  the colour's bytes 0–2 ÷ 255 (`004a4220`), the `DODT` flags and parallax values,
  and "glass" for material 3.
- `004a1a70` walks the node: skips "Decal Node", "Decal", "FaceGen", "BSFaceGen",
  "Bip01" and "Debug Decal Box" nodes, recurses through children, and only
  `NiTriStrips` (`011f4a20`) whose shader isn't itself a decal (`004a2020(0x1a)`)
  get one; at most `iMaxDecalsPerFrame` (`[Display]`, exe 10, `00f3d5a0`) a frame.
  One `BSTempEffectSimpleDecal` per piece (`0068ad20`), lifetime `fDecalLifetime`
  (`[Display]`, exe 10 s, `00f3d570`).
- **Effect model** (only within `fGunParticleCameraDistance`, 2048, of the camera
  and in view: `004b61d0` / `004b5ff0`, a sphere of 32 not outside any frustum
  plane; not while `[011dea2a]`): the impact's model at the point, its Z along the
  surface normal, back along the shot, or the reflection (`005f36f0`), for the
  impact's duration (`00508100`, `006890b0`).
- **Sounds**: `SNAM` and `NAM1` at the point, flags 0x4102, no distance test.

### The decal's geometry (`BSTempEffectSimpleDecal`, `0068b4c0`)

- Frame: `00c4b600` (the effect frame of `00c4b8a0`, rows X, Y, D) turned about D
  by the turn (`004a0c90`, `0043f8d0`): U = cos·X + sin·Y, V = −sin·X + cos·Y.
- Six planes (`0068d660`, `NiPlane`s at `+0x64`…`+0xb4`): |U·(p−o)| ≤ w/2,
  |V·(p−o)| ≤ h/2, |D·(p−o)| ≤ depth; worked in the piece's own space (sizes ÷
  its world scale).
- Triangles (`0068d230`, the strip's triangles with alternating winding,
  degenerate ones skipped): taken when the face normal n has n·D > |n| ×
  max(0.01, cos(threshold°)) (`0068b050`, global `0119c29c`); clipped
  Sutherland–Hodgman (`0068d820`, side 2 = outside, the cut's t clamped to 0..1
  with a log line); added as a fan (`0068cb60`) unless that would reach 512
  vertices (then the decal stops taking triangles); the piece's vertices are
  shared by index, new ones within 0.01 of an earlier new one.
- Vertex colour: the decal colour; alpha max(0, (n̂·D − c) / (1 − c)) by the
  (interpolated) vertex normal (`0068cb60`).
- Texture coordinates: u = (U·(p−o)/w + 0.5)/2, v likewise; +0.5 on u for
  pictures 1, 3, on v for 2, 3; a picture past 3 the whole texture (the decal
  textures are 2 × 2 grids).
- Drawn (`0068be90`): the texture set's textures, alpha blending (`DODT` flag
  0x02) and testing (0x04) from the decal data (`0049ed90`, `004393c0`; test
  function and reference `004393e0(4)`, `0094db80(0x80)`), parallax (0x01) with
  its scale and passes, a glass property for glass.
- Life (`0068c8e0`): full until its lifetime, then alpha 1 → 0 over one second,
  then removed. At most `uMaxDecals` (`[Decals]`, exe 100, `00f3d600`), the oldest
  removed (`0068be90`).

### A hit on someone (`0088e8d0`; sounds `0088e1e0`)

- Blood: the blood impact's model at the point, Z along −S (S = normalize(2 × hit
  direction + U(−0.8, 0.8)³)), lifetime 1 s or its animation; spatter: with chance
  `fCombatEnvironmentBloodChance` (0.75) a ray 512 units along normalize(S − 0.5 z)
  (layer 0x27); on the land, or a reference that takes decals, a decal of the body
  part's own impact, size U(min, max) (×1.5 under Bloody Mess), depth 48, the ray's
  hit normal.

### Effect models (`006890b0`, `00689310`, `00689840`)

- Lifetime = the longest controller span (at least 0.1 s), else what was given.
- The ballistic models carry their own `NiTransformController` (on an
  `NiBillboardNode` in mode 1, `ROTATE_ABOUT_UP`) and `NiAlphaController`.

## Implemented

- `world::decals`: `DecalBox` (frame, planes, triangle test, clipping, UVs, angle
  alpha), `DecalMesh` (fans, sharing, 512-vertex budget), `fade`, the limits; 6
  tests on generated triangles.
- `nif::Nif::own_controllers`: a model's direct transform and material-alpha
  controllers as one sequence (test on a generated NIF);
  `cellview::Game::effect_model` / `effect_piece_at`: the model's pieces posed by
  them from the effect's start, billboards turned, opacity (test).
- `preview` / `cellview`: `ModelMesh::strips` / `MeshData::strips` (which pieces
  are `NiTriStrips`).
- Viewer: decal receivers marked at spawn (strip pieces that aren't decals, the
  land's quarters); a shot on the world requests the decal on the objects whose
  collision near the point has the impact's material (the decal box is what's
  searched, since the clip keeps nothing outside it), the effect model (distance
  and frustum test) and the two sounds at the point (`weapon_fx::play_at`: the
  sound's own distances, re-attenuated every frame as B5's are). Hits on people:
  sounds at the point, blood model and wall/ground spatter where the hit met the
  body (player shots and people's shots; the struck part is now passed so the
  part's spatter set is found). Decal pieces are drawn lit with the texture set's
  diffuse and normal map, vertex colour and alpha, the decal depth pull; they fade
  and go as the game's do. `NV_SHOT_ON_IMPACT=1` makes `--screenshot` wait for an
  effect model on screen. Log lines: `impact sound <EDID> at <d> units: <dB>`,
  `impact effect <model> at x,y,z for <s> s`, `decal <texture> (<IPCT>) <size>
  wide on <n> piece(s) at x,y,z`.
- Guesses (`NV_GUESSES=1` only, [G] in the code): billboard mode 1 turns about the
  node's y toward the camera (Gamebryo's description of the mode; the exe's update
  isn't traced); no-lighting shaders fade by angle only with shader flag 0x40 (the
  impact models store a falloff of all zeros, which the viewer's rule hides; which
  technique the exe picks isn't traced).

## Seen live (release viewer, Goodsprings, `--weapon WeapNV9mmPistol`, `NV_GUESSES=1`, 2026-10-07)

- Ground (dirt, `-68233,1551,8500,195,35`): `BalisticDirtImpact`, sound
  `FXBulletImpactEarth` −8.4 dB at ~455 units, effect `ImpactBallisticDirt01.NIF`
  0.10 s, decal 25–31 wide on the land; the decal shows in a pixel difference with
  a shot-less picture at the crosshair (dirt on dirt: faint), and drawn flat red
  (a debugging build) as a quad on the ground.
- Wooden post (`…,159,6`): wood impact, −12.8 dB at 383 units, decal on the post
  (a narrow strip: the post is round and only faces within 15° take it).
- Metal water tower: metal impact, −14.3 dB at 670 units, decal made on one strip
  piece but not seen (it went on a piece the tank's visible surface hides).
- Boulder (stone, layer 13): concrete impact −26.9 dB at 1129 units; concrete wall
  at 2309 / glass at 1726 units: decals made, too far to make out.
- Sunny Smiles (moved in front, `--freeze-ai`): `FXBulletImpactFlesh` −10.1 dB at
  197 units, her hurt line, blood model spawned, and a blood spatter decal
  (`FXBloodSplatterDecals01.dds`, `BloodSpoutRedData`, 90 wide) clearly visible on
  the ground beside her.
- Effect models: spawned, posed by their controllers (scale 1 → 2, alpha 0.95 →
  0.08 over the wood model's 0.13 s, from the logs), placed and turned as
  intended (checked by drawing them opaque); drawn normally they are not visible
  in the pictures: the dust textures average alpha 0.1 in grey, the dirt puff
  skims along the ground, and head-on hits leave the quad edge-on; the blood model
  sits inside the body (the hit point is on the body's capsules).
- Heard: not checked by ear.

## Not done (B6)

- Decals on people (skinned decals, `004a2070`) and on the attacker's weapon
  (`0088fb00`); screen blood (B7); Bloody Mess's ×1.5 (actor value 55 is never
  raised here); parallax; the glass decal property.
- The `DecalCaster` query's own shape and layer (the viewer looks through the
  collision in the decal's box); the spatter ray meets every surface (layer 0x27's
  filter not applied).
- Blood models following the struck bone; effect models' particle systems; the
  `[011dea2a]` state.
- The exe's billboard mode 1 and no-lighting technique choice (guesses above).
- Nothing compared with the original game.
