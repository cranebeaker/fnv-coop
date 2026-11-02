# Caravan

Caravan is New Vegas's card game (`CaravanMenu`, class 1083,
`menus\caravan_menu.xml`), played against people whose dialogue calls
`ShowCaravanMenu <deck> <difficulty> <bet share>` (27 calls in 18 scripts:
Ringo, the Mojave Outpost's soldiers, the Tops' high rollers…). Until
2026-10-06 nv-rs had none of it.

The PC exe inlines most of the game into the menu's update (`00741500`,
26 KB). The Xbox 360 build's PDB names its parts, and its code
(decompiled from `Fallout_Release_MemDebug.xex`, section 4 at
`0x82250000`) was matched to the PC's piece by piece; both use the same
member offsets (`trackInfo` at +0xDE4, the tracks at +0xB44) up to
+0xE7C; the PC adds a field at +0xE80 (the arrow key waiting, written by
`00749360`), so the Xbox's later members are 4 further on (its
`bContinueProcessing` +0xE82 is the PC's +0xE86, set to 1 by the
constructor `0073b7e0`).

## Read from the game

| What | Xbox 360 (PDB) | PC |
| --- | --- | --- |
| A card's suit and value | `TESCaravanCard::Load` | |
| A deck's cards | `TESCaravanDeck::Load` | |
| `AddCardToPlayer` | `PlayerCharacter::AddCaravanCard` | `005cf3d0` → `00969bc0` |
| `ShowCaravanMenu` | `CaravanMenu::Create` | `005cf250` → `00741060` |
| Whether a card may go on a row | `IsValidCardPlacement` | `0074f6c0` |
| A track's value, way and suit | `UpdateTrackValue` | `0074ee70` |
| Under / sold / over | `CalculateWinningState` | |
| The end | `IsGameOver` | |
| The deal | `PrepareGameMenu` | |
| The player's moves | `DoGamepad` | `00747d30` |
| The turns, jacks, jokers, the opponent's moves | `DoIdle` | `00741500` |
| Which controls work | `UpdateCaravanFlags` | |
| The ante | `PrepareAnteMenu`, `PrepareBettingCurrency`, `ItemSelectCallback` | `0074a090` |
| The stake | `PrepareDeckMenu` | |
| The results | `PrepareResultsMenu` | `00740980` |
| Paying up | `ExchangeCurrency` | `0074cc70` |

- Cards (`CCRD`): the first `INTV` the suit (1 hearts, 2 spades, 3
  diamonds, 4 clubs, 5 blank), the second the value (1 ace, 2–10, 12 jack,
  13 queen, 14 king, 15 joker); `TX00`/`TX01` the face and back. Decks
  (`CDCK`): `CARD`s.
- The player's cards are two lists on the player (PC +0x614 outside the
  deck, +0x618 in it, `00969bc0`; the Xbox's +0x624 and +0x628), not
  things carried: a card's script
  (`CardAddToPlayerScript`) calls `AddCardToPlayer` when the player picks it
  up and `RemoveMe`. A card already in either list isn't added again. The
  player plays with at least 30 cards (`UpdateCaravanFlags`).
- The table: six tracks of up to seven rows; tracks 0–2 the player's, 3–5
  the opponent's, track t against 5 − t. A row is a number card and the
  face cards played on it.
- The deal (`PrepareGameMenu`, `0073ea90`): the player's deck is their
  in-deck cards in the deck screen's order, the opponent's the
  `ShowCaravanMenu` deck's `CARD`s; eight cards each, the player's then
  the opponent's, each picked at random from what's left and taken out with
  the deck's last card moved into its place (`009a4320`); then each side's
  next card is picked at random. Decks are never shuffled. Cards drawn
  later, and hand cards played, are taken out keeping the order
  (`006bf8f0`).
- Starting the caravans (flag `CAF_TRACK_SETUP` 0x800): number cards only;
  the player's go on their first empty track (0, 1, 2), the opponent's on
  its last empty one (5, 4, 3), row 0; nothing is drawn. Throwing a card
  away then draws one and keeps the turn. It ends when tracks 2 and 3 both
  have a card.
