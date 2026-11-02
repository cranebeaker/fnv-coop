# Physics: clutter Havok moves

Free rigid bodies (clutter, props, weapons lying about) as the game hands
them to Havok: read from the models, simulated, pushed by shots, blasts,
the player and people, carried with the Grab key, sounding when they hit
something, drawn where they are and kept where they come to rest.
Branches `claude/m2-physics`, `claude/m2-physics-2` and `claude/m2-physics-3`, 2026-10-06.
Private exports (decompiles, logs, frames):
`%USERPROFILE%\nv-re\work\physics-2026-10-06`, `…\physics2-2026-10-06` and `…\physics3-2026-10-06`.

Status words: **traced** (read in FalloutNV.exe 1.4.0.525 or the game's
data), **implemented**, **tested** (generated regressions), **verified
live** (viewer run on installed data), **not compared** (nothing here has
been checked against the running original game).

## The VCG02 bottles (Back in the Saddle)

`VCG02Bottle` (MISC `0010A1F6`, "Sunset Sarsaparilla Bottle") is ordinary
Havok clutter: model `clutter\junk\SSBottle02.NIF`, one
`bhkConvexVerticesShape` hull (18 corners, 14 faces, convex radius 0.1) on
a `bhkRigidBody` on layer 10 (props): mass 1, linear/angular damping
0.1/0.05, friction 0.5, restitution 0.4, most speeds 1068 Havok units/s and
31.57 rad/s, motion system 4 (box inertia), quality 3 (debris). No `DEST`
data: the bottles don't break, they get knocked off. The seven placed
bottles (`0010A202`–`0010A209`, not `206`) follow `VCG02BottleMarkerREF`
(initially disabled) and stand on the fence rail at z 8438.1; their record
flags are 0 (no "Don't Havok Settle"). Their script `VCG02TargetSCRIPT`:
`OnLoad SetDestroyed 1`, `OnHitWith` counts `VCG02.nTargetCount` (once per
bottle, weapon out, animation types 4–8), Sunny's `SayTo`,
`SetObjectiveCompleted VCG02 10` at 3 and `SunnyREF.evp`; `OnActivate`
(after the quest) `SetDestroyed 0`.

`SetDestroyed 1` is why the game shows no prompt on them: a destroyed
reference (form flag 0x800000, `00477ba0`) under the crosshair gets no
activate prompt unless it is an actor (`00775a00`, the HUD's crosshair
update, clears the prompt tiles), and activating one returns at once
(`005180b0`). The crosshair still finds them, so the Grab key works on them.

## Traced

| What | Where | Rule |
| --- | --- | --- |
| Rigid body block | `bhkRigidBody` (236 bytes, checked on the bottle) | inertia rows 116, centre 164, mass 180, damping 184/188, friction 192, restitution 196, most speeds 200/204, penetration depth 208, motion 212, deactivator 213, solver deactivation 214, quality 215, constraint count 228 (`nif::RigidBodyInfo`) |
| Step clock | `00c66760` (`iUpdateType` 0 from `[HAVOK]`, `0044fb20`) | fixed steps of `fMaxTime` (0.016, `Fallout_default.ini`) × the time multiplier at `011ac3a0` (1); steps = the accumulated time ÷ step rounded, at most 3; the rest (can be negative) carried, held to one step; under half a step waits (`physics::rigid::Clock`) |
| Gravity | `00f4b550` | 98.1 Havok units/s² (existing `physics::GRAVITY`) |
| Havok world settings | `00c681c0` over Havok's defaults `00c90b80` | collision tolerance 0.1, contact resting velocity FLT_MAX, simulation type from `iSimType`, expected step `fMaxTime`; kept from Havok: solver 4 iterations, tau 0.6, damping 1, deactivation reference distance 0.02 Havok units |
| Collision filter (layers) | `bhkCollisionFilter`: table built by `00c828f0` (with `00c827f0`, `00c82870`), asked by `00c84740` (bodies `00c84880`, casts `00c84930`) | 43 × 64-bit rows; bodies of different groups touch when the row of the first has the second's layer; "no collision" flag 0x4000; one ragdoll's bones by a part table (`physics::layers`). Row 30 (character) equals the walking layers worked out earlier; row 6 (projectile) lacks 3 (`TRANSPARENT`), 7, 16, 21, 22, 31, 35, 42, which the character's has, and has 8 (biped), 23, 29 (dead biped), 37, which it lacks |
| Contact materials | `00cfd800` (the contact manager's new point) | friction √(f₁f₂) (entity `+0x90`), restitution √(r₁r₂) (`+0x94`) kept as a byte ×128 rounded |
| Land body | `00621f60` | fixed body, friction `[Landscape] fLandFriction` (2.5), Havok's default restitution 0.4 (`00c8f510`) |
| Gameplay impulse scaling | `TESHavokUtilities::ScaleGameplayImpulseForce` (Xbox PDB), `0062b520` | × `fGameplayImpulseMult{Biped (layers 8, 29) 0.2, Prop (10) 0.1, Trap (14) 0.15, DebrisLarge (20) 1, Clutter (else) 1}`; × mass ÷ `fGameplayImpulseMinMass` (5) when lighter; × `fGameplayImpulseScale` (150) |
| Shot push | `Projectile::ApplyImpactForce` (Xbox PDB), `009c2e80`, from `Projectile::ProcessImpacts` `009c1b70` | normalized projectile velocity (`+0x104`) × PROJ impact force (`+0x94`, `00644930`) scaled as above, applied at the impact point to bodies whose motion type is below 4 (`00517630`); not for projectiles that explode on impact |
| Explosion push | `Explosion::PushRigidBody` (Xbox PDB), `009b0920`; `Explosion::ApplyForces` `009afef0` | EXPL force (`+0x74`) > 0; direction explosion → body, normalized, z + `fExplosionForceClutterUpBias` (0.5) for biped/dead-biped layers only (`00624070`); × `fExplosionSourceRefMult` for the source's own body (then uncapped); scaled as above; min `fExplosionMaxImpulse` (8000); linear × `fExplosionForceMultLinear` at the centre, angular random(−1..1)³ × force × `fExplosionForceMultAngular`; bodies that move or are large debris; "push source only" (flag 0x20) |
| Grab (Z) | `PlayerCharacter::HandlePhysicsGrab`, `::CreateMouseSpring`, `::UpdateMouseSpring`, `::DestroyMouseSpring` (Xbox PDB): `0095f6c0`, `0095f930`, `00960520`, `00961280`; spring `hkpMouseSpringAction::applyAction` `00cbb1e0` | see `physics::grab`: control 27 ("Grab", Z) toggles; crosshair reference whose body moves and weighs ≤ `fGrabMaxWeightWalking` (100); held at the picked point's distance, ≥ controller radius + 5; target = eye + view × distance cut by a cast; lets go past 96 units, or past `fZKeyMaxContactDistance` (10) against a body > `fZKeyMaxContactMassRatio` (4) × as heavy; spring `fZKeySpringDamping` 0.5, `…Elasticity` 0.2 (trap layer × 0.1), `fZKeyObjectDamping` 0.75, `fZKeyMaxForce` 750 (trap × 0.5); the spring: velocities × object damping, impulse −K⁻¹(damping·v + elasticity/dt·error) at the point, at most dt·mass·force. No throw: no setting or code path found |
| Contact listener | `FOCollisionListener::contactPointAddedCallback` (Xbox PDB), `00623cb0` | for each contact point added: sound when |projected velocity| × 7 ≥ `fMinSoundVel` (10) and ≥ 1; then physics damage |
| Impact sounds | `ImpactMixer::PlayCollisionSound` (Xbox PDB), `00837550`; picker `00839e00` | each side's sound by its Havok material, its mass (5 without a moving body) and the speed (`fCollisionSoundHeavyThreshold` picks H/L sets; `f…MediumMassMin`/`LargeMassMin`, glass/wood-for-water 6/15, grass 6/70; below 1e-4 the "Static" sounds); nothing for skin on skin; one per material pair `(a+1)(b+1)` per `iCollisionSoundTimeDelta`; static attenuation (1 − min(speed/400, 1)) × 3000 hundredths of a dB; command 0x42 with 1 or (4500 − attenuation)/3000 (read as the frequency) |
| Physics damage | `FOCollisionListener::StoreObjectDamage` `006238b0`, `0062be90`, `DealObjectDamage` `00623640` | |projected velocity| (Havok units) ≥ `fPhysicsDamageSpeedMin` (150); a side takes it if its reference has destructible data (flag 0x1000000), isn't destroyed and neither side is a projectile; damage by the other side's mass (fixed: 10000): < 10 none, < 50 1 above 500, < 100 5 above 350, else 10 above 150, × (1 + 0.0001 × speed); dealt to the reference later (`+0x144`, the object damage virtual) |
| Character contacts | `00c711d0` with `fMoveLimitMass` (95, `011b0128`) | the controller's contact callback treats bodies ≥ 95 apart (keeps the surface velocity only for lighter ones); read as: walkers push only lighter bodies |
| Saved Havok data | `TESObjectREFR::SaveHavokDataForCollisionObject` / `LoadHavokData…` (Xbox PDB): `00563220`, `00563380` | per body: position and rotation, a flags byte (bit 1 active, bit 2 keyframed), then for an active body its linear and angular velocity; loading sets both and activates it |
| Moved references | `0083fef0` names `CHANGE_REFR_HAVOK_MOVE` (flag 4) | moved objects are saved where they are |
| Adding bodies asleep | `bhkWorld` add (`00c6b0a0`), batch add (`00c674d0`), `hkpWorld::addEntity` `00c914d0`, `::addEntityBatch` `00c94bd0` | a loading place's bodies are queued and added in a batch with activation 0 (don't activate); only those with a linear or angular velocity are then woken (`00c9c1d0`). A body added alone is left asleep too when still and not on the biped layer (flag `011b0c44`, 1). Nothing settles placed clutter at load |
| Statics fixed | `TESObjectREFR::InitHavok` (Xbox PDB, vtable `+0x1c4`), `005768b0` | a reference of base form type 0x20/0x21 (`STAT`, `SCOL`) has its bodies set to motion 5 (fixed) through `00c6a350`, which acts only when the model's `BSXFlags` has bit 1 (Havok) |
| Wind | `TESObjectCELL::InitHavok` (exterior) `00554010` adds `TESWindListener` (`00554590`; `bhkWindListener`, Xbox PDB) as a world entity listener; `GetRB` `00c9a430`; `SetWind` `00c74550` from `00453550`; `Update` `00c74570` from `00c6ae70` → `00c66e20` | see `physics::wind`: bodies whose `bhkWorldObject` flags have `WIND` (1; the NIF rigid body's last word, read into `+0x10` by `00c8ea30`); speed = `fMaxWind` 250 × the sky's wind (weather `DATA` 0 / 255, faded, `0063c490`), direction = the sky's `+0xd0` (1 rad, set by its constructor `00639d40` only); interiors none. Per body per frame: count 1 + ⌊dt/0.0167⌋, speed random(±w) + w/4 within 0–250 × count, heading dir + random(±π/4) within 0–2π, force (0, speed, 0) turned about z, `applyForce(dt)` after waking |
| One ragdoll's bodies | `00624070` | the filter's "linked parts" test is "layer 8 or 29"; bodies of one ragdoll (one system group) meet by the part table `01268078` |
| Biped parts' bones | `004ac1e0` (the biped's slot loop), `004aad00`, tables `01188b74` / `01188be8` | an unskinned part (`004910d0` false) is attached under its slot's bone: head, hair, headband, hat, glasses, nose ring, earrings, mask, choker, mouth → `Bip01 Head`; weapon → `Weapon`; Pip-Boy → `Bip01 L ForeTwist`; backpack → `Bip01 Spine2`; necklace → `Bip01 Neck1`; body, hands and add-ons must be skinned. A model whose flags (`MODD`/`MOSD` bit 0, `TESModel` `+0x14`, `004ae8f0`) mark it FaceGen takes the NPC's face morphs |

