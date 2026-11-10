//! Party saves (docs/coop/SPECS.md S15, S16): each guest's own part of the game, kept by the host
//! next to the host's saves and given back.
//!
//! - **A part** ([`Part`]) is what is a player's own and not the shared story's: where the player
//!   is, and the state of the quests each player runs for themself. It is a small versioned file
//!   of tagged sections; a reader skips a section it does not know, so later versions can add
//!   (the character's values, the inventory) without breaking older files.
//! - **Guest to host.** A guest reads its part every few seconds and, when it changed, sends it
//!   as claims ([`PART_UP_TAG`], cut into chunks). A claim is a report, never a write to the
//!   story.
//! - **Saving.** When the host's game writes a save (co-op sees the name at the script extender's
//!   seam), the host writes every part it holds as `<save>.<player key>.nvcoop` in its party
//!   folder. The game's own save file is not touched, and loads without co-op as it always did.
//! - **Loading, rejoining, joining late.** When the host's game reads a save, the parts kept for
//!   it go into the replicated store ([`PART_DOWN_TAG`]); when a guest leaves, its last part
//!   does too. So a join's snapshot carries them, and the store's hashes cover them. A guest
//!   takes the newest part published for its key: one from a load always, one from a leave only
//!   in a game that just started (a guest that merely lost the connection keeps its own state).
//!
//! A player is known by a key (a hash of the player's name, chosen on the guest), not by its slot
//! in the session, which changes from one session to the next.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use coop_core::ids::{FormId, PlayerId};
use coop_core::schema::Field;
use coop_engine::{gate, live, quest, vars};
use engine_core::Engine;

use crate::record;

/// The tag of a chunk of a guest's part on its way to the host (`Field::Custom`, a claim).
pub const PART_UP_TAG: u16 = 60;
/// The tag of a chunk of a part the host gives back (`Field::Custom` in the replicated store).
pub const PART_DOWN_TAG: u16 = 61;
/// The first form id of the store's part chunks: `ff` and `fe` name no plugin's forms. The part's
/// slot is the next byte, the chunk's index the last.
pub const PART_FORM_BASE: u32 = 0xfe50_0000;
/// The file's first bytes and its version.
pub const MAGIC: &[u8; 4] = b"NVCP";
/// The version this build writes. A reader refuses a higher one.
pub const VERSION: u8 = 1;
/// A guest sends its part at most this often.
pub const PART_EVERY_MS: u64 = 5000;
/// The largest part carried (255 chunks).
pub const PART_MAX: usize = CHUNK_PAYLOAD * 255;
/// The file name's ending.
pub const FILE_ENDING: &str = "nvcoop";

const SECTION_PLACE: u8 = 1;
const SECTION_QUEST: u8 = 2;
const CHUNK_HEADER: usize = 8 + 4 + 1 + 1 + 1;
const CHUNK_PAYLOAD: usize = 480;
const _: () = assert!(CHUNK_HEADER + CHUNK_PAYLOAD <= coop_core::schema::MAX_CUSTOM_BYTES);

/// A player's key: a hash (FNV-1a) of the name the player chose. The name itself never leaves
/// the guest.
pub fn key_of(name: &str) -> u64 {
    fnv(name.as_bytes())
}

fn fnv(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |h, b| {
        (h ^ u64::from(*b)).wrapping_mul(0x0000_0100_0000_01b3)
    })
}

/// Where a player is: the space (an interior cell's form id or a world space's), the position and
/// the heading.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Place {
    /// The space's form id.
    pub space: u32,
    /// The position.
    pub pos: [f32; 3],
    /// The heading, radians.
    pub heading: f32,
}

/// One of the player's own quests: its record as the story channel encodes one, and its script's
/// variables.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnQuest {
    /// The quest's form id.
    pub id: u32,
    /// The record ([`record::encode`]'s bytes).
    pub record: Vec<u8>,
    /// (variable id, the `f64`'s bits).
    pub vars: Vec<(u32, u64)>,
}

