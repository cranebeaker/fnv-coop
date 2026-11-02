//! Caravan, the card game (`CaravanMenu`, class 1083 in FalloutNV.exe;
//! the Xbox 360 build's PDB names its parts, so they're cited by name with
//! the PC function where it was matched). Notes in `docs/CARAVAN.md`.
//!
//! - Cards (`CCRD`, `TESCaravanCard`): the first `INTV` the suit (1 hearts,
//!   2 spades, 3 diamonds, 4 clubs, 5 blank), the second the value (1 ace,
//!   2..10, 12 jack, 13 queen, 14 king, 15 joker) (`TESCaravanCard::Load`).
//!   Decks (`CDCK`): their `CARD`s.
//! - The player's cards (`PlayerCharacter` +0x614 and +0x618 on PC, the
//!   Xbox's +0x624 and +0x628: the inactive
//!   and active lists): `AddCardToPlayer` (`005cf3d0` → `00969bc0`,
//!   `PlayerCharacter::AddCaravanCard`) puts a card form in the inactive
//!   list unless it's in either already; the deck the player plays with is
//!   the active list (`PrepareGameMenu`).
//! - A game ([`Game`]): six tracks (caravans; 0..2 the player's, 3..5 the
//!   opponent's, track t facing 5 − t), each up to seven rows: a number card
//!   and the face cards played on it. Placing (`IsValidCardPlacement`,
//!   `0074f6c0`), a track's value, direction and suit (`UpdateTrackValue`,
//!   `0074ee70`), the game's end (`IsGameOver`), jacks and jokers (the menu's
//!   states 13, 17 to 19, `DoIdle`), drawing (`DoGamepad`, state 21).

use esm::{FormId, FourCC, LoadOrder};

use crate::scripting::GameState;

pub mod ai;
pub mod bet;
pub mod menu;
pub mod money;

pub const ACE: i32 = 1;
pub const JACK: i32 = 12;
pub const QUEEN: i32 = 13;
pub const KING: i32 = 14;
pub const JOKER: i32 = 15;
/// The suit of a track with no cards (`TESCaravanCard::C_BLANK`).
pub const BLANK: i32 = 5;
/// Tracks, rows in a track, cards in a hand at the deal.
pub const TRACKS: usize = 6;
pub const ROWS: usize = 7;
pub const HAND: usize = 8;
/// A sold caravan's value: 21 to 26 (`CalculateWinningState`).
pub const SOLD_MIN: i32 = 21;
pub const SOLD_MAX: i32 = 26;
/// The fewest cards a deck can be played with ("You must have at least 30
/// cards to play Caravan.", `UpdateCaravanFlags`).
pub const MIN_DECK: usize = 30;

const CCRD: FourCC = FourCC::new(b"CCRD");
const CDCK: FourCC = FourCC::new(b"CDCK");
const INTV: FourCC = FourCC::new(b"INTV");
const CARD: FourCC = FourCC::new(b"CARD");

/// A card.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    pub form: FormId,
    pub value: i32,
    pub suit: i32,
}

impl Card {
    pub fn is_number(&self) -> bool {
        self.value <= 10
    }
}

/// A card form's suit and value.
pub fn card(order: &LoadOrder, form: FormId) -> Option<Card> {
    let rr = order.get(form).filter(|r| r.entry.header.kind == CCRD)?;
    let record = rr.record().ok()?;
    let mut ints = record
        .get_all(INTV)
        .filter(|s| s.data.len() >= 4)
        .map(|s| i32::from_le_bytes(s.data[0..4].try_into().unwrap()));
    let suit = ints.next()?;
    let value = ints.next()?;
    Some(Card { form, value, suit })
}

/// A deck's cards, in the record's order.
pub fn deck(order: &LoadOrder, form: FormId) -> Vec<Card> {
    let Some(rr) = order.get(form).filter(|r| r.entry.header.kind == CDCK) else {
        return Vec::new();
    };
    let Ok(record) = rr.record() else {
        return Vec::new();
    };
    record
        .get_all(CARD)
        .filter(|s| s.data.len() >= 4)
        .filter_map(|s| {
            let id = rr
                .plugin
                .to_global(FormId(u32::from_le_bytes(s.data[0..4].try_into().unwrap())));
            card(order, id)
        })
        .collect()
}

