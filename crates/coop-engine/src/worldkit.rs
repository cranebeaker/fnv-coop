//! Placed objects, factions, reputations, notes and dialogue responses for a [`Story`], with
//! stand-ins for the game's save and load of one form, for tests of the changed-form stream and
//! of doors and locks without the game (feature `testkit`; docs/coop/SPECS.md S12).
//!
//! Each stand-in does what co-op relies on the real one to do. The save side writes the game's
//! own layout where `crates/fos` documents it: every value followed by `|`, references to other
//! forms as three-byte indices into the form id map the game object points at, a reference's
//! initial data (cell, position, rotation) when it moved, a faction's reactions, flags and crime
//! counts, a reputation's two values, and nothing but the form flags for a note or a response.
//! What a placed object writes after its initial data (its lock, its contents) is this kit's own
//! layout, not the game's: co-op carries a record's data without reading it, so the tests need a
//! record that is whole, not one that is the game's byte for byte.
//!
//! The stand-ins that change state flag the change through `BGSSaveLoadGame::AddChange`, so the
//! journal sees what it would see in the game; a door shut again loses its flag without it, as in
//! the game (`RemoveChange`).

use engine_core::prelude::*;

use crate::testkit::{self, Story};
use crate::{addr, forms};

/// Where the kit lives.
const WK: u32 = 0x0400_0000;
const WK_LEN: u32 = 0x0040_0000;
/// The game's `BGSSaveLoadGame` and `BGSSaveLoadManager`.
const GAME: u32 = WK;
const MANAGER: u32 = WK + 0x400;
/// The game's own form id map (a save numbers its records with it; co-op must leave it alone).
const OWN_MAP: u32 = WK + 0x800;
/// The changes map: a count, then (form id, flags) pairs.
const FLAGS: u32 = WK + 0x1000;
const FLAGS_MAX: u32 = 0x1000;
/// The form buffer array's contents: a count, then load buffers.
const PENDING: u32 = WK + 0xa000;
/// Counters: unload passes, forms loaded, `InitLoadGame` calls, `FinishLoadGame` calls, the next
/// heap address, forms made, save buffers made, the "a load is running" answer.
const PASSES: u32 = WK + 0xb000;
const LOADS: u32 = WK + 0xb004;
const INITS: u32 = WK + 0xb008;
const FINISHES: u32 = WK + 0xb00c;
const NEXT: u32 = WK + 0xb010;
const MADE: u32 = WK + 0xb014;
const SAVES: u32 = WK + 0xb018;
const LOADING: u32 = WK + 0xb01c;
/// One vtable per kind of form, 0x100 bytes each.
const VTABLES: u32 = WK + 0xc000;
/// The forms, 0x200 bytes each.
const FORMS: u32 = WK + 0x1_0000;
const FORM_LEN: u32 = 0x200;
const FORMS_MAX: u32 = 0x400;
/// The heap `operator new` hands out from.
const HEAP: u32 = WK + 0x10_0000;
/// The stand-ins behind the vtables' slots.
const FNS: u32 = 0x0131_0000;
const REF_SAVE: u32 = FNS;
const REF_LOAD: u32 = FNS + 0x10;
const FACTION_SAVE: u32 = FNS + 0x20;
const FACTION_LOAD: u32 = FNS + 0x30;
const REPUTATION_SAVE: u32 = FNS + 0x40;
const REPUTATION_LOAD: u32 = FNS + 0x50;
const FLAG_SAVE: u32 = FNS + 0x60;
const FLAG_LOAD: u32 = FNS + 0x70;
const CHECK: u32 = FNS + 0x80;
const INIT: u32 = FNS + 0x90;
const FINISH: u32 = FNS + 0xa0;
/// `TESForm::InitLoadGame` and `FinishLoadGame`'s slots (`bgssaveloadgame.rs`).
const SLOT_INIT: u32 = 0x64;
const SLOT_FINISH: u32 = 0x84;
/// The minor version the manager answers (the game's 0x1b, `crates/fos`).
const MINOR: u32 = 0x1b;
/// A save buffer's capacity.
const SAVE_CAP: u32 = 0x1000;
/// Most ids a form id map holds here.
const MAP_CAP: u32 = 0x1000;

// A placed object's fields beyond the game's.
const OPENS: u32 = 0x100;
const OPEN: u32 = 0x101;
const DEFAULT_OPEN: u32 = 0x102;
const HAS_LOCK: u32 = 0x103;
const ITEM_COUNT: u32 = 0x110;
const ITEMS: u32 = 0x114;
const ITEMS_MAX: u32 = 16;
const LOCK: u32 = 0x1a0;
/// Where a placed object's own script is (all zero: not a quest's).
const OBJECT_SCRIPT: u32 = 0x1b4;
/// A reference's rotation (three floats before its position, `OBJ_REFR`).
const REF_ROT_OFFSET: u32 = 0x24;
// A faction's: flags, two crime counts, a reaction count, then (faction, modifier, group).
const FACTION_FLAGS: u32 = 0x100;
const FACTION_CRIMES: u32 = 0x104;
const REACTION_COUNT: u32 = 0x10c;
const REACTIONS: u32 = 0x110;
// A reputation's fame and infamy; a note's or response's "read" or "said once" byte.
const VALUES: u32 = 0x100;
const MARKED: u32 = 0x100;

/// One placed object.
#[derive(Debug, Clone, Default)]
pub struct RefSpec {
    /// Its reference id.
    pub id: u32,
    /// The form id of its cell (a cell of the story), or 0.
    pub cell: u32,
    /// Whether it opens (a door, a container with a lid).
    pub opens: bool,
    /// Its lock's level when it has a lock; it starts locked.
    pub lock: Option<u8>,
    /// Its contents: (item form id, count).
    pub items: Vec<(u32, i32)>,
}

