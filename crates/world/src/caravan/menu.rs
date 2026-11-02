//! The Caravan menu as the game runs it (`CaravanMenu`, class 1083,
//! `menus\caravan_menu.xml`): its four screens, the states its update steps
//! through, its controls. Read from FalloutNV.exe 1.4.0.525, the Xbox 360
//! prototype's PDB naming the parts (notes in `docs/CARAVAN.md`). The 3D,
//! the tiles and the player's things are the caller's: [`Menu`] says what
//! to do as [`Effect`]s.
//!
//! - The screens (`eActiveMenu` +0x28) go one way: the ante
//!   (`PrepareAnteMenu` `0073d020`), building the deck (`PrepareDeckMenu`
//!   `0073d850`), the game (`PrepareGameMenu` `0073ea90`), the results
//!   (`PrepareResultsMenu` `00740980`); then the menu closes. Leaving the ante
//!   or the deck closes it at once, nothing paid, the deck kept.
//! - Each update (`DoIdle` `00741500`) counts the seconds in the state
//!   (`ftotalStateSecs`): while the state is the one it was last update, the
//!   milliseconds since then ÷ 1000 are added, otherwise it starts at 0. An
//!   animated state starts its sequences on its first update (time 0),
//!   updates its models at the time every update and moves on once the time
//!   reaches its sequence's end (passes it, for a track's columns going or
//!   coming back: states 15, 17, 18).
//! - Input only counts in state 0 with nothing playing (`CAF_DISABLE_ALL`
//!   clear) and, in a game, on the player's turn (`DoGamepad` `00747d30`).
//!   Keys (`00749360`): W the A button, A the X button, F Y, Q LT, E RT, S
//!   RB, R B (`DoGamepadUpEvent` `007490e0`); the arrows wait for the next
//!   update, which uses them only in state 0; on the results screen any key
//!   closes the menu. Clicks (`00749560`) press their tile's button.
//! - The opponent moves on the update after the last animation ended (state
//!   21), with no delay of its own; the player's first move of a turn waits
//!   0.25 s (`fSelectionDelay`, set to 0 as each card animation starts).

use esm::FormId;

use super::bet::Bet;
use super::money;
use super::{ai, Card, Dice, Game, ACE, JACK, JOKER, MIN_DECK, ROWS, TRACKS};

/// The states (`CaravanMenu::CaravanState`; 16 and 20 do nothing on PC).
pub mod state {
    pub const IDLE: u8 = 0;
    pub const CAMERA_TO_DECK: u8 = 1;
    pub const CAMERA_TO_GAME: u8 = 2;
    pub const CAMERA_TO_RESULTS: u8 = 3;
    pub const ANTE_MONEY_IN: u8 = 4;
    pub const DECK_FORWARD: u8 = 5;
    pub const DECK_BACK: u8 = 6;
    pub const DECK_FLIP: u8 = 7;
    pub const DECK_FAST_FORWARD: u8 = 8;
    pub const DECK_REWIND: u8 = 9;
    pub const DECK_WIPE: u8 = 10;
    pub const INTRO_DEAL: u8 = 11;
    pub const INTRO_DISCARD: u8 = 12;
    pub const PLACE_CARD: u8 = 13;
    pub const DISCARD_CARD: u8 = 14;
    pub const DISCARD_TRACK: u8 = 15;
    pub const JACK_CLEAR: u8 = 17;
    pub const JACK_FINISH: u8 = 18;
    pub const JOKER_CLEAR: u8 = 19;
    pub const PROCESS_AI: u8 = 21;
    pub const GAME_OVER: u8 = 22;
    pub const CLOSING: u8 = 23;
}

/// The flags (`enumFlags`, +0xE74).
pub mod flag {
    /// Busy: nothing works, every button dimmed.
    pub const DISABLE_ALL: u32 = 0x1;
    pub const DISABLE_Y: u32 = 0x2;
    pub const DISABLE_A: u32 = 0x4;
    pub const DISABLE_B: u32 = 0x8;
    pub const DISABLE_X: u32 = 0x10;
    pub const DISABLE_LB: u32 = 0x20;
    pub const DISABLE_RB: u32 = 0x40;
    pub const DISABLE_LT: u32 = 0x80;
    pub const DISABLE_RT: u32 = 0x100;
    pub const PLAYER_WIN: u32 = 0x400;
    pub const TRACK_SETUP: u32 = 0x800;
    pub const SELECT_TRACK: u32 = 0x1000;
    pub const DESTROY_TRACK: u32 = 0x2000;
    pub const AI: u32 = 0x4000;
    pub const PROCESS_JOKER: u32 = 0x8000;
    pub const IS_ON_TOP: u32 = 0x10000;
    pub const MARK_FOR_CLOSE: u32 = 0x20000;
}

/// The gamepad codes `DoGamepad` takes (`Interface::enumGamepadButtons`).
pub mod button {
    pub const START: u8 = 5;
    pub const BACK: u8 = 6;
    pub const A: u8 = 9;
    pub const B: u8 = 10;
    pub const X: u8 = 11;
    pub const Y: u8 = 12;
    pub const LT: u8 = 13;
    pub const RT: u8 = 14;
    pub const LB: u8 = 15;
    pub const RB: u8 = 16;
    pub const LTHUMB: u8 = 17;
    pub const RTHUMB: u8 = 18;
}

/// The screens (`ActiveMenu`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Ante,
    Deck,
    Game,
    Results,
}

/// An arrow key (`0x80000001`–`4`: left, right, up, down).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arrow {
    Left,
    Right,
    Up,
    Down,
}

impl Arrow {
    /// As `DoIdle` reads it: x and y (y down).
    fn xy(self) -> (f32, f32) {
        match self {
            Arrow::Left => (-1.0, 0.0),
            Arrow::Right => (1.0, 0.0),
            Arrow::Up => (0.0, -1.0),
            Arrow::Down => (0.0, 1.0),
        }
    }
}

/// The models the menu animates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Model {
    /// `NV_Caravan-Table.NIF` (the camera's).
    Table,
    /// The deck screen's `NV_Caravan-Deck.NIF` and `NV_Caravan-Available.NIF`.
    Deck,
    Available,
    /// `Player_Deck.NIF` or `Opponent_Deck.NIF`.
    Hand {
        npc: bool,
    },
    /// `Player_{t+1}_0{r+1}.NIF` (tracks 0–2) or `Opponent_{t−2}_0{r+1}.NIF`.
    Row {
        track: usize,
        row: usize,
    },
    /// A bill or coin on the table.
    Bill(usize),
    Coin(usize),
}

/// How long a model's sequence runs (its end key time, `+0x30`).
pub trait Clips {
    fn end(&self, model: Model, sequence: &str) -> f32;
}

