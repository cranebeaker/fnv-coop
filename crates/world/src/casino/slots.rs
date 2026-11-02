//! The slot machines (`SlotMachineMenu`, class 1080,
//! `menus\slot_machine_menu.xml`; `ShowSlotMachineMenuParams`), as
//! FalloutNV.exe 1.4.0.525 runs them (the Xbox 360 prototype's PDB naming
//! the parts). The 3D, the tiles and the player's things are the caller's:
//! [`SlotMachine`] says what to do as [`Effect`]s.
//!
//! - The strip (`SetupActiveReels` `007c16b0`): 28 places filled by
//!   rounds over the seven symbols, each with stops left putting the
//!   symbol (2k) and its blank (2k + 1); the stops must come to 14.
//! - A spin (`SpinReels` `007c56e0`): each reel takes `strip[rand(0, 2n −
//!   1)]` (n the symbols with stops), so only the first round counts and
//!   its last blank never comes up. All the chance is in the click; the
//!   animation shows what was decided.
//! - The result (`InterpretSpin` `007c5760`): three the same (a symbol) 6,
//!   two wilds (cherries, 12) 5, one 4, a wild with two blanks 2 (never:
//!   the wild cases come first), a pair 1, else 0; above 3 wins. Paid
//!   (`DoIdle` state 2): three of a kind × 10, 20, 30, 40, 60, 100 for A
//!   to F (bell, BAR, 7, lemon, grapes, orange) and × 10 for wilds, two
//!   wilds × 5, one × 2; anything else loses the bet.
//! - Luck (`ApplyLuck` `007c58f0`, read once at the start): with
//!   `rand(0, 100) + |Luck − 5| × 10 > 100`, below 5 wilds turn blank and
//!   three of a kind slips (the "unlucky" flag it sets is overwritten by
//!   its return of 0, so "You feel unlucky" never shows); above 5 a loss
//!   spins again, a pair close enough (squared distance in ids within
//!   Luck) becomes three, a wild's neighbours close enough match.
//! - The states (`DoIdle` `007c2c70`, seconds in a state kept only while
//!   the menu is on top): idle 6, spin 1 (the lever, the reels), results 2
//!   (one update), coins 3 (then the level check: a new level closes the
//!   menu), the payout card in 4 and out 5, closing 7.
//! - Clicks (`DoClick` `007c2460`, only when idle; with the payout card
//!   in, any click takes it out): Spin 3, Increase 4, Decrease 5, Payout
//!   List 6, Bet Max 7, Exit 8.

use super::{CasinoData, Dice};

/// The states (`SlotState`).
pub mod state {
    pub const INTRO: u8 = 0;
    pub const SPIN: u8 = 1;
    pub const RESULTS: u8 = 2;
    pub const WIN: u8 = 3;
    pub const PAYOUT_IN: u8 = 4;
    pub const PAYOUT_OUT: u8 = 5;
    pub const IDLE: u8 = 6;
    pub const CLOSING: u8 = 7;
}

/// The tiles by `id` (`TileIndex`).
pub mod tile {
    pub const CURRENT_BET: i32 = 0;
    pub const CHIP_COUNT: i32 = 1;
    pub const CASINO_INFO: i32 = 2;
    pub const SPIN: i32 = 3;
    pub const INCREASE_BET: i32 = 4;
    pub const DECREASE_BET: i32 = 5;
    pub const PAYOUT_LIST: i32 = 6;
    pub const BET_MAX: i32 = 7;
    pub const EXIT: i32 = 8;
    pub const STATUS: i32 = 9;
}

/// `ReelSymbols`: symbol k is 2k, its blank 2k + 1; the wild (cherries) 12.
pub const WILD: i32 = 12;
/// The strip's length.
pub const STRIP: usize = 28;
/// The texture for a blank face (`NV_SlotMachine-SymbolS.dds`), after the
/// casino's seven.
pub const BLANK_TEXTURE: usize = 7;
/// The blank face's own texture.
pub const BLANK_TEXTURE_PATH: &str =
    "textures\\terminals\\nv_slotmachine\\nv_slotmachine-symbols.dds";