/// What the kit adds to a story.
#[derive(Debug, Clone, Default)]
pub struct WorldSpec {
    /// Placed objects.
    pub refs: Vec<RefSpec>,
    /// Factions' form ids.
    pub factions: Vec<u32>,
    /// Reputations' form ids.
    pub reputations: Vec<u32>,
    /// Notes' form ids.
    pub notes: Vec<u32>,
    /// Dialogue responses' form ids.
    pub topics: Vec<u32>,
}

/// A placed object as a test reads it.
#[derive(Debug, Clone, PartialEq)]
pub struct RefState {
    /// The form flag "disabled".
    pub disabled: bool,
    /// Its position.
    pub pos: [f32; 3],
    /// Whether it is open.
    pub open: bool,
    /// Its lock: (level, flags, tries).
    pub lock: Option<(u8, u8, u32)>,
    /// Its contents.
    pub items: Vec<(u32, i32)>,
}

/// A faction as a test reads it: flags, crime counts, reactions.
pub type FactionState = (u32, [i32; 2], Vec<(u32, i32, i32)>);

fn eax(v: u32) -> Ret {
    Ret {
        eax: v,
        ..Ret::default()
    }
}

fn bump(e: &mut Engine, at: u32) {
    let n = e.mem.u32(at);
    e.mem.set_u32(at, n + 1);
}

fn new(e: &mut Engine, size: u32) -> u32 {
    let at = e.mem.u32(NEXT);
    let next = at + ((size + 0xf) & !0xf);
    assert!(next <= WK + WK_LEN, "the kit's heap is used up");
    e.mem.set_u32(NEXT, next);
    at
}

// ---- The form id map --------------------------------------------------------------------------

fn map_get_or_add(e: &mut Engine, a: &[u32]) -> Ret {
    let (map, id) = (a[0], a[1]);
    if id >> 24 == 0xff {
        return eax(id);
    }
    let ids = e.mem.u32(map);
    let n = e.mem.u32(map + addr::FORM_ID_MAP_NEXT_INDEX_OFFSET);
    for i in 0..n {
        if e.mem.u32(ids + 4 * i) == id {
            return eax(i);
        }
    }
    assert!(
        n < MAP_CAP,
        "a test form id map holds at most {MAP_CAP} ids"
    );
    e.mem.set_u32(ids + 4 * n, id);
    e.mem
        .set_u32(map + addr::FORM_ID_MAP_NEXT_INDEX_OFFSET, n + 1);
    eax(n)
}

fn map_resolve(e: &mut Engine, a: &[u32]) -> Ret {
    let (map, index) = (a[0], a[1]);
    let ids = e.mem.u32(map);
    if index < e.mem.u32(map + addr::FORM_ID_MAP_NEXT_INDEX_OFFSET) {
        eax(e.mem.u32(ids + 4 * index))
    } else {
        eax(0)
    }
}

fn map_reset(e: &mut Engine, a: &[u32]) -> Ret {
    e.mem.set_u32(a[0] + addr::FORM_ID_MAP_NEXT_INDEX_OFFSET, 0);
    map_get_or_add(e, &[a[0], 0]);
    Ret::default()
}

fn map_construct(e: &mut Engine, a: &[u32]) -> Ret {
    let ids = new(e, 4 * MAP_CAP);
    e.mem.set_u32(a[0], ids);
    map_reset(e, a);
    eax(a[0])
}

/// The map the game object points at now.
fn current_map(e: &mut Engine) -> u32 {
    e.mem.u32(GAME + addr::GAME_FORM_ID_MAP_OFFSET)
}

// ---- Buffers ----------------------------------------------------------------------------------

/// Appends `bytes` and their `|` to a save buffer.
fn put(e: &mut Engine, buffer: u32, bytes: &[u8]) {
    let data = e.mem.u32(buffer + addr::BUFFER_DATA_OFFSET);
    let pos = e.mem.u32(buffer + addr::BUFFER_POSITION_OFFSET);
    assert!(pos + (bytes.len() as u32) < SAVE_CAP, "a test record fits");
    e.mem.write(data + pos, bytes);
    e.mem.set_u8(data + pos + bytes.len() as u32, b'|');
    e.mem.set_u32(
        buffer + addr::BUFFER_POSITION_OFFSET,
        pos + bytes.len() as u32 + 1,
    );
}

fn put_u32(e: &mut Engine, buffer: u32, v: u32) {
    put(e, buffer, &v.to_le_bytes());
}

/// A variable-sized value: one, two or four bytes, the size in the low two bits.
fn put_count(e: &mut Engine, buffer: u32, v: u32) {
    if v < 0x40 {
        put(e, buffer, &[(v << 2) as u8]);
    } else if v < 0x4000 {
        put(e, buffer, &((v << 2 | 1) as u16).to_le_bytes());
    } else {
        put(e, buffer, &(v << 2 | 2).to_le_bytes());
    }
}

/// The three bytes of form `id`'s index in the current map (`BGSNumericIDIndex::SetNumericID`).
fn index_bytes(e: &mut Engine, id: u32) -> [u8; 3] {
    let map = current_map(e);
    let mut index = map_get_or_add(e, &[map, id]).eax;
    if index >> 24 == 0xff {
        index |= 0x80_0000;
    }
    [(index >> 16) as u8, (index >> 8) as u8, index as u8]
}

/// The form id three index bytes stand for (`BGSNumericIDIndex::GetNumericID`).
fn id_of(e: &mut Engine, b: &[u8]) -> u32 {
    let index = u32::from(b[0]) << 16 | u32::from(b[1]) << 8 | u32::from(b[2]);
    if index & 0x80_0000 != 0 {
        return index & 0x7f_ffff | 0xff00_0000;
    }
    let map = current_map(e);
    map_resolve(e, &[map, index]).eax
}

