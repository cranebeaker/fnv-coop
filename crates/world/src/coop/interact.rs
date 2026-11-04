//! Player-world interactions on `GameState`: the facts the game sites write,
//! the host and guest bridges, and the bot verb parser.
//!
//! A game site that used to change the world for the player calls
//! [`GameState::coop_route`] (the site applies the rule itself on single player
//! and on the host) or [`GameState::coop_do`] (the call applies it too). On a
//! guest nothing is applied locally: the act becomes a request, the host
//! executes it once ([`super::interact_acts::execute`]) and the result comes
//! back through the replicated fields, the shared records and the verdict.

use std::collections::{BTreeMap, BTreeSet};

use coop_core::ids::{CellId, FormId as CForm, PlayerId};
use coop_core::interact::{
    Act, CompanionCmd, Decision, Follow, IOut, Outcome, PipOp, Shared, StealMode, TrapOutcome, Why,
    DROP_BASE,
};
use coop_core::proto::Msg;
use coop_core::rules::RulesEvent;
use coop_core::sync::{GuestLive, HostLive};
use esm::FormId;

use super::apply::Ctx;
use super::interact_acts::{apply_shared, cf, door_lets_through, execute, snapshot_obj};
use super::live::Outs;
use super::Authority;
use crate::scripting::GameState;

/// Tag base of the barriers opened for one-way transitions.
pub const TRANSIT_TAG: u32 = 0x5452_0000;

/// What a game site does after asking.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Route {
    /// This machine runs the rule (single player, host, or already ran it).
    Local,
    /// Do nothing more: the host decides and the result comes back.
    Remote,
}

/// A world item the host created (the viewer spawns a model for it).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlacedNote {
    /// The new reference.
    pub drop: FormId,
    /// The item.
    pub item: FormId,
    /// Count.
    pub count: u32,
    /// Cell.
    pub cell: CellId,
    /// Position.
    pub pos: [i32; 3],
}

/// A finished request, for the screen.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActResult {
    /// What was asked.
    pub act: Act,
    /// Done.
    pub ok: bool,
    /// Why not.
    pub why: Why,
}

/// Per-machine interaction facts. Never replicated, never saved.
#[derive(Clone, Debug, Default)]
pub struct InteractFacts {
    /// Acts waiting for the bridge: (act, already applied on this machine).
    pub acts: Vec<(Act, bool)>,
    /// Finished requests (the viewer drains them for messages).
    pub results: Vec<ActResult>,
    /// A destination this machine must load now (the viewer takes it).
    pub follow: Option<Follow>,
    /// World items to spawn (the viewer drains them).
    pub placed: Vec<PlacedNote>,
    /// Drops already announced.
    pub seen_drops: BTreeSet<FormId>,
    /// Mines that went off (the host's viewer runs the explosion).
    pub detonations: Vec<FormId>,
    /// Script `MoveTo`s of a player: (slot, destination reference, cell left, cell entered).
    pub script_moves: Vec<(u8, FormId, u32, FormId)>,
    /// Acts done on this machine's behalf, by name.
    pub done: BTreeMap<&'static str, u32>,
    /// Requests refused, by name.
    pub refused: BTreeMap<&'static str, u32>,
    /// Shared records applied here.
    pub shared_in: u32,
    /// Destinations received.
    pub follows_in: u32,
    /// Guest: map markers this machine found and sent to the host, so that
    /// `world::map::discover` finds each once before the party's copy arrives.
    pub found: BTreeSet<FormId>,
    next_drop: u32,
}

impl InteractFacts {
    /// The counters as a JSON object (the dump's `interact`).
    pub fn counters_json(&self) -> String {
        let mut parts: Vec<String> = Vec::new();
        for (k, v) in &self.done {
            parts.push(format!("\"{k}\":{v}"));
        }
        let refused: u32 = self.refused.values().sum();
        parts.push(format!("\"refused\":{refused}"));
        parts.push(format!("\"shared_in\":{}", self.shared_in));
        parts.push(format!("\"follows_in\":{}", self.follows_in));
        parts.push(format!("\"placed\":{}", self.seen_drops.len()));
        parts.push(format!("\"detonations\":{}", self.detonations.len()));
        format!("{{{}}}", parts.join(","))
    }
}