Unused finds: `0081f000`/`008d0020` are another knock path (base 1750 or
the object's `+0x98`, `fProjectileKnockMult…`, `fProjectileCollisionImpulseScale`)
reached from `00810aa0`/`008133f0`/`008cba30`, not the bullet path. Mode 3
of the mouse spring is telekinesis (`fMagicTelekinesis…`).

## Implemented

- `nif::RigidBodyInfo` on every `CollisionPart` (values in the model's
  space and game units), with its constraint count.
- `physics::layers`: the collision filter's table and word test
  (translated). `Collider` triangles carry their body's layer;
  `Collider::raycast_layer` casts as a layer does.
- `physics::rigid`: `RigidWorld`, `Clock`, bodies with their
  hulls/spheres/capsules, contacts against the collider, each other,
  the character proxy's pushes (`apply_surface_interactions`), friction and
  restitution combined as the game does, damping, speed limits, sleeping
  and waking, Havok-unit impulses, `Spring` (the mouse spring),
  `ContactEvent`s (a pair beginning to touch, with its closing speed),
  `set_velocity`. `physics::impulses`: the three pushes. `physics::grab`:
  the grab's rules. `physics::contacts`: sound choice, attenuation, the
  pair key, physics damage. `physics::LAND_SURFACE` on the terrain.
- `preview::CellScene::dynamic_bodies` (one moving body per reference;
  those parts left out of the static collider); models with several
  moving bodies or constrained bodies stay solid and are listed at load
  (`unsimulated_bodies`).
- `world::GameState::havok_moved` (`havokmove`) and `havok_velocity`
  (`havokvel`) in saves.
- Viewer `clutter`: bodies registered (at their saved pose and velocity;
  once the collider has ground under them; asleep), moved in
  `CellCollision` and drawn; shot pushes and blasts;
  the player and every moving `Walker` push through their proxies (and `clutter::actor_walks`
  for actor controllers to report their controller); the Grab control
  (`grab_held`); contact sounds through `SoundRequests` with the material
  pair gate; physics damage worked out and logged for destructible
  references. `combat`: shots cast on the projectile layer; scripted
  objects with bodies are met by their triangles where they are now.
  `scripts`: no prompt on destroyed references; bodies picked where they
  are. `walk`/`hud`: only doors are taken for swinging doors.
  `controls`: the Grab binding.

Batch 3 (`claude/m2-physics-3`):

- Bodies arrive asleep and stay put until something touches them (the
  invented load-time "settle" wake and `DynamicBody::settle` are gone).
- `preview::cell`: statics and static collections with Havok-flagged
  models keep their bodies solid (`fixed_by_game`); `Model::bsx_flags`,
  `nif::Nif::bsx_flags`.
- `nif::RigidBodyInfo::body_flags`, `nif::collision::BODY_WIND`;
  `RigidSetup::wind`; `physics::wind` (translated); viewer `clutter` sets
  the wind from the weathers' mix once a frame and pushes the wind bodies
  (`RigidWorld::apply_force`), logging where they are every 5 s.
- `nif::ragdoll` reads `bhkRigidBodyT` bodies (placed relative to their
  bone): the dog's pelvis/spine bodies and the eight constraints that
  join her legs, head and tail to them.
- `physics::ragdoll`: `BodySetup::layer`/`part`, `parts_meet`, pairs of a
  ragdoll's own capsules as contact points in the solve (PR 8, below).
- `world::actor::slot_parent_bone`, `ActorPart::parent_bone`,
  `Armor::pieces_facegen`; `preview::actor` hangs unskinned armour pieces
  from the slot's bone (FaceGen ones upright, as head parts).

This solver's own (labelled in code; Havok's contact solver isn't
translated yet, B1 PR 7): XPBD substeps (8) and passes (4) for bodies with
something near them; contact generation (no edge-against-edge
between bodies); overlap recovery at most 0.05 units per correction (walkers
as unstoppable capsules reaching 2 units out were this solver's, replaced by
the character proxy's push, "The character proxy"); a contact "added" is a pair that
starts touching (Havok adds and removes single points).

## Havok's world step (B1, PRs 1–4, `claude/b1-havok-step`, 2026-10-07)

The game's own Havok (**7.1.0-r1**, build tree dated 2009-12-22, strings
`010cf778`/`010d6628`) translated from FalloutNV.exe 1.4.0.525, names from
the Xbox prototype's PDB. No Havok SDK, header or documentation was used.
Code: `physics::havok`; used by `physics::rigid` and `physics::ragdoll`.
Units are Havok units (1 = `HAVOK_UNIT`, 6.9991 game units) unless said.

### The frame (`bhkWorld::Update` `00c6ae70`, vtable `+0xc4`)

Per frame and loaded world: batched adds (≤ 100 a frame from a queue of
≤ 3000, `00c674d0`), constraints (≤ 200), actions (≤ 100); then, when the
world is enabled (`+0x15`) and the frame time (`011afe68`) is over 0: the
step = `fMaxTime` × the time multiplier (`01267b38` × `011ac3a0`, set again
here whatever the clock computed); `bhkAction`s updated (`00c6a540`);
`bhkWorld::ScaleSolverInfo` (`00c66a00`); pending operations;
`hkpWorld::setFrameTimeMarker` (`00c91040`, marker = current time + the
accumulated time `011afe64`); `hkpWorld::stepDeltaTime(step)` (`00c91b10`)
until `isSimulationAtMarker` (`00c91060`, exact equality); then the entity
listeners with the frame time (`00c66e20`: wind, traps, water) and the
debugger; batched removes after.

