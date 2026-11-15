//! The game's own map of changed forms (SPEC-15).
//!
//! The game keeps, for its save file, one map of each form that changed since the game's
//! start, with flags for the kinds of change. A script command, a rule of the game and the
//! player's own action all mark their forms there. So the map names each change of the world
//! with no list of forms or commands.
//!
//! This module reads the map between frames. A flag stays set after its first change: the map
//! says which forms to look at, and a state read is the truth for a second change.

use std::collections::BTreeMap;

use crate::addr;
use crate::program::Program;

/// One form of the map.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Entry {
    /// The form's id.
    pub form: u32,
    /// The change flags: one bit for each kind of change. The meaning of a bit depends on
    /// the form's type.
    pub flags: u32,
}

/// The longest chain of one bucket that a walk follows, and the most buckets. A map that is
/// larger is not the game's map: the walk stops.
const CHAIN_LIMIT: u32 = 1 << 16;
const BUCKET_LIMIT: u32 = 1 << 20;

/// Each form of the game's change map, sorted by form id. Empty before the game made its save
/// manager.
pub fn walk(p: &dyn Program) -> Vec<Entry> {
    let manager = p.u32(addr::SAVE_MANAGER);
    if manager == 0 {
        return Vec::new();
    }
    let map = p.u32(manager + addr::SAVE_CHANGE_MAP);
    if map == 0 {
        return Vec::new();
    }
    let buckets = p.u32(map + addr::MAP_BUCKET_COUNT).min(BUCKET_LIMIT);
    let array = p.u32(map + addr::MAP_BUCKETS);
    let mut out = Vec::new();
    if array == 0 {
        return out;
    }
    for bucket in 0..buckets {
        let mut item = p.u32(array + 4 * bucket);
        let mut steps = 0;
        while item != 0 && steps < CHAIN_LIMIT {
            let entry = p.u32(item + addr::MAP_ITEM_ENTRY);
            out.push(Entry {
                form: p.u32(item + addr::MAP_ITEM_FORM),
                flags: match entry {
                    0 => 0,
                    entry => p.u32(entry + addr::CHANGE_FLAGS),
                },
            });
            item = p.u32(item + addr::MAP_ITEM_NEXT);
            steps += 1;
        }
    }
    out.sort_by_key(|e| e.form);
    out
}

/// A form whose entry or flag bits are new since the last look.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Dirty {
    /// The form's id.
    pub form: u32,
    /// The flag bits that were not set at the last look.
    pub new: u32,
    /// All flag bits now.
    pub flags: u32,
}

/// What the adapter saw in the map at its last look.
#[derive(Clone, Debug, Default)]
pub struct Changes {
    seen: BTreeMap<u32, u32>,
}

impl Changes {
    /// The map as it is now is the baseline: nothing in it is a change.
    pub fn baseline(p: &dyn Program) -> Changes {
        Changes {
            seen: walk(p).into_iter().map(|e| (e.form, e.flags)).collect(),
        }
    }

    /// How many forms the adapter knows.
    pub fn len(&self) -> usize {
        self.seen.len()
    }

    /// True when the adapter knows no form.
    pub fn is_empty(&self) -> bool {
        self.seen.is_empty()
    }

    /// Looks at the map: each form that is new or has a new flag bit, sorted by form id. The
    /// look is the new baseline. A form whose entry has no flag at all is not dirty.
    pub fn look(&mut self, p: &dyn Program) -> Vec<Dirty> {
        let mut dirty = Vec::new();
        for entry in walk(p) {
            let before = self.seen.insert(entry.form, entry.flags).unwrap_or(0);
            let new = entry.flags & !before;
            if new != 0 {
                dirty.push(Dirty {
                    form: entry.form,
                    new,
                    flags: entry.flags,
                });
            }
        }
        dirty
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake::FakeProgram;

    const DOOR: u32 = 0x0010_4c13;
    const CHEST: u32 = 0x0016_8c28;

    #[test]
    fn no_save_manager_means_no_entry() {
        let p = FakeProgram::new();
        assert_eq!(walk(&p), []);
        assert!(Changes::baseline(&p).is_empty());
    }

    /// The walk reads each bucket's chain. Forms that share a bucket are all found.
    #[test]
    fn the_walk_reads_each_form_and_its_flags() {
        let mut p = FakeProgram::new();
        p.add_change_map(4);
        p.mark_changed(DOOR, 1);
        p.mark_changed(CHEST, 0x20);
        // The same bucket as the door (the id is 4 more).
        p.mark_changed(DOOR + 4, 0x8000_0000);
        p.mark_changed(DOOR, 2);
        assert_eq!(
            walk(&p),
            [
                Entry {
                    form: DOOR,
                    flags: 3
                },
                Entry {
                    form: DOOR + 4,
                    flags: 0x8000_0000
                },
                Entry {
                    form: CHEST,
                    flags: 0x20
                },
            ]
        );
    }

    /// What the game changed before the baseline is no change. A new form and a new flag bit
    /// are each dirty one time. A second change of the same kind gives no new bit: the map
    /// alone does not show it. Mutation twin: a baseline that is taken after the change sees
    /// nothing.
    #[test]
    fn a_look_gives_each_new_form_and_bit_one_time() {
        let mut p = FakeProgram::new();
        p.add_change_map(8);
        p.mark_changed(CHEST, 0x20);
        let mut changes = Changes::baseline(&p);
        assert_eq!(changes.len(), 1);
        assert_eq!(changes.look(&p), []);

        p.mark_changed(DOOR, 1);
        p.mark_changed(CHEST, 0x40);
        let twin = Changes::baseline(&p).look(&p);
        assert_eq!(twin, [], "the twin: the baseline after the change");
        assert_eq!(
            changes.look(&p),
            [
                Dirty {
                    form: DOOR,
                    new: 1,
                    flags: 1
                },
                Dirty {
                    form: CHEST,
                    new: 0x40,
                    flags: 0x60
                },
            ]
        );
        assert_eq!(changes.look(&p), []);
        // The door is disabled, enabled and disabled again: the same bit.
        p.mark_changed(DOOR, 1);
        assert_eq!(changes.look(&p), [], "a state read must find this");
    }

    /// A chain that does not end (not the game's map) does not hang the walk.
    #[test]
    fn a_chain_with_a_loop_ends() {
        let mut p = FakeProgram::new();
        p.add_change_map(1);
        p.mark_changed(DOOR, 1);
        p.loop_change_chain(DOOR);
        assert_eq!(walk(&p).len() as u32, CHAIN_LIMIT);
    }
}
