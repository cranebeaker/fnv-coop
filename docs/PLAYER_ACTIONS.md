# Player actions: activation, pickups, sneaking, sights, scopes, melee, death

M2 work, 2026-10-06, branches `claude/m2-player-actions` and
`claude/m2-player-actions-2`. What the player does in the opening, Back
in the Saddle and Ghost Town Gunfight beyond walking and shooting: the
crosshair prompt and E, taking things, sneaking with its meter, looking
down the sights and through scopes, the gun's sway, blocking and power
attacks, the ammo swap, and dying. Read from `FalloutNV.exe`
1.4.0.525 with Ghidra; (Xbox PDB) marks names from the Xbox 360
prototype's symbols.

Status words: **implemented** (code exists), **tested** (regression
test), **live** (seen working in the running viewer with real input, how
below), **compared** (checked against the running original game).
**Nothing here has been compared with the original game.**

## Activation and the Info panel

| Address | What | Status |
| --- | --- | --- |
| `00579280` | Action class by base form type and state: ACTI with a name 4, NPC 7 (sneaking 1, dead 2), CREA only with `ACBS` 0x200000 (`GetAllowPCDialogue`, `0047c790`), CONT 2, DOOR with a name 8, FURN `MNAM` 0x40000000 3 / 0x80000000 5, BOOK 6, LIGH carryable 1 else 4, TERM 4, PWAT 14, items 1; destroyed non-actors nothing | implemented, tested (`world::activation::action_class`) |
| `00f80050` | The words table at `011d5160` (`sTargetTypeTake` "Take" … `sTargetTypeRepair`) | implemented |
| `00775a00` | Info panel: crime → Pickpocket / Steal / teammate Talk; dead people Search; doors Pick / Close / Open; books "Take"; name `%s (%d)` with the count; load doors `%s %s %s` ("Door to Goodsprings"); lock line (`sBroken`, `[sLocked - sHUDUseKey]`, `[sLocked - <level>]`, `[Requires Key]`); `sEmpty` for empty containers and bodies; weight `%.1f` / value `%.0f` (`%.1f` under 1, `--` at 0) for ARMO, carried LIGH, WEAP, AMMO, MISC, ALCH, IMOD (weapons ≥ 10 × perk entry 73); crime colour on all lines | implemented, tested |
| `0076bfe0` | Info lines' layout: lock / Empty at y 65 centred; weight right-justified at w/2 − 25, value at w − 30, labels at 10 and w/2 + 5, y 60; `template_info_seperator` | implemented, tested (`ui::hud`) |
| `00586500` | `TESWorldSpace` +0x138: the map region (`RDAT` 4 / `RDMP`) holding the point, highest priority; the cell's `XCLR` regions first | implemented, tested (`world::region::location_name`) |
| `0070bc20`, `00631d60` | Pick: the view caster (below): sphere of `fActivatePickSphereRadius` 16 from the camera node along the view, `iActivatePickLength` 150, layer 40 | implemented, tested (`physics::view_caster`), live (below) |

