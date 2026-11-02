//! Which placed objects' scripts run, and when their events fire: the
//! game's per-frame reference-script pass over the loaded cells, `OnLoad`,
//! and trigger volumes (`OnTriggerEnter`, `OnTrigger`, `OnTriggerLeave`).
//!
//! Traced in FalloutNV.exe 1.4.0.525 (evidence: `docs/SCRIPTS_RUNTIME.md`):
//!
//! - `TES::RunScripts` (Xbox PDB; `00455490`) runs each frame: running
//!   quests' scripts, then the interior's cell, or outdoors each cell of the
//!   `uGridsToLoad` × `uGridsToLoad` grid that is attached (cell state 6,
//!   `CS_ATTACHED` (Xbox PDB)), x outer and y inner; then the player's.
//! - `TESObjectCELL::RunScripts` (Xbox PDB; `0054c740`) goes through the
//!   cell's `ScriptedRefs` list (`LOADED_CELL_DATA` +0x4c (Xbox PDB)) and
//!   runs a reference's script (`TESObjectREFR::RunScript`, `00565870`)
//!   when its 3D is loaded or it is disabled (form flag 0x800); deleted
//!   ones (0x20) never. If a run reports a command that may change the
//!   loaded references ([`Runner::references_changed`]) the whole pass
//!   stops for this frame.
//! - `OnLoad` (event 0x1000; its block handler `005cab00`) is flagged for
//!   every reference of a cell when the cell is attached to the grid
//!   (`0054bcf0`, from the grid load `00452ff0`), and for one reference
//!   when its 3D is attached (`00451ef0`, e.g. after `Enable`, through the
//!   model loader `00440ba0`); a flagged block runs at the reference's
//!   next run. Cells that stay in the grid when it moves are not attached
//!   again.
//! - The grid moves only when the player is `4096` units (3072 with
//!   `uGridsToLoad` 3) or more from the centre cell's centre on an axis,
//!   and then centres on the player's cell (`TES::UpdateCurrentGridCell`
//!   (Xbox PDB), `00452580`).
//! - Triggers: the trap listener updates each trigger every step
//!   (`TriggerEntry::Update` (Xbox PDB), `0062cc90`): a disabled, deleted
//!   or unloaded trigger is skipped; someone already inside gets
//!   `OnTrigger` flagged; someone new is added and an enter event queued
//!   (`TriggerEntry::RefList::Add`, `0062c9d0`); whoever wasn't seen gets
//!   a leave event queued and is removed (`UpdateExitList`, `0062cb90`);
//!   then one queued event is taken and `OnTriggerEnter` or
//!   `OnTriggerLeave` flagged, with that person as the trigger's action
//!   reference (`ExtraAction`, `0041b4b0`). When a trigger is taken out
//!   (`ForceLeaveTrigger` (Xbox PDB), `0062c860`) everyone inside gets
//!   `OnTriggerLeave` and its script runs at once, once per person.

use std::collections::{HashMap, VecDeque};

use esm::{FormId, LoadOrder};

use crate::scripting::{Interactive, Runner};

/// `[General] uGridsToLoad` in the game's `Fallout.ini`.
pub const GRIDS_TO_LOAD: i32 = 5;

/// Heights above the feet at which a person's body is tested against
/// trigger volumes (feet, middle, head). The game tests the character's
/// collision shape against the trigger's phantom; these three points stand
/// for it here (unresolved approximation, kept from the viewer).
pub const BODY_HEIGHTS: [f32; 3] = [10.0, 64.0, 110.0];

pub const ON_LOAD: &str = "onload";
pub const ON_TRIGGER: &str = "ontrigger";
pub const ON_TRIGGER_ENTER: &str = "ontriggerenter";
pub const ON_TRIGGER_LEAVE: &str = "ontriggerleave";

/// The grid's centre after the player moves to `position`: unchanged
/// while the player is less than 4096 units (3072 with a grid of 3) from
/// the current centre cell's centre on both axes, else the player's cell.
/// `None`: no grid yet, the player's cell.
// Translated from 00452580 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn grid_center(current: Option<(i32, i32)>, position: [f32; 3], grids: i32) -> (i32, i32) {
    // `00406d90`: FISTP (round to nearest), then `>> 12`.
    let x = fistp(position[0]);
    let y = fistp(position[1]);
    let here = (x >> 12, y >> 12);
    let Some((cx, cy)) = current else {
        return here;
    };
    let reach = if grids == 3 { 3072.0 } else { 4096.0 };
    let inside_x = reach - (x - (cx * 0x1000 + 0x800)).abs() as f32;
    let inside_y = reach - (y - (cy * 0x1000 + 0x800)).abs() as f32;
    if inside_x > 0.0 && inside_y > 0.0 {
        (cx, cy)
    } else {
        here
    }
}

