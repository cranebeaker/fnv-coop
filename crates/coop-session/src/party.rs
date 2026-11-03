//! Party saves over the session: the host orders a save, every guest saves its own character,
//! and the host writes the whole party as one unit (`world::coop::party_save`).
//!
//! Also here: matching a rejoining guest to its saved character by a persistent player id, a
//! guest that brings a character it saved elsewhere, and the F9 semantics (the host's load
//! gives every connected guest its saved character back; a guest's own load restores only its
//! own character).
//!
//! The layer is off until the viewer (or a test) calls one of the `Session::party_*` setup
//! functions. All messages ride one session extension (`Ext::Party`) on the reliable channel.
//! Incoming messages are queued and handled in [`tick`], which has the `GameState`.
//!
//! Protocol, one guest:
//!
//! ```text
//! guest  Hello{key, offering}            until acknowledged (every second)
//! host   HelloAck{campaign, matched, want_offer}
//!          matched: the saved character for `key` was put into the guest's slot
//! guest  Chunk{kind 1, ..}               its own saved character, only when !matched && want_offer
//! host   OfferResult{ok, conflicts, ..}  baseline conflicts: informational, the host wins
//!
//! host   Order{seq, name}                a host save happened
//! guest  Chunk{kind 0, seq, ..}          its character (or Refused{seq, why})
//! host   writes the party folder; Stored{seq, ok} to the guests
//!
//! guest  Chunk{kind 2, ..}               F9 on a guest: restore this character
//! host   Reloaded{campaign}              after the host loaded a party save (F9 on the host)
//! ```

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use coop_core::ids::PlayerId;
use world::coop::party_char::{self, CharacterFile};
use world::coop::party_save::{self, GuestSidecar, PartySave};
use world::scripting::GameState;

use crate::ext::Ext;
use crate::json::V;
use crate::session::{made_up_id, send_ext, Session};
use crate::Role;

/// Payload bytes per chunk message.
pub const CHUNK_BYTES: usize = 400;
/// How long the host waits for the guests' characters before it writes its own copies.
pub const ORDER_WAIT_MS: u64 = 4000;
/// How long a guest keeps trying to save when the policy holds saving (a scene).
pub const GUEST_SAVE_WAIT_MS: u64 = 20_000;
/// Time between a guest's hello messages.
const HELLO_EVERY_MS: u64 = 1000;
/// Largest character accepted from a guest.
const MAX_CHARACTER: usize = 1 << 20;

/// What the party layer tells its owner.
#[derive(Clone, Debug, PartialEq)]
pub enum PartyEvent {
    /// Host: the party folder was written.
    HostSaved {
        /// Sequence of the save.
        seq: u32,
        /// Where.
        dir: PathBuf,
        /// Guest characters in it.
        guests: usize,
        /// Of those, how many the guests themselves answered (the rest are the host's copies).
        from_guests: usize,
    },
    /// Host: the party folder could not be written.
    HostSaveFailed {
        /// Sequence.
        seq: u32,
        /// Why.
        why: String,
    },
    /// Guest: its character was saved (and written to its local file when one is set).
    GuestSaved {
        /// Sequence of the host's order.
        seq: u32,
        /// The local copy.
        path: Option<PathBuf>,
    },
    /// Guest: the order could not be carried out.
    GuestSaveRefused {
        /// Sequence.
        seq: u32,
        /// Why.
        why: String,
    },
    /// Guest: the host told its campaign.
    Campaign {
        /// Campaign id.
        id: u64,
        /// The host restored this guest's saved character.
        matched: bool,
    },
    /// Guest: the host stored the party save of this sequence.
    Stored {
        /// Sequence.
        seq: u32,
        /// Whether it worked.
        ok: bool,
    },
    /// Guest: the host loaded a party save; the character this guest sees is the saved one.
    CharacterRestored,
    /// Guest: the host's answer to an offered character.
    OfferAnswered {
        /// Accepted.
        ok: bool,
        /// Shared quest stages on which the file and the host differ (the host's apply).
        conflicts: u32,
        /// Rows dropped because the campaign shares them.
        dropped: u32,
        /// Reason when refused.
        why: String,
    },
}

#[derive(Clone, Debug, PartialEq)]
enum Msg {
    Hello {
        key: u64,
        offering: bool,
    },
    HelloAck {
        campaign: u64,
        matched: bool,
        want_offer: bool,
    },
    Order {
        seq: u32,
        name: String,
    },
    Chunk {
        kind: u8,
        seq: u32,
        idx: u16,
        total: u16,
        data: Vec<u8>,
    },
    Refused {
        seq: u32,
        why: String,
    },
    Stored {
        seq: u32,
        ok: bool,
    },
    Reloaded {
        campaign: u64,
    },
    /// Host to one guest: the host installed a saved character for that guest's slot.
    /// The replicated shadow leaves the health damage out (each machine keeps its own
    /// player's health), so the saved health comes with this message.
    Health {
        damage: Option<f64>,
    },
    OfferResult {
        seq: u32,
        ok: bool,
        conflicts: u32,
        dropped: u32,
        why: String,
    },
}