/// The reels' models, left to right (`InitializeCasinoData` `007c1490`).
pub const REEL_MODELS: [&str; 3] = [
    "meshes\\terminals\\nv_slotmachine\\nv_slotmachine-reel01.nif",
    "meshes\\terminals\\nv_slotmachine\\nv_slotmachine-reel02.nif",
    "meshes\\terminals\\nv_slotmachine\\nv_slotmachine-reel03.nif",
];
/// "Slots Games Played", the misc statistic a spin counts (0x2A).
pub const GAMES_PLAYED_STAT: u8 = 0x2a;

/// The strip from the casino's stops (`SetupActiveReels`), and how many
/// symbols have stops; `None` when they don't fill it ("Casino Form is not
/// setup properly. # of stops for the slot reels must sum to 14.", and
/// the menu closes). Stops past 14 would write beyond the game's array;
/// here the strip just stops filling.
pub fn strip(stops: &[i32; 7]) -> Option<([i32; STRIP], usize)> {
    let mut out = [14; STRIP];
    let active = stops.iter().filter(|&&s| s > 0).count();
    let mut left = *stops;
    let mut at = 0;
    while at < STRIP {
        let before = at;
        for (k, n) in left.iter_mut().enumerate() {
            if *n > 0 && at + 1 < STRIP {
                out[at] = 2 * k as i32;
                out[at + 1] = 2 * k as i32 + 1;
                at += 2;
                *n -= 1;
            }
        }
        if at == before {
            return None;
        }
    }
    Some((out, active))
}

/// What three reels come to (`InterpretSpin`).
pub fn interpret(r: &[i32; 3]) -> i32 {
    let wilds = r.iter().filter(|&&x| x == WILD).count();
    let even = |x: i32| x % 2 == 0;
    if r[0] == r[1] && r[0] == r[2] && even(r[1]) {
        return 6;
    }
    if wilds == 2 {
        return 5;
    }
    if wilds == 1 {
        return 4;
    }
    if r.contains(&WILD) {
        return 2;
    }
    let pair = (r[1] == r[2] && even(r[1]))
        || (r[0] == r[2] && even(r[0]))
        || (r[1] == r[0] && even(r[0]));
    i32::from(pair)
}

/// The net chips for a result (`DoIdle` state 2): three of a kind by the
/// first reel's symbol, two wilds × 5, one × 2, else the bet lost.
pub fn payout(result: i32, reels: &[i32; 3], bet: i32) -> i32 {
    match result {
        6 => match reels[0] {
            0 | 12 => bet * 10,
            2 => bet * 20,
            4 => bet * 30,
            6 => bet * 40,
            8 => bet * 60,
            10 => bet * 100,
            _ => -bet,
        },
        5 => bet * 5,
        4 => bet * 2,
        _ => -bet,
    }
}

/// A bet up a step (`DoClick` 4): + 1 under 10, 5 under 40, 10 under 100,
/// 25 under 500, else 100.
pub fn step_up(bet: i32) -> i32 {
    bet + match bet {
        b if b < 10 => 1,
        b if b < 40 => 5,
        b if b < 100 => 10,
        b if b < 500 => 25,
        _ => 100,
    }
}

/// A bet down a step (`DoClick` 5): − 1 under 11; then down to the
/// step's multiple (a whole step when on one): 5 under 41, 10 under 101,
/// 25 under 501, else 100.
pub fn step_down(bet: i32) -> i32 {
    let step = match bet {
        b if b < 11 => return b - 1,
        b if b < 41 => 5,
        b if b < 101 => 10,
        b if b < 501 => 25,
        _ => 100,
    };
    if bet % step == 0 {
        bet - step
    } else {
        bet / step * step
    }
}

