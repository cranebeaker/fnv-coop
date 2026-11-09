//! Container loot: one lease holder per container, optimistic client edits
//! with rollback, an exactly-once transaction ledger, and first-wins claims
//! on dropped items.

mod client;
mod host;

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use xxhash_rust::xxh3::xxh3_64;

use crate::ids::{FormId, PlayerId};

pub use client::{LootClient, LootView};
pub use host::{LootHost, LootOut};

/// Idle time after which a lease is taken back.
pub const LEASE_IDLE_MS: u64 = 300_000;
/// Most item entries in one container message.
pub const MAX_ITEMS: usize = 512;

/// Why a transaction failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TxnFail {
    /// Sender does not hold the lease.
    NoLease,
    /// Not enough of the item in the container.
    Insufficient,
    /// Unknown container or zero count.
    Invalid,
}

/// Loot wire messages (reliable).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LootMsg {
    /// Ask for the lease and the contents.
    Open {
        /// Container.
        container: FormId,
    },
    /// Lease granted; contents at `rev`.
    Opened {
        /// Container.
        container: FormId,
        /// Contents revision.
        rev: u32,
        /// Item counts.
        items: Vec<(FormId, u32)>,
    },
    /// Someone else holds the lease.
    Busy {
        /// Container.
        container: FormId,
        /// Holder.
        holder: PlayerId,
    },
    /// Remove items from the container into the sender's inventory.
    Take {
        /// Container.
        container: FormId,
        /// Per-sender transaction id, applied exactly once.
        txn: u32,
        /// Item.
        item: FormId,
        /// Count.
        count: u32,
    },
    /// Move items from the sender's inventory into the container.
    Put {
        /// Container.
        container: FormId,
        /// Per-sender transaction id.
        txn: u32,
        /// Item.
        item: FormId,
        /// Count.
        count: u32,
    },
    /// Host verdict on a transaction.
    TxnResult {
        /// Container.
        container: FormId,
        /// Transaction.
        txn: u32,
        /// Failure reason, `None` on success.
        fail: Option<TxnFail>,
        /// Container revision after the transaction.
        rev: u32,
        /// Item affected.
        item: FormId,
        /// Count of that item in the container afterwards.
        new_count: u32,
    },
    /// Host: someone else changed the container.
    Changed {
        /// Container.
        container: FormId,
        /// New revision.
        rev: u32,
        /// Item.
        item: FormId,
        /// Count afterwards.
        new_count: u32,
    },
    /// Release the lease, quoting the contents digest the sender sees.
    Close {
        /// Container.
        container: FormId,
        /// [`digest`] of the sender's view.
        digest: u64,
    },
    /// Host: your view differs; here is the truth.
    Resync {
        /// Container.
        container: FormId,
        /// Revision.
        rev: u32,
        /// Items.
        items: Vec<(FormId, u32)>,
    },
    /// Host: your lease ended (idle timeout or revoked).
    Released {
        /// Container.
        container: FormId,
    },
    /// Claim a dropped item; the first claim wins.
    ClaimDrop {
        /// The dropped ref.
        drop: FormId,
    },
    /// Host verdict on a claim.
    DropResult {
        /// The dropped ref.
        drop: FormId,
        /// True when the claimant got it.
        won: bool,
    },
    /// Host: the dropped ref was taken.
    DropGone {
        /// The dropped ref.
        drop: FormId,
    },
}

impl LootMsg {
    /// Structural bounds.
    pub fn is_sane(&self) -> bool {
        match self {
            LootMsg::Opened { items, .. } | LootMsg::Resync { items, .. } => {
                items.len() <= MAX_ITEMS
            }
            _ => true,
        }
    }
}

