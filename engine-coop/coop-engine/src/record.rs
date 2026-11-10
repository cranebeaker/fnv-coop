//! One form's change record (SPEC-15): what the game's own save holds for one changed form.
//!
//! The host's game saves a changed form with the functions that its save file uses. A guest's
//! game attaches those bytes to its own change map, and the game's own load applies them: at
//! once for a form that is in memory, or with the form's cell later.
//!
//! The bytes name other forms by an index of the game's save tables. An index means the same
//! form on two machines only when both tables hold the same pair: the host sends its pairs,
//! and a guest takes a record only after the pairs that the record can name.

use crate::addr;
use crate::program::Program;

/// One form's change record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record {
    /// The form's id.
    pub form: u32,
    /// The change flags that the record holds.
    pub flags: u32,
    /// The form's type, as the game's save names it.
    pub kind: u8,
    /// The save version of the game that made the record.
    pub version: u8,
    /// The bytes that the game's own save wrote for the form.
    pub bytes: Vec<u8>,
    /// The host's count of index pairs when it made the record: the bytes name no index at or
    /// above it.
    pub pairs: u32,
}

/// The game's save state that each record call needs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Save {
    /// The save manager.
    manager: u32,
    /// Its map of changed forms.
    map: u32,
    /// Its table of form indexes.
    ids: u32,
    /// The save version.
    version: u8,
}

fn save(p: &mut dyn Program) -> Option<Save> {
    let manager = p.u32(addr::SAVE_MANAGER);
    if manager == 0 {
        return None;
    }
    let map = p.u32(manager + addr::SAVE_CHANGE_MAP);
    let ids = p.u32(manager + addr::SAVE_IDS);
    let source = p.u32(addr::SAVE_VERSION_SOURCE);
    if map == 0 || ids == 0 || source == 0 {
        return None;
    }
    Some(Save {
        manager,
        map,
        ids,
        version: p.call(addr::SAVE_VERSION, &[source]).eax as u8,
    })
}

/// How many index pairs the game's form table has.
pub fn pair_count(p: &mut dyn Program) -> u32 {
    save(p).map_or(0, |s| p.u32(s.ids + addr::IDS_COUNTER))
}

/// The index pairs of the game's form table from index `from`: (index, form id). The game
/// gives a form its index when a record first names it.
pub fn pairs(p: &mut dyn Program, from: u32) -> Vec<(u32, u32)> {
    let Some(s) = save(p) else { return Vec::new() };
    let count = p.u32(s.ids + addr::IDS_COUNTER);
    (from.max(1)..count)
        .map(|index| (index, p.call(addr::IDS_FORM_OF, &[s.ids, index]).eax))
        .collect()
}

/// The host's index pairs that a guest's game has.
///
/// A guest's own game makes indexes too. After the join they start at
/// [`addr::GUEST_INDEX_BASE`], so that they never meet an index of the host.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pairs {
    /// The host's next index that this game lacks: it has each pair below.
    next: u32,
}

impl Pairs {
    /// A guest joins: its own indexes start above the host's from now on. `None` when the
    /// game has no save manager yet.
    pub fn join(p: &mut dyn Program) -> Option<Pairs> {
        let s = save(p)?;
        let counter = p.u32(s.ids + addr::IDS_COUNTER);
        if counter < addr::GUEST_INDEX_BASE {
            p.set_u32(s.ids + addr::IDS_COUNTER, addr::GUEST_INDEX_BASE);
        }
        Some(Pairs { next: 1 })
    }

    /// The host's next index that this game lacks.
    pub fn next(&self) -> u32 {
        self.next
    }

