# The HUD's corner messages and their pictures

The messages in the HUD's top-left corner (`hud_main_menu.xml`'s message
bracket) each have a picture beside them. Read from FalloutNV.exe
1.4.0.525:

- `QueueUIMessage` (`007052f0`, the HUD's `00775380`) takes the text, a
  type, a picture, a sound, how many seconds and whether it goes first.
  Without a picture the type picks a Vault Boy: 0 neutral, 1 very happy,
  2 sad, 3 in pain.
- A script's `ShowMessage` of a message that isn't a box (`005b4630`)
  passes the message's own picture: its `INAM`, a menu icon record
  (`MICN`) whose `ICON` is the picture; none, the neutral Vault Boy.
- The game's own messages pass a picture per call. Each of the 80 callers
  of `007052f0` was read for the text setting it shows and the picture it
  pushes: `world::message_icon::for_setting` has the ones with one picture
  (sad for the sleep, wait, fast travel, equip and save refusals; in pain
  for rising radiation and hardcore needs, very happy when they fall; the
  padlock for locks needing a key or more skill, the key for opening with
  one; surprised for the casinos' refusals, "out of lockpicks", a broken
  lock, an essential person down, being over-encumbered; the gift box for
  "added"; the map for a new map marker, the radio tower for a station).
  `sHackIneligible` has two (angry `00501310`, sad `005015f0`) and isn't
  listed.
- Reputation changes read their picture from a setting
  (`sRepPositiveGainIcon`…, `00615730`…), karma from `sKarma…Image`
  (`0094fd30`).
- The casino games: refusals and "out of chips" surprised, chips lost sad,
  the ban very happy (`00734db0`, `007bd2a0`, `007c2c70`), chips won the
  "added" notice's gift box (`004821a0`, `0101c140`).

## Here

`world::scripting::Event::Message` carries the picture (`icon`; `None` the
neutral Vault Boy); `world::message_icon` has the pictures, the table and
`of_message`. The viewer's `hud::HudMessages` queue holds `HudMessage`s
(text and picture) for `ui::hud::Hud::queue_message`. Given pictures: a
script's `ShowMessage`, the casinos' messages, the lockpicking menu's,
locked doors, broken equipment (`sCantEquipBrokenItem`), reputation and
karma, hardcore needs and radiation, the sleep/wait refusals of
`ShowSleepWaitMenu` and a bed's; weapon and armour condition and a
weapon breaking (type 2: sad, `00891360`); talking and pickpocketing
refusals (`world::living::pickpocket::Use::Refused`: surprised for the
unconscious, sad otherwise) and being caught (`sPickpocketFail`, sad); a
companion carrying too much (sad); a terminal the player can't hack, used
(angry, `00501310`); locked containers (the padlock); a lock opened with
its key: `UILockpickingUnlock` and "Unlocked with <key>." with the key
(`005180b0` doors, `00516dc0` containers; `world::locks::try_open`); the
Pip-Boy's book reading (very happy), drop refusals (sad) and fast travel
refusals (`world::map::travel_refusal`: sad, neutral for carrying too
much); challenges' counts their record's `ICON` (`0048e730`; vanilla's
have none: neutral).

Not corner messages in the game, so not given a picture: experience and
"LEVEL UP" go to the HUD's XP meter (the viewer filters them out of the
corner); a discovered place, quests added, completed or failed and
objectives shown or completed go to the HUD's quest text (below).

# The HUD's quest text

The block under the corner messages (`QuestReminder` in
`hud_main_menu.xml`, at 2 × safe x + 15, 2 × safe y + 170) shows quest
updates. Read from FalloutNV.exe 1.4.0.525 with the Xbox 360 prototype's
names (`HUDMainMenu`, `QuestUpdateManager`). `0076bfe0` fills it: 64
`template_justify_left_text` in `QuestAdded` (the first is the title, the
rest letters), 32 `template_filled_checkbox` and 32 text tiles in
`QuestStages` (a tile is added at the head of its parent's list,
`00a087d0`, so the last made comes first).

## What's queued