/// Order-independent digest of container contents (zero counts ignored).
pub fn digest(items: &BTreeMap<FormId, u32>) -> u64 {
    let mut buf = Vec::with_capacity(items.len() * 8);
    for (f, c) in items.iter().filter(|(_, c)| **c > 0) {
        buf.extend_from_slice(&f.0.to_le_bytes());
        buf.extend_from_slice(&c.to_le_bytes());
    }
    xxh3_64(&buf)
}

/// Exactly-once application of per-player transactions: a transaction id at
/// or below the high-water mark is never applied again, and its cached result
/// (if still remembered) is replayed to the sender.
#[derive(Clone, Debug)]
pub struct TransferLedger<R: Clone> {
    last: BTreeMap<PlayerId, u32>,
    cache: BTreeMap<(PlayerId, u32), R>,
    cap: usize,
}

impl<R: Clone> TransferLedger<R> {
    /// A ledger remembering up to `cap` results.
    pub fn new(cap: usize) -> Self {
        TransferLedger {
            last: BTreeMap::new(),
            cache: BTreeMap::new(),
            cap,
        }
    }

    /// Runs `apply` unless `(player, txn)` was seen. Returns the result and
    /// whether it was freshly applied. A replay older than the cache yields
    /// `None`.
    pub fn once(
        &mut self,
        player: PlayerId,
        txn: u32,
        apply: impl FnOnce() -> R,
    ) -> Option<(R, bool)> {
        let last = self.last.get(&player).copied().unwrap_or(0);
        if txn <= last {
            return self.cache.get(&(player, txn)).map(|r| (r.clone(), false));
        }
        let r = apply();
        self.last.insert(player, txn);
        self.cache.insert((player, txn), r.clone());
        while self.cache.len() > self.cap {
            let first = *self.cache.keys().next().expect("non-empty");
            self.cache.remove(&first);
        }
        Some((r, true))
    }

    /// Forgets a player (disconnect and rejoin start a fresh txn space).
    pub fn forget(&mut self, player: PlayerId) {
        self.last.remove(&player);
        self.cache.retain(|(p, _), _| *p != player);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn digest_ignores_zero_and_order() {
        let a: BTreeMap<_, _> = [(FormId(1), 2), (FormId(2), 0), (FormId(3), 4)]
            .into_iter()
            .collect();
        let b: BTreeMap<_, _> = [(FormId(3), 4), (FormId(1), 2)].into_iter().collect();
        assert_eq!(digest(&a), digest(&b));
        let c: BTreeMap<_, _> = [(FormId(3), 4), (FormId(1), 3)].into_iter().collect();
        assert_ne!(digest(&a), digest(&c));
    }

    #[test]
    fn ledger_applies_each_txn_once() {
        let mut l = TransferLedger::new(8);
        let p = PlayerId(1);
        assert_eq!(l.once(p, 1, || 10), Some((10, true)));
        assert_eq!(l.once(p, 1, || 99), Some((10, false)));
        assert_eq!(l.once(p, 2, || 20), Some((20, true)));
        assert_eq!(l.once(p, 1, || 99), Some((10, false)), "old but cached");
        assert_eq!(l.once(PlayerId(2), 1, || 5), Some((5, true)), "per player");
    }

    #[test]
    fn ledger_never_reapplies_even_after_eviction() {
        let mut l = TransferLedger::new(2);
        let p = PlayerId(1);
        for t in 1..=5 {
            l.once(p, t, || t);
        }
        assert_eq!(l.once(p, 1, || panic!("must not apply")), None);
        l.forget(p);
        assert_eq!(l.once(p, 1, || 7), Some((7, true)));
    }

    proptest! {
        #[test]
        fn replays_in_any_order_apply_exactly_once(txns in proptest::collection::vec(1u32..30, 1..120)) {
            let mut l = TransferLedger::new(16);
            let mut applied = BTreeMap::<u32, u32>::new();
            for t in &txns {
                l.once(PlayerId(1), *t, || { *applied.entry(*t).or_default() += 1; 0 });
            }
            prop_assert!(applied.values().all(|n| *n == 1));
        }
    }
}
