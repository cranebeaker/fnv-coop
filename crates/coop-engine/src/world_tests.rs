//! The changed-form stream's round trips, one per kind of form (docs/coop/DECISIONS.md D1,
//! condition 1): the host's record, loaded by a guest, read back from the guest byte for byte, and
//! decoded by `crates/fos` where the save decoder knows the type. Two [`Story`]s are two machines;
//! the game functions are the world kit's stand-ins, not the translations.

use fos::decode::{self, InitialData};
use fos::{ChangeForm, RefId};

use crate::changes::{self, Applied, Class, Numbering, Record};
use crate::testkit::{Story, StorySpec};
use crate::worldkit::{self as kit, RefSpec, RefState, WorldSpec};
use crate::{addr, gate, world};

const CELL: u32 = 0x0010_5a00;
const DOOR: u32 = 0x0010_5a10;
const CHEST: u32 = 0x0010_5a11;
const CRATE: u32 = 0x0010_5a12;
const LAMP: u32 = 0x0010_5a13;
const POWDER_GANGERS: u32 = 0x0001_b2a4;
const NCR: u32 = 0x000e_e68b;
const GOODSPRINGS: u32 = 0x0011_6f16;
const REP: u32 = 0x0010_4c22;
const NOTE: u32 = 0x0010_2a01;
const INFO: u32 = 0x0010_2b01;
/// Items of a second plugin: their ids are not their indices in any numbering.
const CAPS: u32 = 0x0000_000f;
const STIMPAK: u32 = 0x0100_15e9;
const DYNAMITE: u32 = 0x0200_109a;

fn machine() -> Story {
    let mut s = Story::new(&StorySpec {
        cells: vec![CELL],
        ..StorySpec::default()
    });
    kit::install(
        &mut s,
        &WorldSpec {
            refs: vec![
                RefSpec {
                    id: DOOR,
                    cell: CELL,
                    opens: true,
                    lock: Some(50),
                    items: vec![],
                },
                RefSpec {
                    id: CHEST,
                    cell: CELL,
                    opens: true,
                    lock: None,
                    items: vec![(CAPS, 20), (STIMPAK, 2)],
                },
                RefSpec {
                    id: CRATE,
                    cell: CELL,
                    ..RefSpec::default()
                },
                RefSpec {
                    id: LAMP,
                    cell: CELL,
                    ..RefSpec::default()
                },
            ],
            factions: vec![POWDER_GANGERS, NCR, GOODSPRINGS],
            reputations: vec![REP],
            notes: vec![NOTE],
            topics: vec![INFO],
        },
    );
    s
}

/// A host and a guest (its story scripts gated), each with a numbering of its own.
fn two() -> (Story, Numbering, Story, Numbering) {
    let mut guest = machine();
    gate::set_guest(&mut guest.e, true);
    (machine(), Numbering::new(), guest, Numbering::new())
}

/// The record as `crates/fos` reads one from a save.
fn change_form(r: &Record) -> ChangeForm<'_> {
    ChangeForm {
        ref_id: RefId(r.index()),
        flags: r.flags(),
        save_type: r.save_type(),
        version: r.version(),
        offset: 0,
        data_offset: 0,
        data: &r.data,
    }
}

/// The decision's round trip for form `id`: the host's record (with every change flag its save
/// holds), over the wire, loaded by the guest, and the guest's own record of the form with the
/// same flags: byte for byte the host's.
fn round_trip(
    host: &mut Story,
    hn: &mut Numbering,
    guest: &mut Story,
    gn: &mut Numbering,
    id: u32,
) -> Record {
    let flags = changes::saved_flags(&mut host.e, id);
    let record = changes::capture(&mut host.e, hn, id, flags).expect("the host's record");
    hn.grown(&mut host.e);
    assert!(gn.follow(&mut guest.e, hn.ids()));
    let wire = Record::from_bytes(id, &record.to_bytes()).expect("a whole record");
    assert_eq!(wire, record);
    assert_eq!(changes::apply(&mut guest.e, gn, &wire), Applied::Done);
    let back = changes::capture(&mut guest.e, gn, id, wire.flags()).expect("the guest's record");
    assert!(!gn.strayed(&mut guest.e));
    assert_eq!(
        back, record,
        "the guest's record is the host's, byte for byte"
    );
    // Neither game's own save numbering was touched, and each has its own map back.
    for s in [&*host, &*guest] {
        assert_eq!(kit::own_numbering(s), 1);
        assert!(kit::own_map_in_place(s));
    }
    record
}

