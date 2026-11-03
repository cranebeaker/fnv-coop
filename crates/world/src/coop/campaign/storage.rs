//! Personal storage (G35): a container a row lists under `storage` holds one
//! inventory per player.
//!
//! Two paths use it:
//!
//! * scripts and menus run as a player (`as_player`): the contents of the
//!   container are swapped with the shadow (`extra::swap`), so
//!   `RemoveAllItems` into the container strips only that player and the
//!   return trigger gives back only that player's things;
//! * the loot ledger (guests' container screen): the container id is
//!   replaced by [`personal_id`], a stable per-slot holder, before it
//!   reaches the ledger ([`redirect_op`]).

use std::sync::Arc;

use coop_policy::PolicyTable;
use esm::{FormId, LoadOrder};

use super::Campaign;
use crate::coop::facts::LootOp;
use crate::scripting::GameState;

/// First id of the synthetic per-slot holders (clear of puppets, shadows and
/// placed references).
pub const PERSONAL_BASE: u32 = 0xFA00_0000;

/// The holder of `container` for `slot`: stable on every machine.
pub fn personal_id(container: FormId, slot: u8) -> FormId {
    let h = container.0.wrapping_mul(0x9E37_79B1).rotate_left(11)
        ^ (u32::from(slot) << 20)
        ^ container.0;
    FormId(PERSONAL_BASE | (h & 0x00FF_FFFF))
}

/// Resolves the rows' storage lists.
pub fn configure(camp: &mut Campaign, order: &LoadOrder, table: &Arc<PolicyTable>) {
    for row in table.rows() {
        for s in &row.storage {
            if let Some(f) = super::owners::resolve(order, s) {
                camp.personal.insert(f);
            }
        }
    }
}

impl Campaign {
    /// True when `container` is personal.
    pub fn is_personal(&self, container: FormId) -> bool {
        self.personal.contains(&container)
    }
}

/// The holder a container action of `slot` really touches.
pub fn redirect(gs: &GameState, container: FormId, slot: u8) -> FormId {
    if gs.coop.campaign.is_personal(container) {
        personal_id(container, slot)
    } else {
        container
    }
}

/// Rewrites a container action to the player's own holder.
pub fn redirect_op(gs: &GameState, slot: u8, op: LootOp) -> LootOp {
    let r = |c: FormId| redirect(gs, c, slot);
    match op {
        LootOp::Open(c) => LootOp::Open(r(c)),
        LootOp::Close(c) => LootOp::Close(r(c)),
        LootOp::Take { container, item, n } => LootOp::Take {
            container: r(container),
            item,
            n,
        },
        LootOp::Put { container, item, n } => LootOp::Put {
            container: r(container),
            item,
            n,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn holders_differ_per_slot_and_container() {
        let c = FormId(0x0100_2233);
        let ids: Vec<FormId> = (0..8).map(|s| personal_id(c, s)).collect();
        for i in 0..8 {
            for j in i + 1..8 {
                assert_ne!(ids[i], ids[j]);
            }
            assert!(ids[i].0 >= PERSONAL_BASE);
        }
        assert_ne!(personal_id(c, 1), personal_id(FormId(0x0100_2234), 1));
        assert_eq!(personal_id(c, 3), personal_id(c, 3));
    }

    #[test]
    fn only_personal_containers_are_redirected() {
        let mut gs = GameState::default();
        gs.coop.campaign.personal.insert(FormId(7));
        assert_eq!(redirect(&gs, FormId(7), 2), personal_id(FormId(7), 2));
        assert_eq!(redirect(&gs, FormId(8), 2), FormId(8));
        assert_eq!(
            redirect_op(&gs, 1, LootOp::Open(FormId(8))),
            LootOp::Open(FormId(8))
        );
    }
}
