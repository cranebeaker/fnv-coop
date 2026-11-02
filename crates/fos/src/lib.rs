//! Reads Fallout: New Vegas's own save files (`.fos`), as the 1.4.0.525
//! game writes them. Read-only: nothing here writes a save or changes game
//! state. The format, with the executable addresses it was read from, is in
//! `docs/FOS_SAVES.md`.
//!
//! [`Save::parse`] splits a file into its parts and checks that each ends
//! where the file's location table says, so a successful parse accounts
//! for every byte. The parts' contents stay raw; [`decode`] reads the ones
//! researched so far (quests, globals, misc statistics, cells, topics,
//! actor bases, factions, classes, challenges, reputations and the
//! references' initial data).

pub mod decode;
pub mod inventory_source;
mod pipe;

pub use pipe::Pipe;

use std::fmt;

/// `"FO3SAVEGAME"`: the file's first 11 bytes (`0084d4b0`).
pub const MAGIC: &[u8; 11] = b"FO3SAVEGAME";

/// The save version the 1.4.0.525 game writes (`0066d730`); its loader
/// takes this and the one before (`00850ed0`).
pub const VERSION: u32 = 0x30;

/// The minor version the game writes after the screenshot and into every
/// change form (`00851110`).
pub const MINOR_VERSION: u8 = 0x1B;

/// Size of the file location table (`00847850` writes 0x6E bytes).
pub const LOCATION_TABLE_SIZE: usize = 0x6E;

/// A problem reading a save: where in the file and what.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    pub offset: usize,
    pub message: String,
}

impl Error {
    pub fn new(offset: usize, message: impl Into<String>) -> Self {
        Self {
            offset,
            message: message.into(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "at byte {:#x}: {}", self.offset, self.message)
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

/// The header block after the magic (`0084d4b0`).
#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    pub version: u32,
    /// The game's language setting, e.g. `ENGLISH`.
    pub language: String,
    pub screenshot_width: u32,
    pub screenshot_height: u32,
    /// `BGSSaveLoadManager` +8 (`008503b0` counts it up).
    pub save_number: u32,
    pub player_name: String,
    /// The karma title shown in the load menu (`0047e0e0`).
    pub karma_title: String,
    pub level: u32,
    /// The place shown in the load menu.
    pub location: String,
    /// Play time as the game writes it, `"hhh.mm.ss"`.
    pub play_time: String,
}

/// The file location table (0x6E bytes after the plugin list).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocationTable {
    pub form_id_array: u32,
    /// Where the history block starts, just past the worldspace id array.
    pub history: u32,
    pub global_data_1: u32,
    pub change_forms: u32,
    pub global_data_2: u32,
    pub global_data_1_count: u32,
    pub global_data_2_count: u32,
    pub change_form_count: u32,
}

/// One global data entry (`0084bb50`): its type and its data.
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalData<'a> {
    pub kind: u32,
    /// Where the data starts in the file.
    pub offset: usize,
    pub data: &'a [u8],
}

/// A change form's reference to a form (`00853570` / `00853500`): an
/// index into the form id array (from 1), or a created form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct RefId(pub u32);

impl RefId {
    /// Bit 0x800000: a form created in game (form id `0xFF......`).
    pub fn is_created(self) -> bool {
        self.0 & 0x80_0000 != 0
    }

    /// The form id it stands for, given the save's form id array; `None`
    /// for refID 0 or an index past the array.
    pub fn form_id(self, form_ids: &[u32]) -> Option<u32> {
        if self.is_created() {
            Some(0xFF00_0000 | (self.0 & 0x7F_FFFF))
        } else if self.0 == 0 {
            None
        } else {
            form_ids.get(self.0 as usize - 1).copied()
        }
    }
}

/// One change form record (`00865a30`, `00866200`).
#[derive(Debug, Clone, PartialEq)]
pub struct ChangeForm<'a> {
    pub ref_id: RefId,
    pub flags: u32,
    /// The save type (the low 6 bits of the type byte), see [`SAVE_TYPES`].
    pub save_type: u8,
    pub version: u8,
    /// Where the record starts in the file.
    pub offset: usize,
    /// Where its data starts in the file.
    pub data_offset: usize,
    pub data: &'a [u8],
}