#[test]
fn a_reference_disabled_and_moved_round_trips_and_fos_reads_its_place() {
    let (mut host, mut hn, mut guest, mut gn) = two();
    kit::disable(&mut host, CRATE, true);
    kit::move_to(&mut host, CRATE, [12.5, -40.0, 7.25]);
    let r = round_trip(&mut host, &mut hn, &mut guest, &mut gn, CRATE);
    assert_eq!(r.flags(), addr::CHANGE_FORM_FLAGS | addr::CHANGE_REFR_MOVE);
    assert_eq!(r.kind, addr::FORM_TYPE_REFR);
    assert_eq!(changes::class_of(r.kind, r.flags()), Some(Class::Reference));
    assert_eq!(
        kit::ref_state(&mut guest, CRATE),
        kit::ref_state(&mut host, CRATE)
    );
    let state = kit::ref_state(&mut guest, CRATE).unwrap();
    assert!(state.disabled);
    assert_eq!(state.pos, [12.5, -40.0, 7.25]);
    // The save decoder reads the record's start as the game's: where the reference stands, its
    // cell named through the numbering.
    let cf = change_form(&r);
    assert_eq!(cf.type_name(), Some("REFR"));
    assert_eq!(cf.ref_id.form_id(hn.ids()), Some(CRATE));
    let start = decode::reference_start(&cf).expect("fos reads the record's start");
    let InitialData::Location {
        space, position, ..
    } = start.initial
    else {
        panic!("a moved reference has its place, not {:?}", start.initial);
    };
    assert_eq!(space.form_id(hn.ids()), Some(CELL));
    assert_eq!(position, [12.5, -40.0, 7.25]);
    assert!(start.havok.is_none());
    // Enabled again on the host: the next record enables it on the guest.
    kit::disable(&mut host, CRATE, false);
    round_trip(&mut host, &mut hn, &mut guest, &mut gn, CRATE);
    assert!(!kit::ref_state(&mut guest, CRATE).unwrap().disabled);
}

#[test]
fn a_door_opened_unlocked_and_forced_round_trips() {
    let (mut host, mut hn, mut guest, mut gn) = two();
    kit::set_locked_as_played(&mut host, DOOR, false);
    kit::fail_lock(&mut host, DOOR);
    kit::open(&mut host, DOOR, true);
    let r = round_trip(&mut host, &mut hn, &mut guest, &mut gn, DOOR);
    assert_eq!(
        r.flags(),
        addr::CHANGE_OBJECT_EXTRA_LOCK | addr::CHANGE_OBJECT_OPEN_STATE
    );
    let want = RefState {
        disabled: false,
        pos: [0.0; 3],
        open: true,
        lock: Some((50, 0, 1)),
        items: vec![],
    };
    assert_eq!(kit::ref_state(&mut guest, DOOR), Some(want.clone()));
    assert_eq!(kit::ref_state(&mut host, DOOR), Some(want));
    // No place: nothing moved, so the record starts with the object's own data.
    let start = decode::reference_start(&change_form(&r)).unwrap();
    assert_eq!(start.initial, InitialData::None);
    // Shut and locked again on the host. The save no longer holds the open state's flag, and the
    // record written with what it does hold shuts the guest's door.
    kit::open(&mut host, DOOR, false);
    kit::set_locked_as_played(&mut host, DOOR, true);
    let r = round_trip(&mut host, &mut hn, &mut guest, &mut gn, DOOR);
    assert_eq!(r.flags(), addr::CHANGE_OBJECT_EXTRA_LOCK);
    let state = kit::ref_state(&mut guest, DOOR).unwrap();
    assert!(!state.open);
    assert_eq!(state.lock, Some((50, addr::LOCK_FLAG_LOCKED, 1)));
}