/// Acts the host bridge executes itself (it mints the reference of a drop).
fn deferred(act: &Act) -> bool {
    matches!(act, Act::Drop { .. })
}

pub use coop_core::interact::Act as CoopAct;

/// The act of accepting a trade.
pub fn barter_act(
    vendor: FormId,
    buys: &[(FormId, i32)],
    sells: &[(FormId, i32)],
    total: f32,
) -> Act {
    let c = |l: &[(FormId, i32)]| {
        l.iter()
            .map(|(i, n)| (cf(*i), (*n).max(0) as u32))
            .collect()
    };
    Act::Barter {
        vendor: cf(vendor),
        buys: c(buys),
        sells: c(sells),
        total: total.floor() as i32,
    }
}

/// The lockpicking game's outcome: `pins` spent, opened, or broke the lock.
pub fn lockpick_act(obj: FormId, won: bool, pins: u8, broke: bool) -> Act {
    Act::Lockpick {
        obj: cf(obj),
        won,
        pins,
        broke,
    }
}

/// The hacking game's outcome.
pub fn hack_act(obj: FormId, won: bool) -> Act {
    Act::Hack { obj: cf(obj), won }
}

/// Swinging a door open or shut.
pub fn door_act(door: FormId) -> Act {
    Act::Door { door: cf(door) }
}

/// Using a (two-way) load door from cell `from` into cell `to`.
pub fn enter_act(door: FormId, from: u32, to: u32) -> Act {
    Act::Transition {
        door: cf(door),
        from: CellId(from),
        to: CellId(to),
        one_way: false,
    }
}

/// Taking a thing lying in the world.
pub fn pickup_act(obj: FormId, item: FormId, count: u32) -> Act {
    Act::Pickup {
        obj: cf(obj),
        item: cf(item),
        count,
    }
}

/// Making things at a bench (`bench` 0 when the menu does not know it).
pub fn craft_act(bench: FormId, recipe: FormId, times: u32) -> Act {
    Act::Craft {
        bench: cf(bench),
        recipe: cf(recipe),
        times,
    }
}

/// Mending an item with a part.
pub fn repair_part_act(item: FormId, part: FormId) -> Act {
    Act::Repair {
        item: cf(item),
        part: cf(part),
        vendor: cf(FormId(0)),
        cost: 0,
    }
}

/// Having a vendor mend an item for `cost` caps.
pub fn repair_vendor_act(item: FormId, vendor: FormId, cost: i32) -> Act {
    Act::Repair {
        item: cf(item),
        part: cf(FormId(0)),
        vendor: cf(vendor),
        cost: cost.max(0) as u32,
    }
}

/// Using an aid item.
pub fn use_act(item: FormId) -> Act {
    Act::Use { item: cf(item) }
}

/// Putting an item on or taking it off.
pub fn equip_act(item: FormId, on: bool) -> Act {
    Act::Equip { item: cf(item), on }
}

/// Choosing the ammunition.
pub fn ammo_act(ammo: FormId) -> Act {
    Act::Ammo { ammo: cf(ammo) }
}

/// Finding a map marker.
pub fn discover_act(marker: FormId) -> Act {
    Act::Discover { marker: cf(marker) }
}

/// Telling a companion something with the wheel (`Request::Variable`):
/// the switch variables map to switches, nothing else is shared.
pub fn companion_var_act(who: FormId, name: &str, value: f32) -> Option<Act> {
    let which = match name.to_ascii_lowercase().as_str() {
        "followerswitchaggressive" => 0,
        "waiting" => 1,
        "isfollowinglong" => 2,
        "combatstyleranged" => 3,
        _ => return None,
    };
    Some(Act::Companion {
        who: cf(who),
        cmd: coop_core::interact::CompanionCmd::Switch(which, value != 0.0),
    })
}