/// The player's cards and record (`PlayerCharacter`'s
/// `pInactiveListofCaravanCards`, `pActiveListofCaravanCards`,
/// `iCaravanCapWinnings`, `iCaravanCapLosses`, `iCaravanWinnings`,
/// `iCaravanLosses`, `iCaravanLargestWinning`).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Collection {
    /// Cards owned but not in the deck, in the list's order (its head
    /// first).
    pub inactive: Vec<FormId>,
    /// The deck, the same way.
    pub active: Vec<FormId>,
    pub cap_winnings: u32,
    pub cap_losses: u32,
    pub winnings: u32,
    pub losses: u32,
    pub largest_winning: u32,
}

impl Collection {
    pub fn owns(&self, form: FormId) -> bool {
        self.inactive.contains(&form) || self.active.contains(&form)
    }
}

/// `AddCardToPlayer` (`PlayerCharacter::AddCaravanCard`): a card the
/// player doesn't have goes in with the cards outside the deck, at the
/// head of that list (`00969bc0` → `005ae3d0`, `AddHead`). False for a
/// form that isn't a card.
pub fn add_card_to_player(order: &LoadOrder, state: &mut GameState, form: FormId) -> bool {
    if card(order, form).is_none() {
        return false;
    }
    if !state.caravan.owns(form) {
        state.caravan.inactive.insert(0, form);
    }
    true
}

/// A track's direction, suit and value (`CaravanMenu::TrackInfo`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrackInfo {
    /// 1 up, -1 down, 0 none yet (`CardFlags`).
    pub direction: i32,
    pub suit: i32,
    pub value: i32,
}

impl Default for TrackInfo {
    fn default() -> Self {
        TrackInfo {
            direction: 0,
            suit: BLANK,
            value: 0,
        }
    }
}

/// `CalculateWinningState`: -1 under 21, 0 sold (21..26), 1 over.
pub fn winning_state(value: i32) -> i32 {
    if value < SOLD_MIN {
        -1
    } else if value <= SOLD_MAX {
        0
    } else {
        1
    }
}

/// What playing a card leads to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Played {
    /// It's on the track.
    Placed,
    /// A jack: its row (the card it's on, and everything on that) goes.
    Jack { track: usize, row: usize },
    /// A joker: these rows (by track) go, the joker staying.
    Joker { rows: Vec<Vec<usize>> },
}

/// A game being played (`CaravanMenu`'s `pCardTracks`, `trackInfo`, the
/// decks and hands, the next card each draws).
#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    pub tracks: [[Vec<Card>; ROWS]; TRACKS],
    pub info: [TrackInfo; TRACKS],
    pub player_deck: Vec<Card>,
    pub npc_deck: Vec<Card>,
    pub player_hand: Vec<Card>,
    pub npc_hand: Vec<Card>,
    /// The deck card each draws next (`iNextPlayerCard`, `iNextNPCCard`),
    /// picked at random from what's left.
    pub next_player: Option<usize>,
    pub next_npc: Option<usize>,
    /// Starting the caravans (`CAF_TRACK_SETUP`): number cards only, one
    /// on each of a side's empty tracks, no drawing.
    pub setup: bool,
    /// The player won (`CAF_PLAYERWIN`, set by [`Game::is_game_over`]).
    pub player_won: bool,
}

/// A random whole number below `n` (the game's generator, `829F9750` on the
/// Xbox).
pub type Dice<'a> = &'a mut dyn FnMut(usize) -> usize;

impl Game {
    /// A new game (`PrepareGameMenu`, `0073ea90`): eight cards dealt to
    /// each, the player's first then the opponent's, each picked at random
    /// from what's left of the deck and taken out with the deck's last card
    /// moved into its place (`009a4320`); then each side's next card
    /// picked.
    pub fn new(mut player_deck: Vec<Card>, mut npc_deck: Vec<Card>, dice: Dice) -> Game {
        let mut player_hand = Vec::new();
        let mut npc_hand = Vec::new();
        for _ in 0..HAND {
            if !player_deck.is_empty() {
                let i = dice(player_deck.len());
                player_hand.push(player_deck.swap_remove(i));
            }
            if !npc_deck.is_empty() {
                let i = dice(npc_deck.len());
                npc_hand.push(npc_deck.swap_remove(i));
            }
        }
        let pick = |deck: &Vec<Card>, dice: Dice| (!deck.is_empty()).then(|| dice(deck.len()));
        let next_player = pick(&player_deck, dice);
        let next_npc = pick(&npc_deck, dice);
        Game {
            tracks: Default::default(),
            info: Default::default(),
            player_deck,
            npc_deck,
            player_hand,
            npc_hand,
            next_player,
            next_npc,
            setup: true,
            player_won: false,
        }
    }

