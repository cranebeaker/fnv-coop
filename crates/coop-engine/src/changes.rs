//! The changed-form stream (docs/coop/DECISIONS.md D1, option C inside hybrid H; docs/coop/SPECS.md
//! S12): one form's change as the game's own save record, taken from the host's game and loaded
//! by a guest's.
//!
//! - [`capture`] does for one form what `BGSSaveLoadGame::SaveGame` and `UnloadForm` do for each
//!   changed form: a `BGSSaveFormBuffer` gets the form's header, the form checks its flags
//!   (`CheckSaveGame`), the initial data and the form's own `SaveGame` fill the buffer. The bytes
//!   are the record a `.fos` file holds for the form (`crates/fos` decodes them).
//! - [`apply`] does what `BGSSaveLoadGame::LoadForm` does when the game itself loads a form's
//!   stored record while playing: a `BGSLoadFormBuffer` with the record's bytes, the initial data,
//!   the form's own `LoadGame`, then the game's unload pass (`InitLoadGame`, `FinishLoadGame`).
//!   On a guest it runs inside the script gate.
//! - [`Numbering`]: a record names other forms by three-byte indices into a
//!   `BGSSaveLoadFormIDMap`, and each game has its own. Co-op keeps one map of its own per game,
//!   put in the game object's place only while a record is made or loaded: the host's grows as it
//!   captures, a guest's follows the host's list in the same order, so an index means the same
//!   form id on every machine and the game's own save numbering is never touched.
//!
//! Nothing here decides which forms travel; [`class_of`] names the kinds of change the stream is
//! meant for, in the order the decision lists them.

use engine_core::prelude::*;

use crate::{addr, forms, gate};

/// One form's change record: what the game writes for it into a save.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    /// The form's id.
    pub form: u32,
    /// The form's type (`TESForm::cFormType`): a guest whose form of that id is of another type
    /// has other data and refuses the record.
    pub kind: u8,
    /// The game's nine-byte form header: the form's index in the numbering (three bytes, most
    /// significant first), the change flags, the form info (save type) and the version.
    pub header: [u8; addr::FORM_HEADER_LEN],
    /// The record's data, as the form's `SaveGame` wrote it after the initial data.
    pub data: Vec<u8>,
}

impl Record {
    /// The change flags the record was written with.
    pub fn flags(&self) -> u32 {
        u32::from_le_bytes([
            self.header[3],
            self.header[4],
            self.header[5],
            self.header[6],
        ])
    }

    /// The form's index in the numbering (bit 0x800000: a form made while playing).
    pub fn index(&self) -> u32 {
        u32::from(self.header[0]) << 16 | u32::from(self.header[1]) << 8 | u32::from(self.header[2])
    }

    /// The save type (the form info's low six bits).
    pub fn save_type(&self) -> u8 {
        self.header[7] & 0x3f
    }

    /// The save version the record was written with.
    pub fn version(&self) -> u8 {
        self.header[8]
    }

    /// The record on the wire: the form type, the header, the data.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(1 + self.header.len() + self.data.len());
        out.push(self.kind);
        out.extend_from_slice(&self.header);
        out.extend_from_slice(&self.data);
        out
    }

    /// Form `form`'s record from its wire bytes; `None` when they are too short to be one.
    pub fn from_bytes(form: u32, bytes: &[u8]) -> Option<Record> {
        let (&kind, rest) = bytes.split_first()?;
        if rest.len() < addr::FORM_HEADER_LEN {
            return None;
        }
        let (header, data) = rest.split_at(addr::FORM_HEADER_LEN);
        Some(Record {
            form,
            kind,
            header: header.try_into().ok()?,
            data: data.to_vec(),
        })
    }
}

/// The kinds of change the stream carries, in the order docs/coop/DECISIONS.md D1 brings them in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Class {
    /// A placed object enabled or disabled, moved, opened, locked or unlocked.
    Reference,
    /// A placed container's contents.
    Container,
    /// A faction's flags, reactions and crime counts.
    Faction,
    /// A reputation's fame and infamy.
    Reputation,
    /// A note read.
    Note,
    /// A dialogue response said once.
    Topic,
}

impl Class {
    /// Every class.
    pub const ALL: [Class; 6] = [
        Class::Reference,
        Class::Container,
        Class::Faction,
        Class::Reputation,
        Class::Note,
        Class::Topic,
    ];
}