/// A Stimpak on a companion.
pub fn companion_stimpak_act(who: FormId) -> Act {
    Act::Companion {
        who: cf(who),
        cmd: coop_core::interact::CompanionCmd::Stimpak,
    }
}

impl GameState {
    /// A game site tells the co-op layer what the player just did. Single
    /// player: nothing is recorded, `Local`. Host: recorded so the shared
    /// result is published, `Local` (the site applies the rule itself) except
    /// for drops and transitions, which the bridge executes. Guest: sent to
    /// the host, `Remote`.
    pub fn coop_route(&mut self, act: Act) -> Route {
        match self.coop.authority {
            Authority::Single => Route::Local,
            Authority::Host if !deferred(&act) => {
                self.coop.interact.acts.push((act, true));
                Route::Local
            }
            Authority::Host => {
                self.coop.interact.acts.push((act, false));
                Route::Remote
            }
            Authority::Guest => {
                self.coop.interact.acts.push((act, false));
                Route::Remote
            }
        }
    }

    /// Does the act: on a guest it is a request; on the host (and single
    /// player) the rule runs here and now, through the same executor the host
    /// uses for guests.
    pub fn coop_do(&mut self, cx: Ctx, act: Act) -> Route {
        match self.coop.authority {
            Authority::Guest => {
                self.coop.interact.acts.push((act, false));
                Route::Remote
            }
            _ if deferred(&act) && self.coop.authority == Authority::Host => {
                self.coop.interact.acts.push((act, false));
                Route::Remote
            }
            Authority::Host if matches!(act, Act::Transition { .. }) => {
                // the bridge decides who follows; the host walks through itself
                self.coop.interact.acts.push((act, true));
                Route::Local
            }
            auth => {
                let mut n = self.coop.interact.next_drop;
                let mut mint = || {
                    n += 1;
                    CForm(DROP_BASE + 0x0100_0000 + n)
                };
                let o = execute(cx, self, PlayerId::HOST, &act, &mut mint);
                self.coop.interact.next_drop = n;
                self.coop.interact.count(&act, &o);
                if auth == Authority::Host {
                    self.coop.interact.acts.push((act, true));
                }
                Route::Local
            }
        }
    }
}

impl GameState {
    /// Drops items in co-op (the host bridge makes the world reference and
    /// tells everyone). False on single player: the caller drops them itself.
    pub fn coop_drop(
        &mut self,
        order: &esm::LoadOrder,
        item: FormId,
        count: i32,
        heading: f32,
    ) -> bool {
        if self.coop.authority == Authority::Single {
            return false;
        }
        let Some((space, cell, p, _)) = self.place(order, crate::dialogue::PLAYER_REF) else {
            return false;
        };
        let spot = [
            (p[0] + 60.0 * heading.sin()) as i32,
            (p[1] + 60.0 * heading.cos()) as i32,
            p[2] as i32,
        ];
        let _ = space;
        self.coop.interact.acts.push((
            Act::Drop {
                item: cf(item),
                count: count.max(1) as u32,
                cell: CellId(cell.0),
                pos: spot,
            },
            false,
        ));
        true
    }

    /// A script is moving the player (`MoveTo`): the host tells the party. Called
    /// from the script command before the move; zero cost off the host.
    #[inline]
    pub fn coop_script_moved(&mut self, to: FormId, entering: FormId) {
        if self.coop.authority != Authority::Host {
            return;
        }
        let from = self.player_cell.map_or(0, |c| c.0);
        let slot = self.active_slot();
        self.coop
            .interact
            .script_moves
            .push((slot, to, from, entering));
    }
}

impl InteractFacts {
    fn count(&mut self, act: &Act, o: &Outcome) {
        let map = if o.ok {
            &mut self.done
        } else {
            &mut self.refused
        };
        *map.entry(act.name()).or_insert(0) += 1;
    }
}

fn send(live: &mut HostLive, outs: Vec<IOut>) -> Outs {
    outs.into_iter()
        .flat_map(|o| live.route_extern(o.aud, Msg::Interact(o.msg)))
        .collect()
}

struct Pending {
    from: PlayerId,
    txn: Option<u32>,
    door: CForm,
    to: CellId,
}

