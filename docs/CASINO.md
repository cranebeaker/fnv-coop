# The casino games

Blackjack, roulette and the slot machines, as FalloutNV.exe 1.4.0.525 runs
them (the Xbox 360 prototype's PDB naming the parts; PC addresses below).
`world::casino` has what the three share and each game's rules and menu
states; the tiles, the 3D and the viewer follow Caravan's layout
([CARAVAN.md](CARAVAN.md)).

## The record (`CSNO`, `TESCasino`, loader `005048d0`)

- `FULL` the name the earnings line shows (Gomorrah's is "Gommorah" in
  FalloutNV.esm).
- Ten models: `MODL`s in order (the chip stacks for 1, 5, 10, 25, 100 and
  500, the roulette chip, the slot machine), then `MOD2` the slot machine
  again (the editor writes both), `MOD3` the blackjack table, `MOD4` the
  roulette table.
- Eleven textures: `ICON`s the reels' symbols (A to F, then the wild:
  Symbol1, 2, 3, 4, 6, 7, 5 in every record), `ICO2`s four card backs.
- `DATA` (`CASINO_DATA`, 0x38 bytes): blackjack's shuffle point (0 means
  0.25) and blackjack payout (0 means 1.5), the seven reel symbols' stops,
  the decks (0 means 1), `iMaxWinnings`, the chip (`CHIP`), the comps
  quest, the dealer's hole card flag.
- Chips (`CHIP`) have no `DATA`: worth nothing, weightless. The cashiers'
  dialogue swaps caps (and NCR money) for them.

## What the games share

- **The player's list** (`PlayerCharacter` +0x610, `CasinoData`): per
  casino the net chips won by every round of every game there (it can go
  below 0) and the level reached. Each game's `Create` adds the casino's
  line first (at the head), even when it then refuses. Saved.
- **Levels**: k (1 to 4) at `winnings ≥ k × 0.25 × iMaxWinnings`. After a
  round's payout (blackjack `00736b56`, roulette `007be412`, the slots after
  their coins `007c360d`) a level above the stored one is stored and the
  menu closes; closing then starts the casino's comps quest
  (`0060c9c0(quest, 1)`). The quests' scripts and the floor managers'
  dialogue hand out the comps (`GetCasinoWinningStage` conditions).
- **Script functions**: `GetCasinoWinningsLevel` (`005dec30`) and the
  condition `GetCasinoWinningStage` (`005a6170`) work the level out from the
  winnings (so it drops with losses), 0 where not played;
  `SetCasinoWinningsLevel` (`005ded40`) sets a level and its winnings;
  `SetCasinoCheatLevel` does nothing on PC.
