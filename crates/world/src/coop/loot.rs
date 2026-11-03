//! Containers and drops through `LootHost` / `LootClient`, on `GameState`
//! items.
//!
//! Host: the host's world edits containers natively; [`HostLoot::reconcile`]
//! keeps the ledger and `items` equal in whichever direction changed since
//! last time (a guest transaction flows into the world, a script or the host
//! player's own move flows into the ledger and out to the peers).
//! Guest: [`GuestLoot`] sends open/take/put/close and mirrors the confirmed
//! view into the guest's `items`.

use std::collections::{BTreeMap, BTreeSet};

use coop_core::ids::{CellId, FormId as CForm, PlayerId};
use coop_core::loot::{digest, LootClient, LootMsg};
use coop_core::proto::Msg;
use coop_core::sync::HostLive;
use esm::FormId;

use super::apply::Ctx;
use super::players::PlayerRegistry;
use crate::dialogue::PLAYER_REF;
use crate::scripting::GameState;

fn cf(f: FormId) -> CForm {
    CForm(f.0)
}

fn wf(f: CForm) -> FormId {
    FormId(f.0)
}

fn world_items(gs: &GameState, container: FormId) -> BTreeMap<CForm, u32> {
    gs.items
        .iter()
        .filter(|((h, _), n)| *h == container && **n > 0)
        .map(|((_, i), n)| (cf(*i), *n as u32))
        .collect()
}

/// Puts what a guest took (or gave) into the host's authoritative copy of that guest's
/// inventory. The guest's own rows are overwritten from this copy on every sync, so a
/// loot credited only to the ledger and to the guest's local rows vanishes again.
fn credit_shadow(gs: &mut GameState, p: PlayerId, item: FormId, delta: i64) {
    if !gs.coop.bank.enabled() || delta == 0 {
        return;
    }
    gs.with_player(p.0, |g| {
        let e = g.items.entry((PLAYER_REF, item)).or_insert(0);
        *e = (i64::from(*e) + delta).clamp(0, i64::from(i32::MAX)) as i32;
        if *e == 0 {
            g.items.remove(&(PLAYER_REF, item));
        }
        g.emit(super::Domain::Items, PLAYER_REF);
    });
}

/// Host side of the loot domain.
#[derive(Default)]
pub struct HostLoot {
    containers: BTreeSet<FormId>,
    seen: BTreeMap<FormId, u64>,
    holder: BTreeMap<FormId, PlayerId>,
}

impl HostLoot {
    /// A fresh bridge.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a container (once per container; later calls change
    /// nothing). The supplies of a dead body are thinned by the party's reward
    /// scaling, once, and the thinned contents are written back to the world.
    /// A living person's stock (a vendor, Doc Mitchell) and a chest are not
    /// thinned: that would delete their items from the world.
    pub fn register(
        &mut self,
        cx: Ctx,
        gs: &mut GameState,
        live: &mut HostLive,
        container: FormId,
    ) -> bool {
        gs.stock(cx.order, container);
        let cell = gs.place(cx.order, container).map_or(0, |p| p.1 .0);
        let scaling = gs
            .coop
            .live
            .scaling
            .filter(|_| gs.dead.contains(&container));
        let items = gs.inventory(cx.order, container);
        let kept: Vec<(CForm, u32)> = items
            .iter()
            .filter(|(i, _)| {
                scaling.map_or(true, |s| {
                    s.keeps_loot(CForm(container.0 ^ i.0.rotate_left(13)))
                })
            })
            .map(|(i, n)| (cf(*i), *n as u32))
            .collect();
        let won = live
            .loot
            .register(cf(container), CellId(cell), || kept.clone());
        if won {
            for (i, _) in &items {
                if !kept.iter().any(|(k, _)| wf(*k) == *i) {
                    gs.items.remove(&(container, *i));
                }
            }
            self.containers.insert(container);
            self.seen
                .insert(container, digest(&world_items(gs, container)));
        }
        won
    }

    /// Registers a dropped item for first-come claims.
    pub fn register_drop(&mut self, cx: Ctx, gs: &GameState, live: &mut HostLive, drop: FormId) {
        let cell = gs.place(cx.order, drop).map_or(0, |p| p.1 .0);
        live.loot.register_drop(cf(drop), CellId(cell));
    }