/// Host side: executes guests' requests and publishes the host's own.
#[derive(Default)]
pub struct HostInteract {
    pending: BTreeMap<u32, Pending>,
    next_tag: u32,
    next_sweep_ms: u64,
    /// Requests executed (including the host's own deferred ones).
    pub executed: u32,
}

/// How often the host looks for shared world changes nobody announced.
const SWEEP_MS: u64 = 200;

impl HostInteract {
    /// A fresh bridge.
    pub fn new() -> Self {
        Self::default()
    }

    fn decide(
        &mut self,
        live: &mut HostLive,
        outs: &mut Outs,
        from: PlayerId,
        txn: Option<u32>,
        act: &Act,
        now_ms: u64,
    ) -> Option<Outcome> {
        let Act::Transition {
            door,
            from: from_cell,
            to,
            one_way,
        } = act
        else {
            return Some(Outcome::refused(Why::Invalid));
        };
        let members = live.members().to_vec();
        let t = live.interact.transit;
        match t.decide(from, *from_cell, *to, *one_way, &members, &live.interest) {
            Decision::Go { followers } => Some(Outcome::done().with_follow(
                Follow {
                    door: *door,
                    to: *to,
                    leader: from,
                },
                followers,
            )),
            Decision::Deny(w) => Some(Outcome::refused(w)),
            Decision::Barrier => {
                self.next_tag += 1;
                let tag = TRANSIT_TAG | (self.next_tag & 0xFFFF);
                let (_, rout) = live.rules.open_barrier(tag, now_ms);
                for o in rout {
                    outs.extend(live.route_extern(o.aud, Msg::Rules(o.msg)));
                }
                self.pending.insert(
                    tag,
                    Pending {
                        from,
                        txn,
                        door: *door,
                        to: *to,
                    },
                );
                None
            }
        }
    }