#[test]
fn a_container_round_trips_with_its_items_named_through_the_numbering() {
    let (mut host, mut hn, mut guest, mut gn) = two();
    // The guest's numbering starts out of step with the host's on purpose: it follows the host's
    // list, whatever this game numbered before.
    kit::set_item(&mut host, CHEST, STIMPAK, 0);
    kit::set_item(&mut host, CHEST, DYNAMITE, 3);
    let r = round_trip(&mut host, &mut hn, &mut guest, &mut gn, CHEST);
    assert_eq!(r.flags(), addr::CHANGE_REFR_INVENTORY);
    assert_eq!(changes::class_of(r.kind, r.flags()), Some(Class::Container));
    assert_eq!(
        kit::ref_state(&mut guest, CHEST).unwrap().items,
        vec![(CAPS, 20), (DYNAMITE, 3)]
    );
    // The host's list: the chest itself, then the items in the order the record names them.
    assert_eq!(hn.ids(), [CHEST, CAPS, DYNAMITE]);
    assert_eq!(gn.ids(), hn.ids());
    // Emptied: the record says so, and the guest's chest is empty.
    kit::set_item(&mut host, CHEST, CAPS, 0);
    kit::set_item(&mut host, CHEST, DYNAMITE, 0);
    round_trip(&mut host, &mut hn, &mut guest, &mut gn, CHEST);
    assert!(kit::ref_state(&mut guest, CHEST).unwrap().items.is_empty());
}

/// The twin of the container's round trip: loaded with a numbering that is not the host's, the
/// same record puts other items into the guest's chest. The numbering is what makes a record
/// mean the same on two machines.
#[test]
fn twin_a_record_loaded_with_another_numbering_names_other_forms() {
    let (mut host, mut hn, mut guest, _) = two();
    kit::set_item(&mut host, CHEST, DYNAMITE, 3);
    let flags = changes::saved_flags(&mut host.e, CHEST);
    let record = changes::capture(&mut host.e, &mut hn, CHEST, flags).unwrap();
    hn.grown(&mut host.e);
    let mut other = Numbering::new();
    other.follow(&mut guest.e, &[LAMP, DOOR, CRATE, CHEST]);
    assert_eq!(
        changes::apply(&mut guest.e, &mut other, &record),
        Applied::Done
    );
    assert_ne!(
        kit::ref_state(&mut guest, CHEST).unwrap().items,
        kit::ref_state(&mut host, CHEST).unwrap().items
    );
}

#[test]
fn a_faction_round_trips_and_fos_reads_it() {
    let (mut host, mut hn, mut guest, mut gn) = two();
    let state = (0x41, [3, 1], vec![(NCR, -100, 1), (GOODSPRINGS, 25, 2)]);
    kit::set_faction(&mut host, POWDER_GANGERS, &state, 2 | 4 | 0x8000_0000);
    let r = round_trip(&mut host, &mut hn, &mut guest, &mut gn, POWDER_GANGERS);
    assert_eq!(changes::class_of(r.kind, r.flags()), Some(Class::Faction));
    assert_eq!(kit::faction(&mut guest, POWDER_GANGERS), state);
    let cf = change_form(&r);
    assert_eq!(cf.type_name(), Some("FACT"));
    let f = decode::faction(&cf).expect("fos reads the faction");
    assert_eq!(f.flags, Some(0x41));
    assert_eq!(f.crimes, Some((3, 1)));
    let reactions: Vec<_> = f
        .reactions
        .unwrap()
        .into_iter()
        .map(|(other, modifier, group)| (other.form_id(hn.ids()).unwrap(), modifier, group))
        .collect();
    assert_eq!(reactions, state.2);
    // Only the crime counts changed in another faction: a record without the other parts.
    kit::set_faction(&mut host, NCR, &(0, [0, 2], vec![]), 0x8000_0000);
    let r = round_trip(&mut host, &mut hn, &mut guest, &mut gn, NCR);
    let f = decode::faction(&change_form(&r)).unwrap();
    assert_eq!((f.flags, f.reactions, f.crimes), (None, None, Some((0, 2))));
    assert_eq!(kit::faction(&mut guest, NCR).1, [0, 2]);
}

