# Fatigue, knock-outs, `ForceFlee` and combat groups

Batch `claude/fatigue-blockers` (2026-10-07), stacked on
`claude/melee-unarmed`. Read from FalloutNV.exe 1.4.0.525 (Ghidra
decompilation, the export's settings) and the Xbox 360 prototype's
symbols (`(Xbox PDB)`); the values are `FalloutNV.esm`'s. Code:
`world::fatigue`, `world::combat_groups`, `world::ai::flee::force`, the
hit in `world::scripting::Runner::blow_at`; viewer hooks in
`viewer/src/ai.rs`, `fighting.rs`, `combat.rs`, `scripts.rs`. Tests:
`crates/world/tests/fatigue.rs` on `testdata::fatigue`.

## Fatigue (actor value 22)

**Full fatigue** (`Actor::GetBaseActorValue` `008803a0`; the value's
flags 0x1841 at `0066f260`): a creature's is its record's (`ACBS` u16 at
4; × its level when the level is a multiple of the player's,
`TESCreature::GetFatigue`). A person's is the record's **plus** a
derived part (flag 0x40 adds them; `00643870`,
`AiFormulas::CalculateDerivedFatigue`):

| Who | Derived part | With the data |
| --- | --- | --- |
| the player | `fAVDFatigueBase` 90 + `fAVDFatigueEnduranceMult` 20 × Endurance + `fAVDFatigueLevelMult` 10 × (level − 1) | record 200: 390 at Endurance 5, level 1 |
| people | `fAVDNPCFatigueBase` 90 + `fAVDNPCFatigueEnduranceMult` 20 × Endurance | most records 50: 240 at Endurance 5; Trudy (E 6) 260 |
| people whose stats the game works out (`ACBS` 0x10) | the record's + 20 × the record's Endurance + `fAVDNPCFatigueLevelMult` 10 × level, whole, 0 when the record's is 0 (`006040d0`, `TESNPC::GetFormFatigueLeveled`) | Sunny Smiles (50, E 4, level 2): 50 + 150 = 200 |

No plugin sets these settings: the exe's defaults stand.
`GetFatiguePercentage` (`00893530`) is fatigue now ÷ the full value as a
whole number.

**Damage from hits** (`009b5170`, `009b5a30`, `009b73d0`, `0089d6f0`):

- Bare fists (no weapon) of someone not a creature do half their health
  damage as fatigue (`00646310`), none to someone already knocked down
  (actor vtable +0x230). `fHandFatigueDamageBase`/`Mult` are read by no
  code.
- With a weapon, its ammunition's kind-5 effect (`AMEF`) is applied to
  it: the bean bag's +250 (`AmmoEffectBeanBagFatigue`).
- When there is fatigue damage and health damage before the armour, the
  fatigue is scaled by the share of the health damage the armour let
  through. The bean bag's own damage effect (× 0.05) leaves the hit at
  the 20% floor (`fMinDamMultiplier`), so a bean bag does **50** fatigue
  a hit, not 250.
- A hit doing fatigue damage wears no armour.
- The player hit in V.A.T.S. takes it × `fVATSPlayerDamageMult` with the
  rest; an unarmed blow's last multiplier (power attack, sneak, part)
  applies to it too.
- It's taken after the health, only while fatigue is above
  `fMinimumFatigue` (−25): fists alone bring someone to about −25.

Scripts damage it like any value: the boxing gloves' and tape's and the
cattle prod's hit scripts (`DamageAV Fatigue 50/35/20`), the gas grenade
(10000), the Sleepytime dog treat (300), Orris's and the Legion
warehouse's scenes. (The weapons' hit scripts are enchantment effects,
which this engine doesn't run on hits yet.)

**Coming back** (`Actor::RestoreFatigue` `0088b5a0`, from the actors'
magic update `008c3c40` while the value is flagged damaged, `008b9960`,
or below 0): `fFatigueReturnBase` 1 + `fFatigueReturnMult` 0 × 10 × a
cached value (not traced; × 0) a second, only while damaged. The moment
it climbs from below 0 to 0 or more, all the damage goes at once.

## Knock-outs

The AI process's knock state (`MiddleHighProcess::UpdateKnockState`
`00920150`, process +0x13c; names at `0118c6d4`: 0 normal, 1 explode, 2
explode lead-in, 3 knocked out, 4 knock-out lead-in, 5 queued, 6 getting
up):

