//! `coverage`: what the game has, counted from its own files, against what
//! nv-rs covers (the tables in [`crate::coverage_table`]).
//!
//! * `coverage records`: every record type and subrecord in every loaded
//!   plugin, counted per plugin, each record type with nv-rs's status and
//!   each subrecord marked read or not.
//! * `coverage files`: every file in every archive in the Data folder and
//!   every loose file, by kind; every NIF block type (in `.nif` and `.kf`
//!   files) with whether the `nif` crate decodes it; texture formats, sound
//!   formats, the menu files and the classes they name.
//!
//! Both print Markdown, so the output can be saved as an inventory page.

use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::path::{Path, PathBuf};

use esm::{FourCC, LoadOrder};

use crate::coverage_table::{self as table, Status};
use crate::fmt::{human_bytes, thousands};
use crate::CliError;

// ---------------------------------------------------------------------------
// Records
// ---------------------------------------------------------------------------

/// Every record and subrecord in a load order, counted.
#[derive(Debug, Default)]
pub struct RecordCensus {
    /// Plugin file names in load order.
    pub plugins: Vec<String>,
    /// Record count per plugin (in load order), by record type. Every
    /// version counts: a record a DLC overrides is counted in both files.
    pub records: BTreeMap<FourCC, Vec<usize>>,
    /// By (record type, subrecord type): occurrences and the number of
    /// records holding at least one.
    pub subrecords: BTreeMap<(FourCC, FourCC), (usize, usize)>,
    /// Records whose data couldn't be decoded (wrong compression and the
    /// like), by type.
    pub unreadable: BTreeMap<FourCC, usize>,
}

impl RecordCensus {
    pub fn total(&self, kind: FourCC) -> usize {
        self.records.get(&kind).map_or(0, |v| v.iter().sum())
    }
}

/// Counts every record version and subrecord of every plugin.
pub fn record_census(order: &LoadOrder) -> RecordCensus {
    let mut census = RecordCensus {
        plugins: order.plugins().iter().map(|p| p.name.clone()).collect(),
        ..RecordCensus::default()
    };
    let n = order.plugins().len();
    for (pi, p) in order.plugins().iter().enumerate() {
        for entry in p.plugin.records() {
            let kind = entry.header.kind;
            census.records.entry(kind).or_insert_with(|| vec![0; n])[pi] += 1;
            let Ok(subs) = p.plugin.subrecords(entry) else {
                *census.unreadable.entry(kind).or_default() += 1;
                continue;
            };
            let mut seen: BTreeSet<FourCC> = BTreeSet::new();
            for sub in &subs {
                let slot = census.subrecords.entry((kind, sub.kind)).or_default();
                slot.0 += 1;
                if seen.insert(sub.kind) {
                    slot.1 += 1;
                }
            }
        }
    }
    census
}

/// Totals for the summary.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct RecordTotals {
    pub types_in_exe: usize,
    pub types_in_data: usize,
    /// Record types in the data with no entry in nv-rs's table.
    pub missing_entries: Vec<String>,
    /// Record types in the data by status (records counted, not types).
    pub by_status: BTreeMap<&'static str, (usize, usize)>,
    pub subrecord_kinds: usize,
    pub subrecord_kinds_read: usize,
    pub subrecords: usize,
    pub subrecords_read: usize,
}

pub fn record_totals(census: &RecordCensus) -> RecordTotals {
    let mut t = RecordTotals {
        types_in_exe: table::EXE_FORM_TYPES.len() - 1,
        types_in_data: census.records.len(),
        ..RecordTotals::default()
    };
    for (&kind, counts) in &census.records {
        let total: usize = counts.iter().sum();
        match table::record(kind) {
            Some(entry) => {
                let slot = t.by_status.entry(entry.status.label()).or_default();
                slot.0 += 1;
                slot.1 += total;
            }
            None => t.missing_entries.push(kind.to_string()),
        }
    }
    for (&(kind, sub), &(count, _)) in &census.subrecords {
        t.subrecord_kinds += 1;
        t.subrecords += count;
        if table::reads_subrecord(kind, sub) {
            t.subrecord_kinds_read += 1;
            t.subrecords_read += count;
        }
    }
    t
}

fn percent(part: usize, whole: usize) -> String {
    if whole == 0 {
        "-".into()
    } else {
        format!("{:.0}%", 100.0 * part as f64 / whole as f64)
    }
}

/// Short column titles for the official plugins.
fn short_plugin_name(name: &str) -> String {
    let stem = name.rsplit_once('.').map_or(name, |(s, _)| s);
    match stem.to_ascii_lowercase().as_str() {
        "falloutnv" => "FNV".into(),
        "deadmoney" => "DM".into(),
        "honesthearts" => "HH".into(),
        "oldworldblues" => "OWB".into(),
        "lonesomeroad" => "LR".into(),
        "gunrunnersarsenal" => "GRA".into(),
        "classicpack" => "CP".into(),
        "mercenarypack" => "MP".into(),
        "tribalpack" => "TP".into(),
        "caravanpack" => "CvP".into(),
        _ => stem.chars().take(8).collect(),
    }
}

pub fn records(out: &mut impl Write, order: &LoadOrder) -> Result<(), CliError> {
    let census = record_census(order);
    write_records(out, &census)
}