- Placing (`IsValidCardPlacement`): a number card only on a new row of the
  side's own track, never the same value as the number card before it, and
  only the track's way (up after a higher card, down after a lower) unless
  it's the track's suit. A face card on a row with its number card and at
  most two others, on any track; a queen only on a track's last row.
- A track's value (`UpdateTrackValue`): each row's number card, doubled for
  each king on it. Its way: up when its last number card is higher than the
  one before, else down (none with one card). Its suit: the last number
  card's, each queen on the last row changing it to hers and turning the way
  round.
- After placing (`DoGamepad`, state 21): the card leaves the hand and,
  past the start, the side draws its next card to the end of its hand (and
  the next is picked at random). Jacks (state 17, 18) take away their row
  (the number card, everything on it, the jack), the rows after it moving
  up. Jokers (states 13, 19) stay and take away every other row whose number
  card has the same value as theirs, or for a joker on an ace the same
  suit, on any track.
- Throwing away a card draws one; throwing away a whole track empties it
  (state 15).
- Turns (state 21, the byte at PC +0xE86): the opponent moves first, then
  each in turn; a card thrown away while starting the caravans keeps the
  turn.
- The opponent (`ProcessAI`, `0074fdc0`; `CaravanAIPackage`): every move it
  could make becomes a package (the value it would bring a track to, how
  that stands, the action, track, row and hand card, a priority); sorted by
  priority with the exe's own `qsort` (Visual Studio 2008's, `00ec6f20`, so
  equal priorities fall as in the game), the first is played. For each of
  its tracks: under 21, number cards and kings that don't pass 26
  (priority the new value; kings +1, −5 at difficulty 2); sold but beaten
  by the player's facing track, number cards (priority 100 − the margin
  over it, 50 for a tie, else 25 − what's short of 26), jacks on rows
  whose loss would put the player below it (75 + or 100 − the margin) and,
  above difficulty 0, jokers (an estimate of what each track loses; at
  difficulty 3 a score over all three pairs); sold and ahead, number cards
  and kings that stay sold; over 26, jacks that bring it back (100 −
  (26 − value + the row)). On the player's tracks: jacks on sold (51 −
  what's left) or short (26 − what's left) tracks, kings that push one
  over (its value − 1). With nothing to play it throws away its highest
  card (priority: its place in the hand) or one of its unsold tracks
  (value − 26; × 2 above difficulty 0; − 5 a jack in hand at 3). Starting
  its caravans: easy plays its last number card unless it has six face
  cards (then throws its last face card away); 1–2 keep 6–10s or 1–3s and
  throw a face card away with too few (a joker first; a jack gives way to
  anything, a king to anything but a jack); 3 plays the card nearest 6.
- The game's oddities, kept: a jack meant for a player's row is checked
  and played on the opponent's own track at that row number; the joker
  estimate counts aces by value on an ace and otherwise sums the rows whose
  suit number equals the card's value; a king counts once under 21 and
  twice when sold; the queen move on the player's tracks can never happen
  (it asks for an empty row inside a loop over non-empty ones).
- `ShowCaravanMenu deck difficulty share` (`CaravanMenu::Create`): on
  someone else; with fewer than 30 cards owned the player is told
  `sCardCountText` instead. (`sPlayerOutOfMoneyText`, "You do not have
  funds to play Caravan.", is in the exe but nothing uses it.)
- The ante (`PrepareAnteMenu`): each side's funds are its caps and its
  caravan money (`CMNY`, each kind's `DATA` value × how many). The
  opponent opens with its funds × the share. Raise (a "how many" up to
  what the player has left, `ItemSelectCallback`/`0074a090`): the
  opponent matches up to its most, funds × share × (1 + the player's
  Barter / 100), Barter 0–100, at most its funds. Match: the player puts
  in the opponent's ante or all they have. Accept works once the player
  has put something in (or the opponent asks nothing); match and raise
  stop working at the player's funds (and match at the opponent's ante)
  (`UpdateCaravanFlags`). The stake is the smaller ante (`PrepareDeckMenu`).
- The results (`PrepareResultsMenu`): the player's record (caps won or
  lost, games won or lost, the biggest win), misc stat 0x25 "Caravan Games
  Won" or 0x26 "Caravan Games Lost", `GAMECaravanWin`/`GAMECaravanLose`;
  the loser pays the stake (`ExchangeCurrency`): caravan money first, each
  kind as far as it goes without passing what's owed, then caps, as many as
  they have.
