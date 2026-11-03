//! Host-side queue of script operations on references in cells nobody has
//! loaded. Quest and story scripts run only on the host; a command that
//! needs a live entity (an animation, a package, combat) cannot run on a
//! reference whose cell the host has not loaded. It waits here, keyed by
//! cell, and is replayed in order when the cell is loaded by any peer.

use std::collections::{BTreeMap, VecDeque};

use serde::{Deserialize, Serialize};

use crate::ids::{CellId, FormId};

/// Most operations kept per cell.
pub const MAX_PER_CELL: usize = 128;
/// Most operations kept in all.
pub const MAX_TOTAL: usize = 1024;

/// One argument of a queued command.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DArg {
    /// A number, as `f64::to_bits` (so arguments compare exactly).
    Num(u64),
    /// A form.
    Form(u32),
    /// A string.
    Text(String),
}

/// One queued command.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeferredOp {
    /// Queue-wide id.
    pub id: u32,
    /// The reference the command is called on.
    pub target: FormId,
    /// Its cell.
    pub cell: CellId,
    /// Script function name.
    pub func: String,
    /// Arguments.
    pub args: Vec<DArg>,
    /// Session time it was queued at.
    pub queued_ms: u64,
}

impl DArg {
    /// A number argument.
    pub fn num(v: f64) -> DArg {
        DArg::Num(v.to_bits())
    }

    /// The number, if this is one.
    pub fn as_num(&self) -> Option<f64> {
        match self {
            DArg::Num(b) => Some(f64::from_bits(*b)),
            _ => None,
        }
    }
}

/// Functions whose later call makes an earlier one on the same reference
/// pointless (set-style): the queue keeps only the newest.
pub fn last_wins(func: &str) -> bool {
    matches!(
        func.to_ascii_lowercase().as_str(),
        "look"
            | "stoplook"
            | "evaluatepackage"
            | "setrestrained"
            | "setalert"
            | "setunconscious"
            | "setignorefriendlyhits"
    )
}

/// The queue.
#[derive(Clone, Debug, Default)]
pub struct DeferredQueue {
    by_cell: BTreeMap<CellId, VecDeque<DeferredOp>>,
    next_id: u32,
    /// Operations queued so far.
    pub queued: u32,
    /// Operations handed back for replay.
    pub replayed: u32,
    /// Operations replaced by a newer set-style call.
    pub superseded: u32,
    /// Operations dropped because the queue was full.
    pub dropped: u32,
}

impl DeferredQueue {
    /// Empty queue.
    pub fn new() -> Self {
        Self::default()
    }

    /// Queues a command; returns its id.
    pub fn queue(
        &mut self,
        target: FormId,
        cell: CellId,
        func: &str,
        args: Vec<DArg>,
        now_ms: u64,
    ) -> u32 {
        self.next_id += 1;
        let id = self.next_id;
        let q = self.by_cell.entry(cell).or_default();
        if last_wins(func) {
            let before = q.len();
            q.retain(|o| !(o.target == target && o.func.eq_ignore_ascii_case(func)));
            self.superseded += (before - q.len()) as u32;
        }
        q.push_back(DeferredOp {
            id,
            target,
            cell,
            func: func.to_string(),
            args,
            queued_ms: now_ms,
        });
        self.queued += 1;
        if q.len() > MAX_PER_CELL {
            q.pop_front();
            self.dropped += 1;
        }
        while self.pending() > MAX_TOTAL {
            // drop the oldest of the longest queue
            let Some(c) = self
                .by_cell
                .iter()
                .max_by_key(|(_, q)| q.len())
                .map(|(c, _)| *c)
            else {
                break;
            };
            if let Some(q) = self.by_cell.get_mut(&c) {
                q.pop_front();
                self.dropped += 1;
            }
        }
        id
    }

    /// Operations waiting.
    pub fn pending(&self) -> usize {
        self.by_cell.values().map(VecDeque::len).sum()
    }

    /// Operations waiting for one cell.
    pub fn pending_in(&self, cell: CellId) -> usize {
        self.by_cell.get(&cell).map_or(0, VecDeque::len)
    }