pub fn write_records(out: &mut impl Write, census: &RecordCensus) -> Result<(), CliError> {
    let totals = record_totals(census);
    writeln!(out, "# Records: the game's plugins against nv-rs")?;
    writeln!(out)?;
    writeln!(
        out,
        "Counted from the plugins themselves (`nvinspect <Data> coverage records`). \
         Every version counts: a record a DLC changes is counted in both files. \
         Status and the subrecords nv-rs reads come from the table in \
         `crates/nvinspect/src/coverage_table.rs`; form type numbers from the exe's \
         form type table (`01187000`)."
    )?;
    writeln!(out)?;
    writeln!(out, "Plugins: {}", census.plugins.join(", "))?;
    writeln!(out)?;
    writeln!(out, "## Summary")?;
    writeln!(out)?;
    writeln!(
        out,
        "- Record types the exe knows: {} (form types 1–{}); in the data: {}.",
        totals.types_in_exe, totals.types_in_exe, totals.types_in_data
    )?;
    let absent: Vec<&str> = table::EXE_FORM_TYPES
        .iter()
        .skip(1)
        .copied()
        .filter(|k| !matches!(*k, "TES4" | "GRUP"))
        .filter(|k| !census.records.contains_key(&FourCC::new(&four(k))))
        .collect();
    writeln!(
        out,
        "- Record types the exe knows that none of these plugins has: {} ({}).",
        absent.len(),
        absent.join(", ")
    )?;
    for status in Status::ALL {
        if let Some(&(types, records)) = totals.by_status.get(status.label()) {
            writeln!(
                out,
                "- {}: {} types, {} records.",
                status.label(),
                types,
                thousands(records)
            )?;
        }
    }
    let total_records: usize = census.records.keys().map(|&k| census.total(k)).sum();
    writeln!(out, "- Records in all: {}.", thousands(total_records))?;
    writeln!(
        out,
        "- Subrecord kinds (record type + subrecord) in the data: {}; read by nv-rs: {} ({}). \
         Weighted by how often they occur: {} of {} ({}).",
        thousands(totals.subrecord_kinds),
        thousands(totals.subrecord_kinds_read),
        percent(totals.subrecord_kinds_read, totals.subrecord_kinds),
        thousands(totals.subrecords_read),
        thousands(totals.subrecords),
        percent(totals.subrecords_read, totals.subrecords),
    )?;
    if totals.missing_entries.is_empty() {
        writeln!(
            out,
            "- Every record type in the data has an entry in nv-rs's table."
        )?;
    } else {
        writeln!(
            out,
            "- **Record types in the data with no entry in nv-rs's table: {}.**",
            totals.missing_entries.join(", ")
        )?;
    }
    let unreadable: usize = census.unreadable.values().sum();
    if unreadable > 0 {
        let list: Vec<String> = census
            .unreadable
            .iter()
            .map(|(k, n)| format!("{k} {n}"))
            .collect();
        writeln!(
            out,
            "- Records whose data couldn't be decoded: {} ({}).",
            unreadable,
            list.join(", ")
        )?;
    }
    writeln!(out)?;

    // One row per form type the exe knows, then any extra type in the data.
    writeln!(out, "## Record types")?;
    writeln!(out)?;
    let short: Vec<String> = census
        .plugins
        .iter()
        .map(|p| short_plugin_name(p))
        .collect();
    writeln!(
        out,
        "| # | Type | What | {} | Total | Subrecords read | Status | nv-rs | Missing |",
        short.join(" | ")
    )?;
    writeln!(
        out,
        "|---|---|---|{}---|---|---|---|---|",
        "---|".repeat(short.len())
    )?;
    let mut kinds: Vec<FourCC> = table::EXE_FORM_TYPES
        .iter()
        .skip(1)
        .map(|s| FourCC::new(&four(s)))
        .collect();
    for &k in census.records.keys() {
        if !kinds.contains(&k) {
            kinds.push(k);
        }
    }
    for kind in kinds {
        let number = table::form_type_number(kind)
            .map(|n| format!("{n}"))
            .unwrap_or_else(|| "-".into());
        let counts = census.records.get(&kind);
        let cells: Vec<String> = (0..census.plugins.len())
            .map(|i| match counts.map_or(0, |c| c[i]) {
                0 => String::new(),
                n => thousands(n),
            })
            .collect();
        let total = census.total(kind);
        let present: Vec<FourCC> = census
            .subrecords
            .range((kind, FourCC([0; 4]))..=(kind, FourCC([0xFF; 4])))
            .map(|(&(_, s), _)| s)
            .collect();
        let read = present
            .iter()
            .filter(|&&s| table::reads_subrecord(kind, s))
            .count();
        let (what, status, modules, note) = match table::record(kind) {
            Some(e) => (e.what, e.status.label(), e.modules, e.note),
            None => ("?", "**no entry**", "", ""),
        };
        writeln!(
            out,
            "| {number} | {kind} | {what} | {} | {} | {} | {status} | {modules} | {note} |",
            cells.join(" | "),
            if total == 0 {
                "0".to_string()
            } else {
                thousands(total)
            },
            if present.is_empty() {
                "-".to_string()
            } else {
                format!("{read}/{}", present.len())
            },
        )?;
    }
    writeln!(out)?;

    writeln!(out, "## Subrecords by record type")?;
    writeln!(out)?;
    writeln!(
        out,
        "Each subrecord present in the data: occurrences, records holding it, and \
         whether nv-rs reads it (✔) or not (·)."
    )?;
    for &kind in census.records.keys() {
        let entry = table::record(kind);
        writeln!(out)?;
        writeln!(
            out,
            "### {kind}: {} — {}",
            entry.map_or("?", |e| e.what),
            entry.map_or("no entry", |e| e.status.label())
        )?;
        writeln!(out)?;
        let subs: Vec<String> = census
            .subrecords
            .range((kind, FourCC([0; 4]))..=(kind, FourCC([0xFF; 4])))
            .map(|(&(_, s), &(count, holders))| {
                let mark = if table::reads_subrecord(kind, s) {
                    "✔"
                } else {
                    "·"
                };
                format!(
                    "{mark} `{}` {}/{}",
                    table::sig_text(s),
                    thousands(count),
                    thousands(holders)
                )
            })
            .collect();
        writeln!(out, "{}", subs.join(", "))?;
        if let Some(e) = entry {
            let listed_absent: Vec<&str> = e
                .read
                .split_whitespace()
                .filter(|s| !s.starts_with('*'))
                .filter(|s| {
                    !census
                        .subrecords
                        .contains_key(&(kind, FourCC::new(&four(s))))
                })
                .collect();
            if !listed_absent.is_empty() {
                writeln!(out)?;
                writeln!(
                    out,
                    "(nv-rs's table lists {} for it, which no record here has.)",
                    listed_absent.join(" ")
                )?;
            }
        }
    }
    Ok(())
}