/// What the caller does for the menu.
#[derive(Debug, Clone, PartialEq)]
pub enum Effect {
    /// A sound by its editor ID.
    Sound(&'static str),
    /// A tutorial message (`MESG` editor ID), shown once; the menu waits
    /// until [`Menu::tutorial_done`].
    Tutorial(&'static str),
    /// Start a sequence on a model (`ActivateSequence`, weight 1), stop one
    /// (`DeactivateSequence`), stop them all.
    Activate {
        model: Model,
        sequence: String,
    },
    Deactivate {
        model: Model,
        sequence: String,
    },
    DeactivateAll(Model),
    /// Update a model at a time: its running sequences pose it (a sequence's
    /// time counts from its first update), and the pose stays.
    Update {
        model: Model,
        time: f32,
    },
    /// Pose a model at a sequence's first frame (activate, update at 0,
    /// deactivate).
    ResetPose {
        model: Model,
        sequence: String,
    },
    /// The deck screen's models in (`PrepareDeckMenu`) and out, the game's
    /// in (`PrepareGameMenu`).
    LoadDeckModels,
    UnloadDeckModels,
    LoadGameModels,
    /// The deck screen's cards around one (`SwapDeckBuildingTextures`
    /// `0074b700`), and the middle one's again (`FlipSelectedCardTextures`
    /// `0074af70`).
    DeckTextures {
        around: i32,
    },
    FlipSelected,
    /// A hand's cards (`UpdateGameHandTextures` `0074d730`; `setup`: its
    /// second argument, the starting rules), the card coming next and the
    /// pile (`UpdateDealCardTexture` `0074d190`).
    HandTextures {
        npc: bool,
        setup: bool,
    },
    DealCard {
        npc: bool,
    },
    /// A row's card (`UpdateTrackCardTexture` `0074e220`).
    TrackCard {
        track: usize,
        row: usize,
        column: usize,
    },
    /// The card being placed on its row (`UpdateSelectedTrackCardTexture`
    /// `0074df90`).
    PlacedCard {
        track: usize,
        row: usize,
        column: usize,
        card: Card,
    },
    /// The player's hand card raised (`SelectDeckCard` `0074e480`), or the
    /// raised one put down.
    LiftHandCard(Option<usize>),
    /// The place a card would go (`SelectTrackCard` `0074e6d0`): the green
    /// marker there when it may, else the red; and both away
    /// (`DeselectTrackCard` `0074edd0`).
    Cursor {
        track: usize,
        row: usize,
        column: usize,
        valid: bool,
        card: Card,
    },
    HideCursors,
    /// A whole track raised (`SelectWholeTrack` `0074ecb0`) or put back
    /// (`0074ed40`).
    LiftTrack(usize),
    LowerTrack(usize),
    /// A side's draw pile used up: its pile and the card coming next hidden.
    CullDrawPile {
        npc: bool,
    },
    /// New money on the table (`LoadNewBill`, `LoadNewCoin`): the pieces
    /// past [`money::Table::bills_from`] and `coins_from`.
    Money,
    /// The "How many?" box for a raise, up to this.
    HowMany {
        max: i32,
    },
    /// "If you quit now…" with Yes and No.
    ConfirmForfeit,
    /// The player's two card lists written back (`Close`, `PrepareGameMenu`),
    /// each head first.
    SaveDeck {
        out: Vec<FormId>,
        deck: Vec<FormId>,
    },
    /// The game's over (`PrepareResultsMenu`): the record and the stake
    /// (`world::caravan::bet::settle`).
    Settle {
        won: bool,
        stake: i32,
    },
    /// The menu's gone.
    Close,
}

/// A card on the deck screen (`pDeckArray`) and whether it's in the deck
/// (the card's +0x98).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeckCard {
    pub card: Card,
    pub in_deck: bool,
}

/// The tutorials (`HelpCaravan…`, ids 0x1E–0x21).
pub const HELP_BETTING: &str = "HelpCaravanBetting";
pub const HELP_DECK: &str = "HelpCaravanDeckBuilding";
pub const HELP_STARTING: &str = "HelpCaravanStartingCaravans";
pub const HELP_CONTRACT_WAR: &str = "HelpCaravanContractWar";

/// A tutorial's id (`world::tutorial`): the menu looks its message up by
/// editor ID and marks the id shown (`00741060`, `00741500`).
pub fn tutorial_id(name: &str) -> Option<u8> {
    use crate::tutorial::id;
    Some(match name {
        HELP_BETTING => id::CARAVAN_BET,
        HELP_DECK => id::CARAVAN_DECK,
        HELP_STARTING => id::CARAVAN_TRACK,
        HELP_CONTRACT_WAR => id::CARAVAN_GAME,
        _ => return None,
    })
}

/// A stick axis as `DoIdle` reads it: within 7849 of the middle 0, else
/// the raw value (−32768..32767) over 32767.
pub fn stick_axis(raw: i32) -> f32 {
    if (raw as f32).abs() <= 7849.0 {
        0.0
    } else {
        raw as f32 / 32767.0
    }
}

/// The most cards a deck can hold (`UpdateCaravanFlags`: Add off above 107).
pub const MAX_DECK: usize = 108;

/// The scrollbar's first value (`Create`: `_current_value` 12).
pub const FIRST_CHOSEN: usize = 12;

/// The hand-out sequence for the card at `sel`: the five-card fan's, or the
/// eight-card one's.
fn hand_out(eight: bool, sel: usize) -> String {
    if eight {
        format!("Deck8-Card{}_Out", sel + 1)
    } else {
        format!("Deck-Card{}_Out", sel + 1)
    }
}

/// A row card's sequence.
fn row_sequence(column: usize, coming_in: bool) -> String {
    format!(
        "Card{}_{}",
        column + 1,
        if coming_in { "In" } else { "Out" }
    )
}

/// The menu.
#[derive(Debug, Clone)]
pub struct Menu {
    pub screen: Screen,
    pub state: u8,
    pub flags: u32,
    /// Seconds in the state (`ftotalStateSecs`).
    pub elapsed: f32,
    prev_state: Option<u8>,
    prev_tick: Option<u32>,
    /// The challenge: the opponent, its deck, how well it plays.
    pub npc: FormId,
    pub npc_deck: Vec<Card>,
    pub difficulty: i32,
    /// The ante (`iPlayerAnte`, `iNPCAnte`, `iTotalFunds`), the player's
    /// Barter, the opponent's most (funds × share × Barter / 100 + funds ×
    /// share, not capped by its funds), the stake (`iAnte`).
    pub bet: Bet,
    pub barter: f32,
    pub most: i32,
    pub stake: i32,
    pub money: money::Table,
    /// The deck screen: the player's cards sorted, how many are in the deck
    /// (`iCardsInDeck`), the chosen one (the scrollbar's `_current_value`),
    /// steps still to show (`iNumSteps`), where a drag started
    /// (`iPrevValue`).
    pub cards: Vec<DeckCard>,
    pub in_deck: usize,
    pub chosen: usize,
    pub steps: i32,
    pub drag_from: Option<usize>,
    /// A mouse drag being followed (`+0xe85`), from the scrollbar's value
    /// when it began (`drag_from`, `+0xaf8`).
    pub dragging: bool,
    /// A pad in use (`004b71d0`): its left stick read, the mouse's drags
    /// not.
    pub pad: bool,
    /// The pad's left stick this update (`DoIdle`'s start): x right and y
    /// down, each the raw value over 32767, 0 within 7849 of the middle
    /// ([`stick_axis`]).
    pub stick: [f32; 2],
    /// The game.
    pub game: Option<Game>,
    /// The hand card chosen (`iSelectedDeckCard`), where a card goes
    /// (`iSelectedTrack`, `iSelectedRow`, `iSelectedColumn`), the card being
    /// placed (`pSelectedCard`).
    pub hand_card: usize,
    pub track: usize,
    pub row: usize,
    pub column: i32,
    pub placed: Option<Card>,
    /// When the cursor last moved (`fSelectionDelay`).
    pub selection_delay: f32,
    /// The rows a joker takes, by track (`jokerLocations`).
    pub joker_rows: [Vec<usize>; TRACKS],
    /// The opponent moves next (`bContinueProcessing`, PC +0xE86).
    pub opponent_next: bool,
    /// An arrow waiting for the update (PC +0xE80).
    pub arrow: Option<Arrow>,
    /// A tutorial being read; the quit box or the "How many?" box up.
    pub tutorial: bool,
    pub prompt: bool,
    /// The draw piles' counts as the tiles show them (29, 30: set when
    /// dealt and at the ends of states 11 to 14, `+0xA4` + 4 × side).
    pub piles: [usize; 2],
    /// The columns launched this update (states 15, 17, 18).
    launched: usize,
}

impl Menu {
    /// `ShowCaravanMenu` with 30 cards or more (`Create` `00741060`): the
    /// cards sorted (`PrepareCaravanData` `0073ced0`: the ones out of the
    /// deck then the deck, in their lists' order, by value with the game's
    /// `qsort` and `TESCaravanCard::SortCardsFunc` `0059ae10`), the ante set
    /// (`PrepareAnteMenu`) with the opponent's ante on the table, the
    /// betting tutorial.
    #[allow(clippy::too_many_arguments)]
    pub fn open(
        npc: FormId,
        npc_deck: Vec<Card>,
        difficulty: i32,
        out: &[Card],
        deck: &[Card],
        bet: Bet,
        barter: f32,
        dice: Dice,
    ) -> (Menu, Vec<Effect>) {
        let mut cards: Vec<DeckCard> = out
            .iter()
            .map(|&card| DeckCard {
                card,
                in_deck: false,
            })
            .chain(deck.iter().map(|&card| DeckCard {
                card,
                in_deck: true,
            }))
            .collect();
        ai::crt_qsort(&mut cards, &|a: &DeckCard, b: &DeckCard| {
            (a.card.value > b.card.value) as i32 - (a.card.value < b.card.value) as i32
        });
        let x = f64::from(bet.npc_funds as f32 * bet.share);
        let most = (f64::from(barter.clamp(0.0, 100.0)) / 100.0 * x + x) as i32;
        let mut menu = Menu {
            screen: Screen::Ante,
            state: state::IDLE,
            flags: 0,
            elapsed: 0.0,
            prev_state: None,
            prev_tick: None,
            npc,
            npc_deck,
            difficulty,
            bet,
            barter,
            most,
            stake: 0,
            money: money::Table::default(),
            cards,
            in_deck: deck.len(),
            chosen: FIRST_CHOSEN,
            steps: 0,
            drag_from: None,
            dragging: false,
            pad: false,
            stick: [0.0; 2],
            game: None,
            hand_card: 0,
            track: 0,
            row: 0,
            column: 0,
            placed: None,
            selection_delay: 0.0,
            joker_rows: Default::default(),
            opponent_next: true,
            arrow: None,
            tutorial: false,
            prompt: false,
            piles: [0; 2],
            launched: 0,
        };
        let mut fx = Vec::new();
        if menu.money.add(menu.most, menu.bet.npc_ante, dice) {
            menu.state = state::ANTE_MONEY_IN;
            fx.push(Effect::Money);
        }
        menu.update_flags();
        menu.tutorial = true;
        fx.push(Effect::Tutorial(HELP_BETTING));
        (menu, fx)
    }

    fn set(&mut self, f: u32, on: bool) {
        if on {
            self.flags |= f;
        } else {
            self.flags &= !f;
        }
    }

    pub fn has(&self, f: u32) -> bool {
        self.flags & f != 0
    }

    /// Whether a button's tile is lit (`UpdateTileAlpha` `00749a40`: 255,
    /// else 128).
    pub fn lit(&self, f: u32) -> bool {
        !self.has(flag::DISABLE_ALL) && !self.has(f)
    }

    /// Which buttons work (`UpdateCaravanFlags` `00749740`).
    pub fn update_flags(&mut self) {
        match self.screen {
            Screen::Ante => {
                let b = self.bet;
                self.set(flag::DISABLE_Y, !b.can_accept());
                let a_off = b.player_funds == 0
                    || b.npc_ante <= b.player_ante
                    || (b.player_funds <= b.player_ante && b.player_ante < b.npc_ante);
                self.set(flag::DISABLE_A, a_off);
                self.set(
                    flag::DISABLE_X,
                    b.player_funds == 0 || b.player_funds <= b.player_ante,
                );
            }
            Screen::Deck => {
                let in_deck = self.cards.get(self.chosen).is_some_and(|c| c.in_deck);
                self.set(flag::DISABLE_Y, self.in_deck < MIN_DECK);
                self.set(flag::DISABLE_A, in_deck || self.in_deck >= MAX_DECK);
                self.set(flag::DISABLE_X, !in_deck || self.in_deck == 0);
                self.set(flag::DISABLE_LB, true);
            }
            Screen::Game => {
                if let Some(g) = &self.game {
                    if g.player_deck.is_empty() && g.next_player.is_none() {
                        self.flags |= flag::DISABLE_LT;
                    }
                    let empty = (0..3).all(|t| g.tracks[t][0].is_empty());
                    self.set(flag::DISABLE_RT, empty || self.has(flag::TRACK_SETUP));
                }
            }
            Screen::Results => {}
        }
    }