    fn hand(&self, npc: bool) -> &Vec<Card> {
        if npc {
            &self.npc_hand
        } else {
            &self.player_hand
        }
    }

    /// The rows in use: up to the first empty one.
    pub fn rows_used(&self, track: usize) -> usize {
        self.tracks[track]
            .iter()
            .take_while(|r| !r.is_empty())
            .count()
    }

    /// Whether a hand card may go on a track's row
    /// (`IsValidCardPlacement`, `0074f6c0`).
    pub fn is_valid_placement(
        &self,
        track: usize,
        row: usize,
        hand_index: usize,
        npc: bool,
    ) -> bool {
        if track >= TRACKS || row >= ROWS {
            return false;
        }
        let Some(&card) = self.hand(npc).get(hand_index) else {
            return false;
        };
        let rows = &self.tracks[track];
        if self.setup {
            return rows[row].is_empty() && card.is_number();
        }
        if !card.is_number() {
            // A face card: on a row with its number card and at most two
            // others; a queen only on the last row.
            let n = rows[row].len();
            if !(1..=3).contains(&n) {
                return false;
            }
            return !(row < ROWS - 1 && !rows[row + 1].is_empty() && card.value == QUEEN);
        }
        // A number card: a new row on one's own track, not the value
        // before it, and the track's way unless it matches its suit.
        if !rows[row].is_empty() || (npc && track < 3) || (!npc && track > 2) {
            return false;
        }
        if row != 0 {
            let Some(before) = rows[row - 1].first() else {
                return false;
            };
            if before.value == card.value {
                return false;
            }
            let info = self.info[track];
            let against = match info.direction {
                1 => before.value >= card.value,
                -1 => before.value <= card.value,
                _ => false,
            };
            if against && info.suit != card.suit {
                return false;
            }
        }
        true
    }

    /// A track's value, direction and suit (`UpdateTrackValue`,
    /// `0074ee70`): each row's number card, doubled for each king on it;
    /// the way the last two number cards go; the last number card's suit,
    /// each queen on the last row changing it to hers and turning the way
    /// round.
    pub fn update_track_value(&mut self, track: usize) {
        let n = self.rows_used(track);
        let rows = &self.tracks[track];
        let value = rows[..n]
            .iter()
            .map(|row| {
                row.iter()
                    .skip(1)
                    .filter(|c| c.value == KING)
                    .fold(row[0].value, |v, _| v << 1)
            })
            .sum();
        let mut info = TrackInfo {
            value,
            ..TrackInfo::default()
        };
        if n >= 1 {
            let last = &rows[n - 1];
            info.direction = if n >= 2 {
                if last[0].value > rows[n - 2][0].value {
                    1
                } else {
                    -1
                }
            } else {
                0
            };
            info.suit = last[0].suit;
            for queen in last.iter().skip(1).filter(|c| c.value == QUEEN) {
                info.suit = queen.suit;
                info.direction = -info.direction;
            }
        }
        self.info[track] = info;
    }

    /// Whether the game is over (`IsGameOver`): every track's value worked
    /// out; each of the three pairs (track t and 5 − t) won by a sold
    /// caravan the other doesn't beat; all three decided, the side with more
    /// wins; else a side with no cards in deck or hand loses.
    pub fn is_game_over(&mut self) -> bool {
        for t in 0..TRACKS {
            self.update_track_value(t);
        }
        let sold = |v: i32| (SOLD_MIN..=SOLD_MAX).contains(&v);
        let (mut mine, mut theirs, mut decided) = (0, 0, 0);
        for t in 0..3 {
            let (p, n) = (self.info[t].value, self.info[5 - t].value);
            if sold(p) && (!sold(n) || n < p) {
                mine += 1;
                decided += 1;
            }
            if sold(n) && (!sold(p) || p < n) {
                theirs += 1;
                decided += 1;
            }
        }
        let player_won = decided == 3 && theirs < mine;
        let npc_won = decided == 3 && mine < theirs;
        if !player_won {
            if npc_won || (self.player_deck.is_empty() && self.player_hand.is_empty()) {
                self.player_won = false;
                return true;
            }
            if !self.npc_deck.is_empty() || !self.npc_hand.is_empty() {
                return false;
            }
        }
        self.player_won = true;
        true
    }