- Someone who can be knocked down (`008845a0`: alive; the record without
  `ACBS` flag 0x4000000; not in certain sit states) in state 0 goes to 4
  (the fall: a ragdoll) when fatigue is damaged below 0, or when essential
  and down (life state 6, `world::combat::hurt`); paralysed (actor value
  47 above 0) straight to 3.
- 4 becomes 3 when the ragdoll stops. In 3 they stay while any of the
  three holds; then the get-up animation is queued (5) and played (6),
  then 0.
- Knocked down (any state but 0) they can't be used (`005fa330`: the
  "is unconscious." message, `world::living::pickpocket`) and fists do
  them no fatigue. `GetKnockedState` (`005a08c0`) is 1 in states 3 and 4,
  else 0 (it said 2 for essential people down before this batch: wrong).

Here 4 ends at the next update and 6 at the one after (the fall and the
get-up are the viewer's); a fatigue knock-out says "X is down." and "X
gets up." (`Event::KnockedOut`, `Event::GotUp`), and the viewer drops
them limp like the essential down (`world::fatigue::lies_down`) and
stands them up after. Essential people keep `world::combat`'s timer
(`fEssentialDeathTime`); their knock state is implied from it.
`SetUnconscious` (life state 3) is separate and unchanged.

## `ForceFlee`

`ForceFlee [cell] [reference]` (`005d09e0`; Xbox
`Script::ForceFleeFunction`), on an actor; nothing on the player.

- **Fighting** (a combat controller): `CombatController::ForceFlee` is
  an empty function in this build (PC `008d0600`, the Xbox's shared
  empty body): nothing happens, except that a flee already running takes
  a new destination.
- **Otherwise** `Actor::InitiateFlee` (`00897de0`), unless restrained,
  unconscious or dead: the engine's flee package (type 0x16,
  `FleePackage`) from nobody, to the reference, else the cell; with
  neither, the process's flee distance becomes 20 if it was 0 or less
  and nobody is looked for to flee from (`009f1140` with none). It
  replaces what they were doing and makes them fleeing
  (`Actor::IsFleeing`): using them says they're fleeing.
- It doesn't end by itself with nobody to avoid
  (`FleePackage::ShouldShutDown` only ends one whose sole avoided
  reference is the player). `EvaluatePackage` and `ResetAI` end it (the
  package picked from their list becomes current,
  `Actor::EvaluatePackage`), as do a fight (the combat package) and
  death.

Here: `world::ai::flee::force` keeps it (`GameState::forced_flee`,
saved), `Event::Flees` prints it, and the viewer's AI runs them to the
reference given when it's in their space, else they stand
(`ai::forced_flee_frame`). The flee package's own avoiding and door
search are the AI's (the maintainer's), not done.

The base game's calls (`VHDKimballSpeechSCRIPT`, `VHDKimballMP01SCRIPT`,
`VHDRangerKimballGuard01SCRIPT` on Kimball at the Hoover Dam speech;
`VCFHPrivateStoneScript` in its `OnStartCombat` after `StopCombat`) pass
nothing. Quests whose scripts call it: VFreeformNellis, VHDKimballSpeech,
VMQ03a, VMQ03b, VMQ05, VMQHouse6, VMQNCRFail, VMQYesMan03, VMS31.

## Combat groups (`GetGroupMemberCount`, `GetGroupTargetCount`)

`005a4240` and `005a42b0`: the members and targets of the actor's combat
group (vtable +0x3f8), 0 out of a fight. How groups form
(`Actor::StartCombat`, `CombatManager::AddCombatant` / `AddGroupMember`,
`CombatGroup::MergeGroup`):

