//! The casinos' games (`CSNO`, `TESCasino`): what blackjack, roulette and
//! the slot machines share (read from FalloutNV.exe 1.4.0.525, the Xbox
//! 360 prototype's PDB naming the parts).
//!
//! - The record ([`Casino`], loader `005048d0`): its name (`FULL`), ten
//!   models (`MODL`s in order, then `MOD2` the slot machine, `MOD3` the
//!   blackjack table, `MOD4` the roulette table), eleven textures (`ICON`s
//!   the reels' seven symbols, `ICO2`s the four card backs) and `DATA`
//!   (`CASINO_DATA`, 0x38 bytes).
//! - The player's list of casinos played at (`PlayerCharacter` +0x610,
//!   [`CasinoData`]): the net chips won there by every round of every game
//!   (it can go below 0) and the level reached. A game's `Create` adds the
//!   casino's entry (at the list's head) before anything else.
//! - Levels: k (1 to 4) once the winnings reach k × 25 % of the casino's
//!   `iMaxWinnings` ([`level_of`]); a round reaching a new one closes the
//!   menu and starts the casino's comps quest when it closes. Banned: the
//!   winnings at the limit; the game is then refused.
//! - Opening (`Create`: blackjack `00733630`, roulette `007bbe20`, slots
//!   `007c0a40`) refuses in turn: the anti-cheat lock ([`AntiCheat`]), the
//!   ban, too few chips for the least bet, too little won for the script's
//!   fourth number (a least winnings, though the command calls it "Max
//!   Winnings"; blackjack never passes it). See [`open_check`].
//! - The menu keeps its own chip count from the player's at the start and
//!   settles once on closing ([`settle`]); chips aren't taken as they're
//!   bet.

use esm::{FormId, FourCC, LoadOrder};

use crate::dialogue::PLAYER_REF;
use crate::scripting::GameState;

pub mod blackjack;
pub mod roulette;
pub mod slots;

const CSNO: FourCC = FourCC::new(b"CSNO");
const MODL: FourCC = FourCC::new(b"MODL");
const MOD2: FourCC = FourCC::new(b"MOD2");
const MOD3: FourCC = FourCC::new(b"MOD3");
const MOD4: FourCC = FourCC::new(b"MOD4");
const ICON: FourCC = FourCC::new(b"ICON");
const ICO2: FourCC = FourCC::new(b"ICO2");
const DATA: FourCC = FourCC::new(b"DATA");

/// A number 0..n from the game's random numbers (`00944460`: `lo +
/// MT() % (hi − lo)`).
pub type Dice<'a> = &'a mut dyn FnMut(usize) -> usize;

/// The model slots (`TESCasino::CASINOMODELS`).
pub mod model {
    /// The chips' stacks: 1, 5, 10, 25, 100, 500.
    pub const CHIPS: usize = 0;
    pub const ROULETTE_CHIP: usize = 6;
    pub const SLOT_MACHINE: usize = 7;
    pub const BLACKJACK_TABLE: usize = 8;
    pub const ROULETTE_TABLE: usize = 9;
}

/// The texture slots (`CASINOTEXTURES`): the reels' symbols A to F and the
/// wild, then four decks' card backs.
pub mod texture {
    pub const REELS: usize = 0;
    pub const DECKS: usize = 7;
}

/// A casino (`TESCasino`).
#[derive(Debug, Clone, PartialEq)]
pub struct Casino {
    pub form: FormId,
    /// `FULL` (Gomorrah's is spelled "Gommorah" in FalloutNV.esm, and the
    /// earnings line shows it so).
    pub name: String,
    pub models: [String; 10],
    pub textures: [String; 11],
    /// `fBJ_ShufflePercent`, `fBJ_BlackjackPayout` (as stored: 0 means the
    /// game's default, [`Casino::shuffle`], [`Casino::payout`]).
    pub shuffle_percent: f32,
    pub blackjack_payout: f32,
    /// `iSlotStop_A..F`, `iSlotStop_W`.
    pub slot_stops: [i32; 7],
    /// `iBJNumDecks` (0 means 1).
    pub decks: i32,
    pub max_winnings: i32,
    /// The chip played with (`CHIP`).
    pub chip: FormId,
    /// The comps quest a new level starts.
    pub quest: FormId,
    /// `bBJ_DealerHoleCard`.
    pub dealer_hole_card: bool,
}

