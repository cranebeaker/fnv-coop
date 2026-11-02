//! The AI procedures, as `FalloutNV.exe` numbers them, and what
//! `GetCurrentAIProcedure` gives for each.
//!
//! A package runs a list of procedures one after another (the lists are at
//! `011a3ff0`, one per procedure array index, each ending with
//! `PROCEDURE_DONE`); the names are the exe's own table at `011a3cc0`
//! (`PROCEDURE_TRAVEL` …). `GetCurrentAIProcedure` (`005a1210`) takes the
//! procedure the actor's current package is at and turns it into the
//! number scripts and conditions compare with, through a switch: the
//! numbers there are Oblivion's (travel 0, activate 1, acquire 2, wait 3,
//! dialogue 4 …) with New Vegas's own after them. Procedures the switch
//! has no case for give −1.

/// The procedures' names (`011a3cc0`), by the exe's number.
pub const NAMES: [&str; 55] = [
    "TRAVEL",
    "WANDER",
    "ACTIVATE",
    "ACQUIRE",
    "SLEEP",
    "EAT",
    "FOLLOW",
    "ESCORT",
    "ALARM",
    "COMBAT",
    "FLEE",
    "YIELD",
    "DIALOGUE",
    "WAIT",
    "TRAVEL_TARGET",
    "PURSUE",
    "GREET",
    "CREATE_FOLLOW",
    "OBSERVE_COMBAT",
    "OBSERVE_DIALOGUE",
    "GREET_DEAD",
    "WARN",
    "GET_UP",
    "MOUNT_HORSE",
    "DISMOUNT_HORSE",
    "DO_NOTHING",
    "NOTIFY",
    "ACCOMPANY",
    "USE_ITEM_AT",
    "FEED",
    "AMBUSH_WAIT",
    "SURFACE",
    "WAIT_FOR_SPELL",
    "CHOOSE_CAST",
    "FLEE_NON_COMBAT",
    "REMOVE_WORN_ITEMS",
    "SEARCH",
    "CLEAR_MOUNT_POSITION",
    "SUMMON_CREATURE_DEFEND",
    "AVOID_RADIATION",
    "UNEQUIP_ARMOR",
    "TAKE_BACK_ITEM",
    "SANDBOX",
    "USE_IDLE_MARKER",
    "PATROL",
    "EXPLOSION_REACTION",
    "GRENADE_MINE_PICKUP_THROW",
    "GUARD",
    "ALERT_SEARCH",
    "DIALOGUE_ACTIVATE",
    "USE_WEAPON",
    "MOVEMENT_BLOCKED",
    "CANNIBAL_FEED",
    "BACK_UP",
    "DONE",
];

/// The exe's numbers of the procedures nv-rs carries out in some form.
pub const TRAVEL: u8 = 0;
pub const WANDER: u8 = 1;
pub const SLEEP: u8 = 4;
pub const EAT: u8 = 5;
pub const FOLLOW: u8 = 6;
pub const COMBAT: u8 = 9;
pub const FLEE: u8 = 10;
pub const DIALOGUE: u8 = 12;
pub const WAIT: u8 = 13;
pub const ACCOMPANY: u8 = 27;
pub const SANDBOX: u8 = 42;
pub const USE_IDLE_MARKER: u8 = 43;
pub const DONE: u8 = 54;

/// What `GetCurrentAIProcedure` gives for a procedure (`005a1210`'s
/// switch); `None` for the sandbox procedure, whose number depends on
/// where the sandbox is ([`sandbox_number`]). The engine's combat
/// package's flee state also gives 16 (the switch's special case: package
/// type 0x12 with `00981990`).
pub fn script_number(procedure: u8) -> Option<i32> {
    Some(match procedure {
        0 => 0,
        1 => 7,
        2 => 1,
        3 => 2,
        4 => 8,
        5 => 10,
        6 => 11,
        7 => 12,
        8 => 14,
        9 => 13,
        10 => 16,
        11 => 18,
        12 => 4,
        13 => 3,
        14 => 19,
        15 => 15,
        16 => 5,
        17 => 20,
        18 => 9,
        20 => 6,
        22 => 21,
        23 => 22,
        24 => 23,
        25 => 24,
        27 => 27,
        28 => 28,
        29 => 29,
        30 => 30,
        31 => 31,
        32 => 32,
        33 => 33,
        34 => 34,
        35 => 35,
        36 => 36,
        37 => 37,
        38 => 38,
        39 => 39,
        40 => 40,
        41 => 47,
        42 => return None,
        43 => 46,
        44 => 41,
        47 => 44,
        49 => 43,
        50 => 42,
        51 => 49,
        52 => 50,
        54 => 17,
        _ => -1,
    })
}

/// `GetCurrentAIProcedure` during a sandbox package (the switch's case for
/// `PROCEDURE_SANDBOX`, reading the procedure's data, process vtable
/// +0x274): its state (`crate::sandbox::Phase` as numbered: 0 just begun,
/// 1 getting up, 2 on the way, 3 doing it) by the table at `010357b8` (45,
/// 21, 0, −1), and while doing it the activity (`crate::sandbox::
/// activities`, −1 none) by the table at `010357c8` indexed by activity + 1
/// (45, 48 sitting, 8 sleeping, 10 eating, 7 wandering, 46 at an idle
/// marker, 4 talking). Out of range: 45.
pub fn sandbox_number(phase: u8, activity: Option<u8>) -> i32 {
    const BY_PHASE: [i32; 4] = [45, 21, 0, -1];
    const BY_ACTIVITY: [i32; 7] = [45, 48, 8, 10, 7, 46, 4];
    if phase == 3 {
        let i = activity.map_or(0, |a| usize::from(a) + 1);
        return BY_ACTIVITY.get(i).copied().unwrap_or(45);
    }
    BY_PHASE.get(usize::from(phase)).copied().unwrap_or(45)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_numbers_scripts_see() {
        assert_eq!(NAMES[usize::from(SANDBOX)], "SANDBOX");
        assert_eq!(NAMES[usize::from(DONE)], "DONE");
        // Dialogue 4 and eating 10, as the idle tree asks them.
        assert_eq!(script_number(DIALOGUE), Some(4));
        assert_eq!(script_number(EAT), Some(10));
        assert_eq!(script_number(WANDER), Some(7));
        assert_eq!(script_number(COMBAT), Some(13));
        // No case in the switch.
        assert_eq!(script_number(21), Some(-1));
        assert_eq!(script_number(SANDBOX), None);
        assert_eq!(sandbox_number(3, Some(2)), 10);
        assert_eq!(sandbox_number(3, Some(5)), 4);
        assert_eq!(sandbox_number(3, None), 45);
        assert_eq!(sandbox_number(1, Some(2)), 21);
    }
}
