//! Record, group and subrecord structures.

use crate::cursor::Cursor;
use crate::error::{Error, Result};
use crate::text;
use crate::types::{sig, FormId, FourCC};

/// Size of a record header in Fallout 3 / New Vegas plugins. (Oblivion used
/// 20 bytes; FO3 and FNV added a version field.)
pub const RECORD_HEADER_LEN: usize = 24;
/// Size of a group header.
pub const GROUP_HEADER_LEN: usize = 24;

/// Record flag bits. Some bits mean different things on different record
/// types; these are the widely shared ones.
pub mod flags {
    /// On the TES4 header: this file is a master (.esm).
    pub const MASTER: u32 = 0x0000_0001;
    pub const DELETED: u32 = 0x0000_0020;
    /// On placed references.
    pub const PERSISTENT: u32 = 0x0000_0400;
    pub const INITIALLY_DISABLED: u32 = 0x0000_0800;
    pub const IGNORED: u32 = 0x0000_1000;
    /// The record's data is a zlib stream preceded by its decompressed size.
    pub const COMPRESSED: u32 = 0x0004_0000;

    /// Names for [`describe`].
    pub const NAMED: [(u32, &str); 5] = [
        (DELETED, "deleted"),
        (PERSISTENT, "persistent"),
        (INITIALLY_DISABLED, "initially disabled"),
        (IGNORED, "ignored"),
        (COMPRESSED, "compressed"),
    ];

    /// Human-readable list of the flags set in `value`.
    pub fn describe(value: u32) -> String {
        let mut parts: Vec<String> = Vec::new();
        let mut rest = value;
        for (bit, name) in NAMED {
            if value & bit != 0 {
                parts.push(name.to_string());
                rest &= !bit;
            }
        }
        if rest != 0 {
            parts.push(format!("other bits {rest:#010x}"));
        }
        if parts.is_empty() {
            "none".into()
        } else {
            parts.join(", ")
        }
    }
}

/// The 24-byte header in front of every record.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RecordHeader {
    pub kind: FourCC,
    /// Size of the record's data on disk, excluding this header.
    pub data_size: u32,
    pub flags: u32,
    pub form_id: FormId,
    /// Version-control info written by the editor.
    pub revision: u32,
    /// Record format version.
    pub version: u16,
    pub unknown: u16,
}

impl RecordHeader {
    pub(crate) fn read(cur: &mut Cursor) -> Result<Self> {
        Ok(Self {
            kind: cur.fourcc("record type")?,
            data_size: cur.u32("record size")?,
            flags: cur.u32("record flags")?,
            form_id: FormId(cur.u32("form ID")?),
            revision: cur.u32("record revision")?,
            version: cur.u16("record version")?,
            unknown: cur.u16("record header")?,
        })
    }

    pub fn is_compressed(&self) -> bool {
        self.flags & flags::COMPRESSED != 0
    }

    pub fn is_deleted(&self) -> bool {
        self.flags & flags::DELETED != 0
    }
}

/// The 24-byte header in front of every group.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GroupHeader {
    /// Size of the whole group, *including* this header.
    pub size: u32,
    /// Meaning depends on `group_type`; see [`GroupHeader::kind`].
    pub label: [u8; 4],
    pub group_type: i32,
    pub stamp: u16,
    pub unknown1: u16,
    pub version: u16,
    pub unknown2: u16,
}

/// What a group contains, decoded from its type and label.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupKind {
    /// Top-level group holding every record of one type.
    Top(FourCC),
    WorldChildren(FormId),
    InteriorCellBlock(i32),
    InteriorCellSubBlock(i32),
    ExteriorCellBlock {
        x: i16,
        y: i16,
    },
    ExteriorCellSubBlock {
        x: i16,
        y: i16,
    },
    CellChildren(FormId),
    TopicChildren(FormId),
    CellPersistentChildren(FormId),
    CellTemporaryChildren(FormId),
    CellVisibleDistantChildren(FormId),
    Unknown {
        group_type: i32,
        label: [u8; 4],
    },
}

