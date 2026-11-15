//! The game's clock: six globals and a count of midnights.
//!
//! The host's game runs its own clock. On a guest the program's clock advance does nothing
//! (the glue overrides it), and the adapter's apply is the one writer of the clock. So a
//! guest's clock at a revision is the host's clock at that revision, bit for bit.
//!
//! [`Guard`] is a guest's check: before each frame it compares the clock with what the
//! adapter left. A difference is a leak (a local `set`, or an advance that the override did
//! not see). The guard reports it and writes the adapter's values back.

use crate::addr;
use crate::program::Program;

/// The clock's globals, in the order of the program's clock object: the year, the month,
/// the day, the hour, the days passed and the time scale.
pub const GLOBALS: [u32; 6] = [0x35, 0x36, 0x37, 0x38, 0x39, 0x3a];

/// The global that carries the count of midnights as a shared slot: `GameDaysPassed`.
pub const COUNT_FORM: u32 = 0x39;

/// The clock's words: the bits of the six globals' values, then the count of midnights.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Words(pub [u32; 7]);

/// One word of the clock that was not what the adapter left.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClockLeak {
    /// The global's form id, or 0 for the count of midnights.
    pub form: u32,
    /// The word that the adapter left.
    pub left: u32,
    /// The word that the program had.
    pub found: u32,
}

/// The addresses of the six clock globals, from the program's clock object.
/// - `Ok(None)`: the program has no clock object (its pointers are zero).
/// - `Err`: the object does not point at the forms of the clock globals. The layout is not
///   the one that the adapter knows, and the adapter must not write there.
pub fn globals(p: &mut dyn Program) -> Result<Option<[u32; 6]>, String> {
    let mut at = [0u32; 6];
    for (slot, pointer) in at.iter_mut().enumerate() {
        *pointer = p.u32(addr::CLOCK + 4 * slot as u32);
    }
    if at.iter().all(|a| *a == 0) {
        return Ok(None);
    }
    for (slot, id) in GLOBALS.iter().enumerate() {
        let form = p.call(addr::FORM_BY_ID, &[*id]).eax;
        if form == 0 || form != at[slot] {
            return Err(format!(
                "the clock object's pointer {slot} is {:08x}, and the global {id:08X} is at \
                 {form:08x}",
                at[slot]
            ));
        }
    }
    Ok(Some(at))
}

/// The count of midnights.
pub fn midnights(p: &dyn Program) -> u32 {
    p.u32(addr::CLOCK + addr::CLOCK_MIDNIGHTS)
}

/// Writes the count of midnights. True when it changed.
pub fn set_midnights(p: &mut dyn Program, count: u32) -> bool {
    let changed = midnights(p) != count;
    if changed {
        p.set_u32(addr::CLOCK + addr::CLOCK_MIDNIGHTS, count);
    }
    changed
}

/// The clock's words now.
pub fn read(p: &dyn Program, globals: &[u32; 6]) -> Words {
    let mut words = [0u32; 7];
    for (word, at) in words.iter_mut().zip(globals) {
        *word = p.u32(at + addr::GLOBAL_VALUE);
    }
    words[6] = midnights(p);
    Words(words)
}

/// A guest's check of its clock.
#[derive(Clone, Debug, Default)]
pub struct Guard {
    globals: Option<[u32; 6]>,
    left: Option<Words>,
    /// The leaks that the guard found and repaired.
    pub leaks: u64,
}

impl Guard {
    /// A guard for a clock with these globals (`None`: the program has no clock object, and
    /// the guard does nothing).
    pub fn new(globals: Option<[u32; 6]>) -> Guard {
        Guard {
            globals,
            ..Guard::default()
        }
    }

    /// The adapter wrote the clock, or took the game over: the clock's words are right now.
    pub fn note(&mut self, p: &dyn Program) {
        if let Some(globals) = &self.globals {
            self.left = Some(read(p, globals));
        }
    }