impl ChangeForm<'_> {
    /// The record's four-letter type, if the save type is known.
    pub fn type_name(&self) -> Option<&'static str> {
        SAVE_TYPES.get(self.save_type as usize).map(|t| t.0)
    }
}

/// A whole save, split into its parts.
#[derive(Debug, Clone, PartialEq)]
pub struct Save<'a> {
    pub header: Header,
    /// Where the screenshot's RGB bytes start (width × height × 3 of them).
    pub screenshot_offset: usize,
    pub minor_version: u8,
    /// The plugins the save was made with, in its load order: form ids in
    /// the form id array use these indices in their top byte.
    pub plugins: Vec<String>,
    pub table: LocationTable,
    pub global_data_1: Vec<GlobalData<'a>>,
    pub change_forms: Vec<ChangeForm<'a>>,
    pub global_data_2: Vec<GlobalData<'a>>,
    /// Element `i` is the form id of refID `i + 1`.
    pub form_ids: Vec<u32>,
    /// Worldspaces, indexed by exterior cells' initial data.
    pub worldspaces: Vec<u32>,
    pub history: Vec<String>,
}

/// The save types in order, with the form type each stands for
/// (`BGSSaveLoadFormInfo::InitSaveGameFormTypes` (Xbox PDB), `008535e0`,
/// from the table at `011a2428`; names from the form type table
/// `01187004`).
pub const SAVE_TYPES: [(&str, u8); 55] = [
    ("REFR", 58),
    ("ACHR", 59),
    ("ACRE", 60),
    ("PMIS", 61),
    ("PGRE", 62),
    ("PBEA", 63),
    ("PFLA", 64),
    ("CELL", 57),
    ("INFO", 70),
    ("QUST", 71),
    ("NPC_", 42),
    ("CREA", 43),
    ("ACTI", 21),
    ("TACT", 22),
    ("TERM", 23),
    ("ARMO", 24),
    ("BOOK", 25),
    ("CLOT", 26),
    ("CONT", 27),
    ("DOOR", 28),
    ("INGR", 29),
    ("LIGH", 30),
    ("MISC", 31),
    ("STAT", 32),
    ("MSTT", 34),
    ("FURN", 39),
    ("WEAP", 40),
    ("AMMO", 41),
    ("KEYM", 46),
    ("ALCH", 47),
    ("IDLM", 48),
    ("NOTE", 49),
    ("ECZN", 97),
    ("CLAS", 7),
    ("FACT", 8),
    ("PACK", 73),
    ("NAVM", 67),
    ("FLST", 85),
    ("LVLC", 44),
    ("LVLN", 45),
    ("LVLI", 52),
    ("WATR", 78),
    ("IMOD", 103),
    ("REPU", 104),
    ("PCBE", 105),
    ("RCPE", 106),
    ("RCCT", 107),
    ("CHIP", 108),
    ("CSNO", 109),
    ("LSCT", 110),
    ("CHAL", 113),
    ("AMEF", 114),
    ("CCRD", 115),
    ("CMNY", 116),
    ("CDCK", 117),
];

/// Save types by name, for the decoders.
pub mod save_type {
    pub const REFR: u8 = 0;
    pub const ACHR: u8 = 1;
    pub const ACRE: u8 = 2;
    pub const PFLA: u8 = 6;
    pub const CELL: u8 = 7;
    pub const INFO: u8 = 8;
    pub const QUST: u8 = 9;
    pub const NPC_: u8 = 10;
    pub const CREA: u8 = 11;
    pub const TACT: u8 = 13;
    pub const BOOK: u8 = 16;
    pub const NOTE: u8 = 31;
    pub const ECZN: u8 = 32;
    pub const CLAS: u8 = 33;
    pub const FACT: u8 = 34;
    pub const PACK: u8 = 35;
    pub const FLST: u8 = 37;
    pub const LVLC: u8 = 38;
    pub const LVLI: u8 = 40;
    pub const WATR: u8 = 41;
    pub const IMOD: u8 = 42;
    pub const REPU: u8 = 43;
    pub const CHIP: u8 = 47;
    pub const CHAL: u8 = 50;
    pub const CMNY: u8 = 53;
}

