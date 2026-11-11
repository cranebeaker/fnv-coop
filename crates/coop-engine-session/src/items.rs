//! Loot taken once and barter through the host (docs/coop/SPECS.md S18).
//!
//! - **A guest's take.** A guest's game moves the item at once, as the game does. The guest sees
//!   it as a change of a holder whose menu its player opened (a container, a body, a trader or
//!   its merchant container) matched by the opposite change of its own player, and asks the host
//!   through coop-core's interaction layer: `Act::Steal` for one item taken or put, `Act::Barter`
//!   when items went both ways in one step (a purchase or a sale, the caps being the item 0xf),
//!   `Act::Pickup` for an item picked up from the world.
//! - **The host's game decides**, in the order the requests arrive: what is asked for must be in
//!   its own holder (or still lie in its own world), and then leaves it. A trade is carried out
//!   whole or not at all. The first taker wins; the next finds nothing and is refused.
//! - **The result.** The asker gets coop-core's verdict; refused, its game takes the item back
//!   from its player (and gives back what it put or sold). Every game gets the holder's contents
//!   as the host's game has them ([`CONTENTS_TAGS`]) and sets its own holder to them, and a
//!   picked-up item's reference leaves every world ([`GONE_TAG`]).

use std::collections::{BTreeMap, BTreeSet};

use coop_core::delta::StateStore;
use coop_core::ids::{FormId, PlayerId};
use coop_core::interact::{Act, InteractMsg, Outcome, StealMode, Why};
use coop_core::proto::Msg;
use coop_core::schema::{Field, MAX_CUSTOM_BYTES};
use coop_core::sync::{GuestPhase, GuestSync, HostSync};
use coop_engine::items::{self, Picked};
use coop_engine::{addr, gate, live, Change};
use engine_core::Engine;

/// The tags of the pieces of a holder's contents, in order. Each piece: the contents'
/// generation (two bytes), how many pieces there are (one byte), then its part of the bytes:
/// the count of players listed, each player's slot and the last request of that player the
/// host's game handled on this holder, then (item form id, count) pairs.
pub const CONTENTS_TAGS: [u16; 4] = [70, 71, 72, 73];
/// The tag of a placed item that was picked up: one byte, 1.
pub const GONE_TAG: u16 = 74;
/// The encodings' version, part of what two games must agree on at the handshake.
pub const VERSION: u32 = 1;

const PIECE_HEADER: usize = 3;
const PIECE_PAYLOAD: usize = MAX_CUSTOM_BYTES - PIECE_HEADER;
const INVENTORY_FLAGS: u32 = addr::CHANGE_REFR_INVENTORY | addr::CHANGE_REFR_LEVELED_INVENTORY;

/// What both games must agree on, for the handshake's rules line.
pub fn rules() -> String {
    format!(
        "items v{VERSION} contents {}..{} gone {GONE_TAG}",
        CONTENTS_TAGS[0], CONTENTS_TAGS[3]
    )
}

type Counts = BTreeMap<u32, i32>;

/// A holder's contents as the host publishes them.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Contents {
    /// The last request of each player (by slot) the host's game handled on this holder.
    applied: BTreeMap<u8, u32>,
    items: Counts,
}

impl Contents {
    fn encode(&self) -> Vec<u8> {
        let mut out = vec![self.applied.len() as u8];
        for (p, txn) in &self.applied {
            out.push(*p);
            out.extend(txn.to_le_bytes());
        }
        for (item, count) in self.items.iter().filter(|(_, c)| **c > 0) {
            out.extend(item.to_le_bytes());
            out.extend((*count as u32).to_le_bytes());
        }
        out
    }

