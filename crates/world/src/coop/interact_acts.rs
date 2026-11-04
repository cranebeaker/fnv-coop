//! Executes an interaction [`Act`] on the host's `GameState` for one player.
//!
//! Everything personal (inventory, equipment, hot keys, skills, health from
//! an aid item) runs inside that player's shadow (`GameState::with_player`),
//! so the existing shadow replication carries the result back to the
//! player's own machine. Everything shared (a vendor's stock, a picked-up
//! world item, a destroyed object) is written to the shared rows the
//! replicated domains already cover. State no domain covers (door swing,
//! locks, terminals, mines) is returned as [`Shared`] records.

use coop_core::ids::{CellId, FormId as CForm, PlayerId};
use coop_core::interact::{
    Act, CompanionCmd, LockState, MinePhase, Outcome, PipOp, Shared, StealMode, TrapOutcome, Why,
    DROP_BASE,
};
use esm::FormId;

use super::apply::Ctx;
use super::facts::player_of;
use super::Domain;
use crate::dialogue::PLAYER_REF;
use crate::scripting::{Event, GameState, Runner};

/// Largest crafting batch.
pub const MAX_BATCH: u32 = 99;

pub(crate) fn cf(f: FormId) -> CForm {
    CForm(f.0)
}

pub(crate) fn wf(f: CForm) -> FormId {
    FormId(f.0)
}

/// Runs `f` as player `who`; what it says to the screen is dropped unless
/// `who` is the host (their screen is this machine's).
fn personal<R>(
    order: &esm::LoadOrder,
    gs: &mut GameState,
    who: PlayerId,
    f: impl FnOnce(&mut GameState) -> R,
) -> R {
    gs.with_player(who.0, |g| {
        // a new player's pack is the record's until it is first touched
        g.stock(order, PLAYER_REF);
        let mark = g.events.len();
        let r = f(g);
        if !who.is_host() {
            g.events.truncate(mark.min(g.events.len()));
        }
        r
    })
}

/// Whether a load door lets player `who` through on the host's world: it is
/// not locked, or `who` carries its key (the key unlocks it for everyone,
/// as `locks::try_open` does). A picked lock is open already, because the
/// guest's `Lockpick` request comes first.
pub(super) fn door_lets_through(
    order: &esm::LoadOrder,
    gs: &mut GameState,
    who: PlayerId,
    door: FormId,
) -> bool {
    personal(order, gs, who, |g| {
        use crate::locks::Opening;
        matches!(
            crate::locks::try_open(order, g, door),
            Opening::Open | Opening::WithKey
        )
    })
}

/// Whether the policy table makes `npc` a plot companion of the whole party
/// (`companion.owner = party`): any player's commands move it, no player owns it.
fn party_companion(order: &esm::LoadOrder, gs: &GameState, npc: FormId) -> bool {
    use coop_policy::CompanionOwner;
    let Some(gate) = gs.coop.gate.as_ref() else {
        return false;
    };
    let base = crate::scripting::base_of(order, npc).unwrap_or(npc);
    let edid = order
        .get(base)
        .and_then(|r| r.record().ok())
        .and_then(|r| r.editor_id());
    let Some(edid) = edid else { return false };
    gate.table().rows().iter().any(|row| {
        row.companion.as_ref().is_some_and(|c| {
            c.owner == CompanionOwner::Party && c.npcs.iter().any(|n| n.eq_ignore_ascii_case(&edid))
        })
    })
}

fn has_on_activate(cx: Ctx, obj: FormId) -> bool {
    crate::scripting::script_of(cx.order, obj)
        .and_then(|s| cx.scripts.script(cx.order, s))
        .is_some_and(|s| s.blocks.iter().any(|b| b.kind == "onactivate"))
}