The clock, `bhkWorld::SetDeltaTime` (`00c66760`, `[HAVOK] iUpdateType` 0):
accumulated = carried (`012677b0`) + frame, held to 166.67 s; ≤ 0: no
steps, the carry untouched; n = round(accumulated ÷ step) (x87: halves to
even), at most 3 (1 when the second argument is set: `00525420`, a game
mode of 4, not followed); n > 0: carry = accumulated − n·step, held to at
most a setting read through `00403e20` (taken as one step, not traced);
accumulated = n·step, so the loop runs exactly n whole steps and bodies are
read at a whole step (no interpolation). n = 0: under half a step it all
waits (unless forced by the third argument or `01267b4c`); exactly half a
step (rounded to 0) runs one step here (Havok's snapping of a step to the
marker isn't traced). `physics::havok::Clock`; the rigid world and every
ragdoll step on it (ragdolls had their own 8-step accumulator).

One step, `hkpSimulation::stepDeltaTime` (`00cf8730`): integrate (`+0x10`)
→ collide (`+0x14`) → advance time (`+0x18`). `iSimType` 1 makes the world
`hkpContinuousSimulation` (`00c661d0` maps 1 → 2), one thread.
`hkpSimulation::integrateInternal` (`00cf8da0`): the dirty islands cleaned
up (`00cb55d0`: put to sleep `00cb5310` or woken `00cb5100`); actions
(`00cf8b00`: the grab's mouse spring); the deactivation flags
(`hkpSolverInfo::incrementDeactivationFlags`, inlined: counter `+0x307`
+1, flag bit 1 of `+0x305` flipped when (c − 4) & 7 = 0, bit 2 when
c & 7 = 0, `+0x306` flipped and the counter zeroed when c & 15 = 0); then
each active island, last to first: without constraints
`hkRigidMotionUtilApplyForcesAndStep` (`00d28a30`), with constraints
`hkpConstraintSolverSetup::solve` (`00d88b50`: `00d29830` forces into
accumulators, the solver `00d8d030`, `00d29bf0` back into the motions);
both return the island's fewest passing deactivation checks.

### World and solver settings (`physics::havok`, PR 1)

| What | Where | Value |
| --- | --- | --- |
| Gravity | cells' `InitHavok` `00554010`/`00552dc0` from `011ca158` (`00f4b550`) | (0, 0, −98.1) |
| Solver tau, damping, iterations, micro steps | Havok's cinfo defaults `00c90b80` | 0.6, 1.0, 4, 1 |
| Friction tau and ratios | `hkpSolverInfo::setTauAndDamping` `00c90e60` | tau ÷ 2; damping ÷ tau, tau ÷ damping, damping ÷ friction tau, friction tau ÷ damping; integrate velocity factor tau ÷ damping |
| Contact resting velocity | Bethesda's cinfo `00c681c0` | FLT_MAX |
| Collision tolerance, expected max linear velocity, expected step | `00c681c0`/`00c90b80` | 0.1, 200, `fMaxTime` |
| Activate on transform change | `00c681c0` (`+0xa7`) | off |
| Deactivation on, islands on | `00c90b80` (`+0xd0`, `+0xd2`) → world `+0xd5` | wanted |
| Per frame | `bhkWorld::ScaleSolverInfo` `00c66a00` | r = step ÷ `fMaxTime`: substeps max(2, trunc(4r)) (4 at normal speed), tau 0.6(1 − k) + 0.6kr, k = `fHavokTauRatio` (`011d1320` → `011afe60`, 0.5); damping ÷ tau and tau ÷ damping refreshed, friction tau and the integrate factor not |
| Deactivation classes | `hkpWorld::hkpWorld` `00c95a80` | g = 98.1; (v, a) = (1.19e-7, 0) for 0/1, (0.01, 0.08), (0.017, 0.2), (0.02, 0.3), (0.025, 0.4) for 2–5: linear threshold inverse 1/(vg), angular 1/(vg·g·0.1), slow multiplier 1 − ¼·0.016·(ag)/(vg), relative sleep velocity ¼·0.016 ÷ a (2.13e37 when a = 0); every class: distances² {d², (4d)²} and rotations² (hkHalf) {(2d)², (8d)²} with d = 0.02 |
| Speeds | hkUFloat8 table `010c7948` (256 floats, 0 … 1000002), encoder `00ca9360` (binary search on the float bits) | a motion's most linear (`+0xbc`) and angular (`+0xbd`) speed are indices; a model's 1068 and 31.57 are table entries |
| hkHalf | `00c95a80` (store: the float's upper 16 bits), `00d28a30` (load) | gravity factor, rotation thresholds |

### The integrator (`hkpMotion` steps, PR 3)

`hkRigidMotionUtilApplyForcesAndStep` (Xbox PDB, `00d28a30`), per motion by
type (`+0x08`: 1–3 dynamic, 4 keyframed, 5 fixed, 6 thin box, 7
character):

1. Fixed: skipped (and not counted for sleeping). Keyframed: no forces.
   Character: damping only. Others: v += gravity factor (hkHalf `+0x11e`)
   × gravity × dt (`hkpSolverInfo::m_globalAccelerationPerStep`, world
   `+0x200`).
2. Damping: v × max(0, 1 − dt × linear damping `+0xb4`); ω × max(0, 1 −
   angular damping `+0xb8` × dt).
3. Any velocity component ≥ 1e6 (or NaN): both velocities zero (`01268370`).
4. Swept transform: centre0 = centre1, time0 = the step's start; |v|
   held to the table's most linear speed; centre1 += dt × v.
5. rotation0 = rotation1; a = ω·dt/2; x = |a|² × 0.4052847 (4/π²); the cap
   c = min(most angular speed × dt, 0.9): when c² < x, ω and a × c/√x and
   x = c²; w = 1 − 0.822948x − 0.130529x² − 0.044408x³; rotation1 = (a, w)
   · rotation0 (`00c66320`), normalized (`005611c0`, `rsqrtss` + one Newton
   step; exact here); the transform rebuilt (`00cb2d90`).
6. The deactivation bookkeeping (below).

(Thin boxes get extra inertia handling and a second clamp: not translated,
none simulated.) Bodies in islands with constraints get the same damping
in `00d29830` (gravity factor kept in the accumulator, gravity added by the
solver per substep, `m_globalAccelerationPerSubStep` `+0x1f0`) and the same
reset, clamps, integration and bookkeeping in `00d29bf0`.

In nv-rs (`physics::havok::Motion`; positions in game units, the cap and
thresholds scaled by `HAVOK_UNIT`): a body with nothing near it (no
triangle, body or walker within its reach plus a step's travel) is stepped
exactly by the translation; one with something near gets the damping once,
gravity per substep inside this solver's XPBD substeps (which also move it:
PR 7 replaces them), then the reset, the linear clamp and the turn cap on
ω. Ragdolls likewise until PR 8 (damping and caps were per substep). The gravity
factor is 1 for every body (nothing traced sets another).

### Sleeping (deactivation, PR 4)

Per motion, at the end of each step (`00d28a30`/`00d29bf0`): the counter
(`+0x09`) goes up; when it's a multiple of 4 there's a check, of slot 1
when it's a multiple of 16 (counter back to 0; 0xff stays "never"), else
slot 0. The slot's reference energy (`+0xfc + 16j`) keeps the largest
|v|² + min(radius `+0xb0`, 1)² |ω|². Pass: the centre within the class's
distance² of the reference position (`+0xf0 + 16j`) and the rotation
within its rotation² of the reference rotation (`+0x110 + 4j`, four bytes
(b − 128) × 0.00859375): the count (`+0x0a + 2j`, bits 0–6) + 1, held at
64; the old count kept in bits 7–13, the solver's select flag in 14–15.
Fail: count 0, references set to where the body is now (rotation
compressed by `00d975d0`: c × 116.36363 + 196736.5, bits 6–13 of the
float). Returned: the larger of the two counts; the island's is the
fewest of its bodies'.

An island with more than 5 is marked inactive (`00cf8da0` →
`markIslandInactive` `00cb5420`, the world's dirty list); at the next
step's start (`00cb55d0` → `00cb5310`) it goes to sleep when every body
passes `00d28560` (a quarter of its energy now − 0.01 not above the
counting slot's reference energy), with both velocities set to zero
(motion vtable `+0x40`/`+0x44` with `01267e30`); else it stays active.
Waking an island (`00cb5100`) starts its bodies' counts again.

So a body at rest from a new motion (references zero, taken as the
constructor's: not traced) passes at steps 8, 12, 20, 24, 28 and 36 (the
first check, at 4, sets the references; 16 and 32 are slot 1's) and sleeps
at the start of step 37 (0.59 s); a body creeping 0.016 Havok units
between checks never sleeps.

In nv-rs: sleeping is per simulation island (PR 5, below). Waking an awake
body (an impulse, the wind's force, the spring) cancels its island's
pending sleep. Each ragdoll is one island.
People pushing a body wake it through the proxy's impulse (`00c9c1d0` in
`applySurfaceInteractions`). The invented rules are gone: "1 s under 2 units/s and
0.3 rad/s" (clutter) and "1 s under 4 units/s and 0.6 rad/s" (ragdolls).

### Simulation islands (PR 5, `claude/b1-p5-islands`)

`hkpSimulationIsland` (Xbox PDB, 0x6c bytes): the bodies Havok steps,
sleeps and wakes together (`physics::islands`; `RigidWorld::islands`).
`iSimType` 1 makes the world continuous, so its `m_minDesiredIslandSize`
(`+0xb4`) stays 0 (`00c95a80` sets it only for the multithreaded world):
no "sparse" islands, an island is exactly a connected group.

| What | Where | Rule |
| --- | --- | --- |
| Adding | `hkpWorld::addEntity` `00c914d0`, `addEntityBatch` `00c94bd0` | a body gets an island of its own, active or not by the add's activation (Bethesda's batch add: not); a fixed body joins the fixed island (none here) |
| Boxes | `hkpEntityAabbUtil::entityBatchRecalcAabb` `00d1a330`, after each island's integration | the shape's box with half the collision tolerance (0.05 Havok units, collision input `+0x8` × 0.5); grown on each side by the step's turn × the object radius (motion `+0xac` × `+0xb0`) but not past the bounding sphere (centre ± (radius + 0.05)), then by the centre's travel back to the step's start (`+0x50` − `+0x60`): a swept box |
| Pairs | broadphase `00cf9c60`/`00d0d3f0` (3-axis sweep), `00cf7080` (the collision filter), `hkpEntityEntityBroadPhaseListener::addCollisionPair` `00d10800` / `removeCollisionPair` `00d10860` (Xbox vtable names) | overlapping boxes the filter lets meet get an agent when the quality table (`dispatcher +0x1bb0`) has a non-zero entry for the two bodies' qualities (fixed against fixed has none) |
| Merging | `hkpWorldAgentUtil::addAgent` `00cc0f40` → `hkpWorldOperationUtil::mergeIslands` `00cb5570` → `internalMergeTwoIslands` `00cb4c60` | unless either body is fixed (motion type 5) or they share one: the island with more bodies is kept (a tie: the one stored first); when either is active, the inactive one is activated first (`00cb5100`) — **a moving body wakes a sleeping one when their boxes meet**, before they touch; active marks and split requests are or-ed (the marks as they were before that activation); the kept island goes on the dirty list when either was |
| Splitting | `removeAgent` `00cc10b0`; `hkpDefaultWorldMaintenanceMgr::performMaintenance` `00d0b280` → `splitSimulationIslands` `00cb6f30` → `00cb6e70` → `00cb6060`, connectivity `00d074c0` | a pair whose boxes part loses its agent; when both bodies share an island it asks for a split check (`+0x25` bits 0–1). At the next step's start (`hkpSimulation::integrate` `00cf9340`, before the dirty islands are cleaned up) each such island is split into its connected parts: bodies joined by an agent with a non-fixed partner (entity `+0x60`), a constraint between two non-fixed bodies (`+0xac`) or an action's bodies. New islands keep the old one's state, and are marked inactive when it was active but marked inactive |
| Sleeping | `00cf8da0` → `markIslandInactive` `00cb5420`; `cleanupDirtyIslands` `00cb55d0` → `00cb5310` / `00cb5100` | PR 4's rule, now per island: more than 5 passing checks for its fewest, still marked active: marked inactive and put on the dirty list; cleaned up at the next step's start: asleep when every body passes `00d28560` (velocities zeroed), else marked active again; an island marked active is woken |
| Waking | `hkpEntity::activate` (Xbox PDB) → `markIslandActive` (Xbox PDB) | an impulse, force or the spring on a sleeping body wakes its whole island (here at once; Havok at the next step's cleanup, before anything moves) |

The island flags' bit positions on the PC are the PDB's mirrored
(`m_splitCheckRequested` `bool:2@6` is `+0x25 & 3`, `m_activeMark`
`@4` is `+0x26 & 0xc`), read from `00cb6f30` and `00cb55d0`.

So a place's clutter that touches (or nearly: within 0.1 Havok units, 0.7
game units, box to box) loads as islands asleep together, and knocking one
wakes the lot (a stack, a shelf's worth); bodies that part sleep apart.

In nv-rs: the broadphase is every pair of bodies' boxes (no 3-axis sweep;
the same pairs); the body's step-start pose is its centre and rotation
before this solver's substeps. The world's statics (the collider) are
Havok's fixed bodies: they pair with every body and join no island. The
split's choice of which part keeps the old island isn't followed (it
changes nothing but storage). Islands are stepped together as before
(this solver's contacts until PR 7).

### Contact points (PR 6, `claude/b1-p6-contacts`)

Each agent (a broadphase pair) has a contact manager
(`hkpSimpleConstraintContactMgr`, vtable `010cc984`, one per pair of
collidables: a body against one fixed body gets one, whatever triangles it
touches) holding its points in a `hkpSimpleContactConstraintAtom` (0x30
bytes + 0x20 a point + the properties): `physics::manifold`,
`RigidWorld::manifolds`.

| What | Where | Rule |
| --- | --- | --- |
| When | `hkpSimulation::collide` `00cf8bc0` (continuous `00d0ef40`) | after a step's integration, for the agents of active islands; a sleeping island's points stay as they were; a body added asleep has none until its island is first stepped (`addEntityBatch` `00c94bd0` makes agents, no points) |
| Tolerance | `hkpCollisionDispatcher::initCollisionQualityInfo` `00cfb570` | points are made and kept up to the collision tolerance apart (0.1 Havok units, 0.7 game units) |
| Adding | `addContactPointImpl` `00cfcf80` → `00d92df0` | the first point adds the contact constraint to the island; a point's impulse and solver data start at 0, its flags at 1 (new: the solver's callbacks finish it); when the point before it isn't paired and has no most impulse, it is paired with it (flags 3: the two are solved as one 2 × 2 block); the info's flag 4 (work the contact radius out again) is set |
| Properties | `setContactPointProperties` `00cfd800` | friction √(f₁f₂) as hkUFloat8 (`00ca9360`: the table's next entry up, so 1.118 → 1.14), restitution √(r₁r₂) × 128 rounded to a byte, most impulse 0 (none) |
| Event | `00cfcba0`, `hkpWorldCallbackUtil::fireContactPointAdded` `00d01850` → the world's contact listeners (`+0x1a0`, slot `+0x10`) | once per new point, with its projected velocity (`+0x1c`): body A's velocity at the point less B's (`00ca0c40`: v + ω × (p − centre)), along the normal. The game's `FOCollisionListener::contactPointAddedCallback` (`00623cb0`) plays the impact sounds and works out physics damage from it |
| Removing | `removeContactPointImpl` `00cfd320` → `00cfd200` | the later points move down, the one now in its place loses its pairing, the info's flags 1 and 4 are set; the removed callbacks are empty in the game; the last point gone removes the constraint |

So a body resting on its points hears nothing more, and a body that lands
is heard once per corner that comes into contact (each with its own
speed), where before a pair beginning to touch was heard each time it
touched again after a bounce or a jitter (the repeated impact sounds).
(`m_allowToSkipConfirmedCallbacks`, collision input `+0x6d`, is 0 in the
game's world: `00c90b80` leaves cinfo `+0x75` 0 and Bethesda's `00c681c0`
doesn't set it.)

In nv-rs the points come from this solver's generator (Havok's agents —
GSK, box-box, MOPP — aren't translated): a body's corners, sphere and
capsule centres and a capsule's axis against triangles and other bodies'
shapes, a triangle's edges into a hull; each point has a feature key (which
corner, edge end, triangle) that keeps its identity from step to step, as
Havok's agents keep their points. The position is on the other side's
surface (taken: the agents aren't traced). A fixed body is a collider
triangle's placed reference (0: the landscape and unowned statics).
The proxy's pushes aren't Havok contacts and aren't heard. The contacts still
move bodies by this solver's substeps until PR 7, which solves these
points.

### The contact solver (PR 7, `claude/b1-p7-solver`)

An active island with contact points (its contact constraints) is stepped
by `hkpConstraintSolverSetup::solve` (`00d88b50`), the rest body by body
(PR 3). `physics::solver`, in Havok units about the island's first body;
`RigidWorld::solve_island`. This replaces this solver's XPBD substeps,
overlap corrections and bounce/friction passes (deleted).

| Stage | Where | Rule |
| --- | --- | --- |
| Accumulators | `hkRigidMotionUtilApplyForcesAndBuildAccumulators` `00d29830` | the world's fixed body first (keyframed, zero); per body (box motion types): the motion's velocities damped in place, then linear velocity (world), angular velocity in the body's axes (the rotation transposed, `00cd4ba0`), inverse inertia diagonal and inverse mass (`+0xc0`), centre, gravity factor, deactivation class |
| New points | `hkSimpleContactConstraintData_fireCallbacks` `00d92900` | for each point flagged new: projected velocity v (`00df2510`, damped motion velocities, accumulator centres); r = restitution byte ÷ 128; when v ≥ −`contactRestingVelocity` (always: Bethesda's FLT_MAX, `00c681c0`) or r ≤ 0.3: impulse guess (r + 1) ÷ (inverse masses + 1e-10) × −0.2 × v, solver data r v × substep × −1.3, allowed penetration that + distance when r > 0, else 0. The other path (`hkpSimpleCollisionResponse::solveSingleContact` `00d9ee50`, an immediate bounce) is never taken in the game: **that is what Bethesda's FLT_MAX resting velocity does** |
| Jacobians | `hkSimpleContactConstraintDataBuildJacobian` `00d72190` | per point a 1-D Jacobian along the normal; right-hand side: with allowed penetration dA and solver data s: step = min(frame dt × 1 (`011b6924`), −0.05 (`011b6928`) × dA), gap = (d − dA) − step; when 2·step + frame dt < −s − (d − dA) (a penetration the solver didn't predict) it is added to the allowance; the allowance (≤ −ε) kept; rhs = −gap × tau ÷ damping ÷ substep. Paired points (flag 2) one 2 × 2 block (coupling × 0.99), inverted with the virtual mass factor (damping). Friction when Σfriction ÷ n × Σ(last impulses) > 0: the mean normal (or the first), an axis from x/y/z by the info's index (re-picked as the normal's least component when within √0.1, clearing the drifts), two 1-D Jacobians at the points' mean, their 2 × 2 inverse, rhs = the drifts the export left × friction tau ÷ damping ÷ substep, at most that total per micro step; with 2+ points the turning friction about the normal at the mean radius (worked out again when info flag 4 is set) |
| Solver | `hkSolveConstraints` `00d8d030` (schema types 0x10, 0x14, 0x16, 0x17) | gravity × factor once into the velocities, sums zeroed; per substep (4 at normal speed), per micro step (1): each schema: contacts keep their total impulse ≥ 0 (a pair: both, else the second alone, else the first alone); friction impulses scaled down together to the most; then per body (`hkSolveIntegrateVelocitiesByTheSteps`, inlined): components ≥ 1e6 → the unit x (`01268370`); per component \|ω\|·angular inverse + \|v\|·linear inverse ≤ 1 → × slow multiplier, or all ≤ the relative sleep velocity → zero (**the deactivation velocity thresholds are read here**, open question 5); sum += 0.6 × (v − sum); last substep: sum × ¼ ÷ 0.6 is the velocity the body moves by, v − old sum its velocity; otherwise v = (v − old sum) + new sum + gravity |
| Export | `hkSolverExport` `00def570` | each point's impulse; its solver data rhs × damping ÷ tau × substep − J·sum × step (the distance the solver expects, negated); friction impulses and drifts (× the last scale; friction tau) into the atom's info (`+0x18`…`+0x2c`) |
| Apply | `hkRigidMotionUtilApplyAccumulators` `00d29bf0` | the motion's velocities the solver's (angular back to the world); moved by the sums (reset to unit x when invalid; held to the most linear speed; turn capped as PR 3), then \|v\| and \|ω\| held to the most speeds; deactivation bookkeeping |

Corrected in passing: the invalid-velocity reset (`00d28a30`, PR 3) is to
the vector at `01268370`, which the static initializer `00fbdde0` sets to
(1, 0, 0, 0), not zero. Box motions keep only their inertia tensor's
diagonal (`hkpBoxMotion::setInertiaLocal` `00d1e750`); impulses and the
solver now use it.

What it does: a box rests without drifting, a ball rolls down a slope with
its spin matching its speed, a sliding box stops after about v² ÷ 2μg, a
bottle stands on the rail and falls off when shot. Restitution in the PSI
step is a one-step offset of a new point's distance: a 0.4 ball dropped
from 200 units leaves the floor at about 2% of its landing speed (the
integrated-velocity scheme spends the target on moving it out).

In nv-rs: the island's constraints are its manifolds in key order (the
game's order is its entities' constraint lists); people pushing a body are
the character proxy's impulses (`applySurfaceInteractions`, "The character
proxy"), not contacts. Not translated: maximum-impulse contacts (0x12; the
game's points have none), constraint priorities >= 4
(full TOI, "Continuous collision" below), the contact impulse limit callbacks (`00d01700`), thin box
and sphere motion specifics (all bodies use the box motion's rules).

Verified live (release viewer, installed data, the PR 4 bottle route,
`--wait 35`; nothing compared with the game): the six other VCG02 bottles
stand on the rail for the whole run (screenshot); the shot one
(`Hit 0010A208 at 149 units`, impulse 15) is heard twice as its points
come (25 and 107 units/s, no repeats) and comes to rest at 10.3 s, 127.5
units off, and stays. The tumbleweeds now roll in the wind (00178A80 1441
units in the first 5 s; before, 0.5–1 unit per 5 s) and are heard as they
bounce. Two Goodsprings crates woken by them come to rest at 3.7 s.
Steady frame time 17.1 ms (vsync), main-thread work 9.5–10 ms (PR 4's run:
11.5 ms).

### Ragdoll constraints (PR 8, `claude/b1-ragdoll`)

Ragdolls (and any `bhkRagdollConstraint`, `bhkLimitedHingeConstraint`,
`bhkBallAndSocketConstraint` data) are solved by the same island solve as
the contacts: `physics::constraint` builds the joints' atoms into the
solver's schemas, `physics::ragdoll` builds the capsules' contacts and
steps through `solver::solve`, once a world step in the world's solver
substeps. Private notes: `%USERPROFILE%\nv-re\work\b1-ragdoll` (decompiles in
`dec\`).

The data objects are the NIF's own: `bhkRagdollConstraint` (`00cc67a0`) holds
a `hkpRagdollConstraintData` (0x140 bytes) built by `00cdf2f0` (defaults) →
`00cdf460` (atoms), the loader copying the file's axes, pivots and angles
into it; the clone `00cc6830` shows which fields come from the file (twist
least/most `+0x104`/`+0x108`, cone widest `+0x11c`, planes least/most
`+0x12c`/`+0x130`, `00cde9f0` the most friction torque `+0xf8`). Atoms
(Xbox PDB `hkpConstraintAtom::AtomType` order, which the builder's switch
follows: 2 set local transforms, 5 ball socket, 0xc 2-D angular, 0xe angular
limit, 0xf twist limit, 0x10 cone limit, 0x11 angular friction, 0x12 angular
motor, 0x13 ragdoll motors):

| Data | Atoms, in memory order | Constants (constructors) |
| --- | --- | --- |
| ragdoll `00cdf460` | transforms, ragdoll motors, angular friction, twist, cone, planes, ball socket | friction: axes 0–2 (`+0xf3` first 0, `+0xf4` count 3); twist: twist axis 0, reference axis 1, tau factor 0.8, defaults ±30° replaced by the file; cone: axis 0 against 0, mode 0 (zero when aligned), least −100, tau 0.8, angle offset at runtime +56; planes: axis 0 against 1, mode 1 (zero when perpendicular), tau 0.8; ball socket: stabilization 1.0 (hkUFloat8), most impulse `HK_REAL_MAX` (`0x7f7fffee`: schema 5, no cap) |
| limited hinge `00cb9170` | transforms, angular motor, angular friction, angular limit, 2-D angular, ball socket | friction axis 0, count 1; limit axis 0, ±π replaced by the file, tau 1.0; 2-D free axis 0; ball socket as above |
| ball and socket `00d54290` | translations, ball socket | stabilization 1.0 |

| Builder (`00d6f460`, one atom at a time) | Rule |
| --- | --- |
| Frames | transforms atom: each body's three axes (the file's twist, plane, motor axes; axle, perpendicular, second perpendicular) and pivot turned by the body's transform |
| Ball socket `00d6ced0` | three rows along the world axes; the right-hand side the pivots' gap + L ((ω_A × r_A) − (ω_B × r_B)), L = stabilization ÷ rhs factor, × the rhs factor; so stabilization 1 leaves the bodies' turning out of the row. Inverse diagonal = virtual mass factor ÷ (effective mass + ε). Schema 5 |
| 2-D angular `00dcee80` | rows about B's axes (i+2)%3 and −(i+1)%3 with rhs −(A's axis i · B's other axis) × rhs factor; schema 0xc |
| Angular limit (0xe) | angle = atan2(B₂·A₁, −(B₀×B₂)·A₁) by the game's polynomial `00cbff80`, kept within π of the last solve's (solver data); row about A's axis; schema 0xd |
| Twist (0xf) | axis = the normalized sum of the two twist axes; angle of A's reference axis against B's about it; tau = ½\|A+B\|² × tau factor × tau |
| Cone / planes (0x10) | angle between A's axis and B's reference axis (or π/2 − that), about the cross product × the mode's sign; skipped when parallel; the cone's stabilization: an allowed angle offset kept in the runtime moves the angle as a contact's allowed penetration moves its distance |
| Limit row | inverse diagonal = tau ÷ (effective mass + ε); rhs −angle ÷ substep; bounds −min and −max ÷ substep × tau; schema 0xd |
| Angular friction (0x11) | per axis one row, rhs = last solve's drift × 1 ÷ substep, at most torque × micro step; schema 0xe; skipped at torque 0 |
| Solver | `00d8d030` cases 5, 0xc, 0xd, 0xe: unbounded rows; the limit sees damping × J(ω − Σ) + tau × (damping ÷ tau) × JΣ (Σ the integrated sums), takes the lower bound's impulse while it stays positive, else the upper's; friction scales the micro step's impulse to its most |
| Export `00def570` | each angular row keeps rhs × substep − JΣ × step (friction × its scale) as solver data for the next step |

In nv-rs: `physics::ragdoll` is one island of dynamic bodies (mass 0 bodies
are keyframed, as the fixed ones of a tree) in the solve with the capsules'
contacts. Joints come first, then the world's contacts, then the pairs'. The
step is the clock's (0.016 s), solver settings `SolverInfo::new` (4 substeps,
1 micro step, tau 0.6, damping 1.0). The old XPBD substeps, positional
joint/contact corrections and friction pass are gone. The bodies' damping,
the speed caps, the turn cap and the deactivation counts are `Motion`'s
(`apply_accumulator`), as for every rigid body.

Taken, not traced: the atoms' runtime data starts at 0 (the data's `Runtime`
constructor isn't traced), the ragdoll motors and the angular motor are
left off (their motor pointers are null in the game's death path; no motor
data is read), the file's malleable strength (tau 0.9) isn't applied, the
actor-scale clone `00cbd9e0` (friction torque scaling) isn't followed, the
violated-constraint list (`m_violatedConstraints`) isn't kept. The contact
points are this generator's (a capsule's two end spheres and the closest
point of its axis against each triangle, ends against the other axis for the
pairs; kept within the collision tolerance plus the distance the body can
close in the step, which stands where continuous collision, PR 9, will
stop a fast body); one contact manifold a body for the whole world, one a
pair. A ragdoll's contacts follow `physics::rigid`'s property rules (friction
√(f₁f₂), restitution √(r₁r₂) × 128); triangles' surface or
`DEFAULT_SURFACE`. A new point's penetration is *allowed* by the solver's
rules (`00d72190`), so a body already deep in another stays so: bodies
start apart.

Unit tests (`physics::ragdoll`): a chain of ball sockets hanging from a
fixed rod keeps every joint gap under 2 units while it whips down; a hinge
keeps its axle aligned and its angle within the limit and hangs; a 30° cone
limit holds (widest angle within 0.12 rad of the limit) with the pivot
together; a three-rod chain dropped on a floor lies still, goes to sleep and
its joints stay together; a rod falls and rests on the floor; own capsules
meet by the part table. `physics::constraint`: the polynomial atan2, the
atoms' result slots, the ball socket's right-hand side.

Verified live (release viewer, installed data, nothing compared with the
game; `Fallout.ini` defaults): `WastelandNV --at -67845,3000,8400,0,0
--freeze-ai --walk --answer-boxes --box-answers 1 --run-at 3
"EasyPeteRef.Kill"` (`--wait 25`; a second view from
`--at -67845,3260,8480,0,45`): "00104C80 goes limp at (-67845, 3334, 8392)
moving (19.9, 1.4, 0.0) units/s" at 4.1 s and "00104C80 comes to rest, its
first body at (-67840.3, 3345.0, 8396.5)" at 7.6 s, 3.5 s after he fell
(before, ragdolls never came to rest: "jitter"). Easy Pete lies on his back
on a porch's boards (Goodsprings), arms by his sides, legs bent at the
knees, every limb joined and the hat by his head (`b1-ragdoll\after3.png`;
`after.png` is the first view, a tumbleweed in front of him). Identical
runs (the same rest time and place in the three).

### Data layouts used (Xbox PDB, matched to the PC code)

`hkpMotion` (0x120, entity `+0xe0`): `+0x08` type, `+0x09` deactivation
counter, `+0x0a` inactive counts [2], `+0x10` transform, `+0x50`/`+0x60`
centre of mass 0/1, `+0x70`/`+0x80` rotation 0/1, `+0x90` local centre of
mass, `+0xa0` delta angle, `+0xb0` object radius, `+0xb4`/`+0xb8`
damping, `+0xbc`/`+0xbd` most speeds, `+0xbe` deactivation class, `+0xc0`
inverse inertia and mass, `+0xd0`/`+0xe0` velocities, `+0xf0` reference
positions [2], `+0x110` reference rotations [2], `+0x11e` gravity factor.
`hkpSolverInfo` (world `+0x1e0`): `+0x04` tau, `+0x08` damping, `+0x0c`
friction tau, `+0x10`/`+0x20` gravity per substep/step, `+0x50`… ratios,
`+0x60` contact resting velocity, `+0x64` deactivation info [6] (0x1c),
`+0x10c` substep dt, `+0x114` substeps, `+0x120` 1/substeps, `+0x125`
select flags [2], `+0x127` integrate counter.

### Verified live (release viewer, installed data; nothing compared with the game)

Route: `WastelandNV --at -68232.9,4900,8440,0,10 --walk --weapon
WeapNV9mmPistol --run "StartQuest VCG02" --run "VCG02BottleMarkerREF.Enable"
--answer-boxes --key-at 3 r:1.0 --key-at 6 mouse-left --key-at 8 mouse-left
--key-at 10 mouse-left --fps --wait 35` (input by `--key-at` only), the same
on a build of `main`'s physics (`1949bd2`) for comparison.

- The VCG02 bottles stand on the rail and stay there (screenshot after 35 s);
  the shot one (`Hit 0010A208 at 148 units`) lands and "comes to rest" at
  10.1 s (main's code: 10.5 s), and no contact or rest line follows in the
  25 s after: it stays still. The other six never move.
- Tumbleweeds (wind 49) creep 0.5–1 unit per 5 s; the same on main's code
  (a probe of this solver shows why: a sphere's contacts act at its centre,
  so friction can't roll it; PR 7).
- Frame time, same route, steady part (16 two-second samples each; the
  machine was shared, so treat as rough): main's physics 19.5 ms a frame,
  main-thread work 15.6 ms; this branch 17.3 ms, 11.5 ms.

Not checked live: Doc's house clutter (it loads asleep and nothing woke it
in either build), a dead NPC coming to rest.

### Tested (generated)

`havok::tests`: the speed table and encoder, hkHalf, the constructor's
settings, `ScaleSolverInfo` at 1, ½ and ¼ speed, the flags over 16 steps,
frame times 5–100 ms and 60 Hz carry, half speed, one step only; free
fall and damping step by step against hand-computed values, speed and spin
caps (in Havok and game units), invalid velocities; a still body passing
its sixth check at step 36 and 32 steps after waking, creeping and slowly
turning bodies never passing, the last test, rotation compression.
`rigid::tests`: a body in the open stepped exactly by the integrator; a
box resting on the floor asleep at step 37; a stack asleep together and a
nudge waking both; the earlier tests (falls and rests, rail bottles, shot
off a rail, stacking, walkers, grab) unchanged.
PR 5: `islands::tests` (bodies alone and merged on a pair, fixed bodies
join nothing, an active island waking the one it merges with, a removed
agent splitting at the next check, sleep and wake through the dirty list,
a wake cancelling a pending sleep, removal renumbering); `rigid::tests`:
boxes put 0.2 apart share an island and a push wakes both, not a far one;
a sliding box wakes a sleeper when their boxes meet, before the faces
touch; two boxes shoved apart split and the one left sleeps alone; layers
the filter keeps apart get no pair.
PR 6: `manifold::tests` (points paired as they come, unpaired when one
goes; properties as the manager combines them); `rigid::tests`: each new
point of a landing box heard once and none while it rests on the same
points; a point's friction, restitution, pairing, normal and distance.

## Tested

`nif` scene `reads_collision_shapes_in_game_units`; `physics`
`layers::tests` (3), `grab::tests` (2), `contacts::tests` (3),
`impulses::tests` (3), `rigid::tests` (clock, fall and rest, shot off a
rail, stacking and the proxy's push, the proxy's impulse values, the player walking into
a body whose triangles are in the collider, deltas, impulse units,
surfaces through `extend`, material combination, landing contact event,
grab spring carry/release/removal, rail end, bottle at the origin and at
Goodsprings' coordinates), `lib` `shots_pass_a_transparent_fence_that_stops_walkers`;
`preview` collision `moving_clutter_is_a_body_and_left_out_of_the_collider`;
`world` `save::tests::havok_moved_objects_keep_their_pose_through_a_save`
(with velocities); viewer `clutter::tests`, `controls::tests`.

## Verified live

See the 2026-10-06 runs below (release viewer, installed data, input
posted to the viewer's own window, F12 reports and logs in
`physics2-2026-10-06\<run>`).

Command: `nv-viewer <Data> WastelandNV --at <x,y,z,heading,pitch> --walk
--weapon <W> --run "StartQuest VCG02" --run "VCG02BottleMarkerREF.Enable"`;
driver `drive2.ps1` (`PostMessage` of keys and clicks to the viewer's
window only).

- **Moved bodies are hit where they are** (a1, at −68232.9, 4900,
  pitch 8.2, varmint rifle): the first shot, `Hit 0010A208 at 149 units`,
  push 9.0, `PHYBottleH` as it lands at 331 units/s, rest at (−68220.2,
  5078.8, 8386.7); the second shot at the same spot: "hit nothing but a
  wall" (before, the bottle's placed bounds took the hit).
- **No prompt on the destroyed bottles** (a2, F12 report 001 with the
  crosshair on 0010A208): no Info panel at all. The old "E) Take" came
  from the swinging-door pick taking any owner of collider triangles
  (clutter bodies too) for a door; it now takes doors only.
- **Z grab** (a2, at −68232.9, 4990, pitch 20): "Grabbed 0010A208
  (VCG02Bottle) 63 units away"; walking back one second (S held) carries
  it off the rail and over the fence in front of the player (report 003);
  Z again: "Let go", it drops and bounces (`PHYBottleH` 293 and 103,
  `PHYBottleL` 57 units/s) and rests at (−68242.9, 4856.6, 8360.1).
- **Explosions** (a6, a8, `WeapNVDynamite` thrown at 1200 units/s, 2.5 s
  fuse, in front of the fence): all seven bottles fly (contacts up to
  1985 units/s) and come to rest around the square.
- **Saved velocities** (a6, a8): F5 0.3 s after the blast saves
  `havokvel` for the 13 bodies still moving; F9 sets them going again and
  they come to rest near where they did before the load.
- **Walking into clutter** (a7, at −66018.6, 4650 heading north, W held
  3 s): tumbleweed 00178A80 (mass 2.5) is pushed ahead from y 4770.9 to
  5529.6.
- **Contact sounds**: every landing logs its two sounds by material
  (`PHYBottleH`/`L`, `CStoneMedium`, `CWoodLarge`, `PHYBabyRattle`…) with
  the game's attenuation and frequency (not applied to playback).
- **The wire mesh under the rail** (a3, pitch 48 through it): the shot
  goes through, but the unfiltered cast does too: the mesh has no
  collision triangles here, so the earlier "the fence stops shots from
  behind" was its rail or posts, which projectiles meet in the game too
  (layer 6 touches static layers). Which layer the rail is on wasn't
  established live.
- **Joined bodies**: the loaded squares list "2 placed objects with joined
  or several moving bodies, kept solid": `MaizeWitheredGroup2b` ×2; no
  constrained bodies (hanging signs, chains) load around the square.

Not seen live: people pushing clutter, physics damage (nothing
destructible was struck hard enough), the grab's contact release.

Batch 3 runs (`physics3-2026-10-06`, release viewer, installed data):

- **Doc Mitchell's house** (`doc2` before, `doc3`/`doc4` after): before,
  every body was woken at load and some fell (the prewar hat 114 units
  off the coat rack, the office fan 78, pork 'n' beans 68, books 10–17)
  and many kept waking; after, no contact and no "comes to rest" line in
  10 s, and the hat hangs on the rack (`doc4.png`).
- **Rail bottles** (`rail2.png`, VCG02 started, marker enabled): all seven
  stand on the rail after 10 s; none moves (0010A204 no longer tips).
- **Burnt pickets** (`pickets1.png`): `NVFencePickBurntBroken01` is a
  `STAT`; its square now has 1 moving body instead of the pickets too,
  which lie where placed.
- **Tumbleweeds** (`tumble22.log`, weather `NVWastelandGS`, wind 49): the
  twelve tumbleweeds roll north-west (heading 1 rad: 00178A80 from
  (−66018.6, 4770.9) to (−68063.3, 6109.1), 2444 units, direction
  (−0.84, 0.55)) and stop against what's in their way; one blew onto
  Easy Pete's porch (`petedead2.png`).
- **Easy Pete's hat** (`pete1.png` hung from `Bip01 Head` in its own axes:
  sideways; `pete2.png` as a FaceGen part: on his head); killed
  (`petedead2.png`) he falls on his back with the hat on.
- **Cheyenne** (`dog3.png`, `dog4.png`, killed from the console in the
  saloon): her body stays whole (before, her 3 torso bodies were dropped
  and 8 of 20 joints with them, so legs, head and tail fell apart from
  the body: the stretching). She stays standing in her death pose for
  25 s: nothing tips a four-legged ragdoll with its knees at their
  limits (see gaps).
- **Acceptance** (`scripts\acceptance.ps1 -Routes doc,vms16`): doc
  passed; vms16 failed once (Trudy killed by a ganger, no "XP +50") and
  passed on the second run.

## Death → ragdoll (`claude/m2-death-ragdoll`, 2026-10-06)

Private exports: `%USERPROFILE%\nv-re\work\death-2026-10-06` (decompiles
in `dec\`, disassembly in `dis\`, runs). Traced in FalloutNV.exe
1.4.0.525; names from the Xbox PDB (PC actor vtable slots from +0x22c on
are the Xbox ones + 4; the process's and `TESForm`'s match).

| What | Where | Rule |
| --- | --- | --- |
| Who goes limp at once | `Actor::Kill` (Xbox PDB) `0089d900`, from `KillActor` `005be2a0` and the damage paths | with a high process (`0045cd60` = 0) and a 3D; otherwise actor +0x118 is set and the death waits for the 3D (`00888070`) |
| No death animation for ragdolls | `00888070` (`UpdateAnimation`) | the `Death` group (0xe0) plays only on that delayed path, and only for a `CREA` whose `HasRagDoll` (vtable +0x38c; creature +0x1b4 from `008d4ab0` at 3D load: the root or its first child, 4 deep, has a `bhkNiCollisionObject`) is false; characters always have one. Cheyenne's `creatures\dog` has no death `.kf` anyway |
| Bodies dynamic | `0089d900` at `0089efee`: `00c6a350(root, 1, 1, 1, 1)` → visitor `00c66280` (`bhkBlendCollisionObject::SetMotionType` `00c823b0`) | knock state (process +0x40c) 0 or 6 and actor +0x1b0 clear (set by `CreateRagdollInstance` `0087e130` when there is no ragdoll): every body of the tree dynamic; biped-layer blend gains (hierarchy/velocity) go to 0 for dynamic bodies (table `011b06d8`, all 1.0), so nothing pulls them back to the animation |
| The nudge | `0089ee31`…`0089f05c`; `00630b40`, `004a0c10`, `004a0c90`/`004b4500` (turning `011a9478` = (0, 1, 0)), `00439180`, `0062b8d0` → `0062b930` | the controller's velocity is read before it's removed (`RemoveCharController`, +0x2a0); direction: that velocity made a unit vector (nothing under 1e-6), or standing still the heading's forward (sin h, cos h, 0); killed by the player (not the player): victim − player controller positions, a unit vector. × `fDeathForceForceMin` (`011cf724`; FalloutNV.esm 20, exe 35), game units/s turned into Havok's (`004a3e90`) and **added** to every dynamic body's velocity (mass-independent). Then `AddChange(4)` (Havok moved) |
| Paralysed instead | `0089f073` | knock state other than 0/6 and paralysed (+0x234): (0, 5, 0) turned by the heading added instead (not done: nv-rs has no paralysis) |
| Killing hit's push after | `0089a760` → `008ae000` | only after a hit that killed with an attacker (existing `death_push`); `KillActor` deals no hit, and with no actor named passes no attacker |
| Bodies alive | `bhkBlendCollisionObject` (vtable `010c53dc`) update `00c81e30`; `00c823b0` sets its blend (+0x18) and velocity gain (+0x1c) | keyframed (blend 1, motion type 4): `00c65b00` either places the body (vtable +0xe8) or hard-keyframes it, `00c8e160`: velocity = (animation's next pose − now) ÷ step, turned into a teleport above the most speed. Which branch the living biped takes (`0047b470`, flag 0x20) isn't traced. Dynamic with gains 0 (the dead): `00c65a80`, the node follows the body only |
| Not the AI | `0089d900` from `Kill`/damage | the death routine isn't the AI's update: under `tai` or in the dialogue menu the dead go limp too (the world, Havok included, waits for the menu) |
| Dying → dead | `FinishDying` (`HighProcess`, process +0x354) `008f7350`, from the actor updates `00882b90`/`00883240`/`00883800` while `IsDying` (+0x2e8, life state 1) | still dying while any ragdoll body is active (`00c6a440` → `00c67260`) and the process ran within `TimeScale` × 0.0005 game hours (1.8 s at 30) (`fHourLastProcessed`, process +0x20); then `008b01c0` (life state 2), `DetachHavok` (+0x1c0, `00930870`: bodies taken out of the world, `00c69ee0` → `bhkWorldObject::Remove`), `SetHavokDeath(1)`. `fDyingTimer` (exe 2, not in the ESM) goes to process +0x330 (`SetGreetingTimer` slot); its use isn't traced |

Implemented: `world::combat::DeathStart` and `GameState::deaths` (how
each death began: killer, hit; `KillActor` none/no hit; fall damage and
effects no hit), `world::combat::death_nudge` (translated), `physics::
ragdoll::Ragdoll::add_velocity`; viewer `ai::fall` takes the death,
gives every body the nudge, then the hit's push (only for a killing
hit), and leaves the dead at load lying (no more pushes from a saved
killer); `ActorRig::go_limp` no longer poses a `Death` group's first
frame. The dead go limp with the AI held still (`--freeze-ai`, the
dialogue menu): before, `ai::move_actors` skipped them and they stood
in their last pose (`ai::dies_while_frozen`). [G] The bodies' own
velocity as they go dynamic (see "Bodies alive") is the actor's, added
only with `NV_GUESSES=1`. The ragdoll stops being stepped once still
for a second (this solver's), which stands where the game detaches the
settled bodies.

Seen live (release viewer, `death-2026-10-06`, WastelandNV at
(−67845, 3000, 8400), `--freeze-ai`, Easy Pete and Cheyenne moved in
front and `Kill`ed from the console): `before.png` both standing,
`after.png` 10 s on: Pete on his back, Cheyenne on her back against the
sign post, each given only the nudge (20 units/s, the way they faced).
The same kill with the earlier binary printed no "goes limp" and left
her standing (`still2.log`). Walking, unfrozen (`still1`, `dbg3`) she
falls as before. Neither comes to rest within 10 s (this solver's
jitter, below).

## People on the ground (B4, `claude/b4-npc-ground`, 2026-10-07)

Playtest: people phase into the terrain or fall through it. Private
outputs (decompiles, logs, screenshots, `ground.ps1`):
`%USERPROFILE%\nv-re\work\b4`.

### What was wrong (found live, `NV_GROUND_LOG=1`)

The viewer logs everyone on screen once a second with `NV_GROUND_LOG=1`
(`ai::ground_log`: feet against the collision under them, cast from 600
above, and the land's `LAND` height, with the controller's state).

- **Ghost Town Gunfight** (run1, the acceptance command, `--wait 200`):
  Ringo (00104C7D) comes into view from his walk out of sight
  (`move_offstage` walks straight between rough positions) 64 under the
  land at (−73298, 3829). With no collision within 256 + 64 below him the
  viewer gave him no controller (its loading bridge) and he walked in place
  under the ground for the whole run (−64 to −76). Moving the player to him
  (`player.MoveTo RingoRef`) put the camera under the terrain
  (`ringo-base\shot.png`).
- **Back in the Saddle** (run2): Sunny, back from out of sight beside a
  tree, sank 29 under the land with a controller (pushed down out of the
  tree's collision) and kept losing and remaking it.
- Everyone else walked with their feet 2.8 above the land (its 3.5 shell
  less the hull's 0.7 lift) or on statics.

### How the game keeps them up

`MobileObject::Move` (its message "AI: MobileObject::Move called on '%s'
(%08X) with invalid angle."; `0092f260`), every actor's move, the
player's too, has two rules around the character controller
(`world::ground`):

| What | Where | Rule |
| --- | --- | --- |
| Far from the camera | `0092f849`…`0092fc6e`; `fCharControllerWarpDistSqr:HAVOK` (setting `01267bd4`, exe default 6,000,000 at `010c4b18`, in no INI); camera = world scene graph `011deb7c` +0xac, world translation +0x8c (`00524c90`) | not the player, not an immobile creature (vfunc +0x4b4, `0087d750` → `005f0c80`: `CREA` with `ACBS` 0x800000), not knocked down/dead/dying/unconscious; squared distance > the setting: the move (turned by the heading) is added to the position and z set to the navmesh's height there (`006d97b0` → `006d9830`: the location resolved to a triangle, `006dd6f0`, its height `00697980`, within ±180); sitting keeps z; no triangle → the controller as usual. `SetPosition` (vfunc +0x2a8, `00931620`) moves the controller there too |
| Under the land | `0093012a`…`00930186` | after the controller's move (controller flag 0x800 clear), outdoors only (TES +0x34 none): the land's height under them (`004572e0` → `0053b550`/`0053f1e0`, a loaded cell's `LAND`; −2048 when none) more than 30 (double `0101db88`) above the feet: put on it |
| Warped back | `009300c7`…`00930126` | controller more than 8192 (²: `0108a7a0`) from the reference: the reference's position is kept (not needed here: the viewer has one position) |

`fCheckPositionFallDistance` (`00f63a80`) has no reader.

Havok's land for the controller is the heightfield's triangle collection
(`00cb0820` → `00d248a0`, `00d26700`, `00d26b50`): radius 0.5 Havok, no
triangle extrusion (zeroed at `00d26700`), no welding; nothing in the shape
pushes something under it back up: the 30-unit rule does.

### Implemented, tested

- `world::ground`: `moves_without_controller`, `navmesh_height`,
  `kept_above_land`, `immobile`, the constants; tests (the 2449.5 edge, the
  exceptions, 30 exactly, the navmesh window).
- `NavMesh::find_triangle` looks only at the triangles indexed over the
  point's square (same order, so the same triangle: a generated test
  against the full scan, 4000 points over 400 overlapping triangles).
- Viewer `ai::move_body`: the far rule first (`far_move`), the land rule
  after the controller and on the no-controller bridge; `walk::walk`: the
  land rule for the player (`CellNav::land_height`, the attached squares'
  land of the player's worldspace). Tests: someone 64 under the land is put
  on it and given a controller; a controller 29/31 under the land stays/is
  lifted; far from the camera people walk through a wall on the navmesh's
  height, near it the controller stops them.

### Verified live (release viewer, installed data; nothing compared)

- Gunfight (run4, `--wait 330`): XP +50; Ringo comes in on the ground
  (+0.2 to +5) and walks up to the player (`run4\end.png`); before
  (run1) −64 to −76 for 200 s. `ringo-fix\shot.png`: Ringo and Sunny
  walking on the ground where the old build showed the terrain from
  below.
- Gangers coming down the road (`gangers-fix`, `--at -67000,1300,8400,163`,
  24 s): on the road, feet +0.3 to +2.9 (`shot.png`).
- Back in the Saddle (run3): completed, XP +50; Sunny's walk back from
  out of sight at the tree is on the navmesh (far from the camera),
  within −14 to +11 of the land; no one with a controller under the land.
- The player started 54 under the land (`--at -67845,3000,8330,180`):
  the old build stayed under the terrain for 8 s (`player-base\shot2.png`),
  now on it (`player-fix\shot2.png`).

### Not done / seen

- People far from the camera stand at the navmesh's height, which lies up
  to 21 under the land in places (00157B39); as the game does by this rule,
  not compared.
- A dead ganger's ragdoll lay sunk to the waist after the gunfight
  (`run4\end.png`): ragdoll capsules meet triangles from either side
  (`physics::ragdoll`; PR 8 makes them the solver's contacts, keeping the
  face's side for a point gone through); not rechecked here.
- The viewer's bridge (no controller until collision is within 320 below)
  stays; the far rule's controller warp conditions (flags 0x2000000,
  0x4000000, 1) and in-air reset (`008e2680`) aren't followed (the
  controller is put at the new position always); `fCharControllerWarpDistSqr`
  isn't read from INI files.

## The character proxy (B1 PR 10, `claude/b1-character-proxy`, 2026-10-08)

Every walking actor, the player too, is now moved by the game's own
character controller: Bethesda's `bhkCharacterController` around Havok
7.1's `hkpCharacterProxy`, translated from FalloutNV.exe 1.4.0.525 with
the Xbox PDB's names and layouts. It replaces the viewer's own capsule
(stepped and snapped down, pushed out of overlaps). Code:
`physics::proxy`, `physics::simplex`, `physics::controller`,
`physics::character_cd`; `physics::Character` keeps its interface.
Private outputs (decompiles, logs, screenshots): `%USERPROFILE%\nv-re\work\b1cc`.

### Shape and settings

| What | Where | Value |
| --- | --- | --- |
| Shape | `00c72410` → `00c70de0`, convex vertices `00cd7f50` | 18 points: a bottom point, an 8-point ring (directions `011b0154`/`011b0174`, 0.7071) of radius 20.25 at step + 0.1 Havok above it (31.7 units), a ring at 53.875 above the centre, a top point; convex radius 0.1. The capsule built beside it isn't used for people |
| Centre | `00c72410` (+0x538) | half the height + 2 × keep distance above the feet: the hull's surface is at the feet |
| Proxy cinfo | `00c6cde0`, `00c6da50` | dynamic friction 1, static 0, keep contact tolerance 0.1, keep distance 0.05, contact angle sensitivity 10, 4 user planes, solver speed 100 (Havok's 10), strength FLT_MAX, mass 0, max slope pi/2 (never acts), penetration recovery speed 1, cast iterations 10 (4 set by each move) |
| Listener | `00c6c7a0` | max slope cos 47° (0.682), step 31 × scale (at least 0.75 × radius), cast depth = step ÷ tan 47° |

### One move (`bhkCharacterController::Move` `00c73170`, vtable +0xc8)

Called once a frame by `MobileObject::Move` with the frame's time, the
displacement and the actor's speed (`fSpeedPct` = speed ÷ the player's
running speed `01267bc4`).

1. Standing still (displacement within 0.05) clears flag 0x8; the wanted
   velocity is the displacement ÷ dt (z dropped unless swimming/flying).
2. The proxy's velocity is the starting velocity (its z cleared standing
   still unless in the air).
3. The support check (`00c6cef0` → `00cae900`), when not on the ground,
   moving, or without a support normal: the manifold's constraints, as
   the listener changes them, solved for 1/60 s of a unit move straight
   down; supported when that stops or turns it; the surface normal and
   velocity from the touched constraints facing up more than 0.08. The
   controller is supported when that says so and not every support was
   walled off, or when anything counted as support (flag 0x200); while
   jumping (flag 0x2000) never, until the jump is over.
4. The state's update sets the velocity:
   - on the ground (`00cd4800`): not supported → the vertical velocity
     cleared, the fall measured from here, into the air; otherwise Havok's
     movement util (`00d6aef0`) moves it toward the wanted velocity in the
     support's plane (gain 1, ≤ 500 an update), keeps the old vertical
     velocity and adds gravity × 0.5 × `fSpeedPct` × dt; a wanted jump
     starts only on walkable ground (flag 0x400);
   - jumping (`00cd4280`): √(2 g h) up **added to the proxy's own
     velocity** (the run-up is kept), then into the air at once;
   - in the air (`00cd3fb0`): supported → landed (flag 0x80, the fall
     damage's cue) and on the ground; else steered across the up axis
     (gain `fAcrobatics` × 0.3, ≤ 2000 an update), gravity added.
5. Standing still in a manifold equal to the last one (`LastManifold`,
   within 0.001, same bodies) **the proxy isn't integrated at all** and its
   velocity is zero; otherwise `hkpCharacterProxy::integrate`.

### The proxy's update (`00cade20`, `integrateImplementation`)

Up to 4 passes while time is left: cast the shape by the last solved move
(`m_oldDisplacement`), collecting what's within 0.15 of the start;
`updateManifold` (`00caf4d0`: each point replaced by the most alike start
point within 1.1 by `surfaceDistance` `00cafd90`, or dropped; the closest
new start points and the cast's first hit join; near-duplicates within 0.1
dropped); one surface constraint per point (`00cacd60`: the plane kept
0.05 back, fixed bodies priority 2; **a crossed plane becomes a velocity
out of it of 1 Havok unit a second per unit crossed**); Bethesda's
listener; Havok's simplex solver (`00d24360` with `00d23f50`, `00d234e0`,
`00d23480`, `00d23850`, `00d23ca0`, `00d23380`); if the solved move
differs from the cast one by more than 0.001, cast it and stop 0.05 short
of the first new surface (`00cacc50`). The last solved velocity is kept.

### Bethesda's listener (`processConstraintsCallback` `00c711d0`)

Per manifold point: other controllers (layer 30, phantoms) are vertical
walls and nothing else; dynamic bodies (clutter, weapons, projectiles)
count as support straight up; `ANIM_STATIC` fixed or keyframed bodies get
extra down friction 0.577; ground facing up at least 0.682 is walkable
(extra down friction 5.77 standing still); steeper ground is a **step**
(`IsStep` `00c6eb00`: the contact no more than step/3 above feet + step,
and a segment at feet + step over the contact, the cast depth further on,
clear of the contacted triangle) and becomes a ramp (normal (x, y, 1)
normalized), or else gets a vertical wall added (≤ 4) and under 0.2
doesn't count as support. On the ground dynamic friction × `fSpeedPct`.

### Not translated / stand-ins (labelled in code)

- Havok's collision agents, what is translated and what not. The proxy's
  cast is the game's GSK agent cast (`hkpGskBaseAgent::staticLinearCast`
  `00daf8e0`; registered by `00d41fd0` for convex against convex, the
  agent functions `{create, getPenetrations 00dac2b0, getClosestPoints
  00dac450, linearCast 00daf8e0}`, a `TtGsk` timer in each): the closest
  points are asked within the start tolerance plus the path's length; the
  cast counts only if `distance + path . normal <= 0` and the path goes
  into the surface by more than the 0.01 extra penetration; a first
  distance within the early-out 0.01 (or penetrating) is a hit at
  fraction 0; otherwise the fraction starts at `distance / -(path .
  normal)` and up to 20 closest-points queries at the moved position
  reject (nothing near, moving away, the rest of the path too short),
  accept (distance within 0.01) or advance the fraction by `distance / -(path
  . normal)`; running out of iterations accepts the last query at the
  fraction it was made at. The hit's point and normal are the last
  query's. (`character_cd::gsk_linear_cast`; the swept-AABB culling of the
  MOPP/world caster `00df7340`/`00df7760`, which passes the same input
  and the all-points collector, is our candidate search.) The static
  getClosestPoints (`00dac450`) is followed for its results: normal from
  the second shape toward the first, distance less both convex radii,
  the point on the second shape's surface, nothing beyond the tolerance.
  Not translated: the GJK underneath (`00daad40`: separate simplexes for
  the two shapes, alternating support queries, a doubling tolerance from
  1e-10, helpers `00da9340`, `00da8fb0`, `00da8500`, `00d22e40`,
  `00da9880`) and the penetration depth for overlapping cores
  (`00daa7e0`, called from it when the caller's flag is 0, as
  getClosestPoints does; getPenetrations `00dac2b0` sets it and treats
  overlap as a hit): here a generic GJK and separating axes, so which of
  several equally close points (a hull's side against a wall) is the
  contact point is ours (`contact_on`), as are the convex-vertices and
  triangle shapes' support-vertex tie breaks. `IsStep`'s convex-shape
  branch (a ray against the whole shape) uses the triangle test too.
- `applySurfaceInteractions`: translated (below). Left of it: the listener
  callback for bodies with property 0x1300 (`00c6ca30`, moving platforms,
  with the moving platforms above) and other characters' interaction
  callbacks.
- Bethesda's point collector (`00cd36a0`: edge-hit filtering, trigger
  bookkeeping), the support material and velocity (`00c6e980`), moving
  platforms (`01267bb4`), hurtful bodies, pitch and roll, lying
  creatures, creatures' shapes from `BSBound`, the swimming, flying,
  climbing and projectile states, `VelocityMod`.

### Pushing bodies (`applySurfaceInteractions` `00cacf80`) and `fSpeedPct`'s divisor

Called by `integrateImplementation` (`00cade20`) after each pass's solve and
before the cast move (`StApplySurf`), with the update's time and the world's
gravity, for every manifold point (so up to 4 times an update). For a point
whose collidable is a rigid body (type 1) that isn't fixed or keyframed
(motion types 4, 5): the listener's `+0x10` for bodies with property 0x1300
(moving platforms, not translated; the character listener's `+0x14` is a
no-op stub, `00c6c760`); then, with `n` the point's normal toward the
character, `v` the body's velocity at the point (`v + ω × (p − centre of
mass)`) less the proxy's velocity (`+0x10`) along `n`, `f = −0.9 v`, plus
`0.4 ÷ dt` × the distance when it is negative; `f < 0`: impulse
`f ÷ (1/m + (r × n)·I⁻¹(r × n))` along `n`, no less than `−strength × dt`
(`+0x6c`); then `g·n × dt` (less `v` when `v < 0`) × the character's mass
(`+0x70`) along `n` when below −1.19e-7. The body is woken (`00c9c1d0`: its
deactivation counters cleared, activated) and the impulse applied at the
point (`applyPointImpulse`, motion vtable `+0x50`) whatever its size (a
resting touch zero), so **any body within 0.15 of the character stays
awake**. Bethesda's settings feed it nothing: the cinfo's strength is
FLT_MAX and mass 0 (`00c6cde0`, nothing sets them later), so a touch gives
the body 0.9 of the closing speed whatever its mass (through its centre of
mass; `fMoveLimitMass` isn't read here), and the weight term is zero.

In nv-rs: `Proxy::integrate` records the touches (`proxy::SurfaceContact`:
the triangle's placed reference, point, normal, distance, the proxy's
velocity, dt, strength, mass), `Character::surface` hands them out, and
`RigidWorld::apply_surface_interactions` applies them in order (the body
state doesn't change between passes but by these impulses, so applying
afterwards equals applying in the pass). The viewer's `clutter` applies the
player's and the people's (`ai::move_body`) once per update. The walker rule
(`Mover`, `WALKER_WAKE_SPEED`, `PUSH_SKIN`, `MOVE_LIMIT_MASS`, keyframed
capsules in the island solver) is gone. Not like the game: nothing keeps a
pushed body out of the character (the game's characters also have the
controller's world presence; not traced), so a pushed body can end inside
the hull the proxy has stepped into (dynamic bodies are support straight up
and never walls, `00c711d0`). `NV_PUSH_LOG=1` logs the player's feet and
each body the proxies touched.

`fSpeedPct`'s divisor (`01267bc4`, zero in the executable) is written only
by `0055e230` (when above zero), from `0055d760` when the player's
controller is set up: `00647f00(player's actor values, 0, 0, 0, 1, 0, 0)`
= `00647d10` × `fMoveRunMult` (`011d0898`, 4). `00647d10`: SpeedMult
(actor value 21) × 0.01 × `fMoveBaseSpeed` (`011d0448`, 77) × `fMoveOneCrippledLegSpeedMult`
(0.85, `011d11b8`) or `fMoveTwoCrippledLegsSpeedMult` (0.75, `011d0268`)
when one or two legs are crippled (actor values 29, 30; 1 otherwise; 1 too
when actor value 72 is set), with
the sneak, no-weapon (`fMoveNoWeaponMult`) and armour terms off for these
arguments. So 308 for SpeedMult 100 and sound legs, and otherwise the
player's own: `physics::controller::set_reference_speed` takes
`world::animation::run_speed` once, at the player's first walking update
(the game keeps it from loading: later changes of SpeedMult or legs don't
change it). The people use the same one value.

### Tested (generated)

`physics` tests: walking up a 16-unit step but not a 32-unit bench or a
64-unit block; 30° and 45° slopes climbed, 50° and 60° not; standing on a
box and walking off its edge (fall 64 counted from the edge); falling and
landing (feet 0.7 + 0.35 above a model's triangles); a wall stopping the
hull's corner the shells and keep distance away; people as walls nobody
stands on; a jump rising 64, keeping the run-up and landing; sunk 40 into
the land: standing still it stays (the move skips integration), walking it
comes up at the recovery speed; the hull's 18 points; GJK and the
overlapping case; the simplex solver's own six (`physics::simplex`).

### Verified live (release viewer, installed data, `NV_GROUND_LOG=1`; nothing compared with the game)

The ground log now gives each actor's controller state, and the player's
feet ten times a second.

- Acceptance: doc, vcg02, vms16 all pass (one run).
- Ghost Town Gunfight (`--wait 330`, XP +50): 5465 log rows of people with
  a controller; their feet never more than 10.2 under the land. The one
  case: a ganger handed from the far-from-camera rule (navmesh height, B4)
  to his controller 10 under, risen to +4 a second later. Everyone else
  stands 1.0 above statics (shell + keep distance) and 3.8 above the land
  (its 3.5 shell + 0.35). Back in the Saddle: 938 rows, none under the
  land. Gangers coming down the road (`gangers\shot.png`): on the road.
- The player: standing jump 61 high (sampled every 0.1 s), 0.85 s in the
  air; a running jump uphill keeps its 330 units a second and lands on the
  slope; walking off the road's edge falls and lands on the land; walking
  up and down the hill east of the road at +3.7 to +4.2 over the land; a
  16-unit ledge inside the saloon's shell ridden up in 0.3 s.
- B25 (people waist-deep while moving) didn't happen in these runs with
  the game's controller; it was likely the viewer's own controller's
  push-out, or needs another place to reproduce. The deepest anyone stood
  was the navmesh hand-over above, which the game has too (its far rule).
- **Pushing bodies through the proxy** (`NV_PUSH_LOG=1 --answer-boxes`, at
  −69538, 2700 heading north, W held 3 s from 3 s; nothing compared with
  the game): the player walks into the two Goodsprings crates
  (0010B8CF/0010B8D0) in front of the general store: 244 log lines of
  touches, the crate at speeds of about 115–175 units a second while
  touched (the walk's), moved from (−69538, 2825) to (−69570, 2844), the
  pair coming to rest at 5.7 s, 52.5 and 67.2 units from where they were
  put (the player arrived at about 4.5 s). The
  specified tumbleweed route no longer meets 00178A80: with the wind it is
  960 units off by 5 s (the player starts walking at 3 s); the run logged no
  touches. Without `--answer-boxes` the "Classic Pack" notice keeps the
  player from moving.

Seen, not changed: an actor placed inside a wall (a bad `--at`) is held
there: its penetration recovery is a slow velocity, and a beam on the
other side stops it, as Havok's proxy would.

## Frozen bodies and far-off clatter (`claude/physics-clock`, 2026-10-08)

Playtest of build 28: moved objects lag or freeze in the air; the
clattering is back.

**Frozen in the air: the game's frame timer.** The viewer runs uncapped
(mailbox, ~210 frames a second in Doc's house). `bhkWorld::SetDeltaTime`
(`00c66760`, [`physics::havok::Clock`]) rounds the carried time plus the
frame to whole steps and carries the rest, which goes negative after
rounding up (a frame of 0.6 steps runs one and carries −0.4); a total at
or under 0 returns early *without touching the carried time*. So once
frames are shorter than the negative carry (under ~6.4 ms) no step runs
again until a slow frame comes: bodies hang where they were, then jump.
The game never meets this: its frame timer (`00aa4ee0`, the timer at
`011f6394` that `0086f260` passes to `00c66760`) counts whole
milliseconds of `GetTickCount`, sleeps out a frame under 10 ms and reports
10, caps one over 166 ms at 166. `physics::havok::FrameTimer` translates
it: the viewer's frame times are gathered until 10 whole ms have passed
and handed over as one game frame (the fraction kept); the bodies' world,
the wind and the ragdolls step only on those frames
(`clutter::HavokFrame`). Tests: `havok::tests::
short_frames_through_the_timer_keep_the_clock_stepping` (straight from
210 Hz frames: 1 step in 2 s; through the timer: 125).

**The clatter: contact sounds everywhere.** `ImpactMixer::
PlayCollisionSound` (`00837550`) first checks each side's sound can be
heard (`0082eca0`: on each axis within the sound's largest distance byte
(`SNDD` +1, `00553b90`, raw) × 100 of the listener, else nothing at all,
the 333 ms gate untouched), then plays it at the contact point (flags
0x4102, the sound's own distances, position `00ad8b60`) with the
contact's static attenuation and frequency (`00ad89b0`, `00ad8a90`). The
viewer queued them as flat 2D sounds at full volume: every tumbleweed
rolling in the wind anywhere in the loaded squares rattled in the
player's ears (163 `PHYBabyRattle`s in 16 s at the VCG02 fence).
Now `clutter::play_contact_sounds` starts them with
`weapon_fx::play_at_with` at the point; `within_reach` is the box test.

Seen in the build 28 playtest as "a tumbleweed blown fast enough passes
through the land" (00178A82, from z 7992 to 1082 in 5 s): **it isn't
tunnelling.** It rolls at 50–150 units/s (2–3 units a step) and falls
freely as it crosses y = −4096, where the viewer's loaded terrain ends
(`--at -68232.9,4990`: the squares y −1 to 3; 2265 triangles in
x −66000..−63000, y −4096..−3000, 6 south of −4096 in z 7000..8500). The
game's own loaded grid ends somewhere else; what the game does with a
body that rolls out of it isn't traced (left).

## Continuous collision (B1 PR 9, `claude/b1-ccd`, 2026-10-08)

`physics::continuous`; `RigidWorld::continuous`. The world is continuous
(`iSimType` 1, `00c681c0`; `hkpContinuousSimulation` ctor `00d0d020`).
Each body's quality is the model's cinfo byte (`bhkRigidBody::LoadBinary`
`00c8ea30` only byte-swaps the blob; `00c8d4a0` names 0 fixed, 1 keyframed,
2 debris, 4 moving, 5 critical, 6 bullet, 7 user, 0xff invalid; 3 is
`DEBRIS_SIMPLE_TOI`, Xbox PDB). `RigidSetup::quality` carries it. This
settles B1 question 3 of the Havok map: **the tumbleweeds (all twelve
near the start) and the bottle model carry 3**, and the dispatcher's
quality table (`00cfb570`, `+0x1bb0`; the whole table is in
`continuous::pair_quality`) gives fixed against 3 the collision quality 2,
simplified time of impact (against debris 2: a plain step; against
moving, critical, bullet: the full one).

| What | Where | Rule |
| --- | --- | --- |
| Queue | `00d0d150` | the narrowphase (`00d0d8b0`, "TtNarrowPhase") queues an event per pair whose agent found a time: time, separating normal, agent; the quality info's `+0x14` byte (set for index 2, `00cfb570`) marks it simple; 250 events (`sizeOfToiEventQueue`), a full queue asserts and loses the event |
| Order | `handleAllToisTill` `00d0e9c0` (from `advanceTimeInternal` `00d0ed10`, "TtTOIs") | the earliest first, a tie by the bodies' ids (`+0xd4`); an event whose agent no longer confirms it (`00d0bc20`) is dropped |
| Handling | `handleSimpleToi` `00d0e210` ("TtSimpleTOI") | for each body of quality 3: the swept transform is set to the event's time (`00cf1d50`: position lerp, rotation by two half blends through the normalized sum of the ends, then both ends of the sweep are that pose), the body's other queued events and its agents' predictions are dropped (`00cc0ea0`); velocities untouched. The contacts at that pose come from the next collide, the solver acts on the next step; the rest of the step's travel is lost |

Translated in `RigidWorld::continuous` / `swept_pose`: the table, the
event order and queue, the handling. **Not traced, labelled in the code:**
how the narrowphase finds an event's time. The predictive agents
(`00cfedd0`, the tables at dispatcher `+0x16b8`/`+0x16dc`) keep a
separating plane, bound the pair's remaining separation by the bodies'
travel and turn, and run the shape pair's linear cast when it falls
below the collision tolerance; the separation the time is for comes from
the quality info's fractions (−0.5, −0.25, −0.35/(n−1)…, `00cfb570`,
n = 3 for simplified) of the bodies' allowed penetration depth. Here: the
first time a corner (sphere, capsule end) of the body touches a triangle
that the step carried it through (behind its plane at the end, the nearest
point straight below), by conservative advancement; only when the step
carried a point further than the PSI's own reach (`point_triangle`), so
slower bodies behave as before. Edges against hull faces, capsule axes
and events between two bodies (quality 3 bodies meet at 1: a plain step)
aren't swept. Not translated: the full time of impact (`simulateToi`
`00d100a0`: a solve at the time and the rest of the step again) for
quality 4 and 5 bodies, the `toiCollisionResponseRotateNormal` (0.2) and
`numToisTillAllowedPenetration` bookkeeping, the safe-time backstep for
them. Unit-tested: a ball and a slanted box shot at a floor of no
thickness at 1500–9000 units/s stay above it; as debris (quality 2) the
same ball goes through. Live (the PR 9 route, 40 s, release viewer, tumbleweeds all quality 3): 00178A82 still goes down at (−64959, −4254) in the run with this on (z 8000 at 10 s at y −4109, 724 at 15 s), in the other three runs it stayed on the land; the runs differ (the dice). So the continuous collision doesn't touch that fall: it is the terrain edge.

## Not compared / gaps

- Nothing compared with the original game: how far bottles fly, how they
  tumble, settle heights, rest times, grab feel, sound choice and volume.
- Havok's contact solver, contact manifolds and penetration recovery
  are not reproduced yet (B1 PRs 6–9; the step driver, integrator,
  deactivation and simulation islands are, above). Clutter constraints aren't
  simulated: joined or constrained clutter bodies stay solid (ragdolls'
  joints are solved: PR 8). Inertia under a reference's
  scale isn't traced (scaled by s², mass kept).
- Dying: the living bodies' velocity handed over at death (which branch
  of `00c65b00` the biped takes) isn't traced; the `Death` group's
  post-animation action (`0089d900` asks the 3D for 0xe0 and, when it
  has one, adds process post-animation action 0x20, `00903180`) isn't
  followed; creatures without a ragdoll still tip over as a stand-in
  (`ai::fallen_transform`) instead of playing `Death`. Ragdolls
  settle now (PR 8); the dog's ragdoll and the other creatures' weren't
  run again.
- The wind listener's call each frame follows `00c6ae70`; the wind
  direction stays 1 rad (no other writer of the sky's `+0xd0` was found
  in the sky's code). Which slot of a multi-slot armour holds its model
  is taken as the lowest (every head slot names the same bone).
- Grab: actors/ragdolls (the complex spring and helper), letting go when
  standing on the held body, the keep-out near the player, the
  hold-Activate grab, `fGrabMaxWeightRunning`.
- Sounds play without the game's attenuation and frequency (the viewer's
  sounds aren't placed or scaled); shell casings' sounds; terrain has no
  Havok material here, so its side is silent.
- Physics damage isn't dealt: nv-rs has no destructible objects yet (the
  amount is logged). Living actors' bones are on the biped layer, which
  clutter's layers don't touch, so flying clutter doesn't hurt people.
- Shots meet bodies on layers in the walking collider only: living
  bones (8) and dead bodies (29) aren't in it (people are met by their
  own capsules in `combat`).
- Explosions' source-reference rule (no body counts as the source's), the
  phantom's exact overlap (centre within the radius), ragdolls taking
  shot/blast pushes (they use `physics::ragdoll`).
- The bullet cast's layer is taken as 6 (`PROJECTILE`) from its name.

**Next action:** record a bottle shot off the fence and a Z-grab carry in
the original game and compare.