    fn decode(bytes: &[u8]) -> Option<Contents> {
        let n = *bytes.first()? as usize;
        let rest = bytes.get(1..)?;
        if rest.len() < 5 * n || (rest.len() - 5 * n) % 8 != 0 {
            return None;
        }
        let (applied, pairs) = rest.split_at(5 * n);
        let word = |b: &[u8]| u32::from_le_bytes([b[0], b[1], b[2], b[3]]);
        Some(Contents {
            applied: applied
                .chunks_exact(5)
                .map(|c| (c[0], word(&c[1..])))
                .collect(),
            items: pairs
                .chunks_exact(8)
                .map(|c| (word(c), word(&c[4..]) as i32))
                .collect(),
        })
    }
}

/// The fields of `bytes` as generation `gen`; `None` when they do not fit the pieces.
fn pieces(gen: u16, bytes: &[u8]) -> Option<Vec<Field>> {
    let parts: Vec<&[u8]> = if bytes.is_empty() {
        vec![&[]]
    } else {
        bytes.chunks(PIECE_PAYLOAD).collect()
    };
    if parts.len() > CONTENTS_TAGS.len() {
        return None;
    }
    let total = parts.len() as u8;
    Some(
        parts
            .into_iter()
            .zip(CONTENTS_TAGS)
            .map(|(p, tag)| {
                let mut data = gen.to_le_bytes().to_vec();
                data.push(total);
                data.extend(p);
                Field::Custom { tag, data }
            })
            .collect(),
    )
}

/// Every holder's published contents in `store` whose pieces are all there and of one
/// generation: (holder, generation, bytes).
fn published(store: &StateStore) -> Vec<(u32, u16, Vec<u8>)> {
    let mut by_form: BTreeMap<u32, [Option<&Vec<u8>>; 4]> = BTreeMap::new();
    for (k, en) in store.iter() {
        if let Field::Custom { tag, data } = &en.field {
            if let Some(i) = CONTENTS_TAGS.iter().position(|t| t == tag) {
                by_form.entry(k.form.0).or_default()[i] = Some(data);
            }
        }
    }
    let mut out = Vec::new();
    for (form, parts) in by_form {
        let Some(first) = parts[0].filter(|d| d.len() >= PIECE_HEADER) else {
            continue;
        };
        let (gen, total) = ([first[0], first[1]], first[2] as usize);
        if total == 0 || total > parts.len() {
            continue;
        }
        let mut bytes = Vec::new();
        let whole = parts[..total].iter().all(|p| {
            p.is_some_and(|d| {
                let ok = d.len() >= PIECE_HEADER && d[..2] == gen && d[2] as usize == total;
                if ok {
                    bytes.extend(&d[PIECE_HEADER..]);
                }
                ok
            })
        });
        if whole {
            out.push((form, u16::from_le_bytes(gen), bytes));
        }
    }
    out
}

fn gone_field() -> Field {
    Field::Custom {
        tag: GONE_TAG,
        data: vec![1],
    }
}

fn is_gone(field: &Field) -> bool {
    matches!(field, Field::Custom { tag, data } if *tag == GONE_TAG && data == &[1])
}

fn inventory_changed(c: &Change) -> bool {
    c.flags & INVENTORY_FLAGS != 0
}

/// The host's side.
#[derive(Default)]
pub struct HostItems {
    started: bool,
    /// Holders whose contents are published: a guest asked for something of theirs, or the
    /// host's own player opened them.
    watched: BTreeSet<u32>,
    dirty: BTreeSet<u32>,
    applied: BTreeMap<u32, BTreeMap<u8, u32>>,
    written: BTreeMap<u32, (u16, Vec<u8>)>,
    /// Guests' takes and puts the host's game carried out.
    pub moved: u32,
    /// Guests' trades it carried out whole.
    pub traded: u32,
    /// Placed items it took out of its world for a guest.
    pub picked: u32,
    /// Requests it refused: the item, the stock, the caps or the placed item was no longer there.
    pub refused: u32,
    /// Holders whose contents do not fit the pieces: not published.
    pub too_large: u32,
    /// What was refused or not sent, for the log (taken by the caller).
    pub notes: Vec<String>,
}