/// A player's own part of the game.
#[derive(Debug, Clone, PartialEq)]
pub struct Part {
    /// Whose it is.
    pub key: u64,
    /// Where the player is (`None`: not in the world when it was read).
    pub place: Option<Place>,
    /// The player's own quests.
    pub quests: Vec<OwnQuest>,
}

/// What applying a part did.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Applied {
    /// The player was asked to be moved to the saved place.
    pub placed: bool,
    /// Quests put back.
    pub quests: u32,
    /// Quests or variables the game refused (its data differ).
    pub refused: u32,
}

fn section(out: &mut Vec<u8>, tag: u8, body: &[u8]) {
    out.push(tag);
    out.extend((body.len() as u32).to_le_bytes());
    out.extend(body);
}

struct Cursor<'a>(&'a [u8]);

impl<'a> Cursor<'a> {
    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        if self.0.len() < n {
            return None;
        }
        let (head, rest) = self.0.split_at(n);
        self.0 = rest;
        Some(head)
    }
    fn u8(&mut self) -> Option<u8> {
        Some(self.take(1)?[0])
    }
    fn u16(&mut self) -> Option<u16> {
        Some(u16::from_le_bytes(self.take(2)?.try_into().ok()?))
    }
    fn u32(&mut self) -> Option<u32> {
        Some(u32::from_le_bytes(self.take(4)?.try_into().ok()?))
    }
    fn u64(&mut self) -> Option<u64> {
        Some(u64::from_le_bytes(self.take(8)?.try_into().ok()?))
    }
    fn f32(&mut self) -> Option<f32> {
        self.u32().map(f32::from_bits)
    }
}

impl Part {
    /// The part as a file's bytes.
    pub fn encode(&self) -> Vec<u8> {
        let mut out = MAGIC.to_vec();
        out.push(VERSION);
        out.extend(self.key.to_le_bytes());
        if let Some(p) = &self.place {
            let mut body = p.space.to_le_bytes().to_vec();
            for v in p.pos.iter().chain([&p.heading]) {
                body.extend(v.to_bits().to_le_bytes());
            }
            section(&mut out, SECTION_PLACE, &body);
        }
        for q in &self.quests {
            let mut body = q.id.to_le_bytes().to_vec();
            body.extend((q.record.len() as u16).to_le_bytes());
            body.extend(&q.record);
            body.extend((q.vars.len() as u16).to_le_bytes());
            for (id, bits) in &q.vars {
                body.extend(id.to_le_bytes());
                body.extend(bits.to_le_bytes());
            }
            section(&mut out, SECTION_QUEST, &body);
        }
        out
    }

    /// Reads a file's bytes: `None` for anything that is not a part this build can read (another
    /// file, a higher version, a cut or malformed section). A section with a tag this build does
    /// not know is skipped.
    pub fn decode(bytes: &[u8]) -> Option<Part> {
        let mut c = Cursor(bytes);
        if c.take(4)? != MAGIC || c.u8()? > VERSION {
            return None;
        }
        let mut part = Part {
            key: c.u64()?,
            place: None,
            quests: Vec::new(),
        };
        while !c.0.is_empty() {
            let tag = c.u8()?;
            let len = c.u32()? as usize;
            let mut body = Cursor(c.take(len)?);
            match tag {
                SECTION_PLACE => {
                    part.place = Some(Place {
                        space: body.u32()?,
                        pos: [body.f32()?, body.f32()?, body.f32()?],
                        heading: body.f32()?,
                    });
                }
                SECTION_QUEST => {
                    let id = body.u32()?;
                    let n = body.u16()? as usize;
                    let record = body.take(n)?.to_vec();
                    let mut vars = Vec::new();
                    for _ in 0..body.u16()? {
                        vars.push((body.u32()?, body.u64()?));
                    }
                    part.quests.push(OwnQuest { id, record, vars });
                }
                _ => {}
            }
        }
        Some(part)
    }

