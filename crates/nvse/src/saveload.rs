//! The co-save and what xNVSE does around saving, loading and starting a game (ADR-0016 §10;
//! build step I-10): translated from `Hooks_SaveLoad.cpp`, `Serialization.cpp` and
//! `Core_Serialization.cpp` (xNVSE), at the game's own sites. The file format is
//! [`crate::cosave`].
//!
//! - **Saving** (the seam at `0x00847C45` in `BGSSaveLoadGame::SaveGame`, before the
//!   "Finished saving" note, where xNVSE's `SaveGameHook` runs): a co-save is written next to the
//!   `.fos` (`CREATE_ALWAYS`): xNVSE's own plugin with the loaded files (`MODS`), the array
//!   variables (`ARVS`, an `ARVR` per array with references, `ARVE`) and the string variables
//!   (`STVS`, `STVR`s, `STVE`), each map cleaned of its temporaries first and listed in its hash
//!   map's order ([`vars::HashOrder`]); with the fixes preset's row, other plugins' sections
//!   kept from the loaded game after it, when today's files are the ones they were saved with.
//!   One is written on every save, an empty one too, as xNVSE does whenever it is installed: a
//!   save without one loads as a new game for the extender without the reset.
//! - **Reading** is xNVSE's, with one guard: a count of references or elements larger than the
//!   bytes left in its chunk is a corrupt file and stops the read as a read past the end does
//!   (xNVSE would loop or allocate on it); the free ids below a high array id are one run.
//! - **Before a game is loaded** (`0x00847FD9`, after the call to `00852b60`, xNVSE's
//!   `PreLoadGameHook`): the co-save is read for each plugin's pre-load callback. xNVSE's own
//!   (`Core_PreLoadCallback`) forgets the locals recorded per event list, drops the delayed
//!   calls, resets the string and array variables, then reads the file list and the variables:
//!   ids as saved, owners and form ids mapped from the saved file order to the current one
//!   (a file no longer loaded: a string goes, an array goes to the first loaded file that refers
//!   to it, or goes). With no co-save, or one that is not a co-save, nothing happens, as in
//!   xNVSE: the variables of the game before stay.
//! - **After the `.fos`** (`0x00848C3C`, xNVSE's `LoadGameHook`): the load callbacks; xNVSE's
//!   own reads nothing there (its chunks were read before). With no co-save it is a new game.
//! - **A new game** (`0x007D33CE` in the new-game confirmation, and `0x0093DB69` in
//!   `PlayerCharacter::CenterOnCell` when a `coc` from the main menu starts one: in the start
//!   menu (`0070edf0`) and not starting one the usual way (`0x011D8907`)): `NewGameHook`: the
//!   delayed calls dropped, then `Core_NewGameCallback`: the temporaries cleaned.
//! - **A save deleted or renamed** (`0x00850398` before `DeleteFileA` in `00850360`,
//!   `0x0085762B` before the backup `rename` in `00857370`): its co-save goes with it.
//!
//! Files go through the game's own imports (`CreateFileA`, `ReadFile`, `WriteFile`,
//! `CloseHandle`, `DeleteFileA`) and its C runtime's `rename`, as the `.fos` does, so the platform
//! layer keeps them where it keeps the save. The messages of these moments go to the events
//! ([`crate::events`]): `SaveGame`, `PreLoadGame`, `LoadGame`, `PostLoadGame` (the seam at
//! `0x00848C91`, the load's end), `NewGame`, `DeleteGame(Name)`, `Rename(New)Game(Name)`. Not here
//! yet: the messages to plugins and JIP LN NVSE's chunks (I-13).

use crate::cosave::{self, tag, Chunk, ChunkWriter, OutOfRange, Plugin, Reader};
use crate::vars::{self, ArrayVar, Container, Data, Element, StringVar};
use crate::{events, tick, Nvse};
use engine_core::extend::{Owner, SiteSlot};
use engine_core::prelude::*;