    /// Makes ledger and world agree; returns the sends.
    pub fn reconcile(
        &mut self,
        gs: &mut GameState,
        live: &mut HostLive,
        reg: &mut PlayerRegistry,
    ) -> Vec<(PlayerId, Msg)> {
        let mut outs = Vec::new();
        for &c in &self.containers.clone() {
            if let Some(h) = live.loot.holder(cf(c)) {
                self.holder.insert(c, h);
            }
            let Some(ledger) = live.loot.items(cf(c)).cloned() else {
                continue;
            };
            let world = world_items(gs, c);
            let (dl, dw) = (digest(&ledger), digest(&world));
            let seen = self.seen.get(&c).copied().unwrap_or(dl);
            if dl != seen {
                // a guest transaction: the world follows the ledger
                let who = self.holder.get(&c).copied();
                for item in ledger
                    .keys()
                    .chain(world.keys())
                    .copied()
                    .collect::<BTreeSet<_>>()
                {
                    let (have, want) = (
                        world.get(&item).copied().unwrap_or(0),
                        ledger.get(&item).copied().unwrap_or(0),
                    );
                    if have == want {
                        continue;
                    }
                    if want == 0 {
                        gs.items.remove(&(c, wf(item)));
                    } else {
                        gs.items.insert((c, wf(item)), want as i32);
                    }
                    gs.emit(super::Domain::Items, c);
                    if let Some(p) = who.filter(|p| !p.is_host()) {
                        let delta = i64::from(have) - i64::from(want);
                        reg.credit(p, wf(item), delta);
                        credit_shadow(gs, p, wf(item), delta);
                    }
                }
                self.seen.insert(c, dl);
            } else if dw != dl {
                // the host's own change: the ledger and the peers follow
                for item in ledger
                    .keys()
                    .chain(world.keys())
                    .copied()
                    .collect::<BTreeSet<_>>()
                {
                    let (have, want) = (
                        ledger.get(&item).copied().unwrap_or(0),
                        world.get(&item).copied().unwrap_or(0),
                    );
                    if have != want {
                        for o in live.loot.host_set(cf(c), item, want) {
                            outs.extend(live.route_extern(o.aud, Msg::Loot(o.msg)));
                        }
                    }
                }
                self.seen.insert(c, dw);
            }
        }
        outs
    }

    /// The host player's own open (takes the lease so guests see it busy).
    pub fn host_open(&self, container: FormId) -> Msg {
        Msg::Loot(LootMsg::Open {
            container: cf(container),
        })
    }
}

/// Guest side: requests and the mirror into `items`.
#[derive(Default)]
pub struct GuestLoot {
    watched: BTreeSet<FormId>,
    credited: BTreeMap<FormId, i64>,
    items_watched: BTreeSet<FormId>,
}

impl GuestLoot {
    /// A fresh bridge.
    pub fn new() -> Self {
        Self::default()
    }

    /// Asks for a container's lease.
    pub fn open(&mut self, client: &LootClient, container: FormId) -> Msg {
        self.watched.insert(container);
        Msg::Loot(client.open(cf(container)))
    }

    /// Takes items from a leased container (optimistic).
    pub fn take(
        &mut self,
        client: &mut LootClient,
        container: FormId,
        item: FormId,
        n: u32,
    ) -> Option<Msg> {
        self.items_watched.insert(item);
        client.take(cf(container), cf(item), n).map(Msg::Loot)
    }

    /// Puts items into a leased container (optimistic).
    pub fn put(
        &mut self,
        client: &mut LootClient,
        container: FormId,
        item: FormId,
        n: u32,
    ) -> Option<Msg> {
        self.items_watched.insert(item);
        client.put(cf(container), cf(item), n).map(Msg::Loot)
    }

    /// Closes a container.
    pub fn close(&mut self, client: &mut LootClient, container: FormId) -> Option<Msg> {
        client.close(cf(container)).map(Msg::Loot)
    }

    /// Claims a dropped item.
    pub fn claim(&self, drop: FormId) -> Msg {
        Msg::Loot(LootMsg::ClaimDrop { drop: cf(drop) })
    }

    /// Mirrors the client's view into the guest's `items`: container
    /// contents as shown (pending operations included) and the confirmed
    /// gain in the player's inventory.
    pub fn apply(&mut self, gs: &mut GameState, client: &LootClient) {
        for &c in &self.watched {
            let Some(v) = client.view(cf(c)) else {
                continue;
            };
            let vis = v.visible();
            let old: Vec<FormId> = gs
                .items
                .keys()
                .filter(|(h, _)| *h == c)
                .map(|(_, i)| *i)
                .collect();
            for i in old {
                if !vis.contains_key(&cf(i)) {
                    gs.items.remove(&(c, i));
                }
            }
            for (i, n) in vis {
                if n > 0 {
                    gs.items.insert((c, wf(i)), n as i32);
                }
            }
        }
        for &item in &self.items_watched {
            let total = client.confirmed_inventory_delta(cf(item));
            let done = self.credited.entry(item).or_insert(0);
            if total != *done {
                let e = gs.items.entry((PLAYER_REF, item)).or_insert(0);
                *e = (i64::from(*e) + total - *done).max(0) as i32;
                if *e == 0 {
                    gs.items.remove(&(PLAYER_REF, item));
                }
                *done = total;
                gs.emit(super::Domain::Items, PLAYER_REF);
            }
        }
    }
}
