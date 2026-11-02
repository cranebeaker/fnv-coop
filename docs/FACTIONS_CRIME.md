# Factions, crime, karma, reputation and disguises (M11)

What the game does, read from `FalloutNV.exe` 1.4.0.525 (PC addresses) and
the Xbox 360 prototype's symbols (names marked Xbox PDB), what nv-rs does,
and what's missing. Code: `crates/world/src/{reputation,crime,factions,
terminal,magic}.rs`, `world::living::{trespass,pickpocket}`; tests:
`crates/world/tests/factions.rs` on `testdata::factions` (the data's own
values: the NCR's and the Legion's reputations, factions and relations,
the NCR trooper armour's script, the disguise quest, spell and effect).

## Reputation (`REPU`, `TESReputation`)

- Fame (type 1) and infamy (type 0) per reputation, both from 0; `DATA` the
  most (NCR 80, Legion 100). Levels from value / most: 1 from 0.15, 2 from
  0.5, 3 at the most (`00616950`, `fReputationThreshold…`). Titles by
  infamy × 4 + fame (`sRepTitlePos<F>Neg<I>`), `GetReputationThreshold`
  per axis (`00616a90`).
- `AddReputation` / `RemoveReputation` move by `fReputationBump…` (sizes
  1–5: 1, 2, 4, 7, 12; others nothing), `…Exact` by the amount. Adding
  stops at the most and has no floor; removing stops at 0 and has no
  ceiling (`00615730`, `00615a00`, `00615c90`, `00615fa0`). `SetReputation`
  sets, nothing shown. **Done.**
- Each change: the HUD notice "<name>\n<sRepPositiveGain / NegativeGain /
  PositiveLoss / NegativeLoss>" (the data's "Fame Gained!", "Infamy
  Gained!", "Fame Reduced", "Infamy Reduced"), by command, not by sign;
  its picture (`sRep…Icon`) comes from `world::message_icon`.
  **Done (text).**
- The changed axis's level changes: a message box (`006155f0`,
  `DisplayReputationTitleChange`, Xbox PDB, via `00703f10`): title the
  reputation's name, text "<title>\n<description>" (`sRepTitle…Desc`, the
  exe's texts but `PosTwoNegThree`, which the data sets), the title's
  picture (`sRepTitle…Icon`), the sound `UIRepGood` when fame rose or
  infamy fell, else `UIRepBad`, one button `sOk`. With a menu open the game
  keeps the latest reputation and shows it later (`006159e0`); the viewer's
  box queue does the waiting. `Event::Popup` → `game_menus::message`.
  **Done.**
- What changes reputations in the engine: only crimes' infamy (below).
  Hostility from reputation (hit squads, `SetEnemy NCRFactionNV
  PlayerFaction` at "Hated" and the like) is the game's scripts
  (`VEFR02NCRBad2QuestSCRIPT`, `VFactionSquadMasterScript`…).

## Karma (actor value 23)

- `RewardKarma` (`0094fd30`): clamped to `iKarmaMin`/`iKarmaMax` (±1000),
  the notice `sKarmaMinor/Major Gained/Lost` (major past
  `iKarmaChangeThreshold` 250). Bands (`0047e040`): very evil ≤ −750, evil
  ≤ −250, good ≥ 250, very good ≥ 750; karmic titles `sKarmicTitle…` by
  band and level (`0047e0e0`). **Done** (the `UIKarmaUp`/`Down` sounds
  aren't played).
- Kills by the player (`0089d900`, `Actor::Kill`) of someone in a faction
  that tracks crime (`DATA` 0x100): by the victim's own karma band, good
  −50, very good −100, evil +100 (the data's), very evil +2, others 0.
  **Done**; the gate on the victim's process flag 4 (`008d7ec0`) isn't
  traced.
- Taking what someone not evil owns (a faction flagged evil `DATA` 0x02,
  or a person whose record karma is evil): `fKarmaModStealing` (−5) for a
  theft (not from a person: `008bfa40`), pickpocketing (`0075e0b0`),
  opening their terminal unless the record is flagged unlocked
  (`BGSTerminal::Activate` `00501310`), picking up their note (only the
  karma, no crime: `BGSNote::Activate` `005e9360`). **Done.** A note taken
  from an owned container (`004c37d0`) still goes through the theft.
- Conditions: `GetIsAlignment`, `GetAV Karma`, the Pip-Boy's karma.

## Crime

- **Ownership** (`00567790`), **may take** (`005785e0`) and
  **trespassing** (`00546da0`): as before (integration).
- **Theft** (`Actor::StealAlarm`, `008bfa40`): the victim (the person
  stolen from, else the owner or a loaded member of the owning faction)
  must not ignore crime and must detect the player; then its
  crime-tracking factions count a minor crime with `fReputationMinor
  CrimeNeg` (2) infamy. The player's day record (`+0x134/+0x138`, only the
  day of the month compared): first crime notes it, another on that day
  makes the victim attack, one on another day clears it. Warnings left
  (`iStealWarnings`): the victim comes to warn (a package of type 0x22 the
  AI follows; `fWarningTimer` 5 s), else the alarm (the AI's). Thefts
  don't count in the player's own minor crimes. **Done** but the warning
  and alarm packages.
- **Pickpocketing** (`PickpocketAlarm`, `008c00e0`): the same pattern
  (`world::living::pickpocket`).
- **Assault** (`AttackAlarm`, `008c0460`, after the friend-hit allowances
  of `008987f0`): none against someone who ignores crime, or when both
  are in special-combat factions (`DATA` 0x04). The crime goes to everyone
  who detects the player, the victim too
  (`ProcessLists::SendCrimetoHighList`); each who doesn't ignore crime
  makes their crime-tracking factions the victim is in hold the player as
  an enemy (`SetFactionsThatCareAboutCrime`, `008b8360`; faction runtime
  flag 0x10, `GetPCEnemyofFaction`). Anyone there: +1 major crime for the
  player and for the victim's factions, no infamy. **Done.**
- **Murder** (`MurderAlarm`, `008c09e0`): the victim's own crime-tracking
  factions hold the player as an enemy whether or not anyone saw it;
  witnesses as for an assault; seen: +1 major crime and
  `fReputationMajorCrimeNeg` (30) infamy. The player becomes a murderer
  (`IsPCAMurderer`) unless every faction the victim is in is flagged evil
  (`005678a0`). **Done**; which deaths count (the victim's `+0x31` flag,
  `008b01c0`) is approximated as "a person who wasn't fighting the
  killer"; creatures and owned victims get the assault alarm in the game,
  nothing here.
- **Trespass alarm** (`TrespassAlarm`, `008c0ec0`): everyone who detects
  the player and cares (in the owner's or the alarmer's factions) turns on
  the player; the first's factions a minor crime with infamy, the player a
  minor crime. Raised by the trespass package, `SendTrespassAlarm` and now
  **hacking an owned terminal** (`00766b80`). Not wired: activating an
  owned activator (`TESObjectACTI::Activate` `005113f0`; its gating isn't
  clear enough: it would fire on casino slot machines).
- **Enemy factions** (`008b87a0`): a faction flagged for the player's
  crimes makes the reaction "enemy" so far, but the actor's other factions
  still count, so a friend or ally relation wins (the disguises rely on
  it). **Done.** Crimes cool down after `iCoolDownTimerSetting` (3) days
  or once everyone who knew is dead (`009eb8a0`); the flags stay until a
  script clears them. The crime list itself isn't kept here.
- No crime gold or bounties: none of the alarms above reads the
  `iCrimeGold…` / `fCrimeGold…` settings (left over from Oblivion).

## Disguises (faction armour)

All data: there's no disguise code in the exe. The armour's object script
(`NCRFactionOutfitWarningScript` and the Legion, Brotherhood, Khans,
Powder Gangers and White Glove ones) on `OnEquip Player` stores and clears
the five reputations, `SetAlly PlayerFaction Armor…Faction 1 1` (members
are in those factions, so friend beats the enemy relation), puts the
player in `Armor…FactionEnemy` (their enemies' enemy), and starts
`DisguiseFactionPulseQuest`; `OnUnequip` (when no `FactionGearNV…` list
item is worn) undoes it and restores the reputations. The quest's script
(every 1 s) casts `DisguiseFactionPulseActorEffect` on the player: a touch
effect of area 25 for 5 s whose script takes sniffers (`V…SnifferFaction`)
out of the armour faction at the start and puts them back at the end; the
sniffers' own scripts sound the alarm when they start a fight with the
player.

What the engine needs, now **done**: `GetEquipped` of a form list
(`0059da90`); a script's cast reaching the area of touch-range effects,
`fMagicUnitsPerFoot` (22) × area = 550 units (`00818ce0`, `00816f10`):
the loaded actors and the player, not the caster, not ghosts, each with a
line of sight from the target unless the spell's `SPIT` flag 0x10 says
area effects ignore it (`008190d0`: to their position raised by half
their height; through the viewer's `Sight`; headless it isn't tested); each effect a cast adds goes
through `MagicTarget::CheckAddEffect` (Xbox PDB, `00823210`,
`magic::cast`): for actor effects, diseases, powers, abilities and
addictions the identical effect from the same spell and caster is
dispelled first (`MagicTarget::Dispel` `00824400`, identity
`004042a0`); a poison's identical effect gets the new duration added
instead (`00824c00`); the new effects start at once. Potions',
enchantments' and the Usage Monitor Effect's (`0x14F`) branches aren't
applied to casts of `ALCH`/`ENCH` sources.

## Missing / next

- The warning and alarm packages (AI): steal/pickpocket warnings, alarm
  responses of witnesses (`HighProcess::ProcessAlarm`).
- The victim's murder flag and kill-karma gate (process flag 4).
- Owned activators' alarm (`005113f0`), area-effect line of sight.
- The deferred title box shows only the latest reputation in the game;
  the viewer queues every box.