/// A four-character code from text (padded with `_` if short).
fn four(s: &str) -> [u8; 4] {
    let mut out = [b'_'; 4];
    for (o, b) in out.iter_mut().zip(s.bytes()) {
        *o = b;
    }
    out
}

// ---------------------------------------------------------------------------
// Files
// ---------------------------------------------------------------------------

/// What one file kind holds, over every archive and the loose files.
#[derive(Debug, Default, Clone)]
pub struct KindCount {
    pub files: usize,
    pub bytes: u64,
}

/// Every file the game's Data folder holds, counted.
#[derive(Debug, Default)]
pub struct FileCensus {
    /// Archives read: name, file count.
    pub archives: Vec<(String, usize)>,
    pub loose_files: usize,
    /// By extension (lower case, without the dot; "" for none).
    pub by_extension: BTreeMap<String, KindCount>,
    /// By top folder and extension.
    pub by_folder: BTreeMap<(String, String), usize>,
    /// NIF block types: (files with it, blocks) in `.nif`, then in `.kf`.
    pub nif_blocks: BTreeMap<String, [usize; 4]>,
    /// NIF header versions: (version, user version, Bethesda version).
    pub nif_versions: BTreeMap<String, usize>,
    pub nif_errors: Vec<(String, String)>,
    /// Texture formats (with "cube" for cube maps).
    pub dds_formats: BTreeMap<String, usize>,
    pub dds_errors: Vec<(String, String)>,
    /// WAV formats: "PCM 16-bit 44100 Hz 2 ch" and the like.
    pub wav_formats: BTreeMap<String, usize>,
    /// OGG Vorbis: rate and channels.
    pub ogg_formats: BTreeMap<String, usize>,
    /// Menu files (`menus\...xml`) and the menu class they name.
    pub menu_files: BTreeMap<String, String>,
}

/// Counts one file. `path` is lower case with backslashes.
pub fn survey_file(census: &mut FileCensus, path: &str, bytes: &[u8]) {
    let ext = extension(path);
    let slot = census.by_extension.entry(ext.clone()).or_default();
    slot.files += 1;
    slot.bytes += bytes.len() as u64;
    let folder = path.split('\\').next().unwrap_or("").to_string();
    *census.by_folder.entry((folder, ext.clone())).or_default() += 1;
    match ext.as_str() {
        "nif" | "kf" | "psa" => survey_nif(census, path, bytes, ext == "kf"),
        "dds" => match dds::Dds::parse(bytes.to_vec()) {
            Ok(d) => {
                let mut name = d.format().name();
                if d.is_cube_map() {
                    name.push_str(" cube");
                }
                *census.dds_formats.entry(name).or_default() += 1;
            }
            Err(e) => census.dds_errors.push((path.into(), e.to_string())),
        },
        "wav" => {
            *census.wav_formats.entry(wav_format(bytes)).or_default() += 1;
        }
        "ogg" => {
            *census.ogg_formats.entry(ogg_format(bytes)).or_default() += 1;
        }
        "xml" if path.starts_with("menus\\") => {
            census.menu_files.insert(path.into(), menu_class(bytes));
        }
        _ => {}
    }
}

fn survey_nif(census: &mut FileCensus, path: &str, bytes: &[u8], kf: bool) {
    // Bethesda's .psa pose arrays are NIFs too.
    if !bytes.starts_with(b"Gamebryo") && !bytes.starts_with(b"NetImmerse") {
        return;
    }
    match nif::Nif::parse(bytes.to_vec()) {
        Ok(n) => {
            let h = n.header();
            *census
                .nif_versions
                .entry(format!(
                    "{} user {} Bethesda {}",
                    nif::version_string(h.version),
                    h.user_version,
                    h.bs_version
                ))
                .or_default() += 1;
            let column = if kf { 2 } else { 0 };
            for (name, count) in n.type_counts() {
                let slot = census.nif_blocks.entry(name).or_default();
                slot[column] += 1;
                slot[column + 1] += count;
            }
        }
        Err(e) => census.nif_errors.push((path.into(), e.to_string())),
    }
}

fn extension(path: &str) -> String {
    let name = path.rsplit('\\').next().unwrap_or(path);
    match name.rsplit_once('.') {
        Some((_, ext)) => ext.to_ascii_lowercase(),
        None => String::new(),
    }
}

fn u16_at(b: &[u8], i: usize) -> Option<u16> {
    Some(u16::from_le_bytes(b.get(i..i + 2)?.try_into().ok()?))
}

fn u32_at(b: &[u8], i: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(i..i + 4)?.try_into().ok()?))
}