/// The next `n` bytes of a load buffer and their `|`.
fn take(e: &mut Engine, buffer: u32, n: u32) -> Vec<u8> {
    let data = e.mem.u32(buffer + addr::BUFFER_DATA_OFFSET);
    let size = e.mem.u32(buffer + addr::BUFFER_SIZE_OFFSET);
    let pos = e.mem.u32(buffer + addr::BUFFER_POSITION_OFFSET);
    assert!(pos + n < size, "a record's value lies within it");
    let bytes = e.mem.bytes(data + pos, n);
    assert_eq!(e.mem.u8(data + pos + n), b'|', "a value ends in a bar");
    e.mem
        .set_u32(buffer + addr::BUFFER_POSITION_OFFSET, pos + n + 1);
    bytes
}

fn take_u32(e: &mut Engine, buffer: u32) -> u32 {
    let b = take(e, buffer, 4);
    u32::from_le_bytes([b[0], b[1], b[2], b[3]])
}

fn take_count(e: &mut Engine, buffer: u32) -> u32 {
    let data = e.mem.u32(buffer + addr::BUFFER_DATA_OFFSET);
    let pos = e.mem.u32(buffer + addr::BUFFER_POSITION_OFFSET);
    match e.mem.u8(data + pos) & 3 {
        0 => u32::from(take(e, buffer, 1)[0]) >> 2,
        1 => {
            let b = take(e, buffer, 2);
            u32::from(u16::from_le_bytes([b[0], b[1]])) >> 2
        }
        _ => take_u32(e, buffer) >> 2,
    }
}

/// The change flags in a form buffer's header.
fn header_flags(e: &mut Engine, buffer: u32) -> u32 {
    e.mem.u32(buffer + addr::FORM_BUFFER_HEADER_OFFSET + 3)
}

/// The game's save type of a form type (`crates/fos`, `SAVE_TYPES`), for the kit's kinds.
fn save_type(kind: u8) -> u8 {
    match kind {
        addr::FORM_TYPE_REFR => 0,
        addr::FORM_TYPE_INFO => 8,
        addr::FORM_TYPE_NOTE => 31,
        addr::FORM_TYPE_FACTION => 34,
        addr::FORM_TYPE_REPUTATION => 43,
        other => panic!("the kit has no form of type {other:#x}"),
    }
}

fn save_buffer_construct(e: &mut Engine, a: &[u32]) -> Ret {
    let block = new(e, SAVE_CAP);
    e.mem
        .write(a[0], &[0u8; addr::SAVE_FORM_BUFFER_SIZE as usize]);
    e.mem.set_u32(a[0] + addr::BUFFER_DATA_OFFSET, block);
    e.mem.set_u32(a[0] + addr::BUFFER_SIZE_OFFSET, SAVE_CAP);
    bump(e, SAVES);
    eax(a[0])
}

fn save_buffer_set_header(e: &mut Engine, a: &[u32]) -> Ret {
    let (buffer, id, flags, kind, minor) = (a[0], a[1], a[2], a[3] as u8, a[4] as u8);
    let at = buffer + addr::FORM_BUFFER_HEADER_OFFSET;
    let index = index_bytes(e, id);
    e.mem.write(at, &index);
    e.mem.set_u32(at + 3, flags);
    e.mem.set_u8(at + 7, save_type(kind));
    e.mem.set_u8(at + 8, minor);
    Ret::default()
}

fn load_buffer_construct(e: &mut Engine, a: &[u32]) -> Ret {
    e.mem
        .write(a[0], &[0u8; addr::LOAD_FORM_BUFFER_SIZE as usize]);
    eax(a[0])
}

// ---- A placed object --------------------------------------------------------------------------

/// A reference moved within its cell has initial data; one moved to another cell has more, which
/// the kit does not make.
fn has_place(e: &mut Engine, form: u32, flags: u32) -> bool {
    e.mem.u8(form + addr::FORM_TYPE_OFFSET) == addr::FORM_TYPE_REFR
        && flags & (addr::CHANGE_REFR_MOVE | addr::CHANGE_REFR_HAVOK_MOVE) != 0
}

/// `0084eb80`: a moved reference's cell, position and rotation, 27 bytes and one bar.
fn save_initial_data(e: &mut Engine, a: &[u32]) -> Ret {
    let (buffer, form, flags) = (a[0], a[1], a[2]);
    if !has_place(e, form, flags) {
        return eax(0);
    }
    let cell = match e.mem.u32(form + addr::REF_CELL_OFFSET) {
        0 => 0,
        c => e.mem.u32(c + addr::FORM_ID_OFFSET),
    };
    let mut bytes = index_bytes(e, cell).to_vec();
    bytes.extend(e.mem.bytes(form + addr::REF_POS_OFFSET, 12));
    bytes.extend(e.mem.bytes(form + REF_ROT_OFFSET, 12));
    put(e, buffer, &bytes);
    eax(4)
}

/// `0084f330`: the same, read and put on the reference.
fn load_initial_data(e: &mut Engine, a: &[u32]) -> Ret {
    let (buffer, form, flags) = (a[0], a[1], a[2]);
    if !has_place(e, form, flags) {
        return eax(0);
    }
    let bytes = take(e, buffer, 27);
    let cell = id_of(e, &bytes[..3]);
    let cell = forms::form(e, cell).unwrap_or(0);
    e.mem.set_u32(form + addr::REF_CELL_OFFSET, cell);
    e.mem.write(form + addr::REF_POS_OFFSET, &bytes[3..15]);
    e.mem.write(form + REF_ROT_OFFSET, &bytes[15..27]);
    eax(4)
}