    /// Reads the part of the player of game `e`: its place and the quests `own` (the player's own,
    /// by form id; one the game lacks or whose record cannot be carried is left out).
    pub fn read(e: &mut Engine, key: u64, own: &[u32]) -> Part {
        let place = live::local(e).map(|l| Place {
            space: l.cell,
            pos: l.pos,
            heading: l.heading,
        });
        let mut quests = Vec::new();
        for id in own {
            let Some(Field::Custom { data, .. }) =
                quest::record(e, *id).and_then(|r| record::encode(&r))
            else {
                continue;
            };
            quests.push(OwnQuest {
                id: *id,
                record: data,
                vars: vars::quest_vars(e, *id).unwrap_or_default(),
            });
        }
        Part { key, place, quests }
    }

    /// Puts the part back into game `e`: each quest's record and variables as on a load (no
    /// script runs for it), then the player is asked to be moved to the saved place.
    pub fn apply(&self, e: &mut Engine) -> Applied {
        let mut out = Applied::default();
        for q in &self.quests {
            let field = Field::Custom {
                tag: record::QUEST_RECORD_TAG,
                data: q.record.clone(),
            };
            let ok = match record::decode(q.id, &field) {
                Some(r) => gate::applying(e, |e| quest::apply(e, &r)),
                None => false,
            };
            if !ok {
                out.refused += 1;
                continue;
            }
            out.quests += 1;
            for (var, bits) in &q.vars {
                if !vars::set_quest_var(e, q.id, *var, *bits) {
                    out.refused += 1;
                }
            }
        }
        if let Some(p) = &self.place {
            out.placed = live::warp(e, p.space, p.pos, p.heading).is_ok();
        }
        out
    }
}

/// Why a part was published.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Origin {
    /// A guest's own upload (never in the store).
    Live = 0,
    /// The host's game read a save that has it.
    Load = 1,
    /// Its player left the session.
    Leave = 2,
}

/// One chunk: whose part, which generation of it, why, and where the chunk goes.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Chunk {
    key: u64,
    gen: u32,
    origin: u8,
    index: u8,
    total: u8,
    payload: Vec<u8>,
}

impl Chunk {
    fn encode(&self) -> Vec<u8> {
        let mut out = self.key.to_le_bytes().to_vec();
        out.extend(self.gen.to_le_bytes());
        out.extend([self.origin, self.index, self.total]);
        out.extend(&self.payload);
        out
    }

    fn decode(data: &[u8]) -> Option<Chunk> {
        let mut c = Cursor(data);
        let chunk = Chunk {
            key: c.u64()?,
            gen: c.u32()?,
            origin: c.u8()?,
            index: c.u8()?,
            total: c.u8()?,
            payload: c.0.to_vec(),
        };
        (chunk.index < chunk.total && chunk.payload.len() <= CHUNK_PAYLOAD).then_some(chunk)
    }
}

/// Cuts a part's bytes into chunk payloads; `None` for a part too large to carry (or empty).
fn cut(key: u64, gen: u32, origin: Origin, bytes: &[u8]) -> Option<Vec<Vec<u8>>> {
    if bytes.is_empty() || bytes.len() > PART_MAX {
        return None;
    }
    let pieces: Vec<&[u8]> = bytes.chunks(CHUNK_PAYLOAD).collect();
    let total = pieces.len() as u8;
    Some(
        pieces
            .into_iter()
            .enumerate()
            .map(|(i, p)| {
                Chunk {
                    key,
                    gen,
                    origin: origin as u8,
                    index: i as u8,
                    total,
                    payload: p.to_vec(),
                }
                .encode()
            })
            .collect(),
    )
}

/// Chunks on their way to being whole parts, by (key, generation).
#[derive(Debug, Default)]
struct Assembler {
    parts: BTreeMap<(u64, u32), Pieces>,
}