    /// Gives the game's form table the host's pairs. Only pairs in index order from
    /// [`Pairs::next`] are taken: a gap stops the insert. The count of pairs taken.
    pub fn insert(&mut self, p: &mut dyn Program, pairs: &[(u32, u32)]) -> u32 {
        let Some(s) = save(p) else { return 0 };
        let mut taken = 0;
        for (index, form) in pairs {
            if *index < self.next {
                continue;
            }
            if *index != self.next || *index >= addr::GUEST_INDEX_BASE {
                break;
            }
            p.call(addr::IDS_INSERT, &[s.ids, *form, *index]);
            p.call(
                addr::IDS_INSERT,
                &[s.ids + addr::IDS_REVERSE, *index, *form],
            );
            self.next += 1;
            taken += 1;
        }
        taken
    }
}

/// Saves the form with this id as the game's own save does, with these change flags. `None`
/// when the form is not in memory, or the game has no save manager yet.
pub fn make(p: &mut dyn Program, form: u32, flags: u32) -> Option<Record> {
    let s = save(p)?;
    let at = p.call(addr::FORM_BY_ID, &[form]).eax;
    if at == 0 {
        return None;
    }
    let kind = p.call(addr::FORM_TYPE, &[at]).eax as u8;
    let buffer = p.alloc(addr::SAVE_BUFFER_BYTES);
    p.write(buffer, &[0; addr::SAVE_BUFFER_BYTES as usize]);
    p.call(addr::SAVE_BUFFER_NEW, &[buffer]);
    // The steps of the game's own "save one form" (addr.rs has the source).
    p.call(addr::SAVE_BUFFER_RESET, &[buffer]);
    p.call(
        addr::SAVE_BUFFER_HEADER,
        &[buffer, form, flags, u32::from(kind), u32::from(s.version)],
    );
    p.call(addr::SAVE_BUFFER_FORM, &[buffer, at]);
    let table = p.u32(at);
    let save_form = p.u32(table + addr::FORM_SAVE_SLOT);
    p.call(save_form, &[at, buffer]);
    p.call(addr::SAVE_EXTRA, &[buffer, at, flags]);
    let save_end = p.u32(table + addr::FORM_SAVE_END_SLOT);
    p.call(save_end, &[at, buffer]);
    let data = p.u32(buffer + addr::SAVE_BUFFER_DATA);
    let length = p
        .u32(buffer + addr::SAVE_BUFFER_LENGTH)
        .min(addr::RECORD_LIMIT);
    let mut bytes = vec![0; length as usize];
    if data != 0 {
        p.read(data, &mut bytes);
    }
    p.free(buffer);
    let pairs = p.u32(s.ids + addr::IDS_COUNTER);
    Some(Record {
        form,
        flags,
        kind,
        version: s.version,
        bytes,
        pairs,
    })
}

/// What a guest's game did with a record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Taken {
    /// The form is in memory: the game's own load applied the record.
    Loaded,
    /// The form is not in memory: the record waits in the change map, and the game loads it
    /// with the form's cell.
    Waits,
    /// The record was not taken: the game has no save manager, its save version is another,
    /// or its form table lacks a pair that the record can name.
    Refused,
}