impl Casino {
    pub fn load(order: &LoadOrder, form: FormId) -> Option<Casino> {
        let rr = order.get(form)?;
        if rr.entry.header.kind != CSNO {
            return None;
        }
        let record = rr.record().ok()?;
        let mut models: [String; 10] = Default::default();
        let mut textures: [String; 11] = Default::default();
        let (mut model_at, mut icon_at, mut ico2_at) = (0, 0, texture::DECKS);
        for s in &record.subrecords {
            let slot = match s.kind {
                k if k == MODL => {
                    model_at += 1;
                    Some((true, model_at - 1))
                }
                k if k == MOD2 => Some((true, model::SLOT_MACHINE)),
                k if k == MOD3 => Some((true, model::BLACKJACK_TABLE)),
                k if k == MOD4 => Some((true, model::ROULETTE_TABLE)),
                k if k == ICON => {
                    icon_at += 1;
                    Some((false, icon_at - 1))
                }
                k if k == ICO2 => {
                    ico2_at += 1;
                    Some((false, ico2_at - 1))
                }
                _ => None,
            };
            match slot {
                Some((true, i)) if i < models.len() => models[i] = s.zstring(),
                Some((false, i)) if i < textures.len() => textures[i] = s.zstring(),
                _ => {}
            }
        }
        let data = record.get(DATA).map(|s| s.data.as_slice()).unwrap_or(&[]);
        let word = |at: usize| {
            data.get(at..at + 4)
                .map_or(0, |b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
        };
        let mut slot_stops = [0; 7];
        for (i, s) in slot_stops.iter_mut().enumerate() {
            *s = word(8 + 4 * i) as i32;
        }
        Some(Casino {
            form,
            name: record.full_name().unwrap_or_default(),
            models,
            textures,
            shuffle_percent: f32::from_bits(word(0)),
            blackjack_payout: f32::from_bits(word(4)),
            slot_stops,
            decks: word(0x24) as i32,
            max_winnings: word(0x28) as i32,
            chip: FormId(word(0x2c)),
            quest: FormId(word(0x30)),
            dealer_hole_card: data.get(0x34).is_some_and(|&b| b != 0),
        })
    }
}

/// One casino's line in the player's list (`CasinoData`, 12 bytes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CasinoData {
    pub casino: FormId,
    /// `iTotalWinnings`: the net chips won there.
    pub winnings: i32,
    /// `iWinningLevel`: the last level a round reached.
    pub level: i16,
}

/// The level winnings come to (`GetCasinoWinningsLevel` `005dec30`, the
/// condition `GetCasinoWinningStage` `005a6170`): the highest k of 1..4
/// with `winnings ≥ k × 0.25 × iMaxWinnings` (in doubles), else 0.
pub fn level_of(max_winnings: i32, winnings: i32) -> i32 {
    (1..=4)
        .rev()
        .find(|&k| f64::from(winnings) >= f64::from(k) * 0.25 * f64::from(max_winnings))
        .unwrap_or(0)
}

/// The player's line for a casino.
pub fn data(state: &GameState, casino: FormId) -> Option<CasinoData> {
    state.casinos.iter().find(|d| d.casino == casino).copied()
}

/// The player's line for a casino, made (winnings 0, level 0, at the
/// list's head, `005ae3d0`) if there isn't one (`Create`).
pub fn data_mut(state: &mut GameState, casino: FormId) -> &mut CasinoData {
    let at = match state.casinos.iter().position(|d| d.casino == casino) {
        Some(i) => i,
        None => {
            state.casinos.insert(
                0,
                CasinoData {
                    casino,
                    ..CasinoData::default()
                },
            );
            0
        }
    };
    &mut state.casinos[at]
}

/// `GetCasinoWinningsLevel` / `GetCasinoWinningStage`: worked out from the
/// winnings (not the stored level, so it falls again with losses); 0 for
/// a casino not played at.
pub fn winnings_level(order: &LoadOrder, state: &GameState, casino: FormId) -> i32 {
    match (data(state, casino), Casino::load(order, casino)) {
        (Some(d), Some(c)) => level_of(c.max_winnings, d.winnings),
        _ => 0,
    }
}

/// `SetCasinoWinningsLevel` (`005ded40`): for a level 0..4 and a casino
/// played at, the level and winnings of exactly that many quarters of
/// the limit (truncated, `00ec62c0`).
pub fn set_winnings_level(order: &LoadOrder, state: &mut GameState, casino: FormId, level: i32) {
    if !(0..=4).contains(&level) {
        return;
    }
    let Some(c) = Casino::load(order, casino) else {
        return;
    };
    if let Some(d) = state.casinos.iter_mut().find(|d| d.casino == casino) {
        d.level = level as i16;
        d.winnings = (f64::from(level) * 0.25 * f64::from(c.max_winnings)) as i32;
    }
}