/// A part's origin, its chunk count, and the chunks that came.
type Pieces = (u8, u8, BTreeMap<u8, Vec<u8>>);

impl Assembler {
    /// Takes one chunk; a part that is now whole comes back as (key, generation, origin, bytes).
    fn push(&mut self, data: &[u8]) -> Option<(u64, u32, u8, Vec<u8>)> {
        let c = Chunk::decode(data)?;
        let id = (c.key, c.gen);
        let entry = self
            .parts
            .entry(id)
            .or_insert((c.origin, c.total, BTreeMap::new()));
        if entry.1 != c.total {
            *entry = (c.origin, c.total, BTreeMap::new());
        }
        entry.2.insert(c.index, c.payload);
        if entry.2.len() < usize::from(entry.1) {
            return None;
        }
        let (origin, _, pieces) = self.parts.remove(&id)?;
        // Only the newest generation of a key is worth keeping pieces of.
        self.parts.retain(|k, _| k.0 != c.key || k.1 > c.gen);
        Some((
            c.key,
            c.gen,
            origin,
            pieces.into_values().flatten().collect(),
        ))
    }
}

/// A save's name as the stem of the party's files: the last path component without `.fos`, with
/// every character a file name may not hold made `_`.
pub fn stem(save: &str) -> String {
    let last = save.rsplit(['/', '\\']).next().unwrap_or(save);
    let last = match last.len().checked_sub(4) {
        Some(n) if last.is_char_boundary(n) && last[n..].eq_ignore_ascii_case(".fos") => &last[..n],
        _ => last,
    };
    let s: String = last
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || matches!(c, ' ' | '-' | '_' | '.') {
                c
            } else {
                '_'
            }
        })
        .collect();
    let s = s.trim_matches([' ', '.']).to_string();
    if s.is_empty() {
        "save".into()
    } else {
        s
    }
}

/// The host's side: the parts it holds, its party folder, and what it has published.
#[derive(Debug, Default)]
pub struct PartyHost {
    /// Where the party's files are kept (`None`: nothing is written or read).
    pub dir: Option<PathBuf>,
    /// The newest part of each key, as bytes.
    held: BTreeMap<u64, Vec<u8>>,
    keys: BTreeMap<PlayerId, u64>,
    incoming: BTreeMap<PlayerId, Assembler>,
    present: BTreeSet<PlayerId>,
    /// The store slot of each key, in the order they were first published.
    slots: Vec<u64>,
    gen: u32,
    /// Saves the party's files were written for, and how many files.
    pub saved: u32,
    /// Files written.
    pub files_written: u32,
    /// Parts given back after a load.
    pub given_back: u32,
    /// Parts published because their player left.
    pub left_behind: u32,
    /// Uploads that were not a readable part, or files that could not be read or written.
    pub bad: u32,
    /// What happened, for the log (taken by the caller).
    pub notes: Vec<String>,
}

impl PartyHost {
    /// The part held for `key`.
    pub fn held(&self, key: u64) -> Option<Part> {
        Part::decode(self.held.get(&key)?)
    }

    /// Takes a guest's upload chunk.
    pub fn on_claim(&mut self, from: PlayerId, field: &Field) {
        let Field::Custom { data, .. } = field else {
            return;
        };
        let Some((key, _, _, bytes)) = self.incoming.entry(from).or_default().push(data) else {
            return;
        };
        match Part::decode(&bytes) {
            Some(p) if p.key == key => {
                self.keys.insert(from, key);
                self.held.insert(key, bytes);
            }
            _ => self.bad += 1,
        }
    }