/// A texture slot as the swaps wrap it: past 6 less 7, below 0 plus 7.
fn wrap(s: i32) -> usize {
    (if s > 6 {
        s - 7
    } else if s < 0 {
        s + 7
    } else {
        s
    }) as usize
}

/// The window's six faces of a reel showing `r` (`SwapFrontfacingTextures`
/// `007c4d70`): offsets −2..3 from its payline face and the texture slot
/// each takes (7 the blank).
pub fn front_faces(r: i32) -> [(i32, usize); 6] {
    if r % 2 == 0 {
        let s = r / 2;
        [
            (-2, wrap(s - 1)),
            (-1, BLANK_TEXTURE),
            (0, wrap(s)),
            (1, BLANK_TEXTURE),
            (2, wrap(s + 1)),
            (3, BLANK_TEXTURE),
        ]
    } else {
        [
            (-2, BLANK_TEXTURE),
            (-1, wrap((r - 1) / 2)),
            (0, BLANK_TEXTURE),
            (1, wrap((r + 1) / 2)),
            (2, BLANK_TEXTURE),
            (3, wrap((r + 3) / 2)),
        ]
    }
}

/// The other eight faces (`SwapBackfacingTextures` `007c40b0`), offsets 4
/// to 11: they skip s + 2 and come back round to s − 1 (only seen while
/// spinning).
pub fn back_faces(r: i32) -> [(i32, usize); 8] {
    if r % 2 == 0 {
        let s = r / 2;
        [
            (4, wrap(s + 3)),
            (5, BLANK_TEXTURE),
            (6, wrap(s + 4)),
            (7, BLANK_TEXTURE),
            (8, wrap(s + 5)),
            (9, BLANK_TEXTURE),
            (10, wrap(s + 6)),
            (11, BLANK_TEXTURE),
        ]
    } else {
        [
            (4, BLANK_TEXTURE),
            (5, wrap((r + 3) / 2)),
            (6, BLANK_TEXTURE),
            (7, wrap((r + 4) / 2)),
            (8, BLANK_TEXTURE),
            (9, wrap((r + 5) / 2)),
            (10, BLANK_TEXTURE),
            (11, wrap((r + 6) / 2)),
        ]
    }
}

/// A reel face's shape: `Cylinder0{6 − reel}:{face}`, the face its payline
/// face (2 × reel) plus `offset`, round the reel's 14.
pub fn face_name(reel: usize, offset: i32) -> String {
    let face = (2 * reel as i32 + offset).rem_euclid(14);
    format!("Cylinder0{}:{face}", 6 - reel)
}

/// The models: the machine and the three reels (the scene node's children
/// 0 to 3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Model {
    Machine,
    Reel(usize),
}

/// The machine's sequences.
pub mod sequence {
    /// The lever (always running; updated in a spin).
    pub const FORWARD: &str = "Forward";
    /// The coins dropping (a win).
    pub const BACKWARD: &str = "Backward";
    pub const PAYOUT_IN: &str = "Payout_In";
    pub const PAYOUT_OUT: &str = "Payout_Out";
}

/// Sequence lengths, from the caller's models (`NiControllerSequence`'s
/// end, `00508100`).
pub trait Clips {
    fn end(&self, model: Model, sequence: &str) -> f32;
}

/// What the caller does for the menu.
#[derive(Debug, Clone, PartialEq)]
pub enum Effect {
    /// A sound by its editor ID.
    Sound(String),
    /// `ActivateSequence` (weight 1), `DeactivateSequence`, every one off.
    Activate {
        model: Model,
        sequence: &'static str,
    },
    Deactivate {
        model: Model,
        sequence: &'static str,
    },
    DeactivateAll(Model),
    /// A model updated at a time (its running sequences pose it; the pose
    /// stays).
    Update {
        model: Model,
        time: f32,
    },
    /// The reels' window faces ([`front_faces`]) and the rest
    /// ([`back_faces`]) retextured, each reel whose result changed from its
    /// last.
    SwapFront,
    SwapBack,
    /// Out of chips while idle: `sGamblingBrokeText` as a corner message
    /// (the surprised Vault Boy, `UIPopUpMessageGeneral`), and the menu
    /// closes.
    Broke,
    /// A spin counted in "Slots Games Played".
    GamePlayed,
    /// The player's line for the casino changed (the winnings).
    Data(CasinoData),
    /// The closing state: settle the chips (`casino::settle` with
    /// [`SlotMachine::chips`] and [`SlotMachine::new_level`]) and close.
    Close,
}

