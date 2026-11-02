//! Roulette (`RouletteMenu`, class 1082, `menus\roulette_menu.xml`;
//! `ShowRouletteMenuParams`), as FalloutNV.exe 1.4.0.525 runs it (the Xbox
//! 360 prototype's PDB naming the parts). The 3D, the tiles and the
//! player's things are the caller's: [`Roulette`] says what to do as
//! [`Effect`]s and keeps what the tiles show.
//!
//! - The wheel: American, 38 pockets (37 is 00), in the order of
//!   [`STRIP`] (`PrepareBetTiles` `007b9c40`). A spin (`SpinWheel`
//!   `007bf670`) draws the pocket `rand(0, 38)` and one of the table's three
//!   `Spin_n` sequences on the click; Luck changes only the winning words.
//! - The table (`PrepareBetTiles`): 159 spots, a 6 × 24 grid `A01:0` to
//!   `F24:0` (straights, splits, streets, corners, six-lines, the
//!   five-number bet, the 0 and 00 trios) and `S01:0` to `S15:0` (00, 0/00,
//!   0, the dozens, the halves, even, red, black, odd, the columns), each at
//!   its mesh's centre in `NV_Roulette-Points.NIF`. Even and red pay on 0,
//!   odd and black on 00, the first column on 00 and the third on 0 (the
//!   game's tests: `r % 2`, the pocket's place on the wheel, `(r − n) % 3`).
//! - Bets (`SetBet` `007bed70`, `RemoveBet` `007bf150`): up to ten, one a
//!   spot, at the current bet; the total no more than the most bet and the
//!   chips. A win pays `(payout + 1) × bet`; the net of the round against
//!   the total moves the chips (nothing is taken as bets are placed).
//! - The cursor (`DoIdle` state 4): the mouse (or the left stick) moves a
//!   chip over the felt, 0.35 units a frame per unit of movement; the spot
//!   under it is the first within its radius (`GetBetIndex` `007bf390`).
//! - States (`DoIdle` `007bd2a0`): spinning 1, the results 3, idle 4,
//!   closing 5.

use super::{CasinoData, Dice};

/// The states (`RouletteState`; 0 and 2 never set).
pub mod state {
    pub const SPIN: u8 = 1;
    pub const RESULTS: u8 = 3;
    pub const IDLE: u8 = 4;
    pub const CLOSING: u8 = 5;
}

/// The tiles by `id` (`TileIndex`).
pub mod tile {
    pub const CURRENT_BET: i32 = 0;
    pub const CHIP_COUNT: i32 = 1;
    pub const CASINO_INFO: i32 = 2;
    pub const PLACE_BET: i32 = 3;
    pub const REMOVE_BET: i32 = 4;
    pub const FINISH_BET: i32 = 5;
    pub const INCREASE_BET: i32 = 6;
    pub const DECREASE_BET: i32 = 7;
    pub const EXIT: i32 = 8;
    pub const STATUS: i32 = 9;
    pub const TOTAL_BET: i32 = 10;
}

/// The wheel's pockets in order (`iNumberStrip`, 37 the 00).
pub const STRIP: [u8; 38] = [
    14, 2, 0, 28, 9, 26, 30, 11, 7, 20, 32, 17, 5, 22, 34, 15, 3, 24, 36, 13, 1, 37, 27, 10, 25,
    29, 12, 8, 19, 31, 18, 6, 21, 33, 16, 4, 23, 35,
];
/// The 00 pocket.
pub const DOUBLE_ZERO: u8 = 37;
/// How many spots, how many bets at once.
pub const SPOTS: usize = 159;
pub const MAX_BETS: usize = 10;
/// The chips' models: the scene's children 3 to 12, their shadows 13 to 22.
pub const FIRST_CHIP: usize = 3;
pub const SHADOW_OFFSET: usize = 10;
/// "Roulette Games Played", the misc statistic a spin counts (0x28).
pub const GAMES_PLAYED_STAT: u8 = 0x28;
/// The table's spins.
pub const SPINS: [&str; 3] = ["Spin_1", "Spin_2", "Spin_3"];
/// A hidden chip's height (`HideBetChip` `007bf480`) and the test for one
/// (`007bf530`: z < −3).
pub const HIDDEN_Z: f32 = -5.0;
/// The cursor's range (`DoIdle` state 4).
pub const CURSOR_X: (f32, f32) = (-27.5, 28.0);
pub const CURSOR_Y: (f32, f32) = (-13.3, 13.1);
/// A tile's text alpha, full and dimmed (`SetTileTextAlpha` `007bf710`).
pub const ALPHA_FULL: f32 = 255.0;
pub const ALPHA_DIM: f32 = 128.0;

/// One spot on the felt (`BetTileInfo`): the pockets it bets on (the
/// grid's and the first three specials'; the other specials test by rule),
/// what it pays, its mesh and the words over the table when the cursor is
/// on it.
#[derive(Debug, Clone, PartialEq)]
pub struct Spot {
    pub numbers: Vec<u8>,
    pub payout: i32,
    pub face: String,
    pub text: String,
}