- The end (`IsGameOver`, before every turn): a caravan is sold at 21–26
  and wins its pair if the other isn't sold or is lower; all three pairs
  decided, the side with more wins. Otherwise a side with no cards in deck
  or hand left loses (the player checked first).

## The menu

`CaravanMenu`'s update (`DoIdle` `00741500`) is a state machine
(`CaravanState` 0–23, the screen `eActiveMenu` +0x28, the flags
`enumFlags` +0xE74), read with the Xbox prototype's PDB names:

- Screens one way: the ante (`PrepareAnteMenu` `0073d020`), the deck
  (`PrepareDeckMenu` `0073d850`, never skipped), the game
  (`PrepareGameMenu` `0073ea90`), the results (`PrepareResultsMenu`
  `00740980`); any key on the results closes the menu. Leaving the ante or
  the deck screen (R / B) closes at once: nothing paid, the deck kept. The
  camera moves are the table's `Bet_to_Deck`, `Deck_to_Play`, `Play_to_Bet`
  (its `Bet_to_Play`, `Play_to_Deck`, `Deck_to_Bet` are never used).
- The clock: the seconds in the state (`ftotalStateSecs`), the update's
  milliseconds ÷ 1000 added while the state is the one it was last update,
  else 0. An animated state starts its sequences at time 0, updates its
  models at the time each update and moves on once the time reaches the
  sequence's end time (passes it, for the column-at-a-time states 15, 17,
  18). Behind a message box or tutorial the clock isn't kept, so the time
  spent there is added at once on the way back.
- The ante: W (A) matches, A (X) raises through the "How many?" box (up to
  the funds left; `ItemSelectCallback` counts the money put on the table
  from the old ante, not the raise), F (Y) accepts. The money on the table
  (`UpdateBettingUI` `0074a2c0`) is only for show: bills and coins picked
  at random by the opponent's most (`world::caravan::money`), landing for
  a second (state 4, `GAMECaravanFundsDrop`).