- Starting a fight alone makes a group of one, its target the group's.
- Taking a friend's side (the detection run `008ff350`: the friend seen,
  fighting a target the helper detects at 1 or more, the helper would
  help, `00992530`) joins the friend's group; someone already fighting in
  another group brings their whole group over. The check runs whether or
  not the helper is fighting, while the groups differ.
- The player's group (`PlayerCharacter` +0xd64) is made when someone
  starts fighting the player (`0093a690`), dropped when it has no targets
  (`009444d0`); helpers join it the same way; teammates move with the
  player when it's split (`00991f80`).

Here (`world::combat_groups`): groups are kept as "who joined whom"
(`GameState::combat_groups`, saved); a group's targets are its members'
(`GameState::combat`, `hit_targets`), and for the player's everyone
fighting the player. The viewer's detection (`fighting.rs`) joins or
merges on the help check; those who stop fighting leave.

The base game asks them only in dialogue conditions (63 `CTDA`s, mostly
"more than one member" on combat barks, some on the combat target):
GenericAdultCombat, GenericRobot, VDialogueBrotherhoodOfSteelFaction,
VFreeformFreeside, VFreeformGoodsprings, vDialogueKings,
vDialogueNCRMilitaryCombat, vDialogueSuperMutant,
vDialogueSuperMutantFirstGen; `GetGroupTargetCount`: Generic,
VFreeformGoodsprings.

## Not done

- The fall and get-up animations, the ragdoll's settling (4 → 3) and the
  explode states (1, 2: `PushActorAway`, explosions' and Super Slam's
  knockdowns still only print).
- Weapons' hit scripts (enchantments: the boxing gloves, the cattle
  prod) aren't run on hits, so they don't tire.
- `ForceAV` sets the base here (the game moves a modifier), so Orris's
  thugs' `ForceAV Fatigue -1` leaves no damage flag and doesn't knock
  them out.
- The flee package's behaviour (avoiding, doors) and what fleeing from
  nobody does.
- Combat group strategies, clusters, search; the group's own target list
  (here: its members').
- Power attacks' fatigue cost and blocking's (`fPowerAttackFatiguePenalty`,
  `fFatigueBlock…`): read by no code in this build.

## Live checks

Release viewer, `GSProspectorSaloonInterior` (2026-10-07; the scene script
and screenshots kept privately):

```powershell
viewer\target\release\nv-viewer.exe "<Data>" GSProspectorSaloonInterior `
  --at 231,-700,3460,0 --run "SunnyRef.DamageAV Fatigue 210" `
  --screenshot shot.png --wait 16
```

- Sunny Smiles (auto-calculated, full 200) taken to −10: "SunnyRef is
  down.", her ragdoll at rest after 4.6 s, lying on the floor beside
  Cheyenne (`sun1`); "SunnyRef gets up." and she greets the player at
  10.0 s (1 a second from −10, then all of it back).
- Bare fists with Unarmed 100 (`--walk --freeze-ai`, five swings): each
  "Hit 00104E85 ... for 6.5; fatigue 3.2" (half), her fatigue 196.8,
  193.5, 190.2, 187.0.
- E on her while down: "Sunny Smiles is unconscious." After
  `SunnyRef.ForceFlee`: "SunnyRef flees (ForceFlee)." and E gives
  "Sunny Smiles is fleeing." (the data's `sNoTalkFleeing`). She still
  greeted the player at 0.0 s in that run: whether fleeing blocks a
  greeting isn't traced.
- Joe Cobb, seated on his stool, logs the same down and up but his limp
  body stays propped on the stool and bar (the game may not knock
  seated actors down at all: `008845a0` tests the process's states 9 and
  0x11, not identified).

Next action: in the original game, knock Trudy out with
`TrudyRef.DamageAV Fatigue 270` and time her getting up (about 10 s
by the code), and punch a person to compare the fatigue per hit.