/// A float to an integer as `FISTP` does with the default rounding mode:
/// to nearest, halves to even.
fn fistp(v: f32) -> i32 {
    let r = v.round();
    let r = if (v - v.trunc()).abs() == 0.5 && r % 2.0 != 0.0 {
        r - v.signum()
    } else {
        r
    };
    r as i32
}

/// The grid's squares around a centre, in the order the game runs their
/// scripts (`00455490`: x outer, y inner, from the low corner).
pub fn grid_squares(center: (i32, i32), grids: i32) -> Vec<(i32, i32)> {
    let grids = grids.max(1);
    let lo = (center.0 - grids / 2, center.1 - grids / 2);
    let mut out = Vec::with_capacity((grids * grids) as usize);
    for i in 0..grids {
        for j in 0..grids {
            out.push((lo.0 + i, lo.1 + j));
        }
    }
    out
}

/// The placed references a script moved somewhere else (a place of their
/// own in the state, [`crate::scripting::GameState::spaces`]), for
/// [`RefScripts::place_moved`]. People and creatures are left out: they
/// change cells as their AI moves them, which isn't followed here.
pub fn moved_references(order: &LoadOrder, state: &crate::scripting::GameState) -> Vec<FormId> {
    let mut out: Vec<FormId> = state
        .spaces
        .keys()
        .copied()
        .filter(|&r| r != crate::dialogue::PLAYER_REF)
        .filter(|&r| {
            crate::scripting::base_of(order, r)
                .and_then(|b| order.get(b))
                .is_some_and(|b| !matches!(b.entry.header.kind.as_bytes(), b"NPC_" | b"CREA"))
        })
        .collect();
    out.sort();
    out
}

/// Who is inside one trigger, and its queued enter (`true`) and leave
/// events (`TriggerEntry::RefList` (Xbox PDB)).
#[derive(Debug, Default, Clone)]
struct Occupancy {
    /// (who, seen this step).
    inside: Vec<(FormId, bool)>,
    queue: VecDeque<(FormId, bool)>,
}

/// The attached cells' references and their scripts' pending events.
#[derive(Debug, Default)]
pub struct RefScripts {
    /// The worldspace and grid centre (`TES` +0x24/+0x28), outdoors.
    world: Option<FormId>,
    center: Option<(i32, i32)>,
    /// Attached cells in pass order, with their references.
    cells: Vec<(FormId, Vec<Interactive>)>,
    all: Vec<Interactive>,
    /// Events flagged in each reference's event list since its last run.
    pending: HashMap<FormId, Vec<(&'static str, Option<FormId>)>>,
    /// A trigger's action reference (`ExtraAction`), once set.
    action: HashMap<FormId, FormId>,
    triggers: HashMap<FormId, Occupancy>,
    /// Whether each scripted reference was disabled last frame.
    disabled: HashMap<FormId, bool>,
}

impl RefScripts {
    /// The grid's centre for the player at `position` in `world`, kept
    /// from frame to frame ([`grid_center`]); a new worldspace starts anew.
    pub fn exterior_center(&mut self, world: FormId, position: [f32; 3], grids: i32) -> (i32, i32) {
        if self.world != Some(world) {
            self.world = Some(world);
            self.center = None;
        }
        let center = grid_center(self.center, position, grids);
        self.center = Some(center);
        center
    }

    /// Indoors: no grid.
    pub fn interior(&mut self) {
        self.world = None;
        self.center = None;
    }

    /// Every reference of the attached cells (pass order).
    pub fn refs(&self) -> &[Interactive] {
        &self.all
    }

    /// The attached cells, in pass order.
    pub fn cells(&self) -> impl Iterator<Item = FormId> + '_ {
        self.cells.iter().map(|(c, _)| *c)
    }

