//! The rest of the world's state on the session (docs/coop/SPECS.md S12): the changed-form
//! stream from the host to the guests, and doors and locks both ways.
//!
//! - **Records.** The host's journal names each changed form. For a change the stream is meant
//!   for (`coop_engine::changes::class_of`) the host takes the form's record as the game would
//!   save it and writes it into the replica as one `Field::Custom` ([`RECORD_TAG`]); a guest's
//!   game loads it. The numbering the records' references use travels beside them
//!   ([`NUMBERS_TAG`]); a record says how much of it it needs and waits on a guest until that has
//!   arrived, until its form is in the guest's game and until that game can take it.
//! - **Doors.** Whether a placed object that opens is open ([`DOOR_TAG`]). The host polls the
//!   doors it has seen change, because a door shut again is not in the journal. A guest that
//!   opens or shuts one asks the host through coop-core's interaction layer
//!   (`Act::World(Shared::Door)`); the host's game does it, and the result reaches everyone.
//! - **Locks.** A placed object's lock: level, flags and failed tries ([`LOCK_TAG`]), so a lock
//!   picked, a terminal's lock opened and a lock broken by forcing are the same for all. A guest
//!   whose own game changed a lock sends the lock as a claim, as a death is claimed; the host's
//!   game sets it.
//!
//! A guest's own change that the host does not take up is put back to the host's state after
//! [`LOCAL_GRACE_STEPS`] steps.

use std::collections::{BTreeMap, BTreeSet};

use coop_core::delta::StateStore;
use coop_core::ids::{Epoch, FormId, PlayerId};
use coop_core::interact::{Act, Outcome, Shared, Why};
use coop_core::proto::Msg;
use coop_core::schema::{Field, MAX_CUSTOM_BYTES};
use coop_core::sync::{GuestPhase, GuestSync, HostSync};
use coop_engine::changes::{self, Applied, Class, Numbering, Record};
use coop_engine::world::{self, Lock};
use coop_engine::{addr, gate, Change};
use engine_core::Engine;

/// The tag of a form's change record: the size of the numbering it needs (four bytes), then the
/// record (`Record::to_bytes`).
pub const RECORD_TAG: u16 = 0x5743;
/// The tag of a piece of the numbering: up to [`NUMBERS_PER_FIELD`] form ids, four bytes each, on
/// the subject [`NUMBERS_FORM`] plus the piece's number.
pub const NUMBERS_TAG: u16 = 0x574e;
/// The tag of a door's state: one byte, 1 open.
pub const DOOR_TAG: u16 = 0x5744;
/// The tag of a lock: the level, the flags, then the tries (four bytes).
pub const LOCK_TAG: u16 = 0x574c;
/// The encodings' version, part of what two games must agree on at the handshake.
pub const VERSION: u32 = 1;
/// The subject of the numbering's first piece. No form of the game is written with
/// [`NUMBERS_TAG`], so the pieces cannot meet a real form's entry.
pub const NUMBERS_FORM: u32 = 0xfe00_0000;
/// Form ids in one piece of the numbering.
pub const NUMBERS_PER_FIELD: usize = 120;
/// How many steps a guest's own change of a door or a lock may stand against the host's state
/// before it is put back (the host's answer normally arrives within a few).
pub const LOCAL_GRACE_STEPS: u64 = 120;

/// What both games must agree on, for the handshake's rules line.
pub fn rules() -> String {
    format!(
        "world v{VERSION} record {RECORD_TAG:04x} numbers {NUMBERS_TAG:04x}/{NUMBERS_PER_FIELD} door {DOOR_TAG:04x} lock {LOCK_TAG:04x}"
    )
}

fn door_field(open: bool) -> Field {
    Field::Custom {
        tag: DOOR_TAG,
        data: vec![u8::from(open)],
    }
}

fn door_value(field: &Field) -> Option<bool> {
    match field {
        Field::Custom { tag, data } if *tag == DOOR_TAG && data.len() == 1 => Some(data[0] != 0),
        _ => None,
    }
}

fn lock_field(l: Lock) -> Field {
    let mut data = vec![l.level, l.flags];
    data.extend_from_slice(&l.tries.to_le_bytes());
    Field::Custom {
        tag: LOCK_TAG,
        data,
    }
}

