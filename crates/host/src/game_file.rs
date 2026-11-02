//! A plugin file in the game's memory (`TESFile`, Xbox PDB), read by the
//! game's own code from the bytes the world's reader already has.
//!
//! The game reads a record with `TESFile::SetOffset` (`004723a0`: seek to
//! the record, read its header) and then the form's `Load`, which reads its
//! chunks (`GetTESChunk`, `GetChunkData`, `NextChunk`, all translated)
//! through the file's `BSFile`. `BSFile` is platform: its reads and seeks are
//! stand-ins over an in-memory copy of the parts of the file the host
//! supplies, at their real offsets (`engine::units::platform::open_memory_file`).
//!
//! [`GameFile::open`] makes the `TESFile` with the game's constructor
//! (`TESFile::TESFile`, `00470710`, given no name, so it opens nothing) and
//! then leaves it as `TESFile::OpenTES` (`00470c70`) leaves an open file:
//! its `BSFile` (`m_pFile`), its size (`m_filesize`) and its load-order
//! index (`cCompileIndex`, which `ReadFormHeader` puts in the form ids it
//! reads). Bridged, not run: `OpenTES` itself (it finds the file on disk,
//! constructs the `BSFile` and reads the file's own header, `TES4`, and
//! its masters). So a plugin with masters isn't supported yet: its form ids
//! need the master table `OpenTES` builds ([`GameFile::open`] refuses it).
//!
//! Compressed records need zlib (`TESFile::DecompressCurrentForm`), which
//! isn't translated: [`GameFile::supply`] refuses them.

use engine::abi::RetVal;
use engine::units::bssystem::bsfile;
use engine::units::platform::{self, is_memory_file, open_memory_file, supply_file_bytes};
use engine::Engine;
use engine::{Ptr, Ret};
use esm::{LoadedPlugin, RecordEntry};

/// `operator new` (`00401000`) and `TESFile::TESFile(directory, name, mode)`
/// (`00470710`).
const NEW: u32 = 0x0040_1000;
const TES_FILE_CONSTRUCT: u32 = 0x0047_0710;
/// `TESFile` (Xbox PDB): its size and the fields `OpenTES` leaves.
const TES_FILE_SIZE: u32 = 0x42c;
const M_PFILE: u32 = 0x10;
const M_FILESIZE: u32 = 0x260;
const C_COMPILE_INDEX: u32 = 0x40c;
/// `TESFile::OpenTES` (second overload, `00470ca0`), the header reader
/// (`00471400`), `TESFile::GenIndexTable` (`00471870`).
const OPEN_TES: u32 = 0x0047_0ca0;
const READ_HEADER: u32 = 0x0047_1400;
const GEN_INDEX_TABLE: u32 = 0x0047_1870;
/// `TESFile::SetOffset` (Xbox PDB, `004723a0`).
const SET_OFFSET: u32 = 0x0047_23a0;
/// A record header's size (`FORM`, 0x18 bytes).
const RECORD_HEADER_SIZE: usize = 0x18;
/// A record's compressed flag (`fn_00472100`).
const RECORD_COMPRESSED: u32 = 0x0004_0000;

/// An in-memory `BSFile` ([`open_memory_file`]), with `BSFile::ReadF` (`00b00650`) and the
/// seek (`00b00590`) routed: the platform stand-ins for the in-memory files
/// (`platform::is_memory_file`), the game's own (translated, `bssystem::bsfile`) for the
/// files the game opens itself (the archives).
fn open_bs_file(e: &mut Engine, name: &str, size: u32) -> u32 {
    let file = open_memory_file(e, name, size);
    e.register(0x00b0_0650, |e, a| {
        let (this, buffer, size) = (Ptr::new(a[0]), Ptr::new(a[1]), a[2]);
        let read = if is_memory_file(e, a[0]) {
            platform::bsfile_read_f(e, this, buffer, size)
        } else {
            bsfile::bs_file_read_f(e, Ptr::new(a[0]), buffer, size)
        };
        RetVal::into_ret(read)
    });
    e.register(0x00b0_0590, |e, a| {
        if is_memory_file(e, a[0]) {
            platform::bsfile_seek(e, Ptr::new(a[0]), a[1] as i32, a[2] as i32);
        } else {
            bsfile::fn_00b00590(e, Ptr::new(a[0]), a[1], a[2]);
        }
        Ret::default()
    });
    file
}