/// A reference's change flags that are news of its place and state.
const REFERENCE_NEWS: u32 = addr::CHANGE_FORM_FLAGS
    | addr::CHANGE_REFR_MOVE
    | addr::CHANGE_REFR_CELL_CHANGED
    | addr::CHANGE_OBJECT_EXTRA_LOCK
    | addr::CHANGE_OBJECT_OPEN_STATE;
/// A reference's change flags that are news of its contents.
const CONTAINER_NEWS: u32 =
    addr::CHANGE_REFR_INVENTORY | addr::CHANGE_OBJECT_EMPTY | addr::CHANGE_REFR_LEVELED_INVENTORY;

/// The class of a change of a form of type `kind` flagged with `flags`, or `None` when the stream
/// does not carry it: an actor, a cell, a quest (its own channel), a reference only pushed about
/// by physics or animated. A reference whose contents changed is a [`Class::Container`] whatever
/// else changed with it.
pub fn class_of(kind: u8, flags: u32) -> Option<Class> {
    match kind {
        addr::FORM_TYPE_REFR if flags & CONTAINER_NEWS != 0 => Some(Class::Container),
        addr::FORM_TYPE_REFR if flags & REFERENCE_NEWS != 0 => Some(Class::Reference),
        addr::FORM_TYPE_FACTION => Some(Class::Faction),
        addr::FORM_TYPE_REPUTATION => Some(Class::Reputation),
        addr::FORM_TYPE_NOTE => Some(Class::Note),
        addr::FORM_TYPE_INFO => Some(Class::Topic),
        _ => None,
    }
}

/// The game's `BGSSaveLoadGame`, once it has made it.
fn game(e: &mut Engine) -> Option<u32> {
    match e.mem.u32(addr::SAVE_LOAD_GAME) {
        0 => None,
        g => Some(g),
    }
}

/// The change flags the game's own save holds for form `id` now (0 for none, or before the game
/// has its save object): everything that differs from the plugins' data, not only what changed
/// last. A record is written with all of them, so loading it gives the whole changed form.
pub fn saved_flags(e: &mut Engine, id: u32) -> u32 {
    let Some(game) = game(e) else {
        return 0;
    };
    let map = e.mem.u32(game + addr::GAME_CHANGES_MAP_OFFSET);
    if map == 0 {
        return 0;
    }
    e.with_stack(4, |e, out| {
        e.mem.set_u32(out.addr(), 0);
        e.call(addr::CHANGES_MAP_GET_FLAGS, &args![map, out.addr(), id]);
        e.mem.u32(out.addr())
    })
}

/// Co-op's numbering of form ids for the records of one session (see the module's text): a
/// `BGSSaveLoadFormIDMap` of co-op's own in this game's memory, and the ids in index order (the
/// first has index 1; index 0 is the null id).
#[derive(Debug, Default)]
pub struct Numbering {
    /// The map in the game's memory, 0 until it is first needed.
    map: u32,
    /// The form id of each index from 1 on.
    ids: Vec<u32>,
}

impl Numbering {
    /// An empty numbering; its map is made when a record is first made or loaded.
    pub fn new() -> Numbering {
        Numbering::default()
    }

    /// The form ids in index order, from index 1.
    pub fn ids(&self) -> &[u32] {
        &self.ids
    }

    /// The map, made on first use and holding [`Numbering::ids`] in order.
    fn ensure(&mut self, e: &mut Engine) -> Option<u32> {
        if self.map == 0 {
            let block = e
                .call(addr::OPERATOR_NEW, &args![addr::FORM_ID_MAP_SIZE])
                .u32();
            if block == 0 {
                return None;
            }
            self.map = e
                .call(
                    addr::FORM_ID_MAP_CONSTRUCT,
                    &args![block, addr::FORM_ID_MAP_BUCKETS],
                )
                .u32();
            if self.map == 0 {
                return None;
            }
            for id in &self.ids {
                e.call(addr::FORM_ID_MAP_GET_OR_ADD, &args![self.map, *id]);
            }
        }
        Some(self.map)
    }

    /// How many indices the map has given out, the null id's among them.
    fn count(&self, e: &mut Engine) -> u32 {
        match self.map {
            0 => 1,
            map => e.mem.u32(map + addr::FORM_ID_MAP_NEXT_INDEX_OFFSET),
        }
    }