fn lock_value(field: &Field) -> Option<Lock> {
    match field {
        Field::Custom { tag, data } if *tag == LOCK_TAG && data.len() == 6 => Some(Lock {
            level: data[0],
            flags: data[1],
            tries: u32::from_le_bytes([data[2], data[3], data[4], data[5]]),
        }),
        _ => None,
    }
}

fn record_field(needs: u32, record: &Record) -> Option<Field> {
    let mut data = needs.to_le_bytes().to_vec();
    data.extend(record.to_bytes());
    (data.len() <= MAX_CUSTOM_BYTES).then_some(Field::Custom {
        tag: RECORD_TAG,
        data,
    })
}

/// `(numbering size needed, record)` of a well-formed record field of form `form`.
fn record_value(form: u32, field: &Field) -> Option<(u32, Record)> {
    match field {
        Field::Custom { tag, data } if *tag == RECORD_TAG && data.len() >= 4 => Some((
            u32::from_le_bytes([data[0], data[1], data[2], data[3]]),
            Record::from_bytes(form, &data[4..])?,
        )),
        _ => None,
    }
}

/// A placed object (not an actor) both games know by id.
fn shared_ref(c: &Change) -> bool {
    c.kind == addr::FORM_TYPE_REFR && world::shareable(c.form)
}

/// The host's side.
pub struct HostWorld {
    /// The classes of change that travel as records. All of them at first; a run on a PC may take
    /// one out until its live load is proven there (docs/coop/DECISIONS.md D1, condition 1).
    pub classes: BTreeSet<Class>,
    /// Whether doors and locks are shared.
    pub doors_and_locks: bool,
    numbering: Numbering,
    /// How many of the numbering's ids are in the replica.
    numbers_written: usize,
    /// Doors seen changed (in this game or by a guest): their state is read every step.
    doors: BTreeSet<u32>,
    /// Records written into the replica.
    pub records: u32,
    /// Records that do not fit a field: not sent.
    pub too_large: u32,
    /// Guests' door requests and lock claims the host's game carried out.
    pub enacted: u32,
    /// Those it did not (the object is not in the host's game now, or is not what was claimed).
    pub refused: u32,
    /// What could not be sent or done, once each, for the log (taken by the caller).
    pub notes: Vec<String>,
    noted: BTreeSet<u32>,
}

impl Default for HostWorld {
    fn default() -> Self {
        HostWorld::new()
    }
}

impl HostWorld {
    /// A host's world sharing with every class on.
    pub fn new() -> HostWorld {
        HostWorld {
            classes: Class::ALL.into_iter().collect(),
            doors_and_locks: true,
            numbering: Numbering::new(),
            numbers_written: 0,
            doors: BTreeSet::new(),
            records: 0,
            too_large: 0,
            enacted: 0,
            refused: 0,
            notes: Vec::new(),
            noted: BTreeSet::new(),
        }
    }

    /// Writes into the replica what `changes` (this step's journal) say changed in the host's
    /// game: each qualifying form's record, the numbering's new ids, locks, and the state of
    /// every door seen so far.
    pub fn collect(&mut self, e: &mut Engine, sync: &mut HostSync, changes: &[Change]) {
        let mut records = Vec::new();
        for c in changes {
            if !world::shareable(c.form) {
                continue;
            }
            if self.doors_and_locks && shared_ref(c) {
                if c.flags & addr::CHANGE_OBJECT_OPEN_STATE != 0 {
                    self.doors.insert(c.form);
                }
                if c.flags & addr::CHANGE_OBJECT_EXTRA_LOCK != 0 {
                    if let Some(l) = world::lock(e, c.form) {
                        sync.write(FormId(c.form), lock_field(l));
                    }
                }
            }
            let Some(class) = changes::class_of(c.kind, c.flags) else {
                continue;
            };
            if !self.classes.contains(&class) {
                continue;
            }
            // Everything the save holds for the form, not only what changed this step: a guest
            // that loads the record then has the whole changed form.
            let flags = match changes::saved_flags(e, c.form) {
                0 => c.flags,
                f => f,
            };
            if let Some(r) = changes::capture(e, &mut self.numbering, c.form, flags) {
                records.push(r);
            }
        }
        self.numbering.grown(e);
        let needs = self.numbering.ids().len() as u32;
        for r in records {
            match record_field(needs, &r) {
                Some(f) => {
                    if sync.write(FormId(r.form), f).is_some() {
                        self.records += 1;
                    }
                }
                None => {
                    self.too_large += 1;
                    if self.noted.insert(r.form) {
                        self.notes.push(format!(
                            "form {:08x}: its record is {} bytes, more than a field carries ({MAX_CUSTOM_BYTES}); not sent",
                            r.form,
                            r.data.len()
                        ));
                    }
                }
            }
        }
        self.write_numbers(sync);
        if self.doors_and_locks {
            for id in self.doors.clone() {
                if let Some(open) = world::open(e, id) {
                    sync.write(FormId(id), door_field(open));
                }
            }
        }
    }

