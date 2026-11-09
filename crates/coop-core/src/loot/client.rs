//! Guest side of container loot: optimistic edits that roll back when the
//! host says no. The visible contents are always the host-confirmed contents
//! with the still-pending local operations laid over them, so a rollback is
//! just dropping the operation.

use std::collections::BTreeMap;

use super::{digest, LootMsg};
use crate::ids::{FormId, PlayerId};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Op {
    txn: u32,
    take: bool,
    item: FormId,
    count: u32,
}

/// One container as this machine sees it.
#[derive(Clone, Debug, Default)]
pub struct LootView {
    confirmed: BTreeMap<FormId, u32>,
    rev: u32,
    pending: Vec<Op>,
    /// True while this machine holds the lease.
    pub leased: bool,
}

impl LootView {
    /// Host-confirmed revision.
    pub fn rev(&self) -> u32 {
        self.rev
    }

    /// Contents with pending operations applied.
    pub fn visible(&self) -> BTreeMap<FormId, u32> {
        let mut m = self.confirmed.clone();
        for op in &self.pending {
            let e = m.entry(op.item).or_insert(0);
            *e = if op.take {
                e.saturating_sub(op.count)
            } else {
                e.saturating_add(op.count)
            };
        }
        m.retain(|_, c| *c > 0);
        m
    }

    /// Count of one item as visible.
    pub fn count(&self, item: FormId) -> u32 {
        self.visible().get(&item).copied().unwrap_or(0)
    }

    /// Operations awaiting the host.
    pub fn pending(&self) -> usize {
        self.pending.len()
    }
}

/// Loot state for one machine.
#[derive(Clone, Debug, Default)]
pub struct LootClient {
    views: BTreeMap<FormId, LootView>,
    next_txn: u32,
    /// Containers refused as busy, with the holder.
    pub busy: BTreeMap<FormId, PlayerId>,
    /// Operations undone because the host refused them.
    pub rollbacks: u32,
    /// Net items gained per item from host-confirmed transactions.
    inv: BTreeMap<FormId, i64>,
    /// Drop claims won.
    pub drops_won: Vec<FormId>,
    /// Drop claims lost.
    pub drops_lost: Vec<FormId>,
}

impl LootClient {
    /// Empty client.
    pub fn new() -> Self {
        LootClient {
            next_txn: 1,
            ..Default::default()
        }
    }

    /// The container view, if opened.
    pub fn view(&self, container: FormId) -> Option<&LootView> {
        self.views.get(&container)
    }

    /// Asks for the lease.
    pub fn open(&self, container: FormId) -> LootMsg {
        LootMsg::Open { container }
    }

    /// Takes items optimistically; `None` when the visible stock is short or
    /// the container is not leased to us.
    pub fn take(&mut self, container: FormId, item: FormId, count: u32) -> Option<LootMsg> {
        let v = self.views.get_mut(&container).filter(|v| v.leased)?;
        if count == 0 || v.count(item) < count {
            return None;
        }
        let txn = self.next_txn;
        self.next_txn += 1;
        v.pending.push(Op {
            txn,
            take: true,
            item,
            count,
        });
        Some(LootMsg::Take {
            container,
            txn,
            item,
            count,
        })
    }

    /// Puts items optimistically (the game checks our own inventory).
    pub fn put(&mut self, container: FormId, item: FormId, count: u32) -> Option<LootMsg> {
        let v = self.views.get_mut(&container).filter(|v| v.leased)?;
        if count == 0 {
            return None;
        }
        let txn = self.next_txn;
        self.next_txn += 1;
        v.pending.push(Op {
            txn,
            take: false,
            item,
            count,
        });
        Some(LootMsg::Put {
            container,
            txn,
            item,
            count,
        })
    }

    /// Closes the container, quoting what we see.
    pub fn close(&mut self, container: FormId) -> Option<LootMsg> {
        let v = self.views.get_mut(&container).filter(|v| v.leased)?;
        v.leased = false;
        Some(LootMsg::Close {
            container,
            digest: digest(&v.visible()),
        })
    }