impl HostItems {
    /// A host's item sharing.
    pub fn new() -> HostItems {
        HostItems::default()
    }

    /// Whether `act` is one this layer carries out.
    pub fn carries(act: &Act) -> bool {
        matches!(
            act,
            Act::Steal { .. } | Act::Barter { .. } | Act::Pickup { .. }
        )
    }

    fn refuse(&mut self, from: PlayerId, what: String, why: Why) -> Outcome {
        self.refused += 1;
        if self.notes.len() < 40 {
            self.notes
                .push(format!("player {}: {what}, refused ({why:?})", from.0));
        }
        Outcome::refused(why)
    }

    /// The holder a guest named, in the host's game now; the request is counted as handled on
    /// it, whatever comes of it.
    fn holder(&mut self, e: &mut Engine, from: PlayerId, txn: u32, id: u32) -> Option<u32> {
        if !items::shareable(e, id) {
            return None;
        }
        let r = items::holder(e, id)?;
        self.watched.insert(id);
        self.dirty.insert(id);
        self.applied.entry(id).or_default().insert(from.0, txn);
        Some(r)
    }

    /// Carries out in the host's game one request of a guest, called by the session's one
    /// dispatcher of interaction requests (`HostWorld::enact_with`, or [`HostItems::enact_alone`]
    /// when no world is shared). The host's game is asked at this moment: whoever's request is
    /// carried out first gets the item.
    pub fn enact(
        &mut self,
        e: &mut Engine,
        sync: &mut HostSync,
        from: PlayerId,
        txn: u32,
        act: &Act,
    ) -> Outcome {
        match act {
            Act::Steal {
                target,
                item,
                count,
                mode,
            } => {
                let Some(r) = self.holder(e, from, txn, target.0) else {
                    return self.refuse(
                        from,
                        format!("holder {:08x} is not in this game now", target.0),
                        Why::Unknown,
                    );
                };
                let n = i32::try_from(*count).unwrap_or(0);
                if n <= 0 {
                    return self.refuse(from, "a count of nothing".into(), Why::Invalid);
                }
                if *mode == StealMode::Plant {
                    if !items::add(e, r, item.0, n, None) {
                        return self.refuse(
                            from,
                            format!("item {:08x} is not in this game", item.0),
                            Why::Invalid,
                        );
                    }
                } else {
                    let have = items::count(e, r, item.0);
                    if have < n {
                        return self.refuse(
                            from,
                            format!(
                                "{n} of {:08x} from {:08x}, which has {have}",
                                item.0, target.0
                            ),
                            if have == 0 {
                                Why::Gone
                            } else {
                                Why::Insufficient
                            },
                        );
                    }
                    items::remove(e, r, item.0, n);
                }
                self.moved += 1;
                Outcome::done()
            }
            Act::Barter {
                vendor,
                buys,
                sells,
                ..
            } => {
                let Some(r) = self.holder(e, from, txn, vendor.0) else {
                    return self.refuse(
                        from,
                        format!("trader {:08x} is not in this game now", vendor.0),
                        Why::Unknown,
                    );
                };
                let mut wanted = Counts::new();
                for (item, count) in buys {
                    *wanted.entry(item.0).or_insert(0) += i32::try_from(*count).unwrap_or(i32::MAX);
                }
                for (item, n) in &wanted {
                    let have = items::count(e, r, *item);
                    if have < *n {
                        return self.refuse(
                            from,
                            format!(
                                "a trade for {n} of {item:08x} with {:08x}, which has {have}",
                                vendor.0
                            ),
                            Why::Insufficient,
                        );
                    }
                }
                if sells
                    .iter()
                    .any(|(item, _)| items::item_form(e, item.0).is_none())
                {
                    return self.refuse(from, "a sale of no item".into(), Why::Invalid);
                }
                for (item, n) in &wanted {
                    items::remove(e, r, *item, *n);
                }
                for (item, count) in sells {
                    items::add(e, r, item.0, i32::try_from(*count).unwrap_or(0), None);
                }
                self.traded += 1;
                Outcome::done()
            }
            Act::Pickup { obj, .. } => {
                if items::shareable(e, obj.0) && items::take_from_world(e, obj.0) {
                    sync.write(*obj, gone_field());
                    self.picked += 1;
                    Outcome::done()
                } else {
                    self.refuse(
                        from,
                        format!("placed item {:08x} is no longer there", obj.0),
                        Why::Gone,
                    )
                }
            }
            _ => Outcome::refused(Why::Policy),
        }
    }