- **Opening** (`ShowSlotMachineMenuParams`, `ShowBlackJackMenuParams`,
  `ShowRouletteMenuParams`: casino, least bet, most bet, and a fourth
  number the command calls "Max Winnings" that's really the least winnings
  to sit down; blackjack's handler drops it). `Create` refuses in turn,
  each as a corner message with `UIPopUpMessageGeneral`: the anti-cheat
  lock, banned (winnings at the limit), too few chips for the least bet,
  too little won.
- **The anti-cheat lock** (not saved): closing a casino menu stamps the
  time (`00969b20`); loading a game while stamped arms it (`00969ac0`);
  armed, every game is refused for `iAntiCheatDuration` (60) seconds after
  the stamp ("…as an anti-cheating measure.\nTime Remaining: N").
- **Chips**: the menu keeps its own count from the player's and settles
  once on closing: fewer, the difference is taken ("N <chip>(s) removed",
  sad Vault Boy); more, given (the usual "added" notice, or at the limit
  quietly with "N <chip>(s) added\nYou have been banned from gambling at
  this casino.").
- **Texts**: the earnings line `"%s %s%i"` (name, "Earnings: ", winnings;
  its padding to 23 characters only takes with a one-character number),
  round results `"%s %d %s%s"` ("You win 5 chip(s)", "You feel lucky. You
  win…").

## The slot machines (`SlotMachineMenu`, class 1080)

- **The strip** (`SetupActiveReels` `007c16b0`): 28 places filled round
  by round, each symbol with stops left putting itself (2k) and its blank
  (2k + 1); the stops must come to 14 or the menu closes with an error.
- **A spin** (`SpinReels` `007c56e0`): each reel `strip[rand(0, 2n − 1)]`
  (n the symbols with stops), so only the first round of the strip counts
  and its last blank never comes up: in the vanilla casinos 13 outcomes a
  reel. Everything is decided on the click.
- **Results** (`InterpretSpin` `007c5760`): three of a kind (6) pays the
  bet × 10, 20, 30, 40, 60, 100 for bell, BAR, 7, lemon, grapes, orange,
  × 10 for three cherries (the wild); two wilds (5) × 5; one (4) × 2;
  anything else loses the bet (pairs too). (The Xbox prototype and the
  first payout texture paid 80 and 160 for grapes and orange.)
- **Luck** (`ApplyLuck` `007c58f0`, Luck read once at the start): acts when
  `rand(0, 100) + |Luck − 5| × 10 > 100`. Below 5: wilds turn blank, three
  of a kind slips (A, B, C to blanks; D, E, F and wilds to three bells);
  the "unlucky" flag it sets is lost to its return of 0, so "You feel
  unlucky. You lose" never shows and losses are silent. Above 5: a loss
  spins again (luck rolled again), a pair whose odd reel is within Luck
  (squared distance in ids) becomes three, a wild's neighbours within Luck
  match.
- **States** (`DoIdle` `007c2c70`; seconds counted only while the menu is
  on top): the lever and reels (`GameSlotsPullLever`; the window's faces
  retextured as the lever ends, the rest as the reels stop), the result
  (one update: paid, the status line), the coins for a win
  (`GAMESlotsWinSmall`, `Med`, `Jackpot`; then the level check), the
  payout card in and out, idle (out of chips: "You must purchase chips…"
  and the menu closes; fewer chips than the bet sets it to 1).
- **Clicks** (`DoClick` `007c2460`; W, E, Q, F, S, R by the menu's own
  keys): Spin (with chips for the bet; counts "Slots Games Played"),
  Increase (+1 under 10, 5 under 40, 10 under 100, 25 under 500, else
  100; no more than the chips or the most bet; `GAMESlotsIncreaseBet01`
  to `03` in turn), Decrease (back by the same steps to their multiples;
  no less than the least bet; `GAMESlotsDecreaseBet`), Payout List, Bet
  Max, Exit. With the payout card in, any click takes it out.
- **The reels' faces** (`SwapFrontfacingTextures` `007c4d70`,
  `SwapBackfacingTextures` `007c40b0`): each reel's 14 two-sided meshes
  `Cylinder0{6 − reel}:{face}`, its payline face 2 × reel; the window's
  six faces show the result with its neighbours, the other eight skip a
  symbol and repeat one (only seen spinning).
- Here: `world::casino::slots` (`SlotMachine`, its `Effect`s).

## Blackjack (`BlackJackMenu`, class 1081)

- **The shoe** (`InitCasinoData` `00731d70`): the casino's decks (0 means
  1) of 52 in order, never shuffled: a "shuffle" (`007399d0`) only puts
  the count of live cards back, when idle below the casino's share of
  them (0 means 0.25) and before a card with fewer than 5 left. Deck k's
  faces are `Textures\Terminals\NV_PlayingCards\<folder>\<prefix>_<h|c|d|s>NN.dds`
  from its back (texture 7 + k).
- **A card** (`DealCard` `00739090`): `rand(0, N)` swapped to the end of
  the live cards; two come off the count (the card and the slot after it).
  With luck (the dealer at Luck 4 or less, the player at 6 or more) |Luck
  − 5| draws are weighed and the one bringing the hand nearest 21 (or 11
  under 10) taken, swapped with the slot one past the live cards (the
  "lucky" and "unlucky" words come from it).
- **Dealing** (`DealHand` `007393f0`): the dealer, the player (plain),
  the dealer, the player (luck); the dealer's first card face down. A
  first twenty-one is blackjack; the dealer's twenty-one ends the round
  (a draw against the player's). Doubling on any first two cards with the
  chips for it; splitting a pair of equal cards (10 and king don't pair)
  once, the split cards dealt without luck and both blackjack tests
  reading the main hand; the split hand can't double (`CanDoubleDown(true)`
  sets 0x200, 0x100 is read); surrender before any other move; Stay ends
  both hands.
- **The dealer** (state 11): hits under 17, and on 17 with any ace unless
  the casino's `bBJ_DealerHoleCard` byte (held on soft 17) is set, to seven
  cards; the hole card is turned first (state 8) even after a bust.
- **Paid** (state 12, "Blackjack Games Played"): a win the bet, blackjack
  `trunc(bet × payout + 0.51)` (the split hand's `+ 0.5`), surrender
  `−trunc(bet / 2 + 0.51)`; nothing is taken at the deal. A doubled bet
  halves again after the round.
- **States** (`DoIdle` `00734db0`, a frame's step over a second counted as
  none): dealing 1 (the four cards, half a flight apart: done at 1.583 s),
  a card 2, splitting 3 (the split hand's first card, then each hand's
  second), the player's turn 4, clearing 7 (the player's cards, then the
  dealer's: 2.6 s; then the level check), the hole card 8, a dealer's card
  9, idle 10, the dealer's turn 11, the result 12, closing 13 (settled a
  second after `Close`).
- **Clicks** (`DoClick` `00733ff0`, idle or the player's turn): Hit (F),
  Deal or Double (W), Increase Bet or Split (E), Decrease Bet or Switch
  Hands (Q), Bet Max or Surrender (S), Exit or Stay (R). Bets step +1 under
  10, 5 under 40, 10 under 100, 25 under 400, 100 under 1500, else 500.
  The buttons that can't be used now are dimmed (`UpdateTileAlpha`
  `00738560`, their children's alpha 128) but still take clicks.
- **The table** (`Prepare3DElements` `00732540`): the casino's table (model
  8), the two player hands and the dealer's (`NV_Blackjack-Hand1/2`,
  `-Dealer.NIF`, their `Hand1_0k`, `Dealer_0k`, `_Discard`, `Dealer_Reveal`
  sequences), the six chip stacks and their shadows (`SetBetChips`
  `00739b30` raises each 0.18 a chip), seen by the table's `object1`; the
  split hand sits 5 under the felt until split, the arrows
  (`Hand1_Arrow:0`, `Hand1_Arrow:0@#2`) mark the hand played, undealt
  cards are hidden while the table clears.
- Here: `world::casino::blackjack` (`Blackjack`), `ui::menus::blackjack`,
  `cellview::blackjack`, the viewer's `game_menus::blackjack`.

## Roulette (`RouletteMenu`, class 1082)

- **The wheel**: American, 38 pockets in the order of
  `world::casino::roulette::STRIP` (37 the 00). A spin (`SpinWheel`
  `007bf670`, on Finish Bet) draws the pocket and one of the table's
  `Spin_1..3` (9.83 s) on the click; the ring `Roulette_Wheel:2` is turned
  so the ball's sequence lands on it (`ShiftWheelTexture` `007bf570`).
  Luck only picks the winning words.
- **The felt** (`PrepareBetTiles` `007b9c40`): 159 spots at their meshes in
  `NV_Roulette-Points.NIF` (`A01:0`…`F24:0`, `S01:0`…`S15:0`): straights
  35:1, splits 17:1, streets and trios 11:1, corners 8:1, the five-number
  bet 6:1, six-lines 5:1, dozens and columns 2:1, halves, even, odd, red
  and black 1:1. The game's own tests make 0 win even, red and the third
  column, and 00 odd, black and the first column.
- **Bets** (`SetBet` `007bed70`, `RemoveBet` `007bf150`): up to ten, one a
  spot, the total within the most bet and the chips; a chip stays where
  the cursor put it. A win pays `(payout + 1) × bet`; the round's net
  against the total moves the chips.
- **The cursor**: the mouse moves a chip over the felt (`DoIdle` state 4:
  `x −= (960 / height × −dx / 10) × 0.35` a frame, the pointer hidden);
  the spot under it is the first within its radius (`GetBetIndex`
  `007bf390`: 0.49 × the grid's spacing; the specials' from their
  neighbours), shown green with its bet and payout on the status line.
- **Clicks** (`DoClick` `007bc870`, idle): Place Bet (W), Remove Bet (F),
  Finish Bet (S), Increase Bet (E), Decrease Bet (Q), Exit (R); bets step
  +1 under 10, 5 under 40, 10 under 100, 25 under 500, else 100.
- Here: `world::casino::roulette` (`Roulette`), `ui::menus::roulette`,
  `cellview::roulette`, the viewer's `game_menus::roulette`.

## Here

- `world::casino`: the record, the player's list (saved as `casino`
  lines), levels, the script functions, `open_check` and the lock,
  `settle` (with its corner message), the texts; `Event::Casino` from the
  three commands.
