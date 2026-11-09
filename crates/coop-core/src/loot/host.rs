//! Host side of container loot.

use std::collections::{BTreeMap, BTreeSet};

use super::{digest, LootMsg, TransferLedger, TxnFail, LEASE_IDLE_MS};
use crate::ids::{CellId, FormId, PlayerId};
use crate::interest::Audience;
use crate::once::OnceMap;

/// One addressed output message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LootOut {
    /// Recipients.
    pub aud: Audience,
    /// Message.
    pub msg: LootMsg,
}

fn out(aud: Audience, msg: LootMsg) -> LootOut {
    LootOut { aud, msg }
}

#[derive(Clone, Debug)]
struct Container {
    cell: CellId,
    items: BTreeMap<FormId, u32>,
    rev: u32,
    lease: Option<(PlayerId, u64)>,
}

type TxnReply = LootMsg;

/// The authoritative container table.
#[derive(Clone, Debug)]
pub struct LootHost {
    containers: BTreeMap<FormId, Container>,
    contents: OnceMap<FormId, ()>,
    ledger: TransferLedger<TxnReply>,
    drops: BTreeMap<FormId, (CellId, Option<PlayerId>)>,
    taken: BTreeSet<FormId>,
}

impl Default for LootHost {
    fn default() -> Self {
        LootHost {
            containers: BTreeMap::new(),
            contents: OnceMap::new(),
            ledger: TransferLedger::new(256),
            drops: BTreeMap::new(),
            taken: BTreeSet::new(),
        }
    }
}

impl LootHost {
    /// Empty table.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a container's contents. The first call per container wins
    /// (leveled loot is rolled once; later calls, for example from another
    /// machine's roll, change nothing). Returns true when this call won.
    pub fn register(
        &mut self,
        container: FormId,
        cell: CellId,
        roll: impl FnOnce() -> Vec<(FormId, u32)>,
    ) -> bool {
        let (_, won) = self.contents.resolve(container, ());
        if won {
            let mut items = BTreeMap::new();
            for (f, c) in roll() {
                *items.entry(f).or_insert(0u32) += c;
            }
            self.containers.insert(
                container,
                Container {
                    cell,
                    items,
                    rev: 1,
                    lease: None,
                },
            );
        }
        won
    }

    /// Contents of a container.
    pub fn items(&self, container: FormId) -> Option<&BTreeMap<FormId, u32>> {
        self.containers.get(&container).map(|c| &c.items)
    }

    /// Current lease holder.
    pub fn holder(&self, container: FormId) -> Option<PlayerId> {
        self.containers
            .get(&container)
            .and_then(|c| c.lease.map(|(p, _)| p))
    }

    /// Registers a dropped item that players may claim.
    pub fn register_drop(&mut self, drop: FormId, cell: CellId) {
        if !self.taken.contains(&drop) {
            self.drops.entry(drop).or_insert((cell, None));
        }
    }

    /// Host-side edit (script or world change). Every interested peer,
    /// including the lease holder, is told.
    pub fn host_set(&mut self, container: FormId, item: FormId, count: u32) -> Vec<LootOut> {
        let Some(c) = self.containers.get_mut(&container) else {
            return vec![];
        };
        c.items.insert(item, count);
        c.rev += 1;
        vec![out(
            Audience::Cell(c.cell),
            LootMsg::Changed {
                container,
                rev: c.rev,
                item,
                new_count: count,
            },
        )]
    }

    /// Handles one message from `from`.
    pub fn on_msg(&mut self, from: PlayerId, msg: LootMsg, now_ms: u64) -> Vec<LootOut> {
        match msg {
            LootMsg::Open { container } => self.open(from, container, now_ms),
            LootMsg::Take {
                container,
                txn,
                item,
                count,
            } => self.txn(from, container, txn, item, count, true, now_ms),
            LootMsg::Put {
                container,
                txn,
                item,
                count,
            } => self.txn(from, container, txn, item, count, false, now_ms),
            LootMsg::Close {
                container,
                digest: theirs,
            } => self.close(from, container, theirs),
            LootMsg::ClaimDrop { drop } => self.claim(from, drop),
            _ => vec![],
        }
    }