#[test]
fn a_reputation_round_trips_and_fos_reads_it() {
    let (mut host, mut hn, mut guest, mut gn) = two();
    kit::set_reputation(&mut host, REP, 12.0, 3.5);
    let r = round_trip(&mut host, &mut hn, &mut guest, &mut gn, REP);
    assert_eq!(
        changes::class_of(r.kind, r.flags()),
        Some(Class::Reputation)
    );
    assert_eq!(kit::reputation(&mut guest, REP), (12.0, 3.5));
    let cf = change_form(&r);
    assert_eq!(cf.type_name(), Some("REPU"));
    assert_eq!(
        decode::pair(&cf).expect("fos reads the reputation"),
        (12.0f32.to_bits(), 3.5f32.to_bits())
    );
}

#[test]
fn a_note_read_and_a_response_said_round_trip_and_fos_reads_them() {
    let (mut host, mut hn, mut guest, mut gn) = two();
    for (id, name, class) in [(NOTE, "NOTE", Class::Note), (INFO, "INFO", Class::Topic)] {
        assert!(!kit::marked(&mut guest, id));
        kit::mark(&mut host, id);
        let r = round_trip(&mut host, &mut hn, &mut guest, &mut gn, id);
        assert_eq!(changes::class_of(r.kind, r.flags()), Some(class));
        assert!(kit::marked(&mut guest, id));
        let cf = change_form(&r);
        assert_eq!(cf.type_name(), Some(name));
        // The flag is the whole record.
        assert_eq!(decode::flags_only(&cf).expect("fos reads it"), None);
        assert!(r.data.is_empty());
        assert_eq!(r.flags(), 0x8000_0000);
    }
}

#[test]
fn a_record_is_loaded_as_the_game_loads_a_form_and_no_script_runs_on_the_guest() {
    let (mut host, mut hn, mut guest, mut gn) = two();
    kit::disable(&mut host, LAMP, true);
    round_trip(&mut host, &mut hn, &mut guest, &mut gn, LAMP);
    // One `LoadGame`, then the game's own unload pass with its `InitLoadGame` and
    // `FinishLoadGame`.
    assert_eq!(kit::loads(&guest), 1);
    assert_eq!(kit::passes(&guest), (1, 1, 1));
    // The object's script the load would start was held back on the guest, and counted.
    assert_eq!(guest.scripts_run(), 0);
    assert_eq!(gate::skipped(&mut guest.e).0, 1);
    // The twin: the same record loaded by a game that is not a guest runs that script, so the
    // apply scope is what held it back.
    let record = {
        let flags = changes::saved_flags(&mut host.e, LAMP);
        changes::capture(&mut host.e, &mut hn, LAMP, flags).unwrap()
    };
    let mut plain = machine();
    let mut pn = Numbering::new();
    pn.follow(&mut plain.e, hn.ids());
    assert_eq!(
        changes::apply(&mut plain.e, &mut pn, &record),
        Applied::Done
    );
    assert_eq!(plain.scripts_run(), 1);
}

#[test]
fn a_record_waits_or_is_refused_and_then_nothing_is_loaded() {
    let (mut host, mut hn, mut guest, mut gn) = two();
    kit::disable(&mut host, CRATE, true);
    let flags = changes::saved_flags(&mut host.e, CRATE);
    let record = changes::capture(&mut host.e, &mut hn, CRATE, flags).unwrap();
    hn.grown(&mut host.e);
    gn.follow(&mut guest.e, hn.ids());
    // Its cell is not loaded on the guest.
    guest.set_present(CRATE, false);
    assert_eq!(
        changes::apply(&mut guest.e, &mut gn, &record),
        Applied::Absent
    );
    guest.set_present(CRATE, true);
    // The guest's game is loading a save.
    kit::set_loading(&mut guest, true);
    assert_eq!(
        changes::apply(&mut guest.e, &mut gn, &record),
        Applied::Busy
    );
    kit::set_loading(&mut guest, false);
    // The guest's form of that id is of another type: other data.
    let odd = Record {
        kind: addr::FORM_TYPE_FACTION,
        ..record.clone()
    };
    assert_eq!(
        changes::apply(&mut guest.e, &mut gn, &odd),
        Applied::WrongType
    );
    assert_eq!(kit::loads(&guest), 0);
    assert_eq!(kit::passes(&guest), (0, 0, 0));
    assert!(!kit::ref_state(&mut guest, CRATE).unwrap().disabled);
    // Nothing to capture: no flags, a form that is not here.
    assert_eq!(changes::capture(&mut host.e, &mut hn, CRATE, 0), None);
    assert_eq!(changes::capture(&mut host.e, &mut hn, 0x9999, 1), None);
    assert_eq!(changes::saved_flags(&mut host.e, LAMP), 0);
    // And then it loads.
    assert_eq!(
        changes::apply(&mut guest.e, &mut gn, &record),
        Applied::Done
    );
    assert!(kit::ref_state(&mut guest, CRATE).unwrap().disabled);
}