    /// The deck the player plays with, in the deck screen's order.
    pub fn player_deck(&self) -> Vec<Card> {
        self.cards
            .iter()
            .filter(|c| c.in_deck)
            .map(|c| c.card)
            .collect()
    }

    /// The player's lists from the deck screen (each card added at its
    /// list's head, in the sorted order).
    fn save_deck(&self) -> Effect {
        let mut out = Vec::new();
        let mut deck = Vec::new();
        for c in &self.cards {
            if c.in_deck {
                deck.insert(0, c.card.form);
            } else {
                out.insert(0, c.card.form);
            }
        }
        Effect::SaveDeck { out, deck }
    }

    /// `Close` (`0073ca50`): from the deck screen the deck is kept.
    fn close(&mut self, fx: &mut Vec<Effect>) {
        if self.screen == Screen::Deck {
            fx.push(self.save_deck());
        }
        self.state = state::CLOSING;
    }

    /// A tutorial message was dismissed.
    pub fn tutorial_done(&mut self) {
        self.tutorial = false;
    }

    // -----------------------------------------------------------------------
    // The update.

    /// The update (`DoIdle`), `tick` the game's milliseconds.
    pub fn update(&mut self, tick: u32, clips: &dyn Clips, dice: Dice) -> Vec<Effect> {
        let mut fx = Vec::new();
        if self.tutorial || self.prompt {
            // Not on top: the clock isn't kept (the time behind the box is
            // added on the way back).
            self.flags &= !flag::IS_ON_TOP;
            return fx;
        }
        match self.prev_tick {
            None => {
                self.elapsed = 0.0;
                self.selection_delay = 0.0;
            }
            Some(prev) if self.prev_state == Some(self.state) => {
                self.elapsed += tick.wrapping_sub(prev) as f32 / 1000.0;
            }
            Some(_) => self.elapsed = 0.0,
        }
        self.flags |= flag::IS_ON_TOP;
        self.prev_tick = Some(tick);
        self.prev_state = Some(self.state);
        if self.has(flag::MARK_FOR_CLOSE) {
            self.flags &= !flag::MARK_FOR_CLOSE;
            self.state = state::GAME_OVER;
            return fx;
        }
        if self.screen == Screen::Results && self.state == state::IDLE && self.elapsed > 1.0 {
            self.flags &= !flag::DISABLE_ALL;
        }
        self.launched = 0;
        match self.state {
            state::IDLE => self.idle(&mut fx),
            state::CAMERA_TO_DECK => {
                if self.camera(&mut fx, "Bet_to_Deck", clips) {
                    self.state = state::IDLE;
                    self.flags &= !flag::DISABLE_ALL;
                    self.tutorial = true;
                    fx.push(Effect::Tutorial(HELP_DECK));
                }
            }
            state::CAMERA_TO_GAME => self.camera_to_game(&mut fx, clips),
            state::CAMERA_TO_RESULTS => {
                if self.camera(&mut fx, "Play_to_Bet", clips) {
                    self.state = state::IDLE;
                    self.screen = Screen::Results;
                    self.flags &= !flag::AI;
                }
            }
            state::ANTE_MONEY_IN => self.money_in(&mut fx),
            state::DECK_FORWARD..=state::DECK_WIPE => self.deck_animation(&mut fx, clips, dice),
            state::INTRO_DEAL | state::INTRO_DISCARD | state::PLACE_CARD | state::DISCARD_CARD => {
                self.hand_animation(&mut fx, clips)
            }
            state::DISCARD_TRACK | state::JACK_CLEAR | state::JACK_FINISH => {
                self.columns(&mut fx, clips)
            }
            state::JOKER_CLEAR => {
                self.flags |= flag::PROCESS_JOKER;
                while self.track < TRACKS && self.joker_rows[self.track].is_empty() {
                    self.track += 1;
                }
                if self.track >= TRACKS {
                    self.state = state::PROCESS_AI;
                    self.flags &= !flag::PROCESS_JOKER;
                } else {
                    self.row = self.joker_rows[self.track][0];
                    self.state = state::JACK_CLEAR;
                }
            }
            state::PROCESS_AI => self.turn(&mut fx, dice),
            state::GAME_OVER => self.game_over(&mut fx),
            state::CLOSING => fx.push(Effect::Close),
            _ => {}
        }
        fx
    }

    /// A camera move: the table's sequence; true once it's over.
    fn camera(&mut self, fx: &mut Vec<Effect>, sequence: &str, clips: &dyn Clips) -> bool {
        let t = self.elapsed;
        if t == 0.0 {
            fx.push(Effect::Activate {
                model: Model::Table,
                sequence: sequence.into(),
            });
            self.flags |= flag::DISABLE_ALL;
        }
        fx.push(Effect::Update {
            model: Model::Table,
            time: t,
        });
        if t < clips.end(Model::Table, sequence) {
            return false;
        }
        fx.push(Effect::Deactivate {
            model: Model::Table,
            sequence: sequence.into(),
        });
        true
    }

    /// State 4: the new money lands; done after a second.
    fn money_in(&mut self, fx: &mut Vec<Effect>) {
        let t = self.elapsed;
        if t == 0.0 {
            self.flags |= flag::DISABLE_ALL;
            fx.push(Effect::Sound("GAMECaravanFundsDrop"));
        }
        for i in self.money.bills_from..self.money.bills.len() {
            fx.push(Effect::Update {
                model: Model::Bill(i),
                time: t,
            });
        }
        for i in self.money.coins_from..self.money.coins.len() {
            fx.push(Effect::Update {
                model: Model::Coin(i),
                time: t,
            });
        }
        if t > 1.0 {
            self.flags &= !flag::DISABLE_ALL;
            self.money.landed();
            self.state = state::IDLE;
        }
    }

    /// State 2: to the table, both hands dealt (`Deck-Cards_In`).
    fn camera_to_game(&mut self, fx: &mut Vec<Effect>, clips: &dyn Clips) {
        let t = self.elapsed;
        if t == 0.0 {
            self.flags = flag::DISABLE_ALL;
            self.selection_delay = 0.0;
            fx.push(Effect::Sound("GAMECaravanStartMatch"));
            fx.push(Effect::Activate {
                model: Model::Table,
                sequence: "Deck_to_Play".into(),
            });
            for npc in [false, true] {
                fx.push(Effect::Activate {
                    model: Model::Hand { npc },
                    sequence: "Deck-Cards_In".into(),
                });
            }
        }
        fx.push(Effect::Update {
            model: Model::Table,
            time: t,
        });
        for npc in [false, true] {
            fx.push(Effect::Update {
                model: Model::Hand { npc },
                time: t,
            });
        }
        let done = t >= clips.end(Model::Table, "Deck_to_Play")
            && t >= clips.end(Model::Hand { npc: false }, "Deck-Cards_In")
            && t >= clips.end(Model::Hand { npc: true }, "Deck-Cards_In");
        if !done {
            return;
        }
        fx.push(Effect::Deactivate {
            model: Model::Table,
            sequence: "Deck_to_Play".into(),
        });
        for npc in [false, true] {
            fx.push(Effect::Deactivate {
                model: Model::Hand { npc },
                sequence: "Deck-Cards_In".into(),
            });
        }
        self.flags |= flag::TRACK_SETUP;
        if let Some(g) = &mut self.game {
            g.setup = true;
        }
        self.state = state::PROCESS_AI;
        self.tutorial = true;
        fx.push(Effect::Tutorial(HELP_STARTING));
    }

    /// State 0: what the arrows do on each screen.
    fn idle(&mut self, fx: &mut Vec<Effect>) {
        let arrow = self.arrow.take();
        match self.screen {
            Screen::Ante => self.update_flags(),
            Screen::Deck => {
                // The stick first, without the arrows' guard: a card on
                // its way (the scrollbar's value plus x / |x|), the right
                // way forward.
                let x = self.stick[0];
                if self.pad && x != 0.0 {
                    let v = (self.chosen as f32 + x / x.abs()) as i32;
                    if v >= 0 && (v as usize) < self.cards.len() {
                        self.chosen = v as usize;
                        self.steps = 1;
                        self.state = if x <= 0.0 {
                            state::DECK_BACK
                        } else {
                            state::DECK_FORWARD
                        };
                        return;
                    }
                }
                // ← and → step through the cards (the 0.25 s guard compares
                // milliseconds with seconds and never holds them back).
                let Some(a @ (Arrow::Left | Arrow::Right)) = arrow else {
                    return;
                };
                let next = if a == Arrow::Left {
                    self.chosen.checked_sub(1)
                } else {
                    Some(self.chosen + 1)
                };
                if let Some(n) = next.filter(|&n| n < self.cards.len()) {
                    self.chosen = n;
                    self.steps = 1;
                    self.state = if a == Arrow::Left {
                        state::DECK_BACK
                    } else {
                        state::DECK_FORWARD
                    };
                }
            }
            Screen::Game => {
                // The stick, an arrow taking the place of its own axis.
                let (mut x, mut y) = if self.pad {
                    (self.stick[0], self.stick[1])
                } else {
                    (0.0, 0.0)
                };
                if let Some(a) = arrow {
                    let (ax, ay) = a.xy();
                    if ax != 0.0 {
                        x = ax;
                    }
                    if ay != 0.0 {
                        y = ay;
                    }
                }
                if x == 0.0 && y == 0.0 {
                    return;
                }
                if self.has(flag::AI) {
                    return;
                }
                // At most one move a quarter second; others are dropped.
                if self.elapsed - self.selection_delay <= 0.25 {
                    return;
                }
                self.selection_delay = self.elapsed;
                self.arrow_in_game(x, y, fx);
            }
            Screen::Results => {}
        }
    }