    /// Every frame on the host. `rule_events` are the events the rules
    /// bridge collected (barrier releases are read from them).
    pub fn frame(
        &mut self,
        cx: Ctx,
        gs: &mut GameState,
        live: &mut HostLive,
        own: &mut GuestLive,
        rule_events: &[RulesEvent],
        now_ms: u64,
    ) -> Outs {
        let mut outs: Outs = Vec::new();
        if now_ms >= self.next_sweep_ms {
            self.next_sweep_ms = now_ms + SWEEP_MS;
            let io = live
                .interact
                .publish(super::interact_acts::world_shared(cx, gs));
            outs.extend(send(live, io));
        }
        // scripts that moved a player: the party follows the leader, a guest is brought along
        for (slot, to, from, entering) in std::mem::take(&mut gs.coop.interact.script_moves) {
            let f = Follow {
                door: cf(to),
                to: CellId(entering.0),
                leader: PlayerId(slot),
            };
            if slot != 0 {
                let io = live.interact.follow(f, &[PlayerId(slot)]);
                outs.extend(send(live, io));
                continue;
            }
            let act = Act::Transition {
                door: cf(to),
                from: CellId(from),
                to: CellId(entering.0),
                one_way: false,
            };
            if let Some(o) = self.decide(live, &mut outs, PlayerId::HOST, None, &act, now_ms) {
                if let Some((f, who)) = o.follow.filter(|_| o.ok) {
                    let io = live.interact.follow(f, &who);
                    outs.extend(send(live, io));
                    *gs.coop.interact.done.entry("script_move").or_insert(0) += 1;
                }
            }
        }
        // the host player's own acts
        for (act, applied) in std::mem::take(&mut gs.coop.interact.acts) {
            if let Act::Transition { .. } = act {
                match self.decide(live, &mut outs, PlayerId::HOST, None, &act, now_ms) {
                    Some(o) if o.ok => {
                        if let Some((f, who)) = o.follow {
                            let io = live.interact.follow(f, &who);
                            outs.extend(send(live, io));
                        }
                        *gs.coop.interact.done.entry("transition").or_insert(0) += 1;
                    }
                    Some(_) => *gs.coop.interact.refused.entry("transition").or_insert(0) += 1,
                    None => {}
                }
                continue;
            }
            let shared = if applied && !matches!(act, Act::Discover { .. }) {
                let mut v = Vec::new();
                if let Act::World(s) = &act {
                    v.push(s.clone());
                }
                if let Some(o) = act.object() {
                    v.extend(snapshot_obj(cx, gs, FormId(o.0)));
                }
                v
            } else {
                let mut mint = || live.interact.mint_drop();
                let o = execute(cx, gs, PlayerId::HOST, &act, &mut mint);
                gs.coop.interact.count(&act, &o);
                self.executed += 1;
                o.shared
            };
            let io = live.interact.publish(shared);
            outs.extend(send(live, io));
        }
        // guests' requests, each exactly once
        for (p, txn, act) in live.interact.take_queue() {
            let outcome = if let Act::Transition { door, .. } = act {
                // the guest's world checked the lock, but the host decides: a
                // transition moves the party along
                if door_lets_through(cx.order, gs, p, FormId(door.0)) {
                    self.decide(live, &mut outs, p, Some(txn), &act, now_ms)
                } else {
                    Some(Outcome::refused(Why::Locked))
                }
            } else {
                let mut mint = || live.interact.mint_drop();
                Some(execute(cx, gs, p, &act, &mut mint))
            };
            if let Some(o) = outcome {
                gs.coop.interact.count(&act, &o);
                self.executed += 1;
                let io = live.interact.resolve(p, txn, o);
                outs.extend(send(live, io));
            }
        }
        // one-way transitions the party agreed to (or refused)
        for e in rule_events {
            match e {
                RulesEvent::BarrierReleased { tag, players, .. } => {
                    if let Some(pd) = self.pending.remove(tag) {
                        let who: Vec<PlayerId> =
                            players.iter().copied().filter(|p| *p != pd.from).collect();
                        let f = Follow {
                            door: pd.door,
                            to: pd.to,
                            leader: pd.from,
                        };
                        let io = match pd.txn {
                            Some(txn) => live.interact.resolve(
                                pd.from,
                                txn,
                                Outcome::done().with_follow(f, who),
                            ),
                            None => live.interact.follow(f, &who),
                        };
                        outs.extend(send(live, io));
                        *gs.coop.interact.done.entry("transition").or_insert(0) += 1;
                    }
                }
                RulesEvent::BarrierCancelled { tag, .. } => {
                    if let Some(pd) = self.pending.remove(tag) {
                        if let Some(txn) = pd.txn {
                            let io =
                                live.interact
                                    .resolve(pd.from, txn, Outcome::refused(Why::Policy));
                            outs.extend(send(live, io));
                        }
                        *gs.coop.interact.refused.entry("transition").or_insert(0) += 1;
                    }
                }
                _ => {}
            }
        }
        // what the shared layer addressed to the host player
        for f in std::mem::take(&mut own.interact.follows) {
            gs.coop.interact.follow = Some(f);
            gs.coop.interact.follows_in += 1;
        }
        for s in std::mem::take(&mut own.interact.shared) {
            apply_shared(cx, gs, &s);
        }
        own.interact.settled.clear();
        outs
    }
}

/// Guest side: requests out, shared records and destinations in.
#[derive(Default)]
pub struct GuestInteract {
    /// Requests sent.
    pub sent: u32,
}

impl GuestInteract {
    /// A fresh bridge.
    pub fn new() -> Self {
        Self::default()
    }

    /// Every frame on a guest; the returned messages go to the host.
    pub fn frame(&mut self, cx: Ctx, gs: &mut GameState, live: &mut GuestLive) -> Vec<Msg> {
        let mut out = Vec::new();
        for (act, _) in std::mem::take(&mut gs.coop.interact.acts) {
            self.sent += 1;
            out.push(live.interact.request(act));
        }
        for s in std::mem::take(&mut live.interact.shared) {
            apply_shared(cx, gs, &s);
            gs.coop.interact.shared_in += 1;
        }
        for st in std::mem::take(&mut live.interact.settled) {
            let o = Outcome {
                ok: st.ok,
                why: st.why,
                shared: Vec::new(),
                follow: None,
            };
            gs.coop.interact.count(&st.act, &o);
            if let (true, Act::Transition { door, to, .. }) = (st.ok, &st.act) {
                gs.coop.interact.follow = Some(Follow {
                    door: *door,
                    to: *to,
                    leader: PlayerId::HOST,
                });
            }
            gs.coop.interact.results.push(ActResult {
                act: st.act,
                ok: st.ok,
                why: st.why,
            });
        }
        for f in std::mem::take(&mut live.interact.follows) {
            gs.coop.interact.follow = Some(f);
            gs.coop.interact.follows_in += 1;
        }
        out
    }
}