    fn open(&mut self, from: PlayerId, container: FormId, now_ms: u64) -> Vec<LootOut> {
        let Some(c) = self.containers.get_mut(&container) else {
            return vec![];
        };
        match c.lease {
            Some((h, _)) if h != from => {
                vec![out(
                    Audience::To(from),
                    LootMsg::Busy {
                        container,
                        holder: h,
                    },
                )]
            }
            _ => {
                c.lease = Some((from, now_ms));
                let items = c
                    .items
                    .iter()
                    .filter(|(_, n)| **n > 0)
                    .map(|(f, n)| (*f, *n))
                    .collect();
                vec![out(
                    Audience::To(from),
                    LootMsg::Opened {
                        container,
                        rev: c.rev,
                        items,
                    },
                )]
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn txn(
        &mut self,
        from: PlayerId,
        container: FormId,
        txn: u32,
        item: FormId,
        count: u32,
        take: bool,
        now_ms: u64,
    ) -> Vec<LootOut> {
        let containers = &mut self.containers;
        let res = self.ledger.once(from, txn, || {
            let fail = |why, rev| LootMsg::TxnResult {
                container,
                txn,
                fail: Some(why),
                rev,
                item,
                new_count: 0,
            };
            let Some(c) = containers.get_mut(&container) else {
                return fail(TxnFail::Invalid, 0);
            };
            let have = c.items.get(&item).copied().unwrap_or(0);
            if count == 0 {
                return fail(TxnFail::Invalid, c.rev);
            }
            if c.lease.map(|(p, _)| p) != Some(from) {
                return LootMsg::TxnResult {
                    container,
                    txn,
                    fail: Some(TxnFail::NoLease),
                    rev: c.rev,
                    item,
                    new_count: have,
                };
            }
            let new_count = if take {
                if have < count {
                    return LootMsg::TxnResult {
                        container,
                        txn,
                        fail: Some(TxnFail::Insufficient),
                        rev: c.rev,
                        item,
                        new_count: have,
                    };
                }
                have - count
            } else {
                have.saturating_add(count)
            };
            c.items.insert(item, new_count);
            c.rev += 1;
            c.lease = Some((from, now_ms));
            LootMsg::TxnResult {
                container,
                txn,
                fail: None,
                rev: c.rev,
                item,
                new_count,
            }
        });
        let Some((reply, fresh)) = res else {
            return vec![];
        };
        let mut v = vec![out(Audience::To(from), reply.clone())];
        if let (
            true,
            LootMsg::TxnResult {
                fail: None,
                rev,
                new_count,
                ..
            },
        ) = (fresh, &reply)
        {
            if let Some(c) = self.containers.get(&container) {
                v.push(out(
                    Audience::CellExcept(c.cell, from),
                    LootMsg::Changed {
                        container,
                        rev: *rev,
                        item,
                        new_count: *new_count,
                    },
                ));
            }
        }
        v
    }

    fn close(&mut self, from: PlayerId, container: FormId, theirs: u64) -> Vec<LootOut> {
        let Some(c) = self.containers.get_mut(&container) else {
            return vec![];
        };
        if c.lease.map(|(p, _)| p) != Some(from) {
            return vec![];
        }
        c.lease = None;
        if digest(&c.items) != theirs {
            let items = c
                .items
                .iter()
                .filter(|(_, n)| **n > 0)
                .map(|(f, n)| (*f, *n))
                .collect();
            return vec![out(
                Audience::To(from),
                LootMsg::Resync {
                    container,
                    rev: c.rev,
                    items,
                },
            )];
        }
        vec![]
    }

    fn claim(&mut self, from: PlayerId, drop: FormId) -> Vec<LootOut> {
        match self.drops.get(&drop).copied() {
            None => vec![out(
                Audience::To(from),
                LootMsg::DropResult { drop, won: false },
            )],
            Some((cell, None)) => {
                self.drops.insert(drop, (cell, Some(from)));
                self.taken.insert(drop);
                vec![
                    out(Audience::To(from), LootMsg::DropResult { drop, won: true }),
                    out(Audience::CellExcept(cell, from), LootMsg::DropGone { drop }),
                ]
            }
            Some((_, Some(w))) => vec![out(
                Audience::To(from),
                LootMsg::DropResult {
                    drop,
                    won: w == from,
                },
            )],
        }
    }

    /// A player left: its leases are released and its txn space reset.
    pub fn on_disconnect(&mut self, p: PlayerId) {
        for c in self.containers.values_mut() {
            if c.lease.map(|(h, _)| h) == Some(p) {
                c.lease = None;
            }
        }
        self.ledger.forget(p);
    }

    /// Takes back idle leases.
    pub fn tick(&mut self, now_ms: u64) -> Vec<LootOut> {
        let mut v = Vec::new();
        for (id, c) in self.containers.iter_mut() {
            if let Some((h, at)) = c.lease {
                if now_ms.saturating_sub(at) >= LEASE_IDLE_MS {
                    c.lease = None;
                    v.push(out(Audience::To(h), LootMsg::Released { container: *id }));
                }
            }
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BOX: FormId = FormId(0xB0);
    const CAPS: FormId = FormId(0xCA);
    const C: CellId = CellId(1);
    const A: PlayerId = PlayerId(1);
    const B: PlayerId = PlayerId(2);

    fn host() -> LootHost {
        let mut h = LootHost::new();
        h.register(BOX, C, || vec![(CAPS, 10), (FormId(2), 1)]);
        h
    }
    fn take(h: &mut LootHost, who: PlayerId, txn: u32, n: u32, now: u64) -> Vec<LootOut> {
        h.on_msg(
            who,
            LootMsg::Take {
                container: BOX,
                txn,
                item: CAPS,
                count: n,
            },
            now,
        )
    }
    fn fail_of(o: &[LootOut]) -> Option<TxnFail> {
        match o[0].msg {
            LootMsg::TxnResult { fail, .. } => fail,
            ref m => panic!("{m:?}"),
        }
    }

    #[test]
    fn contents_resolve_once() {
        let mut h = host();
        assert!(!h.register(BOX, C, || vec![(CAPS, 999)]));
        assert_eq!(h.items(BOX).unwrap()[&CAPS], 10);
    }

    #[test]
    fn one_lease_holder_busy_for_others() {
        let mut h = host();
        assert!(matches!(
            h.on_msg(A, LootMsg::Open { container: BOX }, 0)[0].msg,
            LootMsg::Opened { .. }
        ));
        assert!(matches!(
            h.on_msg(B, LootMsg::Open { container: BOX }, 1)[0].msg,
            LootMsg::Busy { holder: A, .. }
        ));
        assert!(
            matches!(
                h.on_msg(A, LootMsg::Open { container: BOX }, 2)[0].msg,
                LootMsg::Opened { .. }
            ),
            "reopen is idempotent"
        );
        h.on_msg(
            A,
            LootMsg::Close {
                container: BOX,
                digest: digest(h.items(BOX).unwrap()),
            },
            3,
        );
        assert!(matches!(
            h.on_msg(B, LootMsg::Open { container: BOX }, 4)[0].msg,
            LootMsg::Opened { .. }
        ));
    }

    #[test]
    fn take_needs_lease_and_stock() {
        let mut h = host();
        assert_eq!(fail_of(&take(&mut h, A, 1, 1, 0)), Some(TxnFail::NoLease));
        h.on_msg(A, LootMsg::Open { container: BOX }, 0);
        assert_eq!(
            fail_of(&take(&mut h, A, 2, 11, 1)),
            Some(TxnFail::Insufficient)
        );
        assert_eq!(fail_of(&take(&mut h, A, 3, 4, 2)), None);
        assert_eq!(h.items(BOX).unwrap()[&CAPS], 6);
        assert_eq!(fail_of(&take(&mut h, A, 4, 0, 2)), Some(TxnFail::Invalid));
    }

    #[test]
    fn retransmitted_take_applies_once_and_replays_the_result() {
        let mut h = host();
        h.on_msg(A, LootMsg::Open { container: BOX }, 0);
        let first = take(&mut h, A, 1, 4, 1);
        let again = take(&mut h, A, 1, 4, 2);
        assert_eq!(first.len(), 2, "result plus change broadcast");
        assert_eq!(again.len(), 1, "result only, no second broadcast");
        assert_eq!(first[0].msg, again[0].msg);
        assert_eq!(h.items(BOX).unwrap()[&CAPS], 6);
    }

    #[test]
    fn put_adds_items() {
        let mut h = host();
        h.on_msg(A, LootMsg::Open { container: BOX }, 0);
        h.on_msg(
            A,
            LootMsg::Put {
                container: BOX,
                txn: 1,
                item: FormId(9),
                count: 3,
            },
            1,
        );
        assert_eq!(h.items(BOX).unwrap()[&FormId(9)], 3);
    }

    #[test]
    fn digest_mismatch_resyncs_on_close() {
        let mut h = host();
        h.on_msg(A, LootMsg::Open { container: BOX }, 0);
        let o = h.on_msg(
            A,
            LootMsg::Close {
                container: BOX,
                digest: 12345,
            },
            1,
        );
        assert!(matches!(o[0].msg, LootMsg::Resync { rev: 1, .. }));
        assert!(h.holder(BOX).is_none());
        assert!(
            h.on_msg(
                B,
                LootMsg::Close {
                    container: BOX,
                    digest: 0
                },
                1
            )
            .is_empty(),
            "non-holder close ignored"
        );
    }

    #[test]
    fn idle_lease_expires_and_disconnect_frees_it() {
        let mut h = host();
        h.on_msg(A, LootMsg::Open { container: BOX }, 0);
        assert!(h.tick(LEASE_IDLE_MS - 1).is_empty());
        assert!(matches!(
            h.tick(LEASE_IDLE_MS)[0].msg,
            LootMsg::Released { .. }
        ));
        h.on_msg(B, LootMsg::Open { container: BOX }, LEASE_IDLE_MS);
        h.on_disconnect(B);
        assert!(h.holder(BOX).is_none());
    }

    #[test]
    fn host_edit_bumps_revision() {
        let mut h = host();
        let o = h.host_set(BOX, CAPS, 0);
        assert!(matches!(
            o[0].msg,
            LootMsg::Changed {
                rev: 2,
                new_count: 0,
                ..
            }
        ));
    }

    #[test]
    fn drop_claims_first_wins_and_are_idempotent() {
        let mut h = host();
        let d = FormId(0xD0);
        h.register_drop(d, C);
        let a = h.on_msg(A, LootMsg::ClaimDrop { drop: d }, 0);
        let b = h.on_msg(B, LootMsg::ClaimDrop { drop: d }, 0);
        assert!(matches!(a[0].msg, LootMsg::DropResult { won: true, .. }));
        assert!(matches!(a[1].msg, LootMsg::DropGone { .. }));
        assert!(matches!(b[0].msg, LootMsg::DropResult { won: false, .. }));
        assert!(matches!(
            h.on_msg(A, LootMsg::ClaimDrop { drop: d }, 1)[0].msg,
            LootMsg::DropResult { won: true, .. }
        ));
        h.register_drop(d, C);
        assert!(matches!(
            h.on_msg(B, LootMsg::ClaimDrop { drop: d }, 2)[0].msg,
            LootMsg::DropResult { won: false, .. }
        ));
    }
}