    /// An arrow (or the stick) on the player's turn: `x` right, `y` down.
    fn arrow_in_game(&mut self, x: f32, y: f32, fx: &mut Vec<Effect>) {
        let Some(g) = &self.game else {
            return;
        };
        if !self.has(flag::SELECT_TRACK) {
            // Choosing a hand card by the sign of x, round the hand. Up or
            // down alone works out 0 / 0, a NaN that becomes the lowest whole
            // number: the last card.
            let n = g.player_hand.len();
            if n == 0 {
                return;
            }
            let next = if x == 0.0 {
                n - 1
            } else {
                (self.hand_card as i32 + x.signum() as i32).rem_euclid(n as i32) as usize
            };
            self.hand_card = next;
            let face = g.player_hand[next].value > 10;
            fx.push(Effect::Sound("GAMECaravanSwitchCard"));
            fx.push(Effect::LiftHandCard(Some(next)));
            if self.has(flag::TRACK_SETUP) {
                self.set(flag::DISABLE_A, face);
            }
            return;
        }
        if self.has(flag::TRACK_SETUP) {
            return;
        }
        let dx = if x.abs() < 0.01 { 0 } else { x.signum() as i32 };
        let dy = if y.abs() < 0.01 { 0 } else { y.signum() as i32 };
        if self.has(flag::DESTROY_TRACK) {
            fx.push(Effect::LowerTrack(self.track));
            self.track = (self.track as i32 + dx).clamp(0, 2) as usize;
            fx.push(Effect::Sound("GAMECaravanToggleTrack"));
            fx.push(Effect::LiftTrack(self.track));
            return;
        }
        self.handle_track_selection(dx, dy, fx);
    }

    /// `HandleTrackSelection` (`0074f360`): the player's tracks move with
    /// the arrows, the opponent's the other way; going above row 0 crosses
    /// to the facing track's row 0; rows stop at 6.
    fn handle_track_selection(&mut self, dx: i32, dy: i32, fx: &mut Vec<Effect>) {
        let (mut t, mut r) = (self.track as i32, self.row as i32);
        if t < 3 {
            t = (t + dx).clamp(0, 2);
            r += dy;
        } else {
            t = (t - dx).clamp(3, 5);
            r -= dy;
        }
        if r < 0 {
            t = 5 - t;
            r = 0;
        }
        self.track = t as usize;
        self.row = r.min(ROWS as i32 - 1) as usize;
        self.select_track_card(fx);
    }

    /// `SelectTrackCard` (`0074e6d0`): the column the card would take,
    /// whether it may go there.
    fn select_track_card(&mut self, fx: &mut Vec<Effect>) {
        let Some(g) = &self.game else {
            return;
        };
        let column = g.tracks[self.track][self.row].len();
        let valid = g.is_valid_placement(self.track, self.row, self.hand_card, false);
        let Some(card) = g.player_hand.get(self.hand_card).copied() else {
            return;
        };
        self.column = column as i32;
        fx.push(Effect::Sound(if valid {
            "GAMECaravanToggleTrack"
        } else {
            "GAMECaravanInvalidStackLocation"
        }));
        fx.push(Effect::Cursor {
            track: self.track,
            row: self.row,
            column: column.min(3),
            valid,
            card,
        });
        self.set(flag::DISABLE_A, !valid);
    }

    /// States 5 to 10: the deck screen's cards moving.
    fn deck_animation(&mut self, fx: &mut Vec<Effect>, clips: &dyn Clips, dice: Dice) {
        let t = self.elapsed;
        let (name, sound) = match self.state {
            state::DECK_FORWARD => ("Step-Forward", "GAMECaravanNavigate"),
            state::DECK_BACK => ("Step-Reverse", "GAMECaravanNavigate"),
            state::DECK_FLIP => ("Modify", "GAMECaravanAddRemove"),
            state::DECK_FAST_FORWARD => ("Fast-Forward", "GAMECaravanChangeDeck"),
            state::DECK_REWIND => ("Fast-Reverse", "GAMECaravanChangeDeck"),
            _ => ("Wipe", "GAMECaravanChangeDeck"),
        };
        let deck = format!("Deck_{name}");
        let available = format!("Available_{name}");
        if t == 0.0 {
            if self.state == state::DECK_FAST_FORWARD {
                fx.push(Effect::DeckTextures {
                    around: self.chosen as i32,
                });
            }
            fx.push(Effect::Activate {
                model: Model::Deck,
                sequence: deck.clone(),
            });
            fx.push(Effect::Activate {
                model: Model::Available,
                sequence: available.clone(),
            });
            self.flags |= flag::DISABLE_ALL;
            if matches!(self.state, state::DECK_FORWARD | state::DECK_BACK) {
                self.steps -= 1;
            }
            if self.state == state::DECK_BACK {
                // Back: the cards set once, for the first step's card.
                fx.push(Effect::DeckTextures {
                    around: self.chosen as i32 + self.steps,
                });
            }
            fx.push(Effect::Sound(sound));
        }
        for model in [Model::Table, Model::Deck, Model::Available] {
            fx.push(Effect::Update { model, time: t });
        }
        if t < clips.end(Model::Deck, &deck) {
            return;
        }
        let stop = |fx: &mut Vec<Effect>| {
            fx.push(Effect::Deactivate {
                model: Model::Deck,
                sequence: deck.clone(),
            });
            fx.push(Effect::Deactivate {
                model: Model::Available,
                sequence: available.clone(),
            });
        };
        let at_start = |fx: &mut Vec<Effect>| {
            for model in [Model::Table, Model::Deck, Model::Available] {
                fx.push(Effect::Update { model, time: 0.0 });
            }
        };
        match self.state {
            state::DECK_FORWARD | state::DECK_BACK => {
                if self.state == state::DECK_FORWARD {
                    // Forward: the cards for each step as it ends.
                    fx.push(Effect::DeckTextures {
                        around: self.chosen as i32 - self.steps,
                    });
                    at_start(fx);
                }
                if self.steps > 0 {
                    // Again, the sequence still running.
                    self.elapsed = 0.0;
                    self.steps -= 1;
                    return;
                }
                self.state = state::IDLE;
                stop(fx);
            }
            state::DECK_FLIP => {
                self.state = state::IDLE;
                fx.push(Effect::FlipSelected);
                at_start(fx);
                stop(fx);
            }
            state::DECK_FAST_FORWARD => {
                self.state = state::IDLE;
                stop(fx);
            }
            state::DECK_REWIND => {
                self.state = state::IDLE;
                fx.push(Effect::DeckTextures {
                    around: self.chosen as i32,
                });
                stop(fx);
            }
            _ => {
                // The wipe's end: to the game.
                self.state = state::IDLE;
                fx.push(Effect::FlipSelected);
                at_start(fx);
                stop(fx);
                self.opponent_next = true;
                self.flags |= flag::AI;
                self.flags &= !flag::DISABLE_ALL;
                self.update_flags();
                self.prepare_game(fx, dice);
                return;
            }
        }
        self.flags &= !flag::DISABLE_ALL;
        self.update_flags();
    }

    /// `PrepareGameMenu` (`0073ea90`): the lists written back, the deck
    /// models out and the game's in, the game dealt, both hands' cards;
    /// state 2.
    fn prepare_game(&mut self, fx: &mut Vec<Effect>, dice: Dice) {
        fx.push(self.save_deck());
        self.deal(dice);
        fx.push(Effect::UnloadDeckModels);
        fx.push(Effect::LoadGameModels);
        self.screen = Screen::Game;
        self.state = state::CAMERA_TO_GAME;
        self.placed = None;
        self.flags = flag::AI | flag::DISABLE_ALL;
        fx.push(Effect::HandTextures {
            npc: false,
            setup: true,
        });
        fx.push(Effect::HandTextures {
            npc: true,
            setup: true,
        });
        self.update_flags();
    }

    /// The deal (`PrepareGameMenu`): eight each from the player's deck (the
    /// deck screen's order) and the opponent's.
    fn deal(&mut self, dice: Dice) {
        let game = Game::new(self.player_deck(), self.npc_deck.clone(), dice);
        self.piles = [game.player_deck.len(), game.npc_deck.len()];
        self.game = Some(game);
        self.hand_card = 0;
        self.track = 0;
        self.row = 0;
        self.column = 0;
        self.joker_rows = Default::default();
    }

    /// The side whose hand moves: the opponent's on its turn (`CAF_AI`).
    fn npc_side(&self) -> bool {
        self.has(flag::AI)
    }

    /// States 11 to 14: a hand card out (and onto its row when placed).
    fn hand_animation(&mut self, fx: &mut Vec<Effect>, clips: &dyn Clips) {
        let t = self.elapsed;
        let npc = self.npc_side();
        let hand = Model::Hand { npc };
        let Some(g) = &self.game else {
            return;
        };
        let eight = match self.state {
            state::INTRO_DEAL | state::INTRO_DISCARD => true,
            _ if npc => g.next_npc.is_none(),
            _ => g.next_player.is_none(),
        };
        let out = hand_out(eight, self.hand_card);
        let places = matches!(self.state, state::INTRO_DEAL | state::PLACE_CARD);
        let row_model = Model::Row {
            track: self.track,
            row: self.row,
        };
        let column = self.column.clamp(0, 3) as usize;
        let row_in = row_sequence(column, true);
        if t == 0.0 {
            if places {
                if let Some(card) = self.placed {
                    fx.push(Effect::PlacedCard {
                        track: self.track,
                        row: self.row,
                        column,
                        card,
                    });
                }
            }
            if !npc {
                fx.push(Effect::LiftHandCard(None));
            }
            self.flags |= flag::DISABLE_ALL;
            self.selection_delay = 0.0;
            fx.push(Effect::Sound(match self.state {
                state::INTRO_DEAL => "GAMECaravanConfirmAddtoDeck",
                state::INTRO_DISCARD => "GAMECaravanRemoveCard",
                state::PLACE_CARD => "GAMECaravanConfirmAddToTrack",
                _ if eight => "GAMECaravanRemoveCard",
                _ => "GAMECaravanAddRemoveCard",
            }));
            fx.push(Effect::Activate {
                model: hand,
                sequence: out.clone(),
            });
            if places {
                fx.push(Effect::Activate {
                    model: row_model,
                    sequence: row_in.clone(),
                });
            }
        }
        fx.push(Effect::Update {
            model: hand,
            time: t,
        });
        if places {
            fx.push(Effect::Update {
                model: row_model,
                time: t,
            });
        }
        let mut done = t >= clips.end(hand, &out);
        if places {
            done = done && t >= clips.end(row_model, &row_in);
        }
        if !done {
            return;
        }
        fx.push(Effect::Deactivate {
            model: hand,
            sequence: out,
        });
        if places {
            fx.push(Effect::Deactivate {
                model: row_model,
                sequence: row_in,
            });
        }
        if let Some(g) = &self.game {
            let pile = if npc { &g.npc_deck } else { &g.player_deck };
            self.piles[usize::from(npc)] = pile.len();
        }
        if matches!(self.state, state::INTRO_DEAL | state::INTRO_DISCARD) {
            self.setup_end(fx, npc);
            return;
        }
        fx.push(Effect::HandTextures {
            npc,
            setup: self.has(flag::TRACK_SETUP),
        });
        fx.push(Effect::ResetPose {
            model: hand,
            sequence: "Deck-Card1_Out".into(),
        });
        fx.push(Effect::DeactivateAll(hand));
        if self.state == state::PLACE_CARD {
            match self.placed.map(|c| c.value) {
                Some(JACK) => {
                    self.state = state::JACK_CLEAR;
                    return;
                }
                Some(JOKER) => {
                    self.joker_rows = self.joker_marks();
                    self.track = 0;
                    self.state = state::JOKER_CLEAR;
                    return;
                }
                _ => {}
            }
        }
        self.update_flags();
        self.state = state::PROCESS_AI;
    }