/// The status line (tile 9).
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    /// A round's result ([`super::result_line`]).
    Result { won: bool, amount: i32, lucky: bool },
    /// `sSlotPressAnyButtonText` with the payout card in.
    PressAnyButton,
}

/// The slot machine's menu.
#[derive(Debug, Clone)]
pub struct SlotMachine {
    pub max_winnings: i32,
    pub min_bet: i32,
    pub max_bet: i32,
    pub state: u8,
    /// Seconds in the state (`ftotalStateSecs`), the tick it was last
    /// updated (`uiPrevIdleTick`) and the state then.
    pub elapsed: f32,
    last_tick: Option<u32>,
    last_state: u8,
    /// Whether the menu is on top (a message over it stops its clock).
    pub on_top: bool,
    pub luck: i32,
    pub bet: i32,
    /// The menu's chips (settled on closing).
    pub chips: i32,
    pub strip: [i32; STRIP],
    pub active: usize,
    pub results: [i32; 3],
    pub starts: [i32; 3],
    pub result: i32,
    pub lucky: bool,
    /// The next increase sound, 1 to 3.
    pub next_sound: i32,
    /// A round reached a new level: the comps quest starts on closing.
    pub new_level: bool,
    pub data: CasinoData,
    /// The lever's sound played: its texture swap is due.
    swap_due: bool,
    pub status: Option<Status>,
    pub status_shown: bool,
}

impl SlotMachine {
    /// `Create` (`007c0a40`) after its checks, with `InitializeCasinoData`:
    /// the bet at the least, Luck read once (clamped 0..10, whole), the
    /// strip; `None` when the stops don't make one. Starts with
    /// `GAMESlotsActivate` and the first swaps (A, B, C on the paylines),
    /// then idle.
    #[allow(clippy::too_many_arguments)]
    pub fn open(
        stops: &[i32; 7],
        max_winnings: i32,
        min_bet: i32,
        max_bet: i32,
        chips: i32,
        luck: f32,
        data: CasinoData,
    ) -> Option<(SlotMachine, Vec<Effect>)> {
        let (strip, active) = strip(stops)?;
        let m = SlotMachine {
            max_winnings,
            min_bet,
            max_bet,
            state: state::INTRO,
            elapsed: 0.0,
            last_tick: None,
            last_state: 0,
            on_top: true,
            luck: luck.clamp(0.0, 10.0) as i32,
            bet: min_bet,
            chips,
            strip,
            active,
            results: [0, 2, 4],
            starts: [14; 3],
            result: 0,
            lucky: false,
            next_sound: 1,
            new_level: false,
            data,
            swap_due: false,
            status: None,
            status_shown: false,
        };
        let fx = vec![
            Effect::Sound("GAMESlotsActivate".into()),
            Effect::DeactivateAll(Model::Machine),
            Effect::Activate {
                model: Model::Machine,
                sequence: sequence::FORWARD,
            },
            Effect::Update {
                model: Model::Machine,
                time: 0.0,
            },
            Effect::SwapFront,
            Effect::SwapBack,
        ];
        Some((m, fx))
    }

    /// `SpinReels`: each reel's last result kept, a new one drawn.
    fn spin_reels(&mut self, dice: Dice) -> i32 {
        for i in 0..3 {
            self.starts[i] = self.results[i];
            let n = (2 * self.active).saturating_sub(1).max(1);
            self.results[i] = self.strip[dice(n)];
        }
        interpret(&self.results)
    }