/// A plugin's `TESFile` in the game's memory.
#[derive(Debug, Clone, Copy)]
pub struct GameFile {
    /// The `TESFile`.
    pub file: u32,
    /// Its `BSFile` (an in-memory file).
    pub bs_file: u32,
}

impl GameFile {
    /// The `TESFile` of `plugin`, its compile index its load-order index;
    /// see the module for what is bridged.
    pub fn open(e: &mut Engine, plugin: &LoadedPlugin) -> Result<GameFile, String> {
        let index = plugin.load_index;
        let name = plugin.name.clone();
        if !plugin.plugin.header().masters.is_empty() {
            return Err(format!(
                "{name}: has masters, whose form ids need the master table TESFile::OpenTES \
                 builds (not run yet)"
            ));
        }
        let size = u32::try_from(plugin.plugin.file_size())
            .map_err(|_| format!("{name}: larger than 4 GB"))?;
        let block = e.call(NEW, &[TES_FILE_SIZE]).u32();
        let file = e.call(TES_FILE_CONSTRUCT, &[block, 0, 0, 0]).u32();
        let bs_file = open_bs_file(e, &name, size);
        e.mem.set_u32(file + M_PFILE, bs_file);
        e.mem.set_u32(file + M_FILESIZE, size);
        e.mem.set_u8(file + C_COMPILE_INDEX, index);
        Ok(GameFile { file, bs_file })
    }

    /// The `TESFile` of `plugin` opened and its header read by the game's
    /// own code, as `TESDataHandler` reads a plugin: the constructor
    /// (`00470710`), `TESFile::OpenTES` (`00470ca0`, with the plugin's
    /// directory and name) and the header reader (`00471400`: `HEDR`, `CNAM`,
    /// `SNAM`, each `MAST` into `listMasters` and `iMasterCount`, `DATA`, the
    /// file's flags). Platform, as for [`GameFile::open`]: the `BSFile`, an
    /// in-memory file put in `m_pFile` before `OpenTES`, so `OpenTES` skips
    /// finding the file (`FindFirstFileA`) and constructing the `BSFile`
    /// (`00b00260`) and goes on with the open file; and the compile index
    /// (`cCompileIndex`, which the data handler gives each file). Masters are
    /// resolved afterwards with `TESFile::GenIndexTable` against the files
    /// opened ([`gen_index_table`]).
    pub fn open_with_header(
        e: &mut Engine,
        plugin: &LoadedPlugin,
        directory: &str,
    ) -> Result<GameFile, String> {
        let name = plugin.name.clone();
        let size = u32::try_from(plugin.plugin.file_size())
            .map_err(|_| format!("{name}: larger than 4 GB"))?;
        let block = e.call(NEW, &[TES_FILE_SIZE]).u32();
        let file = e.call(TES_FILE_CONSTRUCT, &[block, 0, 0, 0]).u32();
        let bs_file = open_bs_file(e, &name, size);
        e.mem.set_u32(file + M_PFILE, bs_file);
        e.mem.set_u8(file + C_COMPILE_INDEX, plugin.load_index);
        // The `TES4` record and the 0x18 bytes after it.
        let header = plugin.plugin.header_record();
        let end = (RECORD_HEADER_SIZE + header.data_size as usize + RECORD_HEADER_SIZE)
            .min(plugin.plugin.file_size());
        supply_file_bytes(e, bs_file, 0, plugin.plugin.file_bytes(0..end));
        let text = |e: &mut Engine, s: &str| {
            let p = e.mem.alloc(s.len() as u32 + 1);
            e.mem.set_cstr(p, s.as_bytes());
            p
        };
        let directory = text(e, directory);
        let file_name = text(e, &name);
        if !e.call(OPEN_TES, &[file, directory, file_name, 0, 0]).bool() {
            return Err(format!("{name}: TESFile::OpenTES failed"));
        }
        let code = e.call(READ_HEADER, &[file]).u32();
        if code != 0 {
            return Err(format!(
                "{name}: the header reader (00471400) answered {code}"
            ));
        }
        e.mem.set_u32(file + M_FILESIZE, size);
        Ok(GameFile { file, bs_file })
    }