/// `BGSSaveLoadGame::SaveGame`'s `push` of the "Finished saving" note (`kSaveGamePatchAddr`).
pub const SAVE: SiteSlot = SiteSlot::new(Owner::Nvse, 0x0084_7c45);
/// `BGSSaveLoadGame::LoadGame`'s call to `00852b60` (`kPreLoadGamePatchAddr`).
pub const PRE_LOAD: SiteSlot = SiteSlot::new(Owner::Nvse, 0x0084_7fd9);
/// `LoadGame`'s `push` of the "Finished loading" note (`kLoadGamePatchAddr`).
pub const LOAD: SiteSlot = SiteSlot::new(Owner::Nvse, 0x0084_8c3c);
/// The end of `BGSSaveLoadGame::LoadGame` (its epilogue, xNVSE's `FinishLoadGameHook`).
pub const POST_LOAD: SiteSlot = SiteSlot::new(Owner::Nvse, 0x0084_8c91);
/// The new-game confirmation's call to the empty `0040fbe0` (`kNewGamePatchAddr`).
pub const NEW_GAME: SiteSlot = SiteSlot::new(Owner::Nvse, 0x007d_33ce);
/// `CenterOnCell`'s call to the empty `0040fbe0` (`NewGameWithNoSaveLoaded`).
pub const CENTER_ON_CELL: SiteSlot = SiteSlot::new(Owner::Nvse, 0x0093_db69);
/// `00850360`'s `DeleteFileA` call (`kDeleteGamePatchAddr`) and `00857370`'s backup `rename`
/// (`kRenameGamePatchAddr`).
pub const DELETE: SiteSlot = SiteSlot::new(Owner::Nvse, 0x0085_0398);
pub const RENAME: SiteSlot = SiteSlot::new(Owner::Nvse, 0x0085_762b);

/// `StartMenu`'s "in the start menu" test (`cdecl`) and the "starting a new game" flag
/// (`kIsStartingNewGameAddr`).
const IS_IN_START_MENU: u32 = 0x0070_edf0;
const IS_STARTING_NEW_GAME: u32 = 0x011d_8907;
/// `g_saveGameManager` and `SaveGameManager::ConstructSavegamePath` (`thiscall`, a buffer of
/// `MAX_PATH`).
const SAVE_GAME_MANAGER: u32 = 0x011d_e134;
const CONSTRUCT_SAVEGAME_PATH: u32 = 0x0084_ff30;
/// KERNEL32's import slots the game's own file code calls, and the C runtime's `rename`.
const CREATE_FILE_A: u32 = 0x00fd_f088;
const READ_FILE: u32 = 0x00fd_f080;
const WRITE_FILE: u32 = 0x00fd_f0dc;
const CLOSE_HANDLE: u32 = 0x00fd_f07c;
const DELETE_FILE_A: u32 = 0x00fd_f0d4;
const CRT_RENAME: u32 = 0x00ec_862c;
const GENERIC_READ: u32 = 0x8000_0000;
const GENERIC_WRITE: u32 = 0x4000_0000;
const CREATE_ALWAYS: u32 = 2;
const OPEN_EXISTING: u32 = 3;
const FILE_ATTRIBUTE_NORMAL: u32 = 0x80;
const INVALID_HANDLE_VALUE: u32 = 0xffff_ffff;
/// `DataHandler`: its singleton, the list of files (`modList.modInfoList`, `+0x210`), the
/// loaded count (`+0x218`) and the loaded files in mod index order (`loadedMods`, `+0x21C`); a
/// file's name at `+0x20`.
const DATA_HANDLER: u32 = 0x011c_3f2c;
const MOD_INFO_LIST: u32 = 0x210;
const LOADED_COUNT: u32 = 0x218;
const LOADED_MODS: u32 = 0x21c;
const MOD_NAME: u32 = 0x20;
/// `ArrayVarMap::kVersion` and the string map's records' version.
const ARRAY_VERSION: u32 = 2;
const STRING_VERSION: u32 = 0;

/// The co-save's state between moments.
#[derive(Default)]
pub struct State {
    /// The save's file list as the co-save had it, mapped to today's mod indexes
    /// (`s_preloadModRefIDs`, `s_numPreloadMods`).
    pub preload_mods: Vec<u8>,
    /// The fixes preset's row ([`set_keep_other_plugins`]): other plugins' sections are kept
    /// and written back.
    pub keep_other_plugins: bool,
    /// The sections of plugins that are not built in, as the last co-save read held them.
    pub kept: Vec<cosave::Kept>,
    /// That co-save's file list (`MODS`, names as saved), which the kept sections' mod indexes
    /// refer to.
    pub kept_mods: Vec<Vec<u8>>,
}

/// The fixes preset's row (docs/FIXES.md, "The script extender"): a co-save's sections of
/// plugins that are not built in (JIP LN NVSE's and others', until they are) are kept byte for
/// byte and written back on each save, so a save brought from a real modded installation
/// keeps them; xNVSE writes only the plugins loaded. Off (vanilla), they are dropped as xNVSE
/// drops them.
pub fn set_keep_other_plugins(e: &mut Engine, on: bool) {
    e.ext::<Nvse>().saveload.keep_other_plugins = on;
}

