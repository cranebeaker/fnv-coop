//! Loading a plugin file and indexing its records.

use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::Range;
use std::path::Path;

use crate::cursor::Cursor;
use crate::error::{Error, Result};
use crate::inflate;
use crate::record::{
    flags, parse_subrecords, GroupHeader, GroupKind, Record, RecordHeader, Subrecord,
    GROUP_HEADER_LEN, RECORD_HEADER_LEN,
};
use crate::text;
use crate::types::{sig, FormId, FourCC};

/// Groups nest at most a handful of levels deep in real files (worldspace →
/// block → sub-block → cell children → ...). Anything deeper is corrupt.
const MAX_GROUP_DEPTH: usize = 16;

/// Fields from the `TES4` header record at the start of every plugin.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PluginHeader {
    /// Format version from `HEDR` (1.34 for New Vegas files).
    pub version: f32,
    /// Record count the editor wrote into `HEDR`.
    pub declared_record_count: i32,
    pub next_object_id: u32,
    pub author: Option<String>,
    pub description: Option<String>,
    /// Master files this plugin depends on, in load order.
    pub masters: Vec<String>,
    /// The size of each master when this plugin was saved (the `DATA`
    /// after its `MAST`), if given. The game warns when a master's size
    /// differs (`00471af0`, `sGeneralMasterMismatchWarning`).
    pub master_sizes: Vec<Option<u64>>,
    /// Whether the header's master flag is set (normally true for .esm files).
    pub is_master: bool,
}

impl PluginHeader {
    fn from_subrecords(subrecords: &[Subrecord], record_flags: u32) -> Self {
        let mut header = PluginHeader {
            is_master: record_flags & flags::MASTER != 0,
            ..Default::default()
        };
        for sub in subrecords {
            let d = &sub.data;
            match sub.kind {
                k if k == sig::HEDR && d.len() >= 12 => {
                    header.version = f32::from_le_bytes([d[0], d[1], d[2], d[3]]);
                    header.declared_record_count = i32::from_le_bytes([d[4], d[5], d[6], d[7]]);
                    header.next_object_id = u32::from_le_bytes([d[8], d[9], d[10], d[11]]);
                }
                k if k == sig::CNAM => header.author = Some(sub.zstring()),
                k if k == sig::SNAM => header.description = Some(sub.zstring()),
                k if k == sig::MAST => {
                    header.masters.push(sub.zstring());
                    header.master_sizes.push(None);
                }
                k if k == sig::DATA && d.len() >= 8 => {
                    if let Some(last) = header.master_sizes.last_mut() {
                        *last = Some(u64::from_le_bytes(d[..8].try_into().unwrap()));
                    }
                }
                _ => {}
            }
        }
        header
    }
}

/// Where a record's position in the plugin is recorded; its contents are
/// decoded on demand with [`Plugin::record`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RecordEntry {
    pub header: RecordHeader,
    /// File offset of the record header.
    pub offset: usize,
    /// Record type of the top-level group this record sits in. For records
    /// nested inside cells and worldspaces this is `CELL` or `WRLD`.
    pub top_group: FourCC,
    /// For records inside a cell's child groups (placed objects, actors,
    /// navmeshes, terrain): the cell, as a form ID local to this plugin.
    pub cell: Option<FormId>,
    /// For records inside a worldspace's child groups (its cells and their
    /// contents): the worldspace, as a form ID local to this plugin.
    pub world: Option<FormId>,
    /// For dialogue lines (`INFO`) inside a topic's child group: the topic
    /// (`DIAL`), as a form ID local to this plugin.
    pub topic: Option<FormId>,
}

impl RecordEntry {
    /// Byte range of the record's data (after the header) in the file.
    pub fn data_range(&self) -> Range<usize> {
        let start = self.offset + RECORD_HEADER_LEN;
        start..start + self.header.data_size as usize
    }
}