    /// Compares the clock with what the adapter left. Each word that differs is a leak: it
    /// gets the adapter's value again.
    pub fn check(&mut self, p: &mut dyn Program) -> Vec<ClockLeak> {
        let (Some(globals), Some(left)) = (self.globals, self.left) else {
            return Vec::new();
        };
        let now = read(p, &globals);
        let mut leaks = Vec::new();
        for slot in 0..7 {
            if now.0[slot] == left.0[slot] {
                continue;
            }
            leaks.push(ClockLeak {
                form: GLOBALS.get(slot).copied().unwrap_or(0),
                left: left.0[slot],
                found: now.0[slot],
            });
            match globals.get(slot) {
                Some(at) => p.set_u32(at + addr::GLOBAL_VALUE, left.0[slot]),
                None => p.set_u32(addr::CLOCK + addr::CLOCK_MIDNIGHTS, left.0[slot]),
            }
        }
        self.leaks += leaks.len() as u64;
        leaks
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake::FakeProgram;

    const HOUR: u32 = 0x38;

    fn hour(p: &mut FakeProgram) -> f32 {
        let at = p.call(addr::FORM_BY_ID, &[HOUR]).eax;
        p.f32(at + addr::GLOBAL_VALUE)
    }

    fn advance(p: &mut FakeProgram, seconds: f32) {
        p.call(addr::CLOCK_ADVANCE, &[addr::CLOCK, seconds.to_bits()]);
    }

    #[test]
    fn the_clock_object_gives_the_six_globals_or_an_error() {
        let mut p = FakeProgram::new();
        assert_eq!(globals(&mut p), Ok(None), "no clock object");
        p.add_clock(12.0);
        let at = globals(&mut p).unwrap().expect("the clock");
        assert_eq!(at[3], p.call(addr::FORM_BY_ID, &[HOUR]).eax);
        // A pointer that is not the global's form is another layout.
        p.set_u32(addr::CLOCK + 4 * 3, at[2]);
        let why = globals(&mut p).unwrap_err();
        assert!(why.contains("pointer 3"), "{why}");
    }

    /// The fake's advance is the program's: time scale 30, a midnight moves the date and the
    /// count. A blocked advance moves nothing.
    #[test]
    fn the_advance_moves_the_clock_and_a_block_stops_it() {
        let mut p = FakeProgram::new();
        p.add_clock(12.0);
        advance(&mut p, 120.0);
        assert_eq!(hour(&mut p), 13.0, "120 seconds at time scale 30");
        advance(&mut p, 11.5 * 120.0);
        assert!((hour(&mut p) - 0.5).abs() < 1e-4, "{}", hour(&mut p));
        assert_eq!(midnights(&p), 1);
        let at = globals(&mut p).unwrap().unwrap();
        let before = read(&p, &at);
        p.clock_blocked = true;
        advance(&mut p, 3600.0);
        assert_eq!(read(&p, &at), before);
    }

    /// The guard finds a word that the adapter did not leave, reports it one time, and
    /// writes the adapter's value back.
    #[test]
    fn the_guard_reports_a_leak_and_repairs_it() {
        let mut p = FakeProgram::new();
        p.add_clock(12.0);
        let at = globals(&mut p).unwrap();
        let mut guard = Guard::new(at);
        assert!(guard.check(&mut p).is_empty(), "nothing was left yet");
        guard.note(&p);
        assert!(guard.check(&mut p).is_empty());
        // The program's own code moves the hour and the count.
        advance(&mut p, 12.5 * 120.0);
        let leaks = guard.check(&mut p);
        let forms: Vec<u32> = leaks.iter().map(|l| l.form).collect();
        assert!(forms.contains(&HOUR) && forms.contains(&0), "{leaks:?}");
        assert_eq!(hour(&mut p), 12.0, "the hour is the adapter's again");
        assert_eq!(midnights(&p), 0);
        assert!(guard.check(&mut p).is_empty(), "one report for one leak");
        assert_eq!(guard.leaks, leaks.len() as u64);
        // The twin: a guard with no clock object reports nothing.
        let mut none = Guard::new(None);
        none.note(&p);
        advance(&mut p, 120.0);
        assert!(none.check(&mut p).is_empty());
    }
}
