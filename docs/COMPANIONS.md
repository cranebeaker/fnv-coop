# Companions: their wheel, trading with them, and teammates in the engine

A companion's "let's trade" lines call `OpenTeammateContainer` (19 calls in
19 scripts of the official master), which opens the container menu on
their things in its mode 3. Until 2026-10-06 nv-rs didn't carry it out, so
nothing opened.

## Read from `FalloutNV.exe` 1.4.0.525

| What | PC |
| --- | --- |
| `OpenTeammateContainer` (opcode 0x11C8) | `005d9430` → `00709470(…, 3)` |
| The container menu's mode (`+0x80`) | set by `0075b310` |
| Opening, by mode | `0075bc80` |
| Closing, by mode | `0075b750` |
| Moving a thing (the companion's room) | `0075dc80` |
| Their carry limit, what they carry | `008a0c20`, `00577250` |
| A companion's line from a topic | `0075ec60` |

- `OpenTeammateContainer [n]`: on a person or creature (vtable +0x100)
  that's the player's teammate (`00566950`), or on anyone when `n` isn't 0.
- Mode 3 opening (`0075bc80`): the title the companion's name (fitted as a
  container's, `0075e8c0`), `DRSTraderOpen`, the companion says their
  `FollowersTrade` topic's line (`0075ec60`), Take All (id 10) hidden. The
  key that presses Take All does nothing in modes 3 and 4 (`0075d430`).
  Their list is their own things through the usual filter (`0075c280`:
  `00719ef0` with `0075cfc0`).
- Giving them something (`0075dc80`, from the player's list): what they
  carry (`00577250`: their things' weight, at most what they can carry)
  plus its weight × how many, above what they can carry (`008a0c20`: Carry
  Weight, actor value 13, through perk entry 22 "Get Max Carry Weight",
  which only the player's perks touch) is refused: "<name> " +
  `sTeammateOverencumbered` ("can't carry any more.") on screen, and their
  `FollowersOverburdened` line.
- Closing (`0075b750` case 3): the companion sorts out what they wear
  (`00606540` for people, `005f9e00` for creatures; below),
  `DRSTraderClose`, the dialogue taken up again (`007640a0`).

## Here

`OpenTeammateContainer` raises `Event::TeammateContainer(who)`; the viewer
opens `ui::menus::container` with `mode` 3 (`game_menus::container`,
`menus::Menu::Teammate`): the name, the sounds, the topic lines
(`Event::Talk` without a conversation), the room check
(`world::items::carry`, `has_room`). `--open-menu teammate:REF` opens one
for testing.

Checks: `crates/world/tests/companions.rs` (who it opens on, a companion's
room), `ui::menus::container::tests::a_companions_things_have_no_take_all`.
Seen in the viewer in the Mojave Outpost barracks (`--open-menu
teammate:RoseofSharonCassidyREF`): "Cass" over her things (13 caps, an
empty whiskey bottle), no Take All; clicking the 9mm pistol gave it to her
(the player's weight 17 → 15).

## The companion wheel

Using a teammate brings up their wheel of orders, `CompanionWheelMenu`
(class 1075, `menus\companion_wheel_menu.xml`). Until 2026-10-06 nv-rs
talked to them instead.

| What | PC |
| --- | --- |
| Using a teammate | `005fa330`, `00607990` → `00709470(8, …)` |
| Whether they can take orders | `00754d90` |
| Opening, the switches | `00754de0` |
| The pointer on a slice | `00755760` → `007557d0` |
| The context line | `00755dc0` |
| A click | `007552e0` (buttons `007561b0`, `00756490`, `007563a0`, `00756960`, `00756930`, `007560c0`, `007568d0`, `007562b0`) |
| Keys | `007556b0` |
| Each frame | `00755480` |
| A topic said, its scripts run | `007573d0` (subtitle `00757690`), `007575b0` |
| Closing, talking after | `00755270`, `00754c60` → `00756980` |

- When: the player using a teammate after the pickpocketing (sneaking
  never picks a teammate's pockets) gets the wheel if they're alive and up
  (`00884480`) and wouldn't attack the player (`008b06d0`,
  `GetShouldAttack`), else nothing at all; they're never simply talked to.
- The switches are the companion's script variables (`008c1740`, -1 when
  their script lacks one): aggressive `FollowerSwitchAggressive` == 1,
  keep distance `IsFollowingLong` == 1, ranged `CombatStyleRanged` == 1,
  following `Waiting` == 0. Buttons 0, 2, 5 and 7 show their other picture
  (`user11`) when on (2 when waiting).
- The slices are `radial` tiles: the pointer is on one when its angle from
  the screen's centre (`00a216b0`: 0 up, clockwise) is within `user2` to
  `user3` and its distance within `user4` to `user5` (76 to 332). The
  file's slices run 30–150° on the right (0 Aggressive/Passive, 1 Use
  Stimpak, 2 Stay/Follow, 3 Talk To) and 210–330° on the left (4 Back Up,
  5 Near/Far, 6 Open Inventory, 7 Ranged/Melee).
- The pointer on a slice: `UIPipBoyScroll`, the highlight (`user10`) moved,
  a switch's button showing what a click would make it, the preview text
  (the exe's `sCWheel…` strings) and the context line: `HP  75/150` and
  `Stimpak  3` on two lines on the Stimpak, `Wg  40/150` on the
  inventory, the weapon's name on Ranged/Melee.
- A click (then `UIMenuOK`): a switch flips and the companion says the
  topic for its new state, its begin and end result scripts run on them at
  once: `FollowersTacticsCombatAggressive`/`…Passive` (then the menu sets
  `FollowerSwitchAggressive` itself), `FollowersWait`/`FollowersLetsGo`,
  `FollowersTacticsDistanceLong`/`…Default`,
  `FollowersTacticsCombatRanged`/`…Melee`. The game's lines do the work
  (Boone's `FollowersWait`: `set CraigBooneREF.Waiting to 1`, `evp`, …;
  `…CombatRanged`: `SetCombatStyle FollowersCombatStyleRanged`).
- Use Stimpak (`00756490`): needs a Stimpak (default object 0) in the
  player's things and the companion hurt (health below full, `00893590`)
  or a limb crippled (actor values 25 to 30 at 0). Hurt, the Stimpak's
  effects go on them; one Stimpak leaves the player; conditions 25 to 31
  are restored by 1000; they say their `Regenerating` line (no scripts).
- Talk To closes the wheel and starts the conversation; Back Up
  (`008a7760`: the default package 0x27, away from the player) closes it;
  Open Inventory closes it and says `FollowersTrade`, whose result script
  is `OpenTeammateContainer` (above); Exit (11) closes it. Enter clicks
  the chosen slice (Exit with none); the pad's B (code 10) is Exit.
- The line said shows on the wheel's subtitle (id 12) until its voice ends,
  or for its length × `fNoticeTextTimePerCharacter` (0.06 s) without a
  voice; the frame empties it after.

Here: `world::companions` (`wheel_allowed`, `switches`, `variable` /
`set_variable`, `say_topic`, `heal_with_stimpak`),
`world::living::pickpocket::Use::Wheel` / `Nothing`,
`ui::menus::companion_wheel` (the menu), the UI engine's `radial` tiles
(`ui::menu::in_slice`), the viewer's `game_menus::companion_wheel`
(`--open-menu wheel:REF`; Escape or Tab leave it). Checks:
`crates/world/tests/companions.rs::the_companion_wheel`,
`ui::menus::companion_wheel::tests`.

## The rest of the wheel and the container menu (2026-10-07)

- **The line said, picked twice.** The wheel's `007573d0` picks the
  topic's line for the companion and the player (`0061a720`, which wraps
  `TESTopic::GetMatchingInfo`), voices its first response and puts it on
  the subtitle; `007575b0` then picks the line **again** and runs that
  one's begin and end result scripts straight through the script runner
  (`005ac1e0`). Neither goes through the dialogue's own result-script call
  (`0061f170`), which is what marks a say-once line said for an actor
  (`00935920`, extra data 0x73): a line said from the wheel isn't marked
  said, its topics aren't added and the companion doesn't count as talked
  to. `world::companions::say_topic` picks twice the same way.
- **The pad's stick** (`00755480`, each frame while the wheel is the top
  menu, `00702450(1075)`, and the pad is what's in use): with the
  earlier frame's y or the present x of the left stick past 7849
  (XInput's deadzone; the game keeps the state before this frame's read
  at `011d8a54` and this frame's at `011d8a6c`, copied in `0070c4a0`),
  the earlier (x, y) gives an angle as the pointer's would (`00a216b0`),
  `00755c50` moves it out of the gaps above and below the wheel (0–30° →
  0.55, 150–180° → 2.6, 180–210° → 3.7, from 330° → 5.7 radians) and
  chooses the radial tile with `user2` ≤ angle < `user3`.
  `ui::menus::companion_wheel::CompanionWheelMenu::stick`; the viewer
  feeds Bevy's first gamepad's left stick.
- **Back Up** (`00756930` → `008a7760`, `Actor::InitiateBackUpPackage`
  (Xbox PDB)): the companion's default package 0x27, a `BackUpPackage`
  aimed at the player (`BackUpPackage::InitializeBackUpPackage`, its
  point `CalculateBackUpPointExterior` / `…Interior`). Here the hook only:
  `Event::BackUp(who)` (`world::companions::back_up`); packages are the
  AI's.
- **The container menu's subtitle** (`CM_Subtitle`, found by name):
  emptied when the menu opens (`0075b310`); the companion's barks in mode 3
  (`FollowersTrade` on opening, `FollowersOverburdened` when refused;
  `0075ec60`, picked as the wheel picks, no scripts run) go there
  (`0075eea0`) until the voice's length, or the line's length ×
  `fNoticeTextTimePerCharacter` without a voice, has passed (`+0xfc`); the
  menu's frame (`0075eac0`) empties it after. `ui::menus::container::
  ContainerMenu::say` / `update`; the viewer plays the voice as the
  wheel does.

## What a companion wears after trading

The container menu closing in mode 3 (`0075b750`) has a person sort out
their clothes (`00606540` → `TESNPC::InitDefaultWorn` (Xbox PDB),
`006047c0`; a creature `TESCreature::InitDefaultWorn`, `005f9e00`, only
picks a weapon):

- For each body slot 0 to 19 in turn, `InventoryChanges::GetBestArmor`
  (`004c8220`, asked to keep what's locked on): a piece worn in the slot
  that `EquipItem` put on with its no-unequip flag (the worn entry's extra
  data 0x3e: `005d0060` → `0041ab70`) stays; otherwise, of the armour
  they hold covering the slot (the base container's entries first, then
  the changes), the highest truncated DR + DT wins, the first on a tie.
  The game multiplies the DR by its condition factor (`00646d40`) but
  passes the piece's health itself rather than its share of full health,
  so the factor is 1 for anything above 0.5 health; a piece with no health
  left isn't picked. (DR is kept × 100 in `DNAM`: `004be080` divides.)
- The pick is put on (`Actor::EquipItem`, vtable +0x184) unless the
  upper-body piece already put on covers this slot, it's worn already
  (`00575400`), or — in any slot but the upper body's — it covers the upper
  body too.
- Then, when asked, the best weapon (`InventoryChanges::GetBestWeapon`,
  `004c7400`, scored by the damage per second `00645380` works out:
  below).

Here: `world::companions::best_armour` and `wear_best_armour`;
`GameState::equip_locked` (saved) records `EquipItem`'s flag. The viewer's
container menu calls `world::companions::sort_out_gear` on closing in mode
3: the armour and the weapon below.

## What a companion holds after trading (2026-10-07)

Read from `FalloutNV.exe` 1.4.0.525 and the Xbox 360 prototype's
decompile (`x360_weapons.c`; names from its PDB).

| What | PC | Xbox name |
| --- | --- | --- |
| The weapon part of sorting out what they wear | `006047c0` (person), `005f9e00` (creature) | `TESNPC::InitDefaultWorn`, `TESCreature::InitDefaultWorn` |
| Asked from the container menu's closing | `0075b750` → `00606540` / `005f9e00` | `ContainerMenu::Close`, `TESNPC::InitParts` |
| The best weapon | `004c7400` | `InventoryChanges::GetBestWeapon` |
| Its ammunition | `004c7300` | `InventoryChanges::GetBestAmmoForWeapon` |
| May they use it | `008bc9d0` | `Actor::CanUseWeapon` |
| The rating | `00646060` → `00645380` | `CombatFormulas::CalcCombatWeaponDPS`, `…CalcWeaponDamagePerSecond` |
| The shots a second with an entry | `00645dc0` | `CombatFormulas::GetWeaponShotsPerSecond` |
| An animation group's action times | `005f3a20` (`TESAnimGroup` +0x18) | `TESAnimGroup::GetTime` |

- **When.** Closing the container menu on a companion (mode 3): a person
  through `00606540` asks for the armour and the weapon — the weapon
  unless the package they run now has "Weapons Unequipped" (`PKDT`
  flags 0x200000, `00441b00`), and never for the player (vtable +0x22c);
  a creature (`005f9e00`) only the weapon. What they wear has been taken
  off first (`004bfe50`; `004c0cf0` leaves what a script locked on), so
  the weapon in hand goes unless locked there; the best one is put in
  hand (`0088c830`) unless it's there already. With nothing to pick they
  hold nothing. (`006047c0` has 16 callers besides — packages, resets —
  not hooked here.)
- **The best weapon** (`004c7400`, asked for any kind, 6, keeping a
  locked one): a weapon in hand locked there (`EquipItem`'s no-unequip
  flag, extra data 0x3e) is kept. Otherwise every weapon they carry —
  the base container's entries first, in its order, then the inventory
  changes' — that is of the kind asked, that they may use, and that has
  ammunition with it or needs none, is rated; the highest rating above 0
  wins, the first on a tie. Nothing while the actor is planting an
  explosive (process +0x444, Xbox `bPlantedExplosive`; not modelled).
  - May they use it (`008bc9d0`, asked "playable or not"): never a
    "player only" weapon (`DNAM` flags2 0x1); a person any other; a
    **creature only a weapon in its record's weapon list** (`CREA`
    `LNAM`, a form list, the base's +0x158 set by `005f8bd0`; ED-E's is
    `EmbeddedWeapons`, holding both of its zap guns).
  - Ammunition (`004c7300`): the first of the weapon's kinds (a form
    list's, or its one) they carry; whether it takes any.
  - The rating is the combat one (`00646060`): the damage a second below
    without perks, ÷ 10 for grenades, 0 for mines, 0 for a weapon "not
    used in normal combat" (flags2 0x40), and ÷ 10000 for what their
    combat style rules out (`CSSD` u32 at 40: 1 melee only puts guns down,
    2 ranged only puts melee down). **The wheel's Ranged/Melee switch
    works through this**: its lines run `SetCombatStyle
    FollowersCombatStyleRanged` (restriction 2) or `…Melee` (1).
  - What the rating is asked with: the ammunition found, the inventory
    entry when the weapon has one (`ExtraContainerChanges`: here a weapon
    worn, worn down, modded, or held in another number than the record
    gives), and its condition — from its extra data, a weapon with no
    health left passed over; one with no extra data gets the condition
    the last one had (1 at first), as the game's loop leaves it.
- **The player's animations rate everyone's guns.** With an entry the
  figure's shots a second come from `00645dc0`, which reads the
  **player's** third-person animations (`PlayerCharacter::GetAnimation(0)`)
  whoever holds the gun: for a semi-automatic weapon, 1 ÷ the time of the
  `a:` key of the attack group it plays (`DNAM` byte 41; 0xff
  `AttackRight`), looked up for its animation kind as any group is
  (`00495740`; 0 when the lookup lands on another group); else (automatic,
  or no animations) the fire rate (`DNAM` 64) × the attack multiplier
  (`DNAM` 60, + a fitted fire-rate mod); either × the weapon's speed
  (`DNAM` 4) × the attack multiplier again. The editor's "attack shots a
  second" (`DNAM` 88) is the same sum off the same files (the 9mm pistol:
  1.25 ÷ `1hpattackright.kf`'s `a:R` at 0.4 s = 3.125; the caravan shotgun
  1.5 ÷ `2hrattackleft.kf`'s `a:L` at 0.4667 = 3.214), so with the
  animations loaded the vanilla weapons rate as their records say;
  without them a semi-automatic is rated off its fire rate instead.
- **An animation group's action times** (`005f3a20`): the sequence's
  text keys in order, line by line; lines starting `m:`, `BlendIn:`,
  `BlendOut:`, `Blend:`, `Decal:`, `Sound:`, `Enum:` (or a key `prn:`)
  are other things; any other line that starts with the group kind's next
  action name (`01199a50`: an attack's `Start`, `Hit`, `Eject`, `a:`,
  `End`; a reload's `Start`, `End`) sets that action's time; an attack's
  third may skip `Eject` for `a:`. Unset actions stay 0.

## The damage per second

`CombatFormulas::CalcWeaponDamagePerSecond` (`00645380`), with what its
five callers pass (each passes 0 for its arguments 6, 7 and 9–12: a forced
reload, the first jam chance, a critical chance without an actor, a
critical damage multiplier, × 2, × 1.25; and −1, the holder's own skill):

- 0 for a broken weapon (condition 0).
- Damage: `DATA` damage × `fDamageWeaponMult` (+ a fitted damage mod's
  value), × 1.3 with a split beam fitted (New Vegas only: not in the
  Xbox prototype), through the ammunition's damage effects (multiplying
  ones first, `0059a030`), + the projectiles (`00525b20`) × the
  projectile's explosion damage; grenades and mines (animation 10, 11) the
  explosion × `fDamageWeaponMult` instead and the jam index 10.
- × (`fDamageSkillBase` + `fDamageSkillMult` × the holder's skill ÷ 100),
  then, when asked (the cards ask, the combat rating doesn't), the
  holder's perks' "Calculate Weapon Damage" (entry point 0: owner, weapon,
  target the holder).
- Rounds: the clip (with a clip mod) for guns other than grenades, mines
  and throws (animation 13), else 1. Time: rounds ÷ (shots a second +
  rounds × `fWeaponConditionJam(n)` × `DNAM` 96 jam time, n the condition
  × 10 rounded half up; the master sets the first four to 0), + the reload
  (`DNAM` 92) for a clip of `iMinClipSizeToAddReloadDelay` (2) or fewer;
  for others than the player with a gun, the semi-automatic delay's
  average beyond the attack × rounds, and automatics (or a short burst,
  flags2 0x200) × (`fAutomaticWeaponBurstFireTime` + `…CooldownTime`) ÷
  the cooldown.
- Criticals: Critical Chance (actor value 14) × the weapon's `CRDT`
  multiplier, within 0–100, ÷ 100 (÷ the shots a second for automatics)
  × `CRDT` damage.
- The damage × the condition's multiplier (`00646d00`: 1 above 0.75, else
  1 − (0.75 − c) × 0.67); the figure (criticals + damage) × rounds ÷ time.
- Fists: `fUnarmedDamageMult` × Unarmed Damage + `fDamageSkillMult` ×
  Unarmed ÷ 100 over `fUnarmedNPCDPSMult` (1.57); a creature its own
  attack damage over `fUnarmedCreatureDPSMult`.

Who asks: the combat controller (`009993c0` via `00646060`: no perks, no
entry — `world::npc_combat`), `GetBestWeapon` (above), the item cards
(below), the recipe menu's card (`00728da0`: the player, condition 1,
perks, no entry, no ammunition) and the player's cached figure
(`008ac9d0`, `Actor::CalculateCachedWeaponDPS`).

## The numbers on the item cards

`Interface::PopulateItemStatsDisplay` (`00707e30`; the Pip-Boy's ITEMS
card, and the container's and barter's) for a weapon:

- **DPS** (mask 0x2): `00645380` for the player, at the weapon's
  condition, with their perks and its inventory entry (so their mods
  count and the shots a second come off their animations) and, for the
  weapon in their hands, the ammunition it's loaded with (process +0x14c;
  else the weapon's own); "%d" of it rounded half up (`004bd510`).
- **Effects** (0x40): an aid item's or ingredient's effects; a weapon's
  or apparel's enchantment's (`EITM`; not one flagged "hide effect",
  `ENCH` `ENIT` flags 0x04); ammunition's own text (`00503a70`); a weapon
  mod's description (`IMOD` `DESC`). Empty text hides the card, and a
  weapon with a mod fitted never shows it (its mod cards take the place).
- **An effect list's text** (`EffectItemList::BuildMenuString`,
  `00406620`), as the player sees it: of the effects whose conditions pass
  on the player (not the engine's "Usage Monitor Effect", form 0x14F),
  those whose magic effect shows its name alone (`MGEF` flag 0x2000) are
  written first, by name, one a line; the rest are summed by actor value
  (`StatsMenu::StatusDataList`, `007e0b20`: archetypes with an actor
  value, the table at `01183328`; the first one's duration kept, its
  magnitude negative when the effect is detrimental, flag 4, its duration
  none when it recovers, flag 2; "no magnitude" 0x100 and "no duration"
  0x80 give 0) and written **latest-added first**, ", " between, each
  `ABBR %+i` with the `AVIF` abbreviation (`ANAM`: "HP", "PER", "Rads")
  and a duration `(%d%c)` or `(%.1f%c)`: seconds, minutes from 60, hours
  from 3600. An amount: turned for the values counted the other way (the
  actor value flag 0x200: rads, dehydration, hunger, sleep); for a
  **medicine** (`ENIT` flag 4) × (`fMagicMedicineSkillBase` +
  `fMagicMedicineSkillMult` × Medicine ÷ 100); for a **food** (flag 2, and
  not every effect hostile or flagged 0x400000) × the same with Survival,
  except on rads; Health's through "Modify Recovered Health" (entry point
  12) in both; positive rads × (1 − the player's Rad Resistance, at most
  `fPlayerMaxResistance` 85, ÷ 100) rounded down, a food's through
  "Modify Radiation Consumed" (44); each truncated, and 0 written as 1.
  (The master's Stimpak at Medicine 15: "HP +39"; Nuka-Cola: "Rads +3,
  HP +2(25s)".)
- **Ammunition's text** (`00503a70`): each `RCIL` effect as "%s %s %.2f"
  — `sAmmoEffectDAM` "DAM", `…DR` "Target DR", `…DT` "Target DT",
  `…Spread` "Gun Spread", `…Condition` "Gun CND", `…Fatigue` "Target
  Fatigue" by its type, "+", "x" or "-" by its operation, the value — one
  a line ("DAM x 1.20" for buckshot).

Here: `world::dps` (`weapon_dps`, `shots_per_second`,
`combat_weapon_dps`, `best_ammo`, `can_use_weapon`, `best_weapon`),
`world::animation::action_times`, `world::companions::hold_best_weapon`
and `sort_out_gear`, `world::item_card` (`card_dps`, `card_effects`,
`effects_text`, `ammo_effects_text`); the Pip-Boy fills its ITEMS card's
DPS and effects through them (`ui::pipboy::gather::gather_with`, given
the player's set by the viewer's `AnimLibrary::player_library`).
`world::npc_combat`'s controller figure is the same translation. Checks:
`crates/world/tests/companion_weapons.rs` (on `testdata::companion_gear`,
the master's values), `ui::pipboy::gather::tests::the_item_cards_dps_and_effects`.

Seen in the viewer (2026-10-07, Mojave Outpost barracks, `--weapon
Weap10mmSubmachineGun`): with Cass a teammate, her things opened
(`OpenTeammateContainer 1`), the 50 rounds and the submachine gun clicked
over and the menu closed, she put on her hat and outfit and took the gun
(`00135F19 holds 00004321`); her list then holds the gun and rounds. The
ITEMS card shows the 9mm pistol at "DPS 31" (Guns 15: (0.8 + 16 × 0.575)
× 3.125 off `1hpattackright.kf`'s `a:R`); before the player's set was
taken from the body's build it showed 16, the fire rate's figure.

The recipe menu's card (`00728da0`, on `claude/repair-mod-followups`)
would take: its DPS from `world::dps::weapon_dps` with the player,
condition 1, perks on, no entry and no ammunition (no animations needed),
and its effects card from `world::item_card::effects_text` on the
product's effects (an aid item's list, else a weapon's or apparel's
enchantment unless hidden) — once that branch and this one meet; nothing
else on it changes.

## Teammates in the engine

The player's teammate is the actor's byte +0x18d (`00566950`; set by
`SetPlayerTeammate`, `008bca90`, which keeps the player's teammate list
and count). Where the engine looks at it, read from its 53 callers and
the Xbox 360 build's named functions:

| Rule | PC | Here |
| --- | --- | --- |
| Coming along when the player is put somewhere | `0093c200` → `00973de0`; fast travel `0093cdf0` → `00973ee0` | `world::companions::come_along`, viewer `companions.rs` |
| Following (Follow or Accompany package aimed at the player, not waiting) | `009549a0`, `PlayerCharacter::IsActorFollowingPlayer` | `follows_player` |
| Moved with their own followers, skipping a teammate with `Waiting` 1 | `008ad1c0`, `Actor::MoveActorAndFollowers` | in `come_along` |
| Nerve: the player's Charisma on their damage, DR and DT | `00644ce0`, `009b5170`, `009b5a30` | `nerve`, `combat` |
| Essential outside Hardcore | `0087f3d0` | `more_functions::is_essential` |
| Time down stops while the player fights | `00888b50` | `down_time_runs`, in `combat::advance_down` |
| Ammunition used up (playable weapons) | `Actor::ShouldUseAmmo`, `Actor::UseAmmo` | `npc_combat::should_use_ammo`, `npc_combat::fired` (an NPC's shot) |
| No fall damage | `008a62b0` | `combat::land` |
| Sneaking with the player | `008a0d10` | `hidden_with_player`, `Facts::detection_inputs` |
| Their damage counts toward the player's kill experience; killing them gives none unless the player did | `009134c0`, `0089d900` | `experience` |
| Their killing an innocent counts as the player's murder | `0089d900` | `crime::murder` |
| The player's perks they share (`AddPerk`'s teammate flag) | `005e58f0` | `perks` |
| Using them: the wheel; sneaking doesn't pick their pockets | `005fa330`, `00607990` | `living::pickpocket` |
| `OpenTeammateContainer` | `005d9430` | above |

- **Coming along.** Every positioning of the player (`0093c200`: through
  a load door's arrival, a script's `MoveTo`, `coc`; reached from
  `0093bea0`, the player's queued "position player" request) ends in
  `00973de0`: each actor in the high-process list, alive, that is the
  player's teammate, or follows the player and is more than 350 units away
  (another place counts), is moved to the player (`008ad910` →
  `008ad1c0`). Fast travel (`0093cdf0`) first does the same for those
  following, escorting or teammates to the marker (`00973ee0`,
  `ProcessLists::MoveActorsToReference`), then positions the player.
  `008ad1c0` moves the actor and those following them, skipping the dead,
  the disabled and a teammate whose `Waiting` is 1; each stands up from
  furniture and goes where the player stands, facing their way (the game
  first looks for a spot on the navmesh behind the destination, spaced by
  each one's bound radius, `006e7e70`; that search is the pathing's and
  isn't done), then looks at its packages again. Following is the
  package's: a Follow (1) or Accompany (7) package whose target is the
  player (`009549a0`; the Xbox `IsActorFollowingPlayer` also requires
  `!Actor::IsWaiting`). The viewer runs `come_along` on a change of place
  or when `GameState::player_placed` is set (`script_functions::
  player_moved`), with the people drawn the frame before as the high
  list; those moved into the new place come on screen as anyone moved
  there does.
- **Nerve.** A teammate's weapon or fists damage × (1 + 0.05 × the
  player's Charisma, kept within 1 to 10: `0066ef50(8)` on the player,
  Charisma's flags 0x8009 at `0066f260`) in `00644ce0`, and once more for a
  melee, unarmed or creature attack as the hit is made (`009b5170`; a
  shot's, `009b5650`, has it once). When hit, their DR share (min(DR,
  100) / 100) and DT × the same, before the ammunition's effects and the
  `fMaxArmorRating` cap (`009b5a30`).
- **Knocked out.** A teammate is essential outside Hardcore
  (`more_functions::is_essential`), so at 0 health they go down instead of
  dying (`Actor::Kill`; the essential knock-down, `combat::hurt` /
  `advance_down`); in Hardcore they die. Down
  (life state 6), their `fEssentialDeathTime` only runs while the player
  isn't in combat (`00888b50`, `PlayerCharacter::IsPlayerCharacterInCombat`,
  +0xdf0): a companion knocked out in a fight gets up after it. Getting
  up restores health and conditions (`008a1800`, `008a0960`) and, for a
  teammate, queues hint 11 (`008d5cb0`, not done).
- **Ammunition.** `Actor::ShouldUseAmmo`: the player always; anyone whose
  weapon has "NPCs use ammo" (`DNAM` u32 at 56, `iFlagsEx` 0x02); the
  player's teammate with any playable weapon (`DNAM` u8 at 12,
  `cFlags` 0x80 clear). Others never run out. `Actor::UseAmmo` takes the
  weapon's ammunition per shot (`cAmmoPerShot`, `DNAM` u8 at 14) from the
  clip and, when it should, from their things; with none left the weapon
  is put away for the combat AI to choose again (both in
  `world::npc_combat::fired`), and a teammate barks
  (`FollowerBarks::TriggerFollowerBark` type 7, not done).
- **Falls** (`008a62b0`): no damage for the player's teammate; none either
  for a person other than the player in a worldspace flagged "no NPC fall
  damage" (`WRLD` `DATA` 0x40, `00586320`).
- **Sneaking with the player** (`008a0d10`): while the player sneaks
  (`004997b0`) out of combat, a teammate out of combat (+0x104) isn't
  detected at all (−100); a teammate sneaks with the player's Sneak when
  it's higher; and counts as invisible when the player is (actor values
  48, 49 — not modelled by the detection here).
- **Experience** (already here): their damage counts toward the player's
  kill share (`009134c0`); a teammate's own death gives experience only
  when the player killed them (`0089d900`).
- Also read, the AI's: the combat groups (a teammate joins the player's
  new group, `CombatManager::SplitPlayerCombatGroup`, `00991f80`), the
  combat style switches (`0097ae00`, `0097b5a0`, `0097c310`, `0097c8e0`:
  `CombatStyleMelee`), package target lists treating teammates apart
  (`008e2b00`, `00907d90`, `0091b700`), `PlayerCharacter::
  IsPlayerDetectedByNonTeammates` (whether anyone but a teammate detects
  the player); re-equipping a weapon after their things change
  (`005750a0`, `00892520`: for a teammate while a menu is up `0088c830`,
  else `0088c650`); damage hints
  for teammates (`00898650`: below half and a quarter health, `008d5cb0`
  4 and 5; `00891360` a broken weapon, 8).
- **The Followers faction** (`FollowerFaction`, `TeammateFaction`) is the
  scripts' (hiring adds it with `SetPlayerTeammate 1`); the engine reads
  only the teammate flag.

Checks: `crates/world/tests/companions.rs` (`nerve`,
`a_companion_picks_their_armour`, `teammates_come_along`,
`a_teammates_shots_use_ammunition`, `teammates_take_no_fall_damage`,
`a_knocked_out_teammate_waits_for_the_fight_to_end`,
`a_teammate_sneaks_with_the_player`, `the_companion_wheel`),
`ui::menus::companion_wheel::tests::choosing_with_the_stick`,
`ui::menus::container::tests::a_companions_line_on_the_subtitle`.

Seen in the viewer (2026-10-07, Mojave Outpost barracks): with Cass made
a teammate and given reinforced leather armour, opening and closing her
things (`--open-menu teammate:RoseofSharonCassidyREF`, space) logs her
putting on her hat and the armour (`00135F19 puts on 0015EF8B`,
`… 001264FE`); with her hired (`bCassHired`, `SetPlayerTeammate 1`,
`Waiting` 0), `player.moveto MojaveOutpostMapMarker` brings her out with
the player (`00135F19 comes along with the player`, then `comes into
view` outside, standing on the player's spot).

## Not done

- `InitDefaultWorn`'s other callers (packages, resets: 16 of them), and
  `GetBestWeapon`'s explosive-planting check (process +0x444).
- The armour and weapon picked are drawn on them ([NPC_GEAR.md](NPC_GEAR.md),
  people redrawn when what they wear or hold changes), but when a worn
  piece is taken away the redraw falls back to the record's clothes, not
  to the best the companion carries (`world::outfit` doesn't yet re-pick
  through `InitDefaultWorn`).
- The navmesh spot behind the player that those coming along are put on
  (`006e7e70`); here they're put on the player's own spot, as the game does
  when it finds none.
- Followers of a follower coming along (`ProcessLists::
  BuildFollowerListRecursive`).
- The Back Up package itself (the AI's), the follower barks and the
  hints.
- With `NV_GUESSES=1`, a teammate with nothing to do still follows by the
  guessed package (`world::ai::teammate_follows`, Dead Money
  contributor's); the traced rules here move people only when the player
  is put somewhere, not while walking.
- (Done since: `DropMe` — the companions' faction-outfit scripts drop what
  they're given, `OnAdd CraigBooneREF` … `DropMe`; see
  `docs/ITEM_SCRIPTS.md`.)