/// The global data types (`BGSSaveLoadGlobalData::GLOBAL_DATA` (Xbox PDB)).
pub fn global_data_name(kind: u32) -> Option<&'static str> {
    Some(match kind {
        0 => "misc stats",
        1 => "location",
        2 => "TES",
        3 => "globals",
        4 => "process lists",
        5 => "combat",
        6 => "interface",
        7 => "effects",
        8 => "weather",
        9 => "actor causes",
        10 => "radio",
        11 => "audio",
        1000 => "temporary effects",
        _ => return None,
    })
}

/// The Xbox prototype's name (`CHANGE_TYPE` (Xbox PDB), without the
/// `CHANGE_` prefix) for one change flag bit of a save type.
pub fn change_flag_name(save_type: u8, bit: u32) -> Option<&'static str> {
    use crate::save_type as t;
    if bit == 0x1 {
        return Some("FORM_FLAGS");
    }
    let reference = save_type <= t::PFLA;
    let actor = save_type == t::ACHR || save_type == t::ACRE;
    if reference {
        let common = match bit {
            0x2 => Some("REFR_MOVE"),
            0x4 => Some("REFR_HAVOK_MOVE"),
            0x8 => Some("REFR_CELL_CHANGED"),
            0x10 => Some("REFR_SCALE"),
            0x20 => Some("REFR_INVENTORY"),
            0x40 => Some("REFR_EXTRA_OWNERSHIP"),
            0x400_0000 => Some("REFR_EXTRA_ACTIVATING_CHILDREN"),
            0x800_0000 => Some("REFR_LEVELED_INVENTORY"),
            0x1000_0000 => Some("REFR_ANIMATION"),
            0x2000_0000 => Some("REFR_EXTRA_ENCOUNTER_ZONE"),
            0x4000_0000 => Some("REFR_EXTRA_CREATED_ONLY"),
            0x8000_0000 => Some("REFR_EXTRA_GAME_ONLY"),
            _ => None,
        };
        if common.is_some() {
            return common;
        }
        return if actor {
            match bit {
                0x400 => Some("ACTOR_LIFESTATE"),
                0x800 => Some("ACTOR_EXTRA_PACKAGE_DATA"),
                0x1000 => Some("ACTOR_EXTRA_MERCHANT_CONTAINER"),
                0x2_0000 => Some("ACTOR_EXTRA_DISMEMBERED_LIMBS"),
                0x4_0000 => Some("ACTOR_EXTRA_LEVELED_ACTOR"),
                0x8_0000 => Some("ACTOR_DISPOSITION_MODIFIERS"),
                0x10_0000 => Some("ACTOR_TEMP_MODIFIERS"),
                0x20_0000 => Some("ACTOR_DAMAGE_MODIFIERS"),
                0x40_0000 => Some("ACTOR_OVERRIDE_MODIFIERS"),
                0x80_0000 => Some("ACTOR_PERMANENT_MODIFIERS"),
                _ => None,
            }
        } else {
            match bit {
                0x400 => Some("OBJECT_EXTRA_ITEM_DATA"),
                0x800 => Some("OBJECT_EXTRA_AMMO"),
                0x1000 => Some("OBJECT_EXTRA_LOCK"),
                0x2_0000 => Some("DOOR_EXTRA_TELEPORT"),
                0x20_0000 => Some("OBJECT_EMPTY"),
                0x40_0000 => Some("OBJECT_OPEN_DEFAULT_STATE"),
                0x80_0000 => Some("OBJECT_OPEN_STATE"),
                _ => None,
            }
        };
    }
    match (save_type, bit) {
        (t::CELL, 0x2) => Some("CELL_FLAGS"),
        (t::CELL, 0x4) => Some("CELL_FULLNAME"),
        (t::CELL, 0x8) => Some("CELL_OWNERSHIP"),
        (t::CELL, 0x1000_0000) => Some("CELL_EXTERIOR_SHORT"),
        (t::CELL, 0x2000_0000) => Some("CELL_EXTERIOR_CHAR"),
        (t::CELL, 0x4000_0000) => Some("CELL_DETACHTIME"),
        (t::CELL, 0x8000_0000) => Some("CELL_SEENDATA"),
        (t::INFO, 0x8000_0000) => Some("TOPIC_SAIDONCE"),
        (t::QUST, 0x2) => Some("QUEST_FLAGS"),
        (t::QUST, 0x4) => Some("QUEST_SCRIPT_DELAY"),
        (t::QUST, 0x2000_0000) => Some("QUEST_OBJECTIVES"),
        (t::QUST, 0x4000_0000) => Some("QUEST_SCRIPT"),
        (t::QUST, 0x8000_0000) => Some("QUEST_STAGES"),
        (t::NPC_ | t::CREA, 0x2) => Some("ACTOR_BASE_DATA"),
        (t::NPC_ | t::CREA, 0x4) => Some("ACTOR_BASE_ATTRIBUTES"),
        (t::NPC_ | t::CREA, 0x8) => Some("ACTOR_BASE_AIDATA"),
        (t::NPC_ | t::CREA, 0x10) => Some("ACTOR_BASE_SPELLLIST"),
        (t::NPC_ | t::CREA, 0x20) => Some("ACTOR_BASE_FULLNAME"),
        (t::NPC_, 0x200) => Some("NPC_SKILLS"),
        (t::NPC_, 0x400) => Some("NPC_CLASS"),
        (t::NPC_, 0x800) => Some("NPC_FACE"),
        (t::NPC_, 0x100_0000) => Some("NPC_GENDER"),
        (t::NPC_, 0x200_0000) => Some("NPC_RACE"),
        (t::CREA, 0x200) => Some("CREATURE_SKILLS"),
        (t::TACT, 0x80_0000) => Some("TALKING_ACTIVATOR_SPEAKER"),
        (t::BOOK, 0x20) => Some("BOOK_TEACHES_SKILL"),
        (t::NOTE, 0x8000_0000) => Some("NOTE_READ"),
        (12..=31 | t::IMOD | t::CHIP | t::CMNY, 0x2) => Some("BASE_OBJECT_VALUE"),
        (12..=31 | t::IMOD | t::CHIP | t::CMNY, 0x4) => Some("BASE_OBJECT_FULLNAME"),
        (t::ECZN, 0x2) => Some("ENCOUNTER_ZONE_FLAGS"),
        (t::ECZN, 0x8000_0000) => Some("ENCOUNTER_ZONE_GAME_DATA"),
        (t::CLAS, 0x2) => Some("CLASS_TAG_SKILLS"),
        (t::FACT, 0x2) => Some("FACTION_FLAGS"),
        (t::FACT, 0x4) => Some("FACTION_REACTIONS"),
        (t::FACT, 0x8000_0000) => Some("FACTION_CRIME_COUNTS"),
        (t::PACK, 0x4000_0000) => Some("PACKAGE_WAITING"),
        (t::PACK, 0x8000_0000) => Some("PACKAGE_NEVER_RUN"),
        (t::FLST, 0x8000_0000) => Some("FORM_LIST_ADDED_FORM"),
        (t::LVLC..=t::LVLI, 0x8000_0000) => Some("LEVELED_LIST_ADDED_OBJECT"),
        (t::WATR, 0x8000_0000) => Some("WATER_REMAPPED"),
        (t::REPU, 0x2) => Some("REPUTATION_VALUES"),
        (t::CHAL, 0x2) => Some("CHALLENGE_VALUE"),
        _ => None,
    }
}