    /// `ApplyLuck` for a result; true when luck helped.
    fn apply_luck(&mut self, result: i32, dice: Dice) -> bool {
        let a = ((self.luck - 5) * 10).abs();
        if dice(100) as i32 + a <= 100 {
            return false;
        }
        let r = &mut self.results;
        if self.luck - 5 < 0 {
            if result == 4 || result == 5 {
                for x in r.iter_mut() {
                    if *x == WILD {
                        *x = WILD + 1;
                    }
                }
            } else if result == 6 {
                let v = if r[0] + 1 < 7 { r[0] + 1 } else { 0 };
                *r = [v; 3];
            }
            // The flag set here is lost to the return.
            return false;
        }
        let luck = self.luck;
        match result {
            0 => {
                let again = self.spin_reels(dice);
                self.apply_luck(again, dice)
            }
            1 | 5 => {
                let close = |a: i32, b: i32| (a - b).abs() * (a - b).abs() <= luck;
                if r[0] == r[1] {
                    if close(r[0], r[2]) {
                        r[2] = r[0];
                        return true;
                    }
                } else if r[0] == r[2] {
                    if close(r[0], r[1]) {
                        r[1] = r[0];
                        return true;
                    }
                } else if close(r[1], r[0]) {
                    r[0] = r[1];
                    return true;
                }
                false
            }
            2 => {
                // Never reached (`interpret` gives 2 for nothing).
                let n = self.active;
                let others: [usize; 2] = if r[0] == WILD {
                    [1, 2]
                } else if r[1] == WILD {
                    [0, 2]
                } else {
                    [0, 1]
                };
                for i in others {
                    let k = dice(n.max(1));
                    self.results[i] = self.strip[2 * k];
                }
                interpret(&self.results) > result
            }
            4 => {
                let mut wild = 0usize;
                let mut blank: i32 = -1;
                for (i, &x) in r.iter().enumerate() {
                    if x == WILD {
                        wild = i;
                    }
                    if x % 2 != 0 {
                        blank = i as i32;
                    }
                }
                let left = if wild == 0 { 2 } else { wild - 1 };
                let right = if wild + 1 < 2 { wild + 1 } else { 0 };
                let d = (r[left] - r[right]).abs();
                if d <= luck && blank < 0 {
                    r[left] = r[right];
                    return true;
                }
                if d <= luck {
                    if left as i32 == blank {
                        r[left] = r[right];
                    } else {
                        r[right] = r[left];
                    }
                    return true;
                }
                false
            }
            _ => false,
        }
    }

    fn close(&mut self) {
        self.state = state::CLOSING;
    }

    /// A tile clicked (`DoClick`).
    pub fn click(&mut self, id: i32, dice: Dice) -> Vec<Effect> {
        let mut fx = Vec::new();
        if self.state == state::PAYOUT_IN {
            self.state = state::PAYOUT_OUT;
            return fx;
        }
        if self.state != state::IDLE {
            return fx;
        }
        match id {
            tile::SPIN if self.bet <= self.chips => {
                self.next_sound = 1;
                let first = self.spin_reels(dice);
                self.lucky = self.apply_luck(first, dice);
                self.result = interpret(&self.results);
                self.state = state::SPIN;
                fx.push(Effect::GamePlayed);
            }
            tile::INCREASE_BET => {
                let before = self.bet;
                self.bet = step_up(self.bet);
                if self.bet > self.chips {
                    self.bet = self.chips;
                }
                if self.bet > self.max_bet {
                    self.bet = self.max_bet;
                }
                if self.bet != before {
                    self.increase_sound(&mut fx);
                }
            }
            tile::DECREASE_BET => {
                let before = self.bet;
                self.bet = step_down(self.bet).max(self.min_bet);
                if self.bet != before {
                    fx.push(Effect::Sound("GAMESlotsDecreaseBet".into()));
                    self.next_sound = 1;
                }
            }
            tile::PAYOUT_LIST => self.state = state::PAYOUT_IN,
            tile::BET_MAX if self.bet != self.max_bet => {
                self.bet = self.max_bet.min(self.chips);
                self.increase_sound(&mut fx);
            }
            tile::EXIT => self.close(),
            _ => {}
        }
        fx
    }