Live (2026-10-06, Doc's house, posted window messages + F12 reports):
"E) Sit / Ruined Couch", E → temporary third person, seated first
person, E on nothing → stood up; "E) Take / Stimpak" with `--`/`75`
and the separator, E → "Stimpak added" with the neutral Vault Boy;
"E) Open / Gun Case"; Prospector Saloon: "E) Talk / Sunny Smiles",
sneaking "E) Pickpocket" in the crime colour. Real-data listing of Doc's
house: every chair/couch Sit, beds Sleep, the front door "Door to
Goodsprings".

The `WG`/`VAL` labels (checked again, second batch): they are made from
`template_justify_left_text` (`0076dd03`, HUD +0xb8 and +0xc0), whose
`HUDTemplates.xml` entry has `<alpha>0</alpha>`. Alpha is trait 4009
(0xfa9, `ui::names`); every instruction touching HUD +0xb8/+0xc0 is in
`0076bfe0` (x, y, brightness) and `00775a00` (visible 0xfa3, string 0xfc4,
system colour 0xff4) — none sets 0xfa9 (the weight value's tile, +0xb4,
gets its alpha set at `0076dce1`). By data and code the labels stay
invisible and only the numbers show. Still to compare with the game.

## The crosshair's pick (B3, `claude/b3-crosshair-pick`, 2026-10-07)

Playtest bug: some objects could only be activated by looking above them.
Cause: since the physics batches put clutter bodies into the collider, the
viewer took the *first* thing its 16-unit sphere touched; a bottle on a
table lost to the table top 8 units under the line, and the other
references were picked by their record bounds, people by a 25 × 130
cylinder, load doors by a box, each in its own system.

The game (`0070bc20` → `00631d60`, `ViewCaster.cpp`), now
`physics::view_caster` with `world::activation::fuzzy_replaces`:

- A sphere phantom of `fActivatePickSphereRadius` on layer 40
  (`CUSTOMPICK2`, `00631ab0`; the player's system group) is cast from the
  eye along the camera's forward column for `iActivatePickLength`, with an
  all-hits collector, sorted. Layer 40's row touches nearly everything
  (0–14, 16, 17, 19, 20, 22–32, 35–40, 42), bipeds, dead bodies and
  controllers included.
- For each hit (not the player): an exact pick along the line only, by
  Havok's ray against that body (clutter, weapon, projectile layers 4–6,
  or a static-layer shape of type 10, or form `011ca240`;
  `iMaxViewCasterPicksHavok` 10) or Gamebryo's `NiPick` of the
  reference's 3D (triangles, both faces, sorted;
  `iMaxViewCasterPicksGamebryo` 10).
- The fuzzy candidate (`bUseFuzzyPicking` 1): of hits not on layers 1,
  13, 17, 18 (6 only for a body; a sitting/sleeping actor's controller
  not), the one whose touching point is nearest the line, at most
  `iMaxViewCasterPicksFuzzy` 5 checked by a layer-40 ray from the eye to
  the point that must meet nothing or that reference first.
- Result: the nearest exact pick (Havok's only if strictly nearer); if none,
  or a static, static collection, tree or uncarriable light (0x20, 0x21,
  0x25, 0x1e), the fuzzy candidate at the distance to its point. Within
  `iActivatePickLength` it's the activation target (+0xfc), else only the
  crosshair's (+0x100). Settings' values from `00f56a90`…`00f56b20`;
  `Fallout_default.ini` doesn't change them.

The viewer (`viewer::crosshair`) picks once a frame and every user reads it:
talking (`dialogue`), load and swinging doors (`walk`), objects
(`scripts`), the Info panel (`hud`), Grab (`clutter`). The collider's
triangles now carry the placed reference they come from
(`Collider::add_placed`, `preview::cell`), whoever owns them.

Stand-ins, labelled in code: both exact picks are a ray against the
reference's collision triangles (the game's NiPick uses its drawn
triangles; the static "shape type 10" and form `011ca240` cases aren't told
apart); people are their controller's capsule (`CharacterShape::PLAYER`,
the size every controller has) when alive and their ragdoll capsules when
dead (a dead actor without a ragdoll offers nothing); collision on layers
that don't block walking isn't in the collider. Left out: the controller
exclusion while sitting or sleeping (needs the controller's unnamed
`+0x600`; the states are those `GetSitting`/`GetSleeping` count,
`0059dd90`/`0059dc90`), `0070bc20`'s second look for placeable water (cell
`+0xc4` `+0x5c`, `BGSPlaceableWater` by `0057b240`'s cast: drinking), the
HUD's `+0x75e` flag (taken as set).