    /// The store writes that publish `key`'s part: (form, field) for each chunk.
    fn publish(&mut self, key: u64, origin: Origin) -> Vec<(FormId, Field)> {
        let Some(bytes) = self.held.get(&key).cloned() else {
            return Vec::new();
        };
        let slot = match self.slots.iter().position(|k| *k == key) {
            Some(s) => s,
            None if self.slots.len() < 256 => {
                self.slots.push(key);
                self.slots.len() - 1
            }
            None => return Vec::new(),
        };
        self.gen = self.gen.wrapping_add(1);
        let Some(chunks) = cut(key, self.gen, origin, &bytes) else {
            self.bad += 1;
            return Vec::new();
        };
        chunks
            .into_iter()
            .enumerate()
            .map(|(i, data)| {
                (
                    FormId(PART_FORM_BASE | (slot as u32) << 8 | i as u32),
                    Field::Custom {
                        tag: PART_DOWN_TAG,
                        data,
                    },
                )
            })
            .collect()
    }

    /// The players now in the session: a player that left has its last part published, so that
    /// a game it starts anew gets it back when it joins again.
    pub fn on_players(&mut self, now: &[PlayerId]) -> Vec<(FormId, Field)> {
        let now: BTreeSet<PlayerId> = now.iter().copied().collect();
        let gone: Vec<PlayerId> = self.present.difference(&now).copied().collect();
        self.present = now;
        let mut out = Vec::new();
        for p in gone {
            self.incoming.remove(&p);
            if let Some(key) = self.keys.remove(&p) {
                let writes = self.publish(key, Origin::Leave);
                if !writes.is_empty() {
                    self.left_behind += 1;
                    out.extend(writes);
                }
            }
        }
        out
    }

    fn file(&self, save: &str, key: u64) -> Option<PathBuf> {
        Some(
            self.dir
                .as_ref()?
                .join(format!("{}.{key:016x}.{FILE_ENDING}", stem(save))),
        )
    }

    /// The host's game wrote the save `save`: every part held is written next to it.
    pub fn on_saved(&mut self, save: &str) {
        let Some(dir) = self.dir.clone() else {
            return;
        };
        if self.held.is_empty() {
            return;
        }
        if std::fs::create_dir_all(&dir).is_err() {
            self.bad += 1;
            self.notes
                .push("the party folder could not be made; no party save written".into());
            return;
        }
        let mut written = 0;
        for (key, bytes) in &self.held {
            let Some(path) = self.file(save, *key) else {
                continue;
            };
            match std::fs::write(path, bytes) {
                Ok(()) => written += 1,
                Err(_) => self.bad += 1,
            }
        }
        self.saved += 1;
        self.files_written += written;
        self.notes.push(format!(
            "party save for \"{}\": {written} player file(s)",
            stem(save)
        ));
    }

    /// The host's game read the save `save`: the parts kept for it are held from now on and
    /// published, so each guest gets its own back.
    pub fn on_loaded(&mut self, save: &str) -> Vec<(FormId, Field)> {
        let Some(dir) = self.dir.clone() else {
            return Vec::new();
        };
        let prefix = format!("{}.", stem(save));
        let ending = format!(".{FILE_ENDING}");
        let mut found = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().into_owned();
                let Some(mid) = name
                    .strip_prefix(&prefix)
                    .and_then(|r| r.strip_suffix(&ending))
                else {
                    continue;
                };
                let Ok(key) = u64::from_str_radix(mid, 16) else {
                    continue;
                };
                match std::fs::read(entry.path()) {
                    Ok(bytes) if Part::decode(&bytes).is_some_and(|p| p.key == key) => {
                        found.push((key, bytes));
                    }
                    _ => self.bad += 1,
                }
            }
        }
        found.sort();
        let mut out = Vec::new();
        let mut given = 0;
        for (key, bytes) in found {
            self.held.insert(key, bytes);
            let writes = self.publish(key, Origin::Load);
            if !writes.is_empty() {
                given += 1;
                out.extend(writes);
            }
        }
        self.given_back += given;
        self.notes.push(format!(
            "party save for \"{}\": {given} player part(s) given back",
            stem(save)
        ));
        out
    }
}