/// The settings the spots' words take (`sBetText`, `sPayoutText`, the
/// specials' names), from the caller.
pub type Text<'a> = &'a dyn Fn(&str, &str) -> String;

/// The 159 spots (`PrepareBetTiles` `007b9c40`), their words made with
/// `text` (a setting and its exe default).
pub fn spots(text: Text) -> Vec<Spot> {
    let bet = text("sBetText", "Bet");
    let pays = text("sPayoutText", "Payout");
    let spot = |face: String, numbers: Vec<u8>, payout: i32, shown: &str| Spot {
        text: format!("{bet}: {shown} \n{pays}: {payout}:1"),
        numbers,
        payout,
        face,
    };
    let mut out = Vec::with_capacity(SPOTS);
    for idx in 0..144usize {
        let row = idx / 24;
        let c = idx % 24;
        let face = format!("{}{:02}:0", (b'A' + row as u8) as char, c + 1);
        // The face's last digit's parity: even is a number column.
        let even_digit = (c + 1) % 2 == 0;
        let h = (row as i32 + 1) / 2;
        let list = |v: &[i32]| v.iter().map(|&n| n as u8).collect::<Vec<u8>>();
        let joined = |v: &[i32]| {
            v.iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(",")
        };
        let s = match row {
            // B, D, F: straights, the 0 and 00 splits and trio, splits
            // along the row.
            1 | 3 | 5 => {
                if even_digit {
                    let n = h + (c as i32 / 2) * 3;
                    spot(face, list(&[n]), 35, &n.to_string())
                } else if c == 0 {
                    match row {
                        3 => spot(face, vec![DOUBLE_ZERO, 0, 2], 11, "0,00,2"),
                        5 => spot(face, vec![DOUBLE_ZERO, 3], 17, "00,3"),
                        _ => spot(face, vec![0, 1], 17, "0,1"),
                    }
                } else {
                    let n = h + ((c as i32 - 1) / 2) * 3;
                    spot(face, list(&[n, n + 3]), 17, &joined(&[n, n + 3]))
                }
            }
            // A: streets, the five-number bet, six-lines.
            0 => {
                if c % 2 == 1 {
                    let n = (c as i32 / 2) * 3 + 1;
                    let v = [n, n + 1, n + 2];
                    spot(face, list(&v), 11, &joined(&v))
                } else if c == 0 {
                    spot(face, vec![0, DOUBLE_ZERO, 1, 2, 3], 6, "0,00,1,2,3")
                } else {
                    let n = ((c as i32 - 1) / 2) * 3 + 1;
                    let v = [n, n + 1, n + 2, n + 3, n + 4, n + 5];
                    spot(face, list(&v), 5, &joined(&v))
                }
            }
            // C, E: splits across, the trios, corners.
            _ => {
                if even_digit {
                    let n = h + (c as i32 / 2) * 3;
                    spot(face, list(&[n, n + 1]), 17, &joined(&[n, n + 1]))
                } else if c == 0 {
                    if row == 2 {
                        spot(face, vec![1, 2, 0], 11, "0,1,2")
                    } else {
                        spot(face, vec![2, 3, DOUBLE_ZERO], 11, "00,2,3")
                    }
                } else {
                    let n = h + ((c as i32 - 1) / 2) * 3;
                    let v = [n, n + 1, n + 3, n + 4];
                    spot(face, list(&v), 8, &joined(&v))
                }
            }
        };
        out.push(s);
    }
    let special = |i: usize| format!("S{:02}:0", i);
    out.push(spot(special(1), vec![DOUBLE_ZERO], 35, "00"));
    out.push(spot(special(2), vec![DOUBLE_ZERO, 0], 17, "0,00"));
    out.push(spot(special(3), vec![0], 35, "0"));
    let named = |name: &str, default: &str| text(name, default);
    let rules: [(usize, String, i32); 12] = [
        (4, named("sFirstDozenText", "First Dozen"), 2),
        (5, "1-18".into(), 1),
        (6, named("sEvenText", "Even"), 1),
        (7, named("sSecondDozenText", "2nd Dozen"), 2),
        (8, named("sRedText", "Red"), 1),
        (9, named("sBlacktext", "Black"), 1),
        (10, named("sThirdDozenText", "3rd Dozen"), 2),
        (11, named("sOddtext", "Odd"), 1),
        (12, "19-36".into(), 1),
        (13, named("sTwoToOneText", "2 to 1"), 2),
        (14, named("sTwoToOneText", "2 to 1"), 2),
        (15, named("sTwoToOneText", "2 to 1"), 2),
    ];
    for (i, shown, payout) in rules {
        out.push(spot(special(i), Vec::new(), payout, &shown));
    }
    out
}