    /// Net amount of `item` moved into our inventory, pending operations
    /// included (what the inventory screen shows).
    pub fn inventory_delta(&self, item: FormId) -> i64 {
        let confirmed = self.inv.get(&item).copied().unwrap_or(0);
        let pending: i64 = self
            .views
            .values()
            .flat_map(|v| v.pending.iter())
            .filter(|o| o.item == item)
            .map(|o| {
                if o.take {
                    o.count as i64
                } else {
                    -(o.count as i64)
                }
            })
            .sum();
        confirmed + pending
    }

    /// Net amount of `item` moved into our inventory by confirmed
    /// transactions only.
    pub fn confirmed_inventory_delta(&self, item: FormId) -> i64 {
        self.inv.get(&item).copied().unwrap_or(0)
    }

    /// Applies a host message.
    pub fn on_msg(&mut self, msg: LootMsg) {
        match msg {
            LootMsg::Opened {
                container,
                rev,
                items,
            } => {
                let v = self.views.entry(container).or_default();
                self.busy.remove(&container);
                if v.leased && rev <= v.rev {
                    return; // a repeated grant must not wipe operations in flight
                }
                v.confirmed = items.into_iter().collect();
                v.rev = rev;
                if !v.leased {
                    v.pending.clear();
                }
                v.leased = true;
            }
            LootMsg::Busy { container, holder } => {
                self.busy.insert(container, holder);
            }
            LootMsg::TxnResult {
                container,
                txn,
                fail,
                rev,
                item,
                new_count,
            } => {
                let Some(v) = self.views.get_mut(&container) else {
                    return;
                };
                let Some(i) = v.pending.iter().position(|o| o.txn == txn) else {
                    return;
                };
                let op = v.pending.remove(i);
                match fail {
                    None => {
                        if rev >= v.rev {
                            v.confirmed.insert(item, new_count);
                            v.rev = rev;
                        }
                        let d = if op.take {
                            op.count as i64
                        } else {
                            -(op.count as i64)
                        };
                        *self.inv.entry(op.item).or_insert(0) += d;
                    }
                    Some(_) => {
                        self.rollbacks += 1;
                        if rev >= v.rev {
                            // the refusal reports the truth for that item
                            v.confirmed.insert(item, new_count);
                        }
                    }
                }
            }
            LootMsg::Changed {
                container,
                rev,
                item,
                new_count,
            } => {
                if let Some(v) = self.views.get_mut(&container) {
                    if rev > v.rev {
                        v.confirmed.insert(item, new_count);
                        v.rev = rev;
                    }
                }
            }
            LootMsg::Resync {
                container,
                rev,
                items,
            } => {
                if let Some(v) = self.views.get_mut(&container) {
                    v.confirmed = items.into_iter().collect();
                    v.rev = rev;
                    v.pending.clear();
                }
            }
            LootMsg::Released { container } => {
                if let Some(v) = self.views.get_mut(&container) {
                    v.leased = false;
                    // anything unanswered at this point is void
                    self.rollbacks += v.pending.len() as u32;
                    v.pending.clear();
                }
            }
            LootMsg::DropResult { drop, won } => {
                if won {
                    self.drops_won.push(drop);
                } else {
                    self.drops_lost.push(drop);
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BOX: FormId = FormId(0xB0);
    const CAPS: FormId = FormId(0xCA);

    fn opened(c: &mut LootClient, n: u32) {
        c.on_msg(LootMsg::Opened {
            container: BOX,
            rev: 1,
            items: vec![(CAPS, n)],
        });
    }

    #[test]
    fn take_is_optimistic_and_confirms() {
        let mut c = LootClient::new();
        opened(&mut c, 10);
        let Some(LootMsg::Take { txn, .. }) = c.take(BOX, CAPS, 4) else {
            panic!()
        };
        assert_eq!(c.view(BOX).unwrap().count(CAPS), 6, "shown at once");
        assert_eq!(c.inventory_delta(CAPS), 4);
        assert_eq!(c.confirmed_inventory_delta(CAPS), 0);
        c.on_msg(LootMsg::TxnResult {
            container: BOX,
            txn,
            fail: None,
            rev: 2,
            item: CAPS,
            new_count: 6,
        });
        assert_eq!(c.view(BOX).unwrap().count(CAPS), 6);
        assert_eq!(c.confirmed_inventory_delta(CAPS), 4);
        assert_eq!(c.view(BOX).unwrap().pending(), 0);
    }

    #[test]
    fn refusal_rolls_back_and_adopts_truth() {
        let mut c = LootClient::new();
        opened(&mut c, 10);
        let Some(LootMsg::Take { txn, .. }) = c.take(BOX, CAPS, 4) else {
            panic!()
        };
        c.on_msg(LootMsg::TxnResult {
            container: BOX,
            txn,
            fail: Some(crate::loot::TxnFail::Insufficient),
            rev: 3,
            item: CAPS,
            new_count: 1,
        });
        assert_eq!(c.rollbacks, 1);
        assert_eq!(c.inventory_delta(CAPS), 0);
        assert_eq!(c.view(BOX).unwrap().count(CAPS), 1);
    }

    #[test]
    fn pending_ops_survive_a_foreign_change_in_order() {
        let mut c = LootClient::new();
        opened(&mut c, 10);
        c.take(BOX, CAPS, 3).unwrap();
        c.on_msg(LootMsg::Changed {
            container: BOX,
            rev: 2,
            item: CAPS,
            new_count: 8,
        });
        assert_eq!(
            c.view(BOX).unwrap().count(CAPS),
            5,
            "host 8 minus our pending 3"
        );
        c.on_msg(LootMsg::Changed {
            container: BOX,
            rev: 2,
            item: CAPS,
            new_count: 1,
        });
        assert_eq!(
            c.view(BOX).unwrap().count(CAPS),
            5,
            "same or older revision ignored"
        );
    }

    #[test]
    fn cannot_take_what_is_not_visible_or_unleased() {
        let mut c = LootClient::new();
        assert!(c.take(BOX, CAPS, 1).is_none());
        opened(&mut c, 2);
        assert!(c.take(BOX, CAPS, 3).is_none());
        c.take(BOX, CAPS, 2).unwrap();
        assert!(
            c.take(BOX, CAPS, 1).is_none(),
            "pending take already claimed it"
        );
    }

    #[test]
    fn close_quotes_visible_digest_and_resync_replaces() {
        let mut c = LootClient::new();
        opened(&mut c, 5);
        let Some(LootMsg::Close { digest: d, .. }) = c.close(BOX) else {
            panic!()
        };
        assert_eq!(d, digest(&[(CAPS, 5)].into_iter().collect()));
        assert!(c.close(BOX).is_none());
        c.on_msg(LootMsg::Resync {
            container: BOX,
            rev: 9,
            items: vec![(CAPS, 2)],
        });
        assert_eq!(c.view(BOX).unwrap().rev(), 9);
        assert_eq!(c.view(BOX).unwrap().count(CAPS), 2);
    }

    #[test]
    fn lease_loss_voids_pending() {
        let mut c = LootClient::new();
        opened(&mut c, 5);
        c.take(BOX, CAPS, 2).unwrap();
        c.on_msg(LootMsg::Released { container: BOX });
        assert_eq!(c.rollbacks, 1);
        assert_eq!(c.view(BOX).unwrap().count(CAPS), 5);
        assert!(c.take(BOX, CAPS, 1).is_none());
    }

    #[test]
    fn repeated_grant_keeps_operations_in_flight() {
        let mut c = LootClient::new();
        opened(&mut c, 10);
        let Some(LootMsg::Take { txn, .. }) = c.take(BOX, CAPS, 4) else {
            panic!()
        };
        opened(&mut c, 10); // the host answers a duplicate Open with the same grant
        assert_eq!(c.view(BOX).unwrap().pending(), 1);
        c.on_msg(LootMsg::TxnResult {
            container: BOX,
            txn,
            fail: None,
            rev: 2,
            item: CAPS,
            new_count: 6,
        });
        assert_eq!(c.confirmed_inventory_delta(CAPS), 4);
    }

    #[test]
    fn busy_is_recorded_then_cleared_on_open() {
        let mut c = LootClient::new();
        c.on_msg(LootMsg::Busy {
            container: BOX,
            holder: PlayerId(2),
        });
        assert_eq!(c.busy[&BOX], PlayerId(2));
        opened(&mut c, 1);
        assert!(c.busy.is_empty());
    }
}