/// The guest's side.
#[derive(Debug)]
pub struct PartyGuest {
    /// This player's key ([`key_of`]).
    pub key: u64,
    /// The quests this player runs for themself (form ids): their state is part of the part.
    pub own_quests: Vec<u32>,
    /// This game started with this driver: a part left behind by an earlier run is taken.
    fresh: bool,
    scanned: bool,
    sent: Vec<u8>,
    sent_ms: Option<u64>,
    up_gen: u32,
    seen: BTreeSet<(u32, u64)>,
    pending: Option<Part>,
    /// Parts put back into this game.
    pub restored: u32,
    /// Uploads sent.
    pub uploads: u32,
    /// What the last restore did.
    pub last: Applied,
    /// What happened, for the log (taken by the caller).
    pub notes: Vec<String>,
}

impl Default for PartyGuest {
    fn default() -> Self {
        PartyGuest {
            key: key_of("player"),
            own_quests: Vec::new(),
            fresh: true,
            scanned: false,
            sent: Vec::new(),
            sent_ms: None,
            up_gen: 0,
            seen: BTreeSet::new(),
            pending: None,
            restored: 0,
            uploads: 0,
            last: Applied::default(),
            notes: Vec::new(),
        }
    }
}

impl PartyGuest {
    /// Looks through the replica for a part published for this player that it has not taken, and
    /// puts it back once the player is in the world.
    pub fn restore<'a>(&mut self, e: &mut Engine, store: impl Iterator<Item = (u32, &'a Field)>) {
        let mut asm = Assembler::default();
        let mut newest: Option<(u32, u8, Vec<u8>)> = None;
        for (form, field) in store {
            let Field::Custom { tag, data } = field else {
                continue;
            };
            if *tag != PART_DOWN_TAG || form & 0xffff_0000 != PART_FORM_BASE {
                continue;
            }
            if let Some((key, gen, origin, bytes)) = asm.push(data) {
                if key == self.key && newest.as_ref().map_or(true, |n| gen > n.0) {
                    newest = Some((gen, origin, bytes));
                }
            }
        }
        if let Some((gen, origin, bytes)) = newest {
            if self.seen.insert((gen, fnv(&bytes))) {
                let wanted = origin == Origin::Load as u8 || self.fresh;
                match Part::decode(&bytes) {
                    Some(p) if wanted => self.pending = Some(p),
                    Some(_) => {}
                    None => self
                        .notes
                        .push("a part for this player could not be read".into()),
                }
            }
        }
        self.scanned = true;
        self.fresh = false;
        if self.pending.is_some() && live::local(e).is_some() {
            let part = self.pending.take().expect("checked");
            self.last = part.apply(e);
            self.restored += 1;
            self.notes.push(format!(
                "this player's saved part was put back: {} own quest(s), place {}, refused {}",
                self.last.quests,
                if self.last.placed {
                    "asked for"
                } else {
                    "not set"
                },
                self.last.refused
            ));
            // What is in the game now is what the host has.
            self.sent = part.encode();
        }
    }

    /// The claims that send this player's part to the host, when it is due and changed; empty
    /// otherwise (also while a saved part waits to be put back, and before the player is in the
    /// world).
    pub fn upload(&mut self, e: &mut Engine, now_ms: u64) -> Vec<(FormId, Field)> {
        if !self.scanned
            || self.pending.is_some()
            || self
                .sent_ms
                .is_some_and(|t| now_ms.saturating_sub(t) < PART_EVERY_MS)
            || live::local(e).is_none()
        {
            return Vec::new();
        }
        self.sent_ms = Some(now_ms);
        let bytes = Part::read(e, self.key, &self.own_quests).encode();
        if bytes == self.sent {
            return Vec::new();
        }
        self.up_gen = self.up_gen.wrapping_add(1);
        let Some(chunks) = cut(self.key, self.up_gen, Origin::Live, &bytes) else {
            self.notes
                .push("this player's part is too large to send".into());
            self.sent = bytes;
            return Vec::new();
        };
        self.sent = bytes;
        self.uploads += 1;
        chunks
            .into_iter()
            .enumerate()
            .map(|(i, data)| {
                (
                    FormId(i as u32),
                    Field::Custom {
                        tag: PART_UP_TAG,
                        data,
                    },
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn part() -> Part {
        Part {
            key: key_of("Sunny"),
            place: Some(Place {
                space: 0x0010_5a00,
                pos: [1.5, -2.0, 300.25],
                heading: 1.25,
            }),
            quests: vec![
                OwnQuest {
                    id: 0x0010_4c1c,
                    record: vec![1, 2, 3, 4],
                    vars: vec![(1, 7), (9, u64::MAX)],
                },
                OwnQuest {
                    id: 0x0010_2037,
                    record: vec![],
                    vars: vec![],
                },
            ],
        }
    }

    #[test]
    fn a_part_round_trips_and_a_bare_one_too() {
        let p = part();
        assert_eq!(Part::decode(&p.encode()), Some(p.clone()));
        let bare = Part {
            key: 5,
            place: None,
            quests: vec![],
        };
        assert_eq!(bare.encode().len(), 4 + 1 + 8);
        assert_eq!(Part::decode(&bare.encode()), Some(bare));
    }

    #[test]
    fn a_reader_skips_a_section_it_does_not_know_and_refuses_what_it_cannot_read() {
        let p = part();
        let mut bytes = p.encode();
        // A later version's section, at the end.
        section(&mut bytes, 0x7f, &[9, 9, 9]);
        assert_eq!(Part::decode(&bytes), Some(p.clone()));
        // Not a part, a higher version, a cut file.
        assert_eq!(Part::decode(b"FO3SAVEGAME"), None);
        let mut newer = p.encode();
        newer[4] = VERSION + 1;
        assert_eq!(Part::decode(&newer), None);
        // A cut file is refused, or read as the sections that are whole: never as the part.
        let whole = p.encode();
        for n in 0..whole.len() {
            assert_ne!(Part::decode(&whole[..n]), Some(p.clone()), "cut at {n}");
        }
        assert_eq!(Part::decode(&whole[..20]), None, "cut inside a section");
    }

    #[test]
    fn chunks_carry_a_large_part_in_any_order_and_newer_generations_win() {
        let mut p = part();
        p.quests[0].record = (0..2000u32).map(|i| i as u8).collect();
        let bytes = p.encode();
        let chunks = cut(p.key, 3, Origin::Load, &bytes).unwrap();
        assert!(chunks.len() > 4);
        assert!(chunks
            .iter()
            .all(|c| c.len() <= coop_core::schema::MAX_CUSTOM_BYTES));
        let mut asm = Assembler::default();
        let mut whole = None;
        for c in chunks.iter().rev() {
            whole = asm.push(c).or(whole);
        }
        assert_eq!(whole, Some((p.key, 3, Origin::Load as u8, bytes.clone())));
        // A chunk that is not one, and a part too large.
        assert_eq!(asm.push(&[1, 2, 3]), None);
        assert!(cut(1, 1, Origin::Live, &vec![0; PART_MAX + 1]).is_none());
        assert!(cut(1, 1, Origin::Live, &[]).is_none());
    }

    #[test]
    fn a_save_name_becomes_a_plain_file_stem() {
        assert_eq!(stem("Save 12 - Goodsprings.fos"), "Save 12 - Goodsprings");
        assert_eq!(stem("C:\\Saves\\quicksave.FOS"), "quicksave");
        assert_eq!(stem("a/b:c*d"), "b_c_d");
        assert_eq!(stem("..."), "save");
        assert_eq!(stem(""), "save");
    }

    #[test]
    fn keys_differ_by_name() {
        assert_ne!(key_of("Sunny"), key_of("Chet"));
        assert_eq!(key_of("Sunny"), key_of("Sunny"));
    }
}