/// Which file a form ID refers to, relative to this plugin's master list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormOrigin<'a> {
    /// The record is defined in (or overrides one from) this master.
    Master(&'a str),
    /// The record is new in this plugin.
    ThisPlugin,
    /// The mod index is past the end of the master list.
    UnknownMaster(u8),
}

/// A parsed plugin file.
///
/// Parsing reads the whole file into memory and walks the group structure
/// once to index every record; record contents (including decompression)
/// are only decoded when asked for.
pub struct Plugin {
    bytes: Vec<u8>,
    header: PluginHeader,
    header_record: RecordHeader,
    records: Vec<RecordEntry>,
    by_type: HashMap<FourCC, Vec<usize>>,
    by_form_id: HashMap<FormId, usize>,
    group_count: usize,
}

impl Plugin {
    /// Reads and indexes a plugin file from disk.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        Self::from_bytes(std::fs::read(path)?)
    }

    /// Indexes a plugin already in memory.
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        let (header_record, header, body_start) = read_tes4(&bytes)?;

        let mut walk = Walk::default();
        let mut pos = body_start;
        while pos < bytes.len() {
            pos += walk_group(&bytes, pos, bytes.len(), None, 0, &mut walk)?;
        }

        let mut by_type: HashMap<FourCC, Vec<usize>> = HashMap::new();
        let mut by_form_id = HashMap::with_capacity(walk.records.len());
        for (i, entry) in walk.records.iter().enumerate() {
            by_type.entry(entry.header.kind).or_default().push(i);
            by_form_id.entry(entry.header.form_id).or_insert(i);
        }

        Ok(Self {
            bytes,
            header,
            header_record,
            records: walk.records,
            by_type,
            by_form_id,
            group_count: walk.groups,
        })
    }

    pub fn header(&self) -> &PluginHeader {
        &self.header
    }

    /// The raw `TES4` record header (its flags, version and so on).
    pub fn header_record(&self) -> &RecordHeader {
        &self.header_record
    }

    pub fn file_size(&self) -> usize {
        self.bytes.len()
    }

    pub fn group_count(&self) -> usize {
        self.group_count
    }

    /// Every record in file order (the `TES4` header is not included).
    pub fn records(&self) -> &[RecordEntry] {
        &self.records
    }

    /// Records of one type, in file order.
    pub fn records_of_type(&self, kind: FourCC) -> impl Iterator<Item = &RecordEntry> + '_ {
        self.by_type
            .get(&kind)
            .into_iter()
            .flatten()
            .map(move |&i| &self.records[i])
    }

    pub fn count_of_type(&self, kind: FourCC) -> usize {
        self.by_type.get(&kind).map_or(0, Vec::len)
    }

    /// Positions in [`Plugin::records`] of every record of one type.
    pub fn record_indices_of_type(&self, kind: FourCC) -> &[usize] {
        self.by_type.get(&kind).map_or(&[], Vec::as_slice)
    }

    /// Record counts per type, most common first.
    pub fn type_counts(&self) -> Vec<(FourCC, usize)> {
        let mut counts: Vec<_> = self.by_type.iter().map(|(k, v)| (*k, v.len())).collect();
        counts.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        counts
    }

    /// Looks a record up by form ID.
    pub fn get(&self, form_id: FormId) -> Option<&RecordEntry> {
        self.by_form_id.get(&form_id).map(|&i| &self.records[i])
    }

    /// The record's data exactly as stored (still compressed if flagged).
    pub fn raw_data(&self, entry: &RecordEntry) -> &[u8] {
        &self.bytes[entry.data_range()]
    }

    /// The record as it is in the file: its header and its data as stored
    /// (compressed if it is), starting at [`RecordEntry::offset`].
    pub fn record_bytes(&self, entry: &RecordEntry) -> &[u8] {
        &self.bytes[entry.offset..entry.data_range().end]
    }

    /// The file's bytes in `range`, cut at the end of the file.
    pub fn file_bytes(&self, range: Range<usize>) -> &[u8] {
        let end = range.end.min(self.bytes.len());
        &self.bytes[range.start.min(end)..end]
    }

    /// The record's data, decompressed if necessary.
    pub fn data(&self, entry: &RecordEntry) -> Result<Cow<'_, [u8]>> {
        decompress_record(&entry.header, self.raw_data(entry))
    }

    /// Decodes a record's subrecords.
    pub fn subrecords(&self, entry: &RecordEntry) -> Result<Vec<Subrecord>> {
        let data = self.data(entry)?;
        self.parse_entry_subrecords(entry, &data)
    }

    /// The data of a record's first subrecord of one type, without
    /// decoding the rest (borrowed from the file unless the record is
    /// compressed). The same bytes as `record(entry)?.get(kind)`.
    pub fn subrecord_of(&self, entry: &RecordEntry, kind: FourCC) -> Result<Option<Cow<'_, [u8]>>> {
        let data = self.data(entry)?;
        let Some(range) = find_subrecord(&data, kind) else {
            return Ok(None);
        };
        Ok(Some(match data {
            Cow::Borrowed(b) => Cow::Borrowed(&b[range]),
            Cow::Owned(v) => Cow::Owned(v[range].to_vec()),
        }))
    }

    /// Decodes a whole record.
    pub fn record(&self, entry: &RecordEntry) -> Result<Record> {
        Ok(Record {
            header: entry.header,
            subrecords: self.subrecords(entry)?,
        })
    }

    /// The record's editor ID, read without decoding the rest of the
    /// record when `EDID` comes first (as it almost always does).
    pub fn editor_id_of(&self, entry: &RecordEntry) -> Result<Option<String>> {
        let data = self.data(entry)?;
        if data.len() >= 6 && data[..4] == *sig::EDID.as_bytes() {
            let size = usize::from(u16::from_le_bytes([data[4], data[5]]));
            if let Some(payload) = data.get(6..6 + size) {
                return Ok(Some(text::zstring(payload)));
            }
        }
        let subrecords = self.parse_entry_subrecords(entry, &data)?;
        Ok(subrecords
            .iter()
            .find(|s| s.kind == sig::EDID)
            .map(Subrecord::zstring))
    }

    /// Finds a record by editor ID (case-insensitive, as in the game),
    /// optionally restricted to one record type. This scans the plugin.
    pub fn find_by_editor_id(
        &self,
        editor_id: &str,
        kind: Option<FourCC>,
    ) -> Result<Option<&RecordEntry>> {
        let candidates: Box<dyn Iterator<Item = &RecordEntry>> = match kind {
            Some(kind) => Box::new(self.records_of_type(kind)),
            None => Box::new(self.records.iter()),
        };
        for entry in candidates {
            if let Some(found) = self.editor_id_of(entry)? {
                if found.eq_ignore_ascii_case(editor_id) {
                    return Ok(Some(entry));
                }
            }
        }
        Ok(None)
    }

    /// Resolves a form ID's mod index against this plugin's masters.
    pub fn form_origin(&self, form_id: FormId) -> FormOrigin<'_> {
        let index = usize::from(form_id.mod_index());
        match index.cmp(&self.header.masters.len()) {
            std::cmp::Ordering::Less => FormOrigin::Master(&self.header.masters[index]),
            std::cmp::Ordering::Equal => FormOrigin::ThisPlugin,
            std::cmp::Ordering::Greater => FormOrigin::UnknownMaster(form_id.mod_index()),
        }
    }

    fn parse_entry_subrecords(&self, entry: &RecordEntry, data: &[u8]) -> Result<Vec<Subrecord>> {
        // Offsets in errors are file offsets for plain records and offsets
        // into the decompressed data for compressed ones.
        let base = if entry.header.is_compressed() {
            0
        } else {
            entry.data_range().start
        };
        parse_subrecords(data, base).map_err(|e| in_record(&entry.header, e))
    }
}