/// The shared records that describe `obj` right now (door, lock, terminal,
/// mine), read from the state. Used after the host (or a guest's act) changed
/// it, and by the host's own local actions.
pub fn snapshot_obj(cx: Ctx, gs: &GameState, obj: FormId) -> Vec<Shared> {
    let mut v = Vec::new();
    if crate::doors::open_state(cx.order, gs, obj) != crate::doors::OpenState::None {
        v.push(Shared::Door {
            door: cf(obj),
            open: crate::doors::is_open(cx.order, gs, obj),
        });
    }
    if gs.locks.contains_key(&obj) || gs.broken_locks.contains_key(&obj) {
        v.push(Shared::Lock {
            obj: cf(obj),
            state: LockState {
                level: gs.locks.get(&obj).copied().flatten(),
                broken: gs.broken_locks.get(&obj).copied().unwrap_or(0) > 0,
            },
        });
    }
    if let Some(t) = gs.terminal_states.get(&obj) {
        v.push(Shared::Terminal {
            obj: cf(obj),
            hacked: t.hacked,
            lockouts: t.lockouts,
        });
    }
    if gs.more.mines.gone.contains(&obj) {
        v.push(Shared::Mine {
            obj: cf(obj),
            phase: MinePhase::Gone,
        });
    } else if gs.more.mines.disarmed.contains(&obj) {
        v.push(Shared::Mine {
            obj: cf(obj),
            phase: MinePhase::Disarmed,
        });
    }
    v
}

/// Every shared record the host's world holds that no replicated domain
/// carries: doors a script, a player or an act moved, locks, terminals, mines.
/// The host sweeps this a few times a second and publishes what changed, so a
/// script's `Lock`, the host player's own door or a quest unlocking a gate
/// reach the guests without any game site having to announce it.
pub fn world_shared(cx: Ctx, gs: &GameState) -> Vec<Shared> {
    let mut v = Vec::new();
    for (door, open) in &gs.set_by_scripts.open {
        if crate::doors::open_state(cx.order, gs, *door) != crate::doors::OpenState::None {
            v.push(Shared::Door {
                door: cf(*door),
                open: *open,
            });
        }
    }
    let locked: std::collections::BTreeSet<FormId> = gs
        .locks
        .keys()
        .chain(gs.broken_locks.keys())
        .copied()
        .collect();
    for obj in locked {
        v.push(Shared::Lock {
            obj: cf(obj),
            state: LockState {
                level: gs.locks.get(&obj).copied().flatten(),
                broken: gs.broken_locks.get(&obj).copied().unwrap_or(0) > 0,
            },
        });
    }
    for (obj, t) in &gs.terminal_states {
        v.push(Shared::Terminal {
            obj: cf(*obj),
            hacked: t.hacked,
            lockouts: t.lockouts,
        });
    }
    for obj in &gs.more.mines.gone {
        v.push(Shared::Mine {
            obj: cf(*obj),
            phase: MinePhase::Gone,
        });
    }
    for obj in gs
        .more
        .mines
        .disarmed
        .iter()
        .filter(|o| !gs.more.mines.gone.contains(o))
    {
        v.push(Shared::Mine {
            obj: cf(*obj),
            phase: MinePhase::Disarmed,
        });
    }
    v
}

fn done_with(cx: Ctx, gs: &GameState, obj: FormId) -> Outcome {
    Outcome::done().with_shared(snapshot_obj(cx, gs, obj))
}

