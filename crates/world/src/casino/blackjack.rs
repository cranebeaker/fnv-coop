//! Blackjack (`BlackJackMenu`, class 1081, `menus\black_jack_menu.xml`;
//! `ShowBlackJackMenuParams`), as FalloutNV.exe 1.4.0.525 runs it (the Xbox
//! 360 prototype's PDB naming the parts). The 3D, the tiles and the
//! player's things are the caller's: [`Blackjack`] says what to do as
//! [`Effect`]s and keeps what the tiles show.
//!
//! - The shoe (`InitCasinoData` `00731d70`): the casino's decks (0 means 1)
//!   of 52 cards each, never shuffled: a "shuffle" (`007399d0`) only puts
//!   the count of cards left back to all of them, idle below the casino's
//!   share (0 means 0.25) and before a card with fewer than 5 left.
//! - A card (`DealCard` `00739090`): `rand(0, N)` swapped to the end of the
//!   live cards; two come off the count per card (the one dealt and the
//!   one after it). With luck (the dealer at Luck 4 or less, the player at
//!   6 or more) |Luck − 5| draws are looked at and the one that brings the
//!   hand closest to 21 (or to 11 below 10) taken, swapped with the card
//!   one past the live ones.
//! - Dealing (`DealHand` `007393f0`): the dealer, the player (plain), the
//!   dealer, the player (with luck); the dealer's first card face down.
//!   Twenty-one at once is blackjack; the dealer's twenty-one beats it but
//!   for a draw (no insurance). Doubling on any first two cards with the
//!   chips for it, splitting a pair of equal cards (10 and king don't pair)
//!   once, surrendering before any other move.
//! - The dealer (`DoIdle` state 11) hits under 17, and on 17 with an ace
//!   in the hand unless the casino's `bBJ_DealerHoleCard` byte (held on
//!   soft 17) is set, to seven cards.
//! - Paid (state 12): a win the bet, blackjack `trunc(bet × payout +
//!   0.51)` (a split hand's `+ 0.5`), surrender loses `trunc(bet / 2 +
//!   0.51)`; the bet isn't taken at the deal, only the net moves.
//! - The states (`DoIdle` `00734db0`; seconds in a state kept only while
//!   the menu is on top, a frame's step over a second counted as none):
//!   dealing 1, a card 2, splitting 3, the player's turn 4, clearing the
//!   table 7, the hole card turned 8, the dealer's card 9, idle 10, the
//!   dealer's turn 11, the result 12, closing 13.
//! - Clicks (`DoClick` `00733ff0`, only idle or in the player's turn): Hit
//!   3, Deal or Double 4, Increase Bet or Split 5, Decrease Bet or Switch
//!   Hands 6, Bet Max or Surrender 7, Exit or Stay 8.

use super::{CasinoData, Dice};

/// The states (`BlackJackState`; 0, 5 and 6 are never set).
pub mod state {
    pub const INTRO: u8 = 0;
    pub const DEALING: u8 = 1;
    pub const HIT: u8 = 2;
    pub const SPLIT: u8 = 3;
    pub const PLAYER_TURN: u8 = 4;
    pub const CLEAN_TABLE: u8 = 7;
    pub const DEALER_FLIP: u8 = 8;
    pub const DEALER_HIT: u8 = 9;
    pub const IDLE: u8 = 10;
    pub const DEALER_TURN: u8 = 11;
    pub const DETERMINE_WINNER: u8 = 12;
    pub const CLOSING: u8 = 13;
}

/// The tiles by `id` (`TileIndex`).
pub mod tile {
    pub const CURRENT_BET: i32 = 0;
    pub const CHIP_COUNT: i32 = 1;
    pub const CASINO_INFO: i32 = 2;
    pub const HIT: i32 = 3;
    pub const DOUBLE_DEAL: i32 = 4;
    pub const SPLIT_INCREASE_BET: i32 = 5;
    pub const SWITCH_DECREASE_BET: i32 = 6;
    pub const SURRENDER_MAX_BET: i32 = 7;
    pub const STAY_EXIT: i32 = 8;
    pub const STATUS: i32 = 9;
}

/// A hand's result (`HandResults`).
pub mod result {
    pub const UNKNOWN: u8 = 0;
    pub const LOSE: u8 = 1;
    pub const DRAW: u8 = 2;
    pub const WIN: u8 = 3;
    pub const BUST: u8 = 4;
    pub const BLACKJACK: u8 = 5;
    pub const SURRENDER: u8 = 6;
}

/// `iFlags` (`enumFlags`).
pub mod flag {
    pub const DISABLE_SURRENDER: u32 = 0x1;
    pub const CAN_SPLIT: u32 = 0x4;
    pub const SPLIT: u32 = 0x8;
    pub const SPLIT_HAND_ACTIVE: u32 = 0x10;
    pub const BLACKJACK: u32 = 0x20;
    pub const CAN_DOUBLE_MAIN: u32 = 0x40;
    pub const DOUBLED_MAIN: u32 = 0x80;
    /// Never set on PC: `CanDoubleDown(true)` sets [`DOUBLED_SPLIT`].
    pub const CAN_DOUBLE_SPLIT: u32 = 0x100;
    pub const DOUBLED_SPLIT: u32 = 0x200;
    pub const DISABLE_MAIN: u32 = 0x400;
    pub const DISABLE_SPLIT: u32 = 0x800;
    pub const DEALERS_TURN: u32 = 0x1000;
    pub const PLAYER_BUST: u32 = 0x2000;
    pub const SPLIT_ANIMATING: u32 = 0x4000;
}

/// Cards in a deck, the most decks, the most cards in a hand.
pub const DECK: usize = 52;
pub const MAX_DECKS: usize = 4;
pub const MAX_CARDS: usize = 7;
/// "Blackjack Games Played", the misc statistic a round counts (0x29).
pub const GAMES_PLAYED_STAT: u8 = 0x29;
/// The hands' models (`InitCasinoData`), children 1 to 3 of the scene.
pub const HAND_MODELS: [&str; 3] = [
    "meshes\\terminals\\nv_blackjack\\nv_blackjack-hand1.nif",
    "meshes\\terminals\\nv_blackjack\\nv_blackjack-hand2.nif",
    "meshes\\terminals\\nv_blackjack\\nv_blackjack-dealer.nif",
];
/// A tile's text alpha, full and dimmed (`SetTileTextAlpha` `00738c30`).
pub const ALPHA_FULL: f32 = 255.0;
pub const ALPHA_DIM: f32 = 128.0;

/// A card (`CardData`): its value (`CardValue`: 0 the two … 8 the ten, 9
/// jack, 10 queen, 11 king, 12 ace; 14 none) and suit (`CardSymbol`: 0
/// hearts, 1 clubs, 2 diamonds, 3 spades; 5 none), and where its faces
/// come from: the deck (0 to 3) and its number in the deck (0 to 51).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    pub value: u8,
    pub suit: u8,
    pub deck: u8,
    pub face: u8,
}

impl Card {
    /// A slot the casino's decks don't fill (`00731710`).
    pub const EMPTY: Card = Card {
        value: 14,
        suit: 5,
        deck: 0,
        face: 0,
    };

    /// Card `i` of deck `d` (`00731d70`): value `(i + 12) mod 13` (the ace
    /// first), suit `i / 13`.
    pub fn new(deck: usize, i: usize) -> Card {
        Card {
            value: ((i + 12) % 13) as u8,
            suit: (i / 13) as u8,
            deck: deck as u8,
            face: i as u8,
        }
    }

    pub fn is_empty(self) -> bool {
        self.value == 14
    }
}

/// A card's count (`00738fb0`): 2 to 10, the faces 10, the ace 11 (an
/// empty slot 16).
pub fn numeric(value: u8) -> i32 {
    let v = i32::from(value);
    if v > 8 {
        if v < 12 {
            return 10;
        }
        if v == 12 {
            return 11;
        }
    }
    v + 2
}

/// A hand's total (`GetHandValue` `00738ff0`): aces back to 1 while over
/// 21.
pub fn hand_value(cards: &[Card]) -> i32 {
    let mut sum: i32 = cards.iter().map(|c| numeric(c.value)).sum();
    let mut aces = cards.iter().filter(|c| c.value == 12).count();
    while sum > 21 && aces > 0 {
        sum -= 10;
        aces -= 1;
    }
    sum
}