// ---- names and files ----

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Serialization.cpp: ConvertSaveFileName.
/// The co-save's name for a save: the `.bak` suffixes set aside (from the first `.bak`), the
/// last extension made `.nvse`, the suffixes back; a name without a `\` is put in the save
/// folder (`save_dir`).
pub fn co_save_name(name: &[u8], save_dir: impl FnOnce() -> Vec<u8>) -> Vec<u8> {
    let find = |hay: &[u8], needle: &[u8]| hay.windows(needle.len()).position(|w| w == needle);
    let (base, bak) = match find(name, b".bak") {
        Some(at) => (&name[..at], &name[at..]),
        None => (name, &name[name.len()..]),
    };
    let mut out = match base.iter().rposition(|&b| b == b'.') {
        Some(at) => base[..at].to_vec(),
        None => base.to_vec(),
    };
    out.extend_from_slice(b".nvse");
    out.extend_from_slice(bak);
    if !out.contains(&b'\\') {
        let mut full = save_dir();
        full.extend(out);
        out = full;
    }
    out
}

/// `GetSavegamePath`: the save folder, from the game.
fn save_dir(e: &mut Engine) -> Vec<u8> {
    let manager = e.mem.u32(SAVE_GAME_MANAGER);
    e.with_stack(0x104, |e, buf| {
        e.mem.set_u8(buf.addr(), 0);
        e.call(CONSTRUCT_SAVEGAME_PATH, &args![manager, buf.addr()]);
        crate::table::guest_text(e, buf.addr())
    })
}

fn co_save_path(e: &mut Engine, name: u32) -> Vec<u8> {
    let name = crate::table::guest_text(e, name);
    co_save_name(&name, || save_dir(e))
}