/// Whether a spot's bet wins on pocket `r` (`DoIdle` state 3).
pub fn wins(spots: &[Spot], index: usize, r: u8) -> bool {
    let r = i32::from(r);
    let place = STRIP.iter().position(|&p| i32::from(p) == r).unwrap_or(38) as i32;
    match index {
        i if i < 147 => spots.get(i).is_some_and(|s| s.numbers.contains(&(r as u8))),
        147 => r > 0 && r < 13,
        148 => r > 0 && r < 19,
        149 => r % 2 == 0,
        150 => r > 12 && r < 25,
        151 => place % 2 == 0,
        152 => place % 2 == 1,
        153 => r > 24 && r < 37,
        154 => r % 2 == 1,
        155 => r > 18 && r < 37,
        // C's `%` keeps the sign: (0 − 1) % 3 is −1.
        156 => (r - 1) % 3 == 0,
        157 => (r - 2) % 3 == 0,
        158 => r % 3 == 0,
        _ => false,
    }
}

/// The hit radii (`PrepareBetTiles`' end) from the spots' places: the grid's
/// 0.49 × the first two spots' distance; the specials' 0.49 × the smaller
/// of two distances along x and y (as the game reads them: the dozens' and
/// the halves' first against the next special's y).
pub fn radii(at: &[[f32; 3]]) -> (f32, [f32; 15]) {
    let p = |i: usize| at.get(i).copied().unwrap_or([0.0; 3]);
    let d = |a: [f32; 3], b: [f32; 3]| {
        ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
    };
    let valid = d(p(0), p(1)) * 0.49;
    let min = |dx: f32, dy: f32| (dx.abs() * 0.49).min(dy.abs() * 0.49);
    let zeros = min(p(144)[0] - p(0)[0], p(144)[1] - p(145)[1]);
    let dozens = min(p(147)[0] - p(150)[1], p(147)[1] - p(0)[1]);
    let evens = min(p(148)[0] - p(149)[1], p(148)[1] - p(3)[1]);
    let columns = min(p(156)[0] - p(23)[0], p(156)[1] - p(157)[1]);
    let mut r = [0.0; 15];
    for (i, v) in r.iter_mut().enumerate() {
        *v = match i + 144 {
            144..=146 => zeros,
            147 | 150 | 153 => dozens,
            148 | 149 | 151 | 152 | 154 | 155 => evens,
            _ => columns,
        };
    }
    (valid, r)
}

/// A bet's chip stack (`SetBetChip` `007bf840`): how many chips show for
/// a bet (each 0.18 above the felt).
pub fn chips_shown(bet: i32) -> i32 {
    match bet {
        b if b < 5 => 1,
        b if b < 10 => 2,
        b if b < 25 => 3,
        b if b < 40 => 4,
        b if b < 70 => 5,
        b if b < 100 => 6,
        b if b < 250 => 7,
        b if b < 500 => 8,
        b if b < 1000 => 9,
        _ => 10,
    }
}

/// The ring's turn for a result (`ShiftWheelTexture` `007bf570`): `k ×
/// 360° / 38` about z, k the spin's base (43, 41, 48) less the pocket's
/// place on the wheel; radians.
pub fn wheel_angle(spin: usize, result: u8) -> f32 {
    let i = STRIP.iter().position(|&p| p == result).unwrap_or(0) as i32;
    let base = match spin {
        0 => 43,
        1 => 41,
        2 => 48,
        _ => 38,
    };
    (base - i) as f32 * 9.473684 * 0.017453292
}

/// A bet on the table (`RouletteBet`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bet {
    pub value: i32,
    pub index: usize,
    /// Its chip (the scene's child).
    pub model: usize,
}

/// Sequence lengths from the caller's table (`00508100`); `None` for a
/// sequence the table doesn't have.
pub trait Clips {
    fn end(&self, sequence: &str) -> Option<f32>;
}