    /// The rows a joker takes (state 13's end): on every track, each row up
    /// to the first empty one whose number card has the value of the
    /// joker's row's (its suit, when that's an ace), not the joker's own.
    fn joker_marks(&self) -> [Vec<usize>; TRACKS] {
        let mut marks: [Vec<usize>; TRACKS] = Default::default();
        let Some(g) = &self.game else {
            return marks;
        };
        let Some(base) = g.tracks[self.track][self.row].first().copied() else {
            return marks;
        };
        for (t, mark) in marks.iter_mut().enumerate() {
            for r in 0..ROWS {
                let Some(c) = g.tracks[t][r].first() else {
                    break;
                };
                let same = if base.value == ACE {
                    c.suit == base.suit
                } else {
                    c.value == base.value
                };
                if same && (t, r) != (self.track, self.row) {
                    mark.push(r);
                }
            }
        }
        marks
    }

    /// The end of a card placed or thrown away while starting the caravans
    /// (states 11 and 12): the hand keeps the eight-card look while its
    /// side's last track is empty; once the player's last track and the
    /// opponent's both have a card, after the player's card, starting is
    /// over.
    fn setup_end(&mut self, fx: &mut Vec<Effect>, npc: bool) {
        let discard = self.state == state::INTRO_DISCARD;
        let Some(g) = &mut self.game else {
            return;
        };
        let hand = Model::Hand { npc };
        let player_last_empty = g.tracks[2][0].is_empty();
        let npc_last_empty = g.tracks[3][0].is_empty();
        let setup = self.flags & flag::TRACK_SETUP != 0;
        if (player_last_empty || npc) && (npc_last_empty || !npc) {
            fx.push(Effect::HandTextures { npc, setup });
            fx.push(Effect::ResetPose {
                model: hand,
                sequence: "Deck8-Card1_Out".into(),
            });
        } else {
            fx.push(Effect::HandTextures { npc, setup });
            fx.push(Effect::ResetPose {
                model: hand,
                sequence: "Deck-Card1_Out".into(),
            });
            if !npc && !player_last_empty && !npc_last_empty {
                self.flags &= !flag::TRACK_SETUP;
                g.setup = false;
                self.tutorial = true;
                fx.push(Effect::Tutorial(HELP_CONTRACT_WAR));
            }
        }
        fx.push(Effect::DeactivateAll(hand));
        if discard && !npc {
            // The player keeps the turn.
            self.state = state::IDLE;
            self.flags &= !flag::DISABLE_ALL;
            return;
        }
        if npc {
            // The player's last hand card chosen and raised.
            let n = g.player_hand.len();
            self.hand_card = n.saturating_sub(1);
            let face = g.player_hand.last().is_some_and(|c| c.value > 10);
            fx.push(Effect::LiftHandCard(Some(self.hand_card)));
            if g.setup {
                self.set(flag::DISABLE_A, face);
            }
        }
        self.state = state::PROCESS_AI;
    }

    /// Launches a column's sequences on the rows from `from_row` that have a
    /// card there; how many.
    fn launch(&mut self, fx: &mut Vec<Effect>, from_row: usize, coming_in: bool) -> usize {
        let Some(g) = &self.game else {
            return 0;
        };
        let track = self.track;
        let mut n = 0;
        for r in from_row..ROWS {
            if (self.column as usize) < g.tracks[track][r].len() {
                fx.push(Effect::Activate {
                    model: Model::Row { track, row: r },
                    sequence: row_sequence(self.column as usize, coming_in),
                });
                n += 1;
            }
        }
        n
    }

    /// The next column with cards (down from the top going out, up from the
    /// bottom coming in), launched; false when there are no more.
    fn next_column(&mut self, fx: &mut Vec<Effect>, from_row: usize, coming_in: bool) -> bool {
        loop {
            if coming_in {
                if self.column > 3 {
                    return false;
                }
                self.column += 1;
                if self.column > 3 {
                    return false;
                }
            } else {
                if self.column < 1 {
                    return false;
                }
                self.column -= 1;
            }
            let n = self.launch(fx, from_row, coming_in);
            self.launched += n;
            if n > 0 {
                return true;
            }
        }
    }

    /// States 15, 17 and 18: a track's cards going out a column at a time,
    /// highest first (a thrown-away track; a jack's row and those after it),
    /// or coming back lowest first; each column starts the clock again.
    fn columns(&mut self, fx: &mut Vec<Effect>, clips: &dyn Clips) {
        let coming_in = self.state == state::JACK_FINISH;
        let from_row = if self.state == state::DISCARD_TRACK {
            0
        } else {
            self.row
        };
        let track = self.track;
        if self.elapsed == 0.0 {
            self.column = if coming_in { -1 } else { 4 };
            let any = self.next_column(fx, from_row, coming_in);
            self.flags |= flag::DISABLE_ALL;
            if !any {
                if coming_in {
                    // Nothing to bring back: done at once.
                    self.columns_done(fx);
                    return;
                }
                // Nothing to take: column 0's time still passes.
                self.column = 0;
            }
            if self.state == state::DISCARD_TRACK {
                fx.push(Effect::Sound("GAMECaravanRemoveCard"));
                self.selection_delay = 0.0;
            }
        }
        let Some(g) = &self.game else {
            return;
        };
        for r in from_row..ROWS {
            if g.tracks[track][r].is_empty() {
                break;
            }
            fx.push(Effect::Update {
                model: Model::Row { track, row: r },
                time: self.elapsed,
            });
        }
        let column = self.column.clamp(0, 3) as usize;
        let reference = Model::Row {
            track,
            row: from_row,
        };
        if self.elapsed <= clips.end(reference, &row_sequence(column, coming_in)) {
            return;
        }
        for r in from_row..ROWS {
            fx.push(Effect::Deactivate {
                model: Model::Row { track, row: r },
                sequence: row_sequence(column, coming_in),
            });
        }
        if self.launched > 0 {
            return;
        }
        if self.next_column(fx, from_row, coming_in) {
            self.elapsed = 0.0;
            return;
        }
        self.columns_done(fx);
    }

    /// Every column done (the ends of states 15, 17 and 18).
    fn columns_done(&mut self, fx: &mut Vec<Effect>) {
        let track = self.track;
        let Some(g) = &mut self.game else {
            return;
        };
        match self.state {
            state::DISCARD_TRACK => {
                g.discard_track(track);
                self.flags &= !(flag::DESTROY_TRACK | flag::SELECT_TRACK);
                self.state = state::PROCESS_AI;
            }
            state::JACK_CLEAR => {
                // The rows after the jack's (or each marked row) move up; a
                // jack's last row is emptied, a joker's left as it was.
                let rows = if self.flags & flag::PROCESS_JOKER == 0 {
                    vec![self.row]
                } else {
                    let mut rows = Vec::new();
                    let mut marked = std::mem::take(&mut self.joker_rows[track]);
                    while !marked.is_empty() {
                        let r = marked.remove(0);
                        rows.push(r);
                        for m in marked.iter_mut() {
                            if r < *m {
                                *m -= 1;
                            }
                        }
                    }
                    rows
                };
                let jack = self.flags & flag::PROCESS_JOKER == 0;
                for r in rows {
                    for rr in r..ROWS - 1 {
                        g.tracks[track][rr] = g.tracks[track][rr + 1].clone();
                        for column in 0..g.tracks[track][rr].len() {
                            fx.push(Effect::TrackCard {
                                track,
                                row: rr,
                                column,
                            });
                        }
                    }
                    if jack {
                        g.tracks[track][ROWS - 1].clear();
                    }
                }
                self.state = state::JACK_FINISH;
            }
            _ => {
                g.update_track_value(track);
                self.state = if self.flags & flag::PROCESS_JOKER != 0 {
                    state::JOKER_CLEAR
                } else {
                    state::PROCESS_AI
                };
            }
        }
    }