/// Reads raw (unpiped) values from the file, with offsets for errors.
struct Raw<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Raw<'a> {
    fn take(&mut self, n: usize, what: &str) -> Result<&'a [u8]> {
        let end = self
            .pos
            .checked_add(n)
            .filter(|&end| end <= self.bytes.len())
            .ok_or_else(|| {
                Error::new(
                    self.pos,
                    format!(
                        "{what}: needs {n} bytes, {} left",
                        self.bytes.len() - self.pos
                    ),
                )
            })?;
        let slice = &self.bytes[self.pos..end];
        self.pos = end;
        Ok(slice)
    }

    fn u8(&mut self, what: &str) -> Result<u8> {
        Ok(self.take(1, what)?[0])
    }

    fn u16(&mut self, what: &str) -> Result<u16> {
        let b = self.take(2, what)?;
        Ok(u16::from_le_bytes([b[0], b[1]]))
    }

    fn u32(&mut self, what: &str) -> Result<u32> {
        let b = self.take(4, what)?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    /// Checks the reader stands at a file offset the location table gave.
    fn expect_at(&self, offset: u32, what: &str) -> Result<()> {
        if self.pos as u64 == u64::from(offset) {
            Ok(())
        } else {
            Err(Error::new(
                self.pos,
                format!("{what}: the location table puts it at {offset:#x}"),
            ))
        }
    }

    /// A `u32 size` and then that many bytes, as a pipe buffer.
    fn sized_block(&mut self, what: &str) -> Result<Pipe<'a>> {
        let size = self.u32(what)? as usize;
        let start = self.pos;
        let data = self.take(size, what)?;
        Ok(Pipe::new(data, start))
    }

    fn global_data(&mut self, count: u32, what: &str) -> Result<Vec<GlobalData<'a>>> {
        let mut out = Vec::new();
        for _ in 0..count {
            let kind = self.u32(what)?;
            let size = self.u32(what)? as usize;
            let offset = self.pos;
            let data = self.take(size, what)?;
            out.push(GlobalData { kind, offset, data });
        }
        Ok(out)
    }

    fn form_id_array(&mut self, what: &str) -> Result<Vec<u32>> {
        let count = self.u32(what)? as usize;
        if count > (self.bytes.len() - self.pos) / 4 {
            return Err(Error::new(
                self.pos,
                format!("{what}: {count} entries don't fit in the file"),
            ));
        }
        (0..count).map(|_| self.u32(what)).collect()
    }
}