fn ref_save(e: &mut Engine, a: &[u32]) -> Ret {
    let (r, buffer) = (a[0], a[1]);
    let flags = header_flags(e, buffer);
    if flags & addr::CHANGE_FORM_FLAGS != 0 {
        let v = e.mem.u32(r + addr::FORM_FLAGS_OFFSET);
        put_u32(e, buffer, v);
    }
    if flags & addr::CHANGE_OBJECT_EXTRA_LOCK != 0 {
        let has = e.mem.u8(r + HAS_LOCK);
        put(e, buffer, &[has]);
        if has != 0 {
            let level = e.mem.u8(r + LOCK + addr::LOCK_LEVEL_OFFSET);
            let lock_flags = e.mem.u8(r + LOCK + addr::LOCK_FLAGS_OFFSET);
            let tries = e.mem.u32(r + LOCK + addr::LOCK_TRIES_OFFSET);
            put(e, buffer, &[level, lock_flags]);
            put_u32(e, buffer, tries);
        }
    }
    if flags & addr::CHANGE_REFR_INVENTORY != 0 {
        let n = e.mem.u32(r + ITEM_COUNT);
        put_count(e, buffer, n);
        for i in 0..n {
            let item = e.mem.u32(r + ITEMS + 8 * i);
            let count = e.mem.u32(r + ITEMS + 8 * i + 4);
            let index = index_bytes(e, item);
            put(e, buffer, &index);
            put_u32(e, buffer, count);
        }
    }
    Ret::default()
}

fn ref_load(e: &mut Engine, a: &[u32]) -> Ret {
    let (r, buffer) = (a[0], a[1]);
    let flags = header_flags(e, buffer);
    if flags & addr::CHANGE_FORM_FLAGS != 0 {
        let v = take_u32(e, buffer);
        e.mem.set_u32(r + addr::FORM_FLAGS_OFFSET, v);
    }
    if flags & addr::CHANGE_OBJECT_EXTRA_LOCK != 0 {
        let has = take(e, buffer, 1)[0];
        e.mem.set_u8(r + HAS_LOCK, has);
        if has != 0 {
            let b = take(e, buffer, 2);
            let tries = take_u32(e, buffer);
            e.mem.set_u8(r + LOCK + addr::LOCK_LEVEL_OFFSET, b[0]);
            e.mem.set_u8(r + LOCK + addr::LOCK_FLAGS_OFFSET, b[1]);
            e.mem.set_u32(r + LOCK + addr::LOCK_TRIES_OFFSET, tries);
        }
    }
    // The open state is the flag itself: open by default or not, the other way with the flag.
    if e.mem.u8(r + OPENS) != 0 {
        let changed = flags & addr::CHANGE_OBJECT_OPEN_STATE != 0;
        let open = (e.mem.u8(r + DEFAULT_OPEN) != 0) != changed;
        e.mem.set_u8(r + OPEN, u8::from(open));
    }
    if flags & addr::CHANGE_REFR_INVENTORY != 0 {
        let n = take_count(e, buffer);
        assert!(n <= ITEMS_MAX);
        e.mem.set_u32(r + ITEM_COUNT, n);
        for i in 0..n {
            let index = take(e, buffer, 3);
            let item = id_of(e, &index);
            let count = take_u32(e, buffer);
            e.mem.set_u32(r + ITEMS + 8 * i, item);
            e.mem.set_u32(r + ITEMS + 8 * i + 4, count);
        }
    }
    bump(e, LOADS);
    // The object's own script, as a load may start one: it has no quest, so on a guest only the
    // apply scope holds it back.
    e.call(
        addr::SCRIPT_RUNNER_RUN,
        &args![
            0u32,
            r + OBJECT_SCRIPT,
            0u32,
            0u32,
            0u32,
            0u32,
            0u32,
            0u32,
            0u32
        ],
    );
    Ret::default()
}

// ---- A faction, a reputation, a note or a response --------------------------------------------

fn faction_save(e: &mut Engine, a: &[u32]) -> Ret {
    let (f, buffer) = (a[0], a[1]);
    let flags = header_flags(e, buffer);
    if flags & 1 != 0 {
        let v = e.mem.u32(f + addr::FORM_FLAGS_OFFSET);
        put_u32(e, buffer, v);
    }
    if flags & 4 != 0 {
        let n = e.mem.u32(f + REACTION_COUNT);
        put_count(e, buffer, n);
        for i in 0..n {
            let other = e.mem.u32(f + REACTIONS + 12 * i);
            let index = index_bytes(e, other);
            put(e, buffer, &index);
            for o in [4, 8] {
                let v = e.mem.u32(f + REACTIONS + 12 * i + o);
                put_u32(e, buffer, v);
            }
        }
    }
    if flags & 2 != 0 {
        let v = e.mem.u32(f + FACTION_FLAGS);
        put_u32(e, buffer, v);
    }
    if flags & 0x8000_0000 != 0 {
        for o in [0, 4] {
            let v = e.mem.u32(f + FACTION_CRIMES + o);
            put_u32(e, buffer, v);
        }
    }
    Ret::default()
}

fn faction_load(e: &mut Engine, a: &[u32]) -> Ret {
    let (f, buffer) = (a[0], a[1]);
    let flags = header_flags(e, buffer);
    if flags & 1 != 0 {
        let v = take_u32(e, buffer);
        e.mem.set_u32(f + addr::FORM_FLAGS_OFFSET, v);
    }
    if flags & 4 != 0 {
        let n = take_count(e, buffer);
        assert!(n <= 4);
        e.mem.set_u32(f + REACTION_COUNT, n);
        for i in 0..n {
            let index = take(e, buffer, 3);
            let other = id_of(e, &index);
            e.mem.set_u32(f + REACTIONS + 12 * i, other);
            for o in [4, 8] {
                let v = take_u32(e, buffer);
                e.mem.set_u32(f + REACTIONS + 12 * i + o, v);
            }
        }
    }
    if flags & 2 != 0 {
        let v = take_u32(e, buffer);
        e.mem.set_u32(f + FACTION_FLAGS, v);
    }
    if flags & 0x8000_0000 != 0 {
        for o in [0, 4] {
            let v = take_u32(e, buffer);
            e.mem.set_u32(f + FACTION_CRIMES + o, v);
        }
    }
    bump(e, LOADS);
    Ret::default()
}

