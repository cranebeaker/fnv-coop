//! Quest state in the program's memory: stages, done stages, flags, objectives.
//!
//! The export gives the shared fields in the encoding that each game adapter uses
//! (`coop_core::schema`). The apply writes a field back as plain state: no script runs.

use coop_core::ids::FormId;
use coop_core::schema::{flag, tag, Field};

use crate::addr;
use crate::program::Program;

/// The most nodes a list walk follows. A quest has at most 256 stages. The bound stops a walk
/// in damaged memory.
const MAX_NODES: usize = 1024;

/// The items of the list whose head node is at `head`.
pub(crate) fn list_items(p: &dyn Program, head: u32) -> Vec<u32> {
    let mut items = Vec::new();
    let mut node = head;
    while node != 0 && items.len() < MAX_NODES {
        let item = p.u32(node + addr::NODE_ITEM);
        let next = p.u32(node + addr::NODE_NEXT);
        if item != 0 {
            items.push(item);
        }
        node = next;
    }
    items
}

/// The quest's stage items: (item address, stage number), in the list's order.
pub fn stage_items(p: &dyn Program, quest: u32) -> Vec<(u32, u8)> {
    list_items(p, quest + addr::QUEST_STAGES)
        .into_iter()
        .map(|item| (item, p.u8(item + addr::STAGE_NUMBER)))
        .collect()
}

/// The quest's objectives: (objective address, id), in the list's order.
pub fn objective_items(p: &dyn Program, quest: u32) -> Vec<(u32, u32)> {
    list_items(p, quest + addr::QUEST_OBJECTIVES)
        .into_iter()
        .map(|item| (item, p.u32(item + addr::OBJECTIVE_ID)))
        .collect()
}

/// The quest's script variables: (variable address, id), in the list's order. Empty when the
/// quest has no event list.
pub fn variable_items(p: &dyn Program, quest: u32) -> Vec<(u32, u32)> {
    let events = p.u32(quest + addr::QUEST_EVENT_LIST);
    if events == 0 {
        return Vec::new();
    }
    list_items(p, p.u32(events + addr::EVENT_LIST_VARS))
        .into_iter()
        .map(|item| (item, p.u32(item + addr::VAR_ID)))
        .collect()
}

/// A quest's state, read from the program.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct QuestState {
    /// The current stage.
    pub stage: u8,
    /// The flags byte ([`addr::quest_flag`]).
    pub flags: u8,
    /// The done stages, sorted.
    pub done: Vec<u8>,
    /// The objectives with a status above 0: (id, status bits), sorted by id.
    pub objectives: Vec<(u32, u8)>,
    /// The script variables that are not 0: (id, the bits of the value), sorted by id.
    pub variables: Vec<(u32, u64)>,
}

/// Reads the state of the quest at `quest`.
pub fn read(p: &dyn Program, quest: u32) -> QuestState {
    let mut done: Vec<u8> = stage_items(p, quest)
        .into_iter()
        .filter(|(item, _)| p.u8(item + addr::STAGE_DONE) != 0)
        .map(|(_, n)| n)
        .collect();
    done.sort_unstable();
    done.dedup();
    let mut objectives: Vec<(u32, u8)> = objective_items(p, quest)
        .into_iter()
        .map(|(item, id)| (id, (p.u32(item + addr::OBJECTIVE_STATUS) & 3) as u8))
        .filter(|(_, status)| *status != 0)
        .collect();
    objectives.sort_unstable();
    let mut variables: Vec<(u32, u64)> = variable_items(p, quest)
        .into_iter()
        .map(|(item, id)| (id, u64_at(p, item + addr::VAR_VALUE)))
        .filter(|(_, bits)| *bits != 0)
        .collect();
    variables.sort_unstable();
    QuestState {
        stage: p.u8(quest + addr::QUEST_STAGE),
        flags: p.u8(quest + addr::QUEST_FLAGS),
        done,
        objectives,
        variables,
    }
}

fn u64_at(p: &dyn Program, at: u32) -> u64 {
    let mut b = [0; 8];
    p.read(at, &mut b);
    u64::from_le_bytes(b)
}

/// The shared flag bits of a quest's flags byte.
fn shared_flags(flags: u8) -> u32 {
    let mut bits = 0;
    if flags & addr::quest_flag::RUNNING != 0 {
        bits |= flag::RUNNING;
    }
    if flags & addr::quest_flag::COMPLETED != 0 {
        bits |= flag::COMPLETED;
    }
    if flags & addr::quest_flag::FAILED != 0 {
        bits |= flag::FAILED;
    }
    bits
}