Tested: `physics::view_caster` (8: bottle on a table picked at its middle
and fuzzily beside it, chair, a wall hiding a bottle, a person in front of
a table, the player never, fuzzy limits), `world` activation
(`fuzzy_replaces`), `preview` collision (every part names its reference),
viewer `scripts` (E uses the crosshair's object only).

Live (release viewer, `NV_LOG_CROSSHAIR=1` logs each change of target,
screenshots read back; private, `nv-re\work\b3\live*`):
Doc's house: dining chair 00103E45 "E) Sit / Dining Chair" (exact, 121);
the swinging door 001062AC "E) Open / Door" (44); the Stimpak 00105BF3 on
the medical shelf "E) Take / Stimpak" with the crosshair just beside it
(fuzzy, 108–113); Doc Mitchell "E) Talk / Doc Mitchell" (81); killed
(`--run-at 1 DocMitchellREF.kill`), his ragdoll on the floor "E) Search /
Doc Mitchell" (110). Prospector Saloon: the empty whiskey bottle 00107CB8
on the bar shelf "E) Steal / Empty Whiskey Bottle" (exact, 74–75); the
play build before the fix, crosshair on that bottle's body from the same
`--at`, said "E) Steal / Shot glass". Not compared with the original game.

## Pickups (`004ce380`, `008adcf0`)

Taking an item from the world: "<name> added" / "<n> <name>(s) added"
(`sAddItemtoInventory`, `sPlural`) for ARMO, BOOK, LIGH, MISC, WEAP, AMMO,
KEYM, ALCH, COBJ, IMOD, CHIP, CCRD, CMNY, with the item's pickup sound
(own `YNAM`, else a weapon's by equip type, else `UIItemGenericUp`). Before,
items without `YNAM` played nothing (the playtest report) and the message
was the viewer's own. Live: "Stimpak added" (the sound is logged as
"Pick-up sound: …"). Critical hits by the player: "Critical Strike on
<name>" / "Sneak Attack Critical on <name>" with the very happy Vault Boy
(`0089a760`), implemented, not seen live.

## Sneaking

| Address | What | Status |
| --- | --- | --- |
| `0093e860` @ `00940d5b` | Sneak control (8, Left Ctrl) toggles the mover's 0x400; not dead, not in furniture, not in anim actions 0, 1, 8, 10–16 (table `0094423c`); `NPCHumanCrouchDown`/`Up`; out of the sights | implemented, tested, live |
| `00952ff0`, `0094e1d0` | First-person eye is the 1st-person skeleton's `Camera1st`: 118 in `mtidle.kf`, 78 in `sneakmtidle.kf` (read from the files); the 40 drop blends over `fAnimationDefaultBlend` (the linear eye follow is this viewer's reading) | implemented, tested, live (lower view) |
| `007732d0`, `00770430` | Sneak meter: in combat [DANGER] (red, three flashes, `00a07c60` mode 2 / `00a080d0`) or [CAUTION] when all fighters search (`fCombatDetectionLostTime`); hostile detection [CAUTION]; else highest detection (`00973710`) < 1 [HIDDEN] else [DETECTED]; fades in/out 0.5 s | implemented, tested, live ([HIDDEN]; [DANGER] in red with Doc fighting the player after `DocMitchellREF.StartCombat player`; [CAUTION] and [DETECTED] not reached in a run) |
| `005f38d0` | Animation move-type prefix "Sneak": third-person body plays `sneakmt*.kf` (falling back to plain groups is assumed) | implemented, live (third person in Doc's house: the head drops from shoulder height to the bottom of the frame when Ctrl toggles sneaking, reports 019/020) |
| speed | `fMoveSneakMult` through `world::locomotion` (existing) | implemented |

Hostile detection's own test (`008b06d0`) isn't followed: "attacks on
sight" stands for it.

## Iron sights (`world::iron_sights`)

| Address | What | Status |
| --- | --- | --- |
| `0093e860` @ `00941f4f` | Aim control (`Block=00380110`: right mouse, Left Alt) held → sights up for a drawn gun; released → down; not while switching view, in V.A.T.S.; melee/fists block instead (below) | implemented, tested, live |
| `0095de30` | FOV eased at 30 × dt ÷ `fIronSightsFOVTimeChange` to the weapon's sight FOV (`DNAM` +28) and its share of the 1st-person FOV; `fIronSightsZoomDefault` with none; `bIronSightsZoomEnable` | implemented, tested, live (zoom) |
| `008bb650` | Weapon groups +3: `<kind>aimis.kf`, `<kind>attack…is.kf`, blended in over 0.2 s (model-space mix: viewer simplification) | implemented, live |
| `008bbbf0`, `00771700`, `00874c10` | `bTrueIronSights`: `##SightingNode` kept; crosshair hidden in first person; the first-person camera at the node | implemented, live (varmint rifle: front post centred in the peep ring, no crosshair) |
| `0093e860` | No running while aiming (`locomotion::may_run`) | implemented |

The player's shot cone stays the weapon's min spread (`findings\hits.md`:
the aim term only feeds the sway). The sway's rotation also lands in a
global (`011e09ec`, written last in `00962de0`) that shots may read; not
followed (shots leave along the view).

## The first-person model around menus (`viewer::viewmodel`, B23)

| Address | What | Status |
| --- | --- | --- |
| `00870bd0` @ `00870c7b`..`00870ca5` | The frame's first-person pass (`00874c10`, `00875110`) is drawn unless the first-person node is culled, the Pip-Boy is drawn on its own (`InterfaceManager+0x1dc`, its node at +8; only with `bUsePipboyMode` off) or `011d9514` is set. Menu mode itself doesn't hide it: behind other menus (containers, messages) the hands and weapon stay in the world | implemented |
| `007617a0` / `00762160` / `00761960` | `011d9514`: set by the dialogue menu's constructor, cleared when it closes and by its destructor; so no first-person pass while the dialogue menu is in being, also under the barter and repair menus it opens (`00763ff0` keeps it, hidden) | implemented; a line said with `SayTo` (no menu) keeps the view |
| `007f8ba0` | The Pip-Boy is the one first-person model in the game (`pipboyscreen` found under node `00950bb0(1)`), playing `Pipboy.kf` over the hold pose; here its arm is a model of its own holding the hands and weapon, so the ordinary view stays hidden while that arm is up, being raised or being put away (to `Pipboy.kf`'s end, 0.73 s), and comes back when it's down | implemented |

## Scopes (`world::iron_sights::scoped`, `viewer::scope`)

| Address | What | Status |
| --- | --- | --- |
| `008bb650`, `00962de0` | Scoped = sights up, the weapon's `MOD3` scope model, out, the view not switching; "scope from mod" (`DNAM` flags2 0x2000) needs a zoom mod (effect 0xe; this viewer keeps no mods, so the varmint rifle has none) | implemented |
| `008bb650` | Scoped: the first-person model hidden (`SetAppCulled(1)`) | implemented, live |
| `00771700` | HUD mode 0x17: mask 8, the enemy's health only | implemented, live (no compass, meters or crosshair) |
| `0076bfe0` @ `0076fd02`, `0077ee50`, `0077f2f0`, `0077f0d0`, `00709d50` | Overlay: the model under the HUD's scope node (X(π/2)·Z(π/2), scale 0.1, 380 ahead of its camera; the model's root translation zeroed), frustum a = min(0.1 × `fDefaultFOV` × π/180 × 1.1, 1.53938), half height 0.75·tan a, near 1, far 5000; drawn into the HUD picture before its pieces, unlit, blended per `NiAlphaProperty` | implemented, tested (framing), live (Boone's scoped hunting rifle in Doc's hall: black frame, lens shading, green tint ring, crosshair, centre circle; report 002) |
| `0095de30` | Zoom: the sight FOV (same easing as iron sights); mod 0xe's value taken off it | sight FOV implemented, mod part not |
| `00962de0` | Sway: `ScopeWobble.nif`'s change since the last frame × the wobble (`008b0dd0(0)`) × `fGunWobbleMultScope` (data 1), X to the pitch (`00931e50`), Z to the heading (`00931d30`, `SetHeading` (Xbox PDB)) | implemented, live (pitch −1.01° → −0.39° over 1 s with the mouse still, reports 002/003) |

Assumptions (labelled in code): the scope model's own root rotation is
kept (`0077f2f0` sets a rotation from a value the decompiler lost; with
the root's tilt the model faces the camera, without it it is edge-on);
the shaders' view-angle falloff on the overlay isn't applied; Bevy's
transparent pass doesn't write depth (the game's overlay meshes do).
Not done: the world scissor while scoped (`00870bd0`,
`fScopeScissorAmount`: the frame covers it), forcing first person from
third (`00950460(1)`: here the scope only comes up in first person).

## Gun sway (`world::gun_wobble`, `viewer::viewmodel`)

`00962de0`: the weapon kind's `WeaponWobbles\<kind>.nif` (table
`0118a838` → `011977a4`: `2HR` for rifles, `1HP` pistols, …; melee kinds
have no file) sampled at the first-person clock relative to its
controller's start, its X, Y, Z angles (`00a592c0`) × the amount and
rebuilt (`00a59540`), set on an `AdditionalRotation` controller
(`00c8ffd0`, update `00c90030`) on the first-person bone named by the
model (`Bip01 Spine2`), applied after the bone's own rotation. The amount
eases toward the wobble (`world::vats::wobble`, `00646910`) ×
`fNonAttackGunWobbleMult` (data 0.5) when not attacking, by
`fGunWobbleChaseDriftTime` a second, jumping up while attacking.
Implemented, tested (Euler round trip, chase); live only as small motion
between reports (65 of 18,705 sampled pixels in the gun's area changed
between reports 021/022 with the varmint rifle; the hold pose's own idle
moves too, so the sway isn't isolated). Assumed: the controller's
composition order (its first factor's register isn't recovered), the
clock (the game's first-person AnimData +0xd0), the stance passed to the
wobble (the player's own for both modes); the third-person body's sway
(the first half of `00962de0`) and the melee auto-aim chase there aren't
done.

## Blocking and power attacks (`world::melee`)

| Address | What | Status |
| --- | --- | --- |
| `0093e860` → `00894cc0`, `00894940`, `00894d60` | Aim control with a melee weapon or fists out: block (anim action 7, `BlockIdle` 0xaa: `<kind>blockidle.kf`, else `mtblockidle.kf` — the fall-back assumed); letting go or attacking ends it | implemented, live (machete held across the view, report 008) |
| `009b5a30`, `006463a0`, `009a6ae0`, `009a6a40` | A blocked hit: within the blocker's hit cone (`fCombatHitConeAngle` 35°, × 3 within 128 units) the threshold gains `fBlockSkillBase` 5 + skill × `fBlockSkillMult` 0.3 (data), Melee Weapons when both are armed and the blocker's weapon is melee, Unarmed when the attacker isn't armed or the blocker's is hand-to-hand, else 0; fists against guns or projectiles block nothing; before the perks | implemented, tested |
| `009b5a30` (`00407e00(1, 1)`, `fCounterAttackTimer`) | The hit flagged blocked: the player's `BlockHit` plays and the counter-attack timer starts | implemented |
| `00948310` | Attack press starts the normal attack; held past `fPowerAttackDelay` (0.3) a power attack follows the attack playing (none while over-encumbered); sneaking melee attacks are power attacks; directions by the movement keys (`AttackForward/Back/Left/RightPower`, plain `AttackPower` sneaking or legs crippled); unarmed perks' customs (entries 61–66) and the `Counter` | implemented, tested, live ("Power attack: attackforwardpower." holding W and the left button, machete) |
| `009b5170`, `00644520` | Power attack damage × `fDamagePowerAttackBonus` (data 2), not sneaking | implemented |

Assumed: `005a2030` (a flag at +0x14d that sends the hold to a second
normal attack) taken as off; the power attack's hit lands when it starts,
as the viewer's attacks do. Not done: the third-person body's block and
power attack animations (the body animation code belongs to another
batch), block hits' stagger perks, NPCs blocking (they have no block
animation played).

## Ammo swap (`world::ammo_swap`)

Control 18 ("Ammo Swap", the 2 key here): `0093e860` → `009462c0` loads
the next carried kind of the weapon's ammunition list
(`GameState::ammo_loaded`, saved as `ammoloaded`) and reloads, with the
animation when `fPlayerAmmoSwapTimer` (1 s) has passed and the weapon is
out (without it the clip is taken as filled at once: inferred from
`ReloadWeaponNV`'s 0). The HUD's `AmmoTypeLabel` (`007721c0`, placed by
`0076bfe0` at the AP rect + the count − 165) shows the loaded kind's
`QNAM` and moves the condition pieces 50 left while it says something
(`0077f890`: HUD +0x4c/+0x50/+0x54/+0x180; +0x50 is the meter, the others
taken as its label, background, arrows). Implemented, tested, live (9mm
pistol with 20 hollow points: 2 → "Ammo swap", the HUD reads "HP" and
13/7, CND moved left; report 007).

## Death (`world::player_death`)

`0093e860` @ `0093fedf`: dying or dead, the scope goes and a timer
starts at `fPlayerDeathReloadTime` (5); when it runs out the most recent
save loads (`008512f0`), else the main menu (`007d0a70`, not in this
viewer: a console line says so). With the setting at 0 the game asks
("Reloading the most recent save game", Reload / Main Menu; not shown
here). The death view: knocked or paralysed the control handler forces a
temporary third person (`0093fb6d`…`0093fbd3`); a dead player's knock
state is taken as set (not traced) — on reload it ends. Implemented,
tested, live (F5, `player.kill`: third person on the body, "You died",
5 s later the quick save loads back into first person; reports 012/013).
Not done: the player's death ragdoll and death camera movement, the
message box variant.

## Controls and other actions

`viewer::controls` reads `[Controls]` (this install's values as
defaults). Walking, the mouse always looks (pointer held in the window,
free in menus); Caps Lock toggles Always Run (+0x651, from
`bAlwaysRunByDefault`), Q toggles Auto Move (+0x652, ended by a movement
key) (`0093e860` @ `00940c84`). Existing: R reload / hold holster, Space
jump, F view, V V.A.T.S., T wait.

## Not done (not substituted)

- Weapon mods (the scope-from-mod weapons, mod zoom 0xe), the iron-sights
  depth of field (`009650a0`), the scope's world scissor.
- Third-person iron sights framing and scope entry; the third-person
  body's gun sway, block and power attack animations.
- The Info lines' height adjustment for wrapped names; activators' own
  activation text; class 10; terminals' lock line; doors' flag 0x100
  (`0057b460`).
- Hotkeys 1–8 (Pip-Boy batch), grab (physics batch).
- The pick's stand-ins (see "The crosshair's pick"): collision triangles
  for the drawn ones, controller capsules for people, no water.
- The player's death ragdoll; the death message box variant.

## Checks

Root: `cargo test --workspace` (1,097 pass; only `repo_hygiene` fails, on
the worktree's `.git` pointer file), clippy, fmt; viewer: build, test,
clippy, fmt. New tests: `world::melee` (3), `world::ammo_swap` (1),
`world::player_death` (1), `world::gun_wobble` (2), `ui` hud layout
(ammo label), viewer `scope` (1), `controls` (Ammo Swap, Use).

Live runs: release viewer, `--weapon` and `--run`/`--run-at` lines,
posted window messages (keys, mouse buttons, wheel) to the viewer's own
window, F12 reports (private, `nv-re\work\playeractions2-2026-10-06\out\
reports`).

Next action: compare in the original game: the hunting rifle's scope
(framing, tint, sway), a machete block and forward power attack, the
ammo swap label, the death-to-reload timing, the sneak meter's [CAUTION]
and [DETECTED], and whether `WG`/`VAL` show.