    /// State 21: the game over? Else the player's turn starts, or the
    /// opponent's move is chosen (`ProcessAI`) and carried out.
    fn turn(&mut self, fx: &mut Vec<Effect>, dice: Dice) {
        self.flags |= flag::AI;
        let Some(g) = &mut self.game else {
            return;
        };
        if g.is_game_over() {
            let won = g.player_won;
            self.set(flag::PLAYER_WIN, won);
            self.state = state::GAME_OVER;
            return;
        }
        if !self.opponent_next {
            let n = g.player_hand.len();
            self.hand_card = n.saturating_sub(1);
            self.track = 0;
            self.placed = None;
            fx.push(Effect::LiftHandCard(Some(self.hand_card)));
            self.opponent_next = true;
            self.state = state::IDLE;
            self.flags &= !(flag::DISABLE_ALL | flag::AI);
            self.update_flags();
            return;
        }
        self.opponent_next = false;
        let p = g.process_ai(self.difficulty);
        self.hand_card = p.card;
        let had_next = g.next_npc.is_some();
        if g.setup {
            if p.action == ai::action::DISCARD_CARD {
                g.discard(true, p.card, dice);
                self.state = state::INTRO_DISCARD;
                self.opponent_next = true;
            } else {
                let track = g.setup_track(true).unwrap_or(5);
                self.track = track;
                self.row = 0;
                self.column = 0;
                self.placed = g.npc_hand.get(p.card).copied();
                g.play(true, p.card, track, 0, dice);
                self.state = state::INTRO_DEAL;
            }
        } else if p.action == ai::action::DISCARD_CARD {
            g.discard(true, p.card, dice);
            self.state = state::DISCARD_CARD;
        } else if p.action == ai::action::DESTROY_TRACK {
            self.track = p.track;
            g.info[p.track].value = 0;
            self.state = state::DISCARD_TRACK;
        } else {
            self.track = p.track;
            self.row = p.row;
            self.column = g.tracks[p.track][p.row].len() as i32;
            self.placed = g.npc_hand.get(p.card).copied();
            // A jack's or joker's rows go in states 17 to 19.
            g.play(true, p.card, p.track, p.row, dice);
            self.state = state::PLACE_CARD;
        }
        let drew = had_next && !matches!(self.state, state::INTRO_DEAL | state::DISCARD_TRACK);
        if drew {
            fx.push(Effect::DealCard { npc: true });
            if g.next_npc.is_none() {
                fx.push(Effect::CullDrawPile { npc: true });
            }
        }
    }

    /// State 22 (`PrepareResultsMenu`): the result, the money changing
    /// hands, back to the betting camera.
    fn game_over(&mut self, fx: &mut Vec<Effect>) {
        self.flags |= flag::DISABLE_ALL;
        // A forfeit leaves `CAF_PLAYERWIN` clear: a loss.
        let won = self.has(flag::PLAYER_WIN);
        fx.push(Effect::Sound(if won {
            "GAMECaravanWin"
        } else {
            "GAMECaravanLose"
        }));
        fx.push(Effect::Settle {
            won,
            stake: self.stake,
        });
        self.state = state::CAMERA_TO_RESULTS;
    }

    // -----------------------------------------------------------------------
    // Input.

    /// A key (`00749360`): on the results screen any key closes; W, A, F,
    /// Q, E, S and R are buttons. (The arrows: [`Menu::arrow_key`].)
    pub fn key(&mut self, key: char, dice: Dice) -> Vec<Effect> {
        let mut fx = Vec::new();
        if self.screen == Screen::Results && self.has(flag::IS_ON_TOP) {
            self.close(&mut fx);
            return fx;
        }
        let code = match key.to_ascii_uppercase() {
            'W' => button::A,
            'A' => button::X,
            'F' => button::Y,
            'Q' => button::LT,
            'E' => button::RT,
            'S' => button::RB,
            'R' => return self.gamepad_up(button::B),
            _ => return fx,
        };
        self.gamepad(code, dice)
    }

    /// An arrow key (`00749360`: kept for the update, in state 0 only).
    pub fn arrow_key(&mut self, a: Arrow) {
        if self.state == state::IDLE {
            self.arrow = Some(a);
        }
    }

    /// A click on a tile (`DoClick` `00749560`): in state 0 with nothing
    /// playing, its button.
    pub fn click(&mut self, tile: i32, dice: Dice) -> Vec<Effect> {
        if self.state != state::IDLE || self.has(flag::DISABLE_ALL) {
            return Vec::new();
        }
        match tile {
            7 | 14 => self.gamepad(button::Y, dice),
            10 => self.gamepad(button::RB, dice),
            12 | 19 | 37 => self.gamepad(button::A, dice),
            13 | 36 => self.gamepad(button::X, dice),
            20 => self.gamepad(button::LT, dice),
            21 => self.gamepad(button::RT, dice),
            22 | 38 => self.gamepad_up(button::B),
            _ => Vec::new(),
        }
    }

    /// A button pressed (`DoGamepad` `00747d30`).
    pub fn gamepad(&mut self, code: u8, dice: Dice) -> Vec<Effect> {
        let mut fx = Vec::new();
        if self.has(flag::DISABLE_ALL) || self.tutorial || !self.has(flag::IS_ON_TOP) {
            return fx;
        }
        if self.has(flag::AI) || self.state != state::IDLE {
            return fx;
        }
        match self.screen {
            Screen::Ante => self.ante_button(code, &mut fx, dice),
            Screen::Deck => self.deck_button(code, &mut fx, dice),
            Screen::Game => self.game_button(code, &mut fx, dice),
            Screen::Results => {
                // A, X and Y ask their flags (left from the game) first.
                let off = match code {
                    button::A => self.has(flag::DISABLE_A),
                    button::X => self.has(flag::DISABLE_X),
                    button::Y => self.has(flag::DISABLE_Y),
                    button::START
                    | button::BACK
                    | button::LT
                    | button::RT
                    | button::LB
                    | button::RB
                    | button::LTHUMB
                    | button::RTHUMB => false,
                    _ => true,
                };
                if !off {
                    self.close(&mut fx);
                }
            }
        }
        fx
    }

    /// The ante's buttons: A matches, X raises (the "How many?" box), Y
    /// accepts (`PrepareDeckMenu`: the stake fixed, to the deck screen).
    fn ante_button(&mut self, code: u8, fx: &mut Vec<Effect>, dice: Dice) {
        match code {
            button::A if !self.has(flag::DISABLE_A) => {
                let old = self.bet.player_ante;
                self.bet.automatch();
                if self.money.add(self.most, self.bet.player_ante - old, dice) {
                    self.state = state::ANTE_MONEY_IN;
                    fx.push(Effect::Money);
                }
            }
            button::X if !self.has(flag::DISABLE_X) => {
                self.flags |= flag::DISABLE_ALL;
                let max = self.bet.player_funds - self.bet.player_ante;
                if max > 0 {
                    self.prompt = true;
                    fx.push(Effect::HowMany { max });
                }
            }
            button::Y if !self.has(flag::DISABLE_Y) => {
                self.stake = self.bet.stake();
                fx.push(Effect::LoadDeckModels);
                fx.push(Effect::DeckTextures {
                    around: FIRST_CHOSEN as i32,
                });
                self.screen = Screen::Deck;
                self.state = state::CAMERA_TO_DECK;
                fx.push(Effect::Sound("UIMenuOK"));
            }
            _ => {}
        }
        self.update_flags();
    }

    /// The raise's answer (`ItemSelectCallback` `0074a090`): the ante up by
    /// it, the opponent matching up to its most; the money put down counts
    /// from the old ante, not the raise (as the game has it).
    pub fn how_many(&mut self, n: i32, dice: Dice) -> Vec<Effect> {
        let mut fx = Vec::new();
        self.prompt = false;
        let added = n - self.bet.player_ante;
        self.bet.raise(n, self.barter);
        self.flags &= !flag::DISABLE_ALL;
        let most = self.bet.npc_most(self.barter);
        let extra = (self.bet.player_ante.min(most) - self.bet.npc_ante).max(0);
        self.update_flags();
        if self.money.add(most, added + extra, dice) {
            self.state = state::ANTE_MONEY_IN;
            fx.push(Effect::Money);
        }
        fx
    }

    /// The deck screen's buttons: A puts the chosen card in, X takes it out,
    /// Y plays with 30 or more, RB makes a deck at random.
    fn deck_button(&mut self, code: u8, fx: &mut Vec<Effect>, dice: Dice) {
        match code {
            button::A if !self.has(flag::DISABLE_A) => {
                if let Some(c) = self.cards.get_mut(self.chosen) {
                    c.in_deck = true;
                    self.in_deck += 1;
                    self.state = state::DECK_FLIP;
                }
            }
            button::X if !self.has(flag::DISABLE_X) => {
                if let Some(c) = self.cards.get_mut(self.chosen) {
                    c.in_deck = false;
                    self.in_deck = self.in_deck.saturating_sub(1);
                    self.state = state::DECK_FLIP;
                }
            }
            button::Y if self.in_deck >= MIN_DECK => {
                self.state = state::DECK_WIPE;
            }
            button::RB if !self.has(flag::DISABLE_RB) => {
                // Every card out, then 30 (or, with more than 30, a number
                // from 30 up to one fewer than all) picked at random.
                let owned = self.cards.len();
                for c in &mut self.cards {
                    c.in_deck = false;
                }
                self.in_deck = 0;
                let n = if owned < 31 {
                    MIN_DECK
                } else {
                    MIN_DECK + dice(owned - MIN_DECK)
                };
                let mut pool: Vec<usize> = (0..owned).collect();
                for _ in 0..n.min(owned) {
                    let r = dice(pool.len());
                    let i = pool.remove(r);
                    if !self.cards[i].in_deck {
                        self.cards[i].in_deck = true;
                        self.in_deck += 1;
                    }
                }
                fx.push(Effect::DeckTextures {
                    around: self.chosen as i32,
                });
            }
            _ => {}
        }
        self.update_flags();
    }

    /// The mouse in state 0 without a pad (`DoIdle`): a button held
    /// (`InterfaceManager::fMouseHeldTime`, +0x48, above 0) starts
    /// following a drag from the scrollbar's value `bar`; let go on the deck
    /// screen, the cards move as far as the bar did ([`Menu::meter_release`]).
    /// (So the bar moved without a button held, by the wheel, moves no
    /// cards: the next press starts from where it is.)
    pub fn mouse(&mut self, held: bool, bar: usize) {
        if self.pad || self.state != state::IDLE {
            return;
        }
        if !held && self.dragging && self.screen == Screen::Deck {
            self.dragging = false;
            self.meter_release(bar);
        } else if !self.dragging && held {
            self.drag_from = Some(bar);
            self.dragging = true;
        }
    }