/// Parses the words of a bot line after `coop` into an act. `form` turns an
/// editor id or hex form id into a form; `here` is the player's cell and
/// position (for drops).
pub fn parse_act(
    words: &[&str],
    form: &dyn Fn(&str) -> Option<FormId>,
    here: (u32, [i32; 3]),
) -> Option<Act> {
    let verb = words.first()?.to_ascii_lowercase();
    let f = |i: usize| words.get(i).and_then(|w| form(w));
    let n = |i: usize, d: u32| {
        words
            .get(i)
            .and_then(|w| w.parse::<u32>().ok())
            .unwrap_or(d)
    };
    let word = |i: usize| words.get(i).map(|w| w.to_ascii_lowercase());
    let c = |x: Option<FormId>| x.map(cf);
    Some(match verb.as_str() {
        "barter" => {
            let vendor = c(f(1))?;
            let total: i32 = words.get(2)?.parse().ok()?;
            let (mut buys, mut sells) = (Vec::new(), Vec::new());
            for w in &words[3..] {
                let mut p = w.split(':');
                let (kind, item, cnt) = (
                    p.next()?,
                    p.next()?,
                    p.next().and_then(|x| x.parse::<u32>().ok()).unwrap_or(1),
                );
                let item = cf(form(item)?);
                match kind {
                    "buy" => buys.push((item, cnt)),
                    "sell" => sells.push((item, cnt)),
                    _ => return None,
                }
            }
            Act::Barter {
                vendor,
                buys,
                sells,
                total,
            }
        }
        "door" => Act::Door { door: c(f(1))? },
        "activate" => Act::Activate { obj: c(f(1))? },
        "pickup" => Act::Pickup {
            obj: c(f(1))?,
            item: c(f(2))?,
            count: n(3, 1),
        },
        "drop" => Act::Drop {
            item: c(f(1))?,
            count: n(2, 1),
            cell: CellId(here.0),
            pos: here.1,
        },
        "steal" => Act::Steal {
            target: c(f(1))?,
            item: c(f(2))?,
            count: n(3, 1),
            mode: match word(4).as_deref() {
                Some("loot") => StealMode::Loot,
                Some("plant") => StealMode::Plant,
                _ => StealMode::Pickpocket,
            },
        },
        "craft" => Act::Craft {
            bench: c(f(1))?,
            recipe: c(f(2))?,
            times: n(3, 1),
        },
        "repair" => Act::Repair {
            item: c(f(1))?,
            part: cf(f(2).unwrap_or(FormId(0))),
            vendor: cf(f(3).unwrap_or(FormId(0))),
            cost: n(4, 0),
        },
        "use" => Act::Use { item: c(f(1))? },
        "equip" => Act::Equip {
            item: c(f(1))?,
            on: word(2).as_deref() != Some("off"),
        },
        "ammo" => Act::Ammo {
            ammo: cf(f(1).unwrap_or(FormId(0))),
        },
        "hotkey" => Act::Pip(PipOp::Hotkey {
            slot: n(1, 0) as u8,
            item: cf(f(2).unwrap_or(FormId(0))),
        }),
        "note" => Act::Pip(PipOp::Note(c(f(1))?)),
        "companion" => Act::Companion {
            who: c(f(1))?,
            cmd: match word(2)?.as_str() {
                "follow" => CompanionCmd::Follow,
                "wait" => CompanionCmd::Wait,
                "dismiss" => CompanionCmd::Dismiss,
                "recruit" => CompanionCmd::Recruit,
                "stimpak" => CompanionCmd::Stimpak,
                "gear" => CompanionCmd::SortGear,
                _ => return None,
            },
        },
        "lockpick" => {
            let r = word(2)?;
            Act::Lockpick {
                obj: c(f(1))?,
                won: r == "won",
                pins: n(3, 1) as u8,
                broke: r == "broke",
            }
        }
        "hack" => Act::Hack {
            obj: c(f(1))?,
            won: word(2)? == "won",
        },
        "key" => Act::Key {
            obj: c(f(1))?,
            key: c(f(2))?,
        },
        "trap" => Act::Trap {
            obj: c(f(1))?,
            what: match word(2)?.as_str() {
                "disarm" => TrapOutcome::Disarmed,
                "detonate" => TrapOutcome::Detonated,
                "destroy" => TrapOutcome::Destroyed,
                _ => return None,
            },
        },
        "discover" => Act::Discover { marker: c(f(1))? },
        "setlock" => {
            let level = match word(2)?.as_str() {
                "none" => None,
                w => Some(w.parse::<u8>().ok()?),
            };
            Act::World(Shared::Lock {
                obj: c(f(1))?,
                state: coop_core::interact::LockState {
                    level,
                    broken: false,
                },
            })
        }
        "enter" => Act::Transition {
            door: c(f(1))?,
            from: CellId(f(2)?.0),
            to: CellId(f(3)?.0),
            one_way: word(4).as_deref() == Some("oneway"),
        },
        // `levelup PERK|- [AV:N ...]`: the level-up menu's choices
        "levelup" => {
            let mut skills = Vec::new();
            for w in words.iter().skip(2) {
                let (av, pts) = w.split_once(':')?;
                skills.push((av.parse::<u16>().ok()?, pts.parse::<u32>().ok()?));
            }
            let perk = match words.get(1).copied() {
                None | Some("-") => FormId(0),
                Some(p) => form(p)?,
            };
            Act::LevelUp {
                skills,
                perk: cf(perk),
            }
        }
        // `caravan NPC STAKE won|lost`: a caravan game's result
        "caravan" => Act::Caravan {
            npc: c(f(1))?,
            stake: words.get(2)?.parse().ok()?,
            won: word(3)? == "won",
        },
        // `casino CASINO CHIPS WINNINGS [LEVEL] [new]`: a casino game closed
        "casino" => Act::Casino {
            casino: c(f(1))?,
            chips: words.get(2)?.parse().ok()?,
            winnings: words.get(3)?.parse().ok()?,
            level: n(4, 0).min(4) as u8,
            new_level: word(5).as_deref() == Some("new"),
        },
        _ => return None,
    })
}