    /// Supplies the record `entry`'s bytes (header and data, as in the
    /// file) and goes to it as the game does (`TESFile::SetOffset`), so
    /// the form's `Load` reads it next. A compressed record is refused.
    pub fn supply(
        &self,
        e: &mut Engine,
        plugin: &LoadedPlugin,
        entry: &RecordEntry,
    ) -> Result<(), String> {
        if entry.header.flags & RECORD_COMPRESSED != 0 {
            return Err(format!(
                "{} {:?} in {}: compressed; reading it needs zlib (TESFile::DecompressCurrentForm, \
                 00b44000 and nearby), which isn't translated",
                entry.header.kind, entry.header.form_id, plugin.name
            ));
        }
        self.supply_any(e, plugin, entry)
    }

    /// [`GameFile::supply`] without refusing a compressed record: the game's
    /// own reading decompresses it on demand (`TESFile::ReadChunkHeader`
    /// → `00473930` → `TESFile::DecompressCurrentForm` `004740a0` → zlib's
    /// `inflate` `00b44000`), and stops where that isn't translated.
    pub fn supply_any(
        &self,
        e: &mut Engine,
        plugin: &LoadedPlugin,
        entry: &RecordEntry,
    ) -> Result<(), String> {
        // The record, and the 0x18 bytes after it (the next record's or
        // group's header): reading a record's last chunk can read the next
        // chunk header past its end, where the game reads the file's next
        // bytes before its bound check stops it.
        let offset = entry.offset as u32;
        let end = entry.offset + plugin.plugin.record_bytes(entry).len() + RECORD_HEADER_SIZE;
        let bytes = plugin.plugin.file_bytes(entry.offset..end);
        supply_file_bytes(e, self.bs_file, offset, bytes);
        if !e.call(SET_OFFSET, &[self.file, offset]).bool() {
            return Err(format!(
                "{} {:?} in {}: TESFile::SetOffset failed at {offset:#x}",
                entry.header.kind, entry.header.form_id, plugin.name
            ));
        }
        Ok(())
    }
}

/// `TESFile::GenIndexTable` (`00471870`) for `file` against `files` (every
/// file opened, in load order): its `m_pMasterPtrs`, one `TESFile *` per
/// master it names. The list is a `BSSimpleList<TESFile *>` made here in
/// the game's memory, standing in for the data handler's file list
/// (`TESDataHandler` +0x10 `pSourceFiles`, Xbox PDB), which the host
/// doesn't make. False when a master isn't among `files`.
pub fn gen_index_table(e: &mut Engine, file: &GameFile, files: &[GameFile]) -> bool {
    // Nodes: item, next (`BSSimpleList`, `engine::types`).
    let nodes: Vec<u32> = files.iter().map(|_| e.mem.alloc(8)).collect();
    for (i, (node, f)) in nodes.iter().zip(files).enumerate() {
        e.mem.set_u32(*node, f.file);
        e.mem
            .set_u32(*node + 4, nodes.get(i + 1).copied().unwrap_or(0));
    }
    let head = match nodes.first() {
        Some(head) => *head,
        None => e.mem.alloc(8),
    };
    e.call(GEN_INDEX_TABLE, &[file.file, head, 1]).bool()
}