#[test]
fn a_guest_numbering_follows_the_host_list_and_is_put_back_when_it_strays() {
    let (mut host, mut hn, mut guest, mut gn) = two();
    kit::set_item(&mut host, CHEST, DYNAMITE, 1);
    round_trip(&mut host, &mut hn, &mut guest, &mut gn, CHEST);
    // The guest's own chest holds something the host's records never named. Its record gives out
    // an index the host's list does not have: the guest notices, and its map is the host's list
    // again, so the host's next id gets the index the host gave it.
    kit::set_item(&mut guest, CHEST, 0x0300_0042, 1);
    let flags = addr::CHANGE_REFR_INVENTORY;
    let own = changes::capture(&mut guest.e, &mut gn, CHEST, flags).unwrap();
    assert!(gn.strayed(&mut guest.e));
    assert!(!gn.strayed(&mut guest.e));
    kit::set_item(&mut host, CHEST, STIMPAK, 0);
    kit::set_item(&mut host, CHEST, 0x0300_0077, 5);
    let r = round_trip(&mut host, &mut hn, &mut guest, &mut gn, CHEST);
    assert_ne!(own, r);
    assert_eq!(
        kit::ref_state(&mut guest, CHEST).unwrap().items,
        kit::ref_state(&mut host, CHEST).unwrap().items
    );
    // A host that started a new session numbers from the start: the guest takes the new list
    // whole and says its old records are to be loaded again.
    assert!(!gn.follow(&mut guest.e, &[DOOR, CELL]));
    assert_eq!(gn.ids(), [DOOR, CELL]);
    assert!(gn.follow(&mut guest.e, &[DOOR, CELL, LAMP]));
}

#[test]
fn doors_and_locks_are_read_and_set_through_the_game() {
    let mut s = machine();
    crate::arm(&mut s.e, true);
    assert_eq!(world::open(&mut s.e, DOOR), Some(false));
    assert!(world::set_open(&mut s.e, DOOR, true));
    assert_eq!(world::open(&mut s.e, DOOR), Some(true));
    // Opening is in the journal; shutting again is not (the game removes the flag).
    let c = crate::drain(&mut s.e);
    assert_eq!(
        (c.len(), c[0].form, c[0].flags),
        (1, DOOR, addr::CHANGE_OBJECT_OPEN_STATE)
    );
    assert!(world::set_open(&mut s.e, DOOR, false));
    assert!(crate::drain(&mut s.e).is_empty());
    assert_eq!(changes::saved_flags(&mut s.e, DOOR), 0);
    // Something that does not open, something that is not here, an actor's id.
    assert_eq!(world::open(&mut s.e, LAMP), None);
    assert!(!world::set_open(&mut s.e, LAMP, true));
    assert_eq!(world::open(&mut s.e, 0x9999), None);
    assert!(!world::shareable(0xff00_0001));
    // The lock: unlocked through the game's `UnLock`, then level and tries in place.
    let l = world::lock(&mut s.e, DOOR).unwrap();
    assert_eq!((l.level, l.locked(), l.tries), (50, true, 0));
    let to = world::Lock {
        level: 75,
        flags: 0,
        tries: 2,
    };
    assert!(world::set_lock(&mut s.e, DOOR, to));
    assert_eq!(world::lock(&mut s.e, DOOR), Some(to));
    let c = crate::drain(&mut s.e);
    assert_eq!(c[0].flags, addr::CHANGE_OBJECT_EXTRA_LOCK);
    // Already so: nothing is called, nothing changes.
    assert!(world::set_lock(&mut s.e, DOOR, to));
    assert!(crate::drain(&mut s.e).is_empty());
    // No lock: never made here.
    assert_eq!(world::lock(&mut s.e, CHEST), None);
    assert!(!world::set_lock(&mut s.e, CHEST, to));
}