/// The level-up menu's choices as an act (`Act::LevelUp`).
pub fn levelup_act(skills: &[(u16, i32)], perk: Option<FormId>) -> Act {
    Act::LevelUp {
        skills: skills
            .iter()
            .filter(|(_, n)| *n > 0)
            .map(|(av, n)| (*av, *n as u32))
            .collect(),
        perk: cf(perk.unwrap_or(FormId(0))),
    }
}

/// A caravan game's result as an act (`Act::Caravan`).
pub fn caravan_act(npc: FormId, stake: i32, won: bool) -> Act {
    Act::Caravan {
        npc: cf(npc),
        stake: stake.max(0) as u32,
        won,
    }
}

/// A casino game's closing as an act (`Act::Casino`).
pub fn casino_act(
    casino: FormId,
    chips: i32,
    data: crate::casino::CasinoData,
    new_level: bool,
) -> Act {
    Act::Casino {
        casino: cf(casino),
        chips: chips.max(0) as u32,
        winnings: data.winnings,
        level: data.level.clamp(0, 4) as u8,
        new_level,
    }
}

/// Test support (the bots' `coop give TARGET ITEM [N]`): puts items into a
/// player's shadow (`TARGET` a slot number) or into a holder (a reference).
/// Never on a guest. Returns whether anything was given.
pub fn bot_give(
    order: &esm::LoadOrder,
    gs: &mut GameState,
    target: &str,
    item: FormId,
    n: i32,
    holder: Option<FormId>,
) -> bool {
    if gs.coop.authority == Authority::Guest || n <= 0 {
        return false;
    }
    let add = |g: &mut GameState, h: FormId| {
        g.stock(order, h);
        *g.items.entry((h, item)).or_insert(0) += n;
        g.emit(super::Domain::Items, h);
    };
    match target.parse::<u8>() {
        Ok(slot) if slot <= 8 => {
            gs.with_player(slot, |g| add(g, crate::dialogue::PLAYER_REF));
            true
        }
        _ => match holder {
            Some(h) => {
                add(gs, h);
                true
            }
            None => false,
        },
    }
}