/// A WAV file's format chunk in words.
pub fn wav_format(b: &[u8]) -> String {
    if !b.starts_with(b"RIFF") || b.get(8..12) != Some(b"WAVE") {
        return "not RIFF WAVE".into();
    }
    let mut pos = 12;
    while pos + 8 <= b.len() {
        let id = &b[pos..pos + 4];
        let size = u32_at(b, pos + 4).unwrap_or(0) as usize;
        if id == b"fmt " {
            let d = pos + 8;
            let tag = u16_at(b, d).unwrap_or(0);
            let channels = u16_at(b, d + 2).unwrap_or(0);
            let rate = u32_at(b, d + 4).unwrap_or(0);
            let bits = u16_at(b, d + 14).unwrap_or(0);
            let kind = match tag {
                1 => "PCM".to_string(),
                2 => "MS ADPCM".to_string(),
                0x11 => "IMA ADPCM".to_string(),
                0x55 => "MP3".to_string(),
                0xFFFE => "extensible".to_string(),
                t => format!("format {t:#x}"),
            };
            return format!("{kind} {bits}-bit {rate} Hz {channels} ch");
        }
        pos += 8 + size + (size & 1);
    }
    "no format chunk".into()
}

/// An Ogg Vorbis file's identification header: rate and channels.
pub fn ogg_format(b: &[u8]) -> String {
    if !b.starts_with(b"OggS") {
        return "not Ogg".into();
    }
    // First page: 27-byte header, segment table, then the packet.
    let segments = usize::from(*b.get(26).unwrap_or(&0));
    let packet = 27 + segments;
    if b.get(packet..packet + 7) != Some(b"\x01vorbis") {
        return "Ogg, not Vorbis".into();
    }
    let channels = b.get(packet + 11).copied().unwrap_or(0);
    let rate = u32_at(b, packet + 12).unwrap_or(0);
    format!("Vorbis {rate} Hz {channels} ch")
}

/// The `<class>` a menu file names (e.g. `&HUDMainMenu;`), or "" for a
/// file that names none (prefabs and included pieces).
pub fn menu_class(b: &[u8]) -> String {
    let text = String::from_utf8_lossy(b);
    let lower = text.to_ascii_lowercase();
    let Some(start) = lower.find("<class>") else {
        return String::new();
    };
    let rest = &text[start + 7..];
    let end = rest.find('<').unwrap_or(rest.len());
    rest[..end].trim().to_string()
}

/// Loose files under the Data folder's subfolders (files directly in Data
/// are plugins and archives), as lower-case relative paths.
fn loose_files(data_dir: &Path) -> Vec<(String, PathBuf)> {
    fn walk(dir: &Path, prefix: &str, depth: usize, out: &mut Vec<(String, PathBuf)>) {
        if depth > 32 {
            return;
        }
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for e in entries.flatten() {
            let name = e.file_name().to_string_lossy().to_ascii_lowercase();
            let rel = if prefix.is_empty() {
                name.clone()
            } else {
                format!("{prefix}\\{name}")
            };
            let Ok(kind) = e.file_type() else { continue };
            if kind.is_dir() {
                walk(&e.path(), &rel, depth + 1, out);
            } else if depth > 0 {
                out.push((rel, e.path()));
            }
        }
    }
    let mut out = Vec::new();
    walk(data_dir, "", 0, &mut out);
    out.sort();
    out
}