/// A deck's textures from the casino's card back (texture 7 + deck, e.g.
/// `terminals\NV_PlayingCards\DeckG\cardG_Back.dds`): the folder after the
/// second `\`, the prefix the file's name up to `_` (`00731d70`).
/// Returns the back and the 52 faces (`<prefix>_<h|c|d|s><01..13>.dds`, 01
/// the ace), under `Textures\`.
pub fn deck_textures(back: &str) -> Option<(String, Vec<String>)> {
    let mut parts = back.split('\\');
    parts.next()?;
    parts.next()?;
    let folder = parts.next()?;
    let file = parts.next()?;
    let prefix = file.split('_').next()?;
    let base = format!("Textures\\Terminals\\NV_PlayingCards\\{folder}\\");
    let faces = (0..DECK)
        .map(|i| {
            let suit = b"hcds"[i / 13] as char;
            format!("{base}{prefix}_{suit}{:02}.dds", i % 13 + 1)
        })
        .collect();
    Some((format!("{base}{prefix}_Back.dds"), faces))
}

/// The hands (`SwapCardTextures`' first number; the scene's children 1 to
/// 3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Hand {
    Player,
    Split,
    Dealer,
}

impl Hand {
    fn index(self) -> usize {
        match self {
            Hand::Player => 0,
            Hand::Split => 1,
            Hand::Dealer => 2,
        }
    }

    /// The hand's model.
    pub fn model(self) -> Model {
        match self {
            Hand::Player => Model::Player,
            Hand::Split => Model::Split,
            Hand::Dealer => Model::Dealer,
        }
    }

    /// Card `i`'s (1 to 7) face and back shapes (`007396e0`).
    pub fn card_shapes(self, i: usize) -> (String, String) {
        match self {
            Hand::Dealer => (format!("Dealer_0{i}:0"), format!("Dealer_0{i}_Back:0")),
            _ => {
                let n = if self == Hand::Player { 1 } else { 2 };
                (
                    format!("Hand{n}_0{i}:{}", i - 1),
                    format!("Hand{n}_0{i}_Back:0"),
                )
            }
        }
    }

    /// Card `i`'s shadow shape (`UpdateCardAlpha` `00737e50`).
    pub fn shadow_shape(self, i: usize) -> String {
        match self {
            Hand::Dealer => format!("Dealer_0{i}_Shadow:0"),
            Hand::Player => format!("Hand1_0{i}_Shadow:0"),
            Hand::Split => format!("Hand2_0{i}_Shadow:0"),
        }
    }
}

/// The scene's models: the table, the three hands, the six chip stacks and
/// their shadows (children 0, 1 to 3, then 4 + 2i and 5 + 2i).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Model {
    Table,
    Player,
    Split,
    Dealer,
    Chip(usize),
    Shadow(usize),
}

/// A sequence by its place in the menu's arrays, which lie one after the
/// other (`pTableAnimations` +0xE04, `pPlayerAnimations` +0xE0C,
/// `pPlayerSplitAnimations` +0xE2C, `pDealerAnimations` +0xE4C): an index
/// past a hand's eight reads the next array's.
fn hand_sequence(hand: Hand, k: usize) -> Option<String> {
    let flat = match hand {
        Hand::Player => k,
        Hand::Split => 8 + k,
        Hand::Dealer => 16 + k,
    };
    let (prefix, k) = match flat {
        0..=7 => ("Hand1", flat),
        8..=15 => ("Hand2", flat - 8),
        16..=24 => ("Dealer", flat - 16),
        _ => return None,
    };
    Some(match (prefix, k) {
        (_, 0) => format!("{prefix}_Discard"),
        ("Dealer", 8) => "Dealer_Reveal".into(),
        _ => format!("{prefix}_0{k}"),
    })
}

/// The table's sequences.
pub mod sequence {
    pub const PLAYERS_DISCARD: &str = "Players_Discard";
    pub const DEALERS_DISCARD: &str = "Dealers_Discard";
}

/// Sequence lengths from the caller's models (`NiControllerSequence`'s
/// end, `00508100`); `None` for a sequence the model doesn't have (the
/// menu's pointer null).
pub trait Clips {
    fn end(&self, model: Model, sequence: &str) -> Option<f32>;
}

/// What the caller does for the menu.
#[derive(Debug, Clone, PartialEq)]
pub enum Effect {
    /// A sound by its editor ID.
    Sound(&'static str),
    /// `ActivateSequence` (weight 1), `DeactivateSequence`, every one off.
    Activate {
        model: Model,
        sequence: String,
    },
    Deactivate {
        model: Model,
        sequence: String,
    },
    DeactivateAll(Model),
    /// A model updated at a time (its running sequences pose it).
    Update {
        model: Model,
        time: f32,
    },
    /// Every model updated at a time.
    UpdateAll(f32),
    /// A hand's card `index` (1 to 7) shows `card` (the shoe's slot):
    /// its face and its deck's back (`SwapCardTextures` `007396e0`).
    SwapCard {
        hand: Hand,
        index: usize,
        card: usize,
    },
    /// Each hand's cards from its number in `next` on hidden (face, back
    /// and shadow; `UpdateCardAlpha` `00737e50`, the cards not dealt still
    /// on the deck), in player, split, dealer order.
    HideUndealt([usize; 3]),
    /// Every card shown again (`ResetCardAlpha` `00738270`).
    ShowAllCards,
    /// The split hand on the felt or under it (`SetSideHandVisibility`
    /// `00739970`: the model's own z 0, else −5).
    SideHand(bool),
    /// The arrow by the hand being played (`HighlightSelectedDeck`
    /// `007399f0`: `Hand1_Arrow:0` for the main hand, `Hand1_Arrow:0@#2`
    /// for the split one), or both hidden (`ClearDeckHighlight`).
    Highlight(Option<Hand>),
    /// The chip stacks for a bet (`SetBetChips` `00739b30`,
    /// [`bet_chips`]).
    BetChips(i32),
    /// A round counted in "Blackjack Games Played".
    GamePlayed,
    /// The player's line for the casino changed (the winnings).
    Data(CasinoData),
    /// Out of chips while idle: `sGamblingBrokeText` as a corner message
    /// (the surprised Vault Boy, `UIPopUpMessageGeneral`); the menu closes.
    Broke,
    /// The closing state: settle the chips (`casino::settle` with
    /// [`Blackjack::chips`] and [`Blackjack::new_level`]) and close.
    Close,
}

/// The status line (tile 9).
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    /// A round's net: won, lost (with the lucky or unlucky words), or even
    /// (`sYouBreakEvenText`).
    Won {
        amount: i32,
        lucky: bool,
    },
    Lost {
        amount: i32,
        unlucky: bool,
    },
    Even,
    /// "You've reached the max hand size" (`01070cc0`).
    MaxHand,
    /// Switching hands empties it.
    Empty,
}

/// The chip stacks' heights for a bet (`SetBetChips` `00739b30`): each of
/// the six stacks (1, 5, 10, 25, 100, 500) raised `n × 0.18` above the felt,
/// and whether each stack's shadow is hidden (lowered 5).
pub fn bet_chips(b: i32) -> ([f32; 6], [bool; 6]) {
    let c = 0.18f32;
    let f = |n: i32| n as f32 * c;
    let mut hide = [false; 6];
    let z = if b < 7 {
        hide = [false, true, true, true, true, true];
        [f(b), 0.0, 0.0, 0.0, 0.0, 0.0]
    } else if b < 40 {
        hide = [false, b < 11, true, true, true, true];
        [f(b % 5 + 5), f(b / 5 - 1), 0.0, 0.0, 0.0, 0.0]
    } else if b < 101 {
        hide[3..].fill(true);
        [0.9, 0.9, f(b / 10 - 3), 0.0, 0.0, 0.0]
    } else if b < 301 {
        hide = [false, false, false, b / 25 == 4, true, true];
        [1.8, 1.44, f((b % 25) / 10 + 5), f(b / 25 - 4), 0.0, 0.0]
    } else if b < 501 {
        hide[5] = true;
        [1.8, 1.44, 0.9, f(b / 25 - 12), 0.36, 0.0]
    } else if b < 1000 {
        // The 100s' own test is overwritten by a later 0.
        hide[5] = true;
        [1.8, 1.8, 1.62, f((b % 100) / 25 + 6), f(b / 100 - 3), 0.0]
    } else if b < 1500 {
        [1.8, 1.8, 1.62, 1.8, f(b / 100 - 9), 0.18]
    } else {
        hide[5] = b == 1500;
        [1.8, 1.8, 1.8, 1.8, 1.8, f((b / 500 - 3).min(10))]
    };
    (z, hide)
}

/// A bet up a step (`DoClick` 5 idle): + 1 under 10, 5 under 40, 10 under
/// 100, 25 under 400, 100 under 1500, else 500.
pub fn step_up(bet: i32) -> i32 {
    bet + match bet {
        b if b < 10 => 1,
        b if b < 40 => 5,
        b if b < 100 => 10,
        b if b < 400 => 25,
        b if b < 1500 => 100,
        _ => 500,
    }
}

