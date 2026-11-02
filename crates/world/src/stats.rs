//! The Pip-Boy's miscellaneous statistics (`GetPCMiscStat`,
//! `ModPCMiscStat`), by the game's own names and numbers (its table at
//! `01189280`). The game bumps them where things happen (read from its
//! code: 1 on finding a place, 4 on each lock picked, 5 on each terminal
//! hacked, 15 on each skill book read, 12 / 31 on speech checks passed or
//! failed, 23 on sleeping, 13 on a visit's first pocket picked:
//! `world::living`); kills by the player count as people or
//! creatures and in the total (where the game bumps those isn't traced).

use crate::scripting::GameState;

/// The statistics' names, by number.
pub const NAMES: [&str; 43] = [
    "Quests Completed",
    "Locations Discovered",
    "People Killed",
    "Creatures Killed",
    "Locks Picked",
    "Computers Hacked",
    "Stimpaks Taken",
    "Rad-X Taken",
    "RadAway Taken",
    "Chems Taken",
    "Times Addicted",
    "Mines Disarmed",
    "Speech Successes",
    "Pockets Picked",
    "Pants Exploded",
    "Books Read",
    "Health From Stimpaks",
    "Weapons Created",
    "Health From Food",
    "Water Consumed",
    "Sandman Kills",
    "Paralyzing Punches",
    "Robots Disabled",
    "Times Slept",
    "Corpses Eaten",
    "Mysterious Stranger Visits",
    "Doctor Bags Used",
    "Challenges Completed",
    "Miss Fortunate Occurrences",
    "Disintegrations",
    "Have Limbs Crippled",
    "Speech Failures",
    "Items Crafted",
    "Weapon Modifications",
    "Items Repaired",
    "Total Things Killed",
    "Dismembered Limbs",
    "Caravan Games Won",
    "Caravan Games Lost",
    "Barter Amount Traded",
    "Roulette Games Played",
    "Blackjack Games Played",
    "Slots Games Played",
];

pub const QUESTS_COMPLETED: u8 = 0;
pub const LOCATIONS_DISCOVERED: u8 = 1;
pub const PEOPLE_KILLED: u8 = 2;
pub const CREATURES_KILLED: u8 = 3;
pub const LOCKS_PICKED: u8 = 4;
pub const COMPUTERS_HACKED: u8 = 5;
pub const MINES_DISARMED: u8 = 11;
pub const POCKETS_PICKED: u8 = 13;
pub const BOOKS_READ: u8 = 15;
pub const TIMES_SLEPT: u8 = 23;
/// Bumped when a corpse the player killed disintegrates or turns to goo
/// (`world::more_functions::set_critical_stage`, `008a1a70`).
pub const DISINTEGRATIONS: u8 = 29;
pub const TOTAL_THINGS_KILLED: u8 = 35;

/// A statistic's number by its name (any case).
pub fn index(name: &str) -> Option<u8> {
    NAMES
        .iter()
        .position(|n| n.eq_ignore_ascii_case(name.trim()))
        .map(|i| i as u8)
}

/// A statistic now. Quests completed are counted from the quests.
pub fn get(state: &GameState, stat: u8) -> u32 {
    if stat == QUESTS_COMPLETED {
        return state.completed.len() as u32 + state.misc_stats.get(&stat).copied().unwrap_or(0);
    }
    state.misc_stats.get(&stat).copied().unwrap_or(0)
}

/// Adds to a statistic (`ModPCMiscStat` takes negative amounts too; it
/// doesn't go below 0). Every change counts for the challenges about that
/// statistic (`004d5e10` → `005f5950(11, …)`), which look when scripts next
/// run (`world::more_functions::challenges::catch_up`).
pub fn bump(state: &mut GameState, stat: u8, by: i64) {
    let v = state.misc_stats.entry(stat).or_insert(0);
    *v = (i64::from(*v) + by).max(0) as u32;
    if by != 0 {
        state.more.challenges.stat_bumps.push((stat, by));
    }
}