struct W(Vec<u8>);

impl W {
    fn str(&mut self, s: &str) {
        let mut e = s.len().min(200);
        while !s.is_char_boundary(e) {
            e -= 1;
        }
        self.0.extend((e as u16).to_le_bytes());
        self.0.extend(&s.as_bytes()[..e]);
    }
}

struct R<'a>(&'a [u8]);

impl R<'_> {
    fn take(&mut self, n: usize) -> Option<&[u8]> {
        if self.0.len() < n {
            return None;
        }
        let (a, b) = self.0.split_at(n);
        self.0 = b;
        Some(a)
    }
    fn u8(&mut self) -> Option<u8> {
        self.take(1).map(|b| b[0])
    }
    fn u16(&mut self) -> Option<u16> {
        self.take(2).map(|b| u16::from_le_bytes([b[0], b[1]]))
    }
    fn u32(&mut self) -> Option<u32> {
        self.take(4)
            .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }
    fn u64(&mut self) -> Option<u64> {
        self.take(8)
            .and_then(|b| b.try_into().ok())
            .map(u64::from_le_bytes)
    }
    fn str(&mut self) -> Option<String> {
        let n = self.u16()? as usize;
        if n > 200 {
            return None;
        }
        String::from_utf8(self.take(n)?.to_vec()).ok()
    }
}

impl Msg {
    fn encode(&self) -> Vec<u8> {
        let mut w = W(Vec::new());
        match self {
            Msg::Hello { key, offering } => {
                w.0.push(1);
                w.0.extend(key.to_le_bytes());
                w.0.push(u8::from(*offering));
            }
            Msg::HelloAck {
                campaign,
                matched,
                want_offer,
            } => {
                w.0.push(2);
                w.0.extend(campaign.to_le_bytes());
                w.0.push(u8::from(*matched) | (u8::from(*want_offer) << 1));
            }
            Msg::Order { seq, name } => {
                w.0.push(3);
                w.0.extend(seq.to_le_bytes());
                w.str(name);
            }
            Msg::Chunk {
                kind,
                seq,
                idx,
                total,
                data,
            } => {
                w.0.push(4);
                w.0.push(*kind);
                w.0.extend(seq.to_le_bytes());
                w.0.extend(idx.to_le_bytes());
                w.0.extend(total.to_le_bytes());
                w.0.extend((data.len() as u16).to_le_bytes());
                w.0.extend(data);
            }
            Msg::Refused { seq, why } => {
                w.0.push(5);
                w.0.extend(seq.to_le_bytes());
                w.str(why);
            }
            Msg::Stored { seq, ok } => {
                w.0.push(6);
                w.0.extend(seq.to_le_bytes());
                w.0.push(u8::from(*ok));
            }
            Msg::Reloaded { campaign } => {
                w.0.push(7);
                w.0.extend(campaign.to_le_bytes());
            }
            Msg::Health { damage } => {
                w.0.push(9);
                w.0.push(u8::from(damage.is_some()));
                w.0.extend(damage.unwrap_or(0.0).to_bits().to_le_bytes());
            }
            Msg::OfferResult {
                seq,
                ok,
                conflicts,
                dropped,
                why,
            } => {
                w.0.push(8);
                w.0.extend(seq.to_le_bytes());
                w.0.push(u8::from(*ok));
                w.0.extend(conflicts.to_le_bytes());
                w.0.extend(dropped.to_le_bytes());
                w.str(why);
            }
        }
        w.0
    }

    fn decode(b: &[u8]) -> Option<Msg> {
        let mut r = R(b);
        let m = match r.u8()? {
            1 => Msg::Hello {
                key: r.u64()?,
                offering: r.u8()? != 0,
            },
            2 => {
                let campaign = r.u64()?;
                let f = r.u8()?;
                Msg::HelloAck {
                    campaign,
                    matched: f & 1 != 0,
                    want_offer: f & 2 != 0,
                }
            }
            3 => Msg::Order {
                seq: r.u32()?,
                name: r.str()?,
            },
            4 => {
                let (kind, seq, idx, total) = (r.u8()?, r.u32()?, r.u16()?, r.u16()?);
                let n = r.u16()? as usize;
                if n > CHUNK_BYTES * 2 || total == 0 || idx >= total {
                    return None;
                }
                Msg::Chunk {
                    kind,
                    seq,
                    idx,
                    total,
                    data: r.take(n)?.to_vec(),
                }
            }
            5 => Msg::Refused {
                seq: r.u32()?,
                why: r.str()?,
            },
            6 => Msg::Stored {
                seq: r.u32()?,
                ok: r.u8()? != 0,
            },
            7 => Msg::Reloaded { campaign: r.u64()? },
            9 => {
                let has = r.u8()? != 0;
                let d = f64::from_bits(r.u64()?);
                Msg::Health {
                    damage: (has && d.is_finite()).then_some(d),
                }
            }
            8 => Msg::OfferResult {
                seq: r.u32()?,
                ok: r.u8()? != 0,
                conflicts: r.u32()?,
                dropped: r.u32()?,
                why: r.str()?,
            },
            _ => return None,
        };
        r.0.is_empty().then_some(m)
    }
}