/// What the caller does for the menu.
#[derive(Debug, Clone, PartialEq)]
pub enum Effect {
    Sound(&'static str),
    /// The table's sequence on or off, the table updated at a time.
    Activate(&'static str),
    Deactivate(&'static str),
    UpdateTable(f32),
    /// `Roulette_Wheel:2`'s own turn about z ([`wheel_angle`]).
    WheelTurn(f32),
    /// A model's own translation (the cursor 2, the chips, the shadows).
    Place {
        model: usize,
        at: [f32; 3],
    },
    /// The spot marker shown (`Some`: its face) or none; the valid or the
    /// invalid ring under the cursor.
    Face(Option<String>),
    Marker {
        valid: bool,
    },
    GamePlayed,
    Data(CasinoData),
    /// Out of chips while idle (the surprised Vault Boy,
    /// `UIPopUpMessageGeneral`); the menu closes.
    Broke,
    /// The closing state: settle the chips and close.
    Close,
}

/// The status line (tile 9).
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    /// The spot under the cursor's words, or "Bet: \nPayout: " off the
    /// spots.
    Spot(usize),
    NoSpot,
    Won {
        amount: i32,
        lucky: bool,
    },
    Lost {
        amount: i32,
    },
    Even,
}

/// The roulette menu.
#[derive(Debug, Clone)]
pub struct Roulette {
    pub max_winnings: i32,
    pub min_bet: i32,
    pub max_bet: i32,
    pub state: u8,
    pub elapsed: f32,
    last_tick: Option<u32>,
    last_state: u8,
    pub on_top: bool,
    pub luck: i32,
    pub current_bet: i32,
    pub total_bet: i32,
    /// The menu's chips (settled on closing).
    pub chips: i32,
    pub bets: Vec<Bet>,
    /// The chip under the cursor (`iNextModelIndex`, 3 to 12).
    pub next_model: usize,
    pub spots: Vec<Spot>,
    /// Each spot's place (the Points meshes' centres), the grid's radius
    /// and the specials'.
    pub places: Vec<[f32; 3]>,
    pub valid_radius: f32,
    pub special_radius: [f32; 15],
    /// The cursor (`fxTranslate`, `fyTranslate`, `fzTranslate`) and each
    /// chip's own translation.
    pub cursor: [f32; 3],
    pub models: [[f32; 3]; 23],
    pub result: u8,
    pub spin: usize,
    clean_table: bool,
    pub lucky: bool,
    pub unlucky: bool,
    pub new_level: bool,
    /// The spot marker shown, the invalid ring shown (`bInvalidBet`).
    face: Option<usize>,
    invalid: bool,
    pub data: CasinoData,
    /// What the tiles show: each tile's alpha, the status line and
    /// whether it shows, and the idle clamp's write of the bet into tile
    /// 0's `_Value` (the label's place).
    pub alpha: [f32; 11],
    pub status: Option<Status>,
    pub status_shown: bool,
    pub bet_in_label: Option<i32>,
}

impl Roulette {
    /// `Create` (`007bbe20`) after its checks, with `Prepare3DElements`:
    /// the bet at the least, Luck read once, the chips hidden but the one
    /// under the cursor; `places` the spots' centres from the table's
    /// Points model.
    #[allow(clippy::too_many_arguments)]
    pub fn open(
        max_winnings: i32,
        min_bet: i32,
        max_bet: i32,
        chips: i32,
        luck: f32,
        data: CasinoData,
        spots: Vec<Spot>,
        places: Vec<[f32; 3]>,
    ) -> (Roulette, Vec<Effect>) {
        let (valid_radius, special_radius) = radii(&places);
        let mut models = [[0.0f32; 3]; 23];
        // The grid and the cursor at 0.1 above the table.
        models[1] = [0.0, 0.0, 0.1];
        models[2] = [0.0, 0.0, 0.1];
        let mut m = Roulette {
            max_winnings,
            min_bet,
            max_bet,
            state: state::IDLE,
            elapsed: 0.0,
            last_tick: None,
            last_state: 0,
            on_top: true,
            luck: luck.clamp(0.0, 10.0) as i32,
            current_bet: min_bet,
            total_bet: 0,
            chips,
            bets: Vec::new(),
            next_model: FIRST_CHIP,
            spots,
            places,
            valid_radius,
            special_radius,
            cursor: [0.0; 3],
            models,
            result: 0,
            spin: 0,
            clean_table: false,
            lucky: false,
            unlucky: false,
            new_level: false,
            face: None,
            invalid: false,
            data,
            alpha: [ALPHA_FULL; 11],
            status: None,
            status_shown: true,
            bet_in_label: None,
        };
        let mut fx = vec![Effect::Place {
            model: 1,
            at: m.models[1],
        }];
        fx.push(Effect::Place {
            model: 2,
            at: m.models[2],
        });
        for t in [tile::FINISH_BET, tile::REMOVE_BET, tile::DECREASE_BET] {
            m.alpha[t as usize] = ALPHA_DIM;
        }
        m.reset_chips(&mut fx);
        m.set_bet_chip(&mut fx);
        fx.push(Effect::Marker { valid: true });
        (m, fx)
    }

    fn place(&mut self, fx: &mut Vec<Effect>, model: usize, at: [f32; 3]) {
        if let Some(slot) = self.models.get_mut(model) {
            *slot = at;
            fx.push(Effect::Place { model, at });
        }
    }

    /// `ResetBetChips` (`007bf7a0`): every chip and shadow at (0, 0, −5).
    fn reset_chips(&mut self, fx: &mut Vec<Effect>) {
        for k in FIRST_CHIP..FIRST_CHIP + MAX_BETS {
            self.place(fx, k, [0.0, 0.0, HIDDEN_Z]);
            self.place(fx, k + SHADOW_OFFSET, [0.0, 0.0, HIDDEN_Z]);
        }
    }

    /// `SetBetChip` (`007bf840`): the cursor's chip as high as the bet, its
    /// shadow on the felt where it is.
    fn set_bet_chip(&mut self, fx: &mut Vec<Effect>) {
        let k = self.next_model;
        let [x, y, _] = self.models[k];
        let z = chips_shown(self.current_bet) as f32 * 0.18;
        self.place(fx, k, [x, y, z]);
        self.place(fx, k + SHADOW_OFFSET, [x, y, 0.0]);
    }

    fn hidden(&self, k: usize) -> bool {
        self.models[k][2] < -3.0
    }

    fn dim(&mut self, tiles: &[i32], a: f32) {
        for &t in tiles {
            self.alpha[t as usize] = a;
        }
    }

    /// `GetBetIndex` (`007bf390`): the first grid spot within its radius
    /// of the cursor, else the first special within its own; else 159.
    pub fn bet_index(&self) -> usize {
        let p = self.models[2];
        let d = |i: usize| {
            let q = self.places.get(i).copied().unwrap_or([f32::MAX; 3]);
            ((p[0] - q[0]).powi(2) + (p[1] - q[1]).powi(2) + (p[2] - q[2]).powi(2)).sqrt()
        };
        if let Some(i) = (0..144).find(|&i| d(i) <= self.valid_radius) {
            return i;
        }
        (144..SPOTS)
            .find(|&i| d(i) <= self.special_radius[i - 144])
            .unwrap_or(SPOTS)
    }

    /// `SetBet` (`007bed70`).
    fn set_bet(&mut self, index: usize, fx: &mut Vec<Effect>) -> bool {
        if self.bets.len() >= MAX_BETS
            || index >= SPOTS
            || self.current_bet == 0
            || self.bets.iter().any(|b| b.index == index)
        {
            return false;
        }
        self.bets.insert(
            0,
            Bet {
                value: self.current_bet,
                index,
                model: self.next_model,
            },
        );
        self.set_bet_chip(fx);
        self.next_model += 1;
        self.total_bet += self.current_bet;
        fx.push(Effect::Sound("GAMERouletteIncreaseBet"));
        if self.next_model > FIRST_CHIP + MAX_BETS - 1 {
            self.next_model = FIRST_CHIP;
        }
        while !self.hidden(self.next_model) && self.bets.len() < MAX_BETS {
            self.next_model += 1;
            if self.next_model > FIRST_CHIP + MAX_BETS - 1 {
                self.next_model = FIRST_CHIP;
            }
        }
        if self.bets.len() < MAX_BETS && self.total_bet < self.max_bet {
            let [x, y, _] = self.models[2];
            let k = self.next_model;
            self.place(fx, k, [x, y, 0.0]);
            self.place(fx, k + SHADOW_OFFSET, [x, y, 0.0]);
            self.set_bet_chip(fx);
        }
        if self.chips < self.total_bet + self.current_bet {
            self.current_bet = self.chips - self.total_bet;
            self.dim(&[tile::INCREASE_BET], ALPHA_DIM);
        }
        true
    }

    /// `RemoveBet` (`007bf150`).
    fn remove_bet(&mut self, index: usize, fx: &mut Vec<Effect>) -> bool {
        let Some(at) = self.bets.iter().position(|b| b.index == index) else {
            return false;
        };
        let bet = self.bets[at];
        self.total_bet -= bet.value;
        // `HideBetChip` (`007bf480`).
        self.place(fx, bet.model, [0.0, 0.0, HIDDEN_Z]);
        self.place(fx, bet.model + SHADOW_OFFSET, [0.0, 0.0, HIDDEN_Z]);
        if self.bets.len() == MAX_BETS {
            self.next_model = bet.model;
            self.set_bet_chip(fx);
            let z = self.models[self.next_model][2];
            let [x, y, _] = self.cursor;
            let k = self.next_model;
            self.place(fx, k, [x, y, z]);
            self.place(fx, k + SHADOW_OFFSET, [x, y, 0.0]);
        }
        self.bets.remove(at);
        true
    }

    fn close(&mut self) {
        self.state = state::CLOSING;
    }

    /// A tile clicked (`DoClick` `007bc870`, only idle).
    pub fn click(&mut self, id: i32, dice: Dice) -> Vec<Effect> {
        let mut fx = Vec::new();
        if self.state != state::IDLE {
            return fx;
        }
        match id {
            tile::PLACE_BET => {
                let i = self.bet_index();
                if i < SPOTS && self.total_bet < self.max_bet && self.set_bet(i, &mut fx) {
                    if self.max_bet <= self.total_bet + self.current_bet {
                        self.current_bet = self.max_bet - self.total_bet;
                        self.dim(
                            &[tile::PLACE_BET, tile::INCREASE_BET, tile::DECREASE_BET],
                            ALPHA_DIM,
                        );
                    }
                    self.dim(&[tile::REMOVE_BET, tile::FINISH_BET], ALPHA_FULL);
                    if self.current_bet == 0 {
                        self.dim(&[tile::PLACE_BET, tile::DECREASE_BET], ALPHA_DIM);
                    }
                    if self.bets.len() == MAX_BETS {
                        self.current_bet = 0;
                        self.dim(
                            &[tile::PLACE_BET, tile::INCREASE_BET, tile::DECREASE_BET],
                            ALPHA_DIM,
                        );
                    }
                }
            }
            tile::REMOVE_BET => {
                let i = self.bet_index();
                if i < SPOTS && self.remove_bet(i, &mut fx) {
                    fx.push(Effect::Sound("GAMERouletteDecreaseBet"));
                    self.dim(&[tile::INCREASE_BET, tile::PLACE_BET], ALPHA_FULL);
                    if self.bets.is_empty() {
                        self.dim(&[tile::FINISH_BET, tile::REMOVE_BET], ALPHA_DIM);
                    }
                    if self.bets.len() == MAX_BETS - 1 {
                        self.dim(
                            &[
                                tile::FINISH_BET,
                                tile::REMOVE_BET,
                                tile::PLACE_BET,
                                tile::INCREASE_BET,
                                tile::DECREASE_BET,
                                tile::EXIT,
                            ],
                            ALPHA_FULL,
                        );
                    }
                }
            }
            tile::FINISH_BET if !self.bets.is_empty() => {
                // `SpinWheel` (`007bf670`).
                self.spin = dice(3);
                self.result = dice(38) as u8;
                self.lucky = self.luck > 5;
                self.unlucky = self.luck < 5;
                self.dim(
                    &[
                        tile::FINISH_BET,
                        tile::REMOVE_BET,
                        tile::PLACE_BET,
                        tile::INCREASE_BET,
                        tile::DECREASE_BET,
                        tile::EXIT,
                    ],
                    ALPHA_DIM,
                );
                self.state = state::SPIN;
                fx.push(Effect::GamePlayed);
            }
            tile::INCREASE_BET => {
                let before = self.current_bet;
                self.current_bet += match self.current_bet {
                    b if b < 10 => 1,
                    b if b < 40 => 5,
                    b if b < 100 => 10,
                    b if b < 500 => 25,
                    _ => 100,
                };
                self.dim(&[tile::DECREASE_BET], ALPHA_FULL);
                if self.chips <= self.current_bet + self.total_bet {
                    self.current_bet = self.chips - self.total_bet;
                    self.dim(&[tile::INCREASE_BET], ALPHA_DIM);
                }
                if self.max_bet <= self.total_bet + self.current_bet {
                    self.current_bet = self.max_bet - self.total_bet;
                    self.dim(&[tile::INCREASE_BET], ALPHA_DIM);
                }
                if self.bets.len() != MAX_BETS {
                    self.set_bet_chip(&mut fx);
                }
                if before != self.current_bet {
                    fx.push(Effect::Sound("GAMERouletteValidBet"));
                }
            }
            tile::DECREASE_BET => {
                let before = self.current_bet;
                let b = self.current_bet;
                let step = |s: i32| if b % s == 0 { b - s } else { b / s * s };
                self.current_bet = match b {
                    b if b < 11 => b - 1,
                    b if b < 41 => step(5),
                    b if b < 101 => step(10),
                    b if b < 501 => step(25),
                    _ => step(100),
                };
                self.dim(&[tile::INCREASE_BET], ALPHA_FULL);
                if self.current_bet <= self.min_bet && self.min_bet + self.total_bet < self.max_bet
                {
                    self.current_bet = self.min_bet;
                    self.dim(&[tile::DECREASE_BET], ALPHA_DIM);
                }
                if self.current_bet < 1 {
                    self.current_bet = 0;
                    self.dim(&[tile::DECREASE_BET], ALPHA_DIM);
                }
                if self.bets.len() != MAX_BETS {
                    self.set_bet_chip(&mut fx);
                }
                if before != self.current_bet {
                    fx.push(Effect::Sound("GAMERouletteValidBet"));
                }
            }
            tile::EXIT => self.close(),
            _ => {}
        }
        fx
    }

    /// One update (`DoIdle`): `tick` the time in milliseconds, `moved`
    /// this frame's cursor movement (`kx`, `ky`: the mouse's `(s · −dx) /
    /// 10`, `(s · dy) / 10` with `s` 960 over the screen's height, or the
    /// stick's).
    pub fn update(&mut self, tick: u32, moved: (f32, f32), clips: &dyn Clips) -> Vec<Effect> {
        let mut fx = Vec::new();
        match self.last_tick {
            None => self.elapsed = 0.0,
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
        match self.state {
            state::SPIN => {
                let sequence = SPINS[self.spin.min(2)];
                if let Some(end) = clips.end(sequence) {
                    if t == 0.0 {
                        fx.push(Effect::Sound("GAMERouletteSpin"));
                        self.status_shown = false;
                        fx.push(Effect::Activate(sequence));
                        self.clean_table = true;
                        fx.push(Effect::WheelTurn(wheel_angle(self.spin, self.result)));
                    }
                    fx.push(Effect::UpdateTable(t));
                    if end / 2.0 < t && self.clean_table {
                        self.reset_chips(&mut fx);
                        self.next_model = FIRST_CHIP;
                        let [x, y, _] = self.models[2];
                        self.place(&mut fx, FIRST_CHIP, [x, y, 0.0]);
                        self.set_bet_chip(&mut fx);
                        self.clean_table = false;
                    }
                    if end < t {
                        self.state = state::RESULTS;
                        fx.push(Effect::Deactivate(sequence));
                    }
                }
            }
            state::RESULTS => self.results(&mut fx),
            state::IDLE => {
                let (kx, ky) = moved;
                if kx != 0.0 || ky != 0.0 {
                    self.move_cursor(kx, ky, &mut fx);
                }
                if self.chips < self.current_bet + self.total_bet {
                    self.current_bet = self.chips - self.total_bet;
                    self.bet_in_label = Some(self.current_bet);
                    self.dim(&[tile::INCREASE_BET], ALPHA_DIM);
                }
                if self.chips < self.min_bet {
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

    /// The cursor moved: it, the chip on it and that chip's shadow follow;
    /// the spot under it shown and its words on the status line.
    fn move_cursor(&mut self, kx: f32, ky: f32, fx: &mut Vec<Effect>) {
        let c = &mut self.cursor;
        c[0] -= kx * 0.35;
        c[1] -= ky * 0.35;
        if c[0] < CURSOR_X.0 {
            c[0] = CURSOR_X.0;
        }
        if c[1] < CURSOR_Y.0 {
            c[1] = CURSOR_Y.0;
        }
        if c[0] > CURSOR_X.1 {
            c[0] = CURSOR_X.1;
        }
        if c[1] > CURSOR_Y.1 {
            c[1] = CURSOR_Y.1;
        }
        let at = self.cursor;
        self.place(fx, 2, at);
        if self.bets.len() < MAX_BETS {
            let k = self.next_model;
            let z = self.models[k][2];
            self.place(fx, k, [at[0], at[1], z]);
            self.place(fx, k + SHADOW_OFFSET, [at[0], at[1], 0.0]);
        }
        let i = self.bet_index();
        if i < SPOTS {
            self.face = Some(i);
            fx.push(Effect::Face(Some(self.spots[i].face.clone())));
            self.status = Some(Status::Spot(i));
            if self.invalid {
                self.invalid = false;
                fx.push(Effect::Sound("GAMERouletteValidBet"));
            }
            fx.push(Effect::Marker { valid: true });
        } else {
            self.status = Some(Status::NoSpot);
            if !self.invalid {
                self.invalid = true;
                fx.push(Effect::Sound("GAMERouletteInvalidBet"));
            }
            fx.push(Effect::Marker { valid: false });
            if self.face.is_some() {
                fx.push(Effect::Face(None));
            }
        }
    }

    /// State 3 (`007bd2a0`): the buttons back, every bet paid or lost, the
    /// result line, the table cleared, the level check.
    fn results(&mut self, fx: &mut Vec<Effect>) {
        self.state = state::IDLE;
        self.dim(&[tile::FINISH_BET, tile::PLACE_BET], ALPHA_FULL);
        self.dim(&[tile::REMOVE_BET], ALPHA_DIM);
        let a = if self.current_bet < self.min_bet {
            ALPHA_DIM
        } else {
            ALPHA_FULL
        };
        self.dim(&[tile::DECREASE_BET], a);
        // Written again at once (the game's second test, meant for tile 6).
        let a = if self.max_bet < self.current_bet || self.current_bet == self.chips {
            ALPHA_DIM
        } else {
            ALPHA_FULL
        };
        self.dim(&[tile::DECREASE_BET], a);
        self.dim(&[tile::EXIT], ALPHA_FULL);
        let mut won = 0;
        for bet in std::mem::take(&mut self.bets) {
            if wins(&self.spots, bet.index, self.result) {
                won += (self.spots[bet.index].payout + 1) * bet.value;
            }
        }
        let net = won - self.total_bet;
        self.chips += net;
        self.data.winnings += net;
        fx.push(Effect::Data(self.data));
        self.status = Some(if net > 0 {
            fx.push(Effect::Sound("GAMERouletteWin"));
            Status::Won {
                amount: net,
                lucky: self.lucky,
            }
        } else if net < 0 {
            Status::Lost { amount: -net }
        } else {
            Status::Even
        });
        self.status_shown = true;
        self.next_model = FIRST_CHIP;
        self.total_bet = 0;
        self.current_bet = 0;
        if self.current_bet == 0 {
            self.current_bet = self.min_bet;
        }
        self.new_level = super::new_level(&mut self.data, self.max_winnings);
        fx.push(Effect::Data(self.data));
        if self.new_level {
            self.close();
        }
        self.dim(&[tile::INCREASE_BET], ALPHA_FULL);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(_: &str, default: &str) -> String {
        default.to_string()
    }

    /// The spots' places as the vanilla Points model has them (x right,
    /// y away; the grid 2 apart, rows 2.667 apart).
    fn places() -> Vec<[f32; 3]> {
        let mut v: Vec<[f32; 3]> = (0..144)
            .map(|i| {
                let (row, c) = (i / 24, i % 24);
                [-24.0 + 2.0 * c as f32, -2.6667 + 2.6667 * row as f32, 0.075]
            })
            .collect();
        let s: [(f32, f32); 15] = [
            (-26.0, 9.3333),
            (-26.0, 5.3333),
            (-26.0, 1.3333),
            (-16.0, -5.3333),
            (-20.0, -10.6667),
            (-12.0, -10.6667),
            (0.0, -5.3333),
            (-4.0, -10.6667),
            (4.0, -10.6667),
            (16.0, -5.3333),
            (12.0, -10.6667),
            (20.0, -10.6667),
            (26.0, 0.0),
            (26.0, 5.3333),
            (26.0, 10.6667),
        ];
        v.extend(s.iter().map(|&(x, y)| [x, y, 0.075]));
        v
    }

    struct Ends;
    impl Clips for Ends {
        fn end(&self, _: &str) -> Option<f32> {
            Some(9.8333)
        }
    }

    #[test]
    fn the_spots() {
        let s = spots(&text);
        assert_eq!(s.len(), SPOTS);
        assert_eq!(s[25].numbers, [1]);
        assert_eq!(s[25].text, "Bet: 1 \nPayout: 35:1");
        assert_eq!(s[0].text, "Bet: 0,00,1,2,3 \nPayout: 6:1");
        assert_eq!(s[2].numbers, [1, 2, 3, 4, 5, 6]);
        assert_eq!(s[50].numbers, [1, 2, 4, 5]);
        assert_eq!(s[72].text, "Bet: 0,00,2 \nPayout: 11:1");
        assert_eq!(s[96].text, "Bet: 00,2,3 \nPayout: 11:1");
        assert_eq!(s[143].numbers, [36]);
        assert_eq!(s[143].face, "F24:0");
        assert_eq!(s[151].text, "Bet: Red \nPayout: 1:1");
        assert_eq!(s[156].face, "S13:0");
    }

    #[test]
    fn zero_and_double_zero_quirks() {
        let s = spots(&text);
        // 0 wins even, red, the third column; 00 odd, black, the first.
        assert!(wins(&s, 149, 0) && wins(&s, 151, 0) && wins(&s, 158, 0));
        assert!(wins(&s, 154, 37) && wins(&s, 152, 37) && wins(&s, 156, 37));
        assert!(!wins(&s, 147, 0) && !wins(&s, 155, 37));
        // 1 is red (its place on the wheel is 20), 2 black.
        assert!(wins(&s, 151, 1) && wins(&s, 152, 2));
        assert!(wins(&s, 146, 0) && wins(&s, 144, 37) && wins(&s, 145, 0));
    }

    #[test]
    fn radii_from_the_places() {
        let (valid, special) = radii(&places());
        assert!((valid - 0.98).abs() < 1e-3);
        assert!((special[0] - 0.98).abs() < 1e-3);
        assert!((special[3] - 1.3067).abs() < 1e-3);
        assert!((special[4] - 3.92).abs() < 1e-3);
        assert!((special[12] - 1.96).abs() < 1e-3);
    }

    /// Betting on 17 and red, a spin, paid, the table cleared.
    #[test]
    fn a_spin() {
        let (mut m, _) = Roulette::open(
            9000,
            1,
            100,
            100,
            5.0,
            CasinoData::default(),
            spots(&text),
            places(),
        );
        let mut tick = 1000;
        m.update(tick, (0.0, 0.0), &Ends);
        // To 17 (B... straights: 17 = D12 at (−2, 5.333)): the cursor moves
        // by −k × 0.35 a frame.
        m.update(tick, (2.0 / 0.35, -5.3333 / 0.35), &Ends);
        assert_eq!(m.bet_index(), 83, "{:?}", m.cursor);
        assert_eq!(m.status, Some(Status::Spot(83)));
        m.click(tile::INCREASE_BET, &mut |_| 0);
        m.click(tile::PLACE_BET, &mut |_| 0);
        assert_eq!((m.bets.len(), m.total_bet, m.current_bet), (1, 2, 2));
        // Red: S08 at (−4, −10.667).
        m.update(tick, (2.0 / 0.35, 16.0 / 0.35), &Ends);
        assert_eq!(m.bet_index(), 151, "{:?}", m.cursor);
        m.click(tile::PLACE_BET, &mut |_| 0);
        assert_eq!(m.total_bet, 4);
        // The ball on 17: black (its place on the wheel, 11, is odd), so the
        // red bet loses.
        let mut dice = |n: usize| if n == 38 { 17 } else { 0 };
        m.click(tile::FINISH_BET, &mut dice);
        assert_eq!(m.state, state::SPIN);
        let mut fx = Vec::new();
        while m.state != state::IDLE {
            tick += 16;
            fx.extend(m.update(tick, (0.0, 0.0), &Ends));
            assert!(tick < 30_000);
        }
        // 17 pays 36 × 2; red loses 2: net 72 − 4.
        assert_eq!(m.chips, 100 + 68);
        assert_eq!(
            m.status,
            Some(Status::Won {
                amount: 68,
                lucky: false
            })
        );
        assert!(m.bets.is_empty() && m.total_bet == 0 && m.current_bet == 1);
        assert!(fx.contains(&Effect::Sound("GAMERouletteSpin")));
        assert_eq!(m.data.winnings, 68);
    }
}
