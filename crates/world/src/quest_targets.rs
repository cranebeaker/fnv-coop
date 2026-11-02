//! Quest targets: where the compass and the maps point for the active
//! quest's shown objectives (`QSTA`, [`crate::quest::QuestTarget`]).
//!
//! Traced in FalloutNV.exe 1.4.0.525 (names from the Xbox PDB, marked):
//!
//! - The player keeps one active quest (`PlayerCharacter` +0x6b8,
//!   `PlayerCharacter::SetActiveQuest` (Xbox PDB) = `009529d0`). Showing an
//!   objective (`005ec5d0`, state 1) of a quest not completed makes that
//!   quest active when none is; `ForceActiveQuest` (`005d9ac0`) and the
//!   Pip-Boy set it; a quest completed or failed by a stage's log entry
//!   (`0060fb60`) or announced as such (`0077a480`) stops being active.
//! - The current target list (`PlayerCharacter::GetCurrentTargetList` (Xbox
//!   PDB) = `00952ba0`, list at +0x6c4): empty without an active quest;
//!   rebuilt (`TESQuest::UpdateCurrentTargetList` (Xbox PDB) = `0060f110`)
//!   when flagged (+0x206) or when it no longer matches the targets that
//!   pass (`TESQuest::ValidateTargetList` (Xbox PDB) = `0060efd0`). A
//!   completed quest (flag 0x02) has none. Otherwise, for each of the
//!   player's objectives (+0x6bc) belonging to the quest whose state is 1
//!   (shown, not completed: `BGSQuestObjective::eState` +0x20), each target
//!   whose conditions pass (`005ec500`: asked about the target reference,
//!   no second reference) is added, its path built from the player to it
//!   (`PlayerCharacter::BuildPathToTarget` (Xbox PDB) = `00952d60`).
//! - What is followed (`005cbb70`): the first door of that path
//!   (`TeleportPath::Doors[0].pDoor` (Xbox PDB), at target +0x20), else,
//!   with no doors (the same interior or worldspace, or no path), the
//!   target reference itself.
//! - The path (`006d4f70`): the game first tries its navmesh-level search
//!   (`006b8c50`, not translated) and otherwise searches doors
//!   (`TeleportDoorSearch` (Xbox PDB), `006f34e0`, vtable `0106d8fc`):
//!   from the player's place (an interior cell or a worldspace) each load
//!   door there (`0054db50`: the cell's references, or in a worldspace its
//!   persistent cell's (`00588270`), that are enabled, not deleted, whose
//!   base is a door and that have a destination) leads to its far side's
//!   place; a step costs the straight distance from where one stands
//!   (the start, or where the last door put one) to the door, plus 409600
//!   (`0106c238`) for a door the player can't get through
//!   (`00502450`: locked without the key, with lock disposition 2) and
//!   409600 for a door whose base has flag 0x08 (`00518000`, minimal use;
//!   read from `FNAM` here). The first place reached that is the
//!   target's ends it (`006f3b00`), cheapest first (`006f3fb0`). The
//!   navmesh-level search may pick other doors where it applies: that
//!   difference is unresolved.
//!
//! Not modelled: a deleted target's stand-in (`GetReference(true)`, extra
//! data 0x1c), the door base the game's list leaves out (`011ca258`, not
//! identified), ownership/faction branches of the lock check, and the
//! order of the player's objective list (`00952a20`; here the quest
//! record's objective order).

use std::collections::{BinaryHeap, HashMap};

use esm::{flags, sig, FormId, FourCC, LoadOrder};

use crate::dialogue::PLAYER_REF;
use crate::quest::{Quest, TARGET_IGNORES_LOCKS};
use crate::scripting::{Facts, GameState};

const DOOR: FourCC = FourCC::new(b"DOOR");
const FNAM: FourCC = FourCC::new(b"FNAM");

/// `0106c238`: what a door the player can't use, or a minimal-use door,
/// adds to a route.
pub const DOOR_PENALTY: f32 = 409_600.0;
/// `DOOR` `FNAM` flag 0x08, minimal use (`00518000`).
pub const MINIMAL_USE: u8 = 0x08;
/// `TESQuest` flag 0x02 (`0059e400`): the quest is completed.
const QUEST_COMPLETED: u8 = 0x02;

/// A target of the active quest's shown objectives whose conditions pass.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CurrentTarget {
    pub quest: FormId,
    pub objective: i32,
    pub reference: FormId,
    /// `QSTA` flag 0x01. `005ec500` passes it to `00952d60`, which never
    /// reads it (its third argument), so it changes nothing on PC.
    pub ignore_locks: bool,
}