- Quest names (`QuestUpdateManager::xQuestNames`, records of 0x324
  bytes): `0077a480` queues a quest with its state as the type —
  failed (flag 0x40) "Quest FAILED" (`sQuestFailed`), else completed
  (0x02) "Quest completed" (`sQuestCompletedText`), else "Quest added"
  (`sQuestAddedText`) — and stops a completed or failed quest being the
  active one (an added one becomes it when there's none). Its callers:
  the first objective shown for a quest (`005ec5d0`; the quest's flag
  0x20, cleared by `ResetQuest`), completing a quest that wasn't
  (`0060ca30`: `CompleteQuest`, a stage's "complete" entry) and failing
  one that's neither completed nor failed (`0060caf0`, which sets both
  flags).
- Custom text (`SetCustomQuestText`, `0076b960`): title, subtitle, queue
  priority (`HCQQP_NOW` 0 to the head, `NORMAL` 1 and `LAST` 2 to the
  end; the list sorted by priority, `0076bb00`), justification
  (`HCQTJ_LEFT`/`CENTER`/`RIGHT`), fonts (−1 the HUD's; 0, font 1, isn't
  allowed) and a sound. Its one caller is the compass update
  (`00779070`): a map marker found shows `sDiscoveredText` ("You have
  discovered") over the marker's name, left, normal, with
  `UIPopUpQuestNew` (plus the "Locations Discovered" count and
  `iXPRewardDiscoverMapMarker` experience).
- Objective lines (`0077a5b0`, at the head of `HUDMainMenu` +0x258): an
  objective shown (`005ec5d0` state 1) or completed from shown (state 3,
  its quest not completed): text, completed, reminder. The reminder
  (`007736d0`: the active quest's shown objectives, from the HUD's update
  when the Pip-Boy is disabled, `iQuestReminderPipboyDisabledTime`) isn't
  followed here.
- `0077a480` and `0077a5b0` stamp the time (`011d96ac`); nothing new
  shows until 2 seconds after the last stamp (custom text doesn't stamp).

## Quest names (`0077a650`, `0077c170`)

When no name is up (a `QuestAdded` text still visible), the queue isn't
empty, the objectives aren't up with their first line still seen, the
game isn't in menu mode (or the dialogue menu is up, or `00703d50`'s
case) and not in V.A.T.S. (`011f2250` + 8): the next name shows, with its
sound (`UIPopUpQuestNew` for added, the custom sound, else
`UIPopUpQuestComplete`).

- The title (font 7, or the custom font) at y = −(its height + 15); a
  custom one at x = (W − 2 × safe x − width / 2) × 0, ½ or 1 by its
  justification (W the menu's width). A quest completed hides the
  objective lines and drops those waiting (`0077f5e0`).
- The subtitle (the quest's name, or the custom subtitle) in capitals,
  in font 8 (or the custom font), one letter to a tile from x = (W − 2 ×
  safe x − (f + 1) × its width) × f (f the justification's factor), each
  letter's width on; at the last space at or before its 33rd character
  (when it's 32 long or more) the rest start again a line lower (the
  letter's height + 5). Letters past the 63 tiles aren't shown.
- Each tile's `user0` is when it's due (the title now, each letter
  `fQuestCinematicCharacterFadeInDelay` (0.1 s) after the last) and its
  `user1` `fQuestCinematicCharacterRemain` (4 s) after that. Each frame:
  a tile due fades in to `fHudOpacity` × 255 over
  `fQuestCinematicCharacterFadeIn` (2 s); one past `user1` fades out from
  at least 175 over `…FadeOut` (2 s); one faded out is hidden. The
  objective lines are kept at alpha 0 meanwhile.

## Objective lines (`0077b430`)

With no name up and the character generation menu (1048) not up:

- When no line is up, the waiting ones take lines from the 32nd up
  (newest first, so the oldest shows on top): "COMPLETED: " +
  the text with a filled box (`sHUDQuestCompleted`), else the text with
  an empty box; font 7, wrap width W / 3, the box 20 to the left;
  `UIQuestUpdate` for each. A reminder and a real update don't share
  the lines. The lines shown are stacked, each its height lower (the box
  3 lower).
- The top line fades in (0.5 s) and waits 3 ×
  `fQuestCinematicObjectiveFadeInDelay` (1 s; `_WaitComplete` 0 → 1); the
  second and third fade in (`…FadeIn`, 0.5 s) after one and two delays.
  Then the top line holds `fQuestCinematicObjectivePauseTime` (3 s,
  `_HoldTime`), fades out (`…FadeOut`, 0.5 s), is hidden, and the rest
  scroll up by its height over `…ScrollTime` (0.5 s); a fourth comes in.

## Here

`world::quest_text` (`QuestText`, `Event::QuestText`; the quest's flag
0x20 as `GameState::quests_announced`, saved) and `world::map::discover`;
`ui::quest_text` (the tiles, queues and the three functions above) in
`ui::hud::Hud` (`queue_quest`, `queue_objective`, `sounds`); the viewer's
`hud` sends `Event::QuestText` and `Event::Objective` there
(`HudMessages::quests`, `objectives`) and plays the sounds; with the HUD
off they're notices as before. The settings' values come from the game's
data (the exe's defaults otherwise).

Not done: the reminder lines (`007736d0`); `KillQuestUpdates`
(`0077f500`); the HUD's states 7–9, 0x12, 0x13 that stop the update;
the quests' own queue priority (`0077a480` leaves the record's priority
unset: taken as normal).