    /// The dispatcher for a session that shares no world: this layer's requests are carried
    /// out, every other one is refused, so a guest is never left waiting.
    pub fn enact_alone(&mut self, e: &mut Engine, sync: &mut HostSync) -> Vec<(PlayerId, Msg)> {
        let mut out = Vec::new();
        for (from, txn, act) in sync.live.interact.take_queue() {
            let outcome = if HostItems::carries(&act) {
                self.enact(e, sync, from, txn, &act)
            } else {
                Outcome::refused(Why::Policy)
            };
            for o in sync.live.interact.resolve(from, txn, outcome) {
                out.extend(sync.live.route_extern(o.aud, Msg::Interact(o.msg)));
            }
        }
        out
    }

    /// Writes into the replica what `changes` (this step's journal) and the host's own player
    /// did: the contents of every published holder that changed, the holders the host's player
    /// opened, and the placed items it picked up.
    pub fn collect(&mut self, e: &mut Engine, sync: &mut HostSync, changes: &[Change]) {
        if !self.started {
            items::set_sharing(e, true);
            self.started = true;
        }
        for id in items::take_opened(e) {
            if items::shareable(e, id) && self.watched.insert(id) {
                self.dirty.insert(id);
            }
        }
        for p in items::take_picked(e) {
            if items::shareable(e, p.reference) {
                sync.write(FormId(p.reference), gone_field());
            }
        }
        for c in changes.iter().filter(|c| inventory_changed(c)) {
            if self.watched.contains(&c.form) {
                self.dirty.insert(c.form);
            }
        }
        for id in std::mem::take(&mut self.dirty) {
            let Some(r) = items::holder(e, id) else {
                continue;
            };
            let bytes = Contents {
                applied: self.applied.get(&id).cloned().unwrap_or_default(),
                items: items::totals(e, r),
            }
            .encode();
            let gen = match self.written.get(&id) {
                Some((_, last)) if *last == bytes => continue,
                Some((gen, _)) => gen.wrapping_add(1),
                None => 1,
            };
            match pieces(gen, &bytes) {
                Some(fields) => {
                    for f in fields {
                        sync.write(FormId(id), f);
                    }
                    self.written.insert(id, (gen, bytes));
                }
                None => {
                    self.too_large += 1;
                    if self.notes.len() < 40 {
                        self.notes.push(format!(
                            "holder {id:08x}: its contents are {} bytes, more than the pieces carry; not sent",
                            bytes.len()
                        ));
                    }
                }
            }
        }
    }
}

/// A guest's request the host has not answered, or whose effect the holder's published
/// contents do not show yet.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Asked {
    holder: u32,
    takes: Vec<(u32, i32)>,
    puts: Vec<(u32, i32)>,
}

/// The guest's side.
#[derive(Default)]
pub struct GuestItems {
    started: bool,
    /// Each opened or published holder's contents in this game when last looked at.
    base: BTreeMap<u32, Counts>,
    player_base: Option<Counts>,
    /// Requests awaiting the host's verdict, by request id.
    asked: BTreeMap<u32, Asked>,
    picks: BTreeMap<u32, Picked>,
    /// Requests the holders' published contents do not show yet, by request id.
    unshown: BTreeMap<u32, Asked>,
    /// The generation of each holder's contents last applied, and how many requests were not
    /// shown in it then.
    applied: BTreeMap<u32, (u16, usize)>,
    gone: BTreeSet<u32>,
    /// Own takes, puts, trades and pick-ups put to the host.
    pub asked_host: u32,
    /// Those the host refused: the items were taken back from this player (and what it gave was
    /// given back).
    pub taken_back: u32,
    /// Holders set to the host's contents.
    pub set: u32,
    /// Placed items taken out of this world because another player picked them up.
    pub removed: u32,
    /// What was refused, for the log (taken by the caller).
    pub notes: Vec<String>,
}