/// A bet down a step (`DoClick` 6 idle): − 1 under 11; then down to the
/// step's multiple (a whole step when on one): 5 under 41, 10 under 101,
/// 25 under 401, 100 under 1501, else 500.
pub fn step_down(bet: i32) -> i32 {
    let step = match bet {
        b if b < 11 => return b - 1,
        b if b < 41 => 5,
        b if b < 101 => 10,
        b if b < 401 => 25,
        b if b < 1501 => 100,
        _ => 500,
    };
    if bet % step == 0 {
        bet - step
    } else {
        bet / step * step
    }
}

/// The blackjack menu.
#[derive(Debug, Clone)]
pub struct Blackjack {
    pub max_winnings: i32,
    pub min_bet: i32,
    pub max_bet: i32,
    pub state: u8,
    /// Seconds in the state (`ftotalStateSecs`), the tick last updated
    /// (`uiPrevIdleTick`) and the state then.
    pub elapsed: f32,
    last_tick: Option<u32>,
    last_state: u8,
    /// Whether the menu is on top (a message over it stops its clock).
    pub on_top: bool,
    /// `fBlackJackWinRatio`, `fBlackJackShuffle`, `iNumOfDecks`,
    /// `bHoldOnSoft17`.
    pub payout: f32,
    pub shuffle: f32,
    pub decks: usize,
    pub hold_soft_17: bool,
    pub luck: i32,
    pub bet: i32,
    pub split_bet: i32,
    /// The menu's chips (settled on closing).
    pub chips: i32,
    pub good_luck: bool,
    pub bad_luck: bool,
    /// A round reached a new level: the comps quest starts on closing.
    pub new_level: bool,
    /// The next sequence of each hand (`iAnimIndex`: player, split,
    /// dealer).
    pub anim: [usize; 3],
    /// The shoe (`pDeckOfCards`, 4 × 52) and the cards left in it.
    pub shoe: Vec<Card>,
    pub left: usize,
    /// The hands: slots in the shoe (the game's lists of pointers into it).
    pub dealer: Vec<usize>,
    pub player: Vec<usize>,
    pub split: Vec<usize>,
    pub results: [u8; 2],
    pub flags: u32,
    /// Sound toggle and phase of the dealing and clearing (`bCleanDealerHand`).
    toggle: bool,
    /// The clearing's second phase start (`fStartKeyTime`).
    start_key: f32,
    /// The dealing's next step (`animDelay`).
    anim_delay: f32,
    pub data: CasinoData,
    /// What the tiles show: the bet line (tile 0's number), the buttons'
    /// labels (betting or playing: `SetBetTiles`), each tile's text alpha,
    /// the status line.
    pub bet_shown: i32,
    pub betting_labels: bool,
    pub alpha: [f32; 10],
    pub status: Option<Status>,
    pub status_shown: bool,
}

impl Blackjack {
    /// `Create` (`00733630`) after its checks, with `InitCasinoData`
    /// (`00731d70`): the casino's payout (0 means 1.5), shuffle point (0
    /// means 0.25), decks (0 means 1) and hold-on-soft-17 byte; Luck read
    /// once; the bet at the least. Starts with `GAMEBlackJackCardShuffle`,
    /// the betting labels and the bet's chips, then idle.
    #[allow(clippy::too_many_arguments)]
    pub fn open(
        payout: f32,
        shuffle: f32,
        decks: i32,
        hold_soft_17: bool,
        max_winnings: i32,
        min_bet: i32,
        max_bet: i32,
        chips: i32,
        luck: f32,
        data: CasinoData,
    ) -> (Blackjack, Vec<Effect>) {
        let decks = if decks == 0 { 1 } else { decks.clamp(1, 4) } as usize;
        // One slot past the four decks: a lucky card swaps with the slot
        // one past the live cards, which after a reshuffle at four decks
        // is past the game's array (never reached in vanilla: The Tops'
        // reshuffle point keeps 73 cards for a round).
        let mut shoe = vec![Card::EMPTY; MAX_DECKS * DECK + 1];
        for d in 0..decks {
            for i in 0..DECK {
                shoe[d * DECK + i] = Card::new(d, i);
            }
        }
        let m = Blackjack {
            max_winnings,
            min_bet,
            max_bet,
            state: state::IDLE,
            elapsed: 0.0,
            last_tick: None,
            last_state: 0,
            on_top: true,
            payout: if payout == 0.0 { 1.5 } else { payout },
            shuffle: if shuffle == 0.0 { 0.25 } else { shuffle },
            decks,
            hold_soft_17,
            luck: luck.clamp(0.0, 10.0) as i32,
            bet: min_bet,
            split_bet: 0,
            chips,
            good_luck: false,
            bad_luck: false,
            new_level: false,
            anim: [1; 3],
            shoe,
            left: decks * DECK,
            dealer: Vec::new(),
            player: Vec::new(),
            split: Vec::new(),
            results: [0; 2],
            flags: 0,
            toggle: false,
            start_key: 0.0,
            anim_delay: 0.0,
            data,
            bet_shown: min_bet,
            betting_labels: true,
            alpha: [ALPHA_FULL; 10],
            status: None,
            status_shown: false,
        };
        let fx = vec![
            Effect::Sound("GAMEBlackJackCardShuffle"),
            Effect::BetChips(min_bet),
            Effect::SideHand(false),
            Effect::Highlight(None),
            Effect::UpdateAll(0.0),
        ];
        (m, fx)
    }

    fn has(&self, f: u32) -> bool {
        self.flags & f != 0
    }

    fn set(&mut self, f: u32, on: bool) {
        if on {
            self.flags |= f;
        } else {
            self.flags &= !f;
        }
    }

    fn cards(&self, hand: Hand) -> Vec<Card> {
        self.hand(hand).iter().map(|&i| self.shoe[i]).collect()
    }

    fn hand(&self, hand: Hand) -> &Vec<usize> {
        match hand {
            Hand::Player => &self.player,
            Hand::Split => &self.split,
            Hand::Dealer => &self.dealer,
        }
    }

    fn hand_mut(&mut self, hand: Hand) -> &mut Vec<usize> {
        match hand {
            Hand::Player => &mut self.player,
            Hand::Split => &mut self.split,
            Hand::Dealer => &mut self.dealer,
        }
    }

    /// A hand's total.
    pub fn value(&self, hand: Hand) -> i32 {
        hand_value(&self.cards(hand))
    }

    /// `Shuffle` (`007399d0`): the count back to every card.
    fn reshuffle(&mut self) {
        self.left = self.decks * DECK;
    }

    /// `DealCard(hand, bApplyLuck, bIsDealer)` (`00739090`): the slot of the
    /// card dealt.
    fn deal_card(&mut self, hand: Option<Hand>, luck: bool, dealer: bool, dice: Dice) -> usize {
        if self.left < 5 {
            self.reshuffle();
        }
        let lucky_path = match hand {
            Some(_) if luck => (dealer && self.luck <= 4) || (!dealer && self.luck >= 6),
            _ => false,
        };
        let pick = if !lucky_path {
            let n = self.left;
            self.left -= 1;
            dice(n)
        } else {
            let h = hand.map_or(0, |h| self.value(h));
            let tries = (self.luck - 5).unsigned_abs() as usize;
            let draws: Vec<usize> = (0..tries).map(|_| dice(self.left)).collect();
            let (mut best, mut to21, mut to11) = (h, h, h);
            let (mut bj, mut b11) = (0usize, 0usize);
            let mut pick = 0;
            for (j, &d) in draws.iter().enumerate() {
                let mut v = numeric(self.shoe[d].value);
                if v == 11 && h + 11 >= 22 {
                    v = 1;
                }
                let total = h + v;
                if h < 10 {
                    if 21 - total < to21 && total < 22 {
                        to21 = 21 - total;
                        bj = j;
                    }
                    if 11 - total < to11 && total < 12 {
                        to11 = 11 - total;
                        b11 = j;
                    }
                } else if best < total && total < 22 {
                    best = total;
                    bj = j;
                }
                // The game keeps `best` and `to21` in one variable (the
                // total above 9, the distance to 21 below 10).
                let near = if h < 10 { to21 } else { best };
                let mut chosen = None;
                if near < 4 {
                    chosen = Some(bj);
                } else if to11 < near && to11 < 2 {
                    chosen = Some(b11);
                } else {
                    pick = draws[bj];
                }
                if let Some(c) = chosen {
                    pick = draws[c];
                    if c != 0 {
                        if dealer {
                            self.bad_luck = true;
                        } else {
                            self.good_luck = true;
                        }
                    }
                }
            }
            pick
        };
        let end = self.left;
        self.shoe.swap(pick, end);
        self.left = self.left.saturating_sub(1);
        end
    }