/// Pieces of one character on their way in.
#[derive(Default)]
struct Reasm {
    total: u16,
    parts: BTreeMap<u16, Vec<u8>>,
    bytes: usize,
}

impl Reasm {
    /// Adds a piece; the whole when the last one arrives.
    fn add(&mut self, idx: u16, total: u16, data: Vec<u8>) -> Option<Vec<u8>> {
        if self.total != 0 && self.total != total {
            *self = Reasm::default();
        }
        self.total = total;
        self.bytes += data.len();
        if self.bytes > MAX_CHARACTER {
            *self = Reasm::default();
            return None;
        }
        self.parts.insert(idx, data);
        if self.parts.len() == total as usize {
            let all: Vec<u8> = std::mem::take(&mut self.parts)
                .into_values()
                .flatten()
                .collect();
            *self = Reasm::default();
            return Some(all);
        }
        None
    }
}

struct Pending {
    seq: u32,
    dir: PathBuf,
    world_text: String,
    unix: u64,
    deadline_ms: u64,
    asked: BTreeSet<PlayerId>,
    answers: BTreeMap<PlayerId, world::coop::shadow::PlayerShadow>,
    fallback: BTreeSet<PlayerId>,
}

struct GuestOrder {
    seq: u32,
    since_ms: u64,
}

/// State of the party layer of one session.
pub(crate) struct PartyLayer {
    role: Role,
    events: Vec<PartyEvent>,
    inbox: Vec<(PlayerId, Msg)>,
    // shared
    campaign: u64,
    // host
    host_name: String,
    keys: BTreeMap<PlayerId, u64>,
    claimed: BTreeMap<u64, PlayerId>,
    eligible: BTreeSet<u64>,
    loaded: BTreeMap<u64, GuestSidecar>,
    reapply: bool,
    notify_reload: bool,
    pending: Option<Pending>,
    next_seq: u32,
    asm: BTreeMap<(PlayerId, u8, u32), Reasm>,
    // guest
    key: u64,
    char_path: Option<PathBuf>,
    offer: Option<CharacterFile>,
    acked: bool,
    last_hello_ms: Option<u64>,
    seen_orders: BTreeSet<u32>,
    order: Option<GuestOrder>,
    outgoing: Vec<Msg>,
    offer_seq: u32,
}

impl PartyLayer {
    fn new(role: Role) -> PartyLayer {
        PartyLayer {
            role,
            events: Vec::new(),
            inbox: Vec::new(),
            campaign: 0,
            host_name: String::new(),
            keys: BTreeMap::new(),
            claimed: BTreeMap::new(),
            eligible: BTreeSet::new(),
            loaded: BTreeMap::new(),
            reapply: false,
            notify_reload: false,
            pending: None,
            next_seq: 0,
            asm: BTreeMap::new(),
            key: 0,
            char_path: None,
            offer: None,
            acked: false,
            last_hello_ms: None,
            seen_orders: BTreeSet::new(),
            order: None,
            outgoing: Vec::new(),
            offer_seq: 0,
        }
    }
}

fn unix_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn chunks(kind: u8, seq: u32, bytes: &[u8]) -> Vec<Msg> {
    let total = bytes.len().div_ceil(CHUNK_BYTES).max(1) as u16;
    (0..total)
        .map(|i| {
            let a = i as usize * CHUNK_BYTES;
            let b = (a + CHUNK_BYTES).min(bytes.len());
            Msg::Chunk {
                kind,
                seq,
                idx: i,
                total,
                data: bytes[a..b].to_vec(),
            }
        })
        .collect()
}

impl Session {
    // ---- setup ---------------------------------------------------------

    /// Host: turns the party layer on. `campaign` 0 starts a new campaign with a fresh id.
    pub fn party_host_setup(&mut self, campaign: u64, host_name: &str) {
        let l = self
            .party
            .get_or_insert_with(|| Box::new(PartyLayer::new(Role::Host)));
        l.role = Role::Host;
        l.campaign = if campaign == 0 {
            made_up_id()
        } else {
            campaign
        };
        l.host_name = host_name.to_string();
    }