    /// The side draws its next card into its hand (at the end), and the
    /// next one after is picked.
    fn draw(&mut self, npc: bool, dice: Dice) {
        let (deck, hand, next) = if npc {
            (&mut self.npc_deck, &mut self.npc_hand, &mut self.next_npc)
        } else {
            (
                &mut self.player_deck,
                &mut self.player_hand,
                &mut self.next_player,
            )
        };
        if let Some(i) = next.take() {
            if i < deck.len() {
                hand.push(deck.remove(i));
            }
            *next = (!deck.is_empty()).then(|| dice(deck.len()));
        }
    }

    /// Plays a hand card on a track's row (`DoGamepad` for the player, state
    /// 21 for the opponent): out of the hand, a card drawn (not while
    /// starting the caravans), onto the row; a jack or joker then takes
    /// cards away ([`Game::resolve`]).
    pub fn play(
        &mut self,
        npc: bool,
        hand_index: usize,
        track: usize,
        row: usize,
        dice: Dice,
    ) -> Played {
        let hand = if npc {
            &mut self.npc_hand
        } else {
            &mut self.player_hand
        };
        let card = hand.remove(hand_index);
        if !self.setup {
            self.draw(npc, dice);
        }
        self.tracks[track][row].push(card);
        match card.value {
            JACK => Played::Jack { track, row },
            JOKER => {
                // State 13: rows whose number card shares the suit (a joker
                // on an ace) or the value of the one it's on.
                let base = self.tracks[track][row][0];
                let rows = (0..TRACKS)
                    .map(|t| {
                        (0..self.rows_used(t))
                            .filter(|&r| {
                                let c = self.tracks[t][r][0];
                                let same = if base.value == ACE {
                                    c.suit == base.suit
                                } else {
                                    c.value == base.value
                                };
                                same && (t, r) != (track, row)
                            })
                            .collect()
                    })
                    .collect();
                Played::Joker { rows }
            }
            _ => {
                self.update_track_value(track);
                Played::Placed
            }
        }
    }

    /// A row taken off a track (states 17, 18): the rows after it move up,
    /// the last emptied.
    pub fn remove_row(&mut self, track: usize, row: usize) {
        let rows = &mut self.tracks[track];
        for r in row..ROWS - 1 {
            rows[r] = std::mem::take(&mut rows[r + 1]);
        }
        rows[ROWS - 1].clear();
    }

    /// What a jack or joker takes away (states 17 to 19): the jack's row, or
    /// each row the joker marked (the ones after a removed row moving up).
    pub fn resolve(&mut self, played: &Played) {
        match played {
            Played::Placed => {}
            Played::Jack { track, row } => {
                self.remove_row(*track, *row);
                self.update_track_value(*track);
            }
            Played::Joker { rows } => {
                for (t, marked) in rows.iter().enumerate() {
                    let mut marked = marked.clone();
                    while let Some(r) = (!marked.is_empty()).then(|| marked.remove(0)) {
                        self.remove_row(t, r);
                        for m in marked.iter_mut() {
                            if *m > r {
                                *m -= 1;
                            }
                        }
                    }
                    self.update_track_value(t);
                }
            }
        }
    }

    /// Throws a hand card away and draws (`DoGamepad` key 8, state 21's
    /// discard): starting the caravans, the side keeps its turn.
    pub fn discard(&mut self, npc: bool, hand_index: usize, dice: Dice) {
        let hand = if npc {
            &mut self.npc_hand
        } else {
            &mut self.player_hand
        };
        hand.remove(hand_index);
        self.draw(npc, dice);
    }

    /// Throws a whole track away (state 15).
    pub fn discard_track(&mut self, track: usize) {
        for row in self.tracks[track].iter_mut() {
            row.clear();
        }
        self.update_track_value(track);
    }