/// A round's net chips (`iTotalWinnings += Δ`), then the level check
/// (blackjack `00736b56`, roulette `007be412`, slots `007c360d`): each
/// level above the stored one that the winnings reach is stored and asks
/// for the menu to close (the flag the last level sets: a level not above
/// the stored one clears it). True when the menu closes.
pub fn round_won(data: &mut CasinoData, max_winnings: i32, delta: i32) -> bool {
    data.winnings += delta;
    new_level(data, max_winnings)
}

/// The level check on its own (the slots run it after their coins, not
/// with the payout).
pub fn new_level(data: &mut CasinoData, max_winnings: i32) -> bool {
    let mut flag = false;
    for k in 1..=4i16 {
        if k <= data.level {
            flag = false;
        } else if f64::from(data.winnings) >= f64::from(k) * 0.25 * f64::from(max_winnings) {
            data.level = k;
            flag = true;
        }
    }
    flag
}

/// The anti-cheat lock (globals, not saved): closing any casino menu
/// stamps the time (`00969b20`, `GetTickCount() / 1000`); loading a game
/// with a stamp arms it (`00969ac0`, from `00956f70` / `0095a3b0`); while
/// armed and `stamp + iAntiCheatDuration` (60) is still ahead, every game
/// is refused (`CasinoAntiCheatCheck` `00969b80`), and once it isn't the
/// stamp and the arming are cleared.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AntiCheat {
    pub stamp: Option<u64>,
    pub armed: bool,
}

impl AntiCheat {
    /// A casino menu closed at `now` (whole seconds).
    pub fn closed(&mut self, now: u64) {
        self.stamp = Some(now);
    }

    /// A game loaded.
    pub fn loaded(&mut self) {
        if self.stamp.is_some() {
            self.armed = true;
        }
    }

    /// The seconds still to wait (`00969b40`), or `None` when the games are
    /// open (which clears the lock).
    pub fn check(&mut self, now: u64, duration: i64) -> Option<i64> {
        if let (true, Some(stamp)) = (self.armed, self.stamp) {
            let remaining = stamp as i64 + duration - now as i64;
            if remaining > 0 {
                return Some(remaining);
            }
        }
        *self = AntiCheat::default();
        None
    }
}

/// Which game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Game {
    Blackjack,
    Roulette,
    Slots,
}

impl Game {
    /// Its menu's number (`00a09030`'s ids, as `MenuMode` takes them).
    pub fn menu(self) -> u16 {
        match self {
            Game::Slots => 1080,
            Game::Blackjack => 1081,
            Game::Roulette => 1082,
        }
    }
}

/// Why a game won't open.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    /// The anti-cheat lock, with the seconds left.
    AntiCheat(i64),
    Banned,
    Broke,
    NotWonEnough,
}

/// `Create`'s checks in the game's order, after adding the player's line
/// for the casino (which stays even when refused).
pub fn open_check(
    order: &LoadOrder,
    state: &mut GameState,
    casino: &Casino,
    min_bet: i32,
    min_winnings: i32,
    anti_cheat: &mut AntiCheat,
    now: u64,
) -> Result<(), Refusal> {
    let d = *data_mut(state, casino.form);
    let duration = crate::scripting::game_setting(order, "iAntiCheatDuration").unwrap_or(60.0);
    if let Some(left) = anti_cheat.check(now, duration as i64) {
        return Err(Refusal::AntiCheat(left));
    }
    if d.winnings >= casino.max_winnings {
        return Err(Refusal::Banned);
    }
    if state.item_count(order, PLAYER_REF, casino.chip) < min_bet {
        return Err(Refusal::Broke);
    }
    if d.winnings < min_winnings && min_winnings != 0 {
        return Err(Refusal::NotWonEnough);
    }
    Ok(())
}

/// A text setting, else its exe default.
pub fn text(order: &LoadOrder, name: &str, default: &str) -> String {
    crate::scripting::game_setting_text(order, name).unwrap_or_else(|| default.to_string())
}