fn reputation_save(e: &mut Engine, a: &[u32]) -> Ret {
    let (r, buffer) = (a[0], a[1]);
    if header_flags(e, buffer) & 1 != 0 {
        let v = e.mem.u32(r + addr::FORM_FLAGS_OFFSET);
        put_u32(e, buffer, v);
    }
    for o in [0, 4] {
        let v = e.mem.u32(r + VALUES + o);
        put_u32(e, buffer, v);
    }
    Ret::default()
}

fn reputation_load(e: &mut Engine, a: &[u32]) -> Ret {
    let (r, buffer) = (a[0], a[1]);
    if header_flags(e, buffer) & 1 != 0 {
        let v = take_u32(e, buffer);
        e.mem.set_u32(r + addr::FORM_FLAGS_OFFSET, v);
    }
    for o in [0, 4] {
        let v = take_u32(e, buffer);
        e.mem.set_u32(r + VALUES + o, v);
    }
    bump(e, LOADS);
    Ret::default()
}

/// `TESForm::SaveGame`: the form flags when they changed, nothing else.
fn flag_save(e: &mut Engine, a: &[u32]) -> Ret {
    let (f, buffer) = (a[0], a[1]);
    if header_flags(e, buffer) & 1 != 0 {
        let v = e.mem.u32(f + addr::FORM_FLAGS_OFFSET);
        put_u32(e, buffer, v);
    }
    Ret::default()
}

/// A note is read, a response said once, when its record carries the flag.
fn flag_load(e: &mut Engine, a: &[u32]) -> Ret {
    let (f, buffer) = (a[0], a[1]);
    let flags = header_flags(e, buffer);
    if flags & 1 != 0 {
        let v = take_u32(e, buffer);
        e.mem.set_u32(f + addr::FORM_FLAGS_OFFSET, v);
    }
    e.mem.set_u8(f + MARKED, u8::from(flags & 0x8000_0000 != 0));
    bump(e, LOADS);
    Ret::default()
}

// ---- The changes map, doors and locks ---------------------------------------------------------

fn flags_entry(e: &mut Engine, id: u32) -> Option<u32> {
    let n = e.mem.u32(FLAGS);
    (0..n)
        .map(|i| FLAGS + 4 + 8 * i)
        .find(|at| e.mem.u32(*at) == id)
}

fn set_flags(e: &mut Engine, id: u32, change: impl FnOnce(u32) -> u32) {
    let at = match flags_entry(e, id) {
        Some(at) => at,
        None => {
            let n = e.mem.u32(FLAGS);
            assert!(n < FLAGS_MAX, "the kit's changes map is full");
            e.mem.set_u32(FLAGS, n + 1);
            e.mem.set_u32(FLAGS + 4 + 8 * n, id);
            FLAGS + 4 + 8 * n
        }
    };
    let now = e.mem.u32(at + 4);
    e.mem.set_u32(at + 4, change(now));
}

/// The game's `TESForm::AddChange`: the save's map gets the flags, and the call goes through
/// `BGSSaveLoadGame::AddChange`, where the journal listens.
fn add_change(e: &mut Engine, form: u32, flags: u32) {
    let id = e.mem.u32(form + addr::FORM_ID_OFFSET);
    set_flags(e, id, |f| f | flags);
    e.call(addr::SAVE_LOAD_ADD_CHANGE, &args![GAME, form, flags, 0u32]);
}

/// The game's `TESForm::RemoveChange`: the save's map loses the flags, and nothing tells the
/// journal.
fn remove_change(e: &mut Engine, form: u32, flags: u32) {
    let id = e.mem.u32(form + addr::FORM_ID_OFFSET);
    set_flags(e, id, |f| f & !flags);
}

fn changes_map_get_flags(e: &mut Engine, a: &[u32]) -> Ret {
    let (out, id) = (a[1], a[2]);
    let flags = match flags_entry(e, id) {
        Some(at) => e.mem.u32(at + 4),
        None => 0,
    };
    e.mem.set_u32(out, flags);
    eax(out)
}

fn get_open_state(e: &mut Engine, a: &[u32]) -> Ret {
    let r = a[0];
    if r == 0 || e.mem.u8(r + OPENS) == 0 {
        return eax(0);
    }
    eax(if e.mem.u8(r + OPEN) != 0 { 1 } else { 3 })
}

fn set_open_state(e: &mut Engine, a: &[u32]) -> Ret {
    let (r, open) = (a[0], a[1] & 0xff != 0);
    if r == 0 || e.mem.u8(r + OPENS) == 0 {
        return eax(0);
    }
    e.mem.set_u8(r + OPEN, u8::from(open));
    if open != (e.mem.u8(r + DEFAULT_OPEN) != 0) {
        add_change(e, r, addr::CHANGE_OBJECT_OPEN_STATE);
    } else {
        remove_change(e, r, addr::CHANGE_OBJECT_OPEN_STATE);
    }
    eax(1)
}

fn own_lock(e: &mut Engine, a: &[u32]) -> Ret {
    eax(if e.mem.u8(a[0] + HAS_LOCK) != 0 {
        a[0] + LOCK
    } else {
        0
    })
}

fn add_lock_change(e: &mut Engine, a: &[u32]) -> Ret {
    add_change(e, a[0], addr::CHANGE_OBJECT_EXTRA_LOCK);
    Ret::default()
}