fn difference(now: &Counts, before: &Counts) -> Counts {
    let mut d = Counts::new();
    for (item, n) in now {
        let v = n - before.get(item).copied().unwrap_or(0);
        if v != 0 {
            d.insert(*item, v);
        }
    }
    for (item, n) in before {
        if !now.contains_key(item) && *n != 0 {
            d.insert(*item, -n);
        }
    }
    d
}

fn txn_of(msg: &Msg) -> Option<u32> {
    match msg {
        Msg::Interact(InteractMsg::Request { txn, .. }) => Some(*txn),
        _ => None,
    }
}

impl GuestItems {
    /// A guest's item sharing.
    pub fn new() -> GuestItems {
        GuestItems::default()
    }

    /// Requests the host has not answered yet.
    pub fn waiting(&self) -> usize {
        self.asked.len() + self.picks.len()
    }

    /// Reads what this guest's own player did since the last call (`changes` is this step's
    /// journal) and puts it to the host; takes back what the host refused. Answers the messages
    /// to send. Must run before anything empties the interaction layer's verdicts.
    pub fn observe(
        &mut self,
        e: &mut Engine,
        sync: &mut GuestSync,
        changes: &[Change],
    ) -> Vec<Msg> {
        if !self.started {
            items::set_sharing(e, true);
            self.started = true;
        }
        let mut out = Vec::new();
        let Some(player) = live::player(e) else {
            return out;
        };
        self.settle(e, sync, player);
        let active = sync.phase() == GuestPhase::Active;
        for id in items::take_opened(e) {
            if !items::shareable(e, id) {
                continue;
            }
            if let Some(r) = items::holder(e, id) {
                self.base.insert(id, items::totals(e, r));
                if self.player_base.is_none() {
                    self.player_base = Some(items::totals(e, player));
                }
            }
        }
        let picked = items::take_picked(e);
        for p in &picked {
            if !active || !items::shareable(e, p.reference) {
                continue;
            }
            let msg = sync.live.interact.request(Act::Pickup {
                obj: FormId(p.reference),
                item: FormId(p.item),
                count: p.count as u32,
            });
            if let Some(txn) = txn_of(&msg) {
                self.picks.insert(txn, *p);
                self.asked_host += 1;
                out.push(msg);
            }
        }
        let player_id = e.mem.u32(player + addr::FORM_ID_OFFSET);
        let named: BTreeSet<u32> = changes
            .iter()
            .filter(|c| inventory_changed(c) && self.base.contains_key(&c.form))
            .map(|c| c.form)
            .collect();
        let player_named = changes
            .iter()
            .any(|c| c.form == player_id && inventory_changed(c));
        if named.is_empty() && !player_named && picked.is_empty() {
            return out;
        }
        let now = items::totals(e, player);
        let mut gained = match &self.player_base {
            Some(before) => difference(&now, before),
            None => Counts::new(),
        };
        // What was picked up from the world is not from a holder.
        for p in &picked {
            if let Some(g) = gained.get_mut(&p.item) {
                *g -= p.count;
            }
        }
        self.player_base = Some(now);
        for id in named {
            let Some(r) = items::holder(e, id) else {
                continue;
            };
            let held = items::totals(e, r);
            let moved = difference(&held, &self.base[&id]);
            self.base.insert(id, held);
            let (mut takes, mut puts) = (Vec::new(), Vec::new());
            for (item, d) in moved {
                let g = gained.entry(item).or_insert(0);
                if d < 0 && *g > 0 {
                    let n = (-d).min(*g);
                    *g -= n;
                    takes.push((item, n));
                } else if d > 0 && *g < 0 {
                    let n = d.min(-*g);
                    *g += n;
                    puts.push((item, n));
                }
            }
            if !active || (takes.is_empty() && puts.is_empty()) {
                continue;
            }
            let pair = |v: &[(u32, i32)]| -> Vec<(FormId, u32)> {
                v.iter().map(|(i, n)| (FormId(*i), *n as u32)).collect()
            };
            let mut ask = |this: &mut GuestItems, act: Act, takes: Vec<_>, puts: Vec<_>| {
                let msg = sync.live.interact.request(act);
                if let Some(txn) = txn_of(&msg) {
                    let a = Asked {
                        holder: id,
                        takes,
                        puts,
                    };
                    this.asked.insert(txn, a.clone());
                    this.unshown.insert(txn, a);
                    this.asked_host += 1;
                    out.push(msg);
                }
            };
            if !takes.is_empty() && !puts.is_empty() {
                let caps = |v: &[(u32, i32)]| -> i32 {
                    v.iter()
                        .filter(|(i, _)| *i == addr::CAPS_FORM_ID)
                        .map(|(_, n)| *n)
                        .sum()
                };
                let act = Act::Barter {
                    vendor: FormId(id),
                    buys: pair(&takes),
                    sells: pair(&puts),
                    total: caps(&takes) - caps(&puts),
                };
                ask(self, act, takes, puts);
            } else {
                for (item, n) in takes {
                    let act = Act::Steal {
                        target: FormId(id),
                        item: FormId(item),
                        count: n as u32,
                        mode: StealMode::Loot,
                    };
                    ask(self, act, vec![(item, n)], Vec::new());
                }
                for (item, n) in puts {
                    let act = Act::Steal {
                        target: FormId(id),
                        item: FormId(item),
                        count: n as u32,
                        mode: StealMode::Plant,
                    };
                    ask(self, act, Vec::new(), vec![(item, n)]);
                }
            }
        }
        out
    }