/// Reads every archive in the Data folder (loaded or not) and every loose
/// file, in parallel.
pub fn file_census(data_dir: &Path) -> Result<FileCensus, CliError> {
    let mut archives: Vec<String> = std::fs::read_dir(data_dir)
        .map_err(|e| CliError::Open {
            path: data_dir.display().to_string(),
            message: e.to_string(),
        })?
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n.to_ascii_lowercase().ends_with(".bsa"))
        .collect();
    archives.sort_by_key(|n| n.to_ascii_lowercase());
    let threads = std::thread::available_parallelism().map_or(4, |n| n.get());
    let mut census = FileCensus::default();

    for name in &archives {
        let path = data_dir.join(name);
        let archive = bsa::Archive::open(&path)?;
        let count = archive.files().len();
        census.archives.push((name.clone(), count));
        drop(archive);
        let parts: Vec<FileCensus> = std::thread::scope(|s| {
            let handles: Vec<_> = (0..threads)
                .map(|t| {
                    let path = path.clone();
                    s.spawn(move || {
                        let mut part = FileCensus::default();
                        let Ok(archive) = bsa::Archive::open(&path) else {
                            return part;
                        };
                        for (i, entry) in archive.files().iter().enumerate() {
                            if i % threads != t {
                                continue;
                            }
                            match archive.read(entry) {
                                Ok(bytes) => survey_file(&mut part, &entry.path, &bytes),
                                Err(e) => part.nif_errors.push((entry.path.clone(), e.to_string())),
                            }
                        }
                        part
                    })
                })
                .collect();
            handles
                .into_iter()
                .map(|h| h.join().unwrap_or_default())
                .collect()
        });
        for part in parts {
            merge(&mut census, part);
        }
    }

    let loose = loose_files(data_dir);
    census.loose_files = loose.len();
    let parts: Vec<FileCensus> = std::thread::scope(|s| {
        let loose = &loose;
        let handles: Vec<_> = (0..threads)
            .map(|t| {
                s.spawn(move || {
                    let mut part = FileCensus::default();
                    for (i, (rel, path)) in loose.iter().enumerate() {
                        if i % threads != t {
                            continue;
                        }
                        if let Ok(bytes) = std::fs::read(path) {
                            survey_file(&mut part, rel, &bytes);
                        }
                    }
                    part
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().unwrap_or_default())
            .collect()
    });
    for part in parts {
        merge(&mut census, part);
    }
    Ok(census)
}

fn merge(into: &mut FileCensus, part: FileCensus) {
    for (k, v) in part.by_extension {
        let slot = into.by_extension.entry(k).or_default();
        slot.files += v.files;
        slot.bytes += v.bytes;
    }
    for (k, v) in part.by_folder {
        *into.by_folder.entry(k).or_default() += v;
    }
    for (k, v) in part.nif_blocks {
        let slot = into.nif_blocks.entry(k).or_default();
        for i in 0..4 {
            slot[i] += v[i];
        }
    }
    for (k, v) in part.nif_versions {
        *into.nif_versions.entry(k).or_default() += v;
    }
    into.nif_errors.extend(part.nif_errors);
    for (k, v) in part.dds_formats {
        *into.dds_formats.entry(k).or_default() += v;
    }
    into.dds_errors.extend(part.dds_errors);
    for (k, v) in part.wav_formats {
        *into.wav_formats.entry(k).or_default() += v;
    }
    for (k, v) in part.ogg_formats {
        *into.ogg_formats.entry(k).or_default() += v;
    }
    into.menu_files.extend(part.menu_files);
}

/// Totals for the summary of `coverage files`.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct FileTotals {
    pub kinds: usize,
    pub kinds_read: usize,
    pub files: usize,
    pub files_read: usize,
    pub block_types: usize,
    pub block_types_decoded: usize,
    pub blocks: usize,
    pub blocks_decoded: usize,
    /// Extensions in the data with no entry in nv-rs's file table.
    pub missing_entries: Vec<String>,
}

pub fn file_totals(census: &FileCensus) -> FileTotals {
    let mut t = FileTotals::default();
    for (ext, count) in &census.by_extension {
        t.kinds += 1;
        t.files += count.files;
        match table::file_kind(ext) {
            Some(k) if matches!(k.status, Status::Done | Status::Partial) => {
                t.kinds_read += 1;
                t.files_read += count.files;
            }
            Some(_) => {}
            None => t.missing_entries.push(ext.clone()),
        }
    }
    for (name, c) in &census.nif_blocks {
        t.block_types += 1;
        t.blocks += c[1] + c[3];
        if table::nif_block(name).is_some() {
            t.block_types_decoded += 1;
            t.blocks_decoded += c[1] + c[3];
        }
    }
    t
}

pub fn files(out: &mut impl Write, data_dir: &Path) -> Result<(), CliError> {
    let census = file_census(data_dir)?;
    write_files(out, &census)
}

pub fn write_files(out: &mut impl Write, census: &FileCensus) -> Result<(), CliError> {
    let t = file_totals(census);
    writeln!(out, "# Files: the game's Data folder against nv-rs")?;
    writeln!(out)?;
    writeln!(
        out,
        "Counted from every archive in the Data folder (loaded or not) and every loose \
         file (`nvinspect <Data> coverage files`). A path in two archives counts twice. \
         Status from `crates/nvinspect/src/coverage_table.rs`."
    )?;
    writeln!(out)?;
    writeln!(out, "## Summary")?;
    writeln!(out)?;
    let archived: usize = census.archives.iter().map(|a| a.1).sum();
    writeln!(
        out,
        "- Archives: {} holding {} files; loose files: {}.",
        census.archives.len(),
        thousands(archived),
        thousands(census.loose_files)
    )?;
    writeln!(
        out,
        "- File kinds (extensions): {}; read by nv-rs in some way: {} ({} of {} files, {}).",
        t.kinds,
        t.kinds_read,
        thousands(t.files_read),
        thousands(t.files),
        percent(t.files_read, t.files)
    )?;
    writeln!(
        out,
        "- NIF block types (in .nif, .kf, .psa): {}; decoded by the `nif` crate: {} ({}); \
         blocks: {} of {} ({}).",
        t.block_types,
        t.block_types_decoded,
        percent(t.block_types_decoded, t.block_types),
        thousands(t.blocks_decoded),
        thousands(t.blocks),
        percent(t.blocks_decoded, t.blocks)
    )?;
    if !t.missing_entries.is_empty() {
        writeln!(
            out,
            "- **Extensions with no entry in nv-rs's table: {}.**",
            t.missing_entries.join(", ")
        )?;
    }
    if !census.nif_errors.is_empty() || !census.dds_errors.is_empty() {
        writeln!(
            out,
            "- Files that didn't parse: {} NIF/archive, {} DDS (listed at the end).",
            census.nif_errors.len(),
            census.dds_errors.len()
        )?;
    }
    writeln!(out)?;

    writeln!(out, "## Archives")?;
    writeln!(out)?;
    for (name, count) in &census.archives {
        writeln!(out, "- {name}: {} files", thousands(*count))?;
    }
    writeln!(out)?;

    writeln!(out, "## File kinds")?;
    writeln!(out)?;
    writeln!(
        out,
        "| Ext | Files | Size | Folders | What | Status | nv-rs | Missing |"
    )?;
    writeln!(out, "|---|---|---|---|---|---|---|---|")?;
    for (ext, count) in &census.by_extension {
        let folders: Vec<String> = census
            .by_folder
            .iter()
            .filter(|((_, e), _)| e == ext)
            .map(|((f, _), n)| format!("{f} {}", thousands(*n)))
            .collect();
        let (what, status, reader, note) = match table::file_kind(ext) {
            Some(k) => (k.what, k.status.label(), k.reader, k.note),
            None => ("?", "**no entry**", "", ""),
        };
        writeln!(
            out,
            "| .{ext} | {} | {} | {} | {what} | {status} | {reader} | {note} |",
            thousands(count.files),
            human_bytes(count.bytes),
            folders.join(", ")
        )?;
    }
    writeln!(out)?;

    writeln!(out, "## NIF block types")?;
    writeln!(out)?;
    writeln!(
        out,
        "Every block type in the game's models (.nif), animations (.kf) and pose \
         arrays (.psa): files using it and blocks, and where nv-rs decodes it."
    )?;
    writeln!(out)?;
    writeln!(out, "Versions: ")?;
    for (v, n) in &census.nif_versions {
        writeln!(out, "- {v}: {} files", thousands(*n))?;
    }
    writeln!(out)?;
    writeln!(
        out,
        "| Block type | .nif files | .nif blocks | .kf files | .kf blocks | Decoded by |"
    )?;
    writeln!(out, "|---|---|---|---|---|---|")?;
    let mut blocks: Vec<(&String, &[usize; 4])> = census.nif_blocks.iter().collect();
    blocks.sort_by(|a, b| (b.1[1] + b.1[3]).cmp(&(a.1[1] + a.1[3])).then(a.0.cmp(b.0)));
    for (name, c) in blocks {
        writeln!(
            out,
            "| {name} | {} | {} | {} | {} | {} |",
            thousands(c[0]),
            thousands(c[1]),
            thousands(c[2]),
            thousands(c[3]),
            table::nif_block(name).unwrap_or("·")
        )?;
    }
    writeln!(out)?;

    writeln!(out, "## Texture formats (.dds)")?;
    writeln!(out)?;
    for (f, n) in &census.dds_formats {
        writeln!(out, "- {f}: {}", thousands(*n))?;
    }
    writeln!(out)?;
    writeln!(out, "## Sound formats")?;
    writeln!(out)?;
    for (f, n) in &census.wav_formats {
        writeln!(out, "- .wav {f}: {}", thousands(*n))?;
    }
    for (f, n) in &census.ogg_formats {
        writeln!(out, "- .ogg {f}: {}", thousands(*n))?;
    }
    writeln!(out)?;
    writeln!(out, "## Menu files")?;
    writeln!(out)?;
    writeln!(
        out,
        "Every `menus\\` XML file and the menu class its `<class>` names (none: a \
         piece included by others)."
    )?;
    writeln!(out)?;
    for (path, class) in &census.menu_files {
        if class.is_empty() {
            writeln!(out, "- {path}")?;
        } else {
            writeln!(out, "- {path}: `{class}`")?;
        }
    }
    if !census.nif_errors.is_empty() || !census.dds_errors.is_empty() {
        writeln!(out)?;
        writeln!(out, "## Files that didn't parse")?;
        writeln!(out)?;
        for (p, e) in census.nif_errors.iter().chain(&census.dds_errors) {
            writeln!(out, "- {p}: {e}")?;
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Script functions
// ---------------------------------------------------------------------------

/// How the game's data uses one script function.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct FunctionUse {
    pub calls: usize,
    pub scripts: usize,
    pub conditions: usize,
}

/// Every script function call in the data's script sources (`SCTX`) and
/// every condition (`CTDA`), by function number.
pub fn function_uses(order: &LoadOrder) -> BTreeMap<u16, FunctionUse> {
    fn walk(stmts: &[script::Stmt], seen: &mut BTreeMap<u16, usize>) {
        let expr = |e: &script::Expr, seen: &mut BTreeMap<u16, usize>| {
            for item in &e.0 {
                if let script::Item::Call(c) = item {
                    *seen.entry(c.function).or_default() += 1;
                }
            }
        };
        for s in stmts {
            match s {
                script::Stmt::Set { value, .. } => expr(value, seen),
                script::Stmt::Call(c) => *seen.entry(c.function).or_default() += 1,
                script::Stmt::If {
                    branches,
                    otherwise,
                } => {
                    for (cond, body) in branches {
                        expr(cond, seen);
                        walk(body, seen);
                    }
                    if let Some(body) = otherwise {
                        walk(body, seen);
                    }
                }
                script::Stmt::Return => {}
            }
        }
    }
    let (sctx, ctda) = (FourCC::new(b"SCTX"), FourCC::new(b"CTDA"));
    let mut uses: BTreeMap<u16, FunctionUse> = BTreeMap::new();
    for p in order.plugins() {
        for entry in p.plugin.records() {
            let Ok(subs) = p.plugin.subrecords(entry) else {
                continue;
            };
            for sub in &subs {
                if sub.kind == sctx {
                    let source = esm::text::decode_cp1252(&sub.data);
                    let Ok(parsed) = script::parse(&source) else {
                        continue;
                    };
                    let mut seen = BTreeMap::new();
                    for block in &parsed.blocks {
                        walk(&block.body, &mut seen);
                    }
                    walk(&parsed.body, &mut seen);
                    for (f, n) in seen {
                        let u = uses.entry(f).or_default();
                        u.calls += n;
                        u.scripts += 1;
                    }
                } else if sub.kind == ctda {
                    if let Some(f) = sub.data.get(8..10) {
                        uses.entry(u16::from_le_bytes([f[0], f[1]]))
                            .or_default()
                            .conditions += 1;
                    }
                }
            }
        }
    }
    uses
}

/// Whether nv-rs carries out a script function (by its full name).
pub fn function_handled(name: &str) -> bool {
    world::scripting::HANDLED
        .iter()
        .copied()
        .chain(world::script_functions::handled())
        .any(|h| h.eq_ignore_ascii_case(name))
}

pub fn functions(out: &mut impl Write, order: &LoadOrder) -> Result<(), CliError> {
    let uses = function_uses(order);
    write_functions(out, &uses)
}

pub fn write_functions(
    out: &mut impl Write,
    uses: &BTreeMap<u16, FunctionUse>,
) -> Result<(), CliError> {
    let all = &script::functions::FUNCTIONS;
    let is_used = |i: usize| {
        uses.get(&(i as u16))
            .is_some_and(|u| u != &FunctionUse::default())
    };
    let named = |i: usize| !all[i].name.starts_with("UnusedFunction");
    let handled: Vec<bool> = all.iter().map(|s| function_handled(s.name)).collect();
    let real = (0..all.len()).filter(|&i| named(i)).count();
    let used = (0..all.len()).filter(|&i| is_used(i)).count();
    let used_done = (0..all.len()).filter(|&i| is_used(i) && handled[i]).count();
    let done = (0..all.len()).filter(|&i| named(i) && handled[i]).count();
    let weight = |i: usize| uses.get(&(i as u16)).map_or(0, |u| u.calls + u.conditions);
    let total_weight: usize = (0..all.len()).map(weight).sum();
    let done_weight: usize = (0..all.len()).filter(|&i| handled[i]).map(weight).sum();
    writeln!(out, "# Script functions: the exe's table against nv-rs")?;
    writeln!(out)?;
    writeln!(
        out,
        "The exe's script function table (`01190910`, opcodes 0x1000 + n; `script::functions`), \
         each with how often the game's scripts call it and its conditions ask it \
         (`nvinspect <Data> coverage functions`), and whether nv-rs carries it out \
         (`world::scripting::HANDLED`, `world::script_functions::handled()`). Console-only \
         commands (opcodes 0x100–0x1D1) are listed in exe_systems.md."
    )?;
    writeln!(out)?;
    writeln!(out, "## Summary")?;
    writeln!(out)?;
    writeln!(
        out,
        "- Functions in the table: {} ({real} named, the rest `UnusedFunction`).",
        all.len()
    )?;
    writeln!(
        out,
        "- Carried out by nv-rs: {done} of {real} named ({}).",
        percent(done, real)
    )?;
    writeln!(
        out,
        "- Used by the game's data: {used}; of those carried out: {used_done} ({}); \
         weighted by uses: {} of {} ({}).",
        percent(used_done, used),
        thousands(done_weight),
        thousands(total_weight),
        percent(done_weight, total_weight)
    )?;
    writeln!(out)?;
    writeln!(
        out,
        "## Used by the game and not carried out (most used first)"
    )?;
    writeln!(out)?;
    writeln!(out, "| # | Function | Calls | Scripts | Conditions |")?;
    writeln!(out, "|---|---|---|---|---|")?;
    let mut missing: Vec<usize> = (0..all.len())
        .filter(|&i| is_used(i) && !handled[i])
        .collect();
    missing.sort_by_key(|&i| std::cmp::Reverse(weight(i)));
    for i in missing {
        let u = &uses[&(i as u16)];
        writeln!(
            out,
            "| {i} | {} | {} | {} | {} |",
            all[i].name, u.calls, u.scripts, u.conditions
        )?;
    }
    writeln!(out)?;
    writeln!(out, "## Every function")?;
    writeln!(out)?;
    writeln!(
        out,
        "| # | Function | Short | Calls | Scripts | Conditions | nv-rs |"
    )?;
    writeln!(out, "|---|---|---|---|---|---|---|")?;
    for (i, s) in all.iter().enumerate() {
        if !named(i) {
            continue;
        }
        let u = uses.get(&(i as u16)).cloned().unwrap_or_default();
        writeln!(
            out,
            "| {i} | {} | {} | {} | {} | {} | {} |",
            s.name,
            s.short,
            u.calls,
            u.scripts,
            u.conditions,
            if handled[i] { "✔" } else { "·" }
        )?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Command
// ---------------------------------------------------------------------------

pub fn run(
    out: &mut impl Write,
    order: &LoadOrder,
    data_dir: &Path,
    what: &str,
) -> Result<(), CliError> {
    match what {
        "records" => records(out, order),
        "files" => files(out, data_dir),
        "functions" => functions(out, order),
        other => Err(CliError::Usage(format!(
            "coverage: '{other}' isn't one of records, files, functions"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use testdata::{group, record, sub, zstr};

    fn plugin(records: &[u8]) -> esm::Plugin {
        let mut bytes = record(b"TES4", 0, &sub(b"HEDR", &[0; 12]));
        bytes.extend(records);
        esm::Plugin::from_bytes(bytes).unwrap()
    }

    fn census_of(records: &[u8]) -> RecordCensus {
        let order = LoadOrder::single("Test.esm", None, plugin(records)).unwrap();
        record_census(&order)
    }

    #[test]
    fn counts_records_and_subrecords() {
        let mut weapons = Vec::new();
        for id in 1..=2u32 {
            let mut d = sub(b"EDID", &zstr("Gun"));
            d.extend(sub(b"DATA", &[0; 15]));
            d.extend(sub(b"DATA", &[0; 15]));
            weapons.extend(record(b"WEAP", id, &d));
        }
        let mut bytes = group(*b"WEAP", 0, &weapons);
        bytes.extend(group(
            *b"GLOB",
            0,
            &record(b"GLOB", 3, &sub(b"EDID", &zstr("G"))),
        ));
        let c = census_of(&bytes);
        assert_eq!(c.total(FourCC::new(b"WEAP")), 2);
        assert_eq!(c.total(FourCC::new(b"GLOB")), 1);
        // DATA twice in each of two records.
        assert_eq!(
            c.subrecords[&(FourCC::new(b"WEAP"), FourCC::new(b"DATA"))],
            (4, 2)
        );
        assert_eq!(
            c.subrecords[&(FourCC::new(b"WEAP"), FourCC::new(b"EDID"))],
            (2, 2)
        );
    }

    #[test]
    fn a_record_type_without_an_entry_is_reported() {
        let bytes = group(
            *b"ZZZZ",
            0,
            &record(b"ZZZZ", 1, &sub(b"EDID", &zstr("Odd"))),
        );
        let c = census_of(&bytes);
        let t = record_totals(&c);
        assert_eq!(t.missing_entries, vec!["ZZZZ".to_string()]);
        let mut out = Vec::new();
        assert!(write_records(&mut out, &c).is_ok());
        let text = String::from_utf8(out).unwrap();
        assert!(text.contains("no entry in nv-rs's table: ZZZZ"), "{text}");
    }

    #[test]
    fn subrecords_read_are_counted_from_the_table() {
        // EDID is read for every record type; a made-up subrecord isn't.
        let mut d = sub(b"EDID", &zstr("Gun"));
        d.extend(sub(b"QQQQ", &[1, 2]));
        let c = census_of(&group(*b"WEAP", 0, &record(b"WEAP", 1, &d)));
        let t = record_totals(&c);
        assert_eq!(t.subrecord_kinds, 2);
        assert_eq!(t.subrecord_kinds_read, 1);
        assert_eq!(t.missing_entries, Vec::<String>::new());
    }

    #[test]
    fn every_exe_form_type_has_an_entry() {
        for (number, kind) in table::EXE_FORM_TYPES.iter().enumerate().skip(1) {
            let entry = table::record(FourCC::new(&four(kind)))
                .unwrap_or_else(|| panic!("form type {number} {kind} has no entry"));
            assert_eq!(
                table::form_type_number(FourCC::new(&four(kind))),
                Some(number)
            );
            // What it reads must be four-character codes.
            for s in entry.read.split_whitespace() {
                assert_eq!(s.len(), 4, "{kind}: '{s}'");
            }
        }
        // And the table has nothing the exe doesn't know.
        for e in table::RECORDS {
            assert!(
                table::EXE_FORM_TYPES.contains(&e.kind),
                "{} isn't one of the exe's form types",
                e.kind
            );
        }
    }

    #[test]
    fn surveys_files_by_kind() {
        let mut c = FileCensus::default();
        let g = testdata::boxed([0.0; 3], [1.0; 3]);
        survey_file(
            &mut c,
            "meshes\\a.nif",
            &testdata::nif(&g, "textures\\a.dds"),
        );
        survey_file(&mut c, "textures\\a.dds", &testdata::dds([1, 2, 3]));
        let mut wav = b"RIFF\0\0\0\0WAVEfmt \x10\0\0\0".to_vec();
        wav.extend(1u16.to_le_bytes());
        wav.extend(2u16.to_le_bytes());
        wav.extend(44100u32.to_le_bytes());
        wav.extend([0; 6]);
        wav.extend(16u16.to_le_bytes());
        survey_file(&mut c, "sound\\a.wav", &wav);
        survey_file(
            &mut c,
            "menus\\main\\hud.xml",
            b"<menu name=\"HUDMainMenu\">\n  <class> &HUDMainMenu; </class>\n</menu>",
        );
        survey_file(&mut c, "menus\\prefabs\\piece.xml", b"<rect name=\"x\"/>");
        survey_file(&mut c, "misc\\readme.zzz", b"hello");

        assert_eq!(c.by_extension["nif"].files, 1);
        assert_eq!(c.nif_blocks["NiTriShape"][0], 1);
        assert_eq!(c.nif_blocks["BSFadeNode"][1], 1);
        assert_eq!(c.dds_formats.values().sum::<usize>(), 1);
        assert_eq!(c.wav_formats["PCM 16-bit 44100 Hz 2 ch"], 1);
        assert_eq!(c.menu_files["menus\\main\\hud.xml"], "&HUDMainMenu;");
        assert_eq!(c.menu_files["menus\\prefabs\\piece.xml"], "");

        let t = file_totals(&c);
        assert_eq!(t.missing_entries, vec!["zzz".to_string()]);
        // The test model's blocks are all ones the nif crate decodes.
        assert_eq!(t.block_types, t.block_types_decoded);
        let mut out = Vec::new();
        assert!(write_files(&mut out, &c).is_ok());
        let text = String::from_utf8(out).unwrap();
        assert!(text.contains("| .nif | 1 |"), "{text}");
        assert!(text.contains("no entry in nv-rs's table: zzz"), "{text}");
    }

    #[test]
    fn counts_function_uses_from_scripts_and_conditions() {
        // A script calling GetDistance twice and SetGhost once, and a
        // dialogue line whose condition asks GetDistance.
        let source = "scn Test\nbegin GameMode\nif GetDistance player < 5\nset x to GetDistance player\nStartCannibal player\nendif\nend\n";
        let mut script = sub(b"EDID", &zstr("TestScript"));
        script.extend(sub(b"SCTX", source.as_bytes()));
        let mut condition = vec![0u8; 28];
        condition[8..10].copy_from_slice(&1u16.to_le_bytes());
        let info = sub(b"CTDA", &condition);
        let mut bytes = group(*b"SCPT", 0, &record(b"SCPT", 1, &script));
        bytes.extend(group(*b"INFO", 0, &record(b"INFO", 2, &info)));
        let order = LoadOrder::single("Test.esm", None, plugin(&bytes)).unwrap();
        let uses = function_uses(&order);
        assert_eq!(
            uses[&1],
            FunctionUse {
                calls: 2,
                scripts: 1,
                conditions: 1
            }
        );
        assert_eq!(uses[&126].calls, 1);

        let mut out = Vec::new();
        assert!(write_functions(&mut out, &uses).is_ok());
        let text = String::from_utf8(out).unwrap();
        // StartCannibal isn't carried out yet: it's in the missing list.
        assert!(!function_handled("StartCannibal"), "update this test");
        assert!(
            text.contains("| 126 | StartCannibal | 1 | 1 | 0 |"),
            "{text}"
        );
        assert!(text.contains("| 1 | GetDistance |"), "{text}");
    }

    #[test]
    fn reads_ogg_identification_headers() {
        let mut b = b"OggS".to_vec();
        b.extend([0; 22]);
        b.push(1); // one segment
        b.push(30);
        b.extend(b"\x01vorbis");
        b.extend(0u32.to_le_bytes());
        b.push(1);
        b.extend(22050u32.to_le_bytes());
        assert_eq!(ogg_format(&b), "Vorbis 22050 Hz 1 ch");
        assert_eq!(ogg_format(b"RIFF"), "not Ogg");
    }
}