    /// Runs `f` with this numbering in the game object's place of its own form id map, and puts
    /// the game's own back afterwards, unwound or not. `None` before the game has its save object
    /// or when the map cannot be made.
    fn with<R>(&mut self, e: &mut Engine, f: impl FnOnce(&mut Engine) -> R) -> Option<R> {
        let game = game(e)?;
        let map = self.ensure(e)?;
        let slot = game + addr::GAME_FORM_ID_MAP_OFFSET;
        let own = e.mem.u32(slot);
        e.mem.set_u32(slot, map);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(&mut *e)));
        e.mem.set_u32(slot, own);
        match result {
            Ok(r) => Some(r),
            Err(payload) => std::panic::resume_unwind(payload),
        }
    }

    /// On the host, after records were captured: the form ids the captures added to the numbering
    /// are appended to [`Numbering::ids`]. Answers how many.
    pub fn grown(&mut self, e: &mut Engine) -> usize {
        let count = self.count(e);
        let before = self.ids.len();
        for index in before as u32 + 1..count {
            let id = e
                .call(addr::FORM_ID_MAP_RESOLVE, &args![self.map, index])
                .u32();
            self.ids.push(id);
        }
        self.ids.len() - before
    }

    /// On a guest: makes this numbering the host's `ids` (its whole list so far). A list that
    /// goes on from what this numbering holds is appended in order; any other list (a host that
    /// started a new session) replaces it. Answers false when the list was replaced: records
    /// loaded with the old numbering are to be loaded again.
    pub fn follow(&mut self, e: &mut Engine, ids: &[u32]) -> bool {
        if ids.starts_with(&self.ids) && self.count(e) as usize == self.ids.len() + 1 {
            for id in &ids[self.ids.len()..] {
                if self.map != 0 {
                    e.call(addr::FORM_ID_MAP_GET_OR_ADD, &args![self.map, *id]);
                }
                self.ids.push(*id);
            }
            return true;
        }
        let same = ids.starts_with(&self.ids);
        self.ids = ids.to_vec();
        self.rebuild(e);
        same
    }

    /// Empties the map and enters [`Numbering::ids`] again in order.
    fn rebuild(&mut self, e: &mut Engine) {
        if self.map == 0 {
            return;
        }
        e.call(addr::FORM_ID_MAP_RESET, &args![self.map]);
        for id in &self.ids {
            e.call(addr::FORM_ID_MAP_GET_OR_ADD, &args![self.map, *id]);
        }
    }

    /// On a guest, after it made a record of its own to compare: whether making it gave out
    /// indices the host's list does not have (the guest's form names a form the host's records
    /// never named). The map is then put back to the host's list, so a later index of the host's
    /// is not taken by this game's form.
    pub fn strayed(&mut self, e: &mut Engine) -> bool {
        if self.count(e) as usize == self.ids.len() + 1 {
            return false;
        }
        self.rebuild(e);
        true
    }
}