/// The `TES4` header record at the start of a plugin's bytes: its record
/// header, its fields, and where the records after it start.
fn read_tes4(bytes: &[u8]) -> Result<(RecordHeader, PluginHeader, usize)> {
    let mut found = [0u8; 4];
    let n = bytes.len().min(4);
    found[..n].copy_from_slice(&bytes[..n]);
    if FourCC(found) != sig::TES4 {
        return Err(Error::NotAPlugin {
            found: FourCC(found),
        });
    }
    let mut cur = Cursor::new(bytes, 0);
    let header_record = RecordHeader::read(&mut cur)?;
    let raw = cur.take(header_record.data_size as usize, "TES4 header data")?;
    let data = decompress_record(&header_record, raw)?;
    let subrecords =
        parse_subrecords(&data, RECORD_HEADER_LEN).map_err(|e| in_record(&header_record, e))?;
    let header = PluginHeader::from_subrecords(&subrecords, header_record.flags);
    Ok((header_record, header, cur.offset()))
}

/// Reads only a plugin file's header (master flag, masters), the way the
/// game reads every plugin in `Data` to order them before loading any.
pub fn read_header(path: impl AsRef<Path>) -> Result<PluginHeader> {
    use std::io::Read;
    let mut file = std::fs::File::open(path)?;
    let mut bytes = vec![0u8; RECORD_HEADER_LEN];
    file.read_exact(&mut bytes)?;
    let size = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]) as usize;
    if &bytes[..4] == sig::TES4.as_bytes() {
        let mut data = vec![0u8; size];
        file.read_exact(&mut data)?;
        bytes.extend(data);
    }
    Ok(read_tes4(&bytes)?.1)
}