/// Gives a guest's game a record of the host. `pairs`: the host's index pairs that this game
/// has. `load`: apply the record at once when the form is in memory (false: only attach it).
pub fn take(p: &mut dyn Program, record: &Record, pairs: &Pairs, load: bool) -> Taken {
    let Some(s) = save(p) else {
        return Taken::Refused;
    };
    if s.version != record.version || pairs.next() < record.pairs {
        return Taken::Refused;
    }
    let mut entry = p.call(addr::MAP_ENTRY, &[s.map, record.form]).eax;
    if entry == 0 {
        // The game's own mark makes the entry.
        p.call(addr::MAP_MARK, &[s.map, record.form, record.flags]);
        entry = p.call(addr::MAP_ENTRY, &[s.map, record.form]).eax;
        if entry == 0 {
            return Taken::Refused;
        }
    }
    // A guest that saw the form's cell and left has a waiting record of its own. The host's
    // record replaces it: the old block goes back to the game's heap.
    let old = p.u32(entry + addr::ENTRY_PENDING);
    if old != 0 {
        p.call(addr::HEAP_FREE, &[old]);
        p.set_u32(entry + addr::ENTRY_PENDING, 0);
    }
    // The game's load reads the bytes by the entry's flags, so they must be the record's.
    // The game's own mark does not do it: it changes no flag of an entry with a waiting
    // record (measured: the host's bytes were then read with the guest's old flags, and the
    // reference stayed enabled).
    p.set_u32(entry + addr::CHANGE_FLAGS, record.flags);
    let length = record.bytes.len() as u32;
    let bytes = p.alloc(length.max(1));
    p.write(bytes, &record.bytes);
    // The game copies the bytes into a block of its own.
    p.call(
        addr::ENTRY_ATTACH,
        &[
            entry + addr::ENTRY_PENDING,
            bytes,
            length,
            u32::from(record.kind),
            u32::from(record.version),
        ],
    );
    p.free(bytes);
    let at = p.call(addr::FORM_BY_ID, &[record.form]).eax;
    if at == 0 || !load {
        return Taken::Waits;
    }
    p.call(addr::SAVE_LOAD_FORM, &[s.manager, at]);
    p.call(addr::SAVE_LOAD_FINISH, &[s.manager, 0]);
    Taken::Loaded
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake::FakeProgram;

    const DOOR: u32 = 0x0010_4c13;

    fn game() -> FakeProgram {
        let mut p = FakeProgram::new();
        p.add_change_map(8);
        p.add_save_tables(0x0b);
        p
    }

    #[test]
    fn no_save_manager_means_no_record() {
        let mut p = FakeProgram::new();
        p.add_form(DOOR);
        assert_eq!(make(&mut p, DOOR, 1), None);
        assert_eq!(pair_count(&mut p), 0);
        assert!(pairs(&mut p, 1).is_empty());
        let record = Record {
            form: DOOR,
            flags: 1,
            kind: 0x3a,
            version: 0x0b,
            bytes: vec![1, 2, 3],
            pairs: 1,
        };
        assert_eq!(Pairs::join(&mut p), None);
        let pairs = Pairs { next: 1 };
        assert_eq!(take(&mut p, &record, &pairs, true), Taken::Refused);
    }

    /// The host's record crosses to a guest: the guest's game gets the flags and the bytes
    /// for the same form, and its own load runs for a form that is in memory. The calls run
    /// in the game's own order. Mutation twin: a form that is not in memory only waits.
    #[test]
    fn a_record_crosses_from_the_host_to_a_guest() {
        let mut host = game();
        let door = host.add_form(DOOR);
        host.set_form_bytes(door, &[0xaa, 0xbb, 0xcc, 0x01]);
        host.mark_changed(DOOR, 1);
        let record = make(&mut host, DOOR, 1).expect("a record");
        assert_eq!(
            (record.kind, record.version, &record.bytes[..]),
            (0x3a, 0x0b, &[0xaa, 0xbb, 0xcc, 0x01][..])
        );
        assert!(host.live.is_empty(), "the buffer's memory is given back");
        let order: Vec<u32> = host
            .calls
            .iter()
            .map(|(a, _)| *a)
            .filter(|a| {
                [
                    addr::SAVE_BUFFER_NEW,
                    addr::SAVE_BUFFER_RESET,
                    addr::SAVE_BUFFER_HEADER,
                    addr::SAVE_BUFFER_FORM,
                    addr::SAVE_EXTRA,
                ]
                .contains(a)
            })
            .collect();
        assert_eq!(
            order,
            [
                addr::SAVE_BUFFER_NEW,
                addr::SAVE_BUFFER_RESET,
                addr::SAVE_BUFFER_HEADER,
                addr::SAVE_BUFFER_FORM,
                addr::SAVE_EXTRA
            ]
        );

        let mut guest = game();
        let guest_door = guest.add_form(DOOR);
        let joined = Pairs::join(&mut guest).unwrap();
        assert_eq!(take(&mut guest, &record, &joined, true), Taken::Loaded);
        assert_eq!(guest.pending_bytes(DOOR), Some(record.bytes.clone()));
        assert_eq!(guest.loaded, [guest_door], "the game's own load ran");
        assert_eq!(crate::changes::walk(&guest)[0].flags, 1);
        assert!(guest.live.is_empty());

        // The twin: the guest's game does not have the form in memory.
        let mut far = game();
        assert_eq!(take(&mut far, &record, &joined, true), Taken::Waits);
        assert_eq!(far.pending_bytes(DOOR), Some(record.bytes.clone()));
        assert!(far.loaded.is_empty());
        // A caller can ask for the attach only.
        let mut near = game();
        near.add_form(DOOR);
        assert_eq!(take(&mut near, &record, &joined, false), Taken::Waits);
        assert!(near.loaded.is_empty());
    }

    /// A guest that saw the form's cell and left has a waiting record of its own, with its
    /// own flags. The take gives the old block back to the game's heap, and the entry gets
    /// the record's flags and bytes. Mutation twin: the game's own mark alone leaves the old
    /// flags (the fake does what the game does).
    #[test]
    fn a_take_replaces_the_guests_own_waiting_record() {
        let mut host = game();
        let door = host.add_form(DOOR);
        host.set_form_bytes(door, &[0x0b, 0x08, 0, 0]);
        let record = make(&mut host, DOOR, 0x8000_0011).unwrap();

        let mut guest = game();
        // The guest's own unload: flags without the "enabled" bit, and its own bytes.
        guest.mark_changed(DOOR, 0x8000_0010);
        let own = guest.unload_form(DOOR, &[0x7c, 0x00]);
        guest.mark_changed(DOOR, 1);
        assert_eq!(
            crate::changes::walk(&guest)[0].flags,
            0x8000_0010,
            "the twin: the game's mark changes no flag of an entry with a waiting record"
        );
        let joined = Pairs::join(&mut guest).unwrap();
        assert_eq!(take(&mut guest, &record, &joined, true), Taken::Waits);
        assert_eq!(crate::changes::walk(&guest)[0].flags, 0x8000_0011);
        assert_eq!(guest.pending_bytes(DOOR), Some(vec![0x0b, 0x08, 0, 0]));
        assert_eq!(guest.freed, [own], "the old block went back to the heap");
    }

    /// A record is taken only when the guest's form table has each pair that the record can
    /// name, and when the save version is the same. Mutation twin: with the pairs the same
    /// record is taken.
    #[test]
    fn a_record_needs_its_pairs_and_its_version() {
        let mut host = game();
        host.add_form(DOOR);
        host.add_pair(0x0010_4c0f);
        host.add_pair(0x0000_000f);
        let record = make(&mut host, DOOR, 1).unwrap();
        assert_eq!(record.pairs, 3, "the counter after two pairs");
        let sent = pairs(&mut host, 1);
        assert_eq!(sent, [(1, 0x0010_4c0f), (2, 0x0000_000f)]);

        let mut guest = game();
        guest.add_form(DOOR);
        let mut have = Pairs::join(&mut guest).unwrap();
        assert_eq!(
            pair_count(&mut guest),
            addr::GUEST_INDEX_BASE,
            "the guest's own indexes start above the host's"
        );
        assert_eq!(take(&mut guest, &record, &have, true), Taken::Refused);
        assert!(guest.loaded.is_empty());
        // A gap is not taken: the second pair alone names an index that the game cannot
        // resolve in order.
        assert_eq!(have.insert(&mut guest, &sent[1..]), 0);
        assert_eq!(have.insert(&mut guest, &sent), 2);
        assert_eq!(have.next(), 3);
        assert_eq!(
            have.insert(&mut guest, &sent),
            0,
            "a second insert changes nothing"
        );
        assert_eq!(guest.form_of_index(1), 0x0010_4c0f);
        assert_eq!(guest.form_of_index(2), 0x0000_000f);
        assert_eq!(take(&mut guest, &record, &have, true), Taken::Loaded);

        let other = Record {
            version: 0x0a,
            ..record
        };
        assert_eq!(take(&mut guest, &other, &have, true), Taken::Refused);
    }
}