    /// Takes this layer's verdicts out of the interaction layer and undoes on this player what
    /// the host refused.
    fn settle(&mut self, e: &mut Engine, sync: &mut GuestSync, player: u32) {
        let mut undone = false;
        let (asked, picks) = (&mut self.asked, &mut self.picks);
        let mut mine = Vec::new();
        sync.live.interact.settled.retain(|s| {
            if asked.contains_key(&s.txn) || picks.contains_key(&s.txn) {
                mine.push((s.txn, s.ok, s.why));
                false
            } else {
                true
            }
        });
        for (txn, ok, why) in mine {
            let pick = self.picks.remove(&txn);
            let asked = self.asked.remove(&txn);
            if ok {
                continue;
            }
            self.taken_back += 1;
            undone = true;
            if let Some(p) = pick {
                items::remove(e, player, p.item, p.count);
                self.note(format!(
                    "placed item {:08x}: another player had it first; {} of {:08x} taken back",
                    p.reference, p.count, p.item
                ));
            }
            if let Some(a) = asked {
                for (item, n) in &a.takes {
                    items::remove(e, player, *item, *n);
                }
                for (item, n) in &a.puts {
                    items::add(e, player, *item, *n, None);
                }
                if why == Why::Unknown {
                    // The host has no such holder: no contents of it will ever come.
                    self.unshown.remove(&txn);
                }
                self.note(format!(
                    "holder {:08x}: the host refused ({why:?}); this player's items put back as they were",
                    a.holder
                ));
            }
        }
        if undone {
            self.player_base = Some(items::totals(e, player));
        }
    }

    fn note(&mut self, text: String) {
        if self.notes.len() < 40 {
            self.notes.push(text);
        }
    }