/// Form `id`'s record with change flags `flags`, as this game would save it now, its references
/// numbered by `numbering`. `None` when the form is not in this game now, `flags` is 0, the game
/// has no save object yet, or the form itself says nothing of it is to be saved (`CheckSaveGame`
/// left no flag).
pub fn capture(e: &mut Engine, numbering: &mut Numbering, id: u32, flags: u32) -> Option<Record> {
    if flags == 0 {
        return None;
    }
    let form = forms::form(e, id)?;
    let manager = match e.mem.u32(addr::SAVE_LOAD_MANAGER) {
        0 => return None,
        m => m,
    };
    let kind = e.mem.u8(form + addr::FORM_TYPE_OFFSET);
    numbering
        .with(e, |e| {
            let minor = e
                .call(addr::MANAGER_GET_MINOR_VERSION, &args![manager])
                .u32()
                & 0xff;
            e.with_stack(addr::SAVE_FORM_BUFFER_SIZE, |e, buffer| {
                let buffer = buffer.addr();
                let header_at = buffer + addr::FORM_BUFFER_HEADER_OFFSET;
                e.call(addr::SAVE_FORM_BUFFER_CONSTRUCT, &args![buffer]);
                e.call(
                    addr::SAVE_FORM_BUFFER_SET_HEADER,
                    &args![buffer, id, flags, u32::from(kind), minor],
                );
                e.call(addr::SAVE_FORM_BUFFER_SET_FORM, &args![buffer, form]);
                e.vcall(form, addr::FORM_SLOT_CHECK_SAVE_GAME, &args![buffer]);
                let checked = e.mem.u32(header_at + 3);
                let record = (checked != 0).then(|| {
                    e.call(addr::SAVE_INITIAL_DATA, &args![buffer, form, checked]);
                    e.vcall(form, addr::FORM_SLOT_SAVE_GAME, &args![buffer]);
                    let data = e.mem.u32(buffer + addr::BUFFER_DATA_OFFSET);
                    let len = e.mem.u32(buffer + addr::BUFFER_POSITION_OFFSET);
                    let mut header = [0u8; addr::FORM_HEADER_LEN];
                    header.copy_from_slice(&e.mem.bytes(header_at, addr::FORM_HEADER_LEN as u32));
                    // The size type (the form info's top two bits) is only set when a record
                    // goes to a file.
                    header[7] &= 0x3f;
                    Record {
                        form: id,
                        kind,
                        header,
                        data: if len == 0 {
                            Vec::new()
                        } else {
                            e.mem.bytes(data, len)
                        },
                    }
                });
                e.call(addr::SAVE_FORM_BUFFER_DESTRUCT, &args![buffer]);
                record
            })
        })
        .flatten()
}

/// What became of a record given to [`apply`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Applied {
    /// The form's `LoadGame` took the record and the game's unload pass finished it.
    Done,
    /// The form is not in this game now (its cell is not loaded): try again when it is.
    Absent,
    /// The game cannot take a record now (no save object yet, or a load is running): try again.
    Busy,
    /// This game's form of that id is of another type: the two games' data differ.
    WrongType,
    /// The game could not allocate the load buffer.
    NoMemory,
}