    /// Guest: turns the party layer on with this player's persistent id (never zero).
    /// `character_path` is where the guest keeps its own copy of every character save.
    pub fn party_guest_setup(&mut self, key: u64, character_path: Option<PathBuf>) {
        let l = self
            .party
            .get_or_insert_with(|| Box::new(PartyLayer::new(Role::Guest)));
        l.role = Role::Guest;
        l.key = key.max(1);
        l.char_path = character_path;
        l.acked = false;
        l.last_hello_ms = None;
    }

    /// Guest: a character saved elsewhere, offered when the host has none for this player.
    pub fn party_guest_offer(&mut self, file: CharacterFile) {
        if let Some(l) = &mut self.party {
            l.offer = Some(file);
        }
    }

    /// Guest: F9 on a guest. Restores only this player's character from `file`.
    pub fn party_guest_load(&mut self, file: &CharacterFile) {
        if let Some(l) = &mut self.party {
            if l.role != Role::Guest {
                return;
            }
            l.offer_seq += 1;
            let seq = l.offer_seq;
            l.outgoing.extend(chunks(2, seq, &file.to_bytes()));
        }
    }

    /// Host: the characters of a loaded party save. Guests that hello with a matching key get
    /// theirs. Also adopts the save's campaign.
    pub fn party_host_load(&mut self, save: &PartySave) {
        let l = self
            .party
            .get_or_insert_with(|| Box::new(PartyLayer::new(Role::Host)));
        l.campaign = save.campaign;
        l.loaded = save.guests.iter().map(|g| (g.key, g.clone())).collect();
        l.next_seq = l.next_seq.max(save.seq);
        // the campaign keeps its rules unless the host's flags chose others
        if !self.rules_given {
            self.set_rules(save.rules);
        }
    }

    /// Guest: F5 on a guest. Saves only this player's character, into the character file
    /// (`--coop-character`). No host involved.
    pub fn party_guest_save(&mut self, gs: &GameState) -> Result<PathBuf, String> {
        let name = self.name.clone();
        let l = self
            .party
            .as_ref()
            .filter(|l| l.role == Role::Guest)
            .ok_or("not a co-op guest")?;
        if gs.coop_save_blocked() {
            return Err("saving is held by a co-op scene".into());
        }
        let shadow = gs
            .shadow_of(gs.active_slot())
            .ok_or("this session keeps no separate characters")?;
        let path = l.char_path.clone().ok_or("no character file is set")?;
        let file = CharacterFile::new(
            l.key,
            &name,
            l.campaign,
            0,
            unix_now(),
            &shadow,
            party_char::baseline_of(gs),
        );
        party_char::write_character(&path, &file).map_err(|e| e.to_string())?;
        Ok(path)
    }

    /// Guest: F9 on a guest. Restores only this player's character from the character file.
    pub fn party_guest_load_local(&mut self) -> Result<(), String> {
        let l = self
            .party
            .as_ref()
            .filter(|l| l.role == Role::Guest)
            .ok_or("not a co-op guest")?;
        let path = l.char_path.clone().ok_or("no character file is set")?;
        let file = party_char::read_character(&path).map_err(|e| e.to_string())?;
        self.party_guest_load(&file);
        Ok(())
    }

    /// Host: the host just loaded a party save (F9): give every connected guest its saved
    /// character again and tell them.
    pub fn party_host_loaded(&mut self) {
        if let Some(l) = &mut self.party {
            l.reapply = true;
            l.notify_reload = true;
        }
    }

    /// Host: a save happened. Orders every guest to save its character, then writes the party
    /// folder `dir` with `world_text` (the host's world save) once they answered or the wait ran
    /// out. Returns the sequence number.
    pub fn party_begin_save(
        &mut self,
        gs: &GameState,
        dir: PathBuf,
        world_text: String,
        unix: u64,
    ) -> Result<u32, String> {
        if gs.coop_save_blocked() {
            return Err("saving is held by a co-op scene".into());
        }
        let now = self.now_ms;
        let Some(h) = &self.host else {
            return Err("only the host saves a party".into());
        };
        let asked: BTreeSet<PlayerId> = h.peers.keys().copied().collect();
        let Some(l) = &mut self.party else {
            return Err("party saves are not set up".into());
        };
        if l.pending.is_some() {
            return Err("the previous party save is still being collected".into());
        }
        if l.campaign == 0 {
            l.campaign = crate::session::made_up_id();
        }
        l.next_seq += 1;
        let seq = l.next_seq;
        let name = dir
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        l.pending = Some(Pending {
            seq,
            dir,
            world_text,
            unix: if unix == 0 { unix_now() } else { unix },
            deadline_ms: now + ORDER_WAIT_MS,
            asked: asked.clone(),
            answers: BTreeMap::new(),
            fallback: BTreeSet::new(),
        });
        for to in asked {
            send_ext(
                &mut self.net,
                to,
                &Ext::Party(
                    Msg::Order {
                        seq,
                        name: name.clone(),
                    }
                    .encode(),
                ),
            );
        }
        self.log_event("party_order", &[("seq", V::U(seq as u64))]);
        Ok(seq)
    }