/// Executes `act` for player `who`. `mint` makes a fresh form for an item
/// dropped into the world. A [`Act::Transition`] is the host bridge's to
/// decide and is refused here.
pub fn execute(
    cx: Ctx,
    gs: &mut GameState,
    who: PlayerId,
    act: &Act,
    mint: &mut dyn FnMut() -> CForm,
) -> Outcome {
    let order = cx.order;
    match act {
        Act::World(s) => {
            if who.is_host() || !matches!(s, Shared::Placed { .. }) {
                apply_shared(cx, gs, s);
                Outcome::done().with_shared(vec![s.clone()])
            } else {
                Outcome::refused(Why::Invalid)
            }
        }
        Act::Activate { obj } => {
            let obj = wf(*obj);
            if gs.destroyed.contains(&obj) {
                return Outcome::refused(Why::Gone);
            }
            if has_on_activate(cx, obj) {
                gs.with_player(who.0, |g| {
                    let mark = g.events.len();
                    Runner::new(order, cx.scripts, g).run_event(obj, "onactivate", PLAYER_REF);
                    if !who.is_host() {
                        // the screen messages were meant for the activator
                        let tail: Vec<Event> = g.events.split_off(mark);
                        g.events.extend(
                            tail.into_iter()
                                .filter(|e| !matches!(e, Event::Message { .. })),
                        );
                    }
                });
            }
            done_with(cx, gs, obj)
        }
        Act::Door { door } => {
            let door = wf(*door);
            use crate::doors::OpenState;
            match crate::doors::open_state(order, gs, door) {
                OpenState::None => return Outcome::refused(Why::Unknown),
                OpenState::Opening | OpenState::Closing => return Outcome::refused(Why::Busy),
                _ => {}
            }
            if gs.destroyed.contains(&door) {
                return Outcome::refused(Why::Gone);
            }
            let opening = personal(order, gs, who, |g| {
                use crate::locks::Opening;
                match crate::locks::try_open(order, g, door) {
                    Opening::Open | Opening::WithKey => {}
                    _ => return false,
                }
                if has_on_activate(cx, door) {
                    let before = g.events.len();
                    Runner::new(order, cx.scripts, g).run_event(door, "onactivate", PLAYER_REF);
                    let went = g.events[before..]
                        .iter()
                        .any(|e| matches!(e, Event::Activate { what, .. } if *what == door));
                    g.events
                        .retain(|e| !matches!(e, Event::Activate { what, .. } if *what == door));
                    if !went {
                        return false;
                    }
                }
                true
            });
            if !opening {
                return Outcome::refused(Why::Locked);
            }
            crate::doors::activate(order, gs, door, Some(PLAYER_REF));
            done_with(cx, gs, door)
        }
        Act::Pickup { obj, item, count } => {
            let (obj, item) = (wf(*obj), wf(*item));
            if *count == 0 {
                return Outcome::refused(Why::Invalid);
            }
            if gs.disabled.get(&obj).copied() == Some(true) || gs.destroyed.contains(&obj) {
                return Outcome::refused(Why::Gone);
            }
            let n = if obj.0 >= DROP_BASE {
                let there = gs.items.get(&(obj, item)).copied().unwrap_or(0);
                if there <= 0 {
                    return Outcome::refused(Why::Gone);
                }
                there.min(*count as i32)
            } else {
                (*count as i32).min(crate::activation::placed_count(order, obj).max(1))
            };
            if obj.0 >= DROP_BASE {
                let left = gs.items.get(&(obj, item)).copied().unwrap_or(0) - n;
                if left > 0 {
                    gs.items.insert((obj, item), left);
                } else {
                    gs.items.remove(&(obj, item));
                }
                gs.emit(Domain::Items, obj);
            }
            personal(order, gs, who, |g| g.pick_up(order, obj, item, n));
            if obj.0 >= DROP_BASE && gs.items.get(&(obj, item)).copied().unwrap_or(0) > 0 {
                // part of a pile: the pile stays
                gs.disabled.remove(&obj);
                gs.emit(Domain::Disabled, obj);
            }
            Outcome::done()
        }
        Act::Drop {
            item,
            count,
            cell,
            pos,
        } => {
            let item = wf(*item);
            if *count == 0 {
                return Outcome::refused(Why::Invalid);
            }
            let have = personal(order, gs, who, |g| g.item_count(order, PLAYER_REF, item));
            if have < *count as i32 {
                return Outcome::refused(Why::Insufficient);
            }
            let drop = wf(mint());
            let space = personal(order, gs, who, |g| g.player_world.map_or(cell.0, |w| w.0));
            personal(order, gs, who, |g| {
                g.move_item(order, PLAYER_REF, drop, item, *count as i32);
                if g.is_equipped(PLAYER_REF, item) && g.item_count(order, PLAYER_REF, item) == 0 {
                    g.unequip_item(order, PLAYER_REF, item);
                }
            });
            let s = Shared::Placed {
                drop: cf(drop),
                item: cf(item),
                count: *count,
                cell: *cell,
                space: CellId(space),
                pos: *pos,
            };
            apply_shared(cx, gs, &s);
            Outcome::done().with_shared(vec![s])
        }
        Act::Barter {
            vendor,
            buys,
            sells,
            total,
        } => {
            let vendor = wf(*vendor);
            let buys: Vec<(FormId, i32)> = buys.iter().map(|(i, n)| (wf(*i), *n as i32)).collect();
            let sells: Vec<(FormId, i32)> =
                sells.iter().map(|(i, n)| (wf(*i), *n as i32)).collect();
            if buys.iter().chain(sells.iter()).any(|(_, n)| *n <= 0) {
                return Outcome::refused(Why::Invalid);
            }
            let holders = crate::barter::vendor_holders(order, vendor);
            for h in &holders {
                gs.stock(order, *h);
            }
            let in_stock = |gs: &GameState, item: FormId| -> i32 {
                holders.iter().map(|h| gs.item_count(order, *h, item)).sum()
            };
            if buys.iter().any(|(i, n)| in_stock(gs, *i) < *n) {
                return Outcome::refused(Why::Insufficient);
            }
            let caps = crate::barter::caps(order);
            let ok = personal(order, gs, who, |g| {
                if sells
                    .iter()
                    .any(|(i, n)| g.item_count(order, PLAYER_REF, *i) < *n)
                {
                    return false;
                }
                // what he pays with: the caps he has, or the ones he gets from the sale
                if *total < 0 && g.item_count(order, PLAYER_REF, caps) < -*total {
                    return false;
                }
                crate::barter::accept(order, g, vendor, &buys, &sells, *total as f32);
                true
            });
            if ok {
                Outcome::done()
            } else {
                Outcome::refused(Why::Insufficient)
            }
        }
        Act::Steal {
            target,
            item,
            count,
            mode,
        } => {
            let (target, item) = (wf(*target), wf(*item));
            if *count == 0 || target == PLAYER_REF || player_of(target).is_some() {
                return Outcome::refused(Why::Policy);
            }
            let n = *count as i32;
            let dead = gs.dead.contains(&target) || gs.unconscious.contains(&target);
            if *mode == StealMode::Loot && !dead {
                return Outcome::refused(Why::Policy);
            }
            gs.stock(order, target);
            let moved = personal(order, gs, who, |g| match mode {
                StealMode::Pickpocket | StealMode::Loot => {
                    g.move_item(order, target, PLAYER_REF, item, n)
                }
                StealMode::Plant => g.move_item(order, PLAYER_REF, target, item, n),
            });
            if moved == 0 {
                Outcome::refused(Why::Insufficient)
            } else {
                Outcome::done()
            }
        }
        Act::Craft { recipe, times, .. } => {
            let Some(r) = crate::crafting::Recipe::load(order, wf(*recipe)) else {
                return Outcome::refused(Why::Unknown);
            };
            let times = (*times).clamp(1, MAX_BATCH);
            let ok = personal(order, gs, who, |g| {
                g.stock(order, PLAYER_REF);
                if !crate::crafting::has_skill(order, g, PLAYER_REF, &r) {
                    return false;
                }
                if r.ingredients
                    .iter()
                    .any(|c| g.item_count(order, PLAYER_REF, c.item) < (c.count * times) as i32)
                {
                    return false;
                }
                crate::crafting::make(order, g, PLAYER_REF, &r, times).is_some()
            });
            if ok {
                Outcome::done()
            } else {
                Outcome::refused(Why::Insufficient)
            }
        }
        Act::Repair {
            item,
            part,
            vendor,
            cost,
        } => {
            let (item, part, vendor) = (wf(*item), wf(*part), wf(*vendor));
            let ok = personal(order, gs, who, |g| {
                if g.item_count(order, PLAYER_REF, item) <= 0 {
                    return false;
                }
                if part.0 != 0 {
                    if g.item_count(order, PLAYER_REF, part) <= 0 {
                        return false;
                    }
                    crate::repair::repair_with(order, g, item, part);
                    true
                } else if vendor.0 != 0 {
                    if g.item_count(order, PLAYER_REF, crate::barter::caps(order)) < *cost as i32 {
                        return false;
                    }
                    crate::repair::repair_by(order, g, vendor, item, *cost as i32);
                    true
                } else {
                    false
                }
            });
            if ok {
                Outcome::done()
            } else {
                Outcome::refused(Why::Insufficient)
            }
        }
        Act::Use { item } => {
            let item = wf(*item);
            let used = personal(order, gs, who, |g| {
                crate::items::use_item(order, g, PLAYER_REF, item)
            });
            if used.is_some() {
                Outcome::done()
            } else {
                Outcome::refused(Why::Insufficient)
            }
        }
        Act::Equip { item, on } => {
            let item = wf(*item);
            let ok = personal(order, gs, who, |g| {
                if g.item_count(order, PLAYER_REF, item) <= 0 {
                    return false;
                }
                if *on {
                    g.equip(order, PLAYER_REF, item);
                } else {
                    g.unequip_item(order, PLAYER_REF, item);
                }
                true
            });
            if ok {
                Outcome::done()
            } else {
                Outcome::refused(Why::Insufficient)
            }
        }
        Act::Ammo { ammo } => {
            let ammo = wf(*ammo);
            let ok = personal(order, gs, who, |g| {
                if ammo.0 == 0 {
                    g.ammo_loaded.remove(&PLAYER_REF);
                    return true;
                }
                if g.item_count(order, PLAYER_REF, ammo) <= 0 {
                    return false;
                }
                g.ammo_loaded.insert(PLAYER_REF, ammo);
                true
            });
            if ok {
                Outcome::done()
            } else {
                Outcome::refused(Why::Insufficient)
            }
        }
        Act::Pip(op) => match *op {
            PipOp::Hotkey { slot, item } => {
                if slot >= 8 {
                    return Outcome::refused(Why::Invalid);
                }
                personal(order, gs, who, |g| {
                    g.hotkeys[slot as usize] = (item.0 != 0).then(|| wf(item))
                });
                Outcome::done()
            }
            PipOp::Note(n) => {
                gs.notes.insert(wf(n));
                gs.emit(Domain::Note, wf(n));
                Outcome::done()
            }
            PipOp::ActiveQuest(_) => Outcome::done(),
        },
        Act::Companion { who: npc, cmd } => {
            let npc = wf(*npc);
            let party = party_companion(order, gs, npc);
            if party && matches!(cmd, CompanionCmd::Recruit | CompanionCmd::Dismiss) {
                // a plot companion belongs to the party: nobody recruits or dismisses it alone
                return Outcome::refused(Why::Policy);
            }
            if *cmd == CompanionCmd::Recruit {
                let taken = (0..=8u8).filter(|s| *s != who.0).any(|s| {
                    gs.shadow_of(s)
                        .is_some_and(|sh| sh.teammates.contains(&npc))
                });
                if taken {
                    return Outcome::refused(Why::Busy);
                }
                personal(order, gs, who, |g| g.teammates.insert(npc));
                return Outcome::done();
            }
            let ok = personal(order, gs, who, |g| {
                if !party && !g.teammates.contains(&npc) {
                    return false;
                }
                match cmd {
                    CompanionCmd::Follow => {
                        crate::companions::set_variable(order, cx.scripts, g, npc, "Waiting", 0.0);
                        true
                    }
                    CompanionCmd::Wait => {
                        crate::companions::set_variable(order, cx.scripts, g, npc, "Waiting", 1.0);
                        true
                    }
                    CompanionCmd::Dismiss => {
                        g.teammates.remove(&npc);
                        true
                    }
                    CompanionCmd::Stimpak => crate::companions::heal_with_stimpak(order, g, npc),
                    CompanionCmd::SortGear => {
                        crate::companions::wear_best_armour(order, g, npc);
                        true
                    }
                    CompanionCmd::Switch(which, on) => {
                        let name = [
                            "FollowerSwitchAggressive",
                            "Waiting",
                            "IsFollowingLong",
                            "CombatStyleRanged",
                        ]
                        .get(usize::from(*which))
                        .copied();
                        name.is_some_and(|n| {
                            crate::companions::set_variable(
                                order,
                                cx.scripts,
                                g,
                                npc,
                                n,
                                if *on { 1.0 } else { 0.0 },
                            );
                            true
                        })
                    }
                    CompanionCmd::Recruit => true,
                }
            });
            if ok {
                Outcome::done()
            } else {
                Outcome::refused(Why::Insufficient)
            }
        }
        Act::Lockpick {
            obj,
            won,
            pins,
            broke,
        } => {
            let obj = wf(*obj);
            personal(order, gs, who, |g| {
                for _ in 0..(*pins).min(10) {
                    crate::lockpick::use_up_pin(order, g);
                }
                if *won {
                    let difficulty =
                        crate::locks::lock_now(order, g, obj).map_or(1, |l| l.difficulty(order).0);
                    crate::lockpick::picked(order, g, obj, difficulty);
                } else if *broke {
                    crate::lockpick::break_lock(g, obj);
                }
            });
            done_with(cx, gs, obj)
        }
        Act::Key { obj, key } => {
            let (obj, key) = (wf(*obj), wf(*key));
            let ok = personal(order, gs, who, |g| {
                let fits =
                    crate::locks::lock_now(order, g, obj).is_some_and(|l| l.key == Some(key));
                fits && g.item_count(order, PLAYER_REF, key) > 0
            });
            if !ok {
                return Outcome::refused(Why::Locked);
            }
            gs.locks.insert(obj, None);
            done_with(cx, gs, obj)
        }
        Act::Hack { obj, won } => {
            let obj = wf(*obj);
            personal(order, gs, who, |g| {
                if *won {
                    match crate::terminal::placed(order, obj) {
                        Some(t) => crate::terminal::hacked(order, g, &t, obj),
                        None => g.terminal_states.entry(obj).or_default().hacked = true,
                    }
                } else {
                    crate::terminal::lock_out(g, obj);
                }
            });
            done_with(cx, gs, obj)
        }
        Act::Trap { obj, what } => {
            let obj = wf(*obj);
            match what {
                TrapOutcome::Disarmed => {
                    if gs.more.mines.gone.contains(&obj) {
                        return Outcome::refused(Why::Gone);
                    }
                    gs.more.mines.disarmed.insert(obj);
                }
                TrapOutcome::Detonated => {
                    if gs.more.mines.gone.contains(&obj) {
                        // went off already: exactly once
                        return Outcome::refused(Why::Gone);
                    }
                    crate::mines::gone(gs, obj);
                    gs.coop.interact.detonations.push(obj);
                }
                TrapOutcome::Destroyed => {
                    gs.destroyed.insert(obj);
                    gs.emit(Domain::Destroyed, obj);
                }
            }
            done_with(cx, gs, obj)
        }
        Act::Discover { marker } => {
            let m = wf(*marker);
            gs.discovered.insert(m);
            gs.map_markers.insert(m);
            gs.emit(Domain::MapMarker, m);
            Outcome::done()
        }
        Act::Transition { .. } => Outcome::refused(Why::Invalid),
        // a player's own progress and ledgers (`campaign::progress`), in their shadow
        Act::LevelUp { skills, perk } => {
            let r = personal(order, gs, who, |g| {
                super::campaign::progress::level_up(order, g, skills, wf(*perk))
            });
            match r {
                Ok(_) => Outcome::done(),
                Err(why) => Outcome::refused(why),
            }
        }
        Act::Caravan { npc, stake, won } => {
            personal(order, gs, who, |g| {
                super::campaign::progress::caravan_played(order, g, wf(*npc), *stake, *won)
            });
            Outcome::done()
        }
        Act::Casino {
            casino,
            chips,
            winnings,
            level,
            new_level,
        } => {
            let r = personal(order, gs, who, |g| {
                super::campaign::progress::casino_closed(
                    order,
                    g,
                    wf(*casino),
                    *chips,
                    *winnings,
                    *level,
                    *new_level,
                )
            });
            match r {
                Ok(()) => Outcome::done(),
                Err(why) => Outcome::refused(why),
            }
        }
    }
}