    /// `DealHand` (`007393f0`): the results cleared; the dealer and the
    /// player a card each, then each a second with luck.
    fn deal_hand(&mut self, dice: Dice) {
        self.results = [result::UNKNOWN; 2];
        let c = self.deal_card(None, false, false, dice);
        self.dealer.insert(0, c);
        let c = self.deal_card(None, false, false, dice);
        self.player.insert(0, c);
        let c = self.deal_card(Some(Hand::Dealer), true, true, dice);
        self.dealer.push(c);
        let c = self.deal_card(Some(Hand::Player), true, false, dice);
        self.player.push(c);
    }

    /// `CanSplitHand` (`007395a0`): exactly two cards of the same value
    /// and the chips for two bets.
    fn can_split(&mut self) {
        let cards = self.cards(Hand::Player);
        let ok = cards.len() == 2 && cards[0].value == cards[1].value && self.bet * 2 <= self.chips;
        self.set(flag::CAN_SPLIT, ok);
    }

    /// `CanDoubleDown(split)` (`00739650`); the split hand's sets 0x200.
    fn can_double(&mut self, split: bool) {
        if split {
            let ok = self.chips >= self.bet + self.split_bet * 2;
            self.set(flag::DOUBLED_SPLIT, ok);
        } else {
            let ok = self.chips >= self.split_bet + self.bet * 2;
            self.set(flag::CAN_DOUBLE_MAIN, ok);
        }
    }

    fn text_alpha(&mut self, tile: i32, a: f32) {
        self.alpha[tile as usize] = a;
    }

    /// `UpdateTileAlpha(i)` (`00738560`): the buttons dimmed for what can't
    /// be done now (not while idle).
    fn update_tile_alpha(&mut self, i: i32) {
        if self.state == state::IDLE {
            return;
        }
        let a = |on: bool| if on { ALPHA_FULL } else { ALPHA_DIM };
        let split_active = self.has(flag::SPLIT_HAND_ACTIVE);
        match i {
            3 => {
                let off = if split_active {
                    self.has(flag::DISABLE_SPLIT)
                } else {
                    self.has(flag::DISABLE_MAIN)
                };
                self.text_alpha(3, a(!off));
            }
            4 => {
                let on = if split_active {
                    self.has(flag::CAN_DOUBLE_SPLIT)
                } else {
                    self.has(flag::CAN_DOUBLE_MAIN)
                };
                self.text_alpha(4, a(on));
            }
            5 => {
                let on = !split_active && self.has(flag::CAN_SPLIT);
                self.text_alpha(5, a(on));
            }
            6 => {
                let on = if split_active {
                    !self.has(flag::DISABLE_MAIN)
                } else {
                    !self.has(flag::DISABLE_SPLIT) && self.has(flag::SPLIT)
                };
                self.text_alpha(6, a(on));
            }
            7 => {
                let on = !self.has(flag::DISABLE_SURRENDER);
                self.text_alpha(7, a(on));
            }
            8 => self.text_alpha(8, ALPHA_FULL),
            10 => {
                if !split_active {
                    if !self.has(flag::DISABLE_MAIN) && !self.has(flag::BLACKJACK) {
                        self.text_alpha(3, ALPHA_FULL);
                        let v = a(self.has(flag::CAN_DOUBLE_MAIN));
                        self.text_alpha(4, v);
                        let v = a(self.has(flag::CAN_SPLIT));
                        self.text_alpha(5, v);
                        let v = a(!self.has(flag::DISABLE_SURRENDER));
                        self.text_alpha(7, v);
                    } else {
                        for t in [3, 4, 5, 7] {
                            self.text_alpha(t, ALPHA_DIM);
                        }
                    }
                    let stay = !self.has(flag::BLACKJACK)
                        || (self.has(flag::SPLIT) && !self.has(flag::DISABLE_SPLIT));
                    self.text_alpha(8, a(stay));
                    let switch = !self.has(flag::DISABLE_SPLIT) && self.has(flag::SPLIT);
                    self.text_alpha(6, a(switch));
                } else {
                    if !self.has(flag::DISABLE_SPLIT) {
                        self.text_alpha(3, ALPHA_FULL);
                        let v = a(self.has(flag::CAN_DOUBLE_SPLIT));
                        self.text_alpha(4, v);
                    } else {
                        self.text_alpha(3, ALPHA_DIM);
                        self.text_alpha(4, ALPHA_DIM);
                    }
                    self.text_alpha(5, ALPHA_DIM);
                    self.text_alpha(7, ALPHA_DIM);
                    self.text_alpha(8, ALPHA_FULL);
                    let v = a(!self.has(flag::DISABLE_MAIN));
                    self.text_alpha(6, v);
                }
            }
            _ => {}
        }
    }

    /// `SetBetTiles(bet)` (`00738cc0`): the buttons' labels for betting
    /// (Hit hidden, every label at full alpha) or playing (Hit shown).
    fn set_bet_tiles(&mut self, betting: bool) {
        self.betting_labels = betting;
        if betting {
            for t in 4..=8 {
                self.text_alpha(t, ALPHA_FULL);
            }
        }
    }

    fn close(&mut self) {
        self.state = state::CLOSING;
    }