impl<'a> Save<'a> {
    /// Splits a save into its parts, checking every size and offset: on
    /// success every byte of the file belongs to exactly one part.
    pub fn parse(bytes: &'a [u8]) -> Result<Self> {
        let mut raw = Raw { bytes, pos: 0 };
        if raw.take(MAGIC.len(), "magic")? != MAGIC {
            return Err(Error::new(0, "not a save: no FO3SAVEGAME magic"));
        }
        let mut block = raw.sized_block("header")?;
        let header = read_header(&mut block)?;
        block.finish("header")?;

        let shot = u64::from(header.screenshot_width) * u64::from(header.screenshot_height) * 3;
        let screenshot_offset = raw.pos;
        let shot = usize::try_from(shot).map_err(|_| Error::new(raw.pos, "screenshot too big"))?;
        raw.take(shot, "screenshot")?;
        let minor_version = raw.u8("minor version")?;

        let mut block = raw.sized_block("plugin list")?;
        let count = block.u8()?;
        let plugins = (0..count)
            .map(|_| block.wstr())
            .collect::<Result<Vec<_>>>()?;
        block.finish("plugin list")?;

        let table_offset = raw.pos;
        let t = raw.take(LOCATION_TABLE_SIZE, "location table")?;
        let word = |i: usize| u32::from_le_bytes([t[i], t[i + 1], t[i + 2], t[i + 3]]);
        let table = LocationTable {
            form_id_array: word(0x00),
            history: word(0x04),
            global_data_1: word(0x08),
            change_forms: word(0x0C),
            global_data_2: word(0x10),
            global_data_1_count: word(0x14),
            global_data_2_count: word(0x18),
            change_form_count: word(0x1C),
        };
        if t[0x20..].iter().any(|&b| b != 0) {
            return Err(Error::new(
                table_offset + 0x20,
                "location table: the unused part isn't zero",
            ));
        }

        raw.expect_at(table.global_data_1, "global data table 1")?;
        let global_data_1 = raw.global_data(table.global_data_1_count, "global data table 1")?;
        raw.expect_at(table.change_forms, "change forms")?;
        let mut change_forms = Vec::new();
        for _ in 0..table.change_form_count {
            change_forms.push(read_change_form(&mut raw)?);
        }
        raw.expect_at(table.global_data_2, "global data table 2")?;
        let global_data_2 = raw.global_data(table.global_data_2_count, "global data table 2")?;
        raw.expect_at(table.form_id_array, "form id array")?;
        let form_ids = raw.form_id_array("form id array")?;
        let worldspaces = raw.form_id_array("worldspace id array")?;
        raw.expect_at(table.history, "history")?;
        let mut block = raw.sized_block("history")?;
        let count = block.u32()?;
        let history = (0..count)
            .map(|_| block.wstr())
            .collect::<Result<Vec<_>>>()?;
        block.finish("history")?;
        if raw.pos != bytes.len() {
            return Err(Error::new(
                raw.pos,
                format!("{} bytes after the history block", bytes.len() - raw.pos),
            ));
        }
        Ok(Save {
            header,
            screenshot_offset,
            minor_version,
            plugins,
            table,
            global_data_1,
            change_forms,
            global_data_2,
            form_ids,
            worldspaces,
            history,
        })
    }