fn in_record(header: &RecordHeader, source: Error) -> Error {
    Error::InRecord {
        kind: header.kind,
        form_id: header.form_id,
        source: Box::new(source),
    }
}

/// Compressed records store a u32 decompressed size followed by a zlib stream.
/// A wrong checksum is accepted when the data comes out at its stated size,
/// as the game accepts it (see [`inflate::zlib_decompress_unverified`]).
/// Where the first subrecord of a type's data is in a record's data, read
/// as [`crate::record::parse_subrecords`] reads them (an `XXXX` before a
/// subrecord gives its real size); `None` if it has none, or the data ends
/// before it.
fn find_subrecord(data: &[u8], kind: FourCC) -> Option<std::ops::Range<usize>> {
    let mut at = 0usize;
    let mut big: Option<usize> = None;
    while at + 6 <= data.len() {
        let this = FourCC([data[at], data[at + 1], data[at + 2], data[at + 3]]);
        let size = usize::from(u16::from_le_bytes([data[at + 4], data[at + 5]]));
        at += 6;
        if this == sig::XXXX {
            if size != 4 || at + 4 > data.len() {
                return None;
            }
            big = Some(
                u32::from_le_bytes([data[at], data[at + 1], data[at + 2], data[at + 3]]) as usize,
            );
            at += 4;
            continue;
        }
        let size = big.take().unwrap_or(size);
        let end = at.checked_add(size).filter(|&e| e <= data.len())?;
        if this == kind {
            return Some(at..end);
        }
        at = end;
    }
    None
}

fn decompress_record<'a>(header: &RecordHeader, raw: &'a [u8]) -> Result<Cow<'a, [u8]>> {
    if !header.is_compressed() {
        return Ok(Cow::Borrowed(raw));
    }
    let fail = |reason: String| Error::Decompress {
        form_id: header.form_id,
        reason,
    };
    if raw.len() < 4 {
        return Err(fail("data is shorter than its 4-byte size prefix".into()));
    }
    let expected = u32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]) as usize;
    // Cap the pre-allocation so a corrupt size field can't request gigabytes.
    let hint = expected.min(raw.len().saturating_mul(64)).min(64 << 20);
    let (out, checksum) =
        inflate::zlib_decompress_unverified(&raw[4..], Some(hint)).map_err(fail)?;
    if out.len() != expected {
        let checksum = checksum.err().map_or(String::new(), |e| format!("; {e}"));
        return Err(fail(format!(
            "expected {expected} bytes after decompression, got {}{checksum}",
            out.len()
        )));
    }
    Ok(Cow::Owned(out))
}

