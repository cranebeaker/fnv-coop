//! Host side of the interaction layer: exactly-once requests, the shared
//! world log, and the answers.

use std::collections::{BTreeMap, VecDeque};

use super::transit::Transit;
use super::{Act, Follow, InteractMsg, Shared, Why, DROP_BASE, SYNC_CHUNK};
use crate::ids::{FormId, PlayerId};
use crate::interest::Audience;

/// One addressed output message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IOut {
    /// Recipients.
    pub aud: Audience,
    /// Message.
    pub msg: InteractMsg,
}

fn out(aud: Audience, msg: InteractMsg) -> IOut {
    IOut { aud, msg }
}

/// What executing a request produced.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Outcome {
    /// Done.
    pub ok: bool,
    /// Why not.
    pub why: Why,
    /// Shared world facts to publish.
    pub shared: Vec<Shared>,
    /// A destination some players must load.
    pub follow: Option<(Follow, Vec<PlayerId>)>,
}

impl Outcome {
    /// Success with nothing to publish.
    pub fn done() -> Outcome {
        Outcome {
            ok: true,
            why: Why::None,
            shared: Vec::new(),
            follow: None,
        }
    }

    /// Refusal.
    pub fn refused(why: Why) -> Outcome {
        Outcome {
            ok: false,
            why,
            shared: Vec::new(),
            follow: None,
        }
    }

    /// Adds shared facts.
    pub fn with_shared(mut self, s: Vec<Shared>) -> Outcome {
        self.shared = s;
        self
    }

    /// Adds a destination for `who`.
    pub fn with_follow(mut self, f: Follow, who: Vec<PlayerId>) -> Outcome {
        self.follow = Some((f, who));
        self
    }
}

/// Counters for tests and the dump.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct InteractStats {
    /// Fresh requests queued.
    pub requests: u32,
    /// Duplicate requests answered from the cache.
    pub replays: u32,
    /// Requests that succeeded.
    pub ok: u32,
    /// Requests that were refused.
    pub refused: u32,
    /// Shared records published.
    pub shared: u32,
    /// Follow orders sent.
    pub follows: u32,
}

const CACHE_CAP: usize = 256;

/// The authoritative side.
#[derive(Clone, Debug)]
pub struct InteractHost {
    seen: BTreeMap<PlayerId, u32>,
    done: BTreeMap<(PlayerId, u32), InteractMsg>,
    order: VecDeque<(PlayerId, u32)>,
    queue: Vec<(PlayerId, u32, Act)>,
    log: BTreeMap<(u8, FormId), Shared>,
    next_drop: u32,
    /// Transition policy.
    pub transit: Transit,
    /// Counters.
    pub stats: InteractStats,
}

impl Default for InteractHost {
    fn default() -> Self {
        InteractHost {
            seen: BTreeMap::new(),
            done: BTreeMap::new(),
            order: VecDeque::new(),
            queue: Vec::new(),
            log: BTreeMap::new(),
            next_drop: 1,
            transit: Transit::default(),
            stats: InteractStats::default(),
        }
    }
}

impl InteractHost {
    /// Empty state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles a message from `from`. A fresh request waits in the queue for
    /// the game to execute it; a duplicate is answered from the cache and
    /// never executes twice.
    pub fn on_msg(&mut self, from: PlayerId, msg: InteractMsg) -> Vec<IOut> {
        let InteractMsg::Request { txn, act } = msg else {
            return vec![];
        };
        if txn == 0 {
            return vec![];
        }
        if txn <= self.seen.get(&from).copied().unwrap_or(0) {
            self.stats.replays += 1;
            return self
                .done
                .get(&(from, txn))
                .cloned()
                .map(|m| out(Audience::To(from), m))
                .into_iter()
                .collect();
        }
        self.seen.insert(from, txn);
        self.stats.requests += 1;
        self.queue.push((from, txn, act));
        vec![]
    }

    /// Requests waiting for the game, oldest first.
    pub fn take_queue(&mut self) -> Vec<(PlayerId, u32, Act)> {
        std::mem::take(&mut self.queue)
    }

    /// Records the result of an executed request and returns what to send.
    pub fn resolve(&mut self, from: PlayerId, txn: u32, o: Outcome) -> Vec<IOut> {
        let verdict = InteractMsg::Verdict {
            txn,
            ok: o.ok,
            why: o.why,
        };
        if o.ok {
            self.stats.ok += 1;
        } else {
            self.stats.refused += 1;
        }
        self.done.insert((from, txn), verdict.clone());
        self.order.push_back((from, txn));
        while self.order.len() > CACHE_CAP {
            if let Some(k) = self.order.pop_front() {
                self.done.remove(&k);
            }
        }
        let mut v = vec![out(Audience::To(from), verdict)];
        v.extend(self.publish(o.shared));
        if let Some((f, who)) = o.follow {
            v.extend(self.follow(f, &who));
        }
        v
    }

    /// Publishes shared facts (the host's own, or an executed request's).
    pub fn publish(&mut self, shared: Vec<Shared>) -> Vec<IOut> {
        let mut v = Vec::new();
        for s in shared {
            if self.log.get(&s.key()) == Some(&s) {
                continue;
            }
            self.log.insert(s.key(), s.clone());
            self.stats.shared += 1;
            v.push(out(Audience::All, InteractMsg::Shared(s)));
        }
        v
    }

    /// Orders `who` to load a destination.
    pub fn follow(&mut self, f: Follow, who: &[PlayerId]) -> Vec<IOut> {
        who.iter()
            .map(|p| {
                self.stats.follows += 1;
                out(Audience::To(*p), InteractMsg::Follow(f))
            })
            .collect()
    }

    /// What a joiner missed.
    pub fn sync_for(&self, p: PlayerId) -> Vec<IOut> {
        let all: Vec<Shared> = self.log.values().cloned().collect();
        all.chunks(SYNC_CHUNK)
            .map(|c| out(Audience::To(p), InteractMsg::Sync(c.to_vec())))
            .collect()
    }

    /// The shared record under a key, if any.
    pub fn shared(&self, key: (u8, FormId)) -> Option<&Shared> {
        self.log.get(&key)
    }

    /// Number of shared records held.
    pub fn log_len(&self) -> usize {
        self.log.len()
    }

    /// A fresh form for an item dropped into the world.
    pub fn mint_drop(&mut self) -> FormId {
        let id = FormId(DROP_BASE + self.next_drop);
        self.next_drop += 1;
        id
    }

    /// A player left: their next session starts a fresh transaction space.
    pub fn forget(&mut self, p: PlayerId) {
        self.seen.remove(&p);
        self.done.retain(|(q, _), _| *q != p);
        self.order.retain(|(q, _)| *q != p);
        self.queue.retain(|(q, _, _)| *q != p);
    }
}