/// Loads `record` into this game's form of the same id, its references resolved by `numbering`,
/// the way the game loads a form's stored record while playing (`BGSSaveLoadGame::LoadForm`, then
/// the unload pass). No script runs on a guest while it does (the script gate's apply scope).
/// Nothing is reverted first: what the record's flags do not cover stays as it is in this game.
pub fn apply(e: &mut Engine, numbering: &mut Numbering, record: &Record) -> Applied {
    let Some(form) = forms::form(e, record.form) else {
        return Applied::Absent;
    };
    if e.mem.u8(form + addr::FORM_TYPE_OFFSET) != record.kind {
        return Applied::WrongType;
    }
    let Some(game) = game(e) else {
        return Applied::Busy;
    };
    if e.call(addr::GAME_LOADING_FLAG_TEST, &args![game]).bool() {
        return Applied::Busy;
    }
    let done = gate::applying(e, |e| {
        numbering.with(e, |e| {
            let buffer = e
                .call(addr::OPERATOR_NEW, &args![addr::LOAD_FORM_BUFFER_SIZE])
                .u32();
            if buffer == 0 {
                return Applied::NoMemory;
            }
            let len = record.data.len() as u32;
            let block = e.call(addr::OPERATOR_NEW, &args![len.max(1)]).u32();
            if block == 0 {
                e.call(addr::OPERATOR_DELETE, &args![buffer]);
                return Applied::NoMemory;
            }
            let buffer = e
                .call(addr::LOAD_FORM_BUFFER_CONSTRUCT, &args![buffer])
                .u32();
            e.mem.write(block, &record.data);
            e.mem.set_u32(buffer + addr::BUFFER_DATA_OFFSET, block);
            e.mem.set_u32(buffer + addr::BUFFER_SIZE_OFFSET, len);
            e.mem.set_u32(buffer + addr::BUFFER_POSITION_OFFSET, 0);
            e.mem
                .set_u32(buffer + addr::LOAD_FORM_BUFFER_FORM_ID_OFFSET, record.form);
            e.mem
                .write(buffer + addr::FORM_BUFFER_HEADER_OFFSET, &record.header);
            e.mem
                .set_u32(buffer + addr::LOAD_FORM_BUFFER_DATA_SIZE_OFFSET, len);
            let flags = record.flags();
            // As `LoadForm` (`008495d0`), from the load buffer it made of the stored record on.
            let thread_flag = e.call(addr::SET_THREAD_FLAG, &args![game, 0u32]).u32() & 0xff;
            e.call(addr::LOAD_FORM_BUFFER_SET_FORM, &args![buffer, form]);
            e.call(
                addr::LOAD_FORM_BUFFER_SET_HEADER_FLAGS,
                &args![buffer, flags],
            );
            e.call(addr::FORM_MARK_LOADED, &args![form, 1u32]);
            e.call(addr::LOAD_INITIAL_DATA, &args![buffer, form, flags]);
            e.vcall(form, addr::FORM_SLOT_LOAD_GAME, &args![buffer]);
            // `BGSLoadGameBuffer::DeleteBuffer`: the data is freed, the buffer goes on.
            e.call(addr::OPERATOR_DELETE, &args![block]);
            e.mem.set_u32(buffer + addr::BUFFER_DATA_OFFSET, 0);
            e.mem.set_u32(buffer + addr::BUFFER_SIZE_OFFSET, 0);
            e.mem.set_u32(buffer + addr::BUFFER_POSITION_OFFSET, 0);
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), buffer);
                e.call(
                    addr::ARRAY_ADD,
                    &args![game + addr::GAME_FORM_BUFFER_ARRAY_OFFSET, slot.addr()],
                );
            });
            e.call(addr::SET_THREAD_FLAG, &args![game, thread_flag]);
            // The game's own finish of forms loaded while playing: `InitLoadGame` and
            // `FinishLoadGame` for each buffer in its array, which it then deletes.
            e.call(addr::GAME_UNLOAD_PASS, &args![game, 0u32]);
            Applied::Done
        })
    });
    done.unwrap_or(Applied::Busy)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_record_goes_to_bytes_and_back() {
        let r = Record {
            form: 0x0010_5a10,
            kind: addr::FORM_TYPE_REFR,
            header: [0, 0, 3, 0x01, 0x10, 0x80, 0x00, 0, 0x1b],
            data: vec![1, 2, 3, b'|'],
        };
        assert_eq!(r.flags(), 0x0080_1001);
        assert_eq!((r.index(), r.save_type(), r.version()), (3, 0, 0x1b));
        assert_eq!(Record::from_bytes(r.form, &r.to_bytes()), Some(r.clone()));
        // Too short to hold a header.
        assert_eq!(Record::from_bytes(1, &r.to_bytes()[..9]), None);
        assert_eq!(Record::from_bytes(1, &[]), None);
    }

    #[test]
    fn only_the_changes_the_stream_is_meant_for_have_a_class() {
        use addr::*;
        let refr = FORM_TYPE_REFR;
        assert_eq!(class_of(refr, CHANGE_FORM_FLAGS), Some(Class::Reference));
        assert_eq!(class_of(refr, CHANGE_REFR_MOVE), Some(Class::Reference));
        assert_eq!(
            class_of(refr, CHANGE_OBJECT_OPEN_STATE | CHANGE_OBJECT_EXTRA_LOCK),
            Some(Class::Reference)
        );
        assert_eq!(
            class_of(refr, CHANGE_REFR_INVENTORY | CHANGE_REFR_MOVE),
            Some(Class::Container)
        );
        assert_eq!(class_of(refr, CHANGE_OBJECT_EMPTY), Some(Class::Container));
        // Pushed about by physics, or animated: not news.
        assert_eq!(class_of(refr, CHANGE_REFR_HAVOK_MOVE), None);
        assert_eq!(class_of(refr, 0x1000_0000), None);
        assert_eq!(class_of(FORM_TYPE_FACTION, 4), Some(Class::Faction));
        assert_eq!(class_of(FORM_TYPE_REPUTATION, 2), Some(Class::Reputation));
        assert_eq!(class_of(FORM_TYPE_NOTE, 0x8000_0000), Some(Class::Note));
        assert_eq!(class_of(FORM_TYPE_INFO, 0x8000_0000), Some(Class::Topic));
        // Actors, cells and quests have their own channels or none yet.
        for kind in [
            FORM_TYPE_ACHR,
            FORM_TYPE_ACRE,
            FORM_TYPE_CELL,
            FORM_TYPE_QUEST,
        ] {
            assert_eq!(class_of(kind, 0xffff_ffff), None);
        }
    }
}