    /// A tile clicked (`DoClick` `00733ff0`).
    pub fn click(&mut self, id: i32, dice: Dice) -> Vec<Effect> {
        let mut fx = Vec::new();
        let playing = self.state == state::PLAYER_TURN;
        let idle = self.state == state::IDLE;
        if !playing && !idle {
            return fx;
        }
        match id {
            tile::HIT if playing => {
                self.state = state::HIT;
                self.set(flag::DISABLE_SURRENDER, true);
                self.update_tile_alpha(7);
                self.set(flag::CAN_DOUBLE_SPLIT, false);
                self.set(flag::CAN_DOUBLE_MAIN, false);
                self.set(flag::CAN_SPLIT, false);
                let split_active = self.has(flag::SPLIT_HAND_ACTIVE);
                let target = if !split_active || self.has(flag::DISABLE_SPLIT) {
                    (!split_active && !self.has(flag::DISABLE_MAIN)).then_some(Hand::Player)
                } else {
                    Some(Hand::Split)
                };
                if let Some(hand) = target {
                    let c = self.deal_card(Some(hand), true, false, dice);
                    self.hand_mut(hand).push(c);
                    let total = self.value(hand);
                    let (disable, at) = if hand == Hand::Player {
                        (flag::DISABLE_MAIN, 0)
                    } else {
                        (flag::DISABLE_SPLIT, 1)
                    };
                    if total > 20 {
                        self.set(disable, true);
                        if total > 21 {
                            self.results[at] = result::BUST;
                        }
                    }
                    let double = if hand == Hand::Player {
                        flag::CAN_DOUBLE_MAIN
                    } else {
                        flag::CAN_DOUBLE_SPLIT
                    };
                    self.set(double, false);
                    if self.hand(hand).len() > 6 {
                        self.set(disable, true);
                        self.status = Some(Status::MaxHand);
                    }
                }
                self.update_tile_alpha(10);
            }
            tile::DOUBLE_DEAL if idle => {
                self.state = state::DEALING;
                self.flags = 0;
                self.bad_luck = false;
                self.good_luck = false;
                self.split_bet = 0;
                self.set(flag::CAN_DOUBLE_MAIN, true);
                for t in 4..=8 {
                    self.text_alpha(t, ALPHA_DIM);
                }
                self.deal_hand(dice);
            }
            tile::DOUBLE_DEAL => {
                if (!self.has(flag::SPLIT_HAND_ACTIVE) || !self.has(flag::CAN_DOUBLE_SPLIT))
                    && self.has(flag::CAN_DOUBLE_MAIN)
                {
                    self.bet <<= 1;
                    self.set(flag::DOUBLED_MAIN, true);
                    self.set(flag::DISABLE_MAIN, true);
                    let c = self.deal_card(Some(Hand::Player), true, false, dice);
                    self.player.push(c);
                    if !self.has(flag::DISABLE_MAIN) && self.has(flag::CAN_DOUBLE_MAIN) {
                        self.can_double(false);
                    }
                    self.state = state::HIT;
                    self.set(flag::DISABLE_SURRENDER, true);
                    self.bet_shown = self.split_bet + self.bet;
                    if self.value(Hand::Player) > 21 {
                        self.results[0] = result::BUST;
                    }
                    self.set(flag::CAN_DOUBLE_MAIN, false);
                }
                self.update_tile_alpha(10);
            }
            tile::SPLIT_INCREASE_BET if idle => {
                let before = self.bet;
                self.bet = step_up(self.bet);
                if self.bet > self.chips {
                    self.bet = self.chips;
                }
                if self.bet > self.max_bet {
                    self.bet = self.max_bet;
                }
                if before < self.bet {
                    fx.push(Effect::Sound("GAMEBlackJackIncreaseBet"));
                }
                fx.push(Effect::BetChips(self.bet));
                self.bet_shown = self.bet;
            }
            tile::SPLIT_INCREASE_BET if self.has(flag::CAN_SPLIT) => {
                self.set(flag::CAN_DOUBLE_SPLIT, false);
                self.set(flag::CAN_DOUBLE_MAIN, false);
                self.state = state::SPLIT;
                self.set(flag::DISABLE_SURRENDER, true);
                // The main hand's second card goes to the split hand.
                let second = self.player.get(1).copied().unwrap_or(0);
                self.split_bet = self.bet;
                self.split.insert(0, second);
                fx.push(Effect::SwapCard {
                    hand: Hand::Split,
                    index: 1,
                    card: second,
                });
                let c = self.deal_card(Some(Hand::Player), false, false, dice);
                if let Some(slot) = self.player.get_mut(1) {
                    *slot = c;
                }
                let c = self.deal_card(Some(Hand::Split), false, false, dice);
                self.split.push(c);
                // Both twenty-one tests read the main hand.
                let main = self.value(Hand::Player);
                let also_main = self.value(Hand::Player);
                if main == 21 {
                    self.results[0] = result::BLACKJACK;
                    self.set(flag::DISABLE_MAIN, true);
                }
                self.can_double(false);
                if also_main == 21 {
                    self.results[1] = result::BLACKJACK;
                    self.set(flag::DISABLE_SPLIT, true);
                }
                self.can_double(true);
                self.set(flag::SPLIT, true);
                self.set(flag::CAN_SPLIT, false);
                fx.push(self.highlight());
                self.update_tile_alpha(10);
                self.bet_shown = self.split_bet + self.bet;
            }
            tile::SWITCH_DECREASE_BET if idle => {
                let before = self.bet;
                self.bet = step_down(self.bet);
                if self.bet < self.min_bet {
                    self.bet = self.min_bet;
                }
                if self.bet < before {
                    fx.push(Effect::Sound("GAMEBlackJackDecreaseBet"));
                }
                fx.push(Effect::BetChips(self.bet));
                self.bet_shown = self.bet;
            }
            tile::SWITCH_DECREASE_BET if self.has(flag::SPLIT) => {
                if !self.has(flag::SPLIT_HAND_ACTIVE) || self.has(flag::DISABLE_MAIN) {
                    if !self.has(flag::SPLIT_HAND_ACTIVE) && !self.has(flag::DISABLE_SPLIT) {
                        self.set(flag::SPLIT_HAND_ACTIVE, true);
                    }
                } else {
                    self.set(flag::SPLIT_HAND_ACTIVE, false);
                }
                fx.push(self.highlight());
                self.status = Some(Status::Empty);
                self.update_tile_alpha(10);
            }
            tile::SURRENDER_MAX_BET if idle => {
                self.bet = self.max_bet;
                if self.bet > self.chips {
                    self.bet = self.chips;
                }
                fx.push(Effect::BetChips(self.bet));
                self.bet_shown = self.bet;
            }
            tile::SURRENDER_MAX_BET if !self.has(flag::DISABLE_SURRENDER) => {
                self.set(flag::DISABLE_MAIN, true);
                self.results[0] = result::SURRENDER;
                self.state = state::DEALER_FLIP;
                self.update_tile_alpha(10);
            }
            tile::STAY_EXIT if idle => self.close(),
            tile::STAY_EXIT => {
                self.state = state::DEALER_FLIP;
                self.update_tile_alpha(10);
            }
            _ => {}
        }
        fx
    }

    /// `HighlightSelectedDeck` (`007399f0`): the arrow by the hand played.
    fn highlight(&self) -> Effect {
        Effect::Highlight(Some(if self.has(flag::SPLIT_HAND_ACTIVE) {
            Hand::Split
        } else {
            Hand::Player
        }))
    }

    fn seq(hand: Hand, k: usize) -> Option<String> {
        hand_sequence(hand, k)
    }

