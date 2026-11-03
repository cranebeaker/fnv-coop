//! The local player's container actions through the live layer.
//!
//! The viewer's container screen moves the host player's items itself and
//! only reports `Open` / `Close`; a guest's screen only asks. A script or the
//! test bot (`coop loottake` ...) has no screen, so on the host
//! [`Session::live_loot`] does what the screen does: register the container on
//! first open, and move the items with [`GameState::move_item`]. The host's
//! ledger follows the world edit on its next reconcile.

use world::coop::apply::Ctx;
use world::coop::facts::LootOp;
use world::dialogue::PLAYER_REF;
use world::scripting::GameState;

use crate::{Intent, Role, Session};

impl Session {
    /// Carries one container action of the local player.
    pub fn live_loot(&mut self, cx: Ctx, gs: &mut GameState, op: LootOp) {
        let host = self.role == Role::Host;
        // a personal container is this player's own holder in the ledger
        let op = world::coop::campaign::storage::redirect_op(gs, gs.active_slot(), op);
        match op {
            LootOp::Open(c) => {
                if host {
                    self.live_register_container(cx, gs, c);
                }
                self.live_intent(Intent::LootOpen(c));
            }
            LootOp::Take { container, item, n } if host => {
                gs.move_item(cx.order, container, PLAYER_REF, item, n as i32);
            }
            LootOp::Put { container, item, n } if host => {
                gs.move_item(cx.order, PLAYER_REF, container, item, n as i32);
            }
            LootOp::Take { container, item, n } => {
                self.live_intent(Intent::LootTake { container, item, n })
            }
            LootOp::Put { container, item, n } => {
                self.live_intent(Intent::LootPut { container, item, n })
            }
            LootOp::Close(c) => self.live_intent(Intent::LootClose(c)),
        }
    }

    /// Carries every container action the local player queued in `gs`.
    pub fn live_loot_ops(&mut self, cx: Ctx, gs: &mut GameState) {
        for op in std::mem::take(&mut gs.coop.live.loot_ops) {
            self.live_loot(cx, gs, op);
        }
    }
}