fn set_locked(e: &mut Engine, r: u32, locked: bool) {
    if e.mem.u8(r + HAS_LOCK) == 0 {
        return;
    }
    let at = r + LOCK + addr::LOCK_FLAGS_OFFSET;
    let flags = e.mem.u8(at) & !addr::LOCK_FLAG_LOCKED;
    e.mem
        .set_u8(at, flags | if locked { addr::LOCK_FLAG_LOCKED } else { 0 });
    add_change(e, r, addr::CHANGE_OBJECT_EXTRA_LOCK);
}

// ---- The unload pass --------------------------------------------------------------------------

fn array_add(e: &mut Engine, a: &[u32]) -> Ret {
    let n = e.mem.u32(PENDING);
    let buffer = e.mem.u32(a[1]);
    e.mem.set_u32(PENDING + 4 + 4 * n, buffer);
    e.mem.set_u32(PENDING, n + 1);
    Ret::default()
}

/// `008492b0`: each buffer's form gets `InitLoadGame`, then each `FinishLoadGame`, and the array
/// is emptied.
fn unload_pass(e: &mut Engine, _: &[u32]) -> Ret {
    let n = e.mem.u32(PENDING);
    for slot in [SLOT_INIT, SLOT_FINISH] {
        for i in 0..n {
            let buffer = e.mem.u32(PENDING + 4 + 4 * i);
            let form = e.mem.u32(buffer + 0x24);
            if form != 0 {
                e.vcall(form, slot, &args![buffer]);
            }
        }
    }
    e.mem.set_u32(PENDING, 0);
    bump(e, PASSES);
    Ret::default()
}

// ---- The kit ----------------------------------------------------------------------------------

fn vtable(kind: u8) -> u32 {
    VTABLES
        + 0x100
            * match kind {
                addr::FORM_TYPE_REFR => 0,
                addr::FORM_TYPE_FACTION => 1,
                addr::FORM_TYPE_REPUTATION => 2,
                _ => 3,
            }
}

fn make(e: &mut Engine, kind: u8, id: u32) -> u32 {
    let n = e.mem.u32(MADE);
    assert!(n < FORMS_MAX, "the kit holds at most {FORMS_MAX} forms");
    e.mem.set_u32(MADE, n + 1);
    let f = FORMS + FORM_LEN * n;
    e.mem.set_u32(f, vtable(kind));
    e.mem.set_u8(f + addr::FORM_TYPE_OFFSET, kind);
    e.mem.set_u32(f + addr::FORM_ID_OFFSET, id);
    testkit::add_form(e, id, f);
    f
}

fn form_of(s: &mut Story, id: u32) -> u32 {
    forms::form(&mut s.e, id).expect("a form of the kit, in the game now")
}