    /// The deck screen's scrollbar pressed: where a drag starts.
    pub fn meter_press(&mut self) {
        self.drag_from = Some(self.chosen);
    }

    /// The scrollbar let go at a card (`007492f0` and `DoIdle`): one to four
    /// cards on, a step each; five or more, one fast move.
    pub fn meter_release(&mut self, at: usize) {
        let Some(from) = self.drag_from.take() else {
            return;
        };
        if self.state != state::IDLE || at >= self.cards.len() {
            return;
        }
        let d = at as i32 - from as i32;
        self.chosen = at;
        match d {
            0 => {}
            1..=4 => {
                self.steps = d;
                self.state = state::DECK_FORWARD;
            }
            -4..=-1 => {
                self.steps = -d;
                self.state = state::DECK_BACK;
            }
            d if d >= 5 => self.state = state::DECK_FAST_FORWARD,
            _ => self.state = state::DECK_REWIND,
        }
    }

    /// The game's buttons on the player's turn.
    fn game_button(&mut self, code: u8, fx: &mut Vec<Effect>, dice: Dice) {
        let selecting = self.has(flag::SELECT_TRACK);
        let setup = self.has(flag::TRACK_SETUP);
        match code {
            button::A if !self.has(flag::DISABLE_A) => {
                if setup && !selecting {
                    // Onto the first empty track of the three; it can't be
                    // moved.
                    let Some(g) = &self.game else {
                        return;
                    };
                    self.track = g.setup_track(false).unwrap_or(0);
                    self.row = 0;
                    self.select_track_card(fx);
                    fx.push(Effect::LiftHandCard(None));
                    fx.push(Effect::Sound("GAMECaravanAddToTrack"));
                    self.flags |= flag::SELECT_TRACK;
                } else if setup {
                    self.flags &= !flag::SELECT_TRACK;
                    fx.push(Effect::HideCursors);
                    let Some(g) = &mut self.game else {
                        return;
                    };
                    self.placed = g.player_hand.get(self.hand_card).copied();
                    self.column = g.tracks[self.track][self.row].len() as i32;
                    g.play(false, self.hand_card, self.track, self.row, dice);
                    self.state = state::INTRO_DEAL;
                } else if self.has(flag::DESTROY_TRACK) {
                    fx.push(Effect::LowerTrack(self.track));
                    if let Some(g) = &mut self.game {
                        g.info[self.track].value = 0;
                    }
                    self.state = state::DISCARD_TRACK;
                } else if !selecting {
                    fx.push(Effect::Sound("GAMECaravanAddToTrack"));
                    self.flags |= flag::SELECT_TRACK;
                    self.track = 0;
                    self.row = 0;
                    self.column = 0;
                    self.handle_track_selection(0, 0, fx);
                } else {
                    self.flags &= !flag::SELECT_TRACK;
                    fx.push(Effect::HideCursors);
                    let Some(g) = &mut self.game else {
                        return;
                    };
                    let had_next = g.next_player.is_some();
                    self.placed = g.player_hand.get(self.hand_card).copied();
                    self.column = g.tracks[self.track][self.row].len() as i32;
                    g.play(false, self.hand_card, self.track, self.row, dice);
                    if had_next {
                        fx.push(Effect::DealCard { npc: false });
                        if g.next_player.is_none() {
                            fx.push(Effect::CullDrawPile { npc: false });
                        }
                    }
                    self.state = state::PLACE_CARD;
                }
            }
            button::LT if !self.has(flag::DISABLE_LT) && !selecting => {
                let Some(g) = &mut self.game else {
                    return;
                };
                if g.player_hand.is_empty() {
                    return;
                }
                let had_next = g.next_player.is_some();
                let i = self.hand_card.min(g.player_hand.len() - 1);
                g.discard(false, i, dice);
                if had_next {
                    fx.push(Effect::DealCard { npc: false });
                    if g.next_player.is_none() {
                        fx.push(Effect::CullDrawPile { npc: false });
                    }
                }
                self.state = if setup {
                    state::INTRO_DISCARD
                } else {
                    state::DISCARD_CARD
                };
            }
            button::RT if !self.has(flag::DISABLE_RT) && !selecting && !setup => {
                self.flags |= flag::DESTROY_TRACK | flag::SELECT_TRACK;
                self.track = 0;
                fx.push(Effect::LiftTrack(0));
            }
            _ => {}
        }
        self.update_flags();
    }

    /// B let go (`DoGamepadUpEvent` `007490e0`): leaves the ante or the deck
    /// screen; in a game, cancels choosing a place, or asks about quitting;
    /// closes the results.
    pub fn gamepad_up(&mut self, code: u8) -> Vec<Effect> {
        let mut fx = Vec::new();
        if code != button::B || self.has(flag::DISABLE_ALL) {
            return fx;
        }
        match self.screen {
            Screen::Ante | Screen::Deck | Screen::Results => self.close(&mut fx),
            Screen::Game => {
                if self.has(flag::SELECT_TRACK) {
                    if self.has(flag::DESTROY_TRACK) {
                        fx.push(Effect::LowerTrack(self.track));
                    } else {
                        fx.push(Effect::HideCursors);
                    }
                    self.flags &= !(flag::DESTROY_TRACK | flag::SELECT_TRACK | flag::DISABLE_A);
                    self.flags |= flag::DISABLE_X;
                } else if self.has(flag::IS_ON_TOP) {
                    self.flags |= flag::DISABLE_ALL;
                    self.prompt = true;
                    fx.push(Effect::ConfirmForfeit);
                }
            }
        }
        fx
    }

    /// The quit box's answer (`CancelConfirmFunc` `0074a270`): Yes ends the
    /// game as a loss on the next update.
    pub fn forfeit_answer(&mut self, yes: bool) {
        self.prompt = false;
        if yes {
            self.flags |= flag::MARK_FOR_CLOSE;
        } else {
            self.flags &= !flag::DISABLE_ALL;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    thread_local! {
        static VISITED: std::cell::RefCell<std::collections::BTreeSet<u8>> = Default::default();
    }

    /// Every sequence half a second long.
    struct Half;
    impl Clips for Half {
        fn end(&self, _: Model, _: &str) -> f32 {
            0.5
        }
    }

    struct Rng(u64);
    impl Rng {
        fn next(&mut self, n: usize) -> usize {
            self.0 = self
                .0
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((self.0 >> 33) as usize) % n.max(1)
        }
    }

    /// n cards: aces to tens and the face cards in turn, the suits round.
    fn cards(n: u32, base: u32) -> Vec<Card> {
        const VALUES: [i32; 14] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 15];
        (0..n)
            .map(|i| Card {
                form: FormId(base + i),
                value: VALUES[(i % 14) as usize],
                suit: (i % 4) as i32 + 1,
            })
            .collect()
    }

    /// Updates every 16 ms until state 0 with nothing playing (or `max`
    /// updates); tutorials read at once.
    fn settle(m: &mut Menu, tick: &mut u32, rng: &mut Rng, all: &mut Vec<Effect>) -> bool {
        for _ in 0..20_000 {
            *tick += 16;
            let fx = m.update(*tick, &Half, &mut |n| rng.next(n));
            VISITED.with(|v| v.borrow_mut().insert(m.state));
            if fx.iter().any(|e| matches!(e, Effect::Tutorial(_))) {
                m.tutorial_done();
            }
            let closed = fx.contains(&Effect::Close);
            all.extend(fx);
            if closed {
                return true;
            }
            let player_turn = m.screen != Screen::Game || !m.has(flag::AI);
            if m.state == state::IDLE && !m.has(flag::DISABLE_ALL) && player_turn {
                return false;
            }
        }
        panic!(
            "stuck in state {} on {:?} (flags {:#x})",
            m.state, m.screen, m.flags
        );
    }

    /// The player's move: a valid placement if there is one, else a card
    /// thrown away (or a track).
    fn play_turn(m: &mut Menu, tick: &mut u32, rng: &mut Rng, all: &mut Vec<Effect>) {
        let g = m.game.as_ref().unwrap();
        let setup = m.has(flag::TRACK_SETUP);
        let mut choice = None;
        'find: for h in 0..g.player_hand.len() {
            for t in 0..TRACKS {
                for r in 0..ROWS {
                    if g.is_valid_placement(t, r, h, false) {
                        if setup && t != g.setup_track(false).unwrap_or(9) {
                            continue;
                        }
                        choice = Some((h, t, r));
                        break 'find;
                    }
                }
            }
        }
        let mut dice = |n: usize| rng.next(n);
        match choice {
            Some((h, t, r)) => {
                m.hand_card = h;
                m.flags &= !flag::DISABLE_A;
                all.extend(m.key('W', &mut dice));
                assert!(m.has(flag::SELECT_TRACK), "W picks a card");
                if !setup {
                    m.track = t;
                    m.row = r;
                    let mut fx = Vec::new();
                    m.select_track_card(&mut fx);
                    all.extend(fx);
                }
                assert!(!m.has(flag::DISABLE_A));
                all.extend(m.key('W', &mut dice));
                assert!(matches!(m.state, state::INTRO_DEAL | state::PLACE_CARD));
            }
            None if !g.player_hand.is_empty() && !m.has(flag::DISABLE_LT) => {
                all.extend(m.key('Q', &mut dice));
            }
            None => {
                all.extend(m.key('E', &mut dice));
                all.extend(m.key('W', &mut dice));
            }
        }
        let _ = (tick,);
    }