/// Writes a shared record into this machine's world. Idempotent.
pub fn apply_shared(cx: Ctx, gs: &mut GameState, s: &Shared) {
    match s {
        Shared::Door { door, open } => {
            let door = wf(*door);
            if crate::doors::is_open(cx.order, gs, door) != *open
                && crate::doors::activate(cx.order, gs, door, None) == crate::doors::Activated::Busy
            {
                crate::doors::set_at_once(gs, door, *open);
            }
        }
        Shared::Lock { obj, state } => {
            let obj = wf(*obj);
            gs.locks.insert(obj, state.level);
            if state.broken && gs.broken_locks.get(&obj).copied().unwrap_or(0) == 0 {
                gs.broken_locks.insert(obj, 1);
            }
        }
        Shared::Terminal {
            obj,
            hacked,
            lockouts,
        } => {
            let t = gs.terminal_states.entry(wf(*obj)).or_default();
            t.hacked = *hacked;
            t.lockouts = *lockouts;
        }
        Shared::Mine { obj, phase } => {
            let obj = wf(*obj);
            match phase {
                MinePhase::Gone => {
                    if !gs.more.mines.gone.contains(&obj) {
                        crate::mines::gone(gs, obj);
                    }
                }
                MinePhase::Disarmed => {
                    if !gs.more.mines.gone.contains(&obj) {
                        gs.more.mines.disarmed.insert(obj);
                    }
                }
            }
        }
        Shared::Placed {
            drop,
            item,
            count,
            cell,
            space,
            pos,
        } => {
            let note = super::interact::PlacedNote {
                drop: wf(*drop),
                item: wf(*item),
                count: *count,
                cell: CellId(cell.0),
                pos: *pos,
            };
            if gs.coop.interact.seen_drops.insert(note.drop) {
                // a dropped item is a made reference like any the player drops alone
                let at = [pos[0] as f32, pos[1] as f32, pos[2] as f32];
                gs.more.placed.refs.insert(
                    note.drop,
                    crate::more_functions::placed::Made {
                        base: note.item,
                        space: FormId(space.0),
                        cell: FormId(cell.0),
                        position: at,
                        rotation: [0.0; 3],
                        count: *count as i32,
                    },
                );
                gs.events.push(crate::scripting::Event::More(
                    crate::more_functions::Shown::Placed {
                        reference: note.drop,
                    },
                ));
                let at = [pos[0] as f32, pos[1] as f32, pos[2] as f32];
                gs.coop.live.spawns.push(super::facts::SpawnNote {
                    id: note.drop,
                    base: Some(note.item),
                    pos: at,
                });
                gs.coop.interact.placed.push(note);
            }
        }
    }
}