- The deck screen: the player's cards sorted by value (out of the deck
  first, then in it; the game's `qsort`), the chosen one in the middle
  (the scrollbar starts at 12); ← / → or a drag move along (a step each, 5
  or more one fast move). The drag (state 0, no pad): a mouse button held
  (`InterfaceManager::fMouseHeldTime`, the interface's +0x48, above 0)
  notes the scrollbar's value (`+0xaf8`, following `+0xe85`); let go on the
  deck screen, the cards move by how far the bar moved since, so the bar
  moved by the wheel with no button held moves no cards (a press on the bar
  itself, `007492f0`, starts one too). With a pad the left stick (past
  7849 of 32767) steps a card each update the menu is idle, forward for
  right, before the arrows and without their guard; on the player's turn
  it moves as the arrows do (an arrow taking the place of its axis). W (A) adds, A (X) removes, F (Y) plays with 30
  or more (at most 108), S (RB) makes a random deck of 30 (with more than
  30 owned, 30 up to one fewer than all). Steps forward show each card as
  they end; back, the cards are set once for the first step.
- The game (states 11–21): the hands dealt with the camera (state 2);
  the opponent moves first, on the update after the last animation ended
  (no delay of its own). The player's turn: ← / → choose a hand card
  (round; up or down alone picks the last card, the game's 0 / 0), W (A)
  picks it and then the place (arrows move along the player's tracks and
  rows, crossing above row 0 to the facing track; a green or red marker
  says whether it may go there), W again places it; Q (LT) throws it
  away; E (RT) then the arrows and W throw a track away (an empty one
  too); R (B) cancels a choice, or asks whether to forfeit (a loss). At
  most one cursor move a quarter second, the first of a turn after 0.25 s.
  Jacks take their row and those after it out a column at a time (highest
  first), the rows move up, and the rest come back; a joker does that for
  each track with marked rows, in track order, and its rows are taken
  without emptying the last (a full seventh row would be left doubled).
- Sounds (`GAMECaravan…`), the tutorials (`HelpCaravanBetting`,
  `…DeckBuilding`, `…StartingCaravans`, `…ContractWar`, ids 0x1E–0x21 of
  the tutorial manager, [TUTORIALS.md](TUTORIALS.md): opened by the menu
  itself when not yet shown, the menu waiting until it's on top again), the results'
  `GAMECaravanWin` / `GAMECaravanLose` before the camera goes back.

## The table

`cellview::caravan` puts the 3D together as `PrepareShared3DElements`
(`0073cbf0`), `PrepareDeckMenu` (`0073d850`) and `PrepareGameMenu`
(`0073ea90`) do: every model in one node, as authored (no transform set);
the camera is the table's own `NiCamera` `object0` (under `Camera01` and
`Dummy05`, which `Bet_to_Deck`, `Deck_to_Play` and `Play_to_Bet` move),
with the file's frustum (45° across, 16:9, near 1, far 5000), kept on any
shape of screen and so stretched (nothing in the menu fits it to the
screen; the viewer's `FileFrustum`); the lights are
the table's three point lights (also under `Dummy05`, so they move with the
camera), radius 2.5 × the node's bound radius with only the table in it,
attenuation zeroed (`00b5ca70`). Poses stay where a model's last sequence
left them (Gamebryo's `NiControllerSequence`). Card faces are set on their
shapes' diffuse slot (`Textures\` + the card's `TX00`, backs `TX01`):
`Card_0{n}:{column}` on the rows (n the column + 1 on the player's tracks,
4 − the column on the opponent's), `Player-Deck_0{n}:{n−1}` on the hands
(the opponent's showing backs), `Deck_{nn}` / `Available_{nn}` on the deck
screen. The markers (`Select_Add`, `Select_Remove`) go to the grids'
`Select{r}_0{c}:0` shapes (their own turn × a quarter turn about z, their
bound's centre); the money to `Bill-Placement` / `Coin-Placement` shapes.

The game draws the table after the image space pass and under the menus'
pictures (`Draw3DElements` `00740f30`); the viewer draws it into the HUD's
picture with a camera of its own before the HUD's, which then blends its
pictures over it (`viewer/src/caravan_table.rs`).

## Here

`world::caravan`: `card`, `deck`, `Collection` (`GameState::caravan`,
saved as `caravancard` / `caravanrecord` lines), `add_card_to_player`
(the `AddCardToPlayer` command), `Game` (`new`, `is_valid_placement`,
`update_track_value`, `is_game_over`, `play` and `resolve`, `discard`,
`discard_track`, `setup_track`, `setup_finished`, `npc_turn`,
`player_play`), `winning_state`; `world::caravan::ai` (`process_ai`,
`Package`, `crt_qsort`); `world::caravan::bet` (`funds`, `Bet`, `settle`);
`world::caravan::menu` (`Menu`: the screens, states, flags and controls
above, telling the caller what to do as `Effect`s); `world::caravan::money`
(the ante's bills and coins); `ShowCaravanMenu` raises `Event::Caravan`.
`ui::menus::caravan` fills `caravan_menu.xml`'s tiles; `cellview::caravan`
reads the table's models; the viewer runs the menu
(`viewer/src/game_menus/caravan.rs`: input, the "How many?" and quit boxes,
the effects on the table, the cards written back, the stake paid) and draws
the table (`viewer/src/caravan_table.rs`). Seen live against Ringo with his
deck: the ante with its money, the deck screen, the deal and the start of a
game, the quit box (Yes: lost for the stake, 4 caps paid to Ringo), the
results back at the betting camera ("Losses to Date: 4", "0/1"); the betting and deck-building tutorials (closed with E); the deck screen at 1024 × 768 showing the 16:9 frustum stretched (`--screen-size`). Checks: `world::caravan::tests`,
`world::caravan::ai::tests` (39 whole games each end),
`world::caravan::menu::tests` (40 whole games through the menu, every
state reached), `world::caravan::money::tests`,
`crates/world/tests/caravan.rs`.

## Not done yet

- The order money is paid in follows the form IDs here: the game goes
  through the loser's inventory list (`0046f310`: the reference's changed
  entries, then its base container's), and nv-rs keeps no inventory order
  (`GameState::items` is by holder and form).