/// Words the bots resolve beyond editor ids and hex form ids. `any:noskill`
/// is the first recipe in load order that needs no skill and has an
/// ingredient and a product (the same on every peer).
pub fn special_form(order: &esm::LoadOrder, word: &str) -> Option<FormId> {
    if !word.eq_ignore_ascii_case("any:noskill") {
        return None;
    }
    crate::crafting::recipes(order)
        .into_iter()
        .find(|r| r.skill_level <= 0 && !r.ingredients.is_empty() && !r.outputs.is_empty())
        .map(|r| r.form_id)
}

/// Test support (`coop givefor SLOT RECIPE TIMES`): the ingredients of a
/// recipe, `times` over, into a player's shadow. Never on a guest.
pub fn bot_give_ingredients(
    order: &esm::LoadOrder,
    gs: &mut GameState,
    slot: &str,
    recipe: FormId,
    times: u32,
) -> bool {
    let Some(r) = crate::crafting::Recipe::load(order, recipe) else {
        return false;
    };
    let mut any = false;
    for c in &r.ingredients {
        any |= bot_give(order, gs, slot, c.item, (c.count * times) as i32, None);
    }
    any
}

/// Applies every shared record in `shared` (a convenience for tests).
pub fn apply_all(cx: Ctx, gs: &mut GameState, shared: &[Shared]) {
    for s in shared {
        apply_shared(cx, gs, s);
    }
}

/// Where a load door leads.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Arrival {
    /// The worldspace or interior cell to load.
    pub space: FormId,
    /// The cell the arrival marker stands in (an interior's own id).
    pub cell: FormId,
    /// True for an interior.
    pub interior: bool,
    /// Feet position on arrival.
    pub pos: [f32; 3],
    /// Heading on arrival (radians).
    pub heading: f32,
}

/// Where a [`Follow`] sends a player: through a load door's `XTEL`, else to a
/// placed reference (a script's `MoveTo` target).
pub fn arrival_of(order: &esm::LoadOrder, gs: &GameState, target: FormId) -> Option<Arrival> {
    door_arrival(order, target).or_else(|| {
        let (space, cell, pos, heading) = gs.place(order, target)?;
        Some(Arrival {
            space,
            cell,
            interior: space == cell,
            pos,
            heading,
        })
    })
}

/// Where the load door `door` leads (its `XTEL`), for a player who is told to
/// follow through it without having used it.
pub fn door_arrival(order: &esm::LoadOrder, door: FormId) -> Option<Arrival> {
    let rr = order.get(door)?;
    let record = rr.record().ok()?;
    let xtel = record
        .get(esm::FourCC::new(b"XTEL"))
        .filter(|s| s.data.len() >= 28)?;
    let d = &xtel.data;
    let u = |i: usize| u32::from_le_bytes([d[i], d[i + 1], d[i + 2], d[i + 3]]);
    let fl = |i: usize| f32::from_bits(u(i));
    let to = rr.plugin.to_global(FormId(u(0)));
    let to_rr = order.get(to)?;
    let cell = order.cell_of(&to_rr)?;
    let world = order.world_of(&to_rr);
    Some(Arrival {
        space: world.unwrap_or(cell),
        cell,
        interior: world.is_none(),
        pos: [fl(4), fl(8), fl(12)],
        heading: fl(24),
    })
}