/// The flags byte with its shared bits set from the shared flag bits. The other bits stay.
fn with_shared_flags(old: u8, shared: u32) -> u8 {
    let pairs = [
        (flag::RUNNING, addr::quest_flag::RUNNING),
        (flag::COMPLETED, addr::quest_flag::COMPLETED),
        (flag::FAILED, addr::quest_flag::FAILED),
    ];
    pairs.iter().fold(old, |byte, (bit, mask)| {
        if shared & bit != 0 {
            byte | mask
        } else {
            byte & !mask
        }
    })
}

/// The shared fields of a quest's state. A quest that never had a stage set gives no stage
/// field: a stage field means "a stage was set", as in each adapter.
pub fn fields(s: &QuestState) -> Vec<Field> {
    let mut out = Vec::new();
    if s.stage != 0 || !s.done.is_empty() {
        out.push(Field::QuestStage {
            stage: u16::from(s.stage),
        });
    }
    let mut chunks: std::collections::BTreeMap<u16, Vec<u8>> = Default::default();
    for n in &s.done {
        let n = u16::from(*n);
        chunks
            .entry(n / tag::STAGES_CHUNK)
            .or_default()
            .extend(n.to_le_bytes());
    }
    for (chunk, data) in chunks {
        out.push(Field::Custom {
            tag: tag::STAGES_BASE + chunk,
            data,
        });
    }
    out.push(Field::QuestFlags {
        flags: shared_flags(s.flags),
    });
    for (id, status) in &s.objectives {
        if let Ok(index) = u16::try_from(*id) {
            out.push(Field::Objective {
                index,
                state: *status,
            });
        }
    }
    for (id, bits) in &s.variables {
        if let Ok(index) = u16::try_from(*id) {
            out.push(Field::ScriptVar { index, bits: *bits });
        }
    }
    out.retain(|f| !f.is_tombstone());
    out
}

/// Whether a `Custom` tag is a chunk of done stages.
fn stages_chunk(tag_: u16) -> Option<u16> {
    // The chunks of 256 stage numbers fit below the next tag block.
    (tag::STAGES_BASE..tag::STAGES_BASE + 0x100)
        .contains(&tag_)
        .then(|| tag_ - tag::STAGES_BASE)
}

/// Writes one shared field of the quest at `quest` as plain state. True when memory changed.
/// A field of another kind changes nothing.
pub fn apply(p: &mut dyn Program, quest: u32, field: &Field) -> bool {
    match field {
        Field::QuestStage { stage } => {
            let stage = u8::try_from(*stage).unwrap_or(u8::MAX);
            let changed = p.u8(quest + addr::QUEST_STAGE) != stage;
            if changed {
                p.set_u8(quest + addr::QUEST_STAGE, stage);
            }
            changed
        }
        Field::QuestFlags { flags } => {
            let old = p.u8(quest + addr::QUEST_FLAGS);
            let new = with_shared_flags(old, *flags);
            if new != old {
                p.set_u8(quest + addr::QUEST_FLAGS, new);
            }
            new != old
        }
        Field::Objective { index, state } => {
            let Some((item, _)) = objective_items(p, quest)
                .into_iter()
                .find(|(_, id)| *id == u32::from(*index))
            else {
                return false;
            };
            let old = p.u32(item + addr::OBJECTIVE_STATUS);
            let new = (old & !3) | u32::from(*state & 3);
            if new != old {
                // The game's own setter puts the objective in the player's objective list,
                // which the Pip-Boy's quest page walks, and shows the HUD's notice. A status
                // that is only written is not in the Pip-Boy. The host's value is the last word.
                p.call(addr::OBJECTIVE_SET_STATE, &[item, u32::from(*state & 3)]);
                p.set_u32(item + addr::OBJECTIVE_STATUS, new);
            }
            new != old
        }
        Field::ScriptVar { index, bits } => {
            let Some((item, _)) = variable_items(p, quest)
                .into_iter()
                .find(|(_, id)| *id == u32::from(*index))
            else {
                return false;
            };
            let changed = u64_at(p, item + addr::VAR_VALUE) != *bits;
            if changed {
                p.write(item + addr::VAR_VALUE, &bits.to_le_bytes());
            }
            changed
        }
        Field::Custom { tag: t, data } => {
            let Some(chunk) = stages_chunk(*t) else {
                return false;
            };
            let done: Vec<u16> = data
                .chunks_exact(2)
                .map(|b| u16::from_le_bytes([b[0], b[1]]))
                .collect();
            let mut changed = false;
            for (item, n) in stage_items(p, quest) {
                let n = u16::from(n);
                if n / tag::STAGES_CHUNK != chunk {
                    continue;
                }
                let want = u8::from(done.contains(&n));
                if p.u8(item + addr::STAGE_DONE) != want {
                    p.set_u8(item + addr::STAGE_DONE, want);
                    changed = true;
                }
            }
            changed
        }
        _ => false,
    }
}

