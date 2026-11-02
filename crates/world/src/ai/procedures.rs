//! The procedure lists packages run (FalloutNV.exe 1.4.0.525).
//!
//! A package's record type (`PKDT` byte 4) picks one of the exe's
//! procedure lists when the package is set up (`006777b0`, stored at
//! package +0x18); the high-process update (`008eeec0`) runs the list's
//! current procedure through a switch (procedure names:
//! [`crate::more_functions::procedures::NAMES`]). Every list ends with
//! `PROCEDURE_DONE` (54). The table of lists is at `011a3ff0`.

use crate::more_functions::procedures as p;

/// `PROCEDURE_DONE`.
pub const DONE: u8 = p::DONE;
/// `PROCEDURE_FLEE_NON_COMBAT` (34): a flee package's procedure.
pub const FLEE_NON_COMBAT: u8 = 34;
/// `PROCEDURE_AMBUSH_WAIT` (30).
pub const AMBUSH_WAIT: u8 = 30;
/// `PROCEDURE_GUARD` (47).
pub const GUARD: u8 = 47;
/// `PROCEDURE_USE_ITEM_AT` (28).
pub const USE_ITEM_AT: u8 = 28;
/// `PROCEDURE_USE_WEAPON` (50).
pub const USE_WEAPON: u8 = 50;
/// `PROCEDURE_ACQUIRE` (3).
pub const ACQUIRE: u8 = 3;

// Translated from 011a3ff0 (data, FalloutNV.exe 1.4.0.525): the lists,
// indexed by list type, without their closing PROCEDURE_DONE.
const LISTS: [&[u8]; 49] = [
    &[0],
    &[0, 1],
    &[0, 2],
    &[0, 3],
    &[0, 4, 1],
    &[0, 5, 1],
    &[13, 6],
    &[0, 13, 6],
    &[17, 14, 13, 7],
    &[3, 7],
    &[0, 49, 13, 12],
    &[8, 15, 34, 41],
    &[9],
    &[14, 2],
    &[16],
    &[18],
    &[19],
    &[0, 20],
    &[15, 9],
    &[10],
    &[15, 21],
    &[22],
    &[0, 2, 13],
    &[0, 23],
    &[24],
    &[25],
    &[0, 3, 1],
    &[14, 27],
    &[3, 0, 28],
    &[29],
    &[0, 30],
    &[31],
    &[34],
    &[48, 36],
    &[37],
    &[38],
    &[39],
    &[0, 42],
    &[44],
    &[0, 45],
    &[46, 10],
    &[47],
    &[21, 41],
    &[21, 41],
    &[3, 0, 50],
    &[6],
    &[51],
    &[52],
    &[53],
];

/// The procedures of a list type, without the closing `DONE`.
pub fn procedures(list: u8) -> Option<&'static [u8]> {
    LISTS.get(usize::from(list)).copied()
}

/// The list a package record type runs.
// Translated from 006777b0 (decompiled, FalloutNV.exe 1.4.0.525), the
// record types whose list doesn't depend on the package's target: find
// (0), follow (1) and escort (2) choose by their target and second
// location and aren't covered here (`None`), nor are the types the
// switch's default makes invalid (11 and above 16).
pub fn list_type(kind: u8) -> Option<u8> {
    Some(match kind {
        3 => 5,
        4 => 4,
        5 => 1,
        6 => 0,
        7 => 27,
        8 => 28,
        9 => 30,
        10 => 32,
        12 => 37,
        13 => 38,
        14 => 41,
        15 => 10,
        16 => 44,
        _ => return None,
    })
}

/// Whether reaching `DONE` goes back to the list's last procedure instead
/// of finishing the package.
// Translated from 008eeec0 (decompiled, FalloutNV.exe 1.4.0.525), the
// PROCEDURE_DONE case: list types 1, 4, 5, 0x29 and 0x2d step back one
// (process vfunc +0x288 with -1): wander, sleep, eat, guard and follow
// repeat their last procedure.
pub fn repeats_at_done(list: u8) -> bool {
    matches!(list, 1 | 4 | 5 | 0x29 | 0x2d)
}

/// The procedures a package record type runs (its list), when known.
pub fn of_kind(kind: u8) -> Option<&'static [u8]> {
    list_type(kind).and_then(procedures)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::kinds;

    #[test]
    fn the_gunfight_packages_run_these_lists() {
        // Flee: only the non-combat flee procedure.
        assert_eq!(of_kind(kinds::FLEE), Some(&[FLEE_NON_COMBAT][..]));
        // Guard: the guard procedure, repeated.
        assert_eq!(of_kind(kinds::GUARD), Some(&[GUARD][..]));
        assert!(repeats_at_done(list_type(kinds::GUARD).unwrap()));
        // Ambush: travel there, then wait.
        assert_eq!(of_kind(kinds::AMBUSH), Some(&[p::TRAVEL, AMBUSH_WAIT][..]));
        // Travel: travel, then done (no repeat).
        assert_eq!(of_kind(kinds::TRAVEL), Some(&[p::TRAVEL][..]));
        assert!(!repeats_at_done(list_type(kinds::TRAVEL).unwrap()));
        assert!(!repeats_at_done(list_type(kinds::FLEE).unwrap()));
        // Use weapon and use item at: acquire, travel, then their own.
        assert_eq!(
            of_kind(kinds::USE_WEAPON),
            Some(&[ACQUIRE, p::TRAVEL, USE_WEAPON][..])
        );
        assert_eq!(
            of_kind(kinds::USE_ITEM_AT),
            Some(&[ACQUIRE, p::TRAVEL, USE_ITEM_AT][..])
        );
        // Dialogue: travel, activate, wait, dialogue (findings ai_rules §2).
        assert_eq!(of_kind(kinds::DIALOGUE), Some(&[0, 49, 13, 12][..]));
        // Sandbox: travel, then the sandbox procedure.
        assert_eq!(of_kind(kinds::SANDBOX), Some(&[p::TRAVEL, p::SANDBOX][..]));
        // Types that depend on the target, and invalid ones.
        assert_eq!(list_type(kinds::FIND), None);
        assert_eq!(list_type(11), None);
        assert_eq!(list_type(17), None);
        assert_eq!(procedures(49), None);
    }
}