#[derive(Default)]
struct Walk {
    records: Vec<RecordEntry>,
    groups: usize,
}

/// Indexes the group starting at `start`, which must end at or before
/// `limit`. Returns the group's total size.
/// Where a group sits: its top-level record type, and the worldspace and
/// cell it belongs to, if any.
#[derive(Clone, Copy)]
struct GroupContext {
    top: FourCC,
    world: Option<FormId>,
    cell: Option<FormId>,
    topic: Option<FormId>,
}

fn walk_group(
    bytes: &[u8],
    start: usize,
    limit: usize,
    parent: Option<GroupContext>,
    depth: usize,
    walk: &mut Walk,
) -> Result<usize> {
    if depth > MAX_GROUP_DEPTH {
        return Err(Error::BadGroup {
            offset: start,
            reason: format!("groups are nested more than {MAX_GROUP_DEPTH} levels deep"),
        });
    }
    let mut cur = Cursor::new(&bytes[start..limit], start);
    let group = GroupHeader::read(&mut cur)?;
    let size = group.size as usize;
    if size < GROUP_HEADER_LEN {
        return Err(Error::BadGroup {
            offset: start,
            reason: format!("size {size} is smaller than the {GROUP_HEADER_LEN}-byte header"),
        });
    }
    if size > limit - start {
        return Err(Error::BadGroup {
            offset: start,
            reason: format!(
                "size {size} runs past the end of its container ({} bytes available)",
                limit - start
            ),
        });
    }
    let kind = group.kind();
    let mut context = match (kind, parent) {
        (GroupKind::Top(top), _) => GroupContext {
            top,
            world: None,
            cell: None,
            topic: None,
        },
        (_, Some(parent)) => parent,
        (kind, None) => {
            return Err(Error::BadGroup {
                offset: start,
                reason: format!("expected a top-level group, found {kind:?}"),
            })
        }
    };
    match kind {
        GroupKind::WorldChildren(world) => {
            context.world = Some(world);
            context.cell = None;
        }
        GroupKind::CellChildren(cell)
        | GroupKind::CellPersistentChildren(cell)
        | GroupKind::CellTemporaryChildren(cell)
        | GroupKind::CellVisibleDistantChildren(cell) => context.cell = Some(cell),
        GroupKind::TopicChildren(topic) => context.topic = Some(topic),
        _ => {}
    }
    walk.groups += 1;

    let end = start + size;
    let mut pos = start + GROUP_HEADER_LEN;
    while pos < end {
        if bytes[pos..end].starts_with(sig::GRUP.as_bytes()) {
            pos += walk_group(bytes, pos, end, Some(context), depth + 1, walk)?;
            continue;
        }
        let mut cur = Cursor::new(&bytes[pos..end], pos);
        let header = RecordHeader::read(&mut cur)?;
        let data_end = (pos + RECORD_HEADER_LEN)
            .checked_add(header.data_size as usize)
            .filter(|&e| e <= end)
            .ok_or_else(|| Error::BadRecord {
                offset: pos,
                reason: format!(
                    "{} record {} claims {} bytes of data, past the end of its group",
                    header.kind, header.form_id, header.data_size
                ),
            })?;
        walk.records.push(RecordEntry {
            header,
            offset: pos,
            top_group: context.top,
            cell: context.cell,
            world: context.world,
            topic: context.topic,
        });
        pos = data_end;
    }
    Ok(size)
}