/// Runs `f` with `text` as a NUL-terminated string on the guest stack.
fn with_guest_str<T>(e: &mut Engine, text: &[u8], f: impl FnOnce(&mut Engine, u32) -> T) -> T {
    let size = (text.len() as u32 + 1).next_multiple_of(4);
    e.with_stack(size, |e, at| {
        e.mem.write(at.addr(), text);
        e.mem.set_u8(at.addr() + text.len() as u32, 0);
        f(e, at.addr())
    })
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Serialization.cpp: SerializationTask::Load.
/// The whole file, or `None` when it cannot be opened or is empty.
fn read_file(e: &mut Engine, path: &[u8]) -> Option<Vec<u8>> {
    let handle = with_guest_str(e, path, |e, p| {
        e.call(
            CREATE_FILE_A,
            &args![
                p,
                GENERIC_READ,
                1u32,
                0u32,
                OPEN_EXISTING,
                FILE_ATTRIBUTE_NORMAL,
                0u32
            ],
        )
        .u32()
    });
    if handle == INVALID_HANDLE_VALUE {
        return None;
    }
    let mut out = Vec::new();
    e.with_stack(0x1004, |e, frame| {
        let (buf, read) = (frame.addr(), frame.addr() + 0x1000);
        loop {
            e.mem.set_u32(read, 0);
            let ok = e
                .call(READ_FILE, &args![handle, buf, 0x1000u32, read, 0u32])
                .u32();
            let n = e.mem.u32(read);
            if ok == 0 || n == 0 {
                break;
            }
            out.extend(e.mem.bytes(buf, n));
        }
    });
    e.call(CLOSE_HANDLE, &args![handle]);
    (!out.is_empty()).then_some(out)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Serialization.cpp: SerializationTask::Save.
fn write_file(e: &mut Engine, path: &[u8], bytes: &[u8]) -> bool {
    let handle = with_guest_str(e, path, |e, p| {
        e.call(
            CREATE_FILE_A,
            &args![
                p,
                GENERIC_WRITE,
                0u32,
                0u32,
                CREATE_ALWAYS,
                FILE_ATTRIBUTE_NORMAL,
                0u32
            ],
        )
        .u32()
    });
    if handle == INVALID_HANDLE_VALUE {
        return false;
    }
    e.with_stack(0x1004, |e, frame| {
        let (buf, written) = (frame.addr(), frame.addr() + 0x1000);
        for piece in bytes.chunks(0x1000) {
            e.mem.write(buf, piece);
            e.call(
                WRITE_FILE,
                &args![handle, buf, piece.len() as u32, written, 0u32],
            );
        }
    });
    e.call(CLOSE_HANDLE, &args![handle]);
    true
}

fn delete_file(e: &mut Engine, path: &[u8]) {
    with_guest_str(e, path, |e, p| e.call(DELETE_FILE_A, &args![p]));
}

fn rename_file(e: &mut Engine, from: &[u8], to: &[u8]) {
    with_guest_str(e, from, |e, f| {
        with_guest_str(e, to, |e, t| e.call(CRT_RENAME, &args![f, t]))
    });
}

// ---- the file list ----

/// The loaded files' names in mod index order (`modList.loadedMods`, `loadedModCount`).
fn loaded_mods(e: &Engine) -> Vec<Vec<u8>> {
    let handler = e.mem.u32(DATA_HANDLER);
    if handler == 0 {
        return Vec::new();
    }
    let count = e.mem.u8(handler + LOADED_COUNT);
    (0..u32::from(count))
        .map(|i| {
            let info = e.mem.u32(handler + LOADED_MODS + 4 * i);
            if info == 0 {
                Vec::new()
            } else {
                crate::table::guest_text(e, info + MOD_NAME)
            }
        })
        .collect()
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), GameData.cpp: DataHandler::GetModIndex.
/// The position of the file called `name` (without case) in the list of files, `0xFF` when it
/// is not there.
fn mod_index(e: &mut Engine, name: &[u8]) -> u8 {
    let handler = e.mem.u32(DATA_HANDLER);
    if handler == 0 {
        return 0xff;
    }
    let upper = vars::upper_table(e);
    let mut node = handler + MOD_INFO_LIST;
    let mut index: i32 = 0;
    while node != 0 {
        let info = e.mem.u32(node);
        if info != 0 {
            let file = crate::table::guest_text(e, info + MOD_NAME);
            if vars::str_compare(&upper, &file, name).is_eq() {
                return index as u8;
            }
        }
        index += 1;
        node = e.mem.u32(node + 4);
    }
    0xff
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Serialization.cpp: ResolveRefID.
/// A saved form id with today's mod index: ids made in the game pass as they are; one of a file
/// the save listed is mapped; `None` for a file not listed or no longer loaded.
pub fn resolve_ref_id(e: &Engine, ref_id: u32) -> Option<u32> {
    let modi = (ref_id >> 24) as u8;
    if modi == 0xff {
        return Some(ref_id);
    }
    let loaded = *e
        .ext_get::<Nvse>()?
        .saveload
        .preload_mods
        .get(usize::from(modi))?;
    if loaded == 0xff {
        return None;
    }
    Some((u32::from(loaded) << 24) | (ref_id & 0x00ff_ffff))
}

// ---- saving ----

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Core_Serialization.cpp:
// Core_SaveCallback, ArrayVar.cpp: ArrayVarMap::Save, StringVar.cpp: StringVarMap::Save.
/// xNVSE's own chunks.
pub fn core_chunks(e: &mut Engine) -> Vec<Chunk> {
    let mut chunks = Vec::new();
    let mods = loaded_mods(e);
    let mut w = ChunkWriter::default();
    w.u8(mods.len() as u8);
    for name in &mods {
        w.u16(name.len() as u16);
        w.bytes(&name[..usize::from(name.len() as u16)]);
    }
    chunks.push(Chunk {
        kind: tag(b"MODS"),
        version: 0,
        data: w.data,
    });
    // Arrays.
    vars::clean_arrays(e);
    chunks.push(Chunk {
        kind: tag(b"ARVS"),
        version: ARRAY_VERSION,
        data: Vec::new(),
    });
    let st = &e.ext::<Nvse>().vars;
    for id in st.array_order.keys() {
        if st.array_ids.temp.contains(&id) {
            continue;
        }
        let Some(a) = st.arrays.get(&id) else {
            continue;
        };
        if a.refs.is_empty() {
            continue;
        }
        let mut w = ChunkWriter::default();
        w.u8(a.owning_mod);
        w.u32(id);
        w.u8(a.key_type);
        w.u8(u8::from(a.packed));
        w.u32(a.refs.len() as u32);
        w.bytes(&a.refs);
        w.u32(a.len() as u32);
        let keys = a.keys();
        for (i, key) in keys.iter().enumerate() {
            match key {
                vars::Key::Str(s) => c_string(&mut w, s),
                vars::Key::Number(n) if !a.packed => w.f64(*n),
                vars::Key::Number(_) => {}
            }
            let el = element_at(a, i);
            w.u8(el.data.type_code());
            match &el.data {
                Data::Number(n) => w.f64(*n),
                Data::Str(s) => c_string(&mut w, s),
                Data::Array(v) | Data::Form(v) => w.u32(*v),
                Data::Invalid => {}
            }
        }
        chunks.push(Chunk {
            kind: tag(b"ARVR"),
            version: ARRAY_VERSION,
            data: w.data,
        });
    }
    chunks.push(Chunk {
        kind: tag(b"ARVE"),
        version: ARRAY_VERSION,
        data: Vec::new(),
    });
    // Strings.
    vars::clean_strings(e);
    chunks.push(Chunk {
        kind: tag(b"STVS"),
        version: STRING_VERSION,
        data: Vec::new(),
    });
    let st = &e.ext::<Nvse>().vars;
    for id in st.string_order.keys() {
        if st.string_ids.temp.contains(&id) {
            continue;
        }
        let Some(s) = st.strings.get(&id) else {
            continue;
        };
        if s.owning_mod == 0xff {
            continue;
        }
        let mut w = ChunkWriter::default();
        w.u8(s.owning_mod);
        w.u32(id);
        let len = s.data.len() as u16;
        w.u16(len);
        w.bytes(&s.data[..usize::from(len)]);
        chunks.push(Chunk {
            kind: tag(b"STVR"),
            version: STRING_VERSION,
            data: w.data,
        });
    }
    chunks.push(Chunk {
        kind: tag(b"STVE"),
        version: STRING_VERSION,
        data: Vec::new(),
    });
    chunks
}

/// A key or element string as xNVSE writes it: `StrLen` (to the first NUL) as a `u16`, then
/// that many bytes.
fn c_string(w: &mut ChunkWriter, s: &[u8]) {
    let end = s.iter().position(|&b| b == 0).unwrap_or(s.len());
    let len = end as u16;
    w.u16(len);
    w.bytes(&s[..usize::from(len)]);
}

fn element_at(a: &ArrayVar, i: usize) -> &Element {
    match &a.elements {
        Container::Array(v) => &v[i],
        Container::NumMap(v) => &v[i].1,
        Container::StrMap(v) => &v[i].1,
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Serialization.cpp: HandleSaveGame.
/// The co-save of the save called `name` (a guest string), written whole.
pub fn save_game(e: &mut Engine, name: u32) {
    events::message(e, events::id::SAVE_GAME, name);
    let path = co_save_path(e, name);
    let chunks = core_chunks(e);
    let kept = kept_to_write(e);
    let bytes = cosave::write_with(
        &[Plugin {
            opcode_base: cosave::CORE_OPCODE_BASE,
            chunks,
        }],
        &kept,
    );
    write_file(e, &path, &bytes);
}

/// The other plugins' sections to write back (the fixes preset's row): the ones kept, only when
/// today's files are the ones the co-save they came from listed, in the same order. Their data
/// holds mod indexes of that list (JIP LN NVSE maps them through the `MODS` it was saved with),
/// so under another list they would point at other files: then they go, as with xNVSE.
fn kept_to_write(e: &mut Engine) -> Vec<cosave::Kept> {
    let st = &e.ext::<Nvse>().saveload;
    if !st.keep_other_plugins || st.kept.is_empty() {
        return Vec::new();
    }
    let (kept, kept_mods) = (st.kept.clone(), st.kept_mods.clone());
    if kept_mods != loaded_mods(e) {
        return Vec::new();
    }
    kept
}

// ---- loading ----

/// Which callback a load runs (`PluginCallbacks::preLoad` or `::load`).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Pass {
    PreLoad,
    Load,
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Serialization.cpp: HandleLoadGame.
/// Reads the co-save of the save `name` for `pass`; whether there was one to read.
fn handle_load(e: &mut Engine, name: u32, pass: Pass) -> bool {
    let path = co_save_path(e, name);
    let Some(bytes) = read_file(e, &path) else {
        if pass == Pass::Load {
            handle_new_game(e);
        }
        return false;
    };
    let mut r = match Reader::open(&bytes) {
        Ok(r) => r,
        // Shorter than the header: its read throws, the `catch` of a load is a new game.
        Err(cosave::Refused::Short) => {
            if pass == Pass::Load {
                handle_new_game(e);
            }
            return true;
        }
        Err(_) => return true,
    };
    if read_plugins(e, &mut r, pass).is_err() && pass == Pass::Load {
        // The `catch` of an exception during the load: a new game.
        handle_new_game(e);
    }
    true
}

fn read_plugins(e: &mut Engine, r: &mut Reader, pass: Pass) -> Result<(), OutOfRange> {
    let mut core_had_data = false;
    while let Some(plugin) = r.next_plugin()? {
        let start = r.offset();
        if plugin.opcode_base == cosave::CORE_OPCODE_BASE {
            core_had_data = true;
            core_callback(e, r, pass)?;
        } else if pass == Pass::PreLoad && e.ext::<Nvse>().saveload.keep_other_plugins {
            // A plugin that is not built in (JIP LN NVSE's chunks come with I-13): kept for
            // the next save (the fixes preset's row).
            let data = r.take_plugin(plugin.length)?;
            e.ext::<Nvse>().saveload.kept.push(cosave::Kept {
                opcode_base: plugin.opcode_base,
                chunks: plugin.chunks,
                data,
            });
        } else {
            // A plugin that is not loaded: xNVSE skips its data.
            r.skip(plugin.length)?;
        }
        let expected = start + plugin.length as usize;
        if r.offset() != expected {
            r.set_offset(expected);
        }
        r.end_plugin();
    }
    if !core_had_data {
        r.no_records();
        core_callback(e, r, pass)?;
    }
    Ok(())
}

fn core_callback(e: &mut Engine, r: &mut Reader, pass: Pass) -> Result<(), OutOfRange> {
    match pass {
        Pass::PreLoad => core_pre_load(e, r),
        // `Core_LoadCallback`: every chunk of its own was read before the `.fos`.
        Pass::Load => {
            while r.next_record()?.is_some() {}
            Ok(())
        }
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Core_Serialization.cpp:
// Core_PreLoadCallback.
fn core_pre_load(e: &mut Engine, r: &mut Reader) -> Result<(), OutOfRange> {
    e.ext::<Nvse>().saveload.preload_mods.clear();
    vars::forget_collected(e);
    tick::clear_delayed_calls(e);
    events::clear_flush_on_load(e);
    vars::reset(e);
    while let Some((kind, _, _)) = r.next_record()? {
        match kind {
            k if k == tag(b"MODS") => load_mod_list(e, r)?,
            k if k == tag(b"STVS") => load_strings(e, r)?,
            k if k == tag(b"ARVS") => load_arrays(e, r)?,
            _ => {}
        }
    }
    Ok(())
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Core_Serialization.cpp:
// ReadModListFromCoSave.
fn load_mod_list(e: &mut Engine, r: &mut Reader) -> Result<(), OutOfRange> {
    let count = r.read_data(1)?.first().copied().unwrap_or(0);
    let mut name_len: u16 = 0;
    let mut mods = Vec::with_capacity(usize::from(count));
    let mut names = Vec::with_capacity(usize::from(count));
    for _ in 0..count {
        let raw = r.read_data(2)?;
        let mut bytes = name_len.to_le_bytes();
        bytes[..raw.len()].copy_from_slice(&raw);
        name_len = u16::from_le_bytes(bytes);
        let mut name = r.read_data(u32::from(name_len))?;
        if let Some(end) = name.iter().position(|&b| b == 0) {
            name.truncate(end);
        }
        mods.push(mod_index(e, &name));
        names.push(name);
    }
    let st = &mut e.ext::<Nvse>().saveload;
    st.preload_mods = mods;
    st.kept_mods = names;
    Ok(())
}

/// A C string read back: up to its first NUL.
fn c_text(mut b: Vec<u8>) -> Vec<u8> {
    if let Some(end) = b.iter().position(|&c| c == 0) {
        b.truncate(end);
    }
    b
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), StringVar.cpp: StringVarMap::Load.
fn load_strings(e: &mut Engine, r: &mut Reader) -> Result<(), OutOfRange> {
    vars::clean_strings(e);
    while let Some((kind, _, _)) = r.next_record()? {
        if kind == tag(b"STVE") {
            break;
        }
        if kind != tag(b"STVR") {
            continue;
        }
        let saved = r.read8()?;
        let Some(resolved) = resolve_ref_id(e, u32::from(saved) << 24) else {
            continue;
        };
        if saved == 0xff {
            continue;
        }
        let modi = (resolved >> 24) as u8;
        let id = r.read32()?;
        if id == u32::MAX {
            // No id is past it: a corrupt file, the read stops.
            return Err(OutOfRange);
        }
        let len = r.read16()?;
        let text = c_text(r.read_data(u32::from(len))?);
        let st = &mut e.ext::<Nvse>().vars;
        st.string_ids.used.insert(id);
        if !st.strings.contains_key(&id) {
            st.string_order.insert(id);
            st.strings.insert(
                id,
                StringVar {
                    data: text,
                    owning_mod: modi,
                    function_result_cache: false,
                },
            );
        }
    }
    Ok(())
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), ArrayVar.cpp: ArrayVarMap::Load,
// ArrayVarMap::Add.
fn load_arrays(e: &mut Engine, r: &mut Reader) -> Result<(), OutOfRange> {
    vars::clean_arrays(e);
    let upper = vars::upper_table(e);
    let mut last_index_read: u32 = 0;
    let mut num_key = 0.0f64;
    while let Some((kind, version, _)) = r.next_record()? {
        if kind == tag(b"ARVE") {
            break;
        }
        if kind != tag(b"ARVR") {
            continue;
        }
        let saved = r.read8()?;
        let mut modi = match resolve_ref_id(e, u32::from(saved) << 24) {
            Some(id) if saved != 0xff => (id >> 24) as u8,
            // The owning file is gone: the first loaded file that refers to it takes it.
            _ => 0,
        };
        let id = r.read32()?;
        if id == u32::MAX {
            // No id is past it: a corrupt file, the read stops.
            return Err(OutOfRange);
        }
        let key_type = r.read8()?;
        let packed = r.read8()? != 0;
        let mut refs = Vec::new();
        if version >= 1 {
            let count = r.read32()?;
            // A reference takes a byte: more than the chunk has left is a corrupt file.
            if count > r.left() {
                return Err(OutOfRange);
            }
            for _ in 0..count {
                let m = r.read8()?;
                if m == 0xff {
                    continue;
                }
                let resolved = resolve_ref_id(e, u32::from(m) << 24);
                if modi == 0 {
                    if let Some(rid) = resolved {
                        modi = (rid >> 24) as u8;
                    }
                }
                if let Some(rid) = resolved {
                    refs.push((rid >> 24) as u8);
                }
            }
        } else if modi != 0 {
            refs.push(modi);
        }
        if modi == 0 {
            continue;
        }
        // The ids not read below this one are free (`SetIDAvailable`, one at a time in xNVSE;
        // here as one run, so a corrupt id of any size costs nothing).
        last_index_read = last_index_read.wrapping_add(1);
        if last_index_read < id {
            let first = last_index_read.max(1);
            e.ext::<Nvse>()
                .vars
                .array_ids
                .available
                .insert_run(first, id - 1);
            last_index_read = id;
        }
        let elements = if key_type == vars::KEY_STRING {
            Container::StrMap(Vec::new())
        } else if packed {
            Container::Array(Vec::new())
        } else {
            Container::NumMap(Vec::new())
        };
        let mut arr = ArrayVar {
            id,
            owning_mod: modi,
            key_type,
            packed,
            refs,
            elements,
        };
        // `Add` comes before the elements are read: an array whose elements stop short (a
        // truncated or corrupt file) is kept with what was read.
        let read = load_elements(e, r, &mut arr, version, &mut num_key, &upper);
        let st = &mut e.ext::<Nvse>().vars;
        st.array_ids.used.insert(id);
        st.array_ids.available.remove(id);
        if arr.refs.is_empty() {
            st.array_ids.temp.insert(id);
        }
        if !st.arrays.contains_key(&id) {
            st.array_order.insert(id);
        }
        st.arrays.insert(id, arr);
        read?;
    }
    Ok(())
}

/// An array's elements from its record (the rest of `ArrayVarMap::Load`'s loop body).
fn load_elements(
    e: &mut Engine,
    r: &mut Reader,
    arr: &mut ArrayVar,
    version: u32,
    num_key: &mut f64,
    upper: &[u8; 256],
) -> Result<(), OutOfRange> {
    let (id, key_type, packed) = (arr.id, arr.key_type, arr.packed);
    let count = r.read32()?;
    // An element takes a byte at least: more than the chunk has left is a corrupt file.
    if count > r.left() {
        return Err(OutOfRange);
    }
    if let Container::Array(v) = &mut arr.elements {
        v.resize(
            count as usize,
            Element {
                data: Data::Invalid,
                owning_array: id,
            },
        );
    }
    for i in 0..count as usize {
        let mut str_key = Vec::new();
        if key_type == vars::KEY_STRING {
            let len = r.read16()?;
            str_key = c_text(r.read_data(u32::from(len))?);
        } else if !packed || version < 2 {
            if let Some(v) = r.read64()? {
                *num_key = v;
            }
        }
        let kind = r.read8()?;
        let data = match kind {
            1 => Data::Number(r.read64()?.unwrap_or(0.0)),
            3 => {
                let len = r.read16()?;
                if len != 0 {
                    Data::Str(c_text(r.read_data(u32::from(len))?))
                } else {
                    Data::Str(Vec::new())
                }
            }
            4 => Data::Array(r.read32()?),
            2 => {
                let form = r.read32()?;
                Data::Form(resolve_ref_id(e, form).unwrap_or(0))
            }
            _ => Data::Invalid,
        };
        let el = Element {
            data,
            owning_array: id,
        };
        // The maps stay sorted: found by binary search, as `vars::set_element` inserts.
        match &mut arr.elements {
            Container::Array(v) => v[i] = el,
            Container::NumMap(v) => {
                let key = *num_key;
                let p = v.partition_point(|(k, _)| *k < key);
                if p < v.len() && v[p].0 == key {
                    v[p].1 = el;
                } else {
                    v.insert(p, (key, el));
                }
            }
            Container::StrMap(v) => {
                let p = v.partition_point(|(k, _)| vars::str_compare(upper, k, &str_key).is_lt());
                if p < v.len() && vars::str_compare(upper, &v[p].0, &str_key).is_eq() {
                    v[p].1 = el;
                } else {
                    v.insert(p, (str_key, el));
                }
            }
        }
    }
    Ok(())
}

// ---- the moments ----

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Core_Serialization.cpp:
// Core_NewGameCallback.
fn new_game_callback(e: &mut Engine) {
    vars::clean_arrays(e);
    vars::clean_strings(e);
    // A new game holds no other plugin's data.
    let st = &mut e.ext::<Nvse>().saveload;
    st.kept.clear();
    st.kept_mods.clear();
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Serialization.cpp: HandleNewGame.
/// The `NewGame` message, then the new-game callbacks.
fn handle_new_game(e: &mut Engine) {
    events::message(e, events::id::NEW_GAME, 0);
    new_game_callback(e);
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_SaveLoad.cpp: NewGameHook.
/// A new game: the delayed calls dropped, the handlers that go on a load, then
/// `HandleNewGame`.
pub fn new_game(e: &mut Engine) {
    tick::clear_delayed_calls(e);
    events::clear_flush_on_load(e);
    handle_new_game(e);
}

/// Before the `.fos` (`Serialization::HandlePreLoadGame`).
pub fn pre_load(e: &mut Engine, name: u32) {
    events::message(e, events::id::PRE_LOAD_GAME, name);
    // The other plugins' sections kept are the loaded game's, or none.
    let st = &mut e.ext::<Nvse>().saveload;
    st.kept.clear();
    st.kept_mods.clear();
    handle_load(e, name, Pass::PreLoad);
}

/// After the `.fos` (`Serialization::HandleLoadGame`).
pub fn load(e: &mut Engine, name: u32) {
    events::message(e, events::id::LOAD_GAME, name);
    handle_load(e, name, Pass::Load);
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Serialization.cpp: GetSaveName.
/// A save path's file name with its extension (`_splitpath_s`'s name and extension).
pub fn save_name(path: &[u8]) -> Vec<u8> {
    let start = path
        .iter()
        .rposition(|&c| c == b'\\' || c == b'/' || c == b':')
        .map_or(0, |i| i + 1);
    path[start..].to_vec()
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Serialization.cpp: HandleDeleteGame.
fn delete_game(e: &mut Engine, path: u32) {
    let co = co_save_path(e, path);
    let name = save_name(&crate::table::guest_text(e, path));
    events::message_text(e, events::id::DELETE_GAME, &co);
    events::message_text(e, events::id::DELETE_GAME_NAME, &name);
    delete_file(e, &co);
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Serialization.cpp: HandleRenameGame.
fn rename_game(e: &mut Engine, old: u32, new: u32) {
    let (from, to) = (co_save_path(e, old), co_save_path(e, new));
    // xNVSE sends the paths again where the names would be.
    events::message_text(e, events::id::RENAME_GAME, &from);
    events::message_text(e, events::id::RENAME_GAME_NAME, &from);
    events::message_text(e, events::id::RENAME_NEW_GAME, &to);
    events::message_text(e, events::id::RENAME_NEW_GAME_NAME, &to);
    delete_file(e, &to);
    rename_file(e, &from, &to);
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_SaveLoad.cpp: FinishLoadGameHook,
// Serialization.cpp: HandlePostLoadGame.
/// The load's end: the `PostLoadGame` message with whether it succeeded.
fn post_load_site(e: &mut Engine, a: &[u32]) -> Option<u32> {
    events::message(e, events::id::POST_LOAD_GAME, a[0] & 0xff);
    None
}

fn save_site(e: &mut Engine, a: &[u32]) -> Option<u32> {
    save_game(e, a[0]);
    None
}

fn pre_load_site(e: &mut Engine, a: &[u32]) -> Option<u32> {
    pre_load(e, a[0]);
    None
}

fn load_site(e: &mut Engine, a: &[u32]) -> Option<u32> {
    load(e, a[0]);
    None
}

fn new_game_site(e: &mut Engine, _: &[u32]) -> Option<u32> {
    new_game(e);
    None
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Hooks_SaveLoad.cpp:
// NewGameWithNoSaveLoaded::Hook.
fn center_on_cell_site(e: &mut Engine, _: &[u32]) -> Option<u32> {
    let in_start_menu = e.call(IS_IN_START_MENU, &[]).bool();
    if in_start_menu && e.mem.u8(IS_STARTING_NEW_GAME) == 0 {
        new_game(e);
    }
    None
}

fn delete_site(e: &mut Engine, a: &[u32]) -> Option<u32> {
    delete_game(e, a[0]);
    None
}

fn rename_site(e: &mut Engine, a: &[u32]) -> Option<u32> {
    rename_game(e, a[0], a[1]);
    None
}

/// Installs the seams' handlers.
pub fn install(e: &mut Engine) {
    e.on_site(SAVE, save_site);
    e.on_site(PRE_LOAD, pre_load_site);
    e.on_site(LOAD, load_site);
    e.on_site(POST_LOAD, post_load_site);
    e.on_site(NEW_GAME, new_game_site);
    e.on_site(CENTER_ON_CELL, center_on_cell_site);
    e.on_site(DELETE, delete_site);
    e.on_site(RENAME, rename_site);
}

#[cfg(test)]
#[path = "saveload_tests.rs"]
mod tests;