    /// The campaign this session knows (0: none yet).
    pub fn party_campaign(&self) -> u64 {
        self.party.as_ref().map_or(0, |l| l.campaign)
    }

    /// Events since the last call.
    pub fn take_party_events(&mut self) -> Vec<PartyEvent> {
        self.party
            .as_mut()
            .map(|l| std::mem::take(&mut l.events))
            .unwrap_or_default()
    }

    /// Host: a save is being collected.
    pub fn party_save_pending(&self) -> bool {
        self.party.as_ref().is_some_and(|l| l.pending.is_some())
    }

    /// Host: the persistent id of a connected guest's slot, once it said hello.
    pub fn party_key_of(&self, slot: PlayerId) -> Option<u64> {
        self.party.as_ref().and_then(|l| l.keys.get(&slot).copied())
    }

    /// Hook: a party packet arrived.
    pub(crate) fn party_recv(&mut self, from: PlayerId, bytes: &[u8]) {
        let Some(l) = &mut self.party else { return };
        match Msg::decode(bytes) {
            Some(m) => l.inbox.push((from, m)),
            None => self.decode_errors += 1,
        }
    }

    /// Hook: once per frame, after the host collected its changes.
    pub(crate) fn party_tick(&mut self, gs: &mut GameState) {
        let Some(mut l) = self.party.take() else {
            return;
        };
        match l.role {
            Role::Host => host_tick(self, &mut l, gs),
            Role::Guest => guest_tick(self, &mut l, gs),
        }
        self.party = Some(l);
    }
}

fn send(s: &mut Session, to: PlayerId, m: &Msg) {
    send_ext(&mut s.net, to, &Ext::Party(m.encode()));
}

/// Host: installs a saved character for `slot` and sends that guest the saved health
/// (see [`Msg::Health`]). Returns false as `install_for_slot` does.
fn restore_slot(
    s: &mut Session,
    gs: &mut GameState,
    slot: PlayerId,
    sh: world::coop::shadow::PlayerShadow,
) -> bool {
    let damage = sh.damage;
    let ok = party_char::install_for_slot(gs, slot.0, sh);
    if ok {
        send(s, slot, &Msg::Health { damage });
    }
    ok
}

// ---- host --------------------------------------------------------------

fn name_of(s: &Session, slot: PlayerId) -> String {
    s.host
        .as_ref()
        .and_then(|h| h.peers.get(&slot))
        .and_then(|p| p.name.clone())
        .unwrap_or_else(|| format!("Guest {}", slot.0))
}

fn host_tick(s: &mut Session, l: &mut PartyLayer, gs: &mut GameState) {
    let connected: BTreeSet<PlayerId> = s
        .host
        .as_ref()
        .map(|h| h.peers.keys().copied().collect())
        .unwrap_or_default();
    // people who left stop being mapped; their claims stay so a rejoin finds its place
    l.keys.retain(|slot, _| connected.contains(slot));
    let inbox = std::mem::take(&mut l.inbox);
    let mut keep = Vec::new();
    for (from, m) in inbox {
        if !connected.contains(&from) {
            continue;
        }
        match m {
            Msg::Hello { key, offering } => {
                if !gs.coop.bank.enabled() {
                    keep.push((from, Msg::Hello { key, offering }));
                    continue;
                }
                host_hello(s, l, gs, from, key, offering);
            }
            Msg::Chunk {
                kind,
                seq,
                idx,
                total,
                data,
            } => {
                let whole = l
                    .asm
                    .entry((from, kind, seq))
                    .or_default()
                    .add(idx, total, data);
                if let Some(bytes) = whole {
                    l.asm.remove(&(from, kind, seq));
                    host_character(s, l, gs, from, kind, seq, &bytes);
                }
            }
            Msg::Refused { seq, why } => {
                if let Some(p) = l.pending.as_mut().filter(|p| p.seq == seq) {
                    p.fallback.insert(from);
                    s.log_event(
                        "party_refused",
                        &[("from", V::U(from.0 as u64)), ("why", V::S(&why))],
                    );
                }
            }
            _ => {}
        }
    }
    l.inbox = keep;
    // chunks of a character that never finished
    if l.asm.len() > 64 {
        l.asm.clear();
    }
    if l.reapply {
        l.reapply = false;
        if gs.coop.bank.enabled() {
            let slots: Vec<(PlayerId, u64)> = l.keys.iter().map(|(a, b)| (*a, *b)).collect();
            for (slot, key) in slots {
                if let Some(sc) = l.loaded.get(&key) {
                    if let Some(sh) = sc.shadow() {
                        let plan = gs
                            .coop
                            .bank
                            .plan
                            .clone()
                            .unwrap_or_else(world::coop::ShadowPlan::all);
                        let (sh, _) = party_char::sanitize_offer(sh, &plan, true);
                        restore_slot(s, gs, slot, sh);
                        l.claimed.insert(key, slot);
                        l.eligible.remove(&key);
                    }
                }
            }
        }
    }
    if l.notify_reload {
        l.notify_reload = false;
        for to in &connected {
            send(
                s,
                *to,
                &Msg::Reloaded {
                    campaign: l.campaign,
                },
            );
        }
    }
    host_pending(s, l, gs, &connected);
}