    /// Writes the numbering's pieces that changed since the last call.
    fn write_numbers(&mut self, sync: &mut HostSync) {
        let ids = self.numbering.ids();
        if ids.len() == self.numbers_written {
            return;
        }
        let first = self.numbers_written / NUMBERS_PER_FIELD;
        for (piece, chunk) in ids.chunks(NUMBERS_PER_FIELD).enumerate().skip(first) {
            let data = chunk.iter().flat_map(|id| id.to_le_bytes()).collect();
            sync.write(
                FormId(NUMBERS_FORM + piece as u32),
                Field::Custom {
                    tag: NUMBERS_TAG,
                    data,
                },
            );
        }
        self.numbers_written = ids.len();
    }

    /// Carries out in the host's game what guests asked for or reported: a door opened or shut
    /// (coop-core's interaction requests), a lock changed (claims). The host's game does it
    /// through its own functions, ungated, and the result reaches every guest through
    /// [`HostWorld::collect`]. Answers the messages to send. Requests this layer does not carry
    /// out are refused, so a guest is never left waiting.
    pub fn enact(&mut self, e: &mut Engine, sync: &mut HostSync) -> Vec<(PlayerId, Msg)> {
        let mut out = Vec::new();
        for (from, txn, act) in sync.live.interact.take_queue() {
            let outcome = match act {
                Act::World(Shared::Door { door, open })
                    if self.doors_and_locks && world::shareable(door.0) =>
                {
                    if world::set_open(e, door.0, open) {
                        self.doors.insert(door.0);
                        self.enacted += 1;
                        Outcome::done().with_shared(vec![Shared::Door { door, open }])
                    } else {
                        self.refused += 1;
                        self.notes.push(format!(
                            "door {:08x}: player {} moved it, but it is not in this game now",
                            door.0, from.0
                        ));
                        Outcome::refused(Why::Unknown)
                    }
                }
                _ => {
                    self.refused += 1;
                    Outcome::refused(Why::Policy)
                }
            };
            for o in sync.live.interact.resolve(from, txn, outcome) {
                out.extend(sync.live.route_extern(o.aud, Msg::Interact(o.msg)));
            }
        }
        let claims = std::mem::take(&mut sync.claims);
        for (from, form, field) in claims {
            let Some(l) = lock_value(&field) else {
                // Not a lock: another layer's claim.
                sync.claims.push((from, form, field));
                continue;
            };
            if self.doors_and_locks && world::shareable(form.0) && world::set_lock(e, form.0, l) {
                self.enacted += 1;
                sync.write(form, lock_field(l));
            } else {
                self.refused += 1;
                self.notes.push(format!(
                    "lock of {:08x}: player {} changed it, but it is not in this game now",
                    form.0, from.0
                ));
            }
        }
        out
    }
}

/// What `NV_COOP_WORLD` asks for: unset or `all` shares everything here, `doors` only doors and
/// locks (no record is taken or loaded), `off` nothing. A run on a PC that stops in a record's
/// load is tried again one step lower without a rebuild, as the live ladder is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Share {
    /// Nothing of the world beyond the story.
    Off,
    /// Doors and locks only.
    Doors,
    /// Records, doors and locks.
    All,
}

impl Share {
    /// The setting `value` (the environment variable's text) asks for.
    pub fn parse(value: &str) -> Share {
        match value.trim() {
            "off" | "0" => Share::Off,
            "doors" => Share::Doors,
            _ => Share::All,
        }
    }