/// The objectives in the player's objective list (the Pip-Boy's quest page), in its order.
/// Empty when the program has no player.
pub fn player_objectives(p: &dyn Program) -> Vec<u32> {
    match p.u32(addr::PLAYER) {
        0 => Vec::new(),
        player => list_items(p, player + addr::PLAYER_OBJECTIVES),
    }
}

/// Puts into the player's objective list each objective of the quest at `quest` that is shown
/// (a status above 0) and not in the list, with the game's own function. The count of the
/// objectives put back. The list is the player's, not the quest's: a game that empties it
/// keeps the objectives' states, and the Pip-Boy would never show them again.
pub fn relist(p: &mut dyn Program, quest: u32) -> u32 {
    let player = p.u32(addr::PLAYER);
    if player == 0 {
        return 0;
    }
    let listed = player_objectives(p);
    let missing: Vec<u32> = objective_items(p, quest)
        .into_iter()
        .map(|(item, _)| item)
        .filter(|item| p.u32(item + addr::OBJECTIVE_STATUS) & 3 != 0)
        .filter(|item| !listed.contains(item))
        .collect();
    for item in &missing {
        p.call(addr::PLAYER_UPDATE_OBJECTIVE, &[player, *item]);
    }
    missing.len() as u32
}

/// The key's form as the program's form id.
pub fn form_id(form: FormId) -> u32 {
    form.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake::FakeProgram;

    const QUEST: u32 = 0x0015_d912;

    fn program() -> (FakeProgram, u32) {
        let mut p = FakeProgram::new();
        let quest = p.add_quest(QUEST, &[0, 10, 20, 250], &[10, 20]);
        (p, quest)
    }

    #[test]
    fn a_fresh_quest_exports_nothing() {
        let (p, quest) = program();
        assert_eq!(read(&p, quest), QuestState::default());
        assert!(fields(&read(&p, quest)).is_empty());
    }

    #[test]
    fn a_set_stage_shows_in_the_exported_fields() {
        let (mut p, quest) = program();
        assert!(p.call(addr::QUEST_SET_STAGE, &[quest, 10]).bool());
        assert!(!p.call(addr::QUEST_SET_STAGE, &[quest, 11]).bool());
        let s = read(&p, quest);
        assert_eq!((s.stage, s.flags, s.done.as_slice()), (10, 1, &[10u8][..]));
        let f = fields(&s);
        assert!(f.contains(&Field::QuestStage { stage: 10 }));
        assert!(f.contains(&Field::QuestFlags {
            flags: flag::RUNNING
        }));
        assert!(f.contains(&Field::Custom {
            tag: tag::STAGES_BASE,
            data: vec![10, 0],
        }));
    }

    /// Stage 0 set by a script is a stage field too: the done stage shows that it was set.
    #[test]
    fn stage_zero_counts_as_set_once_it_is_done() {
        let (mut p, quest) = program();
        p.call(addr::QUEST_SET_STAGE, &[quest, 0]);
        assert!(fields(&read(&p, quest)).contains(&Field::QuestStage { stage: 0 }));
    }

    /// The fields of one program, applied to a second, give the same state there, and a
    /// second apply changes nothing. Mutation twin: without the apply the states differ.
    #[test]
    fn the_exported_fields_rebuild_the_state_on_a_second_program() {
        let (mut host, hq) = program();
        host.call(addr::QUEST_SET_STAGE, &[hq, 10]);
        host.call(addr::QUEST_SET_STAGE, &[hq, 250]);
        let objective = objective_items(&host, hq)[1].0;
        host.set_u32(objective + addr::OBJECTIVE_STATUS, 3);
        host.set_u8(hq + addr::QUEST_FLAGS, 3);
        let want = read(&host, hq);
        assert_eq!(want.done, [10, 250]);
        assert_eq!(want.objectives, [(20, 3)]);

        let (mut guest, gq) = program();
        assert_ne!(read(&guest, gq), want, "the twin: no apply, no state");
        let f = fields(&want);
        let changed: Vec<bool> = f.iter().map(|x| apply(&mut guest, gq, x)).collect();
        assert!(changed.iter().all(|c| *c), "{changed:?}");
        assert_eq!(read(&guest, gq), want);
        assert!(guest.stages_set.is_empty(), "no SetStage ran on the guest");
        assert!(f.iter().all(|x| !apply(&mut guest, gq, x)), "idempotent");
    }

    /// A tombstone clears the slot: the flags go, an objective hides, a chunk of done stages
    /// empties. The stage goes down with a lower stage field.
    #[test]
    fn a_tombstone_and_a_lower_stage_return_the_state() {
        let (mut p, quest) = program();
        p.call(addr::QUEST_SET_STAGE, &[quest, 20]);
        let objective = objective_items(&p, quest)[0].0;
        p.set_u32(objective + addr::OBJECTIVE_STATUS, 1);
        for f in fields(&read(&p, quest)) {
            let cleared = f.tombstone();
            if cleared != f {
                assert!(apply(&mut p, quest, &cleared), "{f:?}");
            }
        }
        assert!(apply(&mut p, quest, &Field::QuestStage { stage: 0 }));
        assert_eq!(read(&p, quest), QuestState::default());
    }

    /// An objective that a guest takes from the host goes through the game's own setter, so
    /// it is in the player's objective list (the Pip-Boy's quest page). A second apply calls
    /// nothing. Mutation twin: a status that is only written is not in the list, and the
    /// relist puts it there once.
    #[test]
    fn an_applied_objective_is_in_the_players_list() {
        let (mut p, quest) = program();
        p.add_player([0.0; 3]);
        let shown = Field::Objective {
            index: 20,
            state: 1,
        };
        assert!(apply(&mut p, quest, &shown));
        let item = objective_items(&p, quest)[1].0;
        assert_eq!(player_objectives(&p), [item]);
        let calls = p.calls.len();
        assert!(!apply(&mut p, quest, &shown));
        assert_eq!(p.calls.len(), calls, "nothing to set");
        assert_eq!(relist(&mut p, quest), 0);

        let (mut twin, tq) = program();
        twin.add_player([0.0; 3]);
        let first = objective_items(&twin, tq)[0].0;
        twin.set_u32(first + addr::OBJECTIVE_STATUS, 3);
        assert!(
            player_objectives(&twin).is_empty(),
            "the twin: only written"
        );
        assert_eq!(relist(&mut twin, tq), 1);
        assert_eq!(player_objectives(&twin), [first]);
        assert_eq!(relist(&mut twin, tq), 0);
        let mut no_player = FakeProgram::new();
        let q = no_player.add_quest(QUEST, &[10], &[10]);
        assert_eq!(relist(&mut no_player, q), 0);
    }

    /// Bits of the flags byte that are not shared stay as the program has them.
    #[test]
    fn flags_that_are_not_shared_stay() {
        let (mut p, quest) = program();
        p.set_u8(quest + addr::QUEST_FLAGS, 0x10 | 1);
        assert!(apply(&mut p, quest, &Field::QuestFlags { flags: 0 }));
        assert_eq!(p.u8(quest + addr::QUEST_FLAGS), 0x10);
    }

    /// A quest's script variables cross as fields. A variable at 0 is not a field, and its
    /// tombstone writes 0. A quest with no event list has no variables and takes none.
    #[test]
    fn script_variables_cross_as_fields() {
        let (mut host, hq) = program();
        let vars = host.add_variables(hq, &[1, 2, 7]);
        host.write(vars[1] + addr::VAR_VALUE, &2.5f64.to_le_bytes());
        let want = Field::script_var(2, 2.5);
        let f = fields(&read(&host, hq));
        assert_eq!(
            f.iter()
                .filter(|x| matches!(x, Field::ScriptVar { .. }))
                .count(),
            1,
            "{f:?}"
        );
        assert!(f.contains(&want));

        let (mut guest, gq) = program();
        assert!(!apply(&mut guest, gq, &want), "no event list, no variable");
        guest.add_variables(gq, &[1, 2, 7]);
        assert!(apply(&mut guest, gq, &want));
        assert!(!apply(&mut guest, gq, &want), "idempotent");
        assert_eq!(read(&guest, gq).variables, [(2, 2.5f64.to_bits())]);
        assert!(apply(&mut guest, gq, &want.tombstone()));
        assert!(read(&guest, gq).variables.is_empty());
        assert!(
            !apply(&mut guest, gq, &Field::script_var(9, 1.0)),
            "unknown id"
        );
    }

    /// A failed quest has the program's two bits, and exports as failed and completed. The
    /// fields put the same two bits on a second program.
    #[test]
    fn a_failed_quest_crosses_with_both_bits() {
        let (mut host, hq) = program();
        host.set_u8(hq + addr::QUEST_FLAGS, 0x40 | 2 | 0x10);
        let f = fields(&read(&host, hq));
        let want = Field::QuestFlags {
            flags: flag::COMPLETED | flag::FAILED,
        };
        assert!(f.contains(&want), "{f:?}");
        let (mut guest, gq) = program();
        guest.set_u8(gq + addr::QUEST_FLAGS, 1);
        assert!(apply(&mut guest, gq, &want));
        assert_eq!(guest.u8(gq + addr::QUEST_FLAGS), 0x40 | 2);
    }
}