fn host_hello(
    s: &mut Session,
    l: &mut PartyLayer,
    gs: &mut GameState,
    from: PlayerId,
    key: u64,
    offering: bool,
) {
    if key == 0 || from.is_host() {
        return;
    }
    // two sessions with one key: the later one is told no match and keeps a fresh character
    if l.keys.iter().any(|(slot, k)| *k == key && *slot != from) {
        l.keys.retain(|_, k| *k != key);
    }
    l.keys.insert(from, key);
    let mut matched = false;
    match l.claimed.get(&key).copied() {
        Some(old) => {
            // the same player again, maybe under another slot: its character follows it
            if old != from {
                if let Some(sh) = gs.coop.bank.parked.remove(&old.0) {
                    party_char::install_for_slot(gs, from.0, sh);
                    gs.coop.bank.dirty.insert(old.0);
                }
                l.claimed.insert(key, from);
            }
            matched = true;
        }
        None => {
            if let Some(sc) = l.loaded.get(&key) {
                if let Some(sh) = sc.shadow() {
                    let plan = gs
                        .coop
                        .bank
                        .plan
                        .clone()
                        .unwrap_or_else(world::coop::ShadowPlan::all);
                    let (sh, _) = party_char::sanitize_offer(sh, &plan, true);
                    matched = restore_slot(s, gs, from, sh);
                }
            }
            l.claimed.insert(key, from);
            if matched {
                l.eligible.remove(&key);
            } else {
                l.eligible.insert(key);
            }
        }
    }
    let want_offer = !matched && offering && l.eligible.contains(&key);
    s.log_event(
        "party_hello",
        &[
            ("slot", V::U(from.0 as u64)),
            ("key", V::U(key)),
            ("matched", V::B(matched)),
        ],
    );
    send(
        s,
        from,
        &Msg::HelloAck {
            campaign: l.campaign,
            matched,
            want_offer,
        },
    );
}

fn host_character(
    s: &mut Session,
    l: &mut PartyLayer,
    gs: &mut GameState,
    from: PlayerId,
    kind: u8,
    seq: u32,
    bytes: &[u8],
) {
    let Some(&key) = l.keys.get(&from) else {
        return;
    };
    let file = match CharacterFile::from_bytes(bytes) {
        Ok(f) if f.key == key => f,
        _ => {
            if kind != 0 {
                send(
                    s,
                    from,
                    &Msg::OfferResult {
                        seq,
                        ok: false,
                        conflicts: 0,
                        dropped: 0,
                        why: "the character file is damaged".into(),
                    },
                );
            }
            return;
        }
    };
    let Some(shadow) = file.shadow() else { return };
    if kind == 0 {
        if let Some(p) = l
            .pending
            .as_mut()
            .filter(|p| p.seq == seq && p.asked.contains(&from))
        {
            p.answers.insert(from, shadow);
        }
        return;
    }
    // an offered character: at join (once, only when the host has none) or a guest's own F9
    let refuse = |s: &mut Session, why: &str| {
        send(
            s,
            from,
            &Msg::OfferResult {
                seq,
                ok: false,
                conflicts: 0,
                dropped: 0,
                why: why.into(),
            },
        )
    };
    if kind == 1 && !l.eligible.contains(&key) {
        return refuse(s, "the host already has a character for you");
    }
    if kind == 2 && file.campaign != 0 && l.campaign != 0 && file.campaign != l.campaign {
        return refuse(s, "that character belongs to another campaign");
    }
    if !gs.coop.bank.enabled() {
        return refuse(s, "this session keeps no separate characters");
    }
    let plan = gs
        .coop
        .bank
        .plan
        .clone()
        .unwrap_or_else(world::coop::ShadowPlan::all);
    let (sh, rep) =
        party_char::sanitize_offer(shadow, &plan, kind == 2 && file.campaign == l.campaign);
    let conflicts = party_char::baseline_diff(&file.baseline, gs).len() as u32;
    let ok = restore_slot(s, gs, from, sh);
    if ok {
        l.eligible.remove(&key);
    }
    s.log_event(
        "party_offer",
        &[
            ("slot", V::U(from.0 as u64)),
            ("kind", V::U(kind as u64)),
            ("ok", V::B(ok)),
            ("conflicts", V::U(conflicts as u64)),
        ],
    );
    send(
        s,
        from,
        &Msg::OfferResult {
            seq,
            ok,
            conflicts,
            dropped: rep.shared_quest_rows as u32,
            why: String::new(),
        },
    );
}