impl GroupHeader {
    /// Reads a group header, including its `GRUP` tag.
    pub(crate) fn read(cur: &mut Cursor) -> Result<Self> {
        let tag_offset = cur.offset();
        let tag = cur.fourcc("group tag")?;
        if tag != sig::GRUP {
            return Err(Error::BadGroup {
                offset: tag_offset,
                reason: format!("expected 'GRUP', found '{tag}'"),
            });
        }
        Ok(Self {
            size: cur.u32("group size")?,
            label: cur.array("group label")?,
            group_type: cur.i32("group type")?,
            stamp: cur.u16("group stamp")?,
            unknown1: cur.u16("group header")?,
            version: cur.u16("group version")?,
            unknown2: cur.u16("group header")?,
        })
    }

    pub fn kind(&self) -> GroupKind {
        let l = self.label;
        let id = FormId(u32::from_le_bytes(l));
        let number = i32::from_le_bytes(l);
        // Exterior block labels store the grid Y coordinate first, then X
        // (per community format documentation).
        let y = i16::from_le_bytes([l[0], l[1]]);
        let x = i16::from_le_bytes([l[2], l[3]]);
        match self.group_type {
            0 => GroupKind::Top(FourCC(l)),
            1 => GroupKind::WorldChildren(id),
            2 => GroupKind::InteriorCellBlock(number),
            3 => GroupKind::InteriorCellSubBlock(number),
            4 => GroupKind::ExteriorCellBlock { x, y },
            5 => GroupKind::ExteriorCellSubBlock { x, y },
            6 => GroupKind::CellChildren(id),
            7 => GroupKind::TopicChildren(id),
            8 => GroupKind::CellPersistentChildren(id),
            9 => GroupKind::CellTemporaryChildren(id),
            10 => GroupKind::CellVisibleDistantChildren(id),
            t => GroupKind::Unknown {
                group_type: t,
                label: l,
            },
        }
    }
}

/// One typed field inside a record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subrecord {
    pub kind: FourCC,
    pub data: Vec<u8>,
}

impl Subrecord {
    /// The data decoded as a NUL-terminated Windows-1252 string.
    pub fn zstring(&self) -> String {
        text::zstring(&self.data)
    }
}

/// Splits a record's (decompressed) data into subrecords.
///
/// Subrecord sizes are 16-bit. Larger payloads are preceded by an `XXXX`
/// subrecord holding the real 32-bit size, and the following subrecord's own
/// size field is then ignored.
pub fn parse_subrecords(data: &[u8], base_offset: usize) -> Result<Vec<Subrecord>> {
    let mut cur = Cursor::new(data, base_offset);
    let mut out = Vec::new();
    while !cur.is_empty() {
        let kind = cur.fourcc("subrecord type")?;
        let size = cur.u16("subrecord size")?;
        if kind == sig::XXXX {
            if size != 4 {
                return Err(Error::BadRecord {
                    offset: cur.offset(),
                    reason: format!("XXXX subrecord has size {size}, expected 4"),
                });
            }
            let real_size = cur.u32("XXXX size")? as usize;
            let real_kind = cur.fourcc("subrecord type")?;
            let _ignored = cur.u16("subrecord size")?;
            let payload = cur.take(real_size, "large subrecord data")?;
            out.push(Subrecord {
                kind: real_kind,
                data: payload.to_vec(),
            });
        } else {
            let payload = cur.take(usize::from(size), "subrecord data")?;
            out.push(Subrecord {
                kind,
                data: payload.to_vec(),
            });
        }
    }
    Ok(out)
}

/// A decoded record: its header plus all of its subrecords.
#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub header: RecordHeader,
    pub subrecords: Vec<Subrecord>,
}

impl Record {
    /// The first subrecord of the given type.
    pub fn get(&self, kind: FourCC) -> Option<&Subrecord> {
        self.subrecords.iter().find(|s| s.kind == kind)
    }

    /// Every subrecord of the given type, in file order.
    pub fn get_all(&self, kind: FourCC) -> impl Iterator<Item = &Subrecord> {
        self.subrecords.iter().filter(move |s| s.kind == kind)
    }

    /// The editor ID (`EDID`), the internal name used by the game's editor.
    pub fn editor_id(&self) -> Option<String> {
        self.get(sig::EDID).map(Subrecord::zstring)
    }

    /// The in-game display name (`FULL`), if the record has one.
    pub fn full_name(&self) -> Option<String> {
        self.get(sig::FULL).map(Subrecord::zstring)
    }
}