    /// Whole games through the menu, from the ante to the results.
    #[test]
    fn whole_games_through_the_menu() {
        for seed in 0..40u64 {
            let mut rng = Rng(seed * 7919 + 1);
            let mut tick = 1000u32;
            let mut all = Vec::new();
            let out = cards(12, 0x100);
            let mine = cards(30, 0x200);
            let (mut m, fx) = Menu::open(
                FormId(0x50),
                cards(54, 0x300),
                (seed % 4) as i32,
                &out,
                &mine,
                Bet::new(500, 200, 0.5),
                50.0,
                &mut |n| rng.next(n),
            );
            assert!(fx.contains(&Effect::Tutorial(HELP_BETTING)));
            m.tutorial_done();
            assert!(!settle(&mut m, &mut tick, &mut rng, &mut all));
            // Match, accept: the deck screen.
            all.extend(m.key('W', &mut |n| rng.next(n)));
            assert!(!settle(&mut m, &mut tick, &mut rng, &mut all));
            assert_eq!(m.bet.player_ante, m.bet.npc_ante);
            all.extend(m.key('F', &mut |n| rng.next(n)));
            assert_eq!(m.screen, Screen::Deck);
            assert_eq!(m.stake, m.bet.npc_ante);
            assert!(!settle(&mut m, &mut tick, &mut rng, &mut all));
            // Play: the wipe, the deal, the camera.
            all.extend(m.key('F', &mut |n| rng.next(n)));
            assert_eq!(m.state, state::DECK_WIPE);
            let mut turns = 0;
            loop {
                if settle(&mut m, &mut tick, &mut rng, &mut all) {
                    break;
                }
                match m.screen {
                    Screen::Game => {
                        play_turn(&mut m, &mut tick, &mut rng, &mut all);
                        turns += 1;
                        assert!(turns < 500, "seed {seed}: too many turns");
                    }
                    Screen::Results => {
                        all.extend(m.key('x', &mut |n| rng.next(n)));
                    }
                    s => panic!("seed {seed}: back on {s:?}"),
                }
            }
            assert!(
                all.iter().any(|e| matches!(e, Effect::Settle { .. })),
                "seed {seed}: no result"
            );
            assert!(all.iter().any(|e| matches!(e, Effect::SaveDeck { .. })));
        }
        // Every state a game goes through came up.
        let seen = VISITED.with(|v| v.borrow().clone());
        for s in [
            state::CAMERA_TO_DECK,
            state::CAMERA_TO_GAME,
            state::CAMERA_TO_RESULTS,
            state::ANTE_MONEY_IN,
            state::DECK_WIPE,
            state::INTRO_DEAL,
            state::INTRO_DISCARD,
            state::PLACE_CARD,
            state::DISCARD_CARD,
            state::DISCARD_TRACK,
            state::JACK_CLEAR,
            state::JACK_FINISH,
            state::JOKER_CLEAR,
            state::PROCESS_AI,
            state::GAME_OVER,
            state::CLOSING,
        ] {
            assert!(seen.contains(&s), "state {s} never came up: {seen:?}");
        }
    }

    /// Leaving the ante closes at once; leaving the deck screen keeps the
    /// deck.
    #[test]
    fn leaving_early() {
        let mut rng = Rng(3);
        let mut tick = 0;
        let mut all = Vec::new();
        let (mut m, _) = Menu::open(
            FormId(0x50),
            cards(30, 0x300),
            0,
            &cards(5, 0x100),
            &cards(30, 0x200),
            Bet::new(100, 100, 0.5),
            0.0,
            &mut |n| rng.next(n),
        );
        m.tutorial_done();
        settle(&mut m, &mut tick, &mut rng, &mut all);
        let fx = m.key('R', &mut |n| rng.next(n));
        assert!(fx.is_empty());
        assert_eq!(m.state, state::CLOSING);
        assert!(settle(&mut m, &mut tick, &mut rng, &mut all));
        assert!(!all.iter().any(|e| matches!(e, Effect::SaveDeck { .. })));
    }

    /// `PrepareCaravanData`: out of the deck then in it, sorted by value.
    #[test]
    fn deck_screen_order() {
        let (m, _) = Menu::open(
            FormId(0x50),
            Vec::new(),
            0,
            &cards(3, 0x100),
            &cards(30, 0x200),
            Bet::new(0, 0, 0.5),
            0.0,
            &mut |_| 0,
        );
        let values: Vec<i32> = m.cards.iter().map(|c| c.card.value).collect();
        let mut sorted = values.clone();
        sorted.sort();
        assert_eq!(values, sorted);
        assert_eq!(m.in_deck, 30);
        assert_eq!(m.cards.len(), 33);
    }

    #[test]
    fn the_tutorials_are_ids_0x1e_to_0x21() {
        assert_eq!(tutorial_id(HELP_BETTING), Some(0x1E));
        assert_eq!(tutorial_id(HELP_DECK), Some(0x1F));
        assert_eq!(tutorial_id(HELP_STARTING), Some(0x20));
        assert_eq!(tutorial_id(HELP_CONTRACT_WAR), Some(0x21));
        assert_eq!(tutorial_id("HelpHacking"), None);
    }

    /// A menu at the deck screen, idle, with 33 cards.
    fn at_deck() -> (Menu, u32, Rng) {
        let mut rng = Rng(5);
        let mut tick = 0;
        let mut all = Vec::new();
        let (mut m, _) = Menu::open(
            FormId(0x50),
            cards(30, 0x300),
            0,
            &cards(3, 0x100),
            &cards(30, 0x200),
            Bet::new(500, 200, 0.5),
            50.0,
            &mut |n| rng.next(n),
        );
        m.tutorial_done();
        settle(&mut m, &mut tick, &mut rng, &mut all);
        m.key('W', &mut |n| rng.next(n));
        settle(&mut m, &mut tick, &mut rng, &mut all);
        m.key('F', &mut |n| rng.next(n));
        settle(&mut m, &mut tick, &mut rng, &mut all);
        assert_eq!((m.screen, m.state), (Screen::Deck, state::IDLE));
        (m, tick, rng)
    }

    /// A button held starts a drag from the bar; let go, the cards move as
    /// far as the bar did. The bar moved with no button held (the wheel)
    /// moves nothing, and the next press starts from where it is.
    #[test]
    fn the_mouse_drags_from_where_the_bar_was_pressed() {
        let (mut m, _, _) = at_deck();
        let start = m.chosen;
        m.mouse(true, start);
        assert!(m.dragging);
        m.mouse(true, start + 1);
        m.mouse(false, start + 2);
        assert!(!m.dragging);
        assert_eq!(
            (m.state, m.steps, m.chosen),
            (state::DECK_FORWARD, 2, start + 2)
        );
        let (mut m, _, _) = at_deck();
        let start = m.chosen;
        // The wheel: no button, nothing.
        m.mouse(false, start + 3);
        assert_eq!((m.state, m.chosen), (state::IDLE, start));
        m.mouse(true, start + 3);
        m.mouse(false, start + 3);
        assert_eq!((m.state, m.chosen), (state::IDLE, start + 3));
        // Five or more: one fast move; back four, a step each.
        let (mut m, _, _) = at_deck();
        let start = m.chosen;
        m.mouse(true, start);
        m.mouse(false, start + 6);
        assert_eq!(m.state, state::DECK_FAST_FORWARD);
        let (mut m, _, _) = at_deck();
        let start = m.chosen;
        m.mouse(true, start);
        m.mouse(false, start - 4);
        assert_eq!((m.state, m.steps), (state::DECK_BACK, 4));
        // With a pad the mouse's drags aren't followed.
        let (mut m, _, _) = at_deck();
        m.pad = true;
        m.mouse(true, 0);
        assert!(!m.dragging);
    }

    /// The pad's left stick: past the dead zone a card a step on the deck
    /// screen, and on the player's turn the hand card as the arrows choose
    /// it.
    #[test]
    fn the_stick_steps_through_the_cards() {
        assert_eq!(stick_axis(7849), 0.0);
        assert_eq!(stick_axis(-7849), 0.0);
        assert!((stick_axis(16384) - 0.5).abs() < 1e-3);
        assert!((stick_axis(-32768) + 1.0).abs() < 1e-3);
        let (mut m, mut tick, mut rng) = at_deck();
        let start = m.chosen;
        m.pad = true;
        m.stick = [0.4, 0.0];
        tick += 16;
        m.update(tick, &Half, &mut |n| rng.next(n));
        assert_eq!(
            (m.state, m.steps, m.chosen),
            (state::DECK_FORWARD, 1, start + 1)
        );
        // Without a pad the stick isn't read.
        let (mut m, mut tick, mut rng) = at_deck();
        m.stick = [0.4, 0.0];
        tick += 16;
        m.update(tick, &Half, &mut |n| rng.next(n));
        assert_eq!(m.state, state::IDLE);
        // Left at the first card: nothing.
        let (mut m, mut tick, mut rng) = at_deck();
        m.pad = true;
        m.chosen = 0;
        m.stick = [-0.9, 0.0];
        tick += 16;
        m.update(tick, &Half, &mut |n| rng.next(n));
        assert_eq!((m.state, m.chosen), (state::IDLE, 0));
    }

    /// On the player's turn the stick chooses the hand card as the arrows
    /// do (right: the next one).
    #[test]
    fn the_stick_chooses_hand_cards() {
        let (mut m, mut tick, mut rng) = at_deck();
        let mut all = Vec::new();
        m.key('F', &mut |n| rng.next(n));
        settle(&mut m, &mut tick, &mut rng, &mut all);
        assert_eq!(m.screen, Screen::Game);
        let before = m.hand_card;
        let n = m.game.as_ref().unwrap().player_hand.len();
        m.pad = true;
        m.stick = [0.0, 0.0];
        // Past the quarter-second guard.
        for _ in 0..40 {
            tick += 16;
            m.update(tick, &Half, &mut |n| rng.next(n));
        }
        assert_eq!(m.hand_card, before);
        m.stick = [0.6, 0.0];
        tick += 16;
        let fx = m.update(tick, &Half, &mut |n| rng.next(n));
        assert_eq!(m.hand_card, (before + 1) % n);
        assert!(fx.contains(&Effect::Sound("GAMECaravanSwitchCard")));
    }
}