    fn increase_sound(&mut self, fx: &mut Vec<Effect>) {
        fx.push(Effect::Sound(format!(
            "GAMESlotsIncreaseBet0{}",
            self.next_sound
        )));
        self.next_sound = if self.next_sound + 1 < 4 {
            self.next_sound + 1
        } else {
            1
        };
    }

    /// One update (`DoIdle`): `tick` the time in milliseconds.
    pub fn update(&mut self, tick: u32, clips: &dyn Clips) -> Vec<Effect> {
        let mut fx = Vec::new();
        match self.last_tick {
            None => {
                self.elapsed = 0.0;
                self.last_state = 0;
            }
            Some(last) if self.on_top => {
                let dt = tick.wrapping_sub(last) as f32 / 1000.0;
                self.elapsed = if self.last_state == self.state {
                    self.elapsed + dt
                } else {
                    0.0
                };
            }
            _ => {}
        }
        self.last_tick = Some(tick);
        self.last_state = self.state;
        let t = self.elapsed;
        let machine = Model::Machine;
        let end = |seq: &str| clips.end(machine, seq);
        match self.state {
            state::INTRO => self.state = state::IDLE,
            state::SPIN => {
                fx.push(Effect::Deactivate {
                    model: machine,
                    sequence: sequence::BACKWARD,
                });
                if t > 1.0 {
                    self.status_shown = false;
                }
                fx.push(Effect::Update {
                    model: machine,
                    time: t,
                });
                if t == 0.0 && self.chips > 0 {
                    fx.push(Effect::Sound("GameSlotsPullLever".into()));
                    self.swap_due = true;
                }
                if t == 0.0 || self.chips > 0 {
                    for i in 0..3 {
                        fx.push(Effect::Update {
                            model: Model::Reel(i),
                            time: t,
                        });
                    }
                }
                let lever = end(sequence::FORWARD);
                if lever <= t && self.swap_due {
                    self.swap_due = false;
                    fx.push(Effect::SwapFront);
                }
                if lever <= t && clips.end(Model::Reel(0), sequence::FORWARD) <= t {
                    self.state = state::RESULTS;
                    fx.push(Effect::SwapBack);
                }
            }
            state::RESULTS => {
                self.result = interpret(&self.results);
                let delta = payout(self.result, &self.results, self.bet);
                self.chips += delta;
                self.data.winnings += delta;
                fx.push(Effect::Data(self.data));
                let won = self.result >= 4;
                if !won && self.lucky {
                    fx.push(Effect::Sound("GAMESlotsLose".into()));
                }
                self.status = Some(Status::Result {
                    won,
                    amount: if won { delta } else { -delta },
                    lucky: self.lucky,
                });
                self.status_shown = true;
                self.state = if won { state::WIN } else { state::IDLE };
            }
            state::WIN => {
                fx.push(Effect::Activate {
                    model: machine,
                    sequence: sequence::BACKWARD,
                });
                fx.push(Effect::Deactivate {
                    model: machine,
                    sequence: sequence::FORWARD,
                });
                fx.push(Effect::Update {
                    model: machine,
                    time: t,
                });
                if t == 0.0 {
                    let sound = match self.result {
                        4 => Some("GAMESlotsWinSmall"),
                        5 => Some("GAMESlotsWinMed"),
                        6 => Some("GAMESlotsWinJackpot"),
                        _ => None,
                    };
                    fx.extend(sound.map(|s| Effect::Sound(s.into())));
                }
                if t > 1.0 {
                    self.status_shown = false;
                }
                if end(sequence::BACKWARD) <= t {
                    fx.push(Effect::Deactivate {
                        model: machine,
                        sequence: sequence::BACKWARD,
                    });
                    fx.push(Effect::Activate {
                        model: machine,
                        sequence: sequence::FORWARD,
                    });
                    self.new_level = super::new_level(&mut self.data, self.max_winnings);
                    fx.push(Effect::Data(self.data));
                    if self.new_level {
                        self.close();
                    } else {
                        self.state = state::IDLE;
                    }
                }
            }
            state::PAYOUT_IN => {
                if t == 0.0 {
                    self.status = Some(Status::PressAnyButton);
                    self.status_shown = true;
                    fx.push(Effect::DeactivateAll(machine));
                    fx.push(Effect::Activate {
                        model: machine,
                        sequence: sequence::PAYOUT_IN,
                    });
                }
                if t > end(sequence::PAYOUT_IN) {
                    fx.push(Effect::Deactivate {
                        model: machine,
                        sequence: sequence::PAYOUT_IN,
                    });
                } else {
                    fx.push(Effect::Update {
                        model: machine,
                        time: t,
                    });
                }
            }
            state::PAYOUT_OUT => {
                if t == 0.0 {
                    self.status_shown = false;
                    fx.push(Effect::DeactivateAll(machine));
                    fx.push(Effect::Activate {
                        model: machine,
                        sequence: sequence::PAYOUT_OUT,
                    });
                }
                if t > end(sequence::PAYOUT_OUT) {
                    fx.push(Effect::Deactivate {
                        model: machine,
                        sequence: sequence::PAYOUT_OUT,
                    });
                    fx.push(Effect::Activate {
                        model: machine,
                        sequence: sequence::FORWARD,
                    });
                    self.state = state::IDLE;
                } else {
                    fx.push(Effect::Update {
                        model: machine,
                        time: t,
                    });
                }
            }
            state::IDLE => {
                if t > 1.0 {
                    self.status_shown = false;
                }
                if self.chips < self.bet {
                    self.bet = 1;
                }
                if self.chips == 0 {
                    fx.push(Effect::Broke);
                    self.close();
                }
            }
            state::CLOSING => {
                fx.push(Effect::Close);
                // Once: the menu is gone after this.
                self.state = u8::MAX;
            }
            _ => {}
        }
        fx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Ends;
    impl Clips for Ends {
        fn end(&self, model: Model, sequence: &str) -> f32 {
            match (model, sequence) {
                (Model::Machine, sequence::FORWARD) => 0.833,
                (Model::Machine, sequence::BACKWARD) => 3.167,
                (Model::Machine, _) => 0.667,
                (Model::Reel(_), _) => 4.367,
            }
        }
    }

    #[test]
    fn the_vanilla_strip_and_its_thirteen_stops() {
        let (s, n) = strip(&[2; 7]).unwrap();
        assert_eq!(n, 7);
        assert_eq!(&s[..14], &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
        assert_eq!(&s[14..], &s[..14]);
        // Too few stops.
        assert!(strip(&[1, 1, 1, 1, 1, 1, 0]).is_none());
        // One symbol alone fills it.
        let (s, n) = strip(&[14, 0, 0, 0, 0, 0, 0]).unwrap();
        assert_eq!((n, s[0], s[1], s[27]), (1, 0, 1, 1));
    }

    #[test]
    fn results_and_payouts() {
        assert_eq!(interpret(&[4, 4, 4]), 6);
        assert_eq!(interpret(&[12, 12, 12]), 6);
        assert_eq!(interpret(&[12, 3, 12]), 5);
        assert_eq!(interpret(&[12, 3, 5]), 4);
        assert_eq!(interpret(&[2, 2, 9]), 1);
        assert_eq!(interpret(&[3, 3, 3]), 0);
        assert_eq!(payout(6, &[10, 10, 10], 3), 300);
        assert_eq!(payout(6, &[12, 12, 12], 3), 30);
        assert_eq!(payout(5, &[12, 12, 0], 3), 15);
        assert_eq!(payout(1, &[2, 2, 0], 3), -3);
    }

    #[test]
    fn bet_steps() {
        assert_eq!(
            (step_up(9), step_up(10), step_up(95), step_up(475)),
            (10, 15, 105, 500)
        );
        assert_eq!(
            (step_down(10), step_down(11), step_down(15), step_down(17)),
            (9, 10, 10, 15)
        );
        assert_eq!(
            (step_down(101), step_down(130), step_down(600)),
            (100, 125, 500)
        );
    }

    #[test]
    fn faces_wrap_round_the_seven() {
        // The wild's blank (13) shows the wild below and A above.
        let f = front_faces(13);
        assert_eq!((f[1], f[3]), ((-1, 6), (1, 0)));
        assert_eq!(front_faces(0)[0], (-2, 6));
        assert_eq!(face_name(0, -2), "Cylinder06:12");
        assert_eq!(face_name(2, 3), "Cylinder04:7");
    }

    /// A spin through to the result with Luck 5 (luck never acts): three
    /// sevens pay 30 × the bet, the coins drop, then idle.
    #[test]
    fn a_winning_spin() {
        let (mut m, _) =
            SlotMachine::open(&[2; 7], 9000, 1, 60, 100, 5.0, CasinoData::default()).unwrap();
        let mut tick = 1000;
        m.update(tick, &Ends);
        assert_eq!(m.state, state::IDLE);
        m.click(tile::INCREASE_BET, &mut |_| 0);
        assert_eq!(m.bet, 2);
        // Every draw the third place on the strip: C (4); the luck roll 0.
        let mut dice = |n: usize| if n == 100 { 0 } else { 4 };
        let fx = m.click(tile::SPIN, &mut dice);
        assert!(fx.contains(&Effect::GamePlayed));
        assert_eq!((m.results, m.result, m.state), ([4, 4, 4], 6, state::SPIN));
        let mut seen = Vec::new();
        while m.state != state::IDLE {
            tick += 16;
            seen.extend(m.update(tick, &Ends));
            assert!(tick < 20_000);
        }
        assert_eq!(m.chips, 160);
        assert_eq!(m.data.winnings, 60);
        assert!(seen.contains(&Effect::Sound("GameSlotsPullLever".into())));
        assert!(seen.contains(&Effect::Sound("GAMESlotsWinJackpot".into())));
        assert!(seen.contains(&Effect::SwapFront) && seen.contains(&Effect::SwapBack));
        assert_eq!(
            m.status,
            Some(Status::Result {
                won: true,
                amount: 60,
                lucky: false
            })
        );
    }

    /// Reaching a quarter of the limit closes the menu after the coins.
    #[test]
    fn a_new_level_closes() {
        let data = CasinoData {
            winnings: 2200,
            ..CasinoData::default()
        };
        let (mut m, _) = SlotMachine::open(&[2; 7], 9000, 1, 60, 100, 5.0, data).unwrap();
        m.update(0, &Ends);
        m.bet = 10;
        // Every draw 0: three bells, and the luck roll 0.
        let mut dice = |_: usize| 0;
        m.click(tile::SPIN, &mut dice);
        let mut tick = 0;
        let mut closed = false;
        for _ in 0..1000 {
            tick += 16;
            if m.update(tick, &Ends).contains(&Effect::Close) {
                closed = true;
                break;
            }
        }
        assert!(closed && m.new_level);
        assert_eq!(m.data.level, 1);
    }

    /// High luck: a pair whose third reel is close enough becomes three.
    #[test]
    fn luck_mends_a_close_pair() {
        let (mut m, _) =
            SlotMachine::open(&[2; 7], 9000, 1, 60, 100, 10.0, CasinoData::default()).unwrap();
        m.results = [2, 2, 4];
        // rand(0, 100) = 99: 99 + 50 > 100.
        assert!(m.apply_luck(1, &mut |_| 99));
        assert_eq!(m.results, [2, 2, 2]);
        // Too far: (2 − 8)² = 36 > 10.
        m.results = [2, 2, 8];
        assert!(!m.apply_luck(1, &mut |_| 99));
    }
}