/// Adds `spec`'s forms to `story` and the stand-ins for the game's save and load of one form,
/// its changes map, doors and locks.
pub fn install(story: &mut Story, spec: &WorldSpec) {
    let e = &mut story.e;
    e.map(WK, WK_LEN);
    e.map(addr::SAVE_LOAD_GAME & !0xfff, 0x1000);
    e.mem.set_u32(NEXT, HEAP);
    e.mem.set_u32(addr::SAVE_LOAD_GAME, GAME);
    e.mem.set_u32(addr::SAVE_LOAD_MANAGER, MANAGER);
    e.mem.set_u32(GAME + addr::GAME_CHANGES_MAP_OFFSET, FLAGS);
    map_construct(e, &[OWN_MAP, addr::FORM_ID_MAP_BUCKETS]);
    e.mem.set_u32(GAME + addr::GAME_FORM_ID_MAP_OFFSET, OWN_MAP);
    for (kind, save, load) in [
        (addr::FORM_TYPE_REFR, REF_SAVE, REF_LOAD),
        (addr::FORM_TYPE_FACTION, FACTION_SAVE, FACTION_LOAD),
        (addr::FORM_TYPE_REPUTATION, REPUTATION_SAVE, REPUTATION_LOAD),
        (addr::FORM_TYPE_NOTE, FLAG_SAVE, FLAG_LOAD),
    ] {
        let v = vtable(kind);
        e.mem.set_u32(v + addr::FORM_SLOT_SAVE_GAME, save);
        e.mem.set_u32(v + addr::FORM_SLOT_LOAD_GAME, load);
        e.mem.set_u32(v + addr::FORM_SLOT_CHECK_SAVE_GAME, CHECK);
        e.mem.set_u32(v + SLOT_INIT, INIT);
        e.mem.set_u32(v + SLOT_FINISH, FINISH);
    }
    e.register(REF_SAVE, ref_save);
    e.register(REF_LOAD, ref_load);
    e.register(FACTION_SAVE, faction_save);
    e.register(FACTION_LOAD, faction_load);
    e.register(REPUTATION_SAVE, reputation_save);
    e.register(REPUTATION_LOAD, reputation_load);
    e.register(FLAG_SAVE, flag_save);
    e.register(FLAG_LOAD, flag_load);
    e.register(CHECK, |_, _| Ret::default());
    e.register(INIT, |e, _| {
        bump(e, INITS);
        Ret::default()
    });
    e.register(FINISH, |e, _| {
        bump(e, FINISHES);
        Ret::default()
    });
    e.register(addr::OPERATOR_NEW, |e, a| {
        let at = new(e, a[0]);
        eax(at)
    });
    e.register(addr::OPERATOR_DELETE, |_, _| Ret::default());
    e.register(addr::FORM_ID_MAP_CONSTRUCT, map_construct);
    e.register(addr::FORM_ID_MAP_GET_OR_ADD, map_get_or_add);
    e.register(addr::FORM_ID_MAP_RESOLVE, map_resolve);
    e.register(addr::FORM_ID_MAP_RESET, map_reset);
    e.register(addr::MANAGER_GET_MINOR_VERSION, |_, _| eax(MINOR));
    e.register(addr::GAME_LOADING_FLAG_TEST, |e, _| eax(e.mem.u32(LOADING)));
    e.register(addr::CHANGES_MAP_GET_FLAGS, changes_map_get_flags);
    e.register(addr::SAVE_FORM_BUFFER_CONSTRUCT, save_buffer_construct);
    e.register(addr::SAVE_FORM_BUFFER_SET_HEADER, save_buffer_set_header);
    e.register(addr::SAVE_FORM_BUFFER_SET_FORM, |e, a| {
        e.mem.set_u32(a[0] + 0x20, a[1]);
        Ret::default()
    });
    e.register(addr::SAVE_FORM_BUFFER_DESTRUCT, |_, _| Ret::default());
    e.register(addr::SAVE_INITIAL_DATA, save_initial_data);
    e.register(addr::LOAD_INITIAL_DATA, load_initial_data);
    e.register(addr::LOAD_FORM_BUFFER_CONSTRUCT, load_buffer_construct);
    e.register(addr::LOAD_FORM_BUFFER_SET_FORM, |e, a| {
        e.mem.set_u32(a[0] + 0x24, a[1]);
        Ret::default()
    });
    e.register(addr::LOAD_FORM_BUFFER_SET_HEADER_FLAGS, |e, a| {
        e.mem
            .set_u32(a[0] + addr::FORM_BUFFER_HEADER_OFFSET + 3, a[1]);
        Ret::default()
    });
    e.register(addr::FORM_MARK_LOADED, |_, _| Ret::default());
    e.register(addr::SET_THREAD_FLAG, |_, _| eax(1));
    e.register(addr::ARRAY_ADD, array_add);
    e.register(addr::GAME_UNLOAD_PASS, unload_pass);
    e.register(addr::GET_OPEN_STATE, get_open_state);
    e.register(addr::SET_OPEN_STATE, set_open_state);
    e.register(addr::REF_OWN_LOCK, own_lock);
    e.register(addr::REF_ADD_LOCK_CHANGE, add_lock_change);
    e.register(addr::REF_LOCK, |e, a| {
        set_locked(e, a[0], true);
        Ret::default()
    });
    e.register(addr::REF_UNLOCK, |e, a| {
        set_locked(e, a[0], false);
        Ret::default()
    });
    for r in &spec.refs {
        assert!(r.items.len() as u32 <= ITEMS_MAX);
        let f = make(e, addr::FORM_TYPE_REFR, r.id);
        let cell = if r.cell == 0 {
            0
        } else {
            forms::form(e, r.cell).expect("a cell of the story")
        };
        e.mem.set_u32(f + addr::REF_CELL_OFFSET, cell);
        e.mem.set_u8(f + OPENS, u8::from(r.opens));
        if let Some(level) = r.lock {
            e.mem.set_u8(f + HAS_LOCK, 1);
            e.mem.set_u8(f + LOCK + addr::LOCK_LEVEL_OFFSET, level);
            e.mem
                .set_u8(f + LOCK + addr::LOCK_FLAGS_OFFSET, addr::LOCK_FLAG_LOCKED);
        }
        e.mem.set_u32(f + ITEM_COUNT, r.items.len() as u32);
        for (i, (item, count)) in r.items.iter().enumerate() {
            e.mem.set_u32(f + ITEMS + 8 * i as u32, *item);
            e.mem.set_u32(f + ITEMS + 8 * i as u32 + 4, *count as u32);
        }
    }
    for (kind, ids) in [
        (addr::FORM_TYPE_FACTION, &spec.factions),
        (addr::FORM_TYPE_REPUTATION, &spec.reputations),
        (addr::FORM_TYPE_NOTE, &spec.notes),
        (addr::FORM_TYPE_INFO, &spec.topics),
    ] {
        for id in ids {
            make(e, kind, *id);
        }
    }
}

/// Opens or shuts a placed object as activating it does.
pub fn open(s: &mut Story, id: u32, to: bool) {
    assert!(crate::world::set_open(&mut s.e, id, to));
}

/// Locks or unlocks a placed object as a key, a pick or a script does.
pub fn set_locked_as_played(s: &mut Story, id: u32, locked: bool) {
    let r = form_of(s, id);
    set_locked(&mut s.e, r, locked);
}

/// Counts a failed forcing of a lock.
pub fn fail_lock(s: &mut Story, id: u32) {
    let r = form_of(s, id);
    let at = r + LOCK + addr::LOCK_TRIES_OFFSET;
    let n = s.e.mem.u32(at);
    s.e.mem.set_u32(at, n + 1);
    add_change(&mut s.e, r, addr::CHANGE_OBJECT_EXTRA_LOCK);
}

/// Disables or enables a placed object as a script's `Disable` and `Enable` do.
pub fn disable(s: &mut Story, id: u32, disabled: bool) {
    let r = form_of(s, id);
    let flags = s.e.mem.u32(r + addr::FORM_FLAGS_OFFSET) & !addr::FORM_FLAG_DISABLED;
    s.e.mem.set_u32(
        r + addr::FORM_FLAGS_OFFSET,
        flags
            | if disabled {
                addr::FORM_FLAG_DISABLED
            } else {
                0
            },
    );
    add_change(&mut s.e, r, addr::CHANGE_FORM_FLAGS);
}

/// Moves a placed object within its cell as a script's `SetPos` does.
pub fn move_to(s: &mut Story, id: u32, pos: [f32; 3]) {
    let r = form_of(s, id);
    for (i, v) in pos.into_iter().enumerate() {
        s.e.mem.set_f32(r + addr::REF_POS_OFFSET + 4 * i as u32, v);
    }
    add_change(&mut s.e, r, addr::CHANGE_REFR_MOVE);
}