    /// One update (`DoIdle`): `tick` the time in milliseconds.
    pub fn update(&mut self, tick: u32, clips: &dyn Clips, dice: Dice) -> Vec<Effect> {
        let mut fx = Vec::new();
        match self.last_tick {
            None => {
                self.elapsed = 0.0;
                self.last_state = 0;
            }
            Some(last) if self.on_top => {
                let mut dt = tick.wrapping_sub(last);
                if dt > 1000 {
                    dt = 0;
                }
                self.elapsed = if self.last_state == self.state {
                    self.elapsed + dt as f32 / 1000.0
                } else {
                    0.0
                };
            }
            _ => {}
        }
        self.last_tick = Some(tick);
        self.last_state = self.state;
        let t = self.elapsed;
        let end = |hand: Hand, k: usize| -> Option<f32> {
            Self::seq(hand, k).and_then(|s| clips.end(hand.model(), &s))
        };
        let activate = |fx: &mut Vec<Effect>, hand: Hand, k: usize| {
            if let Some(sequence) = Self::seq(hand, k) {
                fx.push(Effect::Activate {
                    model: hand.model(),
                    sequence,
                });
            }
        };
        let deactivate = |fx: &mut Vec<Effect>, hand: Hand, k: usize| {
            if let Some(sequence) = Self::seq(hand, k) {
                fx.push(Effect::Deactivate {
                    model: hand.model(),
                    sequence,
                });
            }
        };
        let update = |fx: &mut Vec<Effect>, model: Model, time: f32| {
            fx.push(Effect::Update { model, time });
        };
        match self.state {
            state::DEALING => self.dealing(t, &mut fx, &end, &activate, &deactivate, &update),
            state::HIT => {
                let split_active = self.has(flag::SPLIT_HAND_ACTIVE);
                let hand = if split_active {
                    Hand::Split
                } else {
                    Hand::Player
                };
                let k = self.anim[hand.index()];
                if let Some(length) = end(hand, k) {
                    if t > 1.0 {
                        self.status_shown = false;
                    }
                    update(&mut fx, Model::Table, t);
                    if t == 0.0 {
                        fx.push(Effect::Sound("GAMEBlackJackCardDeal"));
                        activate(&mut fx, hand, k);
                        // The hand's last card on its last place.
                        let cards = self.hand(hand);
                        if let Some(&last) = cards.last() {
                            fx.push(Effect::SwapCard {
                                hand,
                                index: cards.len(),
                                card: last,
                            });
                        }
                    }
                    update(&mut fx, hand.model(), t);
                    if length <= t {
                        deactivate(&mut fx, hand, k);
                        self.anim[hand.index()] += 1;
                        self.state = if self.has(flag::SPLIT_ANIMATING) {
                            state::SPLIT
                        } else if self.has(flag::DEALERS_TURN)
                            || (self.has(flag::SPLIT)
                                && self.has(flag::DISABLE_MAIN)
                                && self.has(flag::DISABLE_SPLIT))
                            || (!self.has(flag::SPLIT) && self.has(flag::DISABLE_MAIN))
                        {
                            state::DEALER_FLIP
                        } else {
                            state::PLAYER_TURN
                        };
                    }
                }
            }
            state::SPLIT => {
                if end(Hand::Split, 1).is_some()
                    && end(Hand::Split, 2).is_some()
                    && end(Hand::Player, 2).is_some()
                {
                    if self.anim[1] == 1 {
                        self.set(flag::SPLIT_ANIMATING, true);
                        fx.push(Effect::SideHand(true));
                        activate(&mut fx, Hand::Player, 2);
                        update(&mut fx, Model::Dealer, 0.0);
                        update(&mut fx, Model::Player, 0.0);
                        self.set(flag::SPLIT_HAND_ACTIVE, true);
                        self.anim[0] -= 1;
                        self.state = state::HIT;
                    } else if self.anim[0] == 2 {
                        self.set(flag::SPLIT_HAND_ACTIVE, false);
                        self.state = state::HIT;
                    } else if self.anim[1] == 2 {
                        self.set(flag::SPLIT_HAND_ACTIVE, true);
                        self.state = state::HIT;
                    } else {
                        self.set(flag::SPLIT_ANIMATING, false);
                        self.set(flag::SPLIT_HAND_ACTIVE, false);
                        self.state = if self.results == [result::BLACKJACK; 2] {
                            state::DETERMINE_WINNER
                        } else {
                            state::PLAYER_TURN
                        };
                    }
                }
            }
            state::PLAYER_TURN => {
                if self.results[0] != result::UNKNOWN
                    && (!self.has(flag::SPLIT) || self.results[1] != result::UNKNOWN)
                {
                    self.state = state::DEALER_FLIP;
                }
                if self.has(flag::DISABLE_MAIN)
                    && (!self.has(flag::SPLIT) || self.has(flag::DISABLE_SPLIT))
                {
                    self.state = state::DEALER_FLIP;
                }
            }
            state::CLEAN_TABLE => self.clean_table(t, &mut fx, &end, &activate, &deactivate),
            state::DEALER_FLIP => {
                if t > 1.0 {
                    self.status_shown = false;
                }
                match end(Hand::Dealer, 8) {
                    None => self.state = state::DEALER_TURN,
                    Some(length) => {
                        update(&mut fx, Model::Table, t);
                        if t == 0.0 {
                            self.set(flag::DISABLE_MAIN, true);
                            self.set(flag::DISABLE_SPLIT, true);
                            fx.push(Effect::Highlight(None));
                            fx.push(Effect::Sound("GAMEBlackJackCardFlip"));
                            activate(&mut fx, Hand::Dealer, 8);
                        }
                        if t < length {
                            update(&mut fx, Model::Dealer, t);
                        }
                        if length <= t {
                            deactivate(&mut fx, Hand::Dealer, 8);
                            self.state = state::DEALER_TURN;
                        }
                    }
                }
            }
            state::DEALER_HIT => {
                if t > 1.0 {
                    self.status_shown = false;
                }
                let d = self.anim[2];
                let next = |m: &Blackjack| {
                    if m.value(Hand::Dealer) < 22 && !m.has(flag::PLAYER_BUST) {
                        state::DEALER_TURN
                    } else {
                        state::DETERMINE_WINNER
                    }
                };
                match end(Hand::Dealer, d) {
                    None => self.state = next(self),
                    Some(length) => {
                        update(&mut fx, Model::Table, t);
                        if t == 0.0 {
                            fx.push(Effect::Sound("GAMEBlackJackCardFlip"));
                            fx.push(Effect::Highlight(None));
                            if let Some(&last) = self.dealer.last() {
                                fx.push(Effect::SwapCard {
                                    hand: Hand::Dealer,
                                    index: self.dealer.len(),
                                    card: last,
                                });
                            }
                            activate(&mut fx, Hand::Dealer, d);
                        }
                        if t < length {
                            update(&mut fx, Model::Dealer, t);
                        }
                        if length <= t {
                            deactivate(&mut fx, Hand::Dealer, d);
                            self.anim[2] += 1;
                            self.state = next(self);
                        }
                    }
                }
            }
            state::IDLE => {
                if (self.left as f32) < (self.decks * DECK) as f32 * self.shuffle {
                    self.reshuffle();
                }
                if t > 1.0 {
                    self.status_shown = false;
                }
                if self.chips < self.bet {
                    self.bet = self.chips;
                    self.bet_shown = self.bet;
                }
                if self.chips < self.min_bet {
                    fx.push(Effect::Broke);
                    self.close();
                }
            }
            state::DEALER_TURN => {
                self.set(flag::DEALERS_TURN, true);
                let gone = if !self.has(flag::SPLIT) {
                    matches!(self.results[0], result::BUST | result::SURRENDER)
                } else {
                    self.results == [result::BUST; 2]
                };
                if gone {
                    self.set(flag::PLAYER_BUST, true);
                    self.state = state::DETERMINE_WINNER;
                } else if !self.has(flag::BLACKJACK) {
                    let v = self.value(Hand::Dealer);
                    let ace = self.cards(Hand::Dealer).iter().any(|c| c.value == 12);
                    let n = self.dealer.len();
                    if (v < 17 || (v == 17 && ace && !self.hold_soft_17)) && n < 7 {
                        let c = self.deal_card(Some(Hand::Dealer), true, true, dice);
                        self.dealer.push(c);
                        self.state = state::DEALER_HIT;
                    } else {
                        self.state = state::DETERMINE_WINNER;
                    }
                } else {
                    self.state = state::DETERMINE_WINNER;
                }
            }
            state::DETERMINE_WINNER => self.determine_winner(&mut fx),
            state::CLOSING if t >= 1.0 => {
                fx.push(Effect::Close);
                // Once: the menu is gone after this.
                self.state = u8::MAX;
            }
            _ => {}
        }
        fx
    }

    /// State 1 (`00735353`): the four cards dealt in turn, player, dealer,
    /// player, dealer, each half a card's flight after the last; then the
    /// blackjack checks and the player's turn (or the hole card turned).
    #[allow(clippy::too_many_arguments)]
    fn dealing(
        &mut self,
        t: f32,
        fx: &mut Vec<Effect>,
        end: &dyn Fn(Hand, usize) -> Option<f32>,
        activate: &dyn Fn(&mut Vec<Effect>, Hand, usize),
        deactivate: &dyn Fn(&mut Vec<Effect>, Hand, usize),
        update: &dyn Fn(&mut Vec<Effect>, Model, f32),
    ) {
        let d = self.anim[2];
        let (Some(dealer_end), Some(player_end)) = (end(Hand::Dealer, d), end(Hand::Player, d))
        else {
            return;
        };
        let _ = dealer_end;
        let half = player_end / 2.0;
        let first = end(Hand::Player, 1).unwrap_or(0.0);
        if t > 1.0 {
            self.status_shown = false;
        }
        update(fx, Model::Table, t);
        if t == 0.0 {
            fx.push(Effect::UpdateAll(0.0));
            fx.push(Effect::Sound("GAMEBlackJackCardDeal"));
            self.toggle = true;
            for (hand, i) in [
                (Hand::Player, 0),
                (Hand::Player, 1),
                (Hand::Dealer, 0),
                (Hand::Dealer, 1),
            ] {
                if let Some(&card) = self.hand(hand).get(i) {
                    fx.push(Effect::SwapCard {
                        hand,
                        index: i + 1,
                        card,
                    });
                }
            }
            activate(fx, Hand::Player, 1);
            activate(fx, Hand::Dealer, 1);
            self.anim_delay = first;
        }
        if self.anim[0] == 1 {
            update(fx, Model::Player, t);
        }
        if self.anim[2] == 1 && half <= t {
            if self.toggle && t < first {
                self.toggle = false;
                fx.push(Effect::Sound("GAMEBlackJackCardDeal"));
            }
            update(fx, Model::Dealer, t - half);
        }
        if self.anim[0] == 2 && first <= t {
            if !self.toggle && t < first + half {
                self.toggle = true;
                fx.push(Effect::Sound("GAMEBlackJackCardDeal"));
            }
            update(fx, Model::Player, t - first);
        }
        if self.anim[2] == 2 && first + half <= t {
            if self.toggle {
                self.toggle = false;
                fx.push(Effect::Sound("GAMEBlackJackCardFlip"));
            }
            update(fx, Model::Dealer, t - (first + half));
        }
        if self.anim_delay <= t {
            let dealer1 = end(Hand::Dealer, 1).unwrap_or(0.0);
            if self.anim[0] == 1 {
                self.anim[0] = 2;
                deactivate(fx, Hand::Player, d);
                activate(fx, Hand::Player, 2);
                self.anim_delay = dealer1 + half;
            } else if self.anim[2] == 1 {
                self.anim[2] = 2;
                deactivate(fx, Hand::Dealer, d);
                activate(fx, Hand::Dealer, 2);
                self.anim_delay = first + dealer1;
            } else if self.anim[0] == 2 {
                self.anim[0] = 3;
                deactivate(fx, Hand::Player, d);
                let dealer2 = end(Hand::Dealer, 2).unwrap_or(0.0);
                self.anim_delay = dealer1 + half + dealer2;
            } else {
                let player = self.value(Hand::Player);
                if player == 21 {
                    self.results[0] = result::BLACKJACK;
                    self.set(flag::BLACKJACK, true);
                }
                if self.value(Hand::Dealer) == 21 {
                    self.set(flag::BLACKJACK, true);
                    self.results[0] = if self.results[0] == result::BLACKJACK {
                        result::DRAW
                    } else {
                        result::LOSE
                    };
                }
                if !self.has(flag::BLACKJACK) {
                    self.set_bet_tiles(false);
                }
                if self.chips < self.bet * 2 {
                    self.set(flag::CAN_DOUBLE_MAIN, false);
                    self.set(flag::CAN_DOUBLE_SPLIT, false);
                    self.set(flag::CAN_SPLIT, false);
                } else if player == 11 {
                    self.set(flag::CAN_DOUBLE_MAIN, true);
                } else {
                    self.can_split();
                }
                self.update_tile_alpha(10);
                self.anim_delay = 0.0;
                self.anim[2] += 1;
                deactivate(fx, Hand::Dealer, d);
                self.state = if self.has(flag::BLACKJACK) {
                    state::DEALER_FLIP
                } else {
                    state::PLAYER_TURN
                };
            }
        }
    }