/// An objective was shown (`005ec5d0`, state 1): its quest becomes the
/// active one when none is and the quest isn't completed.
pub fn objective_shown(state: &mut GameState, quest: FormId) {
    if state.active_quest.is_none() && !state.completed.contains(&quest) {
        state.active_quest = Some(quest);
    }
}

/// A quest was completed or failed (`0060fb60`, `0077a480`): it's no
/// longer the active one.
pub fn quest_ended(state: &mut GameState, quest: FormId) {
    if state.active_quest == Some(quest) {
        state.active_quest = None;
    }
}

/// The current target list (`00952ba0`): the active quest's shown,
/// uncompleted objectives' targets whose conditions pass.
pub fn current_targets(order: &LoadOrder, quest: &Quest, state: &GameState) -> Vec<CurrentTarget> {
    if state.active_quest != Some(quest.form_id)
        || state.completed.contains(&quest.form_id)
        || quest.flags & QUEST_COMPLETED != 0
    {
        return Vec::new();
    }
    let facts = Facts {
        order,
        state,
        speaker: None,
    };
    let mut out = Vec::new();
    for o in &quest.objectives {
        // State 1: shown and not completed.
        if state.objectives.get(&(quest.form_id, o.index)) != Some(&false) {
            continue;
        }
        for t in &o.targets {
            if t.reference.0 == 0 {
                continue;
            }
            // `00680c30`: the target reference is the subject, no target.
            if !facts.conditions_pass(&t.conditions, t.reference, FormId(0)) {
                continue;
            }
            out.push(CurrentTarget {
                quest: quest.form_id,
                objective: o.index,
                reference: t.reference,
                ignore_locks: t.flags & TARGET_IGNORES_LOCKS != 0,
            });
        }
    }
    out
}

/// A load door as the door search sees it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LoadDoor {
    pub door: FormId,
    pub position: [f32; 3],
    /// The place its far side is in (an interior cell or a worldspace),
    /// and where it puts one there.
    pub to_space: FormId,
    pub arrival: [f32; 3],
    pub minimal_use: bool,
}

/// Each place's load doors, read once.
#[derive(Default)]
pub struct DoorGraph {
    doors: HashMap<FormId, Vec<LoadDoor>>,
}

impl DoorGraph {
    /// A place's load doors (`0054db50`): an interior's own references, a
    /// worldspace's persistent cell's (`00588270`). Enabled state is
    /// checked when searching.
    pub fn doors(&mut self, order: &LoadOrder, space: FormId) -> &[LoadDoor] {
        self.doors
            .entry(space)
            .or_insert_with(|| read_doors(order, space))
    }
}

fn persistent_cell(order: &LoadOrder, world: FormId) -> Option<FormId> {
    order
        .records_of_type(sig::CELL)
        .into_iter()
        .find(|rr| {
            order.world_of(rr) == Some(world)
                && rr.entry.header.flags & flags::PERSISTENT != 0
                && !rr.entry.header.is_deleted()
        })
        .map(|rr| rr.form_id)
}

fn read_doors(order: &LoadOrder, space: FormId) -> Vec<LoadDoor> {
    let Some(kind) = order.get(space).map(|r| r.entry.header.kind) else {
        return Vec::new();
    };
    let cell = if kind == sig::CELL {
        Some(space)
    } else {
        persistent_cell(order, space)
    };
    let Some(cell) = cell else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for rr in order.references_in_cell(cell) {
        if rr.entry.header.is_deleted() {
            continue;
        }
        let Some(t) = crate::placement::teleport_of(&rr) else {
            continue;
        };
        let Some(base) = crate::scripting::base_of(order, rr.form_id) else {
            continue;
        };
        let Some(base_rr) = order.get(base).filter(|b| b.entry.header.kind == DOOR) else {
            continue;
        };
        let minimal_use = base_rr
            .record()
            .ok()
            .and_then(|r| r.get(FNAM).and_then(|s| s.data.first().copied()))
            .is_some_and(|f| f & MINIMAL_USE != 0);
        let Some(here) = crate::scripting::whereabouts(order, rr.form_id) else {
            continue;
        };
        let Some(there) = crate::scripting::whereabouts(order, t.door) else {
            continue;
        };
        out.push(LoadDoor {
            door: rr.form_id,
            position: here.position,
            to_space: there.world.unwrap_or(there.cell),
            arrival: t.position,
            minimal_use,
        });
    }
    out
}

fn distance(a: [f32; 3], b: [f32; 3]) -> f32 {
    (0..3).map(|i| (a[i] - b[i]).powi(2)).sum::<f32>().sqrt()
}

/// Whether the player can't get through a door (`00502450`, lock
/// disposition 2): locked, without its key.
fn blocked(order: &LoadOrder, state: &GameState, door: FormId) -> bool {
    let Some(lock) = crate::locks::lock_now(order, state, door) else {
        return false;
    };
    !lock
        .key
        .is_some_and(|k| state.item_count(order, PLAYER_REF, k) > 0)
}

