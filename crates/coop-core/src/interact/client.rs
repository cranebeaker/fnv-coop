//! Guest side of the interaction layer.

use std::collections::BTreeMap;

use super::{Act, Follow, InteractMsg, Shared, Why};
use crate::proto::Msg;

/// A finished request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Settled {
    /// Request id.
    pub txn: u32,
    /// The action asked for.
    pub act: Act,
    /// Done.
    pub ok: bool,
    /// Why not.
    pub why: Why,
}

/// What a guest tracks.
#[derive(Clone, Debug)]
pub struct InteractClient {
    next: u32,
    pending: BTreeMap<u32, Act>,
    /// Verdicts to hand to the game (drain).
    pub settled: Vec<Settled>,
    /// Shared facts to apply to the world (drain).
    pub shared: Vec<Shared>,
    /// Destinations to load (drain).
    pub follows: Vec<Follow>,
    /// Requests sent.
    pub sent: u32,
    /// Verdicts received.
    pub verdicts: u32,
}

impl Default for InteractClient {
    fn default() -> Self {
        InteractClient {
            next: 1,
            pending: BTreeMap::new(),
            settled: Vec::new(),
            shared: Vec::new(),
            follows: Vec::new(),
            sent: 0,
            verdicts: 0,
        }
    }
}

impl InteractClient {
    /// A fresh client.
    pub fn new() -> Self {
        Self::default()
    }

    /// Asks the host to do `act`; returns the message to send.
    pub fn request(&mut self, act: Act) -> Msg {
        let txn = self.next;
        self.next += 1;
        self.sent += 1;
        self.pending.insert(txn, act.clone());
        Msg::Interact(InteractMsg::Request { txn, act })
    }

    /// Requests without a verdict yet.
    pub fn pending(&self) -> usize {
        self.pending.len()
    }

    /// Applies one host message.
    pub fn on_msg(&mut self, m: InteractMsg) {
        match m {
            InteractMsg::Verdict { txn, ok, why } => {
                if let Some(act) = self.pending.remove(&txn) {
                    self.verdicts += 1;
                    self.settled.push(Settled { txn, act, ok, why });
                }
            }
            InteractMsg::Shared(s) => self.shared.push(s),
            InteractMsg::Sync(v) => self.shared.extend(v),
            InteractMsg::Follow(f) => self.follows.push(f),
            InteractMsg::Request { .. } => {}
        }
    }
}