    /// The caravans are started once the player's last (track 2) and the
    /// opponent's last (track 3) have a card (state 11).
    pub fn setup_finished(&self) -> bool {
        !self.tracks[2][0].is_empty() && !self.tracks[3][0].is_empty()
    }

    /// While starting: the player's track a card goes on (the first empty
    /// of 0..2, `DoGamepad`), and the opponent's (the last empty of 5..3,
    /// state 21).
    pub fn setup_track(&self, npc: bool) -> Option<usize> {
        if npc {
            (3..TRACKS).rev().find(|&t| self.tracks[t][0].is_empty())
        } else {
            (0..3).find(|&t| self.tracks[t][0].is_empty())
        }
    }
}

/// What a turn did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Turn {
    /// A card played (where, and what it took away).
    Played {
        card: Card,
        track: usize,
        row: usize,
        played: Played,
    },
    /// A card thrown away.
    Discarded(Card),
    /// A track thrown away.
    DiscardedTrack(usize),
}

impl Game {
    /// The opponent's turn (state 21: `ProcessAI`'s package carried out).
    /// Starting the caravans, a number card goes on its last empty track,
    /// row 0, nothing drawn; a card thrown away then is replaced and the
    /// opponent goes again (`keeps_turn`).
    pub fn npc_turn(&mut self, difficulty: i32, dice: Dice) -> (Turn, bool) {
        let p = self.process_ai(difficulty);
        if p.action == ai::action::DISCARD_CARD {
            let card = self.npc_hand[p.card];
            self.discard(true, p.card, dice);
            return (Turn::Discarded(card), self.setup);
        }
        if self.setup {
            let track = self.setup_track(true).unwrap_or(2);
            let card = self.npc_hand[p.card];
            let played = self.play(true, p.card, track, 0, dice);
            return (
                Turn::Played {
                    card,
                    track,
                    row: 0,
                    played,
                },
                false,
            );
        }
        if p.action == ai::action::DESTROY_TRACK {
            self.discard_track(p.track);
            return (Turn::DiscardedTrack(p.track), false);
        }
        let card = self.npc_hand[p.card];
        let played = self.play(true, p.card, p.track, p.row, dice);
        self.resolve(&played);
        (
            Turn::Played {
                card,
                track: p.track,
                row: p.row,
                played,
            },
            false,
        )
    }

    /// The player plays a hand card on a row (`DoGamepad`): starting the
    /// caravans it ends once tracks 2 and 3 both have a card (state 11).
    pub fn player_play(&mut self, hand_index: usize, track: usize, row: usize, dice: Dice) -> Turn {
        let card = self.player_hand[hand_index];
        let played = self.play(false, hand_index, track, row, dice);
        self.resolve(&played);
        if self.setup && self.setup_finished() {
            self.setup = false;
        }
        Turn::Played {
            card,
            track,
            row,
            played,
        }
    }
}

/// The player's cards for the save: `caravancard <0|1> <form>` (1 in the
/// deck) and `caravanrecord` with the five counts.
pub(crate) fn save_lines(state: &GameState, line: &mut dyn FnMut(String)) {
    let c = &state.caravan;
    for (active, list) in [(0, &c.inactive), (1, &c.active)] {
        for f in list {
            line(format!("caravancard {active} {:08X}", f.0));
        }
    }
    if c.cap_winnings + c.cap_losses + c.winnings + c.losses + c.largest_winning > 0 {
        line(format!(
            "caravanrecord {} {} {} {} {}",
            c.cap_winnings, c.cap_losses, c.winnings, c.losses, c.largest_winning
        ));
    }
}