    /// State 7 (`007364fe`): the player's cards swept to the deck, then
    /// the dealer's; then everything back as it was, the level check, and
    /// idle (or closing at a new level).
    fn clean_table(
        &mut self,
        t: f32,
        fx: &mut Vec<Effect>,
        end: &dyn Fn(Hand, usize) -> Option<f32>,
        activate: &dyn Fn(&mut Vec<Effect>, Hand, usize),
        deactivate: &dyn Fn(&mut Vec<Effect>, Hand, usize),
    ) {
        let (Some(players), Some(_), Some(dealers)) = (
            end(Hand::Player, 0),
            end(Hand::Split, 0),
            end(Hand::Dealer, 0),
        ) else {
            self.state = state::IDLE;
            return;
        };
        let table = |fx: &mut Vec<Effect>, on: bool, sequence: &str| {
            let sequence = sequence.to_string();
            fx.push(if on {
                Effect::Activate {
                    model: Model::Table,
                    sequence,
                }
            } else {
                Effect::Deactivate {
                    model: Model::Table,
                    sequence,
                }
            });
        };
        let update = |fx: &mut Vec<Effect>, model: Model, time: f32| {
            fx.push(Effect::Update { model, time });
        };
        if t > 1.0 {
            self.status_shown = false;
        }
        if t == 0.0 && !self.toggle {
            fx.push(Effect::Sound("GAMEBlackJackCardCollect"));
            fx.push(Effect::UpdateAll(0.0));
            table(fx, true, sequence::PLAYERS_DISCARD);
            activate(fx, Hand::Player, 0);
            activate(fx, Hand::Split, 0);
            fx.push(Effect::HideUndealt(self.anim));
        }
        if self.toggle || players <= t {
            if self.toggle && t - self.start_key < dealers {
                update(fx, Model::Table, t - self.start_key);
                update(fx, Model::Dealer, t - self.start_key);
            }
        } else {
            update(fx, Model::Table, t);
            update(fx, Model::Player, t);
            update(fx, Model::Split, t);
        }
        if t < players || self.toggle {
            if dealers <= t - self.start_key && self.toggle {
                table(fx, false, sequence::DEALERS_DISCARD);
                deactivate(fx, Hand::Dealer, 0);
                for m in [Model::Table, Model::Player, Model::Split, Model::Dealer] {
                    fx.push(Effect::DeactivateAll(m));
                }
                fx.push(Effect::ShowAllCards);
                self.flags = 0;
                fx.push(Effect::SideHand(false));
                self.toggle = false;
                self.set_bet_tiles(true);
                self.new_level = super::new_level(&mut self.data, self.max_winnings);
                fx.push(Effect::Data(self.data));
                if self.new_level {
                    self.close();
                } else {
                    self.state = state::IDLE;
                }
                self.anim = [1; 3];
            }
        } else {
            table(fx, false, sequence::PLAYERS_DISCARD);
            table(fx, true, sequence::DEALERS_DISCARD);
            activate(fx, Hand::Dealer, 0);
            fx.push(Effect::Sound("GAMEBlackJackCardCollectDealer"));
            self.toggle = true;
            self.start_key = t;
            update(fx, Model::Table, 0.0);
            update(fx, Model::Dealer, 0.0);
        }
    }