/// Sets how many of `item` a container holds (0 takes it out), as a take or a put does.
pub fn set_item(s: &mut Story, id: u32, item: u32, count: i32) {
    let r = form_of(s, id);
    let mut items = ref_state(s, id).expect("in the game").items;
    items.retain(|(i, _)| *i != item);
    if count != 0 {
        items.push((item, count));
    }
    assert!(items.len() as u32 <= ITEMS_MAX);
    s.e.mem.set_u32(r + ITEM_COUNT, items.len() as u32);
    for (i, (item, count)) in items.into_iter().enumerate() {
        s.e.mem.set_u32(r + ITEMS + 8 * i as u32, item);
        s.e.mem.set_u32(r + ITEMS + 8 * i as u32 + 4, count as u32);
    }
    add_change(&mut s.e, r, addr::CHANGE_REFR_INVENTORY);
}

/// A placed object's state; `None` while it is not in the game.
pub fn ref_state(s: &mut Story, id: u32) -> Option<RefState> {
    let r = forms::form(&mut s.e, id)?;
    let m = &s.e.mem;
    let lock = (m.u8(r + HAS_LOCK) != 0).then(|| {
        (
            m.u8(r + LOCK + addr::LOCK_LEVEL_OFFSET),
            m.u8(r + LOCK + addr::LOCK_FLAGS_OFFSET),
            m.u32(r + LOCK + addr::LOCK_TRIES_OFFSET),
        )
    });
    Some(RefState {
        disabled: m.u32(r + addr::FORM_FLAGS_OFFSET) & addr::FORM_FLAG_DISABLED != 0,
        pos: [0, 4, 8].map(|o| m.f32(r + addr::REF_POS_OFFSET + o)),
        open: m.u8(r + OPEN) != 0,
        lock,
        items: (0..m.u32(r + ITEM_COUNT))
            .map(|i| {
                (
                    m.u32(r + ITEMS + 8 * i),
                    m.u32(r + ITEMS + 8 * i + 4) as i32,
                )
            })
            .collect(),
    })
}

/// Sets a faction's flags, crime counts and reactions, as scripts and crimes do; `flags` are the
/// change flags the game gives for it (2 flags, 4 reactions, 0x80000000 crime counts).
pub fn set_faction(s: &mut Story, id: u32, state: &FactionState, flags: u32) {
    let f = form_of(s, id);
    assert!(state.2.len() <= 4);
    s.e.mem.set_u32(f + FACTION_FLAGS, state.0);
    for (i, c) in state.1.into_iter().enumerate() {
        s.e.mem.set_u32(f + FACTION_CRIMES + 4 * i as u32, c as u32);
    }
    s.e.mem.set_u32(f + REACTION_COUNT, state.2.len() as u32);
    for (i, (other, modifier, group)) in state.2.iter().enumerate() {
        let at = f + REACTIONS + 12 * i as u32;
        s.e.mem.set_u32(at, *other);
        s.e.mem.set_u32(at + 4, *modifier as u32);
        s.e.mem.set_u32(at + 8, *group as u32);
    }
    add_change(&mut s.e, f, flags);
}

/// A faction's state.
pub fn faction(s: &mut Story, id: u32) -> FactionState {
    let f = form_of(s, id);
    let m = &s.e.mem;
    (
        m.u32(f + FACTION_FLAGS),
        [0, 4].map(|o| m.u32(f + FACTION_CRIMES + o) as i32),
        (0..m.u32(f + REACTION_COUNT))
            .map(|i| {
                let at = f + REACTIONS + 12 * i;
                (m.u32(at), m.u32(at + 4) as i32, m.u32(at + 8) as i32)
            })
            .collect(),
    )
}

/// Sets a reputation's fame and infamy, as `AddReputation` does.
pub fn set_reputation(s: &mut Story, id: u32, fame: f32, infamy: f32) {
    let r = form_of(s, id);
    s.e.mem.set_f32(r + VALUES, fame);
    s.e.mem.set_f32(r + VALUES + 4, infamy);
    add_change(&mut s.e, r, 2);
}

/// A reputation's fame and infamy.
pub fn reputation(s: &mut Story, id: u32) -> (f32, f32) {
    let r = form_of(s, id);
    (s.e.mem.f32(r + VALUES), s.e.mem.f32(r + VALUES + 4))
}

/// Marks a note read or a dialogue response said once, as reading or saying it does.
pub fn mark(s: &mut Story, id: u32) {
    let f = form_of(s, id);
    s.e.mem.set_u8(f + MARKED, 1);
    add_change(&mut s.e, f, 0x8000_0000);
}

/// Whether a note is read or a response was said.
pub fn marked(s: &mut Story, id: u32) -> bool {
    let f = form_of(s, id);
    s.e.mem.u8(f + MARKED) != 0
}

/// Makes the game say a load is running (or not).
pub fn set_loading(s: &mut Story, on: bool) {
    s.e.mem.set_u32(LOADING, u32::from(on));
}

/// How many forms took a record (`LoadGame` calls).
pub fn loads(s: &Story) -> u32 {
    s.e.mem.u32(LOADS)
}

/// `(unload passes, InitLoadGame calls, FinishLoadGame calls)`.
pub fn passes(s: &Story) -> (u32, u32, u32) {
    (
        s.e.mem.u32(PASSES),
        s.e.mem.u32(INITS),
        s.e.mem.u32(FINISHES),
    )
}

/// How many records the game itself was asked to write (`BGSSaveFormBuffer`s made).
pub fn saves(s: &Story) -> u32 {
    s.e.mem.u32(SAVES)
}

/// How many form ids the game's own save numbering holds (the null id among them): co-op's
/// records must never add to it.
pub fn own_numbering(s: &Story) -> u32 {
    s.e.mem.u32(OWN_MAP + addr::FORM_ID_MAP_NEXT_INDEX_OFFSET)
}

/// Whether the game object points at its own form id map (co-op's is only in its place while a
/// record is made or loaded).
pub fn own_map_in_place(s: &Story) -> bool {
    s.e.mem.u32(GAME + addr::GAME_FORM_ID_MAP_OFFSET) == OWN_MAP
}