fn host_pending(
    s: &mut Session,
    l: &mut PartyLayer,
    gs: &mut GameState,
    connected: &BTreeSet<PlayerId>,
) {
    let now = s.now_ms;
    let Some(p) = &mut l.pending else { return };
    let mut waiting = false;
    for slot in &p.asked {
        let done =
            p.answers.contains_key(slot) || p.fallback.contains(slot) || !connected.contains(slot);
        waiting |= !done;
    }
    if waiting && now < p.deadline_ms {
        return;
    }
    let Some(p) = l.pending.take() else { return };
    let mut guests: BTreeMap<u64, GuestSidecar> = l.loaded.clone();
    let mut from_guests = 0;
    for slot in &p.asked {
        let Some(&key) = l
            .keys
            .get(slot)
            .or_else(|| l.claimed.iter().find(|(_, v)| *v == slot).map(|(k, _)| k))
        else {
            continue;
        };
        let name = name_of(s, *slot);
        let (shadow, from_guest) = match p.answers.get(slot) {
            Some(sh) => (Some(sh.clone()), true),
            None => (gs.shadow_of(slot.0), false),
        };
        if let Some(sh) = shadow {
            from_guests += usize::from(from_guest);
            guests.insert(
                key,
                GuestSidecar::from_shadow(key, &name, slot.0, from_guest, &sh),
            );
        }
    }
    let save = PartySave {
        campaign: l.campaign,
        host_name: l.host_name.clone(),
        seq: p.seq,
        saved_unix: p.unix,
        world_text: p.world_text,
        guests: guests.into_values().collect(),
        rules: s.rules().unwrap_or_default(),
    };
    let n = save.guests.len();
    let result = party_save::write_party(&p.dir, &save);
    let ok = result.is_ok();
    match result {
        Ok(()) => {
            l.loaded = save.guests.iter().map(|g| (g.key, g.clone())).collect();
            s.log_event(
                "party_saved",
                &[
                    ("seq", V::U(p.seq as u64)),
                    ("guests", V::U(n as u64)),
                    ("from_guests", V::U(from_guests as u64)),
                ],
            );
            l.events.push(PartyEvent::HostSaved {
                seq: p.seq,
                dir: p.dir.clone(),
                guests: n,
                from_guests,
            });
        }
        Err(e) => {
            s.log_event(
                "party_save_failed",
                &[("seq", V::U(p.seq as u64)), ("why", V::S(&e.to_string()))],
            );
            l.events.push(PartyEvent::HostSaveFailed {
                seq: p.seq,
                why: e.to_string(),
            });
        }
    }
    for slot in connected.intersection(&p.asked) {
        send(s, *slot, &Msg::Stored { seq: p.seq, ok });
    }
}

// ---- guest -------------------------------------------------------------

fn guest_tick(s: &mut Session, l: &mut PartyLayer, gs: &mut GameState) {
    let now = s.now_ms;
    let connected = s
        .guest
        .as_ref()
        .is_some_and(|g| g.ever_connected && !g.lost);
    for (_, m) in std::mem::take(&mut l.inbox) {
        match m {
            Msg::HelloAck {
                campaign,
                matched,
                want_offer,
            } => {
                l.acked = true;
                l.campaign = campaign;
                l.events.push(PartyEvent::Campaign {
                    id: campaign,
                    matched,
                });
                if want_offer {
                    if let Some(f) = l.offer.take() {
                        l.offer_seq += 1;
                        let seq = l.offer_seq;
                        l.outgoing.extend(chunks(1, seq, &f.to_bytes()));
                    }
                }
            }
            Msg::Order { seq, .. } => {
                if l.seen_orders.insert(seq) && l.order.as_ref().map_or(true, |o| o.seq < seq) {
                    l.order = Some(GuestOrder { seq, since_ms: now });
                }
            }
            Msg::Stored { seq, ok } => l.events.push(PartyEvent::Stored { seq, ok }),
            Msg::Reloaded { campaign } => {
                if campaign != 0 {
                    l.campaign = campaign;
                }
                l.events.push(PartyEvent::CharacterRestored);
            }
            Msg::Health { damage } => {
                let me = world::dialogue::PLAYER_REF;
                match damage {
                    Some(d) => gs.damage.insert(me, d),
                    None => gs.damage.remove(&me),
                };
            }
            Msg::OfferResult {
                ok,
                conflicts,
                dropped,
                why,
                ..
            } => {
                l.events.push(PartyEvent::OfferAnswered {
                    ok,
                    conflicts,
                    dropped,
                    why,
                });
            }
            _ => {}
        }
    }
    if !connected {
        // a lost host: say hello again once back
        l.acked = false;
        return;
    }
    if l.key != 0
        && !l.acked
        && l.last_hello_ms
            .map_or(true, |t| now.saturating_sub(t) >= HELLO_EVERY_MS)
    {
        l.last_hello_ms = Some(now);
        send(
            s,
            PlayerId::HOST,
            &Msg::Hello {
                key: l.key,
                offering: l.offer.is_some(),
            },
        );
    }
    if let Some(o) = &l.order {
        let seq = o.seq;
        let waited = now.saturating_sub(o.since_ms);
        match guest_save(s, l, gs, seq) {
            Ok(()) => l.order = None,
            Err(why) if waited >= GUEST_SAVE_WAIT_MS || why.starts_with("fatal:") => {
                let why = why.trim_start_matches("fatal:").trim().to_string();
                send(
                    s,
                    PlayerId::HOST,
                    &Msg::Refused {
                        seq,
                        why: why.clone(),
                    },
                );
                l.events.push(PartyEvent::GuestSaveRefused { seq, why });
                l.order = None;
            }
            Err(_) => {}
        }
    }
    for m in std::mem::take(&mut l.outgoing) {
        send(s, PlayerId::HOST, &m);
    }
}