- `ui::menus::slots`, `ui::menus::blackjack`, `ui::menus::roulette` (the
  tiles), `cellview::slots`, `cellview::blackjack`, `cellview::roulette`
  (the 3D).
- The viewer: `Event::Casino` runs `Create`'s checks where the script asks
  (`game_menus::casino::create`: a refusal is a corner message with
  `UIPopUpMessageGeneral`), then `menus::Menu::Casino` opens the game's
  menu (`game_menus::slots`, `blackjack`, `roulette`); `casino_scene`
  draws the machine (with the lockpicking menu's camera) or the table
  (with its own) into the HUD's picture, under the menu's tiles (the
  HUD's camera blends over any menu's 3D: `game_menus::compose_hud_over_scene`, `OpenMenu::draws_scene`). The anti-cheat lock (`CasinoLock`) runs on
  the real clock, stamped as a menu closes and armed by F9's load. The
  scripts' `MenuMode 1080`-`1082` run once the game's menu has opened, not
  for a refusal (`game_menus::casino::menu_mode`; the world's
  `Event::Menu` for those numbers is left to it): the same path for the
  base game's casinos and the Sierra Madre's (Dead Money,
  `SierraMadreCasinoData`).
- Checks: `crates/world/tests/casino.rs`, the module tests, the viewer's
  `game_menus::casino` test.

## Trying it

```
nv-viewer <Data> VikkiAndVance --freeze-ai --run "player.additem VikkiVanceChip 100" --run "0016202A.Activate player 1"
```

(`0016202A` is one of Vikki & Vance's slot machines, `vVikkiVanceSlotScript`;
`--freeze-ai` keeps people from starting a conversation that would hold the
second line back.) W spins, E and Q change the bet, F shows the payout card,
S bets the most, R leaves: the chips are settled with a corner message.
`000A5722` is one of its blackjack tables and `000A58D3` a roulette table.
For roulette, `--menu-keys` takes `mDX/DY` to move the mouse (the cursor).

The corner messages carry the game's pictures ([HUD_MESSAGES.md](HUD_MESSAGES.md)):
refusals surprised, chips lost sad, chips won the gift box, the ban very
happy.

## Not done

- The roulette cursor by the pad's left stick.
