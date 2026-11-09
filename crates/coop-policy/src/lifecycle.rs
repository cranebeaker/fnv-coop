//! Death, respawn and late join as pure decisions.

use crate::engine::{PlayerMask, Slot};
use crate::load::PolicyTable;
use crate::model::*;

/// What happens to a player whose health reached zero.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DownOutcome {
    /// Normal death (mode `normal`).
    Die,
    /// Downed on the spot, revived by a teammate within `revive_s` seconds.
    Downed { revive_s: u32 },
    /// Down until the party reaches one of the quest's checkpoint cells (shadow `down`).
    UntilCheckpoint { checkpoints: Vec<String> },
    /// Straight back at the zone checkpoint (mode `respawn`).
    Respawn,
}

/// The outcome of a down while `row` (if any) is the active quest.
pub fn down_outcome(t: &PolicyTable, row: Option<&QuestRow>) -> DownOutcome {
    if let Some(sh) = row.and_then(|r| r.shadow.as_ref()) {
        if sh.down.is_some() {
            return DownOutcome::UntilCheckpoint {
                checkpoints: sh.checkpoint.clone(),
            };
        }
    }
    let d = &t.policy().death;
    match d.mode {
        DeathMode::Normal => DownOutcome::Die,
        DeathMode::Downed => DownOutcome::Downed {
            revive_s: d.revive_s,
        },
        DeathMode::Respawn => DownOutcome::Respawn,
    }
}

/// What to do when every present player is down.
pub fn wipe_outcome(t: &PolicyTable) -> AllDown {
    t.policy().death.all_down
}

/// The whole party is down when every present player is in `down`.
pub fn is_wipe(present: PlayerMask, down: PlayerMask) -> bool {
    !present.is_empty() && present.is_subset_of(down)
}

/// A joiner's placement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoinPlan {
    /// Held out of `present` until the scene ends.
    WaitForScene,
    /// Refused while a scene plays.
    Refuse,
    /// Warp into the party's zone.
    WarpToZone,
    /// Stay where the join put them.
    StayPut,
}

pub fn join_plan(t: &PolicyTable, scene_active: bool) -> JoinPlan {
    let l = &t.policy().late_join;
    if scene_active {
        match l.during_scene {
            LateJoinDuring::Wait => JoinPlan::WaitForScene,
            LateJoinDuring::Refuse => JoinPlan::Refuse,
        }
    } else {
        match l.after_scene {
            LateJoinAfter::WarpToZone => JoinPlan::WarpToZone,
            LateJoinAfter::StayPut => JoinPlan::StayPut,
        }
    }
}

/// Slots whose revive window (`revive_s`) ran out.
pub fn expired(down_since: &[(Slot, u32)], revive_s: u32) -> Vec<Slot> {
    down_since
        .iter()
        .filter(|(_, e)| *e >= revive_s)
        .map(|(s, _)| *s)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bundled_table_outcomes() {
        let t = PolicyTable::bundled().unwrap();
        assert!(matches!(
            down_outcome(&t, None),
            DownOutcome::Downed { .. } | DownOutcome::Respawn | DownOutcome::Die
        ));
        let r = t.row_by_edid("NVDLC01MQ02").unwrap();
        assert!(matches!(
            down_outcome(&t, Some(r)),
            DownOutcome::UntilCheckpoint { .. }
        ));
        let both = PlayerMask::of(0).or(PlayerMask::of(1));
        assert!(is_wipe(both, both));
        assert!(!is_wipe(both, PlayerMask::of(1)));
        assert_eq!(expired(&[(1, 30), (2, 61)], 60), vec![2]);
        assert_eq!(join_plan(&t, true), JoinPlan::WaitForScene);
    }
}