#[derive(PartialEq)]
struct Open {
    cost: f32,
    node: usize,
}

impl Eq for Open {}

impl PartialOrd for Open {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Open {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Cheapest first; then the earlier node.
        other
            .cost
            .total_cmp(&self.cost)
            .then_with(|| other.node.cmp(&self.node))
    }
}

/// The doors from `from` (a place and a point in it) to the place
/// `to_space`, in the order taken (`006f34e0`); empty when already there,
/// `None` when no doors lead there.
pub fn door_path(
    order: &LoadOrder,
    state: &GameState,
    graph: &mut DoorGraph,
    from_space: FormId,
    from: [f32; 3],
    to_space: FormId,
) -> Option<Vec<FormId>> {
    struct Node {
        space: FormId,
        at: [f32; 3],
        cost: f32,
        door: Option<FormId>,
        parent: Option<usize>,
    }
    let mut nodes = vec![Node {
        space: from_space,
        at: from,
        cost: 0.0,
        door: None,
        parent: None,
    }];
    let mut best: HashMap<FormId, usize> = HashMap::from([(from_space, 0)]);
    let mut open = BinaryHeap::from([Open { cost: 0.0, node: 0 }]);
    while let Some(Open { cost, node }) = open.pop() {
        if cost > nodes[node].cost {
            continue;
        }
        if nodes[node].space == to_space {
            let mut doors = Vec::new();
            let mut n = Some(node);
            while let Some(i) = n {
                doors.extend(nodes[i].door);
                n = nodes[i].parent;
            }
            doors.reverse();
            return Some(doors);
        }
        let (space, at) = (nodes[node].space, nodes[node].at);
        let doors: Vec<LoadDoor> = graph.doors(order, space).to_vec();
        for d in doors {
            if !crate::placement::enabled_now(order, d.door, &state.disabled) {
                continue;
            }
            let mut step = distance(at, d.position);
            if blocked(order, state, d.door) {
                step += DOOR_PENALTY;
            }
            if d.minimal_use {
                step += DOOR_PENALTY;
            }
            let total = cost + step;
            if best
                .get(&d.to_space)
                .is_some_and(|&i| nodes[i].cost <= total)
            {
                continue;
            }
            nodes.push(Node {
                space: d.to_space,
                at: d.arrival,
                cost: total,
                door: Some(d.door),
                parent: Some(node),
            });
            let i = nodes.len() - 1;
            best.insert(d.to_space, i);
            open.push(Open {
                cost: total,
                node: i,
            });
        }
    }
    None
}

/// A target as the compass shows it: what to point at.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shown {
    pub target: CurrentTarget,
    /// The first door on the way, else the target reference (`005cbb70`).
    pub follow: FormId,
}

/// The current targets and their paths, rebuilt as the game rebuilds them:
/// a path is kept until the player or the target changes cell
/// (`PlayerCharacter::CheckForQuestTargetUpdate` (Xbox PDB) = `00952c30`
/// flags the list for rebuilding then; who calls it on cell changes isn't
/// traced).
#[derive(Default)]
pub struct Tracker {
    pub graph: DoorGraph,
    quests: HashMap<FormId, Option<Quest>>,
    paths: HashMap<(FormId, FormId, FormId), FormId>,
}

impl Tracker {
    /// The active quest's targets this frame and what each points at.
    pub fn shown(&mut self, order: &LoadOrder, state: &GameState) -> Vec<Shown> {
        let Some(active) = state.active_quest else {
            return Vec::new();
        };
        let quest = self
            .quests
            .entry(active)
            .or_insert_with(|| Quest::load(order, active));
        let Some(quest) = quest.as_ref() else {
            return Vec::new();
        };
        let targets = current_targets(order, quest, state);
        let Some((player_space, player_cell, player_at, _)) = state.place(order, PLAYER_REF) else {
            return targets
                .into_iter()
                .map(|t| Shown {
                    target: t,
                    follow: t.reference,
                })
                .collect();
        };
        let mut out = Vec::new();
        for t in targets {
            let follow = match state.place(order, t.reference) {
                Some((space, cell, _, _)) => {
                    let key = (player_cell, t.reference, cell);
                    match self.paths.get(&key) {
                        Some(&f) => f,
                        None => {
                            let f = door_path(
                                order,
                                state,
                                &mut self.graph,
                                player_space,
                                player_at,
                                space,
                            )
                            .and_then(|doors| doors.first().copied())
                            .unwrap_or(t.reference);
                            self.paths.insert(key, f);
                            f
                        }
                    }
                }
                None => t.reference,
            };
            out.push(Shown { target: t, follow });
        }
        out
    }
}
