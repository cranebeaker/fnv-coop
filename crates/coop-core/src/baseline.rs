//! Per-client acked-baseline tracking (Quake 3 / Source style).
//!
//! The host remembers, for each client, the last `cap` sends as
//! `(seq, rev reached)`. When the client acks a sequence number the baseline
//! moves to the revision that send reached, and the next delta is built
//! against it. If the client stops acking for `cap` sends the baseline has
//! aged out and the host falls back to a full snapshot.

use std::collections::{BTreeMap, VecDeque};

use crate::ids::{PlayerId, Rev, Seq};

/// What the host should send a client this tick.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SendPlan {
    /// The client is up to date, or a snapshot is in flight.
    Nothing,
    /// A delta against this baseline revision.
    Delta {
        /// Baseline the client has acknowledged.
        base: Rev,
    },
    /// A full snapshot (no usable baseline).
    Full,
}

#[derive(Debug, Default)]
struct Client {
    next_seq: Seq,
    sent: VecDeque<(Seq, Rev)>,
    acked_rev: Option<Rev>,
    full_pending: bool,
}

/// Baseline ring for every client.
#[derive(Debug)]
pub struct BaselineRing {
    cap: usize,
    clients: BTreeMap<PlayerId, Client>,
}

impl BaselineRing {
    /// A ring remembering `cap` sends per client (at least 1).
    pub fn new(cap: usize) -> Self {
        BaselineRing {
            cap: cap.max(1),
            clients: BTreeMap::new(),
        }
    }

    /// Forgets the baseline of `p` (new join). The sequence counter keeps
    /// counting so old acks can never match new sends.
    pub fn reset(&mut self, p: PlayerId) {
        let c = self.clients.entry(p).or_default();
        c.sent.clear();
        c.acked_rev = None;
        c.full_pending = false;
    }

    /// Drops a client entirely.
    pub fn remove(&mut self, p: PlayerId) {
        self.clients.remove(&p);
    }

    /// Declares that `p` already holds everything up to `rev` (rejoin).
    pub fn resume_from(&mut self, p: PlayerId, rev: Rev) {
        let c = self.clients.entry(p).or_default();
        c.sent.clear();
        c.acked_rev = Some(rev);
        c.full_pending = false;
    }

    /// Allocates the sequence number of the next send to `p`.
    pub fn next_seq(&mut self, p: PlayerId) -> Seq {
        let c = self.clients.entry(p).or_default();
        let s = c.next_seq;
        c.next_seq = s.next();
        s
    }

    /// The most recent sequence number handed out to `p` (zero if none).
    pub fn last_seq(&self, p: PlayerId) -> Seq {
        self.clients
            .get(&p)
            .map_or(Seq(0), |c| Seq(c.next_seq.0.wrapping_sub(1)))
    }

    /// The acked baseline of `p`, if any.
    pub fn acked_rev(&self, p: PlayerId) -> Option<Rev> {
        self.clients.get(&p).and_then(|c| c.acked_rev)
    }

    /// Records that send `seq` brought the client to `rev` if acked.
    pub fn record_sent(&mut self, p: PlayerId, seq: Seq, rev: Rev) {
        let cap = self.cap;
        let c = self.clients.entry(p).or_default();
        c.sent.push_back((seq, rev));
        while c.sent.len() > cap {
            c.sent.pop_front();
        }
    }

    /// Processes an ack. Returns true when it matched a remembered send.
    /// Older remembered sends are dropped; the baseline never moves backwards.
    pub fn on_ack(&mut self, p: PlayerId, seq: Seq) -> bool {
        let Some(c) = self.clients.get_mut(&p) else {
            return false;
        };
        let Some(pos) = c.sent.iter().position(|(s, _)| *s == seq) else {
            return false;
        };
        let rev = c.sent[pos].1;
        c.sent.drain(..=pos);
        if c.acked_rev.map_or(true, |a| rev > a) {
            c.acked_rev = Some(rev);
        }
        c.full_pending = false;
        true
    }

    /// Decides what to send to `p`, given the host's current revision.
    /// Returning `Full` clears the ring and marks a snapshot as pending; the
    /// caller must then send a snapshot and `record_sent` it.
    pub fn plan(&mut self, p: PlayerId, current: Rev) -> SendPlan {
        let cap = self.cap;
        let c = self.clients.entry(p).or_default();
        if c.full_pending {
            return SendPlan::Nothing;
        }
        let aged_out = c.sent.len() >= cap;
        match c.acked_rev {
            None => {
                c.sent.clear();
                c.full_pending = true;
                SendPlan::Full
            }
            Some(_) if aged_out => {
                c.sent.clear();
                c.acked_rev = None;
                c.full_pending = true;
                SendPlan::Full
            }
            Some(a) if a >= current => SendPlan::Nothing,
            Some(a) => SendPlan::Delta { base: a },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const P: PlayerId = PlayerId(1);

    #[test]
    fn new_client_gets_full_then_deltas() {
        let mut r = BaselineRing::new(8);
        assert_eq!(r.plan(P, Rev(5)), SendPlan::Full);
        assert_eq!(r.plan(P, Rev(5)), SendPlan::Nothing); // snapshot in flight
        let s = r.next_seq(P);
        r.record_sent(P, s, Rev(5));
        assert!(r.on_ack(P, s));
        assert_eq!(r.plan(P, Rev(5)), SendPlan::Nothing);
        assert_eq!(r.plan(P, Rev(9)), SendPlan::Delta { base: Rev(5) });
    }

    #[test]
    fn ack_moves_baseline_forward_only() {
        let mut r = BaselineRing::new(8);
        r.resume_from(P, Rev(1));
        let s1 = r.next_seq(P);
        r.record_sent(P, s1, Rev(10));
        let s2 = r.next_seq(P);
        r.record_sent(P, s2, Rev(20));
        assert!(r.on_ack(P, s2));
        assert_eq!(r.acked_rev(P), Some(Rev(20)));
        assert!(!r.on_ack(P, s1)); // dropped as older
        assert!(!r.on_ack(P, Seq(999)));
        assert_eq!(r.acked_rev(P), Some(Rev(20)));
    }

    #[test]
    fn aged_out_baseline_falls_back_to_full() {
        let mut r = BaselineRing::new(4);
        r.resume_from(P, Rev(1));
        for i in 0..4u64 {
            let s = r.next_seq(P);
            r.record_sent(P, s, Rev(10 + i));
        }
        assert_eq!(r.plan(P, Rev(100)), SendPlan::Full);
        assert_eq!(r.acked_rev(P), None);
        // an ack of an old send cannot resurrect the baseline
        assert!(!r.on_ack(P, Seq(0)));
    }

    #[test]
    fn acks_keep_ring_from_aging() {
        let mut r = BaselineRing::new(4);
        r.resume_from(P, Rev(0));
        for i in 1..100u64 {
            assert!(matches!(r.plan(P, Rev(i)), SendPlan::Delta { .. }));
            let s = r.next_seq(P);
            r.record_sent(P, s, Rev(i));
            r.on_ack(P, s);
        }
    }

    #[test]
    fn reset_keeps_sequence_counter() {
        let mut r = BaselineRing::new(4);
        let a = r.next_seq(P);
        r.reset(P);
        let b = r.next_seq(P);
        assert!(b.is_newer_than(a));
        assert_eq!(r.last_seq(P), b);
    }
}