    /// The line that says what is shared.
    pub fn line(self) -> &'static str {
        match self {
            Share::Off => "co-op world: off (NV_COOP_WORLD=off)",
            Share::Doors => "co-op world: doors and locks only (NV_COOP_WORLD=doors)",
            Share::All => "co-op world: changed forms' records, doors and locks",
        }
    }

    /// The host's side for this setting.
    pub fn host(self) -> Option<HostWorld> {
        let mut w = HostWorld::new();
        match self {
            Share::Off => return None,
            Share::Doors => w.classes.clear(),
            Share::All => {}
        }
        Some(w)
    }

    /// The guest's side for this setting. A guest loads whatever records its host sends.
    pub fn guest(self) -> Option<GuestWorld> {
        (self != Share::Off).then(GuestWorld::new)
    }
}

/// The guest's side.
#[derive(Default)]
pub struct GuestWorld {
    numbering: Numbering,
    /// The data of each world field last applied to this game, by form and tag.
    applied: BTreeMap<(u32, u16), Vec<u8>>,
    /// The replica's revision when everything in it was applied.
    settled_rev: Option<u64>,
    /// Doors whose state is read every step: flagged in this game's journal or set by the host.
    doors: BTreeSet<u32>,
    /// Each door's and lock's state this guest last agreed on with the host (applied from it, or
    /// told to it).
    door_known: BTreeMap<u32, bool>,
    lock_known: BTreeMap<u32, Lock>,
    /// The step at which this guest told the host of its own change, by form and tag.
    told: BTreeMap<(u32, u16), u64>,
    steps: u64,
    /// Records this game loaded.
    pub records_applied: u32,
    /// Records this game's own form did not give back byte for byte after loading: the live load
    /// of that kind of form is not proven by this run.
    pub records_differ: u32,
    /// Fields this game refused: a malformed one, or a form of another type (other data).
    pub refused: u32,
    /// Doors and locks set to the host's state.
    pub set: u32,
    /// Own door changes and lock changes told to the host.
    pub told_host: u32,
    /// Own changes put back because the host did not take them up.
    pub put_back: u32,
    /// What differed or was refused, once per form, for the log (taken by the caller).
    pub notes: Vec<String>,
    noted: BTreeSet<u32>,
}

impl GuestWorld {
    /// A guest's world sharing.
    pub fn new() -> GuestWorld {
        GuestWorld::default()
    }

    fn note(&mut self, form: u32, text: String) {
        if self.noted.len() < 40 && self.noted.insert(form) {
            self.notes.push(text);
        }
    }

    /// Tells the host what this guest's own game did to doors and locks: `changes` is this
    /// step's journal. Answers the messages to send. A state the host's replica already has, or
    /// one this guest applied from it, is not sent back.
    pub fn observe(
        &mut self,
        e: &mut Engine,
        sync: &mut GuestSync,
        changes: &[Change],
    ) -> Vec<Msg> {
        self.steps += 1;
        let mut locks = Vec::new();
        for c in changes.iter().filter(|c| shared_ref(c)) {
            if c.flags & addr::CHANGE_OBJECT_OPEN_STATE != 0 {
                self.doors.insert(c.form);
            }
            if c.flags & addr::CHANGE_OBJECT_EXTRA_LOCK != 0 {
                locks.push(c.form);
            }
        }
        let mut out = Vec::new();
        if sync.phase() != GuestPhase::Active {
            return out;
        }
        for id in self.doors.clone() {
            let Some(now) = world::open(e, id) else {
                continue;
            };
            let hosts = sync
                .store
                .get(FormId(id), &door_field(now))
                .and_then(|en| door_value(&en.field));
            if self.door_known.get(&id) == Some(&now) || hosts == Some(now) {
                continue;
            }
            self.door_known.insert(id, now);
            self.told.insert((id, DOOR_TAG), self.steps);
            self.told_host += 1;
            out.push(sync.live.interact.request(Act::World(Shared::Door {
                door: FormId(id),
                open: now,
            })));
        }
        let mut claims = Vec::new();
        for id in locks {
            let Some(now) = world::lock(e, id) else {
                continue;
            };
            let hosts = sync
                .store
                .get(FormId(id), &lock_field(now))
                .and_then(|en| lock_value(&en.field));
            if self.lock_known.get(&id) == Some(&now) || hosts == Some(now) {
                continue;
            }
            self.lock_known.insert(id, now);
            self.told.insert((id, LOCK_TAG), self.steps);
            self.told_host += 1;
            claims.push((FormId(id), lock_field(now)));
        }
        if !claims.is_empty() {
            out.extend(sync.send_intent(Epoch(0), claims));
        }
        // Verdicts and shared facts of the interaction layer are not kept: the replica's fields
        // are what this game follows.
        sync.live.interact.settled.clear();
        sync.live.interact.shared.clear();
        out
    }