fn guest_save(
    s: &mut Session,
    l: &mut PartyLayer,
    gs: &mut GameState,
    seq: u32,
) -> Result<(), String> {
    if gs.coop_save_blocked() {
        return Err("saving is held by a scene".into());
    }
    let Some(shadow) = gs.shadow_of(gs.active_slot()) else {
        return Err("fatal: this session keeps no separate characters".into());
    };
    let file = CharacterFile::new(
        l.key,
        &s.name,
        l.campaign,
        seq,
        unix_now(),
        &shadow,
        party_char::baseline_of(gs),
    );
    let mut path = None;
    if let Some(p) = &l.char_path {
        // the guest keeps its own copy even if the host never gets the message
        match party_char::write_character(p, &file) {
            Ok(()) => path = Some(p.clone()),
            Err(e) => s.log_event("party_local_copy_failed", &[("why", V::S(&e.to_string()))]),
        }
    }
    l.outgoing.extend(chunks(0, seq, &file.to_bytes()));
    s.log_event(
        "party_char_saved",
        &[
            ("seq", V::U(seq as u64)),
            ("level", V::U(shadow.level as u64)),
        ],
    );
    l.events.push(PartyEvent::GuestSaved { seq, path });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn messages_round_trip() {
        let all = [
            Msg::Hello {
                key: 9,
                offering: true,
            },
            Msg::HelloAck {
                campaign: 5,
                matched: true,
                want_offer: false,
            },
            Msg::Order {
                seq: 3,
                name: "Save 1.txt.partysave".into(),
            },
            Msg::Chunk {
                kind: 2,
                seq: 7,
                idx: 1,
                total: 3,
                data: vec![1, 2, 3],
            },
            Msg::Refused {
                seq: 3,
                why: "scene".into(),
            },
            Msg::Stored { seq: 3, ok: true },
            Msg::Reloaded { campaign: 77 },
            Msg::Health { damage: Some(12.5) },
            Msg::Health { damage: None },
            Msg::OfferResult {
                seq: 1,
                ok: false,
                conflicts: 4,
                dropped: 2,
                why: "no".into(),
            },
        ];
        for m in all {
            assert_eq!(Msg::decode(&m.encode()), Some(m));
        }
        assert_eq!(Msg::decode(&[]), None);
        assert_eq!(Msg::decode(&[99]), None);
        assert_eq!(Msg::decode(&[6, 1, 0, 0, 0, 1, 9]), None, "trailing bytes");
        assert_eq!(
            Msg::decode(
                &Msg::Chunk {
                    kind: 0,
                    seq: 1,
                    idx: 5,
                    total: 2,
                    data: vec![]
                }
                .encode()
            ),
            None
        );
    }

    #[test]
    fn chunks_reassemble_in_any_order_and_cap_size() {
        let bytes: Vec<u8> = (0..1000u32).map(|i| (i % 251) as u8).collect();
        let mut parts = chunks(0, 1, &bytes);
        assert_eq!(parts.len(), 3);
        parts.reverse();
        let mut r = Reasm::default();
        let mut out = None;
        for p in parts {
            if let Msg::Chunk {
                idx, total, data, ..
            } = p
            {
                out = r.add(idx, total, data);
            }
        }
        assert_eq!(out, Some(bytes));
        assert_eq!(
            chunks(0, 1, &[]).len(),
            1,
            "an empty character is one empty chunk"
        );
        let mut r = Reasm::default();
        assert_eq!(r.add(0, 3000, vec![0; MAX_CHARACTER + 1]), None);
    }
}
