//! `NV_COOP=journal`: M1's check on a PC (docs/coop/PLAN.md). Arms the change journal after the
//! game's first frame and, once a second, prints each quest whose stages changed (the flag
//! `SetStageDone` adds) and how many other forms changed. Starting a new game and leaving Doc
//! Mitchell's house shows the opening quest's lines. Removed once M3's session drives the journal.

use std::time::{Duration, Instant};

use engine_core::prelude::*;

use crate::{addr, journal};

/// Whether this run asks for the journal log: `NV_COOP=journal`.
pub fn wanted() -> bool {
    std::env::var("NV_COOP").is_ok_and(|v| v == "journal")
}

/// The log's state between frames.
#[derive(Debug, Default)]
pub struct JournalLog {
    frames: u64,
    last: Option<Instant>,
}

impl JournalLog {
    /// Call after each of main's frames, with main's engine (no frame in flight). Returns the
    /// lines to print.
    pub fn after_frame(&mut self, e: &mut Engine) -> Vec<String> {
        self.frames += 1;
        if self.frames == 1 {
            journal::arm(e, true);
        }
        if self
            .last
            .is_some_and(|at| at.elapsed() < Duration::from_secs(1))
        {
            return Vec::new();
        }
        self.last = Some(Instant::now());
        lines(&journal::drain(e))
    }
}

/// One line per quest whose stages changed, then one counting the other forms (none when nothing changed).
pub fn lines(changes: &[journal::Change]) -> Vec<String> {
    let mut out: Vec<String> = changes
        .iter()
        .filter(|c| c.kind == addr::FORM_TYPE_QUEST && c.flags & addr::CHANGE_QUEST_STAGES != 0)
        .map(|c| {
            format!(
                "coop journal: quest {:08x} stages changed (flags {:08x})",
                c.form, c.flags
            )
        })
        .collect();
    let others = changes.len() - out.len();
    if others > 0 {
        out.push(format!("coop journal: {others} other forms changed"));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::journal::Change;

    #[test]
    fn quest_stage_changes_are_named_and_the_rest_counted() {
        let c = |form, kind, flags| Change {
            form,
            kind,
            flags,
            forced: false,
            cause: None,
        };
        assert!(lines(&[]).is_empty());
        assert_eq!(
            lines(&[
                c(
                    0x0010_4c0c,
                    addr::FORM_TYPE_QUEST,
                    addr::CHANGE_QUEST_STAGES | 2
                ),
                c(0x14, 0x3b, 4),
                // The same bit on a cell is its seen data, not a quest's stages.
                c(0x0100_2ad1, 0x39, addr::CHANGE_QUEST_STAGES)
            ]),
            [
                "coop journal: quest 00104c0c stages changed (flags 80000002)",
                "coop journal: 2 other forms changed"
            ]
        );
    }

    #[test]
    fn the_first_frame_arms_the_journal() {
        let mut e = Engine::new();
        let mut log = JournalLog::default();
        assert!(!journal::armed(&mut e));
        assert!(log.after_frame(&mut e).is_empty());
        assert!(journal::armed(&mut e));
    }
}