    /// Applies to this game the world fields of `store` that it has not applied yet: the
    /// numbering first, then records, then doors and locks. A field whose form is not in this
    /// game now, or that the game cannot take now, stays due and is tried again on the next call.
    pub fn apply(&mut self, e: &mut Engine, store: &StateStore) {
        if self.settled_rev == Some(store.rev().0) {
            return;
        }
        // The host's numbering, whole, in order.
        let mut ids = Vec::new();
        for (k, en) in store.iter() {
            if let Field::Custom { tag, data } = &en.field {
                if *tag == NUMBERS_TAG && k.form.0 >= NUMBERS_FORM {
                    let piece = (k.form.0 - NUMBERS_FORM) as usize;
                    if piece * NUMBERS_PER_FIELD != ids.len() || data.len() % 4 != 0 {
                        // A piece is missing before this one: what follows waits.
                        break;
                    }
                    ids.extend(
                        data.chunks_exact(4)
                            .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]])),
                    );
                }
            }
        }
        // A list that is only the start of what this guest holds is a piece still on its way.
        if !self.numbering.ids().starts_with(&ids) && !self.numbering.follow(e, &ids) {
            // Another numbering (the host started a new session): every record again.
            self.applied.retain(|(_, tag), _| *tag != RECORD_TAG);
        }
        let mut waiting = false;
        let due: Vec<(u32, u16, Field)> = store
            .iter()
            .filter_map(|(k, en)| match &en.field {
                Field::Custom { tag, data }
                    if matches!(*tag, RECORD_TAG | DOOR_TAG | LOCK_TAG)
                        && self.applied.get(&(k.form.0, *tag)) != Some(data) =>
                {
                    Some((k.form.0, *tag, en.field.clone()))
                }
                _ => None,
            })
            .collect();
        // Records before doors and locks: a record's load sets a door's state from its flags,
        // and the door's own field is the newer word.
        for (form, tag, field) in due.iter().filter(|d| d.1 == RECORD_TAG) {
            let Some((needs, record)) = record_value(*form, field) else {
                self.refuse(*form, *tag, field, "a malformed record");
                continue;
            };
            if needs as usize > self.numbering.ids().len() {
                waiting = true;
                continue;
            }
            match changes::apply(e, &mut self.numbering, &record) {
                Applied::Done => {
                    self.records_applied += 1;
                    self.mark(*form, *tag, field);
                    let back = changes::capture(e, &mut self.numbering, *form, record.flags());
                    let strayed = self.numbering.strayed(e);
                    if strayed || back.as_ref() != Some(&record) {
                        self.records_differ += 1;
                        self.note(
                            *form,
                            format!(
                                "form {form:08x} (type {:#04x}, flags {:08x}): after loading the host's record this game's own record of it differs",
                                record.kind,
                                record.flags()
                            ),
                        );
                    }
                }
                Applied::WrongType => {
                    self.refuse(*form, *tag, field, "a form of another type here");
                }
                Applied::Absent | Applied::Busy | Applied::NoMemory => waiting = true,
            }
        }
        for (form, tag, field) in due.iter().filter(|d| d.1 != RECORD_TAG) {
            let done = match (door_value(field), lock_value(field)) {
                (Some(open), _) => self.apply_door(e, *form, open),
                (_, Some(l)) => self.apply_lock(e, *form, l),
                _ => {
                    self.refuse(*form, *tag, field, "a malformed door or lock");
                    continue;
                }
            };
            if done {
                self.mark(*form, *tag, field);
            } else {
                waiting = true;
            }
        }
        self.settled_rev = (!waiting).then_some(store.rev().0);
    }

    fn mark(&mut self, form: u32, tag: u16, field: &Field) {
        if let Field::Custom { data, .. } = field {
            self.applied.insert((form, tag), data.clone());
        }
    }

    fn refuse(&mut self, form: u32, tag: u16, field: &Field, why: &str) {
        self.refused += 1;
        self.mark(form, tag, field);
        self.note(form, format!("form {form:08x}: not applied ({why})"));
    }

    /// Sets this game's door to the host's state; false while the door is not in this game.
    fn apply_door(&mut self, e: &mut Engine, id: u32, open: bool) -> bool {
        let Some(now) = world::open(e, id) else {
            return false;
        };
        self.doors.insert(id);
        self.door_known.insert(id, open);
        self.told.remove(&(id, DOOR_TAG));
        if now != open {
            gate::applying(e, |e| world::set_open(e, id, open));
            self.set += 1;
        }
        true
    }

    /// Sets this game's lock to the host's; false while the object is not in this game.
    fn apply_lock(&mut self, e: &mut Engine, id: u32, l: Lock) -> bool {
        let Some(now) = world::lock(e, id) else {
            return false;
        };
        self.lock_known.insert(id, l);
        self.told.remove(&(id, LOCK_TAG));
        if now != l {
            gate::applying(e, |e| world::set_lock(e, id, l));
            self.set += 1;
        }
        true
    }

    /// Reads this game's doors and locks back against `store` and sets again what drifted from
    /// the host's state. A change this guest told the host of is left alone for
    /// [`LOCAL_GRACE_STEPS`] steps, then put back if the host's state still says otherwise.
    pub fn heal(&mut self, e: &mut Engine, store: &StateStore) {
        let entries: Vec<(u32, Field)> = store
            .iter()
            .filter(|(_, en)| {
                matches!(&en.field, Field::Custom { tag, .. } if matches!(*tag, DOOR_TAG | LOCK_TAG))
            })
            .map(|(k, en)| (k.form.0, en.field.clone()))
            .collect();
        for (form, field) in entries {
            let tag = if door_value(&field).is_some() {
                DOOR_TAG
            } else {
                LOCK_TAG
            };
            let differs = match (door_value(&field), lock_value(&field)) {
                (Some(open), _) => world::open(e, form).is_some_and(|now| now != open),
                (_, Some(l)) => world::lock(e, form).is_some_and(|now| now != l),
                _ => false,
            };
            if !differs {
                continue;
            }
            if let Some(at) = self.told.get(&(form, tag)) {
                if self.steps.saturating_sub(*at) < LOCAL_GRACE_STEPS {
                    continue;
                }
                self.put_back += 1;
            }
            match (door_value(&field), lock_value(&field)) {
                (Some(open), _) => {
                    self.apply_door(e, form, open);
                }
                (_, Some(l)) => {
                    self.apply_lock(e, form, l);
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fields_round_trip_and_malformed_ones_are_refused() {
        for open in [false, true] {
            assert_eq!(door_value(&door_field(open)), Some(open));
        }
        let l = Lock {
            level: 75,
            flags: 5,
            tries: 0x0102_0304,
        };
        assert_eq!(lock_value(&lock_field(l)), Some(l));
        assert_eq!(lock_value(&door_field(true)), None);
        assert_eq!(door_value(&lock_field(l)), None);
        let r = Record {
            form: 0x0010_5a10,
            kind: addr::FORM_TYPE_REFR,
            header: [0, 0, 1, 1, 0, 0, 0, 0, 0x1b],
            data: vec![0, 8, 0, 0, b'|'],
        };
        let f = record_field(7, &r).unwrap();
        assert!(f.is_sane());
        assert_eq!(record_value(r.form, &f), Some((7, r.clone())));
        // Too short for a header, and a record too large for a field.
        let short = Field::Custom {
            tag: RECORD_TAG,
            data: vec![0; 8],
        };
        assert_eq!(record_value(1, &short), None);
        let large = Record {
            data: vec![0; MAX_CUSTOM_BYTES],
            ..r
        };
        assert_eq!(record_field(0, &large), None);
        assert!(rules().contains("world v1"));
        assert_eq!(Share::parse(" off "), Share::Off);
        assert_eq!(Share::parse("doors"), Share::Doors);
        assert_eq!(Share::parse(""), Share::All);
        assert!(Share::Off.host().is_none() && Share::Off.guest().is_none());
        assert!(Share::Doors.host().unwrap().classes.is_empty());
        assert_eq!(Share::All.host().unwrap().classes.len(), Class::ALL.len());
    }
}