    /// Cells with waiting operations.
    pub fn cells(&self) -> Vec<CellId> {
        self.by_cell
            .iter()
            .filter(|(_, q)| !q.is_empty())
            .map(|(c, _)| *c)
            .collect()
    }

    /// A cell was loaded: its operations, oldest first, leave the queue.
    pub fn take_for_cell(&mut self, cell: CellId) -> Vec<DeferredOp> {
        let v: Vec<DeferredOp> = self
            .by_cell
            .remove(&cell)
            .map(Vec::from)
            .unwrap_or_default();
        self.replayed += v.len() as u32;
        v
    }

    /// Every operation whose cell `loaded` says is loaded now, in id order.
    pub fn drain_loaded(&mut self, loaded: impl Fn(CellId) -> bool) -> Vec<DeferredOp> {
        let cells: Vec<CellId> = self.cells().into_iter().filter(|c| loaded(*c)).collect();
        let mut v: Vec<DeferredOp> = cells
            .into_iter()
            .flat_map(|c| self.take_for_cell(c))
            .collect();
        v.sort_by_key(|o| o.id);
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const R: FormId = FormId(0x100);

    #[test]
    fn ops_wait_for_their_cell_and_replay_in_order() {
        let mut q = DeferredQueue::new();
        q.queue(
            R,
            CellId(1),
            "PlayGroup",
            vec![DArg::Text("Idle".into())],
            0,
        );
        q.queue(R, CellId(2), "StartCombat", vec![DArg::Form(5)], 1);
        q.queue(R, CellId(1), "Activate", vec![], 2);
        assert_eq!(q.pending(), 3);
        assert!(q.take_for_cell(CellId(9)).is_empty());
        let got = q.take_for_cell(CellId(1));
        assert_eq!(
            got.iter().map(|o| o.func.as_str()).collect::<Vec<_>>(),
            vec!["PlayGroup", "Activate"]
        );
        assert_eq!((q.pending(), q.replayed), (1, 2));
        assert_eq!(q.take_for_cell(CellId(1)).len(), 0, "replayed once");
    }

    #[test]
    fn set_style_calls_keep_only_the_newest() {
        let mut q = DeferredQueue::new();
        q.queue(R, CellId(1), "Look", vec![DArg::Form(1)], 0);
        q.queue(R, CellId(1), "Look", vec![DArg::Form(2)], 1);
        q.queue(FormId(0x200), CellId(1), "Look", vec![DArg::Form(3)], 2);
        q.queue(R, CellId(1), "Activate", vec![], 3);
        q.queue(R, CellId(1), "Activate", vec![], 4);
        let got = q.take_for_cell(CellId(1));
        assert_eq!(got.len(), 4, "two Looks (different refs) and two Activates");
        assert_eq!(q.superseded, 1);
        assert_eq!(
            got.iter()
                .find(|o| o.target == R && o.func == "Look")
                .unwrap()
                .args,
            vec![DArg::Form(2)]
        );
    }

    #[test]
    fn the_queue_is_bounded() {
        let mut q = DeferredQueue::new();
        for i in 0..(MAX_PER_CELL + 10) {
            q.queue(R, CellId(1), "Activate", vec![DArg::num(i as f64)], 0);
        }
        assert_eq!(q.pending_in(CellId(1)), MAX_PER_CELL);
        assert_eq!(q.dropped, 10);
        for c in 0..20u32 {
            for _ in 0..MAX_PER_CELL {
                q.queue(R, CellId(100 + c), "Activate", vec![], 0);
            }
        }
        assert!(q.pending() <= MAX_TOTAL);
    }

    #[test]
    fn drain_loaded_takes_only_loaded_cells_oldest_first() {
        let mut q = DeferredQueue::new();
        q.queue(R, CellId(2), "A", vec![], 0);
        q.queue(R, CellId(1), "B", vec![], 0);
        q.queue(R, CellId(3), "C", vec![], 0);
        let got = q.drain_loaded(|c| c != CellId(3));
        assert_eq!(
            got.iter().map(|o| o.func.as_str()).collect::<Vec<_>>(),
            vec!["A", "B"]
        );
        assert_eq!(q.pending(), 1);
    }
}
