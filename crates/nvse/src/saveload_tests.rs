//! Tests of the co-save's save and load (I-10), with the game's file calls, its save folder and
//! its file list stood in.

use super::*;
use crate::vars::{Key, Value, KEY_NUMBER, KEY_STRING};
use std::cell::RefCell;
use std::collections::HashMap;

const AREA: u32 = 0x0200_0000;
const HANDLER: u32 = AREA + 0x1000;
const INFOS: u32 = AREA + 0x4000;
const NODES: u32 = AREA + 0x6000;
const NAME: u32 = AREA + 0x8000;

thread_local! {
    /// The stand-in file system: files by path, open handles (path, position, writing), the
    /// calls seen.
    static FILES: RefCell<HashMap<Vec<u8>, Vec<u8>>> = RefCell::new(HashMap::new());
    static HANDLES: RefCell<HashMap<u32, (Vec<u8>, usize)>> = RefCell::new(HashMap::new());
    static CALLS: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

fn text(e: &Engine, at: u32) -> Vec<u8> {
    crate::table::guest_text(e, at)
}

fn create_file(e: &mut Engine, a: &[u32]) -> Ret {
    let path = text(e, a[0]);
    let write = a[1] & GENERIC_WRITE != 0;
    let exists = FILES.with(|f| f.borrow().contains_key(&path));
    if !write && !exists {
        return Ret {
            eax: INVALID_HANDLE_VALUE,
            ..Ret::default()
        };
    }
    if write {
        FILES.with(|f| f.borrow_mut().insert(path.clone(), Vec::new()));
    }
    let h = HANDLES.with(|h| {
        let mut h = h.borrow_mut();
        let id = 0x100 + h.len() as u32;
        h.insert(id, (path, 0));
        id
    });
    Ret {
        eax: h,
        ..Ret::default()
    }
}

fn read_file_stub(e: &mut Engine, a: &[u32]) -> Ret {
    let (h, buf, n, read) = (a[0], a[1], a[2] as usize, a[3]);
    let (path, pos) = HANDLES.with(|hs| hs.borrow()[&h].clone());
    let data = FILES.with(|f| f.borrow()[&path].clone());
    let piece = &data[pos.min(data.len())..(pos + n).min(data.len())];
    e.mem.write(buf, piece);
    e.mem.set_u32(read, piece.len() as u32);
    HANDLES.with(|hs| hs.borrow_mut().get_mut(&h).unwrap().1 += piece.len());
    Ret {
        eax: 1,
        ..Ret::default()
    }
}

fn write_file_stub(e: &mut Engine, a: &[u32]) -> Ret {
    let (h, buf, n, written) = (a[0], a[1], a[2], a[3]);
    let path = HANDLES.with(|hs| hs.borrow()[&h].0.clone());
    let bytes = e.mem.bytes(buf, n);
    FILES.with(|f| f.borrow_mut().get_mut(&path).unwrap().extend(bytes));
    e.mem.set_u32(written, n);
    Ret {
        eax: 1,
        ..Ret::default()
    }
}

fn close_handle(_: &mut Engine, a: &[u32]) -> Ret {
    HANDLES.with(|hs| hs.borrow_mut().remove(&a[0]));
    Ret {
        eax: 1,
        ..Ret::default()
    }
}

fn delete_file_stub(e: &mut Engine, a: &[u32]) -> Ret {
    let path = text(e, a[0]);
    CALLS.with(|c| {
        c.borrow_mut()
            .push(format!("delete {}", String::from_utf8_lossy(&path)))
    });
    FILES.with(|f| f.borrow_mut().remove(&path));
    Ret::default()
}

fn rename_stub(e: &mut Engine, a: &[u32]) -> Ret {
    let (from, to) = (text(e, a[0]), text(e, a[1]));
    CALLS.with(|c| {
        c.borrow_mut().push(format!(
            "rename {} {}",
            String::from_utf8_lossy(&from),
            String::from_utf8_lossy(&to)
        ))
    });
    FILES.with(|f| {
        let mut f = f.borrow_mut();
        if let Some(d) = f.remove(&from) {
            f.insert(to, d);
        }
    });
    Ret::default()
}

fn save_dir_stub(e: &mut Engine, a: &[u32]) -> Ret {
    e.mem.write(a[1], b"C:\\Saves\\\0");
    Ret::default()
}

fn upper(_: &mut Engine, a: &[u32]) -> Ret {
    Ret {
        eax: u32::from((a[0] as u8).to_ascii_uppercase()),
        ..Ret::default()
    }
}

/// The game's files: `names` in the list of files, the loaded ones in that order.
fn set_files(e: &mut Engine, names: &[&str]) {
    e.mem.set_u32(DATA_HANDLER, HANDLER);
    e.mem.set_u8(HANDLER + LOADED_COUNT, names.len() as u8);
    for (i, name) in names.iter().enumerate() {
        let i = i as u32;
        let info = INFOS + i * 0x200;
        e.mem.write(info + MOD_NAME, name.as_bytes());
        e.mem.set_u8(info + MOD_NAME + name.len() as u32, 0);
        e.mem.set_u32(HANDLER + LOADED_MODS + 4 * i, info);
        let node = if i == 0 {
            HANDLER + MOD_INFO_LIST
        } else {
            NODES + i * 8
        };
        e.mem.set_u32(node, info);
        e.mem.set_u32(node + 4, 0);
        if i > 0 {
            let prev = if i == 1 {
                HANDLER + MOD_INFO_LIST
            } else {
                NODES + (i - 1) * 8
            };
            e.mem.set_u32(prev + 4, node);
        }
    }
}

fn engine() -> Engine {
    let mut e = Engine::new();
    e.map(AREA, 0x1_0000);
    e.map(DATA_HANDLER & !0xfff, 0x1000);
    e.map(SAVE_GAME_MANAGER & !0xfff, 0x1000);
    e.register(0x00ec_a7f4, upper);
    e.register(CREATE_FILE_A, create_file);
    e.register(READ_FILE, read_file_stub);
    e.register(WRITE_FILE, write_file_stub);
    e.register(CLOSE_HANDLE, close_handle);
    e.register(DELETE_FILE_A, delete_file_stub);
    e.register(CRT_RENAME, rename_stub);
    e.register(CONSTRUCT_SAVEGAME_PATH, save_dir_stub);
    install(&mut e);
    e.mem.write(NAME, b"Save 7.fos\0");
    e
}

fn file(path: &str) -> Option<Vec<u8>> {
    FILES.with(|f| f.borrow().get(path.as_bytes()).cloned())
}

/// The co-save's name: `.nvse` for the last extension, `.bak` suffixes kept, the save folder
/// before a bare name.
#[test]
fn co_save_names_as_xnvse() {
    let dir = || b"C:\\Saves\\".to_vec();
    assert_eq!(co_save_name(b"Save 1.fos", dir), b"C:\\Saves\\Save 1.nvse");
    assert_eq!(
        co_save_name(b"D:\\x\\a.b.fos.bak.bak", dir),
        b"D:\\x\\a.b.nvse.bak.bak"
    );
    assert_eq!(co_save_name(b"plain", dir), b"C:\\Saves\\plain.nvse");
}

/// xNVSE's map order: a key's bucket `(k * 13) ^ (k >> 15)` of 8, newest first in a bucket,
/// doubled once more entries than buckets are in.
#[test]
fn hash_order_follows_the_unordered_map() {
    let mut o = vars::HashOrder::default();
    for k in [1, 2, 3, 9] {
        o.insert(k);
    }
    // Buckets 13 & 7 = 5, 26 & 7 = 2, 39 & 7 = 7, 117 & 7 = 5.
    assert_eq!(o.keys(), [2, 9, 1, 3]);
    o.remove(9);
    assert_eq!(o.keys(), [2, 1, 3]);
    let mut o = vars::HashOrder::default();
    for k in 1..=10 {
        o.insert(k);
    }
    // The tenth insert finds 9 entries in 8 buckets: 16 buckets, bucket = 13k & 15.
    let mut want: Vec<(u32, u32)> = (1..=10).map(|k| ((13 * k) & 15, k)).collect();
    want.sort_by_key(|&(b, _)| b);
    assert_eq!(o.keys(), want.iter().map(|&(_, k)| k).collect::<Vec<_>>());
}

/// A save writes the core plugin (files, arrays, strings; temporaries, unreferenced arrays and
/// strings of no file left out) and a load before the `.fos` reads it back with the same ids;
/// a later load maps owners and form ids to today's file order and drops a removed file's
/// strings.
#[test]
fn variables_survive_a_save_and_a_load() {
    let mut e = engine();
    set_files(&mut e, &["FalloutNV.esm", "A.esp", "B.esp"]);
    let kept = vars::add_string(&mut e, 1, b"kept", false);
    let temp = vars::add_string(&mut e, 1, b"temp", true);
    let runtime = vars::add_string(&mut e, 0xff, b"console", false);
    let of_b = vars::add_string(&mut e, 2, b"of b", false);
    let arr = vars::create_array(&mut e, KEY_NUMBER, true, 1);
    vars::set_element(&mut e, arr, &Key::Number(0.0), Value::Number(2.5));
    vars::set_element(&mut e, arr, &Key::Number(1.0), Value::Str(b"x".to_vec()));
    vars::set_element(&mut e, arr, &Key::Number(2.0), Value::Form(0x0000_0abc));
    let map = vars::create_array(&mut e, KEY_STRING, false, 2);
    vars::set_element(&mut e, map, &Key::Str(b"Key".to_vec()), Value::Number(1.0));
    let mut held = 0;
    vars::add_reference(&mut e, &mut held, arr, 1);
    let mut held2 = 0;
    vars::add_reference(&mut e, &mut held2, map, 2);
    let unheld = vars::create_array(&mut e, KEY_NUMBER, false, 1);
    save_game(&mut e, NAME);
    let bytes = file("C:\\Saves\\Save 7.nvse").expect("written");
    let mut r = Reader::open(&bytes).unwrap();
    let p = r.next_plugin().unwrap().unwrap();
    assert_eq!(p.opcode_base, 0x1400);
    let mut kinds = Vec::new();
    while let Some((k, _, _)) = r.next_record().unwrap() {
        kinds.push(k.to_be_bytes());
    }
    assert_eq!(
        kinds,
        [
            *b"MODS", *b"ARVS", *b"ARVR", *b"ARVR", *b"ARVE", *b"STVS", *b"STVR", *b"STVR",
            *b"STVE"
        ]
    );
    assert!(!vars::array_exists(&e, unheld), "cleaned before the save");
    // A load: reset, then the same variables back.
    vars::add_string(&mut e, 1, b"from the game before", false);
    pre_load(&mut e, NAME);
    assert_eq!(vars::string(&e, kept), Some(b"kept".to_vec()));
    assert_eq!(vars::string(&e, of_b), Some(b"of b".to_vec()));
    assert!(vars::string(&e, temp).is_none());
    assert!(vars::string(&e, runtime).is_none());
    let a = vars::array(&e, arr).unwrap();
    assert_eq!((a.owning_mod, a.refs.clone(), a.len()), (1, vec![1], 3));
    assert_eq!(
        vars::get_element(&mut e, arr, &Key::Number(1.0))
            .unwrap()
            .data,
        Data::Str(b"x".to_vec())
    );
    assert_eq!(
        vars::get_element(&mut e, map, &Key::Str(b"KEY".to_vec()))
            .unwrap()
            .data,
        Data::Number(1.0)
    );
    // Today: B.esp is gone and A.esp moved to index 2.
    set_files(&mut e, &["FalloutNV.esm", "C.esp", "A.esp"]);
    pre_load(&mut e, NAME);
    assert!(vars::string(&e, of_b).is_none(), "its file is gone");
    assert_eq!(e.ext::<Nvse>().vars.strings[&kept].owning_mod, 2);
    assert_eq!(vars::array(&e, arr).unwrap().refs, vec![2]);
    assert_eq!(
        vars::get_element(&mut e, arr, &Key::Number(2.0))
            .unwrap()
            .data,
        Data::Form(0x0000_0abc),
        "an id of FalloutNV.esm stays"
    );
    assert!(
        !vars::array_exists(&e, map),
        "referred to by no loaded file"
    );
}

/// A delayed call as a script queues one.
fn delayed() -> crate::tick::Delayed {
    crate::tick::Delayed {
        script: 0,
        time: 1.0,
        this_obj: 0,
        mode: 0,
        args: Vec::new(),
        serial: 1,
    }
}

/// Review of #328 (the maintainer reversed the "co-save only when needed" rule, 2026-11-08): a
/// save with nothing of the extender's still gets its co-save, so loading it resets what the game
/// played before held: game A's variables and delayed calls do not leak into game B. A save
/// that really has none (a vanilla save) takes xNVSE's other path: the load before the `.fos`
/// changes nothing, the load after it is a new game (temporaries cleaned).
#[test]
fn a_save_with_nothing_in_it_still_resets_the_game_before() {
    let mut e = engine();
    set_files(&mut e, &["FalloutNV.esm"]);
    // Game B: nothing of the extender's.
    save_game(&mut e, NAME);
    assert!(file(CO_SAVE).is_some(), "written though empty");
    // Game A, played next, then B loaded.
    let of_a = vars::add_string(&mut e, 0, b"of game a", false);
    e.ext::<Nvse>().tick.after.push(delayed());
    pre_load(&mut e, NAME);
    assert!(vars::string(&e, of_a).is_none(), "reset");
    assert!(e.ext::<Nvse>().tick.after.is_empty(), "dropped");
    // A vanilla save: no co-save at all.
    FILES.with(|f| f.borrow_mut().remove(CO_SAVE.as_bytes()));
    let kept = vars::add_string(&mut e, 0, b"kept", false);
    let temp = vars::add_string(&mut e, 0, b"temp", true);
    pre_load(&mut e, NAME);
    assert!(vars::string(&e, kept).is_some() && vars::string(&e, temp).is_some());
    load(&mut e, NAME);
    assert!(vars::string(&e, kept).is_some());
    assert!(vars::string(&e, temp).is_none());
}

/// Arrays read out of id order: xNVSE marks the ids below each one it reads free from the last
/// one read, so a lower id read after a higher leaves a gap unmarked.
#[test]
fn array_ids_freed_as_xnvse_reads_them() {
    let mut e = engine();
    set_files(&mut e, &["FalloutNV.esm", "A.esp"]);
    let arvr = |id: u32| {
        let mut w = ChunkWriter::default();
        w.u8(1);
        w.u32(id);
        w.u8(KEY_NUMBER);
        w.u8(1);
        w.u32(1);
        w.u8(1);
        w.u32(0);
        Chunk {
            kind: tag(b"ARVR"),
            version: 2,
            data: w.data,
        }
    };
    let mut mods = ChunkWriter::default();
    mods.u8(2);
    for n in ["FalloutNV.esm", "A.esp"] {
        mods.u16(n.len() as u16);
        mods.bytes(n.as_bytes());
    }
    let marker = |t: &[u8; 4]| Chunk {
        kind: tag(t),
        version: 2,
        data: Vec::new(),
    };
    let bytes = cosave::write(&[Plugin {
        opcode_base: 0x1400,
        chunks: vec![
            Chunk {
                kind: tag(b"MODS"),
                version: 0,
                data: mods.data,
            },
            marker(b"ARVS"),
            arvr(5),
            arvr(3),
            marker(b"ARVE"),
        ],
    }]);
    FILES.with(|f| {
        f.borrow_mut()
            .insert(b"C:\\Saves\\Save 7.nvse".to_vec(), bytes)
    });
    pre_load(&mut e, NAME);
    let ids = &e.ext::<Nvse>().vars.array_ids;
    assert_eq!(ids.available.iter().collect::<Vec<_>>(), [1, 2, 4]);
    assert!(vars::array_exists(&e, 5) && vars::array_exists(&e, 3));
}

/// Deleting or renaming a save does the same to its co-save first.
#[test]
fn delete_and_rename_take_the_co_save_along() {
    let mut e = engine();
    e.mem.write(NAME, b"C:\\Saves\\Old.fos\0");
    e.mem.write(NAME + 0x100, b"C:\\Saves\\Old.fos.bak\0");
    e.site(DELETE.addr(), &[NAME]);
    e.site(RENAME.addr(), &[NAME, NAME + 0x100]);
    let calls = CALLS.with(|c| c.borrow().clone());
    assert_eq!(
        calls,
        [
            "delete C:\\Saves\\Old.nvse",
            "delete C:\\Saves\\Old.nvse.bak",
            "rename C:\\Saves\\Old.nvse C:\\Saves\\Old.nvse.bak"
        ]
    );
}

/// `Save 7`'s co-save in the stand-in save folder.
const CO_SAVE: &str = "C:\\Saves\\Save 7.nvse";

/// xNVSE's own plugin with the copy's two files and the given chunks between `ARVS` and
/// `ARVE`, written as the co-save of `Save 7`.
fn put_arrays(arrays: Vec<Chunk>, others: &[cosave::Kept]) -> Vec<u8> {
    let mut mods = ChunkWriter::default();
    mods.u8(2);
    for n in ["FalloutNV.esm", "A.esp"] {
        mods.u16(n.len() as u16);
        mods.bytes(n.as_bytes());
    }
    let marker = |t: &[u8; 4]| Chunk {
        kind: tag(t),
        version: 2,
        data: Vec::new(),
    };
    let mut chunks = vec![
        Chunk {
            kind: tag(b"MODS"),
            version: 0,
            data: mods.data,
        },
        marker(b"ARVS"),
    ];
    chunks.extend(arrays);
    chunks.push(marker(b"ARVE"));
    let bytes = cosave::write_with(
        &[Plugin {
            opcode_base: 0x1400,
            chunks,
        }],
        others,
    );
    FILES.with(|f| {
        f.borrow_mut()
            .insert(CO_SAVE.as_bytes().to_vec(), bytes.clone())
    });
    bytes
}

/// An array record of A.esp: its id, the reference count as written, a number map whose
/// element count is `count` with `elements` actually there (key, value).
fn map_record(id: u32, refs: u32, count: u32, elements: &[(f64, f64)]) -> Chunk {
    let mut w = ChunkWriter::default();
    w.u8(1);
    w.u32(id);
    w.u8(KEY_NUMBER);
    w.u8(0);
    w.u32(refs);
    if refs < 16 {
        for _ in 0..refs {
            w.u8(1);
        }
    }
    w.u32(count);
    for (k, v) in elements {
        w.f64(*k);
        w.u8(1);
        w.f64(*v);
    }
    Chunk {
        kind: tag(b"ARVR"),
        version: 2,
        data: w.data,
    }
}

/// Review item 1 of #328: counts and ids from a corrupt file. A reference or element count
/// larger than the bytes left in its chunk stops the read, as a read past the end does (no
/// loop over it, no allocation); an id of any size marks the ids below it free as one run.
#[test]
fn corrupt_co_saves_stop_the_read() {
    let mut e = engine();
    set_files(&mut e, &["FalloutNV.esm", "A.esp"]);
    // A huge id: one run of free ids, the next new array takes 1.
    put_arrays(vec![map_record(0xffff_fff0, 1, 1, &[(2.0, 3.0)])], &[]);
    pre_load(&mut e, NAME);
    assert!(vars::array_exists(&e, 0xffff_fff0));
    assert_eq!(e.ext::<Nvse>().vars.array_ids.available.runs(), 1);
    assert_eq!(vars::create_array(&mut e, KEY_NUMBER, true, 1), 1);
    // A huge reference count: the read stops there; the next array is not read.
    put_arrays(
        vec![map_record(4, 0xffff_ffff, 0, &[]), map_record(5, 1, 0, &[])],
        &[],
    );
    pre_load(&mut e, NAME);
    assert!(!vars::array_exists(&e, 4) && !vars::array_exists(&e, 5));
    // A huge element count: the array is in (xNVSE adds it first), nothing after it.
    put_arrays(
        vec![
            map_record(6, 1, 0x7fff_ffff, &[(1.0, 1.0)]),
            map_record(7, 1, 0, &[]),
        ],
        &[],
    );
    pre_load(&mut e, NAME);
    assert_eq!(vars::array(&e, 6).map(|a| a.len()), Some(0));
    assert!(!vars::array_exists(&e, 7));
    // The last id there is (no id after it): corrupt, the read stops; new ids start at 1.
    put_arrays(
        vec![map_record(u32::MAX, 1, 0, &[]), map_record(8, 1, 0, &[])],
        &[],
    );
    pre_load(&mut e, NAME);
    assert!(!vars::array_exists(&e, u32::MAX) && !vars::array_exists(&e, 8));
    assert_eq!(vars::create_array(&mut e, KEY_NUMBER, true, 1), 1);
    put_strings(&[(u32::MAX, b"last"), (3, b"after")]);
    pre_load(&mut e, NAME);
    assert!(vars::string(&e, u32::MAX).is_none() && vars::string(&e, 3).is_none());
    assert_eq!(vars::add_string(&mut e, 1, b"new", false), 1);
}

/// xNVSE's own plugin with the copy's two files and string records of A.esp (id, text), as
/// the co-save of `Save 7`.
fn put_strings(strings: &[(u32, &[u8])]) {
    let mut mods = ChunkWriter::default();
    mods.u8(2);
    for n in ["FalloutNV.esm", "A.esp"] {
        mods.u16(n.len() as u16);
        mods.bytes(n.as_bytes());
    }
    let chunk = |t: &[u8; 4], data: Vec<u8>| Chunk {
        kind: tag(t),
        version: 0,
        data,
    };
    let mut chunks = vec![chunk(b"MODS", mods.data), chunk(b"STVS", Vec::new())];
    for (id, text) in strings {
        let mut w = ChunkWriter::default();
        w.u8(1);
        w.u32(*id);
        w.u16(text.len() as u16);
        w.bytes(text);
        chunks.push(chunk(b"STVR", w.data));
    }
    chunks.push(chunk(b"STVE", Vec::new()));
    let bytes = cosave::write(&[Plugin {
        opcode_base: 0x1400,
        chunks,
    }]);
    FILES.with(|f| f.borrow_mut().insert(CO_SAVE.as_bytes().to_vec(), bytes));
}

/// Review item 5 of #328: a file cut in the middle of an array's elements keeps the array with
/// the elements read (xNVSE adds it before reading them), the elements sorted as the map keeps
/// them; nothing after it is read.
#[test]
fn a_truncated_co_save_keeps_the_partial_array() {
    let mut e = engine();
    set_files(&mut e, &["FalloutNV.esm", "A.esp"]);
    let full = put_arrays(
        vec![map_record(3, 1, 3, &[(9.0, 1.0), (2.0, 2.0), (5.0, 3.0)])],
        &[],
    );
    // Cut 10 bytes into the third element (a key, a type byte and a value: 17 bytes), which
    // ends where the `ARVE` chunk header (12 bytes) begins.
    let cut = full.len() - 12 - 17 + 10;
    FILES.with(|f| {
        f.borrow_mut()
            .insert(CO_SAVE.as_bytes().to_vec(), full[..cut].to_vec())
    });
    pre_load(&mut e, NAME);
    let a = vars::array(&e, 3).expect("kept");
    assert_eq!(a.keys(), [Key::Number(2.0), Key::Number(9.0)]);
}

/// Review item 3 of #328 (the fixes preset's row): another plugin's section is kept from the
/// co-save loaded and written back after xNVSE's own, byte for byte; without the row it goes,
/// as with xNVSE.
#[test]
fn other_plugins_sections_go_back_with_the_row() {
    let jip = cosave::Kept {
        opcode_base: 0x2200,
        chunks: 1,
        data: [
            &u32::from_le_bytes(*b"FGPJ").to_le_bytes()[..],
            &1u32.to_le_bytes(),
            &8u32.to_le_bytes(),
            &[0, 0, 0, 0, 0, 0, 0x80, 0x3f],
        ]
        .concat(),
    };
    // (row, today's files after the load): the files the co-save lists, or another order.
    for (row, moved) in [(true, false), (false, false), (true, true)] {
        let mut e = engine();
        set_files(&mut e, &["FalloutNV.esm", "A.esp"]);
        set_keep_other_plugins(&mut e, row);
        put_arrays(
            vec![map_record(2, 1, 1, &[(0.0, 4.0)])],
            std::slice::from_ref(&jip),
        );
        pre_load(&mut e, NAME);
        // The array is held by a variable of A.esp, as in the game.
        let mut held = 0;
        vars::add_reference(&mut e, &mut held, 2, 1);
        if moved {
            set_files(&mut e, &["FalloutNV.esm", "B.esp", "A.esp"]);
        }
        save_game(&mut e, NAME);
        let bytes = file(CO_SAVE).unwrap();
        let mut r = Reader::open(&bytes).unwrap();
        let mut bases = Vec::new();
        while let Some(p) = r.next_plugin().unwrap() {
            bases.push(p.opcode_base);
            let section = r.take_plugin(p.length).unwrap();
            if p.opcode_base == 0x2200 {
                assert_eq!(section, jip.data);
            }
            r.end_plugin();
        }
        // Under another file list the section's mod indexes would name other files: dropped.
        let want: &[u32] = if row && !moved {
            &[0x1400, 0x2200]
        } else {
            &[0x1400]
        };
        assert_eq!(bases, want, "row {row}, files moved {moved}");
    }
}

/// Every save writes its co-save, as xNVSE does: an empty one is xNVSE's plugin with the file
/// list and the empty array and string sections, written over one left from an earlier save.
#[test]
fn every_save_writes_a_co_save() {
    let mut e = engine();
    set_files(&mut e, &["FalloutNV.esm"]);
    FILES.with(|f| {
        f.borrow_mut()
            .insert(CO_SAVE.as_bytes().to_vec(), b"stale".to_vec())
    });
    save_game(&mut e, NAME);
    let bytes = file(CO_SAVE).unwrap();
    let mut r = Reader::open(&bytes).unwrap();
    assert_eq!(r.next_plugin().unwrap().unwrap().opcode_base, 0x1400);
    let mut kinds = Vec::new();
    while let Some((k, _, _)) = r.next_record().unwrap() {
        kinds.push(k.to_be_bytes());
    }
    assert_eq!(kinds, [*b"MODS", *b"ARVS", *b"ARVE", *b"STVS", *b"STVE"]);
    assert!(r.next_plugin().unwrap().is_none());
}