    /// The events flagged for a reference and not yet run.
    pub fn pending(&self, reference: FormId) -> &[(&'static str, Option<FormId>)] {
        self.pending.get(&reference).map_or(&[], Vec::as_slice)
    }

    /// Who is inside a trigger.
    pub fn inside(&self, trigger: FormId) -> Vec<FormId> {
        self.triggers
            .get(&trigger)
            .map(|o| o.inside.iter().map(|(w, _)| *w).collect())
            .unwrap_or_default()
    }

    fn flag(&mut self, reference: FormId, event: &'static str, who: Option<FormId>) {
        let list = self.pending.entry(reference).or_default();
        if !list.contains(&(event, who)) {
            list.push((event, who));
        }
    }

    /// Makes `cells` (in pass order) the attached ones. A cell not
    /// attached before is loaded (`load`) and all its references get
    /// `OnLoad` flagged; one no longer attached is dropped, its triggers'
    /// occupants leaving them. Returns whether anything changed.
    pub fn attach(
        &mut self,
        runner: &mut Runner,
        cells: &[FormId],
        mut load: impl FnMut(FormId) -> Vec<Interactive>,
    ) -> bool {
        let same = self.cells.len() == cells.len()
            && self.cells.iter().zip(cells).all(|((a, _), b)| a == b);
        if same {
            return false;
        }
        let mut old: HashMap<FormId, Vec<Interactive>> = self.cells.drain(..).collect();
        // Taken out of the world: triggers let everyone inside go.
        let gone: Vec<FormId> = old
            .iter()
            .filter(|(c, _)| !cells.contains(c))
            .flat_map(|(_, refs)| refs.iter().map(|r| r.reference))
            .collect();
        for reference in gone {
            self.force_leave(runner, reference);
            self.disabled.remove(&reference);
        }
        for &cell in cells {
            let refs = match old.remove(&cell) {
                Some(refs) => refs,
                None => {
                    let refs = load(cell);
                    // `0054bcf0`: every reference of the cell, enabled or
                    // not, before its 3D is attached.
                    for r in refs.iter().filter(|r| r.script.is_some()) {
                        self.flag(r.reference, ON_LOAD, None);
                        let off =
                            !crate::enabled_now(runner.order, r.reference, &runner.state.disabled);
                        self.disabled.insert(r.reference, off);
                    }
                    refs
                }
            };
            self.cells.push((cell, refs));
        }
        self.all = self
            .cells
            .iter()
            .flat_map(|(_, refs)| refs.iter().cloned())
            .collect();
        true
    }

    /// References scripts moved (`MoveTo`, `SetPos`) go with their scripts
    /// to the cell they are in now. The game's `MoveTo` (`005ccb20`) sets
    /// the position and hands the reference to `00573800`: outdoors the
    /// cell of the grid square under it (`005875a0(x >> 12, y >> 12)`)
    /// takes it (`00548230`: out of its old cell's list, `0054ca90`, into
    /// the new one, its 3D loaded when that cell is attached, which flags
    /// `OnLoad`, `00451ef0`); `TESObjectCELL::RunScripts` then runs it with
    /// that cell's references. `now` gives each moved reference
    /// ([`moved_references`]) and the attached cell it stands in (`None`:
    /// none attached). One moved out of the attached cells stops running
    /// and leaves its trigger's occupants; one moved into one gets
    /// `OnLoad`. Returns whether anything changed.
    pub fn place_moved(&mut self, runner: &mut Runner, now: &[(FormId, Option<FormId>)]) -> bool {
        let mut changed = false;
        for &(reference, cell_now) in now {
            let listed = self
                .cells
                .iter()
                .position(|(_, refs)| refs.iter().any(|r| r.reference == reference));
            let listed_cell = listed.map(|i| self.cells[i].0);
            let target = cell_now.filter(|c| self.cells.iter().any(|(cell, _)| cell == c));
            if listed_cell == target {
                // Still there: only where it stands (its trigger moved).
                if let (Some(i), Some((_, _, p, _))) =
                    (listed, runner.state.place(runner.order, reference))
                {
                    for r in self.cells[i]
                        .1
                        .iter_mut()
                        .filter(|r| r.reference == reference)
                    {
                        r.position = p;
                    }
                }
                continue;
            }
            changed = true;
            let mut item = None;
            if let Some(i) = listed {
                let refs = &mut self.cells[i].1;
                if let Some(at) = refs.iter().position(|r| r.reference == reference) {
                    item = Some(refs.remove(at));
                }
                self.pending.remove(&reference);
                self.force_leave(runner, reference);
                self.disabled.remove(&reference);
            }
            let Some(cell) = target else { continue };
            let Some(mut r) =
                item.or_else(|| crate::scripting::interactive_reference(runner.order, reference))
            else {
                continue;
            };
            if let Some((_, _, p, _)) = runner.state.place(runner.order, reference) {
                r.position = p;
            }
            if r.script.is_some() {
                self.flag(reference, ON_LOAD, None);
                let off = !crate::enabled_now(runner.order, reference, &runner.state.disabled);
                self.disabled.insert(reference, off);
            }
            if let Some((_, refs)) = self.cells.iter_mut().find(|(c, _)| *c == cell) {
                refs.push(r);
            }
        }
        if changed {
            self.all = self
                .cells
                .iter()
                .flat_map(|(_, refs)| refs.iter().cloned())
                .collect();
        }
        changed
    }

    /// Everyone inside a trigger leaves it at once (`0062c860`): for each,
    /// `OnTriggerLeave` is flagged, they become the action reference and
    /// the trigger's script runs.
    // Translated from 0062c860 (decompiled, FalloutNV.exe 1.4.0.525)
    fn force_leave(&mut self, runner: &mut Runner, trigger: FormId) {
        let Some(occupancy) = self.triggers.remove(&trigger) else {
            return;
        };
        for (who, _) in occupancy.inside {
            self.flag(trigger, ON_TRIGGER_LEAVE, Some(who));
            self.action.insert(trigger, who);
            let events = self.pending.remove(&trigger).unwrap_or_default();
            runner.run_reference_script(trigger, &events, Some(who));
        }
    }

    /// One trigger's step (`0062cc90`): `people` are those whose body is in
    /// its volume.
    // Translated from 0062cc90 (decompiled, FalloutNV.exe 1.4.0.525)
    fn update_trigger(&mut self, trigger: FormId, people: &[FormId]) {
        let occupancy = self.triggers.entry(trigger).or_default();
        let mut on_trigger = Vec::new();
        for &who in people {
            // `0062c9d0`.
            match occupancy.inside.iter_mut().find(|(w, _)| *w == who) {
                Some((_, seen)) if !*seen => {
                    *seen = true;
                    on_trigger.push(who);
                }
                Some(_) => {}
                None => {
                    occupancy.inside.push((who, true));
                    occupancy.queue.push_back((who, true));
                }
            }
        }
        // `0062cb90`.
        let mut kept = Vec::with_capacity(occupancy.inside.len());
        for (who, seen) in occupancy.inside.drain(..) {
            if seen {
                kept.push((who, false));
            } else {
                occupancy.queue.push_back((who, false));
            }
        }
        occupancy.inside = kept;
        let event = occupancy.queue.pop_front();
        if occupancy.inside.is_empty() && occupancy.queue.is_empty() {
            self.triggers.remove(&trigger);
        }
        for who in on_trigger {
            self.flag(trigger, ON_TRIGGER, Some(who));
        }
        if let Some((who, enter)) = event {
            let kind = if enter {
                ON_TRIGGER_ENTER
            } else {
                ON_TRIGGER_LEAVE
            };
            self.flag(trigger, kind, Some(who));
            self.action.insert(trigger, who);
        }
    }

    /// One frame in game mode: references enabled since last frame get
    /// `OnLoad` (their 3D is attached), triggers are updated for `people`
    /// (reference and feet), then the reference-script pass runs
    /// (`00455490` over the attached cells, `0054c740` within each).
    /// Returns the references whose scripts ran.
    pub fn frame(&mut self, runner: &mut Runner, people: &[(FormId, [f32; 3])]) -> Vec<FormId> {
        let order: &LoadOrder = runner.order;
        let scripted: Vec<FormId> = self
            .all
            .iter()
            .filter(|r| r.script.is_some())
            .map(|r| r.reference)
            .collect();
        // Enabled or disabled since: an enabled one's 3D is attached
        // (`00451ef0`); a disabled one's trigger is taken out (its
        // occupants leave: `00576760` → `0062de90`; the caller chain is
        // not fully traced).
        for &reference in &scripted {
            let off = !crate::enabled_now(order, reference, &runner.state.disabled);
            match self.disabled.insert(reference, off) {
                Some(true) if !off => self.flag(reference, ON_LOAD, None),
                Some(false) if off => self.force_leave(runner, reference),
                _ => {}
            }
        }
        let steps: Vec<(FormId, Vec<FormId>)> = self
            .all
            .iter()
            .filter(|r| r.script.is_some() && r.trigger.is_some())
            .filter(|r| self.disabled.get(&r.reference) != Some(&true))
            .map(|r| {
                let inside = people
                    .iter()
                    .filter(|(_, feet)| {
                        BODY_HEIGHTS
                            .iter()
                            .any(|h| r.contains([feet[0], feet[1], feet[2] + h]))
                    })
                    .map(|(who, _)| *who)
                    .collect::<Vec<_>>();
                (r.reference, inside)
            })
            .filter(|(t, inside)| !inside.is_empty() || self.triggers.contains_key(t))
            .collect();
        for (trigger, inside) in steps {
            self.update_trigger(trigger, &inside);
        }
        // Translated from 0054c740 (decompiled, FalloutNV.exe 1.4.0.525):
        // every scripted reference here has its 3D loaded or is disabled,
        // so each runs; the pass stops when a run reports a change.
        let mut ran = Vec::new();
        for reference in scripted {
            let events = self.pending.remove(&reference).unwrap_or_default();
            let action = self.action.get(&reference).copied();
            ran.push(reference);
            if runner.run_reference_script(reference, &events, action) {
                break;
            }
        }
        ran
    }
}