/// What a refusal says (a corner message with `UIPopUpMessageGeneral`):
/// the anti-cheat lock as `"%s\nTime Remaining: %i"` with the game's own
/// text (roulette's `"%s\n%s%i"` with `sAntiCheatTimeRemainingText`, the
/// same words).
pub fn refusal_text(order: &LoadOrder, game: Game, why: Refusal) -> String {
    match why {
        Refusal::AntiCheat(left) => {
            let (name, default) = match game {
                Game::Blackjack => (
                    "sBlackjackAntiCheatText",
                    "The dealer is taking a minute to swap out the decks as an anti-cheating measure.",
                ),
                Game::Roulette => (
                    "sRouletteAntiCheatText",
                    "The croupier is taking a minute to check the wheel for bias as an anti-cheating measure.",
                ),
                Game::Slots => (
                    "sSlotAntiCheatText",
                    "This machine appears to be taking a minute to reset itself as an anti-cheating measure.",
                ),
            };
            let remaining = if game == Game::Roulette {
                text(order, "sAntiCheatTimeRemainingText", "Time Remaining: ")
            } else {
                "Time Remaining: ".into()
            };
            format!("{}\n{remaining}{left}", text(order, name, default))
        }
        Refusal::Banned => banned_text(order),
        Refusal::Broke => text(
            order,
            "sGamblingBrokeText",
            "You must purchase chips before you can play this game.",
        ),
        Refusal::NotWonEnough => text(
            order,
            "sGamblingMinWinText",
            "You have not won enough at this casino to play this game.",
        ),
    }
}

fn banned_text(order: &LoadOrder) -> String {
    text(
        order,
        "sGamblingBannedText",
        "You have been banned from gambling at this casino.",
    )
}

/// The earnings line (`UpdateEarningsString`: blackjack `0073ab40`,
/// roulette `007bf9e0`, slots `007c5e60`): `"%s %s%i"`, the casino's name,
/// `sCasinoEarningsText` and the winnings. Its padding to 23 characters
/// (for a line shorter than that) writes past the end the second
/// `sprintf` leaves, so it only shows when the number is one character.
pub fn earnings_line(order: &LoadOrder, casino: &Casino, winnings: i32) -> String {
    let label = text(order, "sCasinoEarningsText", "Earnings: ");
    let full = format!("{} {label}{winnings}", casino.name);
    let digits = winnings.to_string();
    if full.len() >= 23 || digits.len() != 1 {
        return full;
    }
    // One character: "<name> <label>" padded to 22, then the digit.
    let mut s = format!("{} {label}", casino.name);
    while s.len() < 22 {
        s.push(' ');
    }
    format!("{s}{digits}")
}

/// What a round's result line says (`"%s %d %s%s"`): "You win 5 chip(s)",
/// the lucky and unlucky words when luck changed things; `amount` as the
/// game prints it.
pub fn result_line(order: &LoadOrder, won: bool, amount: i32, lucky: bool) -> String {
    let prefix = match (won, lucky) {
        (true, true) => text(order, "sLuckyWinText", "You feel lucky. You win"),
        (true, false) => text(order, "sYouWin", "You win"),
        (false, true) => text(order, "sUnluckyLoseText", "You feel unlucky. You lose"),
        (false, false) => text(order, "sYouLose", "You lose"),
    };
    format!(
        "{prefix} {amount} {}{}",
        text(order, "sCasinoChipText", "chip"),
        text(order, "sPlural", "(s)")
    )
}

/// What closing a game did with the player's chips.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Settled {
    Nothing,
    /// Taken away, with the corner message (sad Vault Boy).
    Removed {
        count: i32,
        message: String,
    },
    /// Given: under the limit with the game's usual "added" notice for
    /// them (`004821a0(player, 0)`: "%s %s" for one, "%i %s%s %s" for
    /// more, the gift box); at the limit given quietly (`004821a0(player,
    /// 1)`) with a message ending in the ban (very happy Vault Boy):
    /// `banned`.
    Added {
        count: i32,
        message: String,
        banned: bool,
    },
}

impl Settled {
    /// The picture beside its corner message (`QueueUIMessage`'s): the sad
    /// Vault Boy for chips taken, the very happy one with the ban, else
    /// the gift box of the game's usual "added" notice (`004821a0`,
    /// `0101c140`).
    pub fn icon(&self) -> Option<&'static str> {
        match self {
            Settled::Nothing => None,
            Settled::Removed { .. } => Some(crate::message_icon::SAD),
            Settled::Added { banned: true, .. } => Some(crate::message_icon::VERY_HAPPY),
            Settled::Added { .. } => Some(crate::message_icon::GIFT_BOX),
        }
    }
}

/// A refusal's picture: the surprised Vault Boy (every casino refusal and
/// "out of chips", with `UIPopUpMessageGeneral`).
pub const REFUSAL_ICON: &str = crate::message_icon::SURPRISED;

