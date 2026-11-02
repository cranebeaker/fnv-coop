//! The game's menus, one module each, each following its menu class's code
//! in FalloutNV.exe (filling its tiles, answering clicks and keys). The
//! game state they show comes from the caller; what the player chooses goes
//! back the same way. Findings with addresses:
//! `%USERPROFILE%\nv-re\findings\menus.md`.

pub mod barter;
pub mod blackjack;
pub mod caravan;
pub mod chargen;
pub mod companion_wheel;
pub mod computers;
pub mod container;
pub mod dialog;
pub mod hacking;
pub mod levelup;
pub mod message;
pub mod quantity;
pub mod recipe;
pub mod repair_services;
pub mod roulette;
pub mod sleepwait;
pub mod slots;
pub mod start;
pub mod textedit;
pub mod traits;
pub mod tutorial;
mod typed;
pub mod vigor;

#[cfg(test)]
pub(crate) mod test_support;