    /// Sets this game's holders to the contents the host published, less what this guest took
    /// and plus what it put in requests those contents do not show yet, and takes out of this
    /// world the placed items another player picked up. A holder or item that is not in this
    /// game now is tried again on the next call.
    pub fn apply(&mut self, e: &mut Engine, store: &StateStore, me: PlayerId) {
        for (id, gen, bytes) in published(store) {
            let Some(contents) = Contents::decode(&bytes) else {
                continue;
            };
            let shown = contents.applied.get(&me.0).copied().unwrap_or(0);
            self.unshown.retain(|txn, a| a.holder != id || *txn > shown);
            let open = self.unshown.values().filter(|a| a.holder == id).count();
            if self.applied.get(&id) == Some(&(gen, open)) {
                continue;
            }
            let Some(r) = items::holder(e, id) else {
                continue;
            };
            let mut want = contents.items;
            for a in self.unshown.values().filter(|a| a.holder == id) {
                for (item, n) in &a.takes {
                    *want.entry(*item).or_insert(0) -= n;
                }
                for (item, n) in &a.puts {
                    *want.entry(*item).or_insert(0) += n;
                }
            }
            for n in want.values_mut() {
                *n = (*n).max(0);
            }
            let have = items::totals(e, r);
            let mut changed = false;
            for (item, d) in difference(&want, &have) {
                changed = true;
                gate::applying(e, |e| {
                    if d > 0 {
                        items::add(e, r, item, d, None);
                    } else {
                        items::remove(e, r, item, -d);
                    }
                });
            }
            if changed {
                self.set += 1;
            }
            self.base.insert(id, items::totals(e, r));
            self.applied.insert(id, (gen, open));
        }
        let gone: Vec<u32> = store
            .iter()
            .filter(|(k, en)| is_gone(&en.field) && !self.gone.contains(&k.form.0))
            .map(|(k, _)| k.form.0)
            .collect();
        for id in gone {
            if gate::applying(e, |e| items::take_from_world(e, id)) {
                self.removed += 1;
            }
            self.gone.insert(id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contents_round_trip_in_pieces_and_malformed_ones_are_refused() {
        let c = Contents {
            applied: [(1, 7), (2, 0x0102_0304)].into_iter().collect(),
            items: [(0xf, 250), (0x0001_5169, 3)].into_iter().collect(),
        };
        let bytes = c.encode();
        assert_eq!(Contents::decode(&bytes), Some(c.clone()));
        assert_eq!(Contents::decode(&bytes[..bytes.len() - 1]), None);
        assert_eq!(Contents::decode(&[]), None);
        // An item counted nothing is not listed.
        let mut with_none = c.clone();
        with_none.items.insert(9, 0);
        assert_eq!(with_none.encode(), bytes);
        // One piece for a small holder, several for a trader's stock, none past four.
        assert_eq!(pieces(1, &bytes).unwrap().len(), 1);
        let stock = Contents {
            applied: BTreeMap::new(),
            items: (1..=200).map(|i| (i, 1)).collect(),
        };
        let fields = pieces(3, &stock.encode()).unwrap();
        assert_eq!(fields.len(), 4);
        assert!(fields.iter().all(Field::is_sane));
        assert!(pieces(1, &vec![0; PIECE_PAYLOAD * 4 + 1]).is_none());
        assert!(is_gone(&gone_field()));
        assert!(!is_gone(&fields[0]));
        assert!(rules().contains("items v1"));
        assert!(HostItems::carries(&Act::Pickup {
            obj: FormId(1),
            item: FormId(2),
            count: 1
        }));
        assert!(!HostItems::carries(&Act::Door { door: FormId(1) }));
    }

    #[test]
    fn a_difference_names_what_came_and_what_went() {
        let before: Counts = [(1, 2), (2, 5)].into_iter().collect();
        let now: Counts = [(1, 3), (3, 1)].into_iter().collect();
        let d = difference(&now, &before);
        assert_eq!(d, [(1, 1), (2, -5), (3, 1)].into_iter().collect());
        assert!(difference(&now, &now).is_empty());
    }
}