/// Closing a game (the closing state of each `DoIdle`: blackjack
/// `007350f6`, roulette `007bd65b`, slots ≈`007c2f2f`): the difference
/// between the menu's chips and the player's taken or given (`RemoveItem`,
/// or a container of them given), then a new level starts the casino's
/// quest (`0060c9c0(quest, 1)`, as `StartQuest`). The caller stamps the
/// anti-cheat lock ([`AntiCheat::closed`]) as the menu goes.
pub fn settle(
    order: &LoadOrder,
    state: &mut GameState,
    casino: &Casino,
    menu_chips: i32,
    new_level: bool,
) -> Settled {
    let have = state.item_count(order, PLAYER_REF, casino.chip);
    let diff = menu_chips - have;
    let name = order
        .get(casino.chip)
        .and_then(|r| r.record().ok())
        .and_then(|r| r.full_name())
        .unwrap_or_default();
    let plural = text(order, "sPlural", "(s)");
    let settled = if diff < 0 {
        let n = state.items.entry((PLAYER_REF, casino.chip)).or_insert(0);
        *n = (*n + diff).max(0);
        Settled::Removed {
            count: -diff,
            message: format!(
                "{} {name}{plural} {}",
                -diff,
                text(order, "sRemoveItemfromInventory", "removed")
            ),
        }
    } else if diff > 0 {
        *state.items.entry((PLAYER_REF, casino.chip)).or_insert(0) += diff;
        state.added(order, PLAYER_REF, casino.chip, diff);
        let winnings = data(state, casino.form).map_or(0, |d| d.winnings);
        let added = text(order, "sAddItemtoInventory", "added");
        let banned = winnings >= casino.max_winnings;
        let message = if banned {
            format!("{diff} {name}{plural} {added}\n{}", banned_text(order))
        } else if diff < 2 {
            format!("{name} {added}")
        } else {
            format!("{diff} {name}{plural} {added}")
        };
        Settled::Added {
            count: diff,
            message,
            banned,
        }
    } else {
        Settled::Nothing
    };
    if new_level && casino.quest.0 != 0 {
        state.running.insert(casino.quest);
    }
    settled
}

/// The player's list for the save: `casino <form> <winnings> <level>`, in
/// the list's order.
pub(crate) fn save_lines(state: &GameState, line: &mut dyn FnMut(String)) {
    for d in &state.casinos {
        line(format!(
            "casino {:08X} {} {}",
            d.casino.0, d.winnings, d.level
        ));
    }
}

pub(crate) fn load_line(state: &mut GameState, raw: &str) -> Option<Result<(), String>> {
    let parts: Vec<&str> = raw.split_whitespace().collect();
    if parts.first() != Some(&"casino") {
        return None;
    }
    let casino = parts.get(1).and_then(|s| u32::from_str_radix(s, 16).ok());
    let winnings = parts.get(2).and_then(|s| s.parse().ok());
    let level = parts.get(3).and_then(|s| s.parse().ok());
    match (casino, winnings, level) {
        (Some(c), Some(winnings), Some(level)) => {
            state.casinos.push(CasinoData {
                casino: FormId(c),
                winnings,
                level,
            });
            Some(Ok(()))
        }
        _ => Some(Err(format!("can't read '{raw}'"))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn levels_are_quarters_of_the_limit() {
        assert_eq!(level_of(9000, -50), 0);
        assert_eq!(level_of(9000, 2249), 0);
        assert_eq!(level_of(9000, 2250), 1);
        assert_eq!(level_of(9000, 6750), 3);
        assert_eq!(level_of(9000, 20000), 4);
        let mut d = CasinoData::default();
        // A quarter of 2500 is 625.
        assert!(!round_won(&mut d, 2500, 600));
        assert!(round_won(&mut d, 2500, 30));
        assert_eq!(d.level, 1);
        // A loss and a win back to the same level: nothing new.
        assert!(!round_won(&mut d, 2500, -100));
        assert!(!round_won(&mut d, 2500, 100));
        // Two levels at once: one close.
        assert!(round_won(&mut d, 2500, 1300));
        assert_eq!(d.level, 3);
    }

    #[test]
    fn the_anti_cheat_lock_needs_a_load_and_lasts_a_minute() {
        let mut a = AntiCheat::default();
        a.closed(1000);
        // No load: open.
        assert_eq!(a.check(1010, 60), None);
        a.closed(1000);
        a.loaded();
        assert_eq!(a.check(1010, 60), Some(50));
        assert_eq!(a.check(1060, 60), None);
        // Cleared once over.
        assert_eq!(a, AntiCheat::default());
    }
}