    /// The form id a refID stands for in this save.
    pub fn form_id(&self, ref_id: RefId) -> Option<u32> {
        ref_id.form_id(&self.form_ids)
    }

    /// The change form of a form id, if the save has one.
    pub fn change_form_of(&self, form_id: u32) -> Option<&ChangeForm<'a>> {
        self.change_forms
            .iter()
            .find(|c| self.form_id(c.ref_id) == Some(form_id))
    }

    /// A global data entry by type, from either table.
    pub fn global_data(&self, kind: u32) -> Option<&GlobalData<'a>> {
        self.global_data_1
            .iter()
            .chain(&self.global_data_2)
            .find(|g| g.kind == kind)
    }
}

fn read_header(p: &mut Pipe<'_>) -> Result<Header> {
    let version = p.u32()?;
    let language = p.bytes(64)?;
    let end = language.iter().position(|&b| b == 0).unwrap_or(64);
    let language = latin1(&language[..end]);
    Ok(Header {
        version,
        language,
        screenshot_width: p.u32()?,
        screenshot_height: p.u32()?,
        save_number: p.u32()?,
        player_name: p.wstr()?,
        karma_title: p.wstr()?,
        level: p.u32()?,
        location: p.wstr()?,
        play_time: p.wstr()?,
    })
}

fn read_change_form<'a>(raw: &mut Raw<'a>) -> Result<ChangeForm<'a>> {
    let offset = raw.pos;
    let id = raw.take(3, "change form")?;
    let ref_id = RefId(u32::from(id[0]) << 16 | u32::from(id[1]) << 8 | u32::from(id[2]));
    let flags = raw.u32("change form")?;
    let kind = raw.u8("change form")?;
    let version = raw.u8("change form")?;
    let length = match kind >> 6 {
        0 => u32::from(raw.u8("change form")?),
        1 => u32::from(raw.u16("change form")?),
        2 => raw.u32("change form")?,
        _ => return Err(Error::new(offset + 7, "change form: length size 3")),
    };
    let data_offset = raw.pos;
    let data = raw.take(length as usize, "change form data")?;
    Ok(ChangeForm {
        ref_id,
        flags,
        save_type: kind & 0x3F,
        version,
        offset,
        data_offset,
        data,
    })
}

/// The game's strings are 8-bit (Windows-1252); Latin-1 is close enough
/// for display and keeps every byte.
pub(crate) fn latin1(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| char::from(b)).collect()
}

/// Builds synthetic saves for tests (feature `write`); not a save writer
/// for the game.
#[cfg(any(test, feature = "write"))]
pub mod write;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_refs;