    /// State 12 (`0073749a`): each hand's result against the dealer's, the
    /// net paid, the status line, the hands cleared; then the table
    /// cleared.
    fn determine_winner(&mut self, fx: &mut Vec<Effect>) {
        fx.push(Effect::GamePlayed);
        let judge = |p: i32, d: i32| {
            if p < 22 && (d < p || d > 21) {
                result::WIN
            } else if (p < d && d < 22) || p > 21 {
                result::LOSE
            } else {
                result::DRAW
            }
        };
        let d = self.value(Hand::Dealer);
        if self.results[0] == result::UNKNOWN {
            self.results[0] = judge(self.value(Hand::Player), d);
        }
        if self.has(flag::SPLIT) && self.results[1] == result::UNKNOWN {
            self.results[1] = judge(self.value(Hand::Split), d);
        }
        let mut net = 0;
        let ratio = f64::from(self.payout);
        net += match self.results[0] {
            result::LOSE | result::BUST => -self.bet,
            result::WIN => self.bet,
            result::BLACKJACK => (f64::from(self.bet) * ratio + 0.51) as i32,
            result::SURRENDER => -((f64::from(self.bet) / 2.0 + 0.51) as i32),
            _ => 0,
        };
        net += match self.results[1] {
            result::LOSE | result::BUST => -self.split_bet,
            result::WIN => self.split_bet,
            result::BLACKJACK => (f64::from(self.split_bet) * ratio + 0.5) as i32,
            _ => 0,
        };
        self.chips += net;
        self.data.winnings += net;
        fx.push(Effect::Data(self.data));
        if net > 0 {
            self.status = Some(Status::Won {
                amount: net,
                lucky: self.good_luck,
            });
            fx.push(Effect::Sound("GAMEBlackJackWin"));
        } else if net < 0 {
            self.status = Some(Status::Lost {
                amount: -net,
                unlucky: self.bad_luck,
            });
            fx.push(Effect::Sound("GAMEBlackJackLose"));
        } else {
            self.status = Some(Status::Even);
        }
        self.status_shown = true;
        self.dealer.clear();
        self.player.clear();
        if self.has(flag::SPLIT) {
            self.split.clear();
            self.bet_shown = self.bet;
            self.split_bet = 0;
        }
        if self.has(flag::DOUBLED_MAIN) {
            self.bet /= 2;
            self.bet_shown = self.bet;
        }
        if self.chips < self.bet {
            self.bet = self.chips;
            self.bet_shown = self.bet;
        }
        self.state = state::CLEAN_TABLE;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The vanilla models' lengths: a card's flight 0.633 s, the sweeps
    /// 1.3 s.
    struct Ends;
    impl Clips for Ends {
        fn end(&self, model: Model, sequence: &str) -> Option<f32> {
            let s = sequence.to_ascii_lowercase();
            match model {
                Model::Table => Some(1.3),
                Model::Player | Model::Split | Model::Dealer => {
                    if s.ends_with("discard") {
                        Some(1.3)
                    } else {
                        Some(0.6333)
                    }
                }
                _ => None,
            }
        }
    }

    fn open(chips: i32, luck: f32) -> Blackjack {
        Blackjack::open(
            1.5,
            0.2,
            3,
            false,
            9000,
            1,
            200,
            chips,
            luck,
            CasinoData::default(),
        )
        .0
    }

    /// Runs updates until the state isn't `from` (at most 20 s).
    fn run_while(m: &mut Blackjack, tick: &mut u32, from: &[u8], dice: Dice) -> Vec<Effect> {
        let mut seen = Vec::new();
        for _ in 0..2000 {
            if !from.contains(&m.state) {
                break;
            }
            *tick += 16;
            seen.extend(m.update(*tick, &Ends, dice));
        }
        seen
    }

    #[test]
    fn values_and_aces() {
        let c = |v: u8| Card {
            value: v,
            suit: 0,
            deck: 0,
            face: 0,
        };
        assert_eq!(hand_value(&[c(12), c(8)]), 21);
        assert_eq!(hand_value(&[c(12), c(12), c(8)]), 12);
        assert_eq!(hand_value(&[c(11), c(4), c(9)]), 26);
        assert_eq!(numeric(Card::EMPTY.value), 16);
        // Card 0 of a deck is the ace of hearts; 13 the ace of clubs.
        assert_eq!((Card::new(0, 0).value, Card::new(0, 13).suit), (12, 1));
        assert_eq!((Card::new(0, 9).value, Card::new(0, 12).value), (8, 11));
    }

    #[test]
    fn deck_texture_names() {
        let (back, faces) =
            deck_textures("terminals\\NV_PlayingCards\\DeckG\\cardG_Back.dds").unwrap();
        assert_eq!(
            back,
            "Textures\\Terminals\\NV_PlayingCards\\DeckG\\cardG_Back.dds"
        );
        assert!(faces[0].ends_with("DeckG\\cardG_h01.dds"));
        assert!(faces[12].ends_with("cardG_h13.dds"));
        assert!(faces[51].ends_with("cardG_s13.dds"));
    }

    #[test]
    fn bets_and_chip_stacks() {
        assert_eq!((step_up(9), step_up(395), step_up(1400)), (10, 420, 1500));
        assert_eq!((step_down(10), step_down(37), step_down(43)), (9, 35, 40));
        assert_eq!((step_down(400), step_down(1501)), (375, 1500));
        let (z, hide) = bet_chips(23);
        assert!((z[0] - 8.0 * 0.18).abs() < 1e-6 && (z[1] - 3.0 * 0.18).abs() < 1e-6);
        assert_eq!(hide, [false, false, true, true, true, true]);
        assert!(bet_chips(1500).1[5]);
    }

    /// A plain card comes off the count twice; a reshuffle under five.
    #[test]
    fn the_shoe_counts_down_by_two() {
        let mut m = open(100, 5.0);
        assert_eq!(m.left, 156);
        let slot = m.deal_card(Some(Hand::Player), true, false, &mut |_| 0);
        assert_eq!((slot, m.left), (155, 154));
        // The card dealt was the shoe's first (the ace of hearts).
        assert_eq!(m.shoe[155], Card::new(0, 0));
        m.left = 4;
        m.deal_card(None, false, false, &mut |_| 0);
        assert_eq!(m.left, 154);
    }

    /// A round: dealt, stand, the dealer draws to 17 or more, paid, the
    /// table cleared, back to idle.
    #[test]
    fn a_round_through_to_idle() {
        let mut m = open(100, 5.0);
        let mut tick = 1000;
        m.update(tick, &Ends, &mut |_| 0);
        m.click(tile::SURRENDER_MAX_BET, &mut |_| 0);
        assert_eq!(m.bet, 100);
        m.click(tile::SWITCH_DECREASE_BET, &mut |_| 0);
        assert_eq!(m.bet, 90);
        // Every draw the first live card: the shoe's front, swapped back.
        let mut dice = |_: usize| 0;
        m.click(tile::DOUBLE_DEAL, &mut dice);
        assert_eq!(m.state, state::DEALING);
        assert_eq!((m.dealer.len(), m.player.len()), (2, 2));
        let fx = run_while(&mut m, &mut tick, &[state::DEALING], &mut dice);
        assert!(fx.contains(&Effect::Sound("GAMEBlackJackCardFlip")));
        assert!(!m.betting_labels || m.has(flag::BLACKJACK));
        if m.state == state::PLAYER_TURN {
            m.click(tile::STAY_EXIT, &mut dice);
        }
        let fx = run_while(
            &mut m,
            &mut tick,
            &[
                state::DEALER_FLIP,
                state::DEALER_TURN,
                state::DEALER_HIT,
                state::DETERMINE_WINNER,
                state::CLEAN_TABLE,
            ],
            &mut dice,
        );
        assert_eq!(m.state, state::IDLE);
        assert!(fx.contains(&Effect::GamePlayed));
        assert!(m.dealer.is_empty() && m.player.is_empty());
        assert!(m.betting_labels);
        assert_eq!(m.data.winnings, m.chips - 100);
        assert!(m.status.is_some());
    }

    /// Settling a split: both hands against the dealer, the bets added.
    #[test]
    fn a_split_round_is_paid_per_hand() {
        let mut m = open(500, 5.0);
        let c = |v: u8| Card {
            value: v,
            suit: 0,
            deck: 0,
            face: 0,
        };
        m.shoe[0] = c(8); // 10
        m.shoe[1] = c(7); // 9
        m.shoe[2] = c(8);
        m.shoe[3] = c(5); // 7
        m.shoe[4] = c(8);
        m.shoe[5] = c(6); // 8
        m.dealer = vec![0, 3]; // 17
        m.player = vec![1, 4]; // 19
        m.split = vec![2, 5]; // 18
        m.bet = 10;
        m.split_bet = 10;
        m.flags = flag::SPLIT;
        m.state = state::DETERMINE_WINNER;
        m.update(0, &Ends, &mut |_| 0);
        m.update(16, &Ends, &mut |_| 0);
        assert_eq!(m.results, [result::WIN, result::WIN]);
        assert_eq!(m.chips, 520);
        assert_eq!(
            m.status,
            Some(Status::Won {
                amount: 20,
                lucky: false
            })
        );
    }

    /// Blackjack pays the casino's ratio (+0.51, truncated); surrender
    /// loses half (+0.51).
    #[test]
    fn blackjack_and_surrender_payouts() {
        let mut m = open(500, 5.0);
        m.bet = 15;
        m.results = [result::BLACKJACK, result::UNKNOWN];
        m.state = state::DETERMINE_WINNER;
        m.update(0, &Ends, &mut |_| 0);
        assert_eq!(m.chips, 500 + 23);
        let mut m = open(500, 5.0);
        m.bet = 15;
        m.results = [result::SURRENDER, result::UNKNOWN];
        m.state = state::DETERMINE_WINNER;
        m.update(0, &Ends, &mut |_| 0);
        assert_eq!(m.chips, 500 - 8);
    }

    /// The dealer hits soft 17 unless the casino holds on it.
    #[test]
    fn soft_seventeen() {
        for (hold, hits) in [(false, true), (true, false)] {
            let mut m = Blackjack::open(
                1.5,
                0.2,
                3,
                hold,
                9000,
                1,
                200,
                100,
                5.0,
                CasinoData::default(),
            )
            .0;
            m.shoe[0] = Card::new(0, 0); // ace
            m.shoe[1] = Card::new(0, 5); // 6
            m.shoe[2] = Card::new(0, 9); // 10
            m.dealer = vec![0, 1];
            m.player = vec![2, 2];
            m.state = state::DEALER_TURN;
            m.update(0, &Ends, &mut |_| 10);
            assert_eq!(m.state == state::DEALER_HIT, hits);
        }
    }

    /// Doubling doubles the bet, takes one card and ends the hand; the
    /// bet comes back to its size after the round.
    #[test]
    fn doubling() {
        let mut m = open(100, 5.0);
        m.bet = 10;
        m.player = vec![0, 1];
        m.dealer = vec![2, 3];
        m.state = state::PLAYER_TURN;
        m.flags = flag::CAN_DOUBLE_MAIN;
        m.click(tile::DOUBLE_DEAL, &mut |_| 10);
        assert_eq!((m.bet, m.player.len(), m.state), (20, 3, state::HIT));
        assert!(m.has(flag::DOUBLED_MAIN) && m.has(flag::DISABLE_MAIN));
        assert_eq!(m.bet_shown, 20);
    }

    /// Splitting a pair: the second card moves to the split hand, each
    /// hand gets a card (no luck), and the animations run split, main,
    /// split before the player's turn.
    #[test]
    fn splitting_a_pair() {
        let mut m = open(100, 5.0);
        let eight = Card::new(0, 7);
        m.shoe[100] = eight;
        m.shoe[101] = eight;
        m.player = vec![100, 101];
        m.dealer = vec![0, 1];
        m.bet = 10;
        m.anim = [3, 1, 3];
        m.state = state::PLAYER_TURN;
        m.flags = flag::CAN_DOUBLE_MAIN;
        m.can_split();
        assert!(m.has(flag::CAN_SPLIT));
        let fx = m.click(tile::SPLIT_INCREASE_BET, &mut |_| 5);
        assert_eq!(m.split[0], 101);
        assert_eq!((m.player.len(), m.split.len(), m.split_bet), (2, 2, 10));
        assert_eq!(m.bet_shown, 20);
        assert!(fx.contains(&Effect::SwapCard {
            hand: Hand::Split,
            index: 1,
            card: 101
        }));
        let mut tick = 0;
        let seen = run_while(&mut m, &mut tick, &[state::SPLIT, state::HIT], &mut |_| 0);
        assert_eq!(m.state, state::PLAYER_TURN);
        let order: Vec<&str> = seen
            .iter()
            .filter_map(|e| match e {
                Effect::Activate { sequence, .. } => Some(sequence.as_str()),
                _ => None,
            })
            .collect();
        assert_eq!(order, ["Hand1_02", "Hand2_01", "Hand1_02", "Hand2_02"]);
        assert_eq!(m.anim, [3, 3, 3]);
        // Split hands can't double on PC (0x200 set, 0x100 read).
        assert!(!m.has(flag::CAN_DOUBLE_SPLIT));
    }
}