pub(crate) fn load_line(state: &mut GameState, raw: &str) -> Option<Result<(), String>> {
    let parts: Vec<&str> = raw.split_whitespace().collect();
    let bad = || format!("can't read '{raw}'");
    match *parts.first()? {
        "caravancard" => {
            let form = parts.get(2).and_then(|s| u32::from_str_radix(s, 16).ok());
            let Some(form) = form.map(FormId) else {
                return Some(Err(bad()));
            };
            match parts.get(1) {
                Some(&"1") => state.caravan.active.push(form),
                _ => state.caravan.inactive.push(form),
            }
            Some(Ok(()))
        }
        "caravanrecord" => {
            let n: Vec<u32> = parts[1..].iter().filter_map(|s| s.parse().ok()).collect();
            if n.len() != 5 {
                return Some(Err(bad()));
            }
            let c = &mut state.caravan;
            (
                c.cap_winnings,
                c.cap_losses,
                c.winnings,
                c.losses,
                c.largest_winning,
            ) = (n[0], n[1], n[2], n[3], n[4]);
            Some(Ok(()))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(value: i32, suit: i32) -> Card {
        Card {
            form: FormId(0x100 + (value * 10 + suit) as u32),
            value,
            suit,
        }
    }

    /// A game with these hands, no decks, past the start.
    fn game(player: Vec<Card>, npc: Vec<Card>) -> Game {
        let mut g = Game::new(Vec::new(), Vec::new(), &mut |_| 0);
        g.player_hand = player;
        g.npc_hand = npc;
        g.setup = false;
        g
    }

    /// `0074f6c0`: number cards on one's own tracks, new rows, with the
    /// track's way or its suit; face cards on a number card with room;
    /// queens on the last row only.
    #[test]
    fn placing() {
        let mut g = game(
            vec![c(5, 1), c(5, 2), c(3, 2), c(7, 3), c(QUEEN, 4), c(KING, 1)],
            vec![],
        );
        g.tracks[0][0].push(c(2, 1));
        g.update_track_value(0);
        // A new row only, on the player's own tracks.
        assert!(g.is_valid_placement(0, 1, 0, false));
        assert!(!g.is_valid_placement(0, 0, 0, false));
        assert!(!g.is_valid_placement(0, 2, 0, false));
        assert!(!g.is_valid_placement(3, 0, 0, false));
        g.tracks[0][1].push(c(5, 1));
        g.update_track_value(0);
        assert_eq!(
            g.info[0],
            TrackInfo {
                direction: 1,
                suit: 1,
                value: 7
            }
        );
        // Up now: a lower card only in the track's suit; never the same
        // value.
        assert!(!g.is_valid_placement(0, 2, 1, false));
        assert!(!g.is_valid_placement(0, 2, 2, false));
        assert!(g.is_valid_placement(0, 2, 3, false));
        // Faces: on a number card; queens not under another row.
        assert!(g.is_valid_placement(0, 0, 5, false));
        assert!(!g.is_valid_placement(0, 0, 4, false));
        assert!(g.is_valid_placement(0, 1, 4, false));
        assert!(!g.is_valid_placement(0, 2, 5, false));
        // On the opponent's tracks too.
        g.tracks[4][0].push(c(9, 3));
        assert!(g.is_valid_placement(4, 0, 5, false));
        // Starting the caravans: number cards on empty rows.
        g.setup = true;
        assert!(g.is_valid_placement(1, 0, 2, false));
        assert!(!g.is_valid_placement(1, 0, 4, false));
    }

    /// `0074ee70`: kings double their row; a queen on the last row turns
    /// the way round and takes her suit.
    #[test]
    fn values_kings_and_queens() {
        let mut g = game(vec![], vec![]);
        g.tracks[1][0] = vec![c(4, 1), c(KING, 2), c(KING, 3)];
        g.tracks[1][1] = vec![c(6, 2)];
        g.update_track_value(1);
        assert_eq!(
            g.info[1],
            TrackInfo {
                direction: 1,
                suit: 2,
                value: 22
            }
        );
        g.tracks[1][1].push(c(QUEEN, 4));
        g.update_track_value(1);
        assert_eq!(
            g.info[1],
            TrackInfo {
                direction: -1,
                suit: 4,
                value: 22
            }
        );
        g.discard_track(1);
        assert_eq!(g.info[1], TrackInfo::default());
        assert_eq!(winning_state(20), -1);
        assert_eq!(winning_state(26), 0);
        assert_eq!(winning_state(27), 1);
    }

    /// States 13, 17 to 19: a jack takes its row (the rows after move up);
    /// a joker on a 2..10 takes every other row of that value, on an ace
    /// every other row of that suit.
    #[test]
    fn jacks_and_jokers() {
        let mut g = game(vec![c(JACK, 1), c(JOKER, 5), c(JOKER, 5)], vec![]);
        g.tracks[0][0] = vec![c(3, 1)];
        g.tracks[0][1] = vec![c(7, 2), c(KING, 1)];
        g.tracks[0][2] = vec![c(9, 1)];
        g.tracks[5][0] = vec![c(7, 4)];
        g.tracks[5][1] = vec![c(ACE, 2)];
        let jack = g.play(false, 0, 0, 1, &mut |_| 0);
        assert_eq!(jack, Played::Jack { track: 0, row: 1 });
        g.resolve(&jack);
        assert_eq!(g.tracks[0][1], vec![c(9, 1)]);
        assert!(g.tracks[0][2].is_empty());
        assert_eq!(g.info[0].value, 12);
        // A joker on the 9: other 9s go (none) — then on the opponent's 7.
        g.tracks[0][2] = vec![c(7, 3)];
        let joker = g.play(false, 0, 5, 0, &mut |_| 0);
        assert_eq!(
            joker,
            Played::Joker {
                rows: vec![vec![2], vec![], vec![], vec![], vec![], vec![]]
            }
        );
        g.resolve(&joker);
        assert_eq!(g.rows_used(0), 2);
        assert_eq!(g.tracks[5][0].len(), 2);
        // On an ace: the spades go.
        g.tracks[2][0] = vec![c(4, 2)];
        let joker = g.play(false, 0, 5, 1, &mut |_| 0);
        g.resolve(&joker);
        assert!(g.tracks[2][0].is_empty());
        assert_eq!(g.tracks[5][1][0], c(ACE, 2));
    }

    /// `IsGameOver`: three pairs decided, more won wins; a side out of cards
    /// loses.
    #[test]
    fn the_end() {
        let mut g = game(vec![c(2, 1)], vec![c(2, 2)]);
        g.npc_deck = vec![c(3, 3)];
        // 21, 26 and 22 against 20, 25 and 23.
        g.tracks[0][0] = vec![c(10, 1), c(KING, 1)];
        g.tracks[0][1] = vec![c(ACE, 1)];
        g.tracks[5][0] = vec![c(10, 2), c(KING, 2)];
        g.tracks[1][0] = vec![c(10, 1), c(KING, 1)];
        g.tracks[1][1] = vec![c(6, 1)];
        g.tracks[4][0] = vec![c(10, 2), c(KING, 2)];
        g.tracks[4][1] = vec![c(5, 2)];
        g.tracks[2][0] = vec![c(10, 3), c(KING, 3)];
        g.tracks[2][1] = vec![c(2, 3)];
        g.tracks[3][0] = vec![c(10, 4), c(KING, 4)];
        g.tracks[3][1] = vec![c(3, 4)];
        assert!(g.is_game_over());
        assert!(g.player_won);
        // A tie in a pair leaves it undecided: the game goes on.
        g.tracks[3][1] = vec![c(2, 4)];
        assert!(!g.is_game_over());
        // The player with no cards left loses.
        g.player_hand.clear();
        assert!(g.is_game_over());
        assert!(!g.player_won);
    }

    /// `PrepareGameMenu`: eight each, dealt at random; playing draws the
    /// picked next card to the end of the hand (not while starting).
    #[test]
    fn dealing_and_drawing() {
        let deck: Vec<Card> = (1..=10).map(|v| c(v, 1)).collect();
        let mut g = Game::new(deck.clone(), deck, &mut |_| 0);
        assert_eq!(g.player_hand.len(), 8);
        // Each pick takes the first card, the last moving into its place
        // (`009a4320`).
        let values: Vec<i32> = g.player_hand.iter().map(|c| c.value).collect();
        assert_eq!(values, vec![1, 10, 9, 8, 7, 6, 5, 4]);
        assert_eq!(g.player_deck, vec![c(3, 1), c(2, 1)]);
        assert_eq!(g.next_player, Some(0));
        assert!(g.setup);
        assert_eq!(g.setup_track(false), Some(0));
        assert_eq!(g.setup_track(true), Some(5));
        g.play(false, 0, 0, 0, &mut |_| 0);
        assert_eq!(g.player_hand.len(), 7);
        g.setup = false;
        g.play(false, 0, 1, 0, &mut |_| 0);
        // Draws keep the deck's order (`006bf8f0`).
        assert_eq!(g.player_hand.last(), Some(&c(3, 1)));
        assert_eq!(g.player_deck, vec![c(2, 1)]);
        g.discard(false, 0, &mut |_| 0);
        assert!(g.player_deck.is_empty());
        assert_eq!(g.next_player, None);
    }
}
